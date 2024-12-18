const CAN_ID_BRAKES: u16 = 0x085; // 100hz
const CAN_ID_HEX_STR_BRAKES: &str = "085";
const CAN_ID_RPM_SPEED_ACCEL: u16 = 0x201; // 100hz
const CAN_ID_HEX_STR_RPM_SPEED_ACCEL: &str = "201";
const CAN_ID_COOLANT_THROTTLE_INTAKE: u16 = 0x240; // 10hz
const CAN_ID_HEX_STR_COOLANT_THROTTLE_INTAKE: &str = "240";
const CAN_ID_FUEL_LEVEL: u16 = 0x430; // 10 hz
const CAN_ID_HEX_STR_FUEL_LEVEL: &str = "430";
const CAN_ID_WHEEL_SPEEDS: u16 =  0x4b0; // 100hz
const CAN_ID_HEX_STR_WHEEL_SPEEDS: &str = "4B0";

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
    accelerator_pedal_position_pct: u8,
    calculated_engine_load_pct: u8,
    engine_coolant_temp_c: i16,
    throttle_valve_position_pct: u8,
    intake_air_temp_c: i16,
    fuel_level_pct: u8,
    brakes_pct: u8,
    fl_speed_kmh: u16,
    fr_speed_kmh: u16,
    rl_speed_kmh: u16,
    rr_speed_kmh: u16
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            brakes_pct: 0,
            rpm: 0,
            speed_kmh: 0,
            accelerator_pedal_position_pct: 0,
            calculated_engine_load_pct: 0,
            engine_coolant_temp_c: 0,
            throttle_valve_position_pct: 0,
            intake_air_temp_c: 0,
            fuel_level_pct: 0,
            fl_speed_kmh: 0,
            fr_speed_kmh: 0,
            rl_speed_kmh: 0,
            rr_speed_kmh: 0
        }
    }

    pub fn handle_can_msg(&mut self, can_id: u16, can_data: u64) {
        match can_id {
            CAN_ID_BRAKES => self.handle_brakes(can_data),
            CAN_ID_RPM_SPEED_ACCEL => self.handle_rpm_speed_accel(can_data),
            CAN_ID_COOLANT_THROTTLE_INTAKE => self.handle_load_coolant_throttle_intake(can_data),
            CAN_ID_FUEL_LEVEL => self.handle_fuel_level(can_data),
            CAN_ID_WHEEL_SPEEDS => self.handle_wheel_speeds(can_data),
            _ => println!("Unhandled CAN ID: {:#x}", can_id)
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

    fn handle_brakes(&mut self, can_data: u64) {
        /*let brake_pressure = (((can_data & BRAKE_PRESSURE_MASK) >> BRAKE_PRESSURE_BIT_SHIFT) as i16 - BRAKE_PRESSURE_OFFSET)
            .max(0);
        self.brakes_pct = (brake_pressure as f32 * BRAKE_PRESSURE_COEF) as u8;*/
    }

    fn handle_rpm_speed_accel(&mut self, can_data: u64) {
        //
    }

    fn handle_load_coolant_throttle_intake(&mut self, can_data: u64) {
        //
    }

    fn handle_fuel_level(&mut self, can_data: u64) {
        //
    }

    fn handle_wheel_speeds(&mut self, can_data: u64) {
        //
    }
}