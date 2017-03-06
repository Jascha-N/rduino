#[macro_export]
macro_rules! rduino_main {
    ($f:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub extern "C" fn __rduino_main() -> ! {
            // FIXME: ! type is unstable
            // #[inline(always)]
            // fn __call_closure<F: Fn() -> ! + Send + Sync>(f: F) -> ! {
            //     f();
            // }

            // __call_closure($f)

            // workaround: ducktyping
            $f()
        }
    }
}

// #[macro_export]
// #[cfg(feature = "alloc")]
// macro_rules! rduino_isr_local {
//     ($f:expr) => {{
//         static __CLOSURE: $crate::sync::Mutex<Option<Box<Fn() + Send + Sync>>> = $crate::sync::Mutex::new(None);

//         extern fn __routine() {
//             let closure = __CLOSURE.lock();
//             let closure = closure.as_ref().unwrap();
//             closure();
//         }

//         *__CLOSURE.lock() = Some(Box::new($f));

//         $crate::interrupts::InterruptServiceRoutine::__new(__routine)
//     }}
// }

#[macro_export]
macro_rules! rduino_isr {
    ($f:expr) => {{
        extern "C" fn __routine() {
            #[inline(always)]
            fn __call_closure<F: Fn() + Send + Sync>(f: F) {
                f();
            }

            $crate::interrupts::__enter_isr();
            __call_closure($f);
            $crate::interrupts::__exit_isr();
        }

        $crate::interrupts::InterruptServiceRoutine::__new(__routine)
    }}
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::__print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! check_isr {
    ($fun:expr) => {
        if $crate::interrupts::inside_isr() {
            panic!("`{}` called from within an interrupt service routine", $fun);
        }
    }
}
