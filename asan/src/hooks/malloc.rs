use {
    crate::{asan_alloc, size_t},
    core::{ffi::c_void, ptr::null_mut},
    log::trace,
};

/// # Safety
/// See man pages
#[cfg_attr(not(feature = "test"), no_mangle)]
#[cfg_attr(feature = "test", export_name = "patch_malloc")]
pub unsafe extern "C" fn malloc(size: size_t) -> *mut c_void {
    trace!("malloc - size: {:#x}", size);
    if size == 0 {
        null_mut()
    } else {
        asan_alloc(size, 0)
    }
}
