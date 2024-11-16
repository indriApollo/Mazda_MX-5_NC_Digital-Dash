use std::thread::sleep;
use std::time::Duration;
use crate::serial_port::{SerialPort};

fn contains_slice(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.len() > haystack.len() {
        return false;
    }

    haystack.windows(needle.len()).any(|window| window == needle)
}

pub struct Stnobd {
    serial_port: SerialPort,
    reset_in_progress: bool,
    must_configure: bool,
    in_monitoring_mode: bool,
    cfg_cmds: [String],
    current_cfg_cmd: u8,
}

impl Stnobd {
    fn send_cfg_cmd(&self) {
        let cmd = self.cfg_cmds[self.current_cfg_cmd];

        println!("sending cfg cmd {}", cmd);
        self.serial_port.write(cmd);
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

    fn send_reset_cmd(&mut self) {
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
}