use ffi::{self, RduinoAnalogReference, RduinoPinLevel, RduinoPinMode};
use interrupts::ToInterrupt;

/// Digital pin state.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum PinLevel {
    /// The state is low.
    Low,

    /// The state is high.
    High
}

impl PinLevel {
    fn from_ffi(level: RduinoPinLevel) -> PinLevel {
        match level {
            RduinoPinLevel::Low => PinLevel::Low,
            RduinoPinLevel::High => PinLevel::High
        }
    }

    fn to_ffi(self) -> RduinoPinLevel {
        match self {
            PinLevel::Low => RduinoPinLevel::Low,
            PinLevel::High => RduinoPinLevel::High
        }
    }
}

/// Pin modes.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum PinMode {
    /// Input mode.
    ///
    /// Generally used for reading values from the pin.
    Input,

    /// Input mode with pull-up.
    ///
    /// The same as `Input`, but with an internal pull-up resistor active.
    InputPullup,

    /// Output mode.
    ///
    /// Generally used for writing values to the pin.
    Output,

    #[doc(hidden)]
    __Raw(RduinoPinMode)
}

impl PinMode {
    #[inline]
    fn to_ffi(self) -> RduinoPinMode {
        match self {
            PinMode::Input => RduinoPinMode::Input,
            PinMode::InputPullup => RduinoPinMode::InputPullup,
            PinMode::Output => RduinoPinMode::Output,

            PinMode::__Raw(mode) => mode
        }
    }
}

/// Reference value for analog inputs.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum AnalogReference {
    /// The board's default reference voltage (`VCC`).
    ///
    /// Uses 5 volts for 5V boards or 3.3 volts for 3.3V boards.
    Default,

    /// External reference using the voltage applied to the `AREF` pin.
    External,

    /// Built-in internal reference voltage.
    ///
    /// The actual voltage depends on the board.
    Internal,

    #[doc(hidden)]
    __Raw(RduinoAnalogReference)
}

impl AnalogReference {
    #[inline]
    fn to_ffi(self) -> RduinoAnalogReference {
        match self {
            AnalogReference::Default => RduinoAnalogReference::Default,
            AnalogReference::External => RduinoAnalogReference::External,
            AnalogReference::Internal => RduinoAnalogReference::Internal,

            AnalogReference::__Raw(mode) => mode
        }
    }
}

impl Default for AnalogReference {
    #[inline]
    fn default() -> AnalogReference {
        AnalogReference::Default
    }
}

/// Trait implemented by all pin types.
pub trait Pin {
    /// The *digital* number of this pin.
    fn number(&self) -> u8;

    /// Set the mode of this pin.
    ///
    /// The parameter of this method can either be a `PinMode` or a `PinModeExt` from the
    /// platform-specific extensions if the platform supports it.
    #[inline]
    fn set_mode<M: Into<PinMode>>(&self, mode: M) {
        unsafe {
            ffi::rduino_pin_mode(self.number(), mode.into().to_ffi());
        }
    }
}

/// A digital pin.
///
/// Digital pin can be used to read and write digital values (high and low).
#[derive(Clone, Debug)]
pub struct DigitalPin {
    number: u8
}

impl DigitalPin {
    /// Set the digital value of this pin.
    #[inline]
    pub fn write(&self, value: PinLevel) {
        unsafe {
            ffi::rduino_digital_write(self.number, value.to_ffi());
        }
    }

    /// Get the value of this digital pin.
    #[inline]
    pub fn read(&self) -> PinLevel {
        PinLevel::from_ffi(unsafe { ffi::rduino_digital_read(self.number) })
    }

    /// Whether this pin can be used as a PWM output pin.
    #[inline]
    pub fn has_pwm(&self) -> bool {
        unsafe { ffi::rduino_digital_pin_has_pwm(self.number) }
    }

    /// Use this pin as a PWM output pin if supported.
    ///
    /// Returns the software analog output pin if this pin supports it.
    ///
    /// Pulse Width Module (PWM) can be used to simulate analog output on pins that are not
    /// connected to a Digital-to-Analog Converter (DAC).
    #[inline]
    pub fn to_pwm(&self) -> Option<AnalogOutputPin> {
        if self.has_pwm() {
            Some(AnalogOutputPin {
                number: self.number,
                kind: AnalogOutputKind::Software
            })
        } else {
            None
        }
    }
}

impl Pin for DigitalPin {
    #[inline]
    fn number(&self) -> u8 {
        self.number
    }
}

impl ToInterrupt for DigitalPin {
    fn to_interrupt(self) -> Option<u8> {
        ToInterrupt::to_interrupt(&self)
    }
}

impl<'a> ToInterrupt for &'a DigitalPin {
    fn to_interrupt(self) -> Option<u8> {
        let interrupt = unsafe { ffi::rduino_digital_pin_to_interrupt(self.number) };
        if interrupt >= 0 {
            Some(interrupt as u8)
        } else {
            None
        }
    }
}



/// An analog input pin.
///
/// Analog input pins can be used to read analog values through an Analog-to-Digital Converter (ADC).
#[derive(Clone, Debug)]
pub struct AnalogInputPin {
    number: u8
}

impl AnalogInputPin {
    /// Get the analog value for this pin.
    ///
    /// The value returned is between `0` (inclusive) to `2^n` (exclusive) where `n` is the current
    /// digital read resolution (see `set_resolution()`). These values corresponds to inputs between
    /// 0 volts and the voltage set by `set_reference()`.
    #[inline]
    pub fn read(&self) -> u16 {
        unsafe { ffi::rduino_analog_read(self.number) }
    }

    /// Set the resolution in number of bits for *all* analog input pins.
    #[inline]
    pub fn set_resolution(bits: u8) {
        unsafe {
            ffi::rduino_analog_read_resolution(bits);
        }
    }

    /// Set the reference voltage for *all* analog input pins.
    ///
    /// The parameter of this method can either be an `AnalogReference` or an `AnalogReferenceExt`
    /// from the platform-specific extensions if the platform supports it.
    #[inline]
    pub fn set_reference<R: Into<AnalogReference>>(reference: R) {
        unsafe {
            ffi::rduino_analog_reference(reference.into().to_ffi());
        }
    }
}

impl Pin for AnalogInputPin {
    #[inline]
    fn number(&self) -> u8 {
        self.number
    }
}



/// An analog output pin.
///
/// Analog output pins can be used to write analog values. The pin can either be a hardware analog
/// output using a DAC or a software analog output using PWM.
#[derive(Clone, Debug)]
pub struct AnalogOutputPin {
    number: u8,
    kind: AnalogOutputKind
}

impl AnalogOutputPin {
    /// The type of this pin (software or hardware).
    #[inline]
    pub fn kind(&self) -> AnalogOutputKind {
        self.kind
    }

    /// Set the analog value for this pin.
    ///
    /// The value must be between `0` (inclusive) to `2^n` (exclusive) where `n` is the current
    /// digital write resolution (see `set_resolution()`). Any values exceeding the maximum value
    /// are clamped.
    #[inline]
    pub fn write(&self, value: u16) {
        unsafe {
            ffi::rduino_analog_write(self.number, value);
        }
    }

    /// Set the resolution in number of bits for *all* analog output pins.
    #[inline]
    pub fn set_resolution(bits: u8) {
        unsafe {
            ffi::rduino_analog_write_resolution(bits);
        }
    }
}

impl Pin for AnalogOutputPin {
    #[inline]
    fn number(&self) -> u8 {
        self.number
    }
}



/// The type of an analog output pin.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum AnalogOutputKind {
    /// The pin uses software emulation (PWM).
    Software,

    /// The pin uses a hardware DAC.
    Hardware
}



/// Obtain a handle to the digital pin with the given number if it exists.
///
/// # Panics
/// Panics if this function is called from inside an interrupt service routine. Because the
/// Arduino library provides no guarantees with regards to reentrancy, this is required to avoid an
/// unsafe API.
#[inline]
pub fn digital_pin(number: u8) -> Option<DigitalPin> {
    check_isr!("digital_pin");

    if number as u32 >= ffi::RDUINO_NUM_DIGITAL_PINS {
        return None;
    }

    Some(DigitalPin { number: number })
}

/// Obtain a handle to the analog input pin with the given number if it exists.
///
/// On Arduino these pins are named `An` where `n` is the analog input number (e.g. `A3`).
///
/// # Panics
/// Panics if this function is called from inside an interrupt service routine. Because the
/// Arduino library provides no guarantees with regards to reentrancy, this is required to avoid an
/// unsafe API.
#[inline]
pub fn analog_input_pin(number: u8) -> Option<AnalogInputPin> {
    check_isr!("analog_input_pin");

    let number = unsafe { ffi::RDUINO_ANALOG_INPUT_PINS.get(number as usize).cloned() };
    number.map(|number| AnalogInputPin { number: number })
}

/// Obtain a handle to the *hardware* analog output pin with the given number if it exists.
///
/// On Arduino these pins are named `DACn` where `n` is the analog output number (e.g. `DAC0`). The
/// returned pin is always of the hardware (DAC) type.
///
/// # Panics
/// Panics if this function is called from inside an interrupt service routine. Because the
/// Arduino library provides no guarantees with regards to reentrancy, this is required to avoid an
/// unsafe API.
#[inline]
pub fn analog_output_pin(number: u8) -> Option<AnalogOutputPin> {
    check_isr!("analog_output_pin");

    let number = unsafe { ffi::RDUINO_ANALOG_OUTPUT_PINS.get(number as usize).cloned() };
    number.map(|number| AnalogOutputPin { number: number, kind: AnalogOutputKind::Hardware })
}
