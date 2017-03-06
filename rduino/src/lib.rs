#![no_std]

#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(core_float)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(optin_builtin_traits)]
#![cfg_attr(feature = "alloc", feature(alloc, collections))]

//#![warn(missing_docs)]

extern crate compiler_builtins;
extern crate libc_arduino as libc;

#[cfg(feature = "alloc")]
extern crate alloc_arduino;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
extern crate collections;

pub use sys_common::{__rduino_setup, __rduino_loop};

#[macro_use]
mod macros;

pub mod ffi;
pub mod interrupts;
pub mod io;
pub mod num;
pub mod panic;
pub mod platform;
pub mod pins;
pub mod serial;
pub mod sync;
pub mod time;

mod sys;
mod sys_common;

pub mod prelude {
    #[doc(no_inline)]
    pub use num::Float;
    pub use pins::Pin;

    #[cfg(feature = "alloc")]
    #[doc(no_inline)]
    pub use alloc::boxed::Box;

    #[cfg(feature = "alloc")]
    #[doc(no_inline)]
    pub use alloc::rc::Rc;

    #[cfg(feature = "alloc")]
    #[doc(no_inline)]
    pub use collections::{String, Vec};
}
