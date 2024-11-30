mod serial_port;
mod stnobd;

use std::collections::VecDeque;
use nix::sys::signal::{self, sigprocmask, Signal};
use nix::sys::signalfd::{SigSet, SignalFd};
use nix::sys::epoll::{Epoll, EpollCreateFlags, EpollEvent, EpollFlags, EpollTimeout};
use nix::sys::termios::BaudRate;
use crate::stnobd::{Stnobd, STNOBD_CFG_DISABLE_ECHO, STNOBD_CFG_DISABLE_SPACES, STNOBD_CFG_ENABLE_HEADER, STNOBD_CFG_FILTER_BRAKES, STNOBD_CFG_FILTER_COOLANT_THROTTLE_INTAKE, STNOBD_CFG_FILTER_FUEL_LEVEL, STNOBD_CFG_FILTER_RPM_SPEED_ACCEL, STNOBD_CFG_FILTER_WHEEL_SPEEDS};

fn main() {
    enum EpollEventId {
        Signal,
        Stnobd
    }

    let mut cmds = VecDeque::new();
    cmds.push_back(STNOBD_CFG_DISABLE_ECHO);
    cmds.push_back(STNOBD_CFG_ENABLE_HEADER);
    cmds.push_back(STNOBD_CFG_DISABLE_SPACES);
    cmds.push_back(STNOBD_CFG_FILTER_BRAKES);
    cmds.push_back(STNOBD_CFG_FILTER_RPM_SPEED_ACCEL);
    cmds.push_back(STNOBD_CFG_FILTER_COOLANT_THROTTLE_INTAKE);
    cmds.push_back(STNOBD_CFG_FILTER_FUEL_LEVEL);
    cmds.push_back(STNOBD_CFG_FILTER_WHEEL_SPEEDS);

    let mut stnobd = Stnobd::new("/dev/pts/2", BaudRate::B921600, cmds);

    let sfd = setup_signal_handler();

    let epoll = Epoll::new(EpollCreateFlags::empty())
        .expect("epoll");

    epoll.add(&sfd, EpollEvent::new(EpollFlags::EPOLLIN, EpollEventId::Signal as u64))
        .expect("epoll add signalFd");

    epoll.add(stnobd.get_fd(), EpollEvent::new(EpollFlags::EPOLLIN, EpollEventId::Stnobd as u64))
        .expect("epoll add stnobd");

    stnobd.send_reset_cmd();

    println!("Ready");

    let mut events = [EpollEvent::empty()];

    loop {
        epoll.wait(&mut events, EpollTimeout::NONE)
            .expect("epoll wait");

        if events[0].data() == EpollEventId::Signal as u64 {
            handle_signal(&sfd);
            break;
        }

        if events[0].data() == EpollEventId::Stnobd as u64 {
            stnobd.handle_incoming_stnobd_msg();
        }
    }
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