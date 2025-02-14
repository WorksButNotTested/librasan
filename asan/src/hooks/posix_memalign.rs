use {
    crate::{
        hooks::{asan_alloc, asan_panic, size_t},
        GuestAddr,
    },
    core::{
        ffi::{c_char, c_int, c_void},
        mem::size_of,
        ptr::null_mut,
    },
    log::trace,
};

/// # Safety
/// See man pages
#[no_mangle]
#[cfg_attr(feature = "test", export_name = "patch_posix_memalign")]
pub unsafe extern "C" fn posix_memalign(
    memptr: *mut *mut c_void,
    align: size_t,
    size: size_t,
) -> c_int {
    trace!(
        "posix_memalign - memptr: {:p}, align: {:#x}, size: {:#x}",
        memptr,
        align,
        size
    );

    if memptr.is_null() {
        asan_panic(c"posix_memalign - memptr is null".as_ptr() as *const c_char);
    }

    fn is_power_of_two(n: size_t) -> bool {
        n != 0 && (n & (n - 1)) == 0
    }

    if align % size_of::<GuestAddr>() != 0 {
        asan_panic(
            c"posix_memalign - align is not a multiple of pointer size".as_ptr() as *const c_char,
        );
    } else if !is_power_of_two(align) {
        asan_panic(c"posix_memalign - align is not a power of two".as_ptr() as *const c_char);
    } else if size == 0 {
        *memptr = null_mut();
        0
    } else {
        let p = asan_alloc(size, align);
        *memptr = p;
        0
    }
}
