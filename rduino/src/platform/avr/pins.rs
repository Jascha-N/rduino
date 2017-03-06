//! AVR-specific extensions to the `rduino::pins` module.

use ffi::RduinoAnalogReference;
use pins::AnalogReference;

/// Additional AVR-specific analog reference modes.
///
/// Can be used with `AnalogPin::set_reference`.
///
/// **NOTE**: Not all modes are supported by all AVR boards.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum AnalogReferenceExt {
    /// 1.1V internal reference.
    #[cfg(any(
        arduino_mcu = "attiny24",
        arduino_mcu = "attiny44",
        arduino_mcu = "attiny84",

        arduino_mcu = "attiny25",
        arduino_mcu = "attiny45",
        arduino_mcu = "attiny85",

        arduino_mcu = "atmega1280",
        arduino_mcu = "atmega2560",
        arduino_mcu = "atmega1284",
        arduino_mcu = "atmega1284p",
        arduino_mcu = "atmega644",
        arduino_mcu = "atmega644a",
        arduino_mcu = "atmega644p",
        arduino_mcu = "atmega644pa"
    ))]
    Internal1v1,

    /// 2.56V internal reference.
    #[cfg(any(
        arduino_mcu = "attiny25",
        arduino_mcu = "attiny45",
        arduino_mcu = "attiny85",

        arduino_mcu = "atmega1280",
        arduino_mcu = "atmega2560",
        arduino_mcu = "atmega1284",
        arduino_mcu = "atmega1284p",
        arduino_mcu = "atmega644",
        arduino_mcu = "atmega644a",
        arduino_mcu = "atmega644p",
        arduino_mcu = "atmega644pa"
    ))]
    Internal2v56,

    /// Not documented by Arduino.
    #[cfg(any(
        arduino_mcu = "attiny25",
        arduino_mcu = "attiny45",
        arduino_mcu = "attiny85"
    ))]
    Internal2v56ExtCap
}

impl From<AnalogReferenceExt> for AnalogReference {
    #[inline]
    fn from(reference: AnalogReferenceExt) -> AnalogReference {
        let raw = match reference {
            AnalogReferenceExt::Internal1v1 => RduinoAnalogReference::Internal1v1,
            AnalogReferenceExt::Internal2v56 => RduinoAnalogReference::Internal2v56,
            AnalogReferenceExt::Internal2v56ExtCap => RduinoAnalogReference::Internal2v56ExtCap
        };
        AnalogReference::__Raw(raw)
    }
}
