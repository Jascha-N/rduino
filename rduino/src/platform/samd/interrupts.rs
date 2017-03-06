//! SAMD-specific extensions to the `rduino::interrupts` module.

use ffi::RduinoInterruptMode;
use interrupts::InterruptMode;

/// Additional SAMD-specific interrupt modes.
///
/// Can be used with `InterruptServiceRoutine::attach`.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum InterruptModeExt {
    /// Trigger when pin level is high.
    High
}

impl From<InterruptModeExt> for InterruptMode {
    #[inline]
    fn from(_: InterruptModeExt) -> InterruptMode {
        InterruptMode::__Raw(RduinoInterruptMode::High)
    }
}
