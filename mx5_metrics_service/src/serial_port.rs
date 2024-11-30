use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::path::Path;
use nix::fcntl::{open, OFlag};
use nix::{ioctl_none_bad, libc};
use nix::sys::stat::Mode;
use nix::sys::termios::{cfmakeraw, cfsetspeed, tcflush, tcgetattr, tcsetattr, BaudRate, ControlFlags, FlushArg, InputFlags, OutputFlags, SetArg, SpecialCharacterIndices};
use nix::unistd::{read, write};

ioctl_none_bad!(tioc_excl, libc::TIOCEXCL);
ioctl_none_bad!(tioc_nxcl, libc::TIOCNXCL);

fn set_tiocexcl(fd: &OwnedFd) -> nix::Result<()> {
    unsafe { tioc_excl(fd.as_raw_fd()) }?;
    Ok(())
}

fn set_tiocnxcl(fd: &OwnedFd) -> nix::Result<()> {
    unsafe { tioc_nxcl(fd.as_raw_fd()) }?;
    Ok(())
}

pub struct SerialPort {
    pub fd: OwnedFd
}

impl SerialPort {
    pub fn new(port_name: &str) -> SerialPort {
        let fd = open(Path::new(port_name), OFlag::O_RDWR | OFlag::O_NOCTTY, Mode::empty())
            .expect("open serial");
        SerialPort {
            fd: unsafe { OwnedFd::from_raw_fd(fd) }
        }
    }

    pub fn set_access_exclusive(&self) {
        set_tiocexcl(&self.fd)
            .expect("ioctl TIOCEXCL");
    }

    pub fn set_access_nonexclusive(&self) {
        set_tiocnxcl(&self.fd)
            .expect("ioctl TIOCNXCL");
    }

    pub fn configure(&self, vtime: u8, vmin: u8, baud: BaudRate) {
        let mut tty = tcgetattr(&self.fd)
            .expect("tcgetattr");

        /*
         * Disable any special handling of received bytes
         * termios_p->c_iflag &= ~(IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON);
         *
         * Prevent special interpretation of output bytes (e.g. newline chars)
         * termios_p->c_oflag &= ~OPOST;
         *
         * Disable echo, use non-canonical mode, disable interpretation of INTR, QUIT and SUSP
         * termios_p->c_lflag &= ~(ECHO | ECHONL | ICANON | ISIG | IEXTEN);
         *
         * Clear parity bit, disabling parity, 8 bits per byte
         * termios_p->c_cflag &= ~(CSIZE | PARENB);
         * termios_p->c_cflag |= CS8;
         */
        cfmakeraw(&mut tty);

        tty.control_flags &= !ControlFlags::CSTOPB; // Clear stop field, only one stop bit used in communication
        tty.control_flags &= !ControlFlags::CRTSCTS; // Disable RTS/CTS hardware flow control
        tty.control_flags |= ControlFlags::CREAD | ControlFlags::CLOCAL; // Turn on READ & ignore ctrl lines

        tty.input_flags &= !(InputFlags::IXON | InputFlags::IXOFF | InputFlags::IXANY); // Turn off s/w flow ctrl

        tty.output_flags &= !OutputFlags::ONLCR; // Prevent conversion of newline to carriage return/line feed

        tty.control_chars[SpecialCharacterIndices::VTIME as usize] = vtime;
        tty.control_chars[SpecialCharacterIndices::VTIME as usize] = vmin;

        // Set in/out baud rate
        cfsetspeed(&mut tty, baud)
            .expect("cfsetspeed");

        tcsetattr(&self.fd, SetArg::TCSANOW, &tty)
            .expect("tcsetattr");
    }

    pub fn flush_all(&self) {
        tcflush(&self.fd, FlushArg::TCIOFLUSH)
            .expect("tcflush");
    }

    pub fn write(&self, buf: &[u8]) {
        let c = write(&self.fd, buf)
            .expect("write");

        if c != buf.len() {
            panic!("incomplete write (actual {} expected {})\n", c, buf.len());
        }
    }

    pub fn read(&self, buf: &mut [u8]) -> usize {
        let fd = self.fd.as_raw_fd();
        read(fd, buf)
            .expect("read")
    }
}