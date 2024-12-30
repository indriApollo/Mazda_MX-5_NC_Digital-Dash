use std::os::fd::OwnedFd;
use nix::sys::termios::BaudRate;
use serial_port::SerialPort;

const UBX_SYNC_CHAR_1: u8 = 0xb5;
const UBX_SYNC_CHAR_2: u8 = 0x62;

const UBX_SYNC_LEN: usize = 2;
const UBX_CLASS_LEN: usize = 1;
const UBX_ID_LEN: usize = 1;
const UBX_LEN_LEN: usize = 2;
const UBX_HEADER_LEN: usize = UBX_CLASS_LEN + UBX_ID_LEN + UBX_LEN_LEN;
const UBX_CK_LEN: usize = 2;
const UBX_MIN_LEN: usize = UBX_SYNC_LEN + UBX_HEADER_LEN + UBX_CK_LEN;

const UBX_CLASS_OFFSET: usize = UBX_SYNC_LEN;
const UBX_ID_OFFSET: usize = UBX_CLASS_OFFSET + UBX_CLASS_LEN;
const UBX_LEN_OFFSET: usize = UBX_ID_OFFSET + UBX_ID_LEN;
const UBX_PAYLOAD_OFFSET: usize = UBX_LEN_OFFSET + UBX_LEN_LEN;

const CLASS_NAV: u8 = 0x01;
const CLASS_ACK: u8 = 0x05;
const CLASS_CFG: u8 = 0x06;

const ID_NAV_POSLLH: u8 = 0x02;
const ID_CFG_VALSET: u8 = 0x8a;

const CFG_USBOUTPROT_UBX: u32 = 0x10780001;
const CFG_USBOUTPROT_NMEA: u32 = 0x10780002;
const CFG_NAVSPG_FIXMODE: u32 = 0x20110011;
const CFG_NAVSPG_DYNMODEL: u32 = 0x20110021;
const CFG_MSGOUT_UBX_NAV_POSLLH_USB: u32 = 0x2091002c;
const CFG_MSGOUT_UBX_NAV_STATUS_USB: u32 = 0x2091001d;
const CFG_RATE_MEAS: u32 = 0x30210001;
const CFG_RATE_NAV: u32 = 0x30210002;

const CFG_VERSION: u8 = 0;
const CFG_LAYER_RAM: u8 = 1;

const CFG_NAVSPG_FIXMODE_2DONLY: u8 = 1;
const CFG_NAVSPG_DYNMODEL_AUTOMOT: u8 = 4;

fn set_bool_cfg(cmd: &mut Vec<u8>, cfg: u32, val: bool) {
    set_u8_cfg(cmd, cfg, val as u8);
}

fn set_u8_cfg(cmd: &mut Vec<u8>, cfg: u32, val: u8) {
    cmd.append(&mut Vec::from(cfg.to_le_bytes()));
    cmd.push(val);
}

fn set_u16_cfg(cmd: &mut Vec<u8>, cfg: u32, val: u16) {
    cmd.append(&mut Vec::from(cfg.to_le_bytes()));
    cmd.append(&mut Vec::from(val.to_le_bytes()));
}

fn fletcher8(buffer: &[u8]) -> u16 { // todo fix
    let mut ck_a: u8 = 0;
    let mut ck_b: u8 = 0;

    for &e in buffer {
        ck_a = ck_a.wrapping_add(e);
        ck_b = ck_b.wrapping_add(ck_a);
    }

    u16::from_be_bytes([ck_a, ck_b])
}

pub struct Ublox {
    serial_port: SerialPort,
}

impl Ublox {
    pub fn new(port_name: &str, baud: BaudRate) -> Ublox {
        let sp = SerialPort::new(port_name);
        sp.set_access_exclusive();
        sp.configure(1, 1, baud);

        Ublox {
            serial_port: sp,
        }
    }

    pub fn get_fd(&self) -> &OwnedFd {
        &self.serial_port.fd
    }

    pub fn configure(&self) {
        let mut cmd = vec![
            UBX_SYNC_CHAR_1,
            UBX_SYNC_CHAR_2,
            CLASS_CFG,
            ID_CFG_VALSET
        ];

        let mut payload = vec![
            CFG_VERSION,
            CFG_LAYER_RAM,
            0, 0 // reserved
        ];

        // enable UBX, disable NMEA over usb
        set_bool_cfg(&mut payload, CFG_USBOUTPROT_UBX, true);
        set_bool_cfg(&mut payload, CFG_USBOUTPROT_NMEA, false);

        // set fix mode to 2d, automotive dynamic profile
        set_u8_cfg(&mut payload, CFG_NAVSPG_FIXMODE, CFG_NAVSPG_FIXMODE_2DONLY);
        set_u8_cfg(&mut payload, CFG_NAVSPG_DYNMODEL, CFG_NAVSPG_DYNMODEL_AUTOMOT);

        // set gnss measurements to 25hz
        set_u8_cfg(&mut payload, CFG_MSGOUT_UBX_NAV_POSLLH_USB, 1);
        set_u16_cfg(&mut payload, CFG_RATE_MEAS, 25);
        set_u16_cfg(&mut payload, CFG_RATE_NAV, 1);

        // set len in header and append payload
        let payload_len = payload.len() as u16;
        cmd.append(&mut Vec::from(payload_len.to_le_bytes()));
        cmd.append(&mut payload);

        // compute and append checksum
        let ck = fletcher8(&cmd[UBX_CLASS_OFFSET..]);
        cmd.append(&mut Vec::from(ck.to_le_bytes()));

        self.serial_port.write(&cmd);
    }
}

impl Drop for Ublox {
    fn drop(&mut self) {
        self.serial_port.set_access_nonexclusive();
    }
}