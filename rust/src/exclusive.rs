//! Thread-safe exclusive access wrapper.
//!
//! Provides `Exclusive<T>` which allows shared access to an `UnsafeCell<T>`
//! across thread boundaries, bypassing Rust's borrow checker.
//! Used to allow sharing mutable state between benchmark iterations.

use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

/// A wrapper that provides exclusive mutable access to an inner value
/// via `UnsafeCell`. Implements `Sync` to allow sharing across threads.
///
/// # Safety
/// The caller must ensure that only one thread accesses the inner value at a time.
pub struct Exclusive<T> {
    inner: UnsafeCell<T>,
}

impl<T> Exclusive<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
        }
    }

    /// Get a mutable reference to the inner value.
    ///
    /// # Safety
    /// Caller must ensure exclusive access.
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn get_mut(&self) -> &mut T {
        &mut *self.inner.get()
    }
}

impl<T> Deref for Exclusive<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner.get() }
    }
}

impl<T> DerefMut for Exclusive<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.get_mut()
    }
}

// SAFETY: Benchmarks run single-threaded; Exclusive access is manually enforced.
unsafe impl<T> Send for Exclusive<T> {}
unsafe impl<T> Sync for Exclusive<T> {}
