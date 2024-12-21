use log::{debug, error};

const CAN_ID_BRAKES: u16 = 0x085; // 100hz
const CAN_ID_RPM_SPEED_ACCEL: u16 = 0x201; // 100hz
const CAN_ID_COOLANT_THROTTLE_INTAKE: u16 = 0x240; // 10hz
const CAN_ID_FUEL_LEVEL: u16 = 0x430; // 10 hz
const CAN_ID_WHEEL_SPEEDS: u16 =  0x4b0; // 100hz

// masks and shifts assume little endian
const BRAKE_PRESSURE_MASK: u64 = 0xff_ff_00_00_00_00_00_00; // 6-7
const BRAKE_PRESSURE_BIT_SHIFT: usize = 6 * 8;

const RPM_MASK: u64 = 0xff_ff_00_00_00_00_00_00; // 6-7
const RPM_BIT_SHIFT: usize = 6 * 8;

const SPEED_MASK: u64 = 0xff_ff_00_00; // 2-3
const SPEED_BIT_SHIFT: usize = 2 * 8;

const ACCEL_MASK: u64 = 0xff_00; // 1
const ACCEL_BIT_SHIFT: usize = 1 * 8;

const ENGINE_LOAD_MASK: u64 = 0xff_00_00_00_00_00_00_00; // 7
const ENGINE_LOAD_BIT_SHIFT: usize = 7 * 8;

const COOLANT_MASK: u64 = 0xff_00_00_00_00_00_00; // 6
const COOLANT_BIT_SHIFT: usize = 6 * 8;

const THROTTLE_MASK: u64 = 0xff_00_00_00_00; // 4
const THROTTLE_BIT_SHIFT: usize = 4 * 8;

const INTAKE_MASK: u64 = 0xff_00_00_00; // 3
const INTAKE_BIT_SHIFT: usize = 3 * 8;

const FUEL_LEVEL_MASK: u64 = 0xff_00_00_00_00_00_00_00; // 7
const FUEL_LEVEL_BIT_SHIFT: usize = 7 * 8;

const FL_SPEED_MASK: u64 = 0xff_ff_00_00_00_00_00_00; // 6-7
const FL_SPEED_BIT_SHIFT: usize = 6 * 8;

const FR_SPEED_MASK: u64 = 0xff_ff_00_00_00_00; // 4-5
const FR_SPEED_BIT_SHIFT: usize = 4 * 8;

const RL_SPEED_MASK: u64 = 0xff_ff_00_00; // 2-3
const RL_SPEED_BIT_SHIFT: usize = 2 * 8;

const RR_SPEED_MASK: u64 = 0xff_ff; // 0-1

const BRAKE_PRESSURE_OFFSET: i16 = 102;
const BRAKE_PRESSURE_COEF: f32 = 0.2;
const RPM_DIV: u16 = 4;
const SPEED_DIV: f32 = 100.0;
const SPEED_OFFSET: i16 = 100;
const ACCEL_DIV: u8 = 2;
const PCT_DIV: f32 = 2.55;
const TEMP_OFFSET: i16 = 40;

#[repr(C)]
pub struct Metrics {
    rpm: u16,
    speed_kmh: u16,
    engine_coolant_temp_c: i16,
    intake_air_temp_c: i16,
    fl_speed_kmh: u16,
    fr_speed_kmh: u16,
    rl_speed_kmh: u16,
    rr_speed_kmh: u16,
    accelerator_pedal_position_pct: u8,
    calculated_engine_load_pct: u8,
    throttle_valve_position_pct: u8,
    fuel_level_pct: u8,
    brakes_pct: u8
}

impl Metrics {
    pub fn handle_can_msg(&mut self, can_id: u16, can_data: u64) {
        match can_id {
            CAN_ID_BRAKES => self.handle_brakes(can_data),
            CAN_ID_RPM_SPEED_ACCEL => self.handle_rpm_speed_accel(can_data),
            CAN_ID_COOLANT_THROTTLE_INTAKE => self.handle_load_coolant_throttle_intake(can_data),
            CAN_ID_FUEL_LEVEL => self.handle_fuel_level(can_data),
            CAN_ID_WHEEL_SPEEDS => self.handle_wheel_speeds(can_data),
            _ => error!("Unhandled CAN ID: {:#x}", can_id)
        }
    }

    fn handle_brakes(&mut self, can_data: u64) {
        let mut brake_pressure = ((can_data & BRAKE_PRESSURE_MASK) >> BRAKE_PRESSURE_BIT_SHIFT) as i16;
        brake_pressure -= BRAKE_PRESSURE_OFFSET;

        // Pressure can be momentarily negative (vacuum ?)
        // Limit to min 0
        self.brakes_pct = (brake_pressure.max(0) as f32 * BRAKE_PRESSURE_COEF) as u8;

        debug!("brakes {} %", self.brakes_pct)
    }

    fn handle_rpm_speed_accel(&mut self, can_data: u64) {
        let rpm = ((can_data & RPM_MASK) >> RPM_BIT_SHIFT) as u16;
        self.rpm = rpm / RPM_DIV;

        let speed = ((can_data & SPEED_MASK) >> SPEED_BIT_SHIFT) as u16;
        self.speed_kmh = raw_speed_to_kmh(speed);

        let accel = ((can_data & ACCEL_MASK) >> ACCEL_BIT_SHIFT) as u8;
        self.accelerator_pedal_position_pct = accel / ACCEL_DIV;

        debug!("rpm {}, speed {} kmh, accel {} %",
            self.rpm, self.speed_kmh, self.accelerator_pedal_position_pct);
    }

    fn handle_load_coolant_throttle_intake(&mut self, can_data: u64) {
        let engine_load = ((can_data & ENGINE_LOAD_MASK) >> ENGINE_LOAD_BIT_SHIFT) as u8;
        self.calculated_engine_load_pct = raw_to_pct(engine_load);

        let coolant_temp = ((can_data & COOLANT_MASK) >> COOLANT_BIT_SHIFT) as i16;
        self.engine_coolant_temp_c = raw_to_temp(coolant_temp);

        let throttle_valve = ((can_data & THROTTLE_MASK) >> THROTTLE_BIT_SHIFT) as u8;
        self.throttle_valve_position_pct = raw_to_pct(throttle_valve);

        let intake_temp = ((can_data & INTAKE_MASK) >> INTAKE_BIT_SHIFT) as i16;
        self.intake_air_temp_c = raw_to_temp(intake_temp);


        debug!("engine {} %, coolant {} °C, throttle {} %, intake {} °C",
               self.calculated_engine_load_pct, self.engine_coolant_temp_c,
               self.throttle_valve_position_pct, self.intake_air_temp_c);
    }

    fn handle_fuel_level(&mut self, can_data: u64) {
        let fuel_level = ((can_data & FUEL_LEVEL_MASK) >> FUEL_LEVEL_BIT_SHIFT) as u8;
        self.fuel_level_pct = raw_to_pct(fuel_level);

        debug!("fuel {} %", self.fuel_level_pct);
    }

    fn handle_wheel_speeds(&mut self, can_data: u64) {
        let fl = ((can_data & FL_SPEED_MASK) >> FL_SPEED_BIT_SHIFT) as u16;
        self.fl_speed_kmh = raw_speed_to_kmh(fl);

        let fr = ((can_data & FR_SPEED_MASK) >> FR_SPEED_BIT_SHIFT)as u16;
        self.fr_speed_kmh = raw_speed_to_kmh(fr);

        let rl = ((can_data & RL_SPEED_MASK) >> RL_SPEED_BIT_SHIFT)as u16;
        self.rl_speed_kmh = raw_speed_to_kmh(rl);

        let rr = (can_data & RR_SPEED_MASK)as u16;
        self.rr_speed_kmh = raw_speed_to_kmh(rr);

        debug!("fl {} fr {} rl {} rr {} kmh\n",
               self.fl_speed_kmh, self.fr_speed_kmh, self.rl_speed_kmh, self.rr_speed_kmh);
    }
}

fn raw_speed_to_kmh(raw_speed: u16) -> u16 {
    let speed = (raw_speed as f32 / SPEED_DIV) as i16 - SPEED_OFFSET;
    if speed < 0 { 0 } else { speed as u16 }
}

fn raw_to_pct(raw: u8) -> u8 {
    (raw as f32 / PCT_DIV) as u8
}

fn raw_to_temp(raw: i16) -> i16 {
    raw - TEMP_OFFSET
}