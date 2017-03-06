use core::intrinsics;

pub mod sync;

extern {
    // FIXME: Workaround for https://github.com/rust-lang/rust/issues/28728
    // The actual type is extern fn() -> !, but empty loops in user code get optimized away
    fn __rduino_main();
}

#[export_name = "setup"]
#[doc(hidden)]
pub extern "C" fn __rduino_setup() {
    unsafe {
        __rduino_main();

        // FIXME: Workaround for https://github.com/rust-lang/rust/issues/28728
        // Force empty loop to not get optimized away
        loop {
            asm!("" :::: "volatile");
        }
    }
}

#[export_name = "loop"]
#[doc(hidden)]
pub extern "C" fn __rduino_loop() {
    // This code should never run; if it does, abort
    unsafe {
        intrinsics::abort();
    }
}
