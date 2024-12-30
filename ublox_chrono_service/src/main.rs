mod ublox;

use log::{debug, info};
use nix::sys::signal::{self, sigprocmask, Signal};
use nix::sys::signalfd::{SigSet, SignalFd};
use nix::sys::epoll::{Epoll, EpollCreateFlags, EpollEvent, EpollFlags, EpollTimeout};
use nix::sys::termios::BaudRate;
use crate::ublox::Ublox;

const SHM_NAME: &str = "/ubloxchrono";

fn main() {
    let env = env_logger::Env::default()
        .filter_or("RUST_LOG", "info");

    env_logger::init_from_env(env);

    enum EpollEventId {
        Signal,
        Ublox
    }

    let mut ublox = Ublox::new("/dev/pts/9", BaudRate::B38400);

    let sfd = setup_signal_handler();

    let epoll = Epoll::new(EpollCreateFlags::empty())
        .expect("epoll");

    epoll.add(&sfd, EpollEvent::new(EpollFlags::EPOLLIN, EpollEventId::Signal as u64))
        .expect("epoll add signalFd");

    epoll.add(ublox.get_fd(), EpollEvent::new(EpollFlags::EPOLLIN, EpollEventId::Ublox as u64))
        .expect("epoll add ublox");

    ublox.configure();


    info!("Ready at /dev/shm{}", SHM_NAME);

    let mut events = [EpollEvent::empty()];

    loop {
        epoll.wait(&mut events, EpollTimeout::NONE)
            .expect("epoll wait");

        if events[0].data() == EpollEventId::Signal as u64 {
            handle_signal(&sfd);
            break;
        }

        if events[0].data() == EpollEventId::Ublox as u64 {
            //
        }
    }

    info!("Shutting down ....");

    drop(ublox);

    info!("Bye :)");
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
                debug!("Got SIGINT")
            }
            else if signal.ssi_signo == Signal::SIGTERM as u32 {
                debug!("Got SIGTERM")
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
