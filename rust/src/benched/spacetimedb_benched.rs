//! `Benched` implementation for SpacetimeDB.

use crate::{benched::Benched, spacetimedb_impl::SpacetimeDbLinks, Fork, Links};
use std::ops::{Deref, DerefMut};

/// Benchmark subject for SpacetimeDB using the official `spacetimedb-sdk` Rust crate.
///
/// Connects to a running SpacetimeDB server and benchmarks link operations
/// via the official client SDK.
pub struct SpacetimeDbBenched {
    links: SpacetimeDbLinks,
}

impl Benched for SpacetimeDbBenched {
    type Builder = ();

    fn setup(_builder: Self::Builder) -> Self {
        Self {
            links: SpacetimeDbLinks::connect(),
        }
    }

    fn fork(&mut self) -> Fork<Self> {
        Fork::new(self)
    }

    unsafe fn unfork(&mut self) {
        self.links.delete_all();
    }
}

impl Deref for SpacetimeDbBenched {
    type Target = SpacetimeDbLinks;

    fn deref(&self) -> &Self::Target {
        &self.links
    }
}

impl DerefMut for SpacetimeDbBenched {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.links
    }
}
