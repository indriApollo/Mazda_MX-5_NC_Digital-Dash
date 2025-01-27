use std::os::fd::OwnedFd;
use nix::sys::termios::BaudRate;
use serial_port::SerialPort;
use crate::ublox::ReadProgress::SearchForSync;

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

fn fletcher8(buffer: &[u8]) -> u16 {
    let mut ck_a: u8 = 0;
    let mut ck_b: u8 = 0;

    for &e in buffer {
        ck_a = ck_a.wrapping_add(e);
        ck_b = ck_b.wrapping_add(ck_a);
    }

    u16::from_be_bytes([ck_a, ck_b])
}

enum ReadProgress {
    SearchForSync,
    HandleHeaderPayload,
    VerifyCk
}

pub struct Ublox {
    serial_port: SerialPort,
    buf: [u8; 256],
    buf_read_pos: usize,
    buf_read_count: usize,
    buf_read_progress: ReadProgress,
    buf_msg_start: usize
}

impl Ublox {
    pub fn new(port_name: &str, baud: BaudRate) -> Ublox {
        let sp = SerialPort::new(port_name);
        sp.set_access_exclusive();
        sp.configure(1, 1, baud);

        Ublox {
            serial_port: sp,
            buf: [0; 256],
            buf_read_pos: 0,
            buf_read_count: UBX_MIN_LEN,
            buf_read_progress: SearchForSync,
            buf_msg_start: 0
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

    pub fn handle_incoming_ublox_msg(&mut self) {

    }

    fn reset_read(&mut self, nbytes_to_keep: usize) {
        self.buf_read_pos = nbytes_to_keep;
        self.buf_read_count = UBX_MIN_LEN - nbytes_to_keep;
        self.buf_read_progress = SearchForSync;
    }

    fn parse_ublox_msg(&mut self) {
        let buf_slice = &mut self.buf[self.buf_read_pos..self.buf_read_count];
        let c = self.serial_port.read(buf_slice);

        self.buf_read_pos += c;
        self.buf_read_count -= c;

        if self.buf_read_count > 0 {
            return // partial msg
        }

        if let SearchForSync = self.buf_read_progress {
            match self.buf[..UBX_MIN_LEN].iter().position(|&b| b == UBX_SYNC_CHAR_1) {
                Some(sync_pos) => {
                    // the message starts at sync char 1
                    self.buf_msg_start = sync_pos;
                    self.buf_read_progress = ReadProgress::HandleHeaderPayload;
                    if sync_pos > 0 {
                        // since we had to skip n bytes to find sync char 1,
                        // we need to fetch the next n bytes to again have UBX_MIN_LEN
                        // bytes in our buffer
                        self.buf_read_count = sync_pos;
                        return // partial msg
                    }
                }
                None => {
                    // we didn't find sync char 1 in the available data
                    self.reset_read(0);
                    return
                }
            }
        }

        if let ReadProgress::HandleHeaderPayload = self.buf_read_progress {
            if self.buf[self.buf_msg_start + 1] != UBX_SYNC_CHAR_2 {
                // we expected sync char 2 immediately after sync char 1
                // discard wrong sync char and look for sync char 1 again
                let nbytes_to_keep = UBX_MIN_LEN - 1;
                self.buf.copy_within(self.buf_msg_start + 1..nbytes_to_keep, 0);
                self.reset_read(nbytes_to_keep);

                return // unknown data
            }

            const uint16_t payload_len = as_uint16(buffer_read.ubx_msg + UBX_LEN_OFFSET);

            buffer_read.progress = VERIFY_CK;

            if (payload_len != 0) {
                buffer_read.requested_count = payload_len;

                if (buffer_read.offset + buffer_read.requested_count > sizeof(input_buffer)) {
                    const uint8_t msg_class = buffer_read.ubx_msg[UBX_CLASS_OFFSET];
                    const uint8_t msg_id = buffer_read.ubx_msg[UBX_ID_OFFSET];

                    fprintf(stderr, "ubx msg class %d id %d len %d is too big for our %lu buffer\n", msg_class, msg_id,
                            payload_len, sizeof(input_buffer));

                    reset_read(0);

                    return -PAYLOAD_TOO_BIG;
                }

                return PARTIAL_MSG;
            }

            assert(buffer_read.progress == VERIFY_CK);
            const uint16_t payload_len = as_uint16(buffer_read.ubx_msg + UBX_LEN_OFFSET);
            const uint16_t expected_ck = as_uint16(buffer_read.ubx_msg + UBX_PAYLOAD_OFFSET + payload_len);

            const uint16_t actual_ck = fletcher8(buffer_read.ubx_msg + UBX_CLASS_OFFSET, UBX_HEADER_LEN + payload_len);
            if (actual_ck != expected_ck) {
                fprintf(stderr, "ck actual %04x but expected %04x\n", actual_ck, expected_ck);
                return -CK_FAIL;
            }

            *msg = buffer_read.ubx_msg;
            reset_read(0);

            return 0;
        }
    }
}

impl Drop for Ublox {
    fn drop(&mut self) {
        self.serial_port.set_access_nonexclusive();
    }
}