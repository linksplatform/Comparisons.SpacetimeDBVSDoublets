//! Doublets storage implementation for links.
//!
//! Adapts the Doublets link store to the shared `Links` trait used in benchmarks.
//!
//! # Doublets Storage Types
//!
//! Four storage layouts are benchmarked: volatile (in-memory) and non-volatile
//! (file-backed) variants of united and split stores.
//!
//! - **United Volatile**: Each link stored as a contiguous unit `(index, source, target)`.
//!   Single allocation; best cache locality for small stores. Data lives in RAM only.
//! - **Split Volatile**: Data part and index part in separate memory regions.
//!   Better cache efficiency for index-heavy workloads. Data lives in RAM only.
//! - **United NonVolatile**: Same layout as United Volatile but backed by a memory-mapped
//!   file. Data persists to disk on drop via `sync_all()`.
//! - **Split NonVolatile**: Same layout as Split Volatile but backed by two memory-mapped
//!   files (one for data, one for the index). Data persists to disk on drop via `sync_all()`.
//!
//! # Operation Complexity
//!
//! | Operation             | United | Split |
//! |-----------------------|--------|-------|
//! | Create                | O(log n) | O(log n) |
//! | Update                | O(log n) | O(log n) |
//! | Delete                | O(log n) | O(log n) |
//! | Query All             | O(n)   | O(n)  |
//! | Query by Id           | O(1)   | O(1)  |
//! | Query by Source       | O(log n + k) | O(log n + k) |
//! | Query by Target       | O(log n + k) | O(log n + k) |
//! | Query by Source+Target| O(log n + k) | O(log n + k) |

use crate::{Link, Links};
use doublets::{
    mem::Alloc,
    split::{self, DataPart, IndexPart},
    unit::{self, LinkPart},
    Doublets, DoubletsExt,
};
use platform_mem::FileMapped;
use std::alloc::Global;

/// In-memory united (single contiguous region) doublets store.
pub type DoubletsUnitedVolatile<T = usize> = unit::Store<T, Alloc<LinkPart<T>, Global>>;

/// In-memory split (separate data and index regions) doublets store.
pub type DoubletsSplitVolatile<T = usize> =
    split::Store<T, Alloc<DataPart<T>, Global>, Alloc<IndexPart<T>, Global>>;

/// File-backed united (single contiguous region) doublets store.
pub type DoubletsUnitedNonVolatile<T = usize> = unit::Store<T, FileMapped<LinkPart<T>>>;

/// File-backed split (separate data and index regions) doublets store.
pub type DoubletsSplitNonVolatile<T = usize> =
    split::Store<T, FileMapped<DataPart<T>>, FileMapped<IndexPart<T>>>;

/// Wrapper adapting a `doublets::Doublets` store to the shared `Links` trait.
pub struct DoubletsLinks<S> {
    store: S,
}

impl<S> DoubletsLinks<S> {
    pub fn new(store: S) -> Self {
        Self { store }
    }
}

impl<S: Doublets<usize> + DoubletsExt<usize>> Links for DoubletsLinks<S> {
    fn create(&mut self, source: u64, target: u64) -> u64 {
        self.store
            .create_by([source as usize, target as usize])
            .expect("Failed to create link") as u64
    }

    fn create_point(&mut self) -> u64 {
        self.store.create_point().expect("Failed to create point") as u64
    }

    fn update(&mut self, id: u64, source: u64, target: u64) {
        self.store
            .update(id as usize, source as usize, target as usize)
            .expect("Failed to update link");
    }

    fn delete(&mut self, id: u64) {
        self.store
            .delete(id as usize)
            .expect("Failed to delete link");
    }

    fn delete_all(&mut self) {
        let any = self.store.constants().any;
        let ids: Vec<usize> = self
            .store
            .each_iter([any, any, any])
            .map(|link| link.index)
            .collect();
        for id in ids {
            let _ = self.store.delete(id);
        }
    }

    fn query_all(&self) -> Vec<Link> {
        let any = self.store.constants().any;
        self.store
            .each_iter([any, any, any])
            .map(|link| Link::new(link.index as u64, link.source as u64, link.target as u64))
            .collect()
    }

    fn query_by_id(&self, id: u64) -> Option<Link> {
        self.store
            .get_link(id as usize)
            .map(|link| Link::new(link.index as u64, link.source as u64, link.target as u64))
    }

    fn query_by_source(&self, source: u64) -> Vec<Link> {
        let any = self.store.constants().any;
        self.store
            .each_iter([any, source as usize, any])
            .map(|link| Link::new(link.index as u64, link.source as u64, link.target as u64))
            .collect()
    }

    fn query_by_target(&self, target: u64) -> Vec<Link> {
        let any = self.store.constants().any;
        self.store
            .each_iter([any, any, target as usize])
            .map(|link| Link::new(link.index as u64, link.source as u64, link.target as u64))
            .collect()
    }

    fn query_by_source_target(&self, source: u64, target: u64) -> Vec<Link> {
        let any = self.store.constants().any;
        self.store
            .each_iter([any, source as usize, target as usize])
            .map(|link| Link::new(link.index as u64, link.source as u64, link.target as u64))
            .collect()
    }

    fn count(&self) -> usize {
        self.store.count()
    }
}

/// Create a new in-memory doublets united store.
pub fn create_united_volatile() -> DoubletsLinks<DoubletsUnitedVolatile> {
    let mem = Alloc::new(Global);
    let store = DoubletsUnitedVolatile::new(mem).expect("Failed to create doublets united store");
    DoubletsLinks::new(store)
}

/// Create a new in-memory doublets split store.
pub fn create_split_volatile() -> DoubletsLinks<DoubletsSplitVolatile> {
    let data_mem = Alloc::new(Global);
    let index_mem = Alloc::new(Global);
    let store = DoubletsSplitVolatile::new(data_mem, index_mem)
        .expect("Failed to create doublets split store");
    DoubletsLinks::new(store)
}

/// Create a new file-backed doublets united store at the given path.
pub fn create_united_non_volatile(path: &str) -> DoubletsLinks<DoubletsUnitedNonVolatile> {
    let mem = FileMapped::from_path(path).expect("Failed to open united links file");
    let store = DoubletsUnitedNonVolatile::new(mem)
        .expect("Failed to create doublets united non-volatile store");
    DoubletsLinks::new(store)
}

/// Create a new file-backed doublets split store at the given paths.
///
/// `data_path` stores the link data (source, target), `index_path` stores the tree index.
pub fn create_split_non_volatile(
    data_path: &str,
    index_path: &str,
) -> DoubletsLinks<DoubletsSplitNonVolatile> {
    let data_mem = FileMapped::from_path(data_path).expect("Failed to open split data file");
    let index_mem = FileMapped::from_path(index_path).expect("Failed to open split index file");
    let store = DoubletsSplitNonVolatile::new(data_mem, index_mem)
        .expect("Failed to create doublets split non-volatile store");
    DoubletsLinks::new(store)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_query_united() {
        let mut db = create_united_volatile();
        let id = db.create_point();
        assert_eq!(id, 1);

        let link = db.query_by_id(id).unwrap();
        assert_eq!(link.source, id);
        assert_eq!(link.target, id);
    }

    #[test]
    fn test_create_and_query_split() {
        let mut db = create_split_volatile();
        let id = db.create_point();
        assert_eq!(id, 1);

        let link = db.query_by_id(id).unwrap();
        assert_eq!(link.source, id);
        assert_eq!(link.target, id);
    }

    #[test]
    fn test_update_united() {
        let mut db = create_united_volatile();
        let id = db.create(1, 2);
        db.update(id, 3, 4);

        let link = db.query_by_id(id).unwrap();
        assert_eq!(link.source, 3);
        assert_eq!(link.target, 4);
    }

    #[test]
    fn test_delete_united() {
        let mut db = create_united_volatile();
        let id = db.create_point();
        db.delete(id);
        assert!(db.query_by_id(id).is_none());
    }

    #[test]
    fn test_query_by_source_united() {
        let mut db = create_united_volatile();
        let id1 = db.create_point();
        let id2 = db.create_point();
        db.update(id1, id1, id2);
        db.update(id2, id1, id1);

        let links = db.query_by_source(id1);
        assert_eq!(links.len(), 2);
    }

    #[test]
    fn test_query_by_target_split() {
        let mut db = create_split_volatile();
        // Create independent links: (1,1,1), (2,2,2), (3,3,3)
        let id1 = db.create_point();
        let id2 = db.create_point();
        let id3 = db.create_point();
        // Update link3 to point at id1 as target; use id3 as source (independent)
        db.update(id3, id3, id1);
        // Update link2 to also point at id1 as target; use id2 as source (still points to itself but as source)
        db.update(id2, id2, id1);

        let links = db.query_by_target(id1);
        // id1 itself is a point (source=1, target=1), id2 now has target=id1, id3 now has target=id1
        assert!(links.len() >= 2);
    }

    #[test]
    fn test_delete_all_united() {
        let mut db = create_united_volatile();
        for _ in 0..10 {
            db.create_point();
        }
        assert_eq!(db.count(), 10);
        db.delete_all();
        assert_eq!(db.count(), 0);
    }

    #[test]
    fn test_create_and_query_united_non_volatile() {
        let path = "/tmp/test_united_non_volatile.links";
        // Remove any leftover file from previous runs
        let _ = std::fs::remove_file(path);
        {
            let mut db = create_united_non_volatile(path);
            let id = db.create_point();
            assert_eq!(id, 1);
            let link = db.query_by_id(id).unwrap();
            assert_eq!(link.source, id);
            assert_eq!(link.target, id);
        }
        // Clean up
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_create_and_query_split_non_volatile() {
        let data_path = "/tmp/test_split_non_volatile_data.links";
        let index_path = "/tmp/test_split_non_volatile_index.links";
        // Remove any leftover files from previous runs
        let _ = std::fs::remove_file(data_path);
        let _ = std::fs::remove_file(index_path);
        {
            let mut db = create_split_non_volatile(data_path, index_path);
            let id = db.create_point();
            assert_eq!(id, 1);
            let link = db.query_by_id(id).unwrap();
            assert_eq!(link.source, id);
            assert_eq!(link.target, id);
        }
        // Clean up
        let _ = std::fs::remove_file(data_path);
        let _ = std::fs::remove_file(index_path);
    }
}
