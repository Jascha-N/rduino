//! Timing related functions.
//!
//! **Note**: Many of these functions do not work properly when interrupts are disabled, which is
//! generally the case inside of interrupt service routines, when a mutex lock is held or inside
//! of a panic hook.

use ffi;

// pub enum SystemTime {
//     Millisecs(u32),
//     Microsecs(u32)
// }

/// The system time in milliseconds.
///
/// **Note**: This function might not return a reliable result when interrupts are disabled.
pub fn millisecs() -> u32 {
    unsafe { ffi::rduino_millis()}
}

/// The system time in microseconds.
///
/// **Note**: This function might not return a reliable result when interrupts are disabled.
pub fn microsecs() -> u32 {
    unsafe { ffi::rduino_micros() }
}

/// Wait until the given amount of milliseconds have passed.
///
/// **Warning**: This function might cause a deadlock when interrupts are disabled.
pub fn sleep_millisecs(ms: u32) {
    unsafe {
        ffi::rduino_delay(ms);
    }
}

/// Wait until the given amount of microseconds have passed.
pub fn sleep_microsecs(us: u32) {
    unsafe {
        ffi::rduino_delay_microseconds(us);
    }
}
