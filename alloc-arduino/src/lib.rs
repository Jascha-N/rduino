#![feature(allocator)]
#![allocator]
#![no_std]

extern crate libc_arduino as libc;

#[no_mangle]
#[doc(hidden)]
pub extern fn __rust_allocate(size: usize, _align: usize) -> *mut u8 {
    unsafe {
        libc::malloc(size) as *mut u8
    }
}

#[no_mangle]
#[doc(hidden)]
pub extern fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
    unsafe {
        libc::free(ptr as *mut libc::c_void)
    }
}

#[no_mangle]
#[doc(hidden)]
pub extern fn __rust_reallocate(ptr: *mut u8, _old_size: usize, size: usize,
                                _align: usize) -> *mut u8 {
    unsafe {
        libc::realloc(ptr as *mut libc::c_void, size) as *mut u8
    }
}

#[no_mangle]
#[doc(hidden)]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize,
                                        _size: usize, _align: usize) -> usize {
    old_size
}

#[no_mangle]
#[doc(hidden)]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}
