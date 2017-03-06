pub use self::imp::*;

#[cfg(target_arch = "avr")]
#[path = "avr/mod.rs"]
mod imp;

#[cfg(target_arch = "arm")]
#[path = "arm/mod.rs"]
mod imp;
