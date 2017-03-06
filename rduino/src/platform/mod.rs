#[cfg(arduino_arch = "avr")]
pub mod avr;

#[cfg(arduino_arch = "samd")]
pub mod samd;

#[doc(inline)]
pub mod raw {
    //! Raw platform-specific types.

    pub use libc::c_char;
    pub use libc::c_schar;
    pub use libc::c_uchar;

    pub use libc::c_short;
    pub use libc::c_ushort;

    pub use libc::c_int;
    pub use libc::c_uint;

    pub use libc::c_long;
    pub use libc::c_ulong;

    pub use libc::c_longlong;
    pub use libc::c_ulonglong;

    pub use libc::c_float;
    pub use libc::c_double;

    pub use libc::c_void;
}
