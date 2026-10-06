//! SpacetimeDB vs Doublets benchmark suite.
//!
//! Runs criterion benchmarks for basic CRUD operations with links,
//! comparing SpacetimeDB 2.0 (via official `spacetimedb-sdk`) against Doublets in-memory
//! and file-backed stores.
//!
//! Requires a running SpacetimeDB server with the links module published:
//! ```bash
//! spacetime start &
//! spacetime publish --project-path spacetime-module benchmark-links
//! ```
//!
//! Run benchmarks:
//! ```bash
//! cargo bench --bench bench -- --output-format bencher | tee out.txt
//! ```
//!
//! Configure scale via environment variables:
//! - `BENCHMARK_LINK_COUNT` — links to create/update/delete per iteration (default: 1000)
//! - `BACKGROUND_LINK_COUNT` — pre-populated links for realistic DB state (default: 3000)
//! - `SPACETIMEDB_URI` — SpacetimeDB server URI (default: `http://localhost:3000`)
//! - `SPACETIMEDB_DB` — database name (default: `benchmark-links`)

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use spacetimedb_vs_doublets::{
    benched::{
        Benched, DoubletsSplitNonVolatileBenched, DoubletsSplitVolatileBenched,
        DoubletsUnitedNonVolatileBenched, DoubletsUnitedVolatileBenched, SpacetimeDbBenched,
    },
    Links, BACKGROUND_LINK_COUNT, BENCHMARK_LINK_COUNT,
};
use std::time::{Duration, Instant};

// Select a backend before constructing it, so Doublets VMs need no server.
fn backend_enabled(backend: &str) -> bool {
    match std::env::var("BENCHMARK_BACKEND").as_deref() {
        Ok("all") | Err(_) => true,
        Ok("spacetimedb") => backend == "spacetimedb",
        Ok("doublets") => backend == "doublets",
        Ok(other) => panic!("Unknown BENCHMARK_BACKEND: {other}"),
    }
}

// ===================== HELPERS =====================

/// Populate background links before each measured iteration.
/// Uses the `Links` trait via deref from the fork.
macro_rules! setup_background {
    ($fork:expr) => {
        for _ in 0..*BACKGROUND_LINK_COUNT {
            $fork.create_point();
        }
    };
}

// ===================== CREATE =====================

fn spacetimedb_create(c: &mut Criterion) {
    if !backend_enabled("spacetimedb") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = SpacetimeDbBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("create/SpacetimeDB", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    let start = Instant::now();
                    for _ in 0..n {
                        fork.create_point();
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_united_create(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsUnitedVolatileBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("create/Doublets_United_Volatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    let start = Instant::now();
                    for _ in 0..n {
                        fork.create_point();
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_split_create(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsSplitVolatileBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("create/Doublets_Split_Volatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    let start = Instant::now();
                    for _ in 0..n {
                        fork.create_point();
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_united_non_volatile_create(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsUnitedNonVolatileBenched::setup(
        "/tmp/bench_united_non_volatile_create.links".to_string(),
    );
    c.bench_with_input(
        BenchmarkId::new("create/Doublets_United_NonVolatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    let start = Instant::now();
                    for _ in 0..n {
                        fork.create_point();
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_split_non_volatile_create(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsSplitNonVolatileBenched::setup((
        "/tmp/bench_split_non_volatile_create_data.links".to_string(),
        "/tmp/bench_split_non_volatile_create_index.links".to_string(),
    ));
    c.bench_with_input(
        BenchmarkId::new("create/Doublets_Split_NonVolatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    let start = Instant::now();
                    for _ in 0..n {
                        fork.create_point();
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

// ===================== DELETE =====================

fn spacetimedb_delete(c: &mut Criterion) {
    if !backend_enabled("spacetimedb") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = SpacetimeDbBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("delete/SpacetimeDB", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    let ids: Vec<u64> = (0..n).map(|_| fork.create_point()).collect();
                    let start = Instant::now();
                    for id in ids {
                        fork.delete(id);
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_united_delete(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsUnitedVolatileBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("delete/Doublets_United_Volatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    let ids: Vec<u64> = (0..n).map(|_| fork.create_point()).collect();
                    let start = Instant::now();
                    for id in ids {
                        fork.delete(id);
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_split_delete(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsSplitVolatileBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("delete/Doublets_Split_Volatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    let ids: Vec<u64> = (0..n).map(|_| fork.create_point()).collect();
                    let start = Instant::now();
                    for id in ids {
                        fork.delete(id);
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_united_non_volatile_delete(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsUnitedNonVolatileBenched::setup(
        "/tmp/bench_united_non_volatile_delete.links".to_string(),
    );
    c.bench_with_input(
        BenchmarkId::new("delete/Doublets_United_NonVolatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    let ids: Vec<u64> = (0..n).map(|_| fork.create_point()).collect();
                    let start = Instant::now();
                    for id in ids {
                        fork.delete(id);
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_split_non_volatile_delete(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsSplitNonVolatileBenched::setup((
        "/tmp/bench_split_non_volatile_delete_data.links".to_string(),
        "/tmp/bench_split_non_volatile_delete_index.links".to_string(),
    ));
    c.bench_with_input(
        BenchmarkId::new("delete/Doublets_Split_NonVolatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    let ids: Vec<u64> = (0..n).map(|_| fork.create_point()).collect();
                    let start = Instant::now();
                    for id in ids {
                        fork.delete(id);
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

// ===================== UPDATE =====================

fn spacetimedb_update(c: &mut Criterion) {
    if !backend_enabled("spacetimedb") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = SpacetimeDbBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("update/SpacetimeDB", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    let ids: Vec<u64> = (0..n).map(|_| fork.create_point()).collect();
                    let start = Instant::now();
                    for &id in &ids {
                        fork.update(id, 0, 0);
                    }
                    for &id in &ids {
                        fork.update(id, id, id);
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_united_update(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsUnitedVolatileBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("update/Doublets_United_Volatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    let ids: Vec<u64> = (0..n).map(|_| fork.create_point()).collect();
                    let start = Instant::now();
                    for &id in &ids {
                        fork.update(id, 0, 0);
                    }
                    for &id in &ids {
                        fork.update(id, id, id);
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_split_update(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsSplitVolatileBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("update/Doublets_Split_Volatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    let ids: Vec<u64> = (0..n).map(|_| fork.create_point()).collect();
                    let start = Instant::now();
                    for &id in &ids {
                        fork.update(id, 0, 0);
                    }
                    for &id in &ids {
                        fork.update(id, id, id);
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_united_non_volatile_update(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsUnitedNonVolatileBenched::setup(
        "/tmp/bench_united_non_volatile_update.links".to_string(),
    );
    c.bench_with_input(
        BenchmarkId::new("update/Doublets_United_NonVolatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    let ids: Vec<u64> = (0..n).map(|_| fork.create_point()).collect();
                    let start = Instant::now();
                    for &id in &ids {
                        fork.update(id, 0, 0);
                    }
                    for &id in &ids {
                        fork.update(id, id, id);
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_split_non_volatile_update(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsSplitNonVolatileBenched::setup((
        "/tmp/bench_split_non_volatile_update_data.links".to_string(),
        "/tmp/bench_split_non_volatile_update_index.links".to_string(),
    ));
    c.bench_with_input(
        BenchmarkId::new("update/Doublets_Split_NonVolatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    let ids: Vec<u64> = (0..n).map(|_| fork.create_point()).collect();
                    let start = Instant::now();
                    for &id in &ids {
                        fork.update(id, 0, 0);
                    }
                    for &id in &ids {
                        fork.update(id, id, id);
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

// ===================== QUERY ALL =====================

fn spacetimedb_query_all(c: &mut Criterion) {
    if !backend_enabled("spacetimedb") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = SpacetimeDbBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("query_all/SpacetimeDB", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    for _ in 0..n {
                        fork.create_point();
                    }
                    let start = Instant::now();
                    let _ = fork.query_all();
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_united_query_all(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsUnitedVolatileBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("query_all/Doublets_United_Volatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    for _ in 0..n {
                        fork.create_point();
                    }
                    let start = Instant::now();
                    let _ = fork.query_all();
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_split_query_all(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsSplitVolatileBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("query_all/Doublets_Split_Volatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    for _ in 0..n {
                        fork.create_point();
                    }
                    let start = Instant::now();
                    let _ = fork.query_all();
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_united_non_volatile_query_all(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsUnitedNonVolatileBenched::setup(
        "/tmp/bench_united_non_volatile_query_all.links".to_string(),
    );
    c.bench_with_input(
        BenchmarkId::new("query_all/Doublets_United_NonVolatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    for _ in 0..n {
                        fork.create_point();
                    }
                    let start = Instant::now();
                    let _ = fork.query_all();
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_split_non_volatile_query_all(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsSplitNonVolatileBenched::setup((
        "/tmp/bench_split_non_volatile_query_all_data.links".to_string(),
        "/tmp/bench_split_non_volatile_query_all_index.links".to_string(),
    ));
    c.bench_with_input(
        BenchmarkId::new("query_all/Doublets_Split_NonVolatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    for _ in 0..n {
                        fork.create_point();
                    }
                    let start = Instant::now();
                    let _ = fork.query_all();
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

// ===================== QUERY BY ID =====================

fn spacetimedb_query_by_id(c: &mut Criterion) {
    if !backend_enabled("spacetimedb") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = SpacetimeDbBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("query_by_id/SpacetimeDB", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    let ids: Vec<u64> = (0..n).map(|_| fork.create_point()).collect();
                    let start = Instant::now();
                    for id in ids {
                        std::hint::black_box(fork.query_by_id(id));
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_united_query_by_id(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsUnitedVolatileBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("query_by_id/Doublets_United_Volatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    let ids: Vec<u64> = (0..n).map(|_| fork.create_point()).collect();
                    let start = Instant::now();
                    for id in ids {
                        std::hint::black_box(fork.query_by_id(id));
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_split_query_by_id(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsSplitVolatileBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("query_by_id/Doublets_Split_Volatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    let ids: Vec<u64> = (0..n).map(|_| fork.create_point()).collect();
                    let start = Instant::now();
                    for id in ids {
                        std::hint::black_box(fork.query_by_id(id));
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_united_non_volatile_query_by_id(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsUnitedNonVolatileBenched::setup(
        "/tmp/bench_united_non_volatile_query_by_id.links".to_string(),
    );
    c.bench_with_input(
        BenchmarkId::new("query_by_id/Doublets_United_NonVolatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    let ids: Vec<u64> = (0..n).map(|_| fork.create_point()).collect();
                    let start = Instant::now();
                    for id in ids {
                        std::hint::black_box(fork.query_by_id(id));
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_split_non_volatile_query_by_id(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsSplitNonVolatileBenched::setup((
        "/tmp/bench_split_non_volatile_query_by_id_data.links".to_string(),
        "/tmp/bench_split_non_volatile_query_by_id_index.links".to_string(),
    ));
    c.bench_with_input(
        BenchmarkId::new("query_by_id/Doublets_Split_NonVolatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    let ids: Vec<u64> = (0..n).map(|_| fork.create_point()).collect();
                    let start = Instant::now();
                    for id in ids {
                        std::hint::black_box(fork.query_by_id(id));
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

// ===================== QUERY BY SOURCE =====================

fn spacetimedb_query_by_source(c: &mut Criterion) {
    if !backend_enabled("spacetimedb") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = SpacetimeDbBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("query_by_source/SpacetimeDB", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    let background_ids: Vec<u64> = (0..*BACKGROUND_LINK_COUNT)
                        .map(|_| fork.create_point())
                        .collect();
                    assert!(
                        background_ids.len() >= 10 + n.div_ceil(10),
                        "query workloads require at least 10 + ceil(N/10) background links"
                    );
                    // Create links with distributed sources
                    for i in 1..=n as u64 {
                        let (source, target) = spacetimedb_vs_doublets::workload::distributed_link(
                            (i - 1) as usize,
                            &background_ids,
                            true,
                        );
                        fork.create(source, target);
                    }
                    let start = Instant::now();
                    for &src in background_ids.iter().take(n.min(10)) {
                        std::hint::black_box(fork.query_by_source(src));
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_united_query_by_source(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsUnitedVolatileBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("query_by_source/Doublets_United_Volatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    let background_ids: Vec<u64> = (0..*BACKGROUND_LINK_COUNT)
                        .map(|_| fork.create_point())
                        .collect();
                    assert!(
                        background_ids.len() >= 10 + n.div_ceil(10),
                        "query workloads require at least 10 + ceil(N/10) background links"
                    );
                    // Create links with distributed sources
                    for i in 1..=n as u64 {
                        let (source, target) = spacetimedb_vs_doublets::workload::distributed_link(
                            (i - 1) as usize,
                            &background_ids,
                            true,
                        );
                        fork.create(source, target);
                    }
                    let start = Instant::now();
                    for &src in background_ids.iter().take(n.min(10)) {
                        std::hint::black_box(fork.query_by_source(src));
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_split_query_by_source(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsSplitVolatileBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("query_by_source/Doublets_Split_Volatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    let background_ids: Vec<u64> = (0..*BACKGROUND_LINK_COUNT)
                        .map(|_| fork.create_point())
                        .collect();
                    assert!(
                        background_ids.len() >= 10 + n.div_ceil(10),
                        "query workloads require at least 10 + ceil(N/10) background links"
                    );
                    // Create links with distributed sources
                    for i in 1..=n as u64 {
                        let (source, target) = spacetimedb_vs_doublets::workload::distributed_link(
                            (i - 1) as usize,
                            &background_ids,
                            true,
                        );
                        fork.create(source, target);
                    }
                    let start = Instant::now();
                    for &src in background_ids.iter().take(n.min(10)) {
                        std::hint::black_box(fork.query_by_source(src));
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_united_non_volatile_query_by_source(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsUnitedNonVolatileBenched::setup(
        "/tmp/bench_united_non_volatile_query_by_source.links".to_string(),
    );
    c.bench_with_input(
        BenchmarkId::new("query_by_source/Doublets_United_NonVolatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    let background_ids: Vec<u64> = (0..*BACKGROUND_LINK_COUNT)
                        .map(|_| fork.create_point())
                        .collect();
                    assert!(
                        background_ids.len() >= 10 + n.div_ceil(10),
                        "query workloads require at least 10 + ceil(N/10) background links"
                    );
                    // Create links with distributed sources
                    for i in 1..=n as u64 {
                        let (source, target) = spacetimedb_vs_doublets::workload::distributed_link(
                            (i - 1) as usize,
                            &background_ids,
                            true,
                        );
                        fork.create(source, target);
                    }
                    let start = Instant::now();
                    for &src in background_ids.iter().take(n.min(10)) {
                        std::hint::black_box(fork.query_by_source(src));
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_split_non_volatile_query_by_source(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsSplitNonVolatileBenched::setup((
        "/tmp/bench_split_non_volatile_query_by_source_data.links".to_string(),
        "/tmp/bench_split_non_volatile_query_by_source_index.links".to_string(),
    ));
    c.bench_with_input(
        BenchmarkId::new("query_by_source/Doublets_Split_NonVolatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    let background_ids: Vec<u64> = (0..*BACKGROUND_LINK_COUNT)
                        .map(|_| fork.create_point())
                        .collect();
                    assert!(
                        background_ids.len() >= 10 + n.div_ceil(10),
                        "query workloads require at least 10 + ceil(N/10) background links"
                    );
                    // Create links with distributed sources
                    for i in 1..=n as u64 {
                        let (source, target) = spacetimedb_vs_doublets::workload::distributed_link(
                            (i - 1) as usize,
                            &background_ids,
                            true,
                        );
                        fork.create(source, target);
                    }
                    let start = Instant::now();
                    for &src in background_ids.iter().take(n.min(10)) {
                        std::hint::black_box(fork.query_by_source(src));
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

// ===================== QUERY BY TARGET =====================

fn spacetimedb_query_by_target(c: &mut Criterion) {
    if !backend_enabled("spacetimedb") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = SpacetimeDbBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("query_by_target/SpacetimeDB", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    let background_ids: Vec<u64> = (0..*BACKGROUND_LINK_COUNT)
                        .map(|_| fork.create_point())
                        .collect();
                    assert!(
                        background_ids.len() >= 10 + n.div_ceil(10),
                        "query workloads require at least 10 + ceil(N/10) background links"
                    );
                    // Create links with distributed targets
                    for i in 1..=n as u64 {
                        let (source, target) = spacetimedb_vs_doublets::workload::distributed_link(
                            (i - 1) as usize,
                            &background_ids,
                            false,
                        );
                        fork.create(source, target);
                    }
                    let start = Instant::now();
                    for &tgt in background_ids.iter().take(n.min(10)) {
                        std::hint::black_box(fork.query_by_target(tgt));
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_united_query_by_target(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsUnitedVolatileBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("query_by_target/Doublets_United_Volatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    let background_ids: Vec<u64> = (0..*BACKGROUND_LINK_COUNT)
                        .map(|_| fork.create_point())
                        .collect();
                    assert!(
                        background_ids.len() >= 10 + n.div_ceil(10),
                        "query workloads require at least 10 + ceil(N/10) background links"
                    );
                    // Create links with distributed targets
                    for i in 1..=n as u64 {
                        let (source, target) = spacetimedb_vs_doublets::workload::distributed_link(
                            (i - 1) as usize,
                            &background_ids,
                            false,
                        );
                        fork.create(source, target);
                    }
                    let start = Instant::now();
                    for &tgt in background_ids.iter().take(n.min(10)) {
                        std::hint::black_box(fork.query_by_target(tgt));
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_split_query_by_target(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsSplitVolatileBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("query_by_target/Doublets_Split_Volatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    let background_ids: Vec<u64> = (0..*BACKGROUND_LINK_COUNT)
                        .map(|_| fork.create_point())
                        .collect();
                    assert!(
                        background_ids.len() >= 10 + n.div_ceil(10),
                        "query workloads require at least 10 + ceil(N/10) background links"
                    );
                    // Create links with distributed targets
                    for i in 1..=n as u64 {
                        let (source, target) = spacetimedb_vs_doublets::workload::distributed_link(
                            (i - 1) as usize,
                            &background_ids,
                            false,
                        );
                        fork.create(source, target);
                    }
                    let start = Instant::now();
                    for &tgt in background_ids.iter().take(n.min(10)) {
                        std::hint::black_box(fork.query_by_target(tgt));
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_united_non_volatile_query_by_target(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsUnitedNonVolatileBenched::setup(
        "/tmp/bench_united_non_volatile_query_by_target.links".to_string(),
    );
    c.bench_with_input(
        BenchmarkId::new("query_by_target/Doublets_United_NonVolatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    let background_ids: Vec<u64> = (0..*BACKGROUND_LINK_COUNT)
                        .map(|_| fork.create_point())
                        .collect();
                    assert!(
                        background_ids.len() >= 10 + n.div_ceil(10),
                        "query workloads require at least 10 + ceil(N/10) background links"
                    );
                    // Create links with distributed targets
                    for i in 1..=n as u64 {
                        let (source, target) = spacetimedb_vs_doublets::workload::distributed_link(
                            (i - 1) as usize,
                            &background_ids,
                            false,
                        );
                        fork.create(source, target);
                    }
                    let start = Instant::now();
                    for &tgt in background_ids.iter().take(n.min(10)) {
                        std::hint::black_box(fork.query_by_target(tgt));
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_split_non_volatile_query_by_target(c: &mut Criterion) {
    if !backend_enabled("doublets") {
        return;
    }
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = DoubletsSplitNonVolatileBenched::setup((
        "/tmp/bench_split_non_volatile_query_by_target_data.links".to_string(),
        "/tmp/bench_split_non_volatile_query_by_target_index.links".to_string(),
    ));
    c.bench_with_input(
        BenchmarkId::new("query_by_target/Doublets_Split_NonVolatile", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    let background_ids: Vec<u64> = (0..*BACKGROUND_LINK_COUNT)
                        .map(|_| fork.create_point())
                        .collect();
                    assert!(
                        background_ids.len() >= 10 + n.div_ceil(10),
                        "query workloads require at least 10 + ceil(N/10) background links"
                    );
                    // Create links with distributed targets
                    for i in 1..=n as u64 {
                        let (source, target) = spacetimedb_vs_doublets::workload::distributed_link(
                            (i - 1) as usize,
                            &background_ids,
                            false,
                        );
                        fork.create(source, target);
                    }
                    let start = Instant::now();
                    for &tgt in background_ids.iter().take(n.min(10)) {
                        std::hint::black_box(fork.query_by_target(tgt));
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

criterion_group!(
    create_benches,
    spacetimedb_create,
    doublets_united_create,
    doublets_split_create,
    doublets_united_non_volatile_create,
    doublets_split_non_volatile_create,
);

criterion_group!(
    delete_benches,
    spacetimedb_delete,
    doublets_united_delete,
    doublets_split_delete,
    doublets_united_non_volatile_delete,
    doublets_split_non_volatile_delete,
);

criterion_group!(
    update_benches,
    spacetimedb_update,
    doublets_united_update,
    doublets_split_update,
    doublets_united_non_volatile_update,
    doublets_split_non_volatile_update,
);

criterion_group!(
    query_all_benches,
    spacetimedb_query_all,
    doublets_united_query_all,
    doublets_split_query_all,
    doublets_united_non_volatile_query_all,
    doublets_split_non_volatile_query_all,
);

criterion_group!(
    query_by_id_benches,
    spacetimedb_query_by_id,
    doublets_united_query_by_id,
    doublets_split_query_by_id,
    doublets_united_non_volatile_query_by_id,
    doublets_split_non_volatile_query_by_id,
);

criterion_group!(
    query_by_source_benches,
    spacetimedb_query_by_source,
    doublets_united_query_by_source,
    doublets_split_query_by_source,
    doublets_united_non_volatile_query_by_source,
    doublets_split_non_volatile_query_by_source,
);

criterion_group!(
    query_by_target_benches,
    spacetimedb_query_by_target,
    doublets_united_query_by_target,
    doublets_split_query_by_target,
    doublets_united_non_volatile_query_by_target,
    doublets_split_non_volatile_query_by_target,
);

criterion_main!(
    create_benches,
    delete_benches,
    update_benches,
    query_all_benches,
    query_by_id_benches,
    query_by_source_benches,
    query_by_target_benches,
);
