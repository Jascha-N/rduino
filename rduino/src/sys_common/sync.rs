use sys::interrupts::{self, Register};

use core::sync::atomic::{self, Ordering};

#[must_use]
pub struct CriticalSection {
    prev_reg: Register
}

impl CriticalSection {
    #[inline]
    pub fn enter() -> CriticalSection {
        let prev_reg;
        unsafe {
            prev_reg = interrupts::disable_interrupts();
        }
        atomic::fence(Ordering::Acquire);
        CriticalSection {
            prev_reg: prev_reg
        }
    }
}

impl Drop for CriticalSection {
    #[inline]
    fn drop(&mut self) {
        atomic::fence(Ordering::Release);
        unsafe {
            interrupts::restore_interrupts(self.prev_reg);
        }
    }
}

impl !Send for CriticalSection {}
