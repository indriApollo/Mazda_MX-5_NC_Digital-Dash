mod serial_port;
mod stnobd;

use nix::sys::signal::{self, sigprocmask, Signal};
use nix::sys::signalfd::{SigSet, SignalFd};
use nix::sys::epoll::{Epoll, EpollCreateFlags, EpollEvent, EpollFlags, EpollTimeout};
use nix::sys::termios::BaudRate;
use crate::serial_port::{SerialPort};

fn main() {
    enum EpollEventId {
        Signal
    }

    let serial_port = SerialPort::new("/dev/pts/2");
    serial_port.set_access_exclusive();
    serial_port.configure(1, 1, BaudRate::B921600);

    let sfd = setup_signal_handler();

    let epoll = Epoll::new(EpollCreateFlags::empty())
        .expect("epoll");

    epoll.add(&sfd, EpollEvent::new(EpollFlags::EPOLLIN, EpollEventId::Signal as u64))
        .expect("epoll add");

    println!("Ready");

    let mut events = [EpollEvent::empty()];

    loop {
        epoll.wait(&mut events, EpollTimeout::NONE)
            .expect("epoll wait");

        if events[0].data() == EpollEventId::Signal as u64 {
            handle_signal(&sfd);
            break;
        }

    }

    serial_port.set_access_nonexclusive();
}

fn setup_signal_handler() -> SignalFd {
    let mut sigset = SigSet::empty();
    sigset.add(Signal::SIGINT);
    sigset.add(Signal::SIGTERM);
    sigprocmask(signal::SigmaskHow::SIG_BLOCK, Some(&sigset), None)
        .expect("sigprocmask");

    SignalFd::new(&sigset).expect("signalFd")
}

fn handle_signal(sfd: &SignalFd) {
    match sfd.read_signal() {
        Ok(Some(signal)) => {
            if signal.ssi_signo == Signal::SIGINT as u32 {
                println!("Got SIGINT")
            }
            else if signal.ssi_signo == Signal::SIGTERM as u32 {
                println!("Got SIGTERM")
            }
            else {
                panic!("Unexpected signal: {}", signal.ssi_signo);
            }
        }
        Ok(None) => {
            unreachable!("SIG_BLOCK")
        },
        Err(e) => {
            panic!("Error reading signal: {}", e);
        }
    }
}