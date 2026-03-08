//! SpacetimeDB 2.0 storage implementation using the official `spacetimedb-sdk` Rust crate.
//!
//! Connects to a running SpacetimeDB 2.0 server via WebSocket and calls
//! reducers for link CRUD operations. Reads are served from the client-side
//! subscription cache.
//!
//! # Setup
//!
//! Requires a running SpacetimeDB server with the links module published.
//! Set `SPACETIMEDB_URI` (default: `http://localhost:3000`) and
//! `SPACETIMEDB_DB` (default: `benchmark-links`) environment variables.
//!
//! # SpacetimeDB version
//!
//! Uses `spacetimedb-sdk` v2 from crates.io (the official Rust client SDK).

use crate::{
    module_bindings::{
        create_link_reducer::create_link, delete_all_links_reducer::delete_all_links,
        delete_link_reducer::delete_link, link_table::LinkTableAccess,
        update_link_reducer::update_link, DbConnection, Link as SdbLink,
    },
    Link, Links,
};
use once_cell::sync::Lazy;
use spacetimedb_sdk::{DbContext, Table};
use std::{
    env,
    sync::{Arc, Condvar, Mutex},
    thread::JoinHandle,
    time::Duration,
};

const DEFAULT_URI: &str = "http://localhost:3000";
const DEFAULT_DB: &str = "benchmark-links";

static SPACETIMEDB_URI: Lazy<String> =
    Lazy::new(|| env::var("SPACETIMEDB_URI").unwrap_or_else(|_| DEFAULT_URI.to_string()));

static SPACETIMEDB_DB: Lazy<String> =
    Lazy::new(|| env::var("SPACETIMEDB_DB").unwrap_or_else(|_| DEFAULT_DB.to_string()));

/// SpacetimeDB 2.0 links storage using the official `spacetimedb-sdk` Rust crate.
///
/// Connects to a running SpacetimeDB server via WebSocket, subscribes to the
/// `links` table, and calls reducers for CRUD operations. Reads are served
/// from the client-side subscription cache.
pub struct SpacetimeDbLinks {
    conn: DbConnection,
    /// Background thread handle keeping the SDK event loop alive.
    _thread: JoinHandle<()>,
}

impl SpacetimeDbLinks {
    /// Connect to the SpacetimeDB server and subscribe to the links table.
    ///
    /// Reads server URI and database name from environment variables:
    /// - `SPACETIMEDB_URI` (default: `http://localhost:3000`)
    /// - `SPACETIMEDB_DB` (default: `benchmark-links`)
    ///
    /// Panics if the server is not reachable or the module is not published.
    pub fn connect() -> Self {
        let sub_ready = Arc::new((Mutex::new(false), Condvar::new()));
        let sub_ready_clone = Arc::clone(&sub_ready);

        let uri = SPACETIMEDB_URI.as_str();
        let db = SPACETIMEDB_DB.as_str();

        eprintln!("[SpacetimeDB] Connecting to {uri}/{db}");

        let conn = DbConnection::builder()
            .with_uri(uri)
            .with_database_name(db)
            .on_connect(|_ctx, identity, _token| {
                eprintln!("[SpacetimeDB] Connected as {identity}");
            })
            .on_connect_error(|_ctx, err| {
                panic!("[SpacetimeDB] Connection error: {err}");
            })
            .on_disconnect(|_ctx, _err| {
                eprintln!("[SpacetimeDB] Disconnected");
            })
            .build()
            .expect("Failed to connect to SpacetimeDB server");

        // Subscribe to all links so the client cache is populated.
        conn.subscription_builder()
            .on_applied(move |_ctx| {
                let (lock, cvar) = &*sub_ready_clone;
                let mut ready = lock.lock().unwrap();
                *ready = true;
                cvar.notify_all();
            })
            .on_error(|_ctx, err| {
                panic!("[SpacetimeDB] Subscription error: {err}");
            })
            .subscribe(["SELECT * FROM links"]);

        // Run the SDK event loop in a background thread.
        let thread = conn.run_threaded();

        // Wait until the initial subscription is applied (cache is populated).
        let (lock, cvar) = &*sub_ready;
        let mut ready = lock.lock().unwrap();
        while !*ready {
            let result = cvar
                .wait_timeout(ready, Duration::from_secs(30))
                .expect("Subscription wait interrupted");
            ready = result.0;
            if result.1.timed_out() {
                panic!("[SpacetimeDB] Timed out waiting for subscription to be applied");
            }
        }

        eprintln!("[SpacetimeDB] Subscription ready, client cache populated");

        Self {
            conn,
            _thread: thread,
        }
    }

    /// Wait for a reducer to complete by using its `_then` callback with a condvar.
    ///
    /// The background thread (from `run_threaded()`) processes messages and triggers
    /// the callback. We wait on the condvar until the callback signals completion.
    fn wait_for_reducer<F>(&self, invoke: F)
    where
        F: FnOnce(Arc<(Mutex<bool>, Condvar)>),
    {
        let done = Arc::new((Mutex::new(false), Condvar::new()));
        invoke(Arc::clone(&done));
        let (lock, cvar) = &*done;
        let mut finished = lock.lock().unwrap();
        while !*finished {
            let result = cvar
                .wait_timeout(finished, Duration::from_secs(30))
                .expect("Reducer wait interrupted");
            finished = result.0;
            if result.1.timed_out() {
                panic!("[SpacetimeDB] Timed out waiting for reducer to complete");
            }
        }
    }

    fn to_link(row: SdbLink) -> Link {
        Link::new(row.id, row.source, row.target)
    }
}

impl Links for SpacetimeDbLinks {
    fn create(&mut self, source: u64, target: u64) -> u64 {
        self.wait_for_reducer(|done| {
            self.conn
                .reducers
                .create_link_then(source, target, move |_ctx, _result| {
                    let (lock, cvar) = &*done;
                    *lock.lock().unwrap() = true;
                    cvar.notify_all();
                })
                .expect("create_link reducer failed");
        });
        // Return the id of the newly inserted link (max id matching source+target).
        self.conn
            .db
            .links()
            .iter()
            .filter(|l| l.source == source && l.target == target)
            .map(|l| l.id)
            .max()
            .unwrap_or(0)
    }

    fn update(&mut self, id: u64, source: u64, target: u64) {
        self.wait_for_reducer(|done| {
            self.conn
                .reducers
                .update_link_then(id, source, target, move |_ctx, _result| {
                    let (lock, cvar) = &*done;
                    *lock.lock().unwrap() = true;
                    cvar.notify_all();
                })
                .expect("update_link reducer failed");
        });
    }

    fn delete(&mut self, id: u64) {
        self.wait_for_reducer(|done| {
            self.conn
                .reducers
                .delete_link_then(id, move |_ctx, _result| {
                    let (lock, cvar) = &*done;
                    *lock.lock().unwrap() = true;
                    cvar.notify_all();
                })
                .expect("delete_link reducer failed");
        });
    }

    fn delete_all(&mut self) {
        self.wait_for_reducer(|done| {
            self.conn
                .reducers
                .delete_all_links_then(move |_ctx, _result| {
                    let (lock, cvar) = &*done;
                    *lock.lock().unwrap() = true;
                    cvar.notify_all();
                })
                .expect("delete_all_links reducer failed");
        });
    }

    fn query_all(&self) -> Vec<Link> {
        self.conn.db.links().iter().map(Self::to_link).collect()
    }

    fn query_by_id(&self, id: u64) -> Option<Link> {
        self.conn
            .db
            .links()
            .iter()
            .find(|l| l.id == id)
            .map(Self::to_link)
    }

    fn query_by_source(&self, source: u64) -> Vec<Link> {
        self.conn
            .db
            .links()
            .iter()
            .filter(|l| l.source == source)
            .map(Self::to_link)
            .collect()
    }

    fn query_by_target(&self, target: u64) -> Vec<Link> {
        self.conn
            .db
            .links()
            .iter()
            .filter(|l| l.target == target)
            .map(Self::to_link)
            .collect()
    }

    fn query_by_source_target(&self, source: u64, target: u64) -> Vec<Link> {
        self.conn
            .db
            .links()
            .iter()
            .filter(|l| l.source == source && l.target == target)
            .map(Self::to_link)
            .collect()
    }

    fn count(&self) -> usize {
        self.conn.db.links().count() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn server_available() -> bool {
        let uri = SPACETIMEDB_URI.as_str();
        let addr = uri
            .trim_start_matches("http://")
            .trim_start_matches("https://");
        let addr = if addr.contains(':') {
            addr.to_string()
        } else {
            format!("{addr}:3000")
        };
        std::net::TcpStream::connect_timeout(
            &addr
                .parse()
                .unwrap_or_else(|_| "127.0.0.1:3000".parse().unwrap()),
            Duration::from_secs(1),
        )
        .is_ok()
    }

    #[test]
    fn test_create_and_query() {
        if !server_available() {
            eprintln!("SpacetimeDB server not available, skipping test");
            return;
        }
        let mut db = SpacetimeDbLinks::connect();
        db.delete_all();
        let id = db.create_point();
        assert!(id > 0);
        let link = db.query_by_id(id).unwrap();
        assert_eq!(link.source, id);
        assert_eq!(link.target, id);
    }

    #[test]
    fn test_update() {
        if !server_available() {
            eprintln!("SpacetimeDB server not available, skipping test");
            return;
        }
        let mut db = SpacetimeDbLinks::connect();
        db.delete_all();
        let id = db.create(1, 2);
        db.update(id, 3, 4);
        let link = db.query_by_id(id).unwrap();
        assert_eq!(link.source, 3);
        assert_eq!(link.target, 4);
    }

    #[test]
    fn test_delete() {
        if !server_available() {
            eprintln!("SpacetimeDB server not available, skipping test");
            return;
        }
        let mut db = SpacetimeDbLinks::connect();
        db.delete_all();
        let id = db.create_point();
        db.delete(id);
        assert!(db.query_by_id(id).is_none());
    }

    #[test]
    fn test_delete_all() {
        if !server_available() {
            eprintln!("SpacetimeDB server not available, skipping test");
            return;
        }
        let mut db = SpacetimeDbLinks::connect();
        for _ in 0..5 {
            db.create_point();
        }
        assert!(db.count() >= 5);
        db.delete_all();
        assert_eq!(db.count(), 0);
    }
}
