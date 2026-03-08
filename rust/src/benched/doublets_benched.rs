//! `Benched` implementations for Doublets stores.

use crate::{
    benched::Benched,
    doublets_impl::{
        create_split_non_volatile, create_split_volatile, create_united_non_volatile,
        create_united_volatile, DoubletsLinks, DoubletsSplitNonVolatile, DoubletsSplitVolatile,
        DoubletsUnitedNonVolatile, DoubletsUnitedVolatile,
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

/// Benchmark subject for Doublets united non-volatile (file-backed, contiguous) store.
///
/// Uses a memory-mapped file for persistent storage. The file path is provided at setup.
pub struct DoubletsUnitedNonVolatileBenched {
    links: DoubletsLinks<DoubletsUnitedNonVolatile>,
    path: String,
}

impl Benched for DoubletsUnitedNonVolatileBenched {
    type Builder = String;

    fn setup(builder: Self::Builder) -> Self {
        // Remove any leftover file from a previous run so the store starts empty.
        let _ = std::fs::remove_file(&builder);
        let links = create_united_non_volatile(&builder);
        Self {
            links,
            path: builder,
        }
    }

    fn fork(&mut self) -> Fork<Self> {
        Fork::new(self)
    }

    unsafe fn unfork(&mut self) {
        self.links.delete_all();
    }
}

impl Deref for DoubletsUnitedNonVolatileBenched {
    type Target = DoubletsLinks<DoubletsUnitedNonVolatile>;

    fn deref(&self) -> &Self::Target {
        &self.links
    }
}

impl DerefMut for DoubletsUnitedNonVolatileBenched {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.links
    }
}

impl Drop for DoubletsUnitedNonVolatileBenched {
    fn drop(&mut self) {
        // Clean up the benchmark file when the benched subject is dropped.
        let _ = std::fs::remove_file(&self.path);
    }
}

/// Benchmark subject for Doublets split non-volatile (file-backed, separate data/index) store.
///
/// Uses two memory-mapped files for persistent storage. File paths are provided at setup.
pub struct DoubletsSplitNonVolatileBenched {
    links: DoubletsLinks<DoubletsSplitNonVolatile>,
    data_path: String,
    index_path: String,
}

impl Benched for DoubletsSplitNonVolatileBenched {
    type Builder = (String, String);

    fn setup(builder: Self::Builder) -> Self {
        let (data_path, index_path) = builder;
        // Remove any leftover files from a previous run so the store starts empty.
        let _ = std::fs::remove_file(&data_path);
        let _ = std::fs::remove_file(&index_path);
        let links = create_split_non_volatile(&data_path, &index_path);
        Self {
            links,
            data_path,
            index_path,
        }
    }

    fn fork(&mut self) -> Fork<Self> {
        Fork::new(self)
    }

    unsafe fn unfork(&mut self) {
        self.links.delete_all();
    }
}

impl Deref for DoubletsSplitNonVolatileBenched {
    type Target = DoubletsLinks<DoubletsSplitNonVolatile>;

    fn deref(&self) -> &Self::Target {
        &self.links
    }
}

impl DerefMut for DoubletsSplitNonVolatileBenched {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.links
    }
}

impl Drop for DoubletsSplitNonVolatileBenched {
    fn drop(&mut self) {
        // Clean up the benchmark files when the benched subject is dropped.
        let _ = std::fs::remove_file(&self.data_path);
        let _ = std::fs::remove_file(&self.index_path);
    }
}
