#![feature(untagged_unions)]
#![allow(approx_constant, bad_style, expl_impl_clone_on_copy, improper_ctypes, unknown_lints)]
#![no_std]

use core::fmt;

#[cfg(target_arch = "arm")]
mod platform {
    pub type c_char = u8;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_double = f64;
}

#[cfg(target_arch = "avr")]
mod platform {
    pub type c_char = i8;
    pub type c_int = i16;
    pub type c_uint = u16;
    pub type c_double = f32;
}

pub type c_char = platform::c_char;
pub type c_schar = i8;
pub type c_uchar = u8;

pub type c_short = i16;
pub type c_ushort = u16;

pub type c_int = platform::c_int;
pub type c_uint = platform::c_uint;

pub type c_long = i32;
pub type c_ulong = u32;

pub type c_longlong = i64;
pub type c_ulonglong = u64;

pub type c_float = f32;
pub type c_double = platform::c_double;

#[repr(u8)]
pub enum c_void {
    #[doc(hidden)] __variant1,
    #[doc(hidden)] __variant2
}

impl fmt::Debug for c_void {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("c_void")
    }
}

include!(concat!(env!("OUT_DIR"), "/libc.rs"));
