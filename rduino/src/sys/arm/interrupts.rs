pub type Register = u32;

#[inline]
pub unsafe fn disable_interrupts() -> Register {
    let primask: u32;
    asm!("mrs $0, primask; cpsid i" : "=r"(primask) ::: "volatile");
    primask
}

#[inline]
pub unsafe fn restore_interrupts(prev_primask: Register) {
    asm!("msr primask, $0" :: "r"(prev_primask) :: "volatile");
}

#[inline]
pub fn inside_isr() -> bool {
    let ipsr: u32;
    unsafe {
        asm!("mrs $0, ipsr" : "=r"(ipsr) ::: "volatile");
    }
    ipsr > 0
}

#[inline]
#[doc(hidden)]
pub fn __enter_isr() {}

#[inline]
#[doc(hidden)]
pub fn __exit_isr() {}
