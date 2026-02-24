//! `Benched` implementations for Doublets stores.

use crate::{
    benched::Benched,
    doublets_impl::{
        create_split_volatile, create_united_volatile, DoubletsLinks, DoubletsSplitVolatile,
        DoubletsUnitedVolatile,
    },
    Fork, Links,
};
use std::ops::{Deref, DerefMut};

/// Benchmark subject for Doublets united volatile (in-memory, contiguous) store.
pub struct DoubletsUnitedVolatileBenched {
    links: DoubletsLinks<DoubletsUnitedVolatile>,
}

impl Benched for DoubletsUnitedVolatileBenched {
    type Builder = ();

    fn setup(_builder: Self::Builder) -> Self {
        Self {
            links: create_united_volatile(),
        }
    }

    fn fork(&mut self) -> Fork<Self> {
        Fork::new(self)
    }

    unsafe fn unfork(&mut self) {
        self.links.delete_all();
    }
}

impl Deref for DoubletsUnitedVolatileBenched {
    type Target = DoubletsLinks<DoubletsUnitedVolatile>;

    fn deref(&self) -> &Self::Target {
        &self.links
    }
}

impl DerefMut for DoubletsUnitedVolatileBenched {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.links
    }
}

/// Benchmark subject for Doublets split volatile (in-memory, separate data/index) store.
pub struct DoubletsSplitVolatileBenched {
    links: DoubletsLinks<DoubletsSplitVolatile>,
}

impl Benched for DoubletsSplitVolatileBenched {
    type Builder = ();

    fn setup(_builder: Self::Builder) -> Self {
        Self {
            links: create_split_volatile(),
        }
    }

    fn fork(&mut self) -> Fork<Self> {
        Fork::new(self)
    }

    unsafe fn unfork(&mut self) {
        self.links.delete_all();
    }
}

impl Deref for DoubletsSplitVolatileBenched {
    type Target = DoubletsLinks<DoubletsSplitVolatile>;

    fn deref(&self) -> &Self::Target {
        &self.links
    }
}

impl DerefMut for DoubletsSplitVolatileBenched {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.links
    }
}
