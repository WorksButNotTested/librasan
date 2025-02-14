#[cfg(test)]
#[cfg(all(feature = "hooks"))]
mod tests {
    use {asan::hooks::valloc::valloc, core::ptr::null_mut};

    #[test]
    fn test_valloc_zero() {
        let p = unsafe { valloc(0) };
        assert_eq!(p, null_mut());
    }

    #[test]
    fn test_valloc_buff() {
        let p = unsafe { valloc(10) };
        assert_ne!(p, null_mut());
        assert!(p as usize & 0xfff == 0);
    }
}
