//! SpacetimeDB vs Doublets benchmark library.
//!
//! Provides implementations for benchmarking SpacetimeDB and Doublets
//! storage systems on basic CRUD operations with links.

#![feature(allocator_api)]

pub mod benched;
pub mod doublets_impl;
pub mod exclusive;
pub mod fork;
pub mod spacetimedb_impl;

pub use benched::Benched;
pub use exclusive::Exclusive;
pub use fork::Fork;

use once_cell::sync::Lazy;
use std::env;

/// Number of links to use for benchmarking.
/// Configurable via `BENCHMARK_LINK_COUNT` environment variable.
/// Defaults to 1000 for main branch, 10 for PRs (controlled by CI).
pub static BENCHMARK_LINK_COUNT: Lazy<usize> = Lazy::new(|| {
    env::var("BENCHMARK_LINK_COUNT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000)
});

/// Number of background links to create before benchmarking to simulate a realistic database state.
/// Configurable via `BACKGROUND_LINK_COUNT` environment variable.
/// Defaults to 3000 for main branch, 100 for PRs (controlled by CI).
pub static BACKGROUND_LINK_COUNT: Lazy<usize> = Lazy::new(|| {
    env::var("BACKGROUND_LINK_COUNT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000)
});

/// A link structure representing a doublet (source -> target relationship).
/// Each link has a unique id, a source, and a target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Link {
    pub id: u64,
    pub source: u64,
    pub target: u64,
}

impl Link {
    #[must_use]
    pub fn new(id: u64, source: u64, target: u64) -> Self {
        Self { id, source, target }
    }
}

/// Shared trait for database operations on links.
/// Both SpacetimeDB and Doublets implement this interface.
pub trait Links {
    /// Create a link with given source and target. Returns the new link's id.
    fn create(&mut self, source: u64, target: u64) -> u64;

    /// Create a self-referential point link (id == source == target). Returns the new link's id.
    fn create_point(&mut self) -> u64 {
        let id = self.create(0, 0);
        self.update(id, id, id);
        id
    }

    /// Update an existing link's source and target.
    fn update(&mut self, id: u64, source: u64, target: u64);

    /// Delete a link by id.
    fn delete(&mut self, id: u64);

    /// Delete all links (used to reset state between benchmark iterations).
    fn delete_all(&mut self);

    /// Retrieve all links.
    fn query_all(&self) -> Vec<Link>;

    /// Retrieve a link by its id.
    fn query_by_id(&self, id: u64) -> Option<Link>;

    /// Retrieve all links with the given source.
    fn query_by_source(&self, source: u64) -> Vec<Link>;

    /// Retrieve all links with the given target.
    fn query_by_target(&self, target: u64) -> Vec<Link>;

    /// Retrieve all links matching both source and target.
    fn query_by_source_target(&self, source: u64, target: u64) -> Vec<Link>;

    /// Count all links in the database.
    fn count(&self) -> usize;
}
