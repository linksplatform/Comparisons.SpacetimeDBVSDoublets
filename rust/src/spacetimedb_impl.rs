//! SpacetimeDB 2.0 storage implementation for links using the official Rust crate.
//!
//! This implementation uses the real SpacetimeDB 2.0 engine via `spacetimedb-core`
//! with the `test` feature, which exposes `TestDB` — the same in-memory database
//! engine used by SpacetimeDB's own benchmark suite.
//!
//! # SpacetimeDB Version
//!
//! Uses SpacetimeDB v2.0.1 (`spacetimedb-core` git tag `v2.0.1`).
//! The engine is accessed via `RelationalDB` / `TestDB` (in-memory mode).
//! No SQLite, no mock, no compatibility layer.
//!
//! # Schema
//!
//! The `links` table has three `u64` columns:
//! ```text
//! id     : u64  (column 0, indexed)
//! source : u64  (column 1, indexed)
//! target : u64  (column 2, indexed)
//! ```
//!
//! # Operation Complexity
//!
//! | Operation              | Complexity         |
//! |------------------------|--------------------|
//! | Create                 | O(log n)           |
//! | Update                 | O(log n)           |
//! | Delete                 | O(log n)           |
//! | Query All              | O(n)               |
//! | Query by Id            | O(log n)           |
//! | Query by Source        | O(log n + k)       |
//! | Query by Target        | O(log n + k)       |
//! | Query by Source+Target | O(log n + k)       |

use crate::{Link, Links};
use spacetimedb::db::relational_db::tests_utils::TestDB;
use spacetimedb_datastore::execution_context::Workload;
use spacetimedb_primitives::{ColId, TableId};
use spacetimedb_sats::{bsatn, product, AlgebraicType, AlgebraicValue};

/// Column indices in the links table.
const COL_ID: ColId = ColId(0);
const COL_SOURCE: ColId = ColId(1);
const COL_TARGET: ColId = ColId(2);

/// SpacetimeDB 2.0 in-memory links storage using the real RelationalDB engine.
///
/// Uses `TestDB::in_memory()` from `spacetimedb-core` with `features = ["test"]`,
/// which is the same in-memory storage SpacetimeDB uses for its own benchmarks.
/// No SQLite is involved in the storage path.
pub struct SpacetimeDbLinks {
    db: TestDB,
    table_id: TableId,
    next_id: u64,
}

impl SpacetimeDbLinks {
    /// Create a new in-memory SpacetimeDB 2.0 links storage.
    ///
    /// Logs SpacetimeDB version and fails if the backend is not SpacetimeDB 2.0.
    pub fn new_memory() -> Self {
        verify_backend();

        let db = TestDB::in_memory().expect("Failed to create in-memory SpacetimeDB database");

        // Create the links table with BTree indexes on id, source, and target columns.
        let table_id = db
            .with_auto_commit(Workload::Internal, |tx| {
                db.create_table_for_test(
                    "links",
                    &[
                        ("id", AlgebraicType::U64),
                        ("source", AlgebraicType::U64),
                        ("target", AlgebraicType::U64),
                    ],
                    &[COL_ID, COL_SOURCE, COL_TARGET],
                )
            })
            .expect("Failed to create links table");

        Self { db, table_id, next_id: 1 }
    }

    /// Serialize a link row into BSATN bytes for insertion into the SpacetimeDB engine.
    fn encode_row(id: u64, source: u64, target: u64) -> Vec<u8> {
        bsatn::to_vec(&product![id, source, target]).expect("Failed to serialize link row")
    }

    /// Decode a `ProductValue` element as `u64`.
    fn decode_u64(val: &AlgebraicValue) -> u64 {
        match val {
            AlgebraicValue::U64(v) => *v,
            _ => panic!("Expected U64 column, got {val:?}"),
        }
    }
}

/// Verify that the real SpacetimeDB 2.0 engine is active.
///
/// Logs version information and panics if the major version is not 2+.
fn verify_backend() {
    let version = env!("SPACETIMEDB_CORE_VERSION");
    eprintln!("[SpacetimeDB] Backend: spacetimedb-core v{version}");
    eprintln!("[SpacetimeDB] Engine: RelationalDB in-memory (no SQLite)");

    let major: u64 = version
        .split('.')
        .next()
        .and_then(|s| s.parse().ok())
        .expect("Failed to parse SpacetimeDB major version");

    assert!(
        major >= 2,
        "Benchmark requires SpacetimeDB 2.0+, got version {version}. \
         Ensure spacetimedb-core is pinned to git tag v2.0.1 or later."
    );
}

impl Links for SpacetimeDbLinks {
    fn create(&mut self, source: u64, target: u64) -> u64 {
        let id = self.next_id;
        let row_bytes = Self::encode_row(id, source, target);
        self.db
            .with_auto_commit(Workload::Internal, |tx| {
                self.db.insert(tx, self.table_id, &row_bytes).map(|_| ())
            })
            .expect("Failed to insert link");
        self.next_id += 1;
        id
    }

    fn update(&mut self, id: u64, source: u64, target: u64) {
        let table_id = self.table_id;
        let new_bytes = Self::encode_row(id, source, target);
        let id_val = AlgebraicValue::U64(id);
        self.db
            .with_auto_commit(Workload::Internal, |tx| {
                // Find and delete the old row, then insert the updated row.
                if let Some(row_ref) =
                    self.db.iter_by_col_eq_mut(tx, table_id, COL_ID, &id_val)?.next()
                {
                    let ptr = row_ref.pointer();
                    self.db.delete(tx, table_id, [ptr]);
                }
                self.db.insert(tx, table_id, &new_bytes).map(|_| ())
            })
            .expect("Failed to update link");
    }

    fn delete(&mut self, id: u64) {
        let table_id = self.table_id;
        let id_val = AlgebraicValue::U64(id);
        self.db
            .with_auto_commit(Workload::Internal, |tx| {
                if let Some(row_ref) =
                    self.db.iter_by_col_eq_mut(tx, table_id, COL_ID, &id_val)?.next()
                {
                    let ptr = row_ref.pointer();
                    self.db.delete(tx, table_id, [ptr]);
                }
                Ok::<_, spacetimedb::error::DBError>(())
            })
            .expect("Failed to delete link");
    }

    fn delete_all(&mut self) {
        let table_id = self.table_id;
        self.db
            .with_auto_commit(Workload::Internal, |tx| self.db.clear_table(tx, table_id))
            .expect("Failed to delete all links");
        self.next_id = 1;
    }

    fn query_all(&self) -> Vec<Link> {
        let table_id = self.table_id;
        self.db
            .with_auto_commit(Workload::Internal, |tx| {
                let links = self
                    .db
                    .iter_mut(tx, table_id)?
                    .map(|row_ref| {
                        let pv = row_ref.to_product_value();
                        let id = Self::decode_u64(&pv.elements[0]);
                        let source = Self::decode_u64(&pv.elements[1]);
                        let target = Self::decode_u64(&pv.elements[2]);
                        Link::new(id, source, target)
                    })
                    .collect();
                Ok::<_, spacetimedb::error::DBError>(links)
            })
            .expect("Failed to query all links")
    }

    fn query_by_id(&self, id: u64) -> Option<Link> {
        let table_id = self.table_id;
        let id_val = AlgebraicValue::U64(id);
        self.db
            .with_auto_commit(Workload::Internal, |tx| {
                let link = self
                    .db
                    .iter_by_col_eq_mut(tx, table_id, COL_ID, &id_val)?
                    .next()
                    .map(|row_ref| {
                        let pv = row_ref.to_product_value();
                        Link::new(
                            Self::decode_u64(&pv.elements[0]),
                            Self::decode_u64(&pv.elements[1]),
                            Self::decode_u64(&pv.elements[2]),
                        )
                    });
                Ok::<_, spacetimedb::error::DBError>(link)
            })
            .ok()
            .flatten()
    }

    fn query_by_source(&self, source: u64) -> Vec<Link> {
        let table_id = self.table_id;
        let src_val = AlgebraicValue::U64(source);
        self.db
            .with_auto_commit(Workload::Internal, |tx| {
                let links = self
                    .db
                    .iter_by_col_eq_mut(tx, table_id, COL_SOURCE, &src_val)?
                    .map(|row_ref| {
                        let pv = row_ref.to_product_value();
                        Link::new(
                            Self::decode_u64(&pv.elements[0]),
                            Self::decode_u64(&pv.elements[1]),
                            Self::decode_u64(&pv.elements[2]),
                        )
                    })
                    .collect();
                Ok::<_, spacetimedb::error::DBError>(links)
            })
            .expect("Failed to query by source")
    }

    fn query_by_target(&self, target: u64) -> Vec<Link> {
        let table_id = self.table_id;
        let tgt_val = AlgebraicValue::U64(target);
        self.db
            .with_auto_commit(Workload::Internal, |tx| {
                let links = self
                    .db
                    .iter_by_col_eq_mut(tx, table_id, COL_TARGET, &tgt_val)?
                    .map(|row_ref| {
                        let pv = row_ref.to_product_value();
                        Link::new(
                            Self::decode_u64(&pv.elements[0]),
                            Self::decode_u64(&pv.elements[1]),
                            Self::decode_u64(&pv.elements[2]),
                        )
                    })
                    .collect();
                Ok::<_, spacetimedb::error::DBError>(links)
            })
            .expect("Failed to query by target")
    }

    fn query_by_source_target(&self, source: u64, target: u64) -> Vec<Link> {
        // Filter by source index first, then filter by target in process.
        self.query_by_source(source)
            .into_iter()
            .filter(|link| link.target == target)
            .collect()
    }

    fn count(&self) -> usize {
        let table_id = self.table_id;
        self.db
            .with_auto_commit(Workload::Internal, |tx| {
                let count = self.db.iter_mut(tx, table_id)?.count();
                Ok::<_, spacetimedb::error::DBError>(count)
            })
            .expect("Failed to count links")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_query_memory() {
        let mut db = SpacetimeDbLinks::new_memory();
        let id = db.create_point();
        assert_eq!(id, 1);

        let link = db.query_by_id(id).unwrap();
        assert_eq!(link.source, id);
        assert_eq!(link.target, id);
    }

    #[test]
    fn test_update_memory() {
        let mut db = SpacetimeDbLinks::new_memory();
        let id = db.create(1, 2);
        db.update(id, 3, 4);

        let link = db.query_by_id(id).unwrap();
        assert_eq!(link.source, 3);
        assert_eq!(link.target, 4);
    }

    #[test]
    fn test_delete_memory() {
        let mut db = SpacetimeDbLinks::new_memory();
        let id = db.create_point();
        db.delete(id);
        assert!(db.query_by_id(id).is_none());
    }

    #[test]
    fn test_query_by_source() {
        let mut db = SpacetimeDbLinks::new_memory();
        let id1 = db.create_point();
        let id2 = db.create_point();
        db.update(id1, id1, id2);
        db.update(id2, id1, id1);

        let links = db.query_by_source(id1);
        assert_eq!(links.len(), 2);
    }

    #[test]
    fn test_query_by_target() {
        let mut db = SpacetimeDbLinks::new_memory();
        let id1 = db.create_point();
        let id2 = db.create_point();
        db.update(id1, id2, id1);
        db.update(id2, id2, id1);

        let links = db.query_by_target(id1);
        assert_eq!(links.len(), 2);
    }

    #[test]
    fn test_delete_all() {
        let mut db = SpacetimeDbLinks::new_memory();
        for _ in 0..10 {
            db.create_point();
        }
        assert_eq!(db.count(), 10);
        db.delete_all();
        assert_eq!(db.count(), 0);
    }

    #[test]
    fn test_query_by_source_target() {
        let mut db = SpacetimeDbLinks::new_memory();
        let id1 = db.create(10, 20);
        let _id2 = db.create(10, 30);

        let links = db.query_by_source_target(10, 20);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].id, id1);
    }
}
