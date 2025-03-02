use {
    crate::{asan_panic, asan_store, size_t},
    core::{
        ffi::{c_char, c_void},
        ptr::write_bytes,
    },
    log::trace,
};

/// # Safety
/// See man pages
#[export_name = "patch_explicit_bzero"]
pub unsafe extern "C" fn explicit_bzero(s: *mut c_void, len: size_t) {
    trace!("explicit_bzero - s: {:p}, len: {:#x}", s, len);

    if len == 0 {
        return;
    }

    if s.is_null() {
        asan_panic(b"explicit_bzero - s is null".as_ptr() as *const c_char);
    }

    asan_store(s, len);
    write_bytes(s, 0, len);
}
