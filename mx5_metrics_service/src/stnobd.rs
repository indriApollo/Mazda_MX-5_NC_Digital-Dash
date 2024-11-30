use std::collections::VecDeque;
use std::os::fd::OwnedFd;
use std::thread::sleep;
use std::time::Duration;
use nix::sys::termios::BaudRate;
use crate::serial_port::{SerialPort};

fn contains_slice(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.len() > haystack.len() {
        return false;
    }

    haystack.windows(needle.len()).any(|window| window == needle)
}

pub const STNOBD_CFG_DISABLE_ECHO: &str = "ATE0\r";
pub const STNOBD_CFG_ENABLE_HEADER: &str = "ATH1\r";
pub const STNOBD_CFG_DISABLE_SPACES: &str = "ATS0\r";
pub const STNOBD_CFG_FILTER_BRAKES: &str = "STFPA085,FFF\r";
pub const STNOBD_CFG_FILTER_RPM_SPEED_ACCEL: &str = "STFPA201,FFF\r";
pub const STNOBD_CFG_FILTER_COOLANT_THROTTLE_INTAKE: &str = "STFPA240,FFF\r";
pub const STNOBD_CFG_FILTER_FUEL_LEVEL: &str = "STFPA430,FFF\r";
pub const STNOBD_CFG_FILTER_WHEEL_SPEEDS: &str = "STFPA4B0,FFF\r";


pub struct Stnobd {
    serial_port: SerialPort,
    reset_in_progress: bool,
    must_configure: bool,
    in_monitoring_mode: bool,
    cfg_cmds: VecDeque<&'static str>
}

impl Stnobd {
    pub fn new(port_name: &str, baud: BaudRate, cmds: VecDeque<&'static str>) -> Stnobd {
        let sp = SerialPort::new(port_name);
        sp.set_access_exclusive();
        sp.configure(1, 1, baud);

        Stnobd {
            serial_port: sp,
            reset_in_progress: false,
            must_configure: false,
            in_monitoring_mode: false,
            cfg_cmds: cmds
        }
    }

    pub fn get_fd(&self) -> &OwnedFd {
        &self.serial_port.fd
    }

    fn send_cfg_cmd(&mut self) {
        match self.cfg_cmds.pop_front() {
            // Send next command
            Some(cmd) => {
                println!("sending cfg cmd {}", cmd);
                self.serial_port.write(cmd.as_bytes());
            }
            // Or start monitoring once all commands were sent
            None => {
                self.start_monitoring_mode();
            }
        }
    }

    fn handle_cfg_rsp(&mut self) {
        const CFG_ACK: &str = ">OK\r";

        let mut buf: [u8; CFG_ACK.len()] = [0; CFG_ACK.len()];

        // Wait for full response (> prompt char can lag behind initial startup msg chars)
        sleep(Duration::from_millis(100));

        let c = self.serial_port.read(&mut buf);

        // TODO retry
        if c != buf.len() || !contains_slice(&buf, CFG_ACK.as_bytes()) {
            println!("didnt get expected cfg ack {}, {}\n", c, String::from_utf8_lossy(&buf));
        }

        self.serial_port.flush_all();
        self.send_cfg_cmd()
    }

    fn start_monitoring_mode(&mut self) {
        const CMD: &str = "STM\r";

        // Get rid of any existing unwanted bytes
        self.serial_port.flush_all();

        self.serial_port.write(CMD.as_bytes());

        self.in_monitoring_mode = true;
    }

    fn stop_monitoring_mode(&mut self) {
        const CMD: &str = "\r";

        self.serial_port.write(CMD.as_bytes());

        self.in_monitoring_mode = false;
    }

    pub fn send_reset_cmd(&mut self) {
        const CMD: &str = "ATZ\r";

        // Get rid of any existing unwanted bytes
        self.serial_port.flush_all();

        self.serial_port.write(CMD.as_bytes());

        self.reset_in_progress = true;
        self.must_configure = true;

        println!("STN reset in progress");
    }

    fn handle_reset_rsp(&mut self) {
        // TODO : the startup msg might be chopped when reading and we'd miss it

        const STARTUP_MSG: &str = "ELM327";

        let mut buf: [u8; 32] = [0; 32];

        // Wait for full response (> prompt char can lag behind initial startup msg chars)
        sleep(Duration::from_millis(100));

        // Read a bunch of bytes in the hope of finding the STN startup msg
        let c = self.serial_port.read(&mut buf);

        if c < STARTUP_MSG.len() {
            println!("not enough bytes to contain STN startup msg");
            return;
        }

        if contains_slice(&buf, STARTUP_MSG.as_bytes())
        {
            // We got the STN startup message, reset is complete
            self.reset_in_progress = false;
            // Get rid of any existing unwanted bytes
            self.serial_port.flush_all();

            println!("STN reset done");

            self.send_cfg_cmd();
        }
    }

    fn handle_monitoring_rsp(&self) {

    }

    pub fn handle_incoming_stnobd_msg(&mut self)
    {
        if self.reset_in_progress {
            return self.handle_reset_rsp();
        }

        if self.must_configure {
            return self.handle_cfg_rsp();
        }

        if self.in_monitoring_mode {
            return self.handle_monitoring_rsp();
        }

        self.serial_port.flush_all();
        println!("got unhandled stn msg");
    }
}

impl Drop for Stnobd {
    fn drop(&mut self) {
        if self.in_monitoring_mode {
            self.stop_monitoring_mode()
        }
        self.serial_port.set_access_nonexclusive();
    }
}
