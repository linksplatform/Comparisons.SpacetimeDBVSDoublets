//! Benched trait and implementations for SpacetimeDB and Doublets.
//!
//! The `Benched` trait defines the lifecycle for benchmark subjects:
//! 1. `setup()` — construct the database and prepare it for benchmarking
//! 2. `fork()` — create a `Fork` wrapper for a single iteration
//! 3. `unfork()` — reset state after each iteration (called by `Fork::drop`)

mod doublets_benched;
mod spacetimedb_benched;

pub use doublets_benched::*;
pub use spacetimedb_benched::*;

use crate::Fork;

/// Lifecycle trait for benchmark subjects.
///
/// Implementors wrap a database and manage its state across iterations.
/// The fork/unfork pattern ensures each benchmark iteration starts from
/// a consistent baseline state (background links pre-populated).
pub trait Benched: Sized {
    /// The builder type used to construct this benched subject.
    type Builder;

    /// Set up the database and return a ready-to-benchmark instance.
    fn setup(builder: Self::Builder) -> Self;

    /// Create a fork for a single isolated benchmark iteration.
    fn fork(&mut self) -> Fork<Self>;

    /// Reset database state after an iteration (called by `Fork::drop`).
    ///
    /// # Safety
    /// Must only be called from `Fork::drop`.
    unsafe fn unfork(&mut self);
}
