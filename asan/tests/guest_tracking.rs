#[cfg(test)]
mod tests {

    use {
        asan::{
            tracking::{
                guest::{GuestTracking, GuestTrackingError},
                Tracking,
            },
            GuestAddr,
        },
        spin::Lazy,
        std::sync::Mutex,
    };

    static INIT_ONCE: Lazy<Mutex<()>> = Lazy::new(|| {
        {
            env_logger::init();
        };
        Mutex::new(())
    });

    fn get_tracking() -> GuestTracking {
        drop(INIT_ONCE.lock().unwrap());
        GuestTracking::new().unwrap()
    }

    #[test]
    #[cfg(feature = "guest")]
    fn test_max() {
        let mut tracking = get_tracking();
        assert_eq!(tracking.alloc(GuestAddr::MAX, 1), Ok(()));
    }

    #[test]
    #[cfg(feature = "guest")]
    fn test_out_of_bounds() {
        let mut tracking = get_tracking();
        assert_eq!(
            tracking.alloc(GuestAddr::MAX, 2),
            Err(GuestTrackingError::AddressRangeOverflow(GuestAddr::MAX, 2))
        );
    }

    #[test]
    #[cfg(feature = "guest")]
    fn test_track_identical() {
        let mut tracking = get_tracking();
        assert_eq!(tracking.alloc(0x1000, 0x1000), Ok(()));
        assert_eq!(
            tracking.alloc(0x1000, 0x1000),
            Err(GuestTrackingError::TrackingConflict(
                0x1000, 0x1000, 0x1000, 0x1000
            ))
        );
    }

    #[test]
    #[cfg(feature = "guest")]
    fn test_track_adjacent_after() {
        let mut tracking = get_tracking();
        assert_eq!(tracking.alloc(0x1000, 0x1000), Ok(()));
        assert_eq!(tracking.alloc(0x2000, 0x1000), Ok(()));
    }

    #[test]
    #[cfg(feature = "guest")]
    fn test_track_adjacent_before() {
        let mut tracking = get_tracking();
        assert_eq!(tracking.alloc(0x1000, 0x1000), Ok(()));
        assert_eq!(tracking.alloc(0x0000, 0x1000), Ok(()));
    }

    #[test]
    #[cfg(feature = "guest")]
    fn test_track_overlapping_start() {
        let mut tracking = get_tracking();
        assert_eq!(tracking.alloc(0x1000, 0x1000), Ok(()));
        assert_eq!(
            tracking.alloc(0x0000, 0x1001),
            Err(GuestTrackingError::TrackingConflict(
                0x1000, 0x1000, 0x0000, 0x1001
            ))
        );
    }

    #[test]
    #[cfg(feature = "guest")]
    fn test_track_overlapping_end() {
        let mut tracking = get_tracking();
        assert_eq!(tracking.alloc(0x1000, 0x1000), Ok(()));
        assert_eq!(
            tracking.alloc(0x1fff, 0x1001),
            Err(GuestTrackingError::TrackingConflict(
                0x1000, 0x1000, 0x1fff, 0x1001
            ))
        );
    }

    #[test]
    #[cfg(feature = "guest")]
    #[cfg(target_pointer_width = "64")]
    fn test_example_1() {
        let mut tracking = get_tracking();
        // alloc - start: 0xffffffffb5b5ff21, len: 0x3ff
        // alloc - start: 0xffffffffb5b60107, len: 0xdb
        assert_eq!(tracking.alloc(0xffffffffb5b5ff21, 0x3ff), Ok(()));
        assert_eq!(
            tracking.alloc(0xffffffffb5b60107, 0xdb),
            Err(GuestTrackingError::TrackingConflict(
                0xffffffffb5b5ff21,
                0x3ff,
                0xffffffffb5b60107,
                0xdb
            ))
        );
    }
}
