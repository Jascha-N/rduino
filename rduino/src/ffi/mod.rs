//! Raw FFI bindings.
//!
//! **Warning**: This module should only be used by library developers if at all.
#![allow(bad_style, missing_docs, unused)]

include!(concat!(env!("OUT_DIR"), "/rduino.rs"));
