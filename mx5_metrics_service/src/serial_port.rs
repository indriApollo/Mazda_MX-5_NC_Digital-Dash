use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::path::Path;
use nix::fcntl::{open, OFlag};
use nix::{ioctl_none_bad, libc};
use nix::sys::stat::Mode;
use nix::sys::termios::{cfmakeraw, cfsetspeed, tcgetattr, tcsetattr, BaudRate, ControlFlags, InputFlags, OutputFlags, SetArg, SpecialCharacterIndices};

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

pub fn open_serial_port_blocking_io(port_name: &str) -> OwnedFd {
    let fd = open(Path::new(port_name), OFlag::O_RDWR | OFlag::O_NOCTTY, Mode::empty())
        .expect("open serial");
    unsafe { OwnedFd::from_raw_fd(fd) }
}

pub fn set_serial_port_access_exclusive(fd: &OwnedFd) {
    set_tiocexcl(fd)
        .expect("ioctl TIOCEXCL");
}

pub fn set_serial_port_access_nonexclusive(fd: &OwnedFd) {
    set_tiocnxcl(fd)
        .expect("ioctl TIOCNXCL");
}

pub fn configure_serial_port(fd: &OwnedFd, vtime: u8, vmin: u8, baud: BaudRate) {
    let mut tty = tcgetattr(fd)
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

    tcsetattr(fd, SetArg::TCSANOW, &tty)
        .expect("tcsetattr");
}