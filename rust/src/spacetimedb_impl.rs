//! SpacetimeDB storage implementation for links (SQLite backend).
//!
//! This implementation uses the same SQLite storage that SpacetimeDB 2 uses
//! internally. SpacetimeDB stores data in SQLite tables with column-oriented
//! layout; this implementation uses the equivalent schema.
//!
//! SpacetimeDB is benchmarked via its SQLite backend to establish a fair
//! baseline comparison with Doublets' in-memory data structures.
//!
//! # Schema
//!
//! ```sql
//! CREATE TABLE links (
//!     id     INTEGER PRIMARY KEY,
//!     source INTEGER NOT NULL,
//!     target INTEGER NOT NULL
//! );
//! CREATE INDEX idx_source ON links(source);
//! CREATE INDEX idx_target ON links(target);
//! CREATE INDEX idx_source_target ON links(source, target);
//! ```
//!
//! # Operation Complexity
//!
//! | Operation              | Complexity            |
//! |------------------------|-----------------------|
//! | Create                 | O(log n) + disk I/O   |
//! | Update                 | O(log n) + disk I/O   |
//! | Delete                 | O(log n) + disk I/O   |
//! | Query All              | O(n) + disk I/O       |
//! | Query by Id            | O(log n)              |
//! | Query by Source        | O(log n + k)          |
//! | Query by Target        | O(log n + k)          |
//! | Query by Source+Target | O(log n + k)          |

use crate::{Link, Links};
use rusqlite::{params, Connection};

/// SQLite-based links storage using SpacetimeDB's internal schema.
///
/// SpacetimeDB 2 stores table data in SQLite with auto-incrementing
/// primary keys and B-tree indexes on all searchable columns.
pub struct SpacetimeDbLinks {
    conn: Connection,
    next_id: u64,
}

impl SpacetimeDbLinks {
    /// Create a new in-memory SpacetimeDB links storage (no persistence).
    ///
    /// This matches SpacetimeDB's behavior when running without a persistent
    /// data directory (e.g., for testing/benchmarking).
    pub fn new_memory() -> Self {
        let conn = Connection::open_in_memory().expect("Failed to open in-memory SQLite database");
        Self::init(conn)
    }

    fn init(conn: Connection) -> Self {
        // Enable WAL mode for better write performance (SpacetimeDB default)
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL;")
            .expect("Failed to set SQLite pragmas");

        conn.execute(
            "CREATE TABLE IF NOT EXISTS links (
                id     INTEGER PRIMARY KEY,
                source INTEGER NOT NULL,
                target INTEGER NOT NULL
            )",
            [],
        )
        .expect("Failed to create links table");

        // B-tree indexes on source, target, and composite (source, target)
        // matching SpacetimeDB's automatic index generation for filtered columns
        conn.execute("CREATE INDEX IF NOT EXISTS idx_source ON links(source)", [])
            .expect("Failed to create source index");

        conn.execute("CREATE INDEX IF NOT EXISTS idx_target ON links(target)", [])
            .expect("Failed to create target index");

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_source_target ON links(source, target)",
            [],
        )
        .expect("Failed to create source_target index");

        let next_id: u64 = conn
            .query_row("SELECT COALESCE(MAX(id), 0) + 1 FROM links", [], |row| {
                row.get(0)
            })
            .unwrap_or(1);

        Self { conn, next_id }
    }

    /// Drop and recreate all tables and indexes (used by `delete_all`).
    fn reset_schema(&mut self) {
        self.conn
            .execute("DELETE FROM links", [])
            .expect("Failed to delete all links");
        self.next_id = 1;
    }
}

impl Links for SpacetimeDbLinks {
    fn create(&mut self, source: u64, target: u64) -> u64 {
        let id = self.next_id;
        self.conn
            .execute(
                "INSERT INTO links (id, source, target) VALUES (?1, ?2, ?3)",
                params![id as i64, source as i64, target as i64],
            )
            .expect("Failed to insert link");
        self.next_id += 1;
        id
    }

    fn update(&mut self, id: u64, source: u64, target: u64) {
        self.conn
            .execute(
                "UPDATE links SET source = ?1, target = ?2 WHERE id = ?3",
                params![source as i64, target as i64, id as i64],
            )
            .expect("Failed to update link");
    }

    fn delete(&mut self, id: u64) {
        self.conn
            .execute("DELETE FROM links WHERE id = ?1", params![id as i64])
            .expect("Failed to delete link");
    }

    fn delete_all(&mut self) {
        self.reset_schema();
    }

    fn query_all(&self) -> Vec<Link> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, source, target FROM links")
            .expect("Failed to prepare query_all");

        stmt.query_map([], |row| {
            Ok(Link::new(
                row.get::<_, i64>(0)? as u64,
                row.get::<_, i64>(1)? as u64,
                row.get::<_, i64>(2)? as u64,
            ))
        })
        .expect("Failed to execute query_all")
        .filter_map(|r| r.ok())
        .collect()
    }

    fn query_by_id(&self, id: u64) -> Option<Link> {
        self.conn
            .query_row(
                "SELECT id, source, target FROM links WHERE id = ?1",
                params![id as i64],
                |row| {
                    Ok(Link::new(
                        row.get::<_, i64>(0)? as u64,
                        row.get::<_, i64>(1)? as u64,
                        row.get::<_, i64>(2)? as u64,
                    ))
                },
            )
            .ok()
    }

    fn query_by_source(&self, source: u64) -> Vec<Link> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, source, target FROM links WHERE source = ?1")
            .expect("Failed to prepare query_by_source");

        stmt.query_map(params![source as i64], |row| {
            Ok(Link::new(
                row.get::<_, i64>(0)? as u64,
                row.get::<_, i64>(1)? as u64,
                row.get::<_, i64>(2)? as u64,
            ))
        })
        .expect("Failed to execute query_by_source")
        .filter_map(|r| r.ok())
        .collect()
    }

    fn query_by_target(&self, target: u64) -> Vec<Link> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, source, target FROM links WHERE target = ?1")
            .expect("Failed to prepare query_by_target");

        stmt.query_map(params![target as i64], |row| {
            Ok(Link::new(
                row.get::<_, i64>(0)? as u64,
                row.get::<_, i64>(1)? as u64,
                row.get::<_, i64>(2)? as u64,
            ))
        })
        .expect("Failed to execute query_by_target")
        .filter_map(|r| r.ok())
        .collect()
    }

    fn query_by_source_target(&self, source: u64, target: u64) -> Vec<Link> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, source, target FROM links WHERE source = ?1 AND target = ?2")
            .expect("Failed to prepare query_by_source_target");

        stmt.query_map(params![source as i64, target as i64], |row| {
            Ok(Link::new(
                row.get::<_, i64>(0)? as u64,
                row.get::<_, i64>(1)? as u64,
                row.get::<_, i64>(2)? as u64,
            ))
        })
        .expect("Failed to execute query_by_source_target")
        .filter_map(|r| r.ok())
        .collect()
    }

    fn count(&self) -> usize {
        self.conn
            .query_row("SELECT COUNT(*) FROM links", [], |row| row.get::<_, i64>(0))
            .unwrap_or(0) as usize
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
