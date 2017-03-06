use ffi::{self, RduinoSerial, RduinoSerialConfig};
use io;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum CharBits {
    Five,
    Six,
    Seven,
    Eight,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Parity {
    None,
    Even,
    Odd,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum StopBits {
    One,
    Two
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct SerialMode {
    speed: u32,
    char_size: CharBits,
    parity: Parity,
    stop_bits: StopBits
}

impl SerialMode {
    pub fn new() -> SerialMode {
        SerialMode {
            speed: 9600,
            char_size: CharBits::Eight,
            parity: Parity::None,
            stop_bits: StopBits::One
        }
    }

    #[inline]
    pub fn speed(mut self, speed: u32) -> SerialMode {
        self.speed = speed;
        self
    }

    #[inline]
    pub fn char_size(mut self, char_size: CharBits) -> SerialMode {
        self.char_size = char_size;
        self
    }

    #[inline]
    pub fn parity(mut self, parity: Parity) -> SerialMode {
        self.parity = parity;
        self
    }

    #[inline]
    pub fn stop_bits(mut self, stop_bits: StopBits) -> SerialMode {
        self.stop_bits = stop_bits;
        self
    }

    fn to_ffi(self) -> (u32, RduinoSerialConfig) {
        let mode = match (self.char_size, self.parity, self.stop_bits) {
            (CharBits::Five, Parity::None, StopBits::One) => RduinoSerialConfig::Serial5N1,
            (CharBits::Six, Parity::None, StopBits::One) => RduinoSerialConfig::Serial6N1,
            (CharBits::Seven, Parity::None, StopBits::One) => RduinoSerialConfig::Serial7N1,
            (CharBits::Eight, Parity::None, StopBits::One) => RduinoSerialConfig::Serial8N1,
            (CharBits::Five, Parity::None, StopBits::Two) => RduinoSerialConfig::Serial5N2,
            (CharBits::Six, Parity::None, StopBits::Two) => RduinoSerialConfig::Serial6N2,
            (CharBits::Seven, Parity::None, StopBits::Two) => RduinoSerialConfig::Serial7N2,
            (CharBits::Eight, Parity::None, StopBits::Two) => RduinoSerialConfig::Serial8N2,
            (CharBits::Five, Parity::Even, StopBits::One) => RduinoSerialConfig::Serial5E1,
            (CharBits::Six, Parity::Even, StopBits::One) => RduinoSerialConfig::Serial6E1,
            (CharBits::Seven, Parity::Even, StopBits::One) => RduinoSerialConfig::Serial7E1,
            (CharBits::Eight, Parity::Even, StopBits::One) => RduinoSerialConfig::Serial8E1,
            (CharBits::Five, Parity::Even, StopBits::Two) => RduinoSerialConfig::Serial5E2,
            (CharBits::Six, Parity::Even, StopBits::Two) => RduinoSerialConfig::Serial6E2,
            (CharBits::Seven, Parity::Even, StopBits::Two) => RduinoSerialConfig::Serial7E2,
            (CharBits::Eight, Parity::Even, StopBits::Two) => RduinoSerialConfig::Serial8E2,
            (CharBits::Five, Parity::Odd, StopBits::One) => RduinoSerialConfig::Serial5O1,
            (CharBits::Six, Parity::Odd, StopBits::One) => RduinoSerialConfig::Serial6O1,
            (CharBits::Seven, Parity::Odd, StopBits::One) => RduinoSerialConfig::Serial7O1,
            (CharBits::Eight, Parity::Odd, StopBits::One) => RduinoSerialConfig::Serial8O1,
            (CharBits::Five, Parity::Odd, StopBits::Two) => RduinoSerialConfig::Serial5O2,
            (CharBits::Six, Parity::Odd, StopBits::Two) => RduinoSerialConfig::Serial6O2,
            (CharBits::Seven, Parity::Odd, StopBits::Two) => RduinoSerialConfig::Serial7O2,
            (CharBits::Eight, Parity::Odd, StopBits::Two) => RduinoSerialConfig::Serial8O2
        };
        (self.speed, mode)
    }
}

impl Default for SerialMode {
    #[inline]
    fn default() -> SerialMode {
        SerialMode::new()
    }
}

#[derive(Clone, Debug)]
pub struct Serial {
    inner: *mut RduinoSerial
}

impl Serial {
    #[inline]
    fn new(inner: *mut RduinoSerial) -> Serial {
        Serial { inner: inner }
    }

    #[inline]
    pub fn open(port: SerialPort) -> Option<Serial> {
        check_isr!("serial");
        port.open()
    }

    #[inline]
    pub fn reset(&self, mode: SerialMode) -> io::Result<()> {
        let (speed, mode) = mode.to_ffi();
        if unsafe { ffi::rduino_serial_begin(self.inner, speed, mode) } {
            Ok(())
        } else {
            Err(io::Error::UnsupportedSerialMode)
        }
    }

    #[inline]
    pub fn ready(&self) -> bool {
        unsafe { ffi::rduino_serial_ready(self.inner) }
    }

    #[inline]
    pub fn available_for_write(&self) -> usize {
        unsafe { ffi::rduino_serial_available_for_write(self.inner) }
    }

    #[inline]
    pub fn wait(&self) {
        while !self.ready() {}
    }

    #[inline]
    pub fn close(self) {
        unsafe {
            ffi::rduino_serial_end(self.inner);
        }
    }
}

impl !Send for Serial {}
impl !Sync for Serial {}

impl Default for Serial {
    #[inline]
    fn default() -> Serial {
        Serial { inner: unsafe { ffi::rduino_serial_default() } }
    }
}

impl io::Stream for Serial {
    #[inline]
    fn available(&self) -> usize {
        unsafe { ffi::rduino_serial_available(self.inner) }
    }

    #[inline]
    fn set_read_timeout(&self, timeout_ms: u32) {
        unsafe {
            ffi::rduino_serial_set_timeout(self.inner, timeout_ms);
        }
    }

    #[inline]
    fn read_byte(&self) -> Option<u8> {
        let value = unsafe { ffi::rduino_serial_read(self.inner) };
        if value >= 0 {
            Some(value as u8)
        } else {
            None
        }
    }

    #[inline]
    fn read_bytes(&self, buf: &mut [u8]) -> usize {
        unsafe { ffi::rduino_serial_read_bytes(self.inner, buf.as_mut_ptr(), buf.len()) }
    }

    #[inline]
    fn peek_byte(&self) -> Option<u8> {
        let value = unsafe { ffi::rduino_serial_peek(self.inner) };
        if value >= 0 {
            Some(value as u8)
        } else {
            None
        }
    }

    #[inline]
    fn write_byte(&self, byte: u8) -> bool {
        unsafe { ffi::rduino_serial_write(self.inner, byte) }
    }

    #[inline]
    fn write_bytes(&self, buf: &[u8]) -> usize {
        if buf.is_empty() {
            0
        } else {
            unsafe { ffi::rduino_serial_write_bytes(self.inner, buf.as_ptr(), buf.len()) }
        }
    }

    #[inline]
    fn flush(&self) {
        unsafe {
            ffi::rduino_serial_flush(self.inner);
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum SerialPort {
    UsbVirtual,
    Monitor,
    LinuxBridge,
    Hardware,
    HardwareOpen
}

impl SerialPort {
    fn open(&self) -> Option<Serial> {
        let ffi_serial = match *self {
            SerialPort::UsbVirtual => ffi::rduino_serial_usbvirtual,
            SerialPort::Monitor => ffi::rduino_serial_monitor,
            SerialPort::LinuxBridge => ffi::rduino_serial_linuxbridge,
            SerialPort::Hardware => ffi::rduino_serial_hardware,
            SerialPort::HardwareOpen => ffi::rduino_serial_hardware_open
        };
        let serial = unsafe { ffi_serial() };
        if serial.is_null() {
            None
        } else {
            Some(Serial::new(serial))
        }
    }
}
