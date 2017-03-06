//! Synchronization primitives.

use sys_common::sync::CriticalSection;

use core::cell::UnsafeCell;
use core::fmt;
use core::ops;

/// A synchronization primitive for gaining exclusive access to some resource.
///
/// **Note**: Unlike mutexes in `std`, this mutex type does not implement poisoning, since unwinding
/// is not currently available on embedded platforms.
#[derive(Default)]
pub struct Mutex<T: ?Sized> {
    data: UnsafeCell<T>
}

impl<T> Mutex<T> {
    /// Create a new mutex holding the given initial value.
    pub const fn new(value: T) -> Mutex<T> {
        Mutex { data: UnsafeCell::new(value) }
    }
}

impl<T: ?Sized> Mutex<T> {
    /// Lock the mutex and gain access to the resource.
    ///
    /// Returns a guard that gives access to the resource and unlocks the mutex when it goes out of
    /// scope.
    ///
    /// **Warning**: Exclusive access is implemented through disabling *all* interrupts while the
    /// lock is held. This means that code executed while the lock is held should be as short as
    /// possible to prevent any interrupts that would have occurred from being mistimed or even
    /// entirely dropped.
    ///
    /// **Warning**: If any functions that depend on interrupts being active are called while the
    /// lock is held a deadlock might occur. Any functions *known* to depend on interrupts are
    /// documented.
    pub fn lock(&self) -> MutexGuard<T> {
        MutexGuard {
            data: &self.data,
            _lock: CriticalSection::enter(),
        }
    }
}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

impl<T: ?Sized + fmt::Debug> fmt::Debug for Mutex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let guard = self.lock();
        write!(f, "Mutex {{ data: {:?} }}", &*guard)
    }
}



/// A scoped guard for a locked mutex.
///
/// Access to the locked resource can be acquired through deref coercions. When the guard goes out
/// of scope, the lock is dropped.
///
/// **Warning**: Exclusive access is implemented through disabling *all* interrupts while the
/// lock is held. This means that code executed while the lock is held should be as short as
/// possible to prevent any interrupts that would have occurred from being mistimed or even
/// entirely dropped.
///
/// **Warning**: If any functions that depend on interrupts being active are called while the
/// lock is held a deadlock might occur. Any functions *known* to depend on interrupts are
/// documented.
#[must_use]
pub struct MutexGuard<'a, T: ?Sized + 'a> {
    data: &'a UnsafeCell<T>,
    _lock: CriticalSection
}

impl<'mutex, T: ?Sized + 'mutex> ops::Deref for MutexGuard<'mutex, T> {
    type Target =  T;

    fn deref(&self) -> &T {
        unsafe { &*self.data.get() }
    }
}

impl<'mutex, T: ?Sized + 'mutex> ops::DerefMut for MutexGuard<'mutex, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.data.get() }
    }
}
