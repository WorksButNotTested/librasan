#[cfg(test)]
mod tests {

    use {
        asan::{
            allocator::{
                backend::dlmalloc::DlmallocBackend,
                frontend::{default::DefaultFrontend, Allocator},
            },
            mmap::linux::LinuxMmap,
            shadow::{
                guest::{DefaultShadowLayout, GuestShadow},
                Shadow,
            },
            tracking::guest::GuestTracking,
        },
        spin::{Lazy, Mutex, MutexGuard},
    };

    static INIT_ONCE: Lazy<Mutex<DF>> = Lazy::new(|| {
        Mutex::new({
            env_logger::init();
            let backend = DlmallocBackend::<LinuxMmap>::new();
            let shadow = GuestShadow::<LinuxMmap, DefaultShadowLayout>::new().unwrap();
            let tracking = GuestTracking::new().unwrap();
            DF::new(
                backend,
                shadow,
                tracking,
                DF::DEFAULT_REDZONE_SIZE,
                DF::DEFAULT_QUARANTINE_SIZE,
            )
            .unwrap()
        })
    });

    type DF = DefaultFrontend<
        DlmallocBackend<LinuxMmap>,
        GuestShadow<LinuxMmap, DefaultShadowLayout>,
        GuestTracking,
    >;

    fn frontend() -> MutexGuard<'static, DF> {
        INIT_ONCE.lock()
    }

    #[test]
    #[cfg(all(feature = "linux", feature = "dlmalloc"))]
    fn test_allocate() {
        let mut frontend = frontend();
        let buf = frontend.alloc(16, 8).unwrap();
        frontend.dealloc(buf).unwrap();
    }

    #[test]
    #[cfg(all(feature = "linux", feature = "dlmalloc"))]
    fn test_allocate_is_poisoned() {
        let mut frontend = frontend();
        let len = 16;
        let buf = frontend.alloc(len, 8).unwrap();
        for i in buf - DF::DEFAULT_REDZONE_SIZE..buf {
            assert!(frontend.shadow.is_poison(i, 1).unwrap());
        }
        for i in buf..buf + len {
            assert!(!frontend.shadow.is_poison(i, 1).unwrap());
        }
        for i in buf + len..buf + len + DF::DEFAULT_REDZONE_SIZE {
            assert!(frontend.shadow.is_poison(i, 1).unwrap());
        }
        frontend.dealloc(buf).unwrap();
    }
}
