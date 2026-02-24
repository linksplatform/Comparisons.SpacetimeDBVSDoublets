//! Fork mechanism for isolated benchmark iterations.
//!
//! `Fork` wraps a `Benched` implementation and automatically calls
//! `unfork()` when dropped, resetting the database state between
//! benchmark iterations.

use crate::Benched;
use std::ops::{Deref, DerefMut};

/// A fork of a `Benched` instance that resets database state on drop.
///
/// Created by `Benched::fork()`. Provides direct access to the underlying
/// `Benched` type via `Deref`/`DerefMut`. On drop, calls `Benched::unfork()`
/// to clean up state for the next iteration.
pub struct Fork<'a, B: Benched> {
    benched: &'a mut B,
}

impl<'a, B: Benched> Fork<'a, B> {
    pub fn new(benched: &'a mut B) -> Self {
        Self { benched }
    }
}

impl<'a, B: Benched> Deref for Fork<'a, B> {
    type Target = B;

    fn deref(&self) -> &Self::Target {
        self.benched
    }
}

impl<'a, B: Benched> DerefMut for Fork<'a, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.benched
    }
}

impl<'a, B: Benched> Drop for Fork<'a, B> {
    fn drop(&mut self) {
        // SAFETY: Called only from Drop, ensuring no other references exist.
        unsafe { self.benched.unfork() };
    }
}
