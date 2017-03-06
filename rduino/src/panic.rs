//! Panic handling.

use sync::Mutex;
use sys::interrupts;

use core::fmt::Arguments;
use core::intrinsics;

static PANIC_HANDLER: Mutex<Option<fn(&PanicInfo)>> = Mutex::new(None);

/// A struct providing information about a panic.
///
/// A `PanicInfo` structure is passed to a panic hook set by the [`set_hook()`] function.
#[derive(Debug)]
pub struct PanicInfo<'a> {
    payload: Arguments<'a>,
    location: Location<'a>,
}

impl<'a> PanicInfo<'a> {
    /// The payload associated with the panic.
    pub fn payload(&self) -> &Arguments {
        &self.payload
    }

    /// Information about the location from which the panic originated, if available.
    pub fn location(&self) -> Option<&Location> {
        Some(&self.location)
    }
}

/// A struct containing information about the location of a panic.
///
/// This structure is created by the `PanicInfo::location` method.
#[derive(Debug)]
pub struct Location<'a> {
    file: &'a str,
    line: u32,
}

impl<'a> Location<'a> {
    /// The name of the source file from which the panic originated.
    pub fn file(&self) -> &str {
        self.file
    }

    /// The line number from which the panic originated.
    pub fn line(&self) -> u32 {
        self.line
    }
}

/// Register a custom panic hook, replacing any that was previously registered.
///
/// The panic hook is invoked when the main code panics, but not when the interrupt service routine
/// panics. When the panic hook returns the runtime aborts. By default no panic hook is installed
/// and the runtime simply aborts. Panics inside of an interrupt serivce routine *always* cause
/// the runtime to abort.
///
/// The hook is provided with a `PanicInfo` struct which contains information about the origin of
/// the panic, including the payload passed to `panic!` and the source code location from which the
/// panic originated.
///
/// **Warning**: Interrupts are disabled when a panic occurs, meaning that calling any functions
/// that rely on interrupts inside the panic hook might cause a deadlock.
#[inline]
pub fn set_hook(hook: fn(&PanicInfo)) {
    let mut handler = PANIC_HANDLER.lock();
    *handler = Some(hook);
}

/// Unregister the current panic hook.
///
/// Returns the current hook if one was installed.
#[inline]
pub fn take_hook() -> Option<fn(&PanicInfo)> {
    let mut handler = PANIC_HANDLER.lock();
    handler.take()
}

#[lang = "panic_fmt"]
extern fn rust_begin_panic(message: ::core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    unsafe {
        interrupts::disable_interrupts();
    }

    if !interrupts::inside_isr() {
        let handler = take_hook();
        if let Some(handler) = handler {
            let location = Location {
                file: file,
                line: line
            };
            let info = PanicInfo {
                payload: message,
                location: location
            };
            handler(&info);
        }
    }

    unsafe {
        intrinsics::abort();
    }
}
