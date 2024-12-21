use std::num::NonZeroUsize;
use log::trace;
use nix::fcntl::OFlag;
use nix::libc::off_t;
use nix::sys::mman::{mmap, shm_open, shm_unlink, MapFlags, ProtFlags};
use nix::sys::stat::Mode;
use nix::unistd::ftruncate;
use crate::metrics::Metrics;

fn setup_shared_memory_metrics(name: &str) -> &'static mut Metrics {
    let shm_size = NonZeroUsize::new(size_of::<Metrics>()).unwrap();

    let mode_755 = Mode::S_IRWXU | Mode::S_IRGRP | Mode::S_IXGRP | Mode::S_IROTH | Mode::S_IXOTH;
    let shm_fd = shm_open(name, OFlag::O_CREAT | OFlag::O_RDWR, mode_755)
        .expect("shm_open");

    ftruncate(&shm_fd, shm_size.get() as off_t)
        .expect("ftruncate");

    unsafe {
        let mmap_c_void_ptr = mmap(None, shm_size, ProtFlags::PROT_READ | ProtFlags::PROT_WRITE, MapFlags::MAP_SHARED, &shm_fd, 0)
            .expect("mmap");

        &mut *(mmap_c_void_ptr.as_ptr() as *mut Metrics)
    }
}

pub struct ShmMetrics {
    name: &'static str,
    pub metrics: &'static mut Metrics
}

impl ShmMetrics {
    pub fn new(name: &'static str) -> ShmMetrics {
        ShmMetrics {
            name,
            metrics: setup_shared_memory_metrics(name)
        }
    }
}

impl Drop for ShmMetrics {
    fn drop(&mut self) {
        trace!("shm unlink {}", self.name);
        shm_unlink(self.name)
            .expect("shm_unlink");
    }
}
