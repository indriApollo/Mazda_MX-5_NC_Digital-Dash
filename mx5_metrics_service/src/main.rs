use nix::sys::signal::{self, sigprocmask, Signal};
use nix::sys::signalfd::{SigSet, SignalFd};

fn main() {
    let sfd = setup_signal_handler();
    println!("Ready");

    loop {
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
                break;
            }
            Ok(None) => {
                unreachable!("SIG_BLOCK")
            },
            Err(e) => {
                panic!("Error reading signal: {}", e);
            }
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
