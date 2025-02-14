use {
    crate::hooks::{asan_panic, asan_store, size_t},
    core::{
        ffi::{c_char, c_int, c_void},
        ptr::write_bytes,
    },
    log::trace,
};

/// # Safety
/// See man pages
#[no_mangle]
#[export_name = "patch_memset"]
pub unsafe extern "C" fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void {
    trace!("memset - dest: {:p}, c: {:#x}, n: {:#x}", dest, c, n);

    if n == 0 {
        return dest;
    }

    if dest.is_null() {
        asan_panic(c"memset - dest is null".as_ptr() as *const c_char);
    }

    asan_store(dest, n);
    write_bytes(dest, c as u8, n);
    dest
}
