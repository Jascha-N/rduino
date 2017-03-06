//! Traits, helpers and type definitions for core I/O functionality.

use serial::{Serial, SerialPort};

#[cfg(feature = "alloc")]
use collections::Vec;

use core::fmt;
use core::result;

pub mod prelude {
    pub use super::{Read, Stream, Write};
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Error {
    WriteZero,
    UnexpectedEof,
    InvalidInterruptPin,
    UnsupportedSerialMode,
    Other,

    #[doc(hidden)]
    __NonExhaustive,
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::WriteZero => write!(fmt, "write zero"),
            Error::UnexpectedEof => write!(fmt, "unexpected end of file"),
            Error::InvalidInterruptPin => write!(fmt, "invalid interrupt pin"),
            Error::UnsupportedSerialMode => write!(fmt, "unsupported serial mode"),
            Error::Other => write!(fmt, "other error"),
            Error::__NonExhaustive => unreachable!()
        }
    }
}

/// A trait for Arduino objects that can act as both as a byte-oriented input and output stream.
///
/// All types implementing `Stream` will also implement the `Read` and `Write` traits. Using these
/// traits over `Serial` is preferred, unless the required method is not available otherwise, e.g.
/// `set_read_timeout` is only available in `Stream`.
///
/// For more information see the Arduino reference on the [`Stream` class][Stream]. This trait also
/// contains some methods from the `Print` class (which is unfortunately not documented by Arduino).
///
/// [Stream]: https://www.arduino.cc/en/Reference/Stream
pub trait Stream {
    /// The number of bytes available for reading.
    fn available(&self) -> usize;

    /// Set the time to wait for data to become available in milliseconds.
    fn set_read_timeout(&self, timeout_ms: u32);

    /// Try to read one byte.
    ///
    /// Returns `Ok(byte)` if a byte was read or `None` if no data is available. This method will
    /// never block.
    fn read_byte(&self) -> Option<u8>;

    /// Peek at the first byte in the stream.
    ///
    /// Returns `Ok(byte)` if there was at least one byte available or `None` otherwise. This method
    /// does not remove the byte from the stream and will never block.
    fn peek_byte(&self) -> Option<u8>;

    /// Read some bytes into the given buffer.
    ///
    /// Returns the number of bytes read. This method might block until data becomes available or
    /// the operation times out.
    fn read_bytes(&self, buf: &mut [u8]) -> usize;

    /// Read some bytes into the given buffer.
    ///
    /// Returns the number of bytes read. Unlike `read_bytes`, this method will not try to block
    /// until enough data becomes available, but only reads the data that is _immediately_
    /// available, thus also ignoring any time-out setting. The maximum number of bytes read
    /// typically depends on the size of the board's internal input buffer for this stream, e.g. the
    /// serial input buffer.
    #[inline]
    fn read_bytes_immediately(&self, buf: &mut [u8]) -> usize {
        let available = self.available();
        if available > buf.len() {
            self.read_bytes(buf)
        } else {
            self.read_bytes(&mut buf[..available])
        }
    }

    /// Try to write one byte to the stream.
    ///
    /// Returns whether the byte was written. This method might block.
    #[inline]
    fn write_byte(&self, byte: u8) -> bool {
        self.write_bytes(&[byte]) == 1
    }

    /// Write some bytes to the stream.
    ///
    /// Returns the number of bytes written. This method might block.
    fn write_bytes(&self, buf: &[u8]) -> usize;

    /// Flushes any buffered output, ensuring these contents reach their destination.
    fn flush(&self);
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, mut buf: &[u8]) -> Result<()> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => {
                    return Err(Error::WriteZero);
                }
                Ok(n) => {
                    buf = &buf[n..];
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()> {
        struct Adaptor<'a, T: ?Sized + 'a> {
            inner: &'a mut T,
            error: Option<Error>,
        }

        impl<'a, T: Write + ?Sized> fmt::Write for Adaptor<'a, T> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.inner.write_all(s.as_bytes()).map_err(|error| {
                    self.error = Some(error);
                    fmt::Error
                })
            }
        }

        let mut output = Adaptor { inner: self, error: None };
        fmt::write(&mut output, fmt).map_err(|_| {
            if let Some(error) = output.error {
                error
            } else {
                Error::Other
            }
        })
    }

    fn by_ref(&mut self) -> &mut Self where Self: Sized {
        self
    }
}

impl<T: Stream + ?Sized> Write for T {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(self.write_bytes(buf))
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        Stream::flush(self);
        Ok(())
    }
}

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    #[cfg(feature = "alloc")]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        read_to_end(self, buf)
    }

    // TODO: implement me
    // #[cfg(feature = "alloc")]
    // fn read_to_string(&mut self, _buf: &mut String) -> Result<usize> {
    //     unimplemented!()
    // }

    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()> {
        while !buf.is_empty() {
            match self.read(buf) {
                Ok(0) => {
                    return Err(Error::UnexpectedEof);
                }
                Ok(n) => {
                    let tmp = buf;
                    buf = &mut tmp[n..];
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    #[inline]
    fn by_ref(&mut self) -> &mut Self where Self: Sized {
        self
    }
}

impl<T: Stream + ?Sized> Read for T {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        Ok(self.read_bytes(buf))
    }
}

#[cfg(feature = "alloc")]
fn read_to_end<R: Read + ?Sized>(r: &mut R, buf: &mut Vec<u8>) -> Result<usize> {
    let start_len = buf.len();
    let mut len = start_len;
    let mut new_write_size = 8;

    let result;
    loop {
        if len == buf.len() {
            if new_write_size < 64 {
                new_write_size *= 2;
            }
            buf.resize(len + new_write_size, 0);
        }

        match r.read(&mut buf[len..]) {
            Ok(0) => {
                result = Ok(len - start_len);
                break;
            }
            Ok(n) => {
                len += n;
            }
            Err(e) => {
                result = Err(e);
                break;
            }
        }
    }

    buf.truncate(len);
    result
}

#[doc(hidden)]
pub fn __print(args: fmt::Arguments) {
    let mut serial = Serial::open(SerialPort::Monitor).unwrap_or_default();
    if let Err(error) = serial.write_fmt(args) {
        panic!("failed printing to serial monitor: {}", error);
    }
}
