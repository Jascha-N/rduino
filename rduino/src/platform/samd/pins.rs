//! SAMD-specific extensions to the `rduino::pins` module.

use ffi::{RduinoAnalogReference, RduinoPinMode};
use pins::{AnalogReference, PinMode};

/// Additional SAMD-specific pin modes.
///
/// Can be used with `Pin::set_mode`.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum PinModeExt {
    /// Use the internal pulldown resistor.
    InputPulldown
}

impl From<PinModeExt> for PinMode {
    #[inline]
    fn from(_: PinModeExt) -> PinMode {
        PinMode::__Raw(RduinoPinMode::InputPulldown)
    }
}

/// Additional SAMD-specific analog reference modes.
///
/// Can be used with `AnalogPin::set_reference`.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum AnalogReferenceExt {
    /// 1.0V internal reference.
    Internal1v0,

    /// 1.65V internal reference.
    Internal1v65,

    /// 2.23V internal reference.
    Internal2v23
}

impl From<AnalogReferenceExt> for AnalogReference {
    #[inline]
    fn from(reference: AnalogReferenceExt) -> AnalogReference {
        let raw = match reference {
            AnalogReferenceExt::Internal1v0 => RduinoAnalogReference::Internal1v0,
            AnalogReferenceExt::Internal1v65 => RduinoAnalogReference::Internal1v65,
            AnalogReferenceExt::Internal2v23 => RduinoAnalogReference::Internal2v23,
        };
        AnalogReference::__Raw(raw)
    }
}
