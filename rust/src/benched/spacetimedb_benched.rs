//! `Benched` implementation for SpacetimeDB (SQLite backend).

use crate::{benched::Benched, spacetimedb_impl::SpacetimeDbLinks, Fork, Links};
use std::ops::{Deref, DerefMut};

/// Benchmark subject for SpacetimeDB with in-memory SQLite backend.
///
/// Uses SpacetimeDB's SQLite storage layer directly, which is the same
/// storage engine SpacetimeDB 2 uses internally for its tables.
pub struct SpacetimeDbMemoryBenched {
    links: SpacetimeDbLinks,
}

impl Benched for SpacetimeDbMemoryBenched {
    type Builder = ();

    fn setup(_builder: Self::Builder) -> Self {
        Self {
            links: SpacetimeDbLinks::new_memory(),
        }
    }

    fn fork(&mut self) -> Fork<Self> {
        Fork::new(self)
    }

    unsafe fn unfork(&mut self) {
        self.links.delete_all();
    }
}

impl Deref for SpacetimeDbMemoryBenched {
    type Target = SpacetimeDbLinks;

    fn deref(&self) -> &Self::Target {
        &self.links
    }
}

impl DerefMut for SpacetimeDbMemoryBenched {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.links
    }
}
