use core::intrinsics;

pub type Register = u8;

#[inline]
pub unsafe fn disable_interrupts() -> Register {
    let sreg: u8;
    asm!("in $0, sreg; cli" : "=r"(sreg) ::: "volatile");
    sreg
}

#[inline]
pub unsafe fn restore_interrupts(prev_sreq: Register) {
    asm!("out sreg, $0" :: "r"(prev_sreq) :: "volatile");
}

// FIXME: does not work for nested interrupts
static mut INSIDE_ISR: bool = false;

#[inline]
pub fn inside_isr() -> bool {
    unsafe { INSIDE_ISR }
}

#[inline]
#[doc(hidden)]
pub fn __enter_isr() {
    unsafe {
        INSIDE_ISR = true;
        intrinsics::atomic_singlethreadfence();
    }
}

#[inline]
#[doc(hidden)]
pub fn __exit_isr() {
    unsafe {
        INSIDE_ISR = false;
        intrinsics::atomic_singlethreadfence();
    }
}
