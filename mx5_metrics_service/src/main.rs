mod serial_port;
mod stnobd;
mod metrics;

use std::collections::VecDeque;
use std::num::NonZeroUsize;
use nix::fcntl::OFlag;
use nix::libc::{off_t};
use nix::sys::signal::{self, sigprocmask, Signal};
use nix::sys::signalfd::{SigSet, SignalFd};
use nix::sys::epoll::{Epoll, EpollCreateFlags, EpollEvent, EpollFlags, EpollTimeout};
use nix::sys::mman::{mmap, shm_open, MapFlags, ProtFlags};
use nix::sys::stat::Mode;
use nix::sys::termios::BaudRate;
use nix::unistd::ftruncate;
use crate::metrics::Metrics;
use crate::stnobd::{Stnobd, STNOBD_CFG_DISABLE_ECHO, STNOBD_CFG_DISABLE_SPACES, STNOBD_CFG_ENABLE_HEADER, STNOBD_CFG_FILTER_BRAKES, STNOBD_CFG_FILTER_COOLANT_THROTTLE_INTAKE, STNOBD_CFG_FILTER_FUEL_LEVEL, STNOBD_CFG_FILTER_RPM_SPEED_ACCEL, STNOBD_CFG_FILTER_WHEEL_SPEEDS};

const SHM_NAME: &str = "/mx5metrics";

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

    let mut metrics = setup_shared_memory_metrics();

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
            stnobd.handle_incoming_stnobd_msg(&mut metrics);
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

fn setup_shared_memory_metrics() -> &'static mut Metrics {
    let shm_size = NonZeroUsize::new(size_of::<Metrics>()).unwrap();

    let mode_755 = Mode::S_IRWXU | Mode::S_IRGRP | Mode::S_IXGRP | Mode::S_IROTH | Mode::S_IXOTH;
    let shm_fd = shm_open(SHM_NAME, OFlag::O_CREAT | OFlag::O_RDWR, mode_755)
        .expect("shm_open");

    ftruncate(&shm_fd, shm_size.get() as off_t)
        .expect("ftruncate");

    unsafe {
        let mmap_c_void_ptr = mmap(None, shm_size, ProtFlags::PROT_READ | ProtFlags::PROT_WRITE, MapFlags::MAP_SHARED, &shm_fd, 0)
            .expect("mmap");

        &mut *(mmap_c_void_ptr.as_ptr() as *mut Metrics)
    }
}