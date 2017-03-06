//! Primitives for dealing with interrupts.

pub use sys::interrupts::{__enter_isr, __exit_isr};

use ffi::{self, RduinoInterruptMode};
use io;
use sys::interrupts;
use sys_common::sync::CriticalSection;

use core::mem;

/// Trait representing a type that can be used as an external interrupt.
///
/// This trait is typically implemented by `u8` which represents a raw external interrupt number and
/// `DigitalPin` which may possibly serve as an interrupt as well.
pub trait ToInterrupt {
    /// Convert this type into an external interrupt number if possible.
    fn to_interrupt(self) -> Option<u8>;
}

impl ToInterrupt for u8 {
    #[inline]
    fn to_interrupt(self) -> Option<u8> {
        Some(self)
    }
}



/// An interrupt service routine (or ISR).
///
/// An ISR can be constructed with the `rduino_isr!` macro.
#[derive(Debug)]
pub struct InterruptServiceRoutine {
    #[doc(hidden)]
    pub routine: extern fn()
}

impl InterruptServiceRoutine {
    #[doc(hidden)]
    #[inline]
    pub const fn __new(routine: extern fn()) -> InterruptServiceRoutine {
        InterruptServiceRoutine {
            routine: routine
        }
    }

    /// Attach this ISR to an interrupt.
    ///
    /// Attaches the interrupt service routine to an external interrupt using the given mode.
    /// In case of success, returns a guard that detaches the ISR when it goes out of scope.
    ///
    /// The `mode` parameter can either be an `InterruptMode` or an `InterruptModeExt` from the
    /// platform-specific extensions if the platform supports it.
    ///
    /// **Note**: If an ISR is already attached to the given interrupt, it is replaced.
    ///
    /// See also: [`attachInterrupt()`] from the Arduino reference.
    ///
    /// # Errors
    /// If the given interrupt is invalid. **Note**: The Arduino SDK does not provide a reliable
    /// way to find out if an interrupt is actually valid. So even if this method succeeds it is
    /// unfortunately still possible that it actually failed.
    ///
    /// # Panics
    /// Panics if this function is called from inside an interrupt service routine. Because the
    /// Arduino library provides no guarantees with regards to reentrancy, this is required to avoid
    /// an unsafe API.
    ///
    /// [`attachInterrupt()`]: https://www.arduino.cc/en/Reference/AttachInterrupt
    pub fn attach<I: ToInterrupt, M: Into<InterruptMode>>(self, interrupt: I, mode: M) -> io::Result<IsrGuard> {
        check_isr!("InterruptServiceRoutine::attach");

        let interrupt = interrupt.to_interrupt().map_or_else(|| Err(io::Error::InvalidInterruptPin), Ok)?;
        unsafe {
            ffi::rduino_attach_interrupt(interrupt, Some(self.routine), mode.into().to_ffi());
        }
        Ok(IsrGuard { interrupt: interrupt })
    }
}



/// A RAII implementation of a scoped interrupt service routine.
///
/// When this structure is dropped or `detach` is called, the ISR is detached.
#[must_use]
#[derive(Debug)]
pub struct IsrGuard {
    interrupt: u8
}

impl IsrGuard {
    /// Detach the interrupt service routine.
    ///
    /// This has the same effect as dropping the guard.
    pub fn detach(self) {
        mem::drop(self)
    }
}

impl Drop for IsrGuard {
    fn drop(&mut self) {
        unsafe {
            ffi::rduino_detach_interrupt(self.interrupt);
        }
    }
}

impl !Send for IsrGuard {}



/// A digital pin state or state transition that can trigger an interrupt.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum InterruptMode {
    /// Trigger an interrupt when the digital level of the pin is low.
    Low,

    /// Trigger an interrupt when the digital level of the pin changes.
    Change,

    /// Trigger an interrupt when the digital level changes from low to high.
    Rising,

    /// Trigger an interrupt when the digital level changes from high to low.
    Falling,

    #[doc(hidden)]
    __Raw(RduinoInterruptMode)
}

impl InterruptMode {
    #[inline]
    fn to_ffi(self) -> RduinoInterruptMode {
        match self {
            InterruptMode::Low => RduinoInterruptMode::Low,
            InterruptMode::Change => RduinoInterruptMode::Change,
            InterruptMode::Rising => RduinoInterruptMode::Rising,
            InterruptMode::Falling => RduinoInterruptMode::Falling,

            InterruptMode::__Raw(mode) => mode
        }
    }
}

/// Call a closure with interrupts disabled.
///
/// **Warning**: Code executed while the lock is held should be as short as possible to prevent any
/// interrupts that would have occurred from being mistimed or even entirely dropped.
///
/// **Warning**: If any functions that depend on interrupts being active are called inside of the
/// closure a deadlock might occur. Any functions *known* to depend on interrupts are documented.
#[inline]
pub fn without_interrupts<T, F: FnOnce() -> T>(f: F) -> T {
    let _guard = CriticalSection::enter();
    f()
}

/// Whether the code is called from inside an interrupt service routine.
#[inline]
pub fn inside_isr() -> bool {
    interrupts::inside_isr()
}
