//! SpacetimeDB vs Doublets benchmark suite.
//!
//! Runs criterion benchmarks for basic CRUD operations with links,
//! comparing SpacetimeDB's SQLite backend against Doublets in-memory stores.
//!
//! Run benchmarks:
//! ```bash
//! cargo bench --bench bench -- --output-format bencher | tee out.txt
//! ```
//!
//! Configure scale via environment variables:
//! - `BENCHMARK_LINK_COUNT` — links to create/update/delete per iteration (default: 1000)
//! - `BACKGROUND_LINK_COUNT` — pre-populated links for realistic DB state (default: 3000)

#![feature(allocator_api)]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use spacetimedb_vs_doublets::{
    benched::{
        Benched, DoubletsSplitVolatileBenched, DoubletsUnitedVolatileBenched,
        SpacetimeDbMemoryBenched,
    },
    Links, BACKGROUND_LINK_COUNT, BENCHMARK_LINK_COUNT,
};
use std::time::{Duration, Instant};

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
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = SpacetimeDbMemoryBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("create/SpacetimeDB_Memory", count),
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

// ===================== DELETE =====================

fn spacetimedb_delete(c: &mut Criterion) {
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = SpacetimeDbMemoryBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("delete/SpacetimeDB_Memory", count),
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

// ===================== UPDATE =====================

fn spacetimedb_update(c: &mut Criterion) {
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = SpacetimeDbMemoryBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("update/SpacetimeDB_Memory", count),
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

// ===================== QUERY ALL =====================

fn spacetimedb_query_all(c: &mut Criterion) {
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = SpacetimeDbMemoryBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("query_all/SpacetimeDB_Memory", count),
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

// ===================== QUERY BY ID =====================

fn spacetimedb_query_by_id(c: &mut Criterion) {
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = SpacetimeDbMemoryBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("query_by_id/SpacetimeDB_Memory", count),
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
                        let _ = fork.query_by_id(id);
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_united_query_by_id(c: &mut Criterion) {
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
                        let _ = fork.query_by_id(id);
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_split_query_by_id(c: &mut Criterion) {
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
                        let _ = fork.query_by_id(id);
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
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = SpacetimeDbMemoryBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("query_by_source/SpacetimeDB_Memory", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    // Create links with distributed sources
                    for i in 1..=n as u64 {
                        fork.create(i % 10 + 1, i % 7 + 1);
                    }
                    let start = Instant::now();
                    for src in 1..=(n.min(10) as u64) {
                        let _ = fork.query_by_source(src);
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_united_query_by_source(c: &mut Criterion) {
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
                    setup_background!(fork);
                    // Create links with distributed sources
                    for i in 1..=n as u64 {
                        fork.create(i % 10 + 1, i % 7 + 1);
                    }
                    let start = Instant::now();
                    for src in 1..=(n.min(10) as u64) {
                        let _ = fork.query_by_source(src);
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_split_query_by_source(c: &mut Criterion) {
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
                    setup_background!(fork);
                    // Create links with distributed sources
                    for i in 1..=n as u64 {
                        fork.create(i % 10 + 1, i % 7 + 1);
                    }
                    let start = Instant::now();
                    for src in 1..=(n.min(10) as u64) {
                        let _ = fork.query_by_source(src);
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
    let count = *BENCHMARK_LINK_COUNT;
    let mut benched = SpacetimeDbMemoryBenched::setup(());
    c.bench_with_input(
        BenchmarkId::new("query_by_target/SpacetimeDB_Memory", count),
        &count,
        |b, &n| {
            b.iter_custom(|iters| {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let mut fork = Benched::fork(&mut benched);
                    setup_background!(fork);
                    // Create links with distributed targets
                    for i in 1..=n as u64 {
                        fork.create(i % 7 + 1, i % 10 + 1);
                    }
                    let start = Instant::now();
                    for tgt in 1..=(n.min(10) as u64) {
                        let _ = fork.query_by_target(tgt);
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_united_query_by_target(c: &mut Criterion) {
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
                    setup_background!(fork);
                    // Create links with distributed targets
                    for i in 1..=n as u64 {
                        fork.create(i % 7 + 1, i % 10 + 1);
                    }
                    let start = Instant::now();
                    for tgt in 1..=(n.min(10) as u64) {
                        let _ = fork.query_by_target(tgt);
                    }
                    total += start.elapsed();
                }
                total
            });
        },
    );
}

fn doublets_split_query_by_target(c: &mut Criterion) {
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
                    setup_background!(fork);
                    // Create links with distributed targets
                    for i in 1..=n as u64 {
                        fork.create(i % 7 + 1, i % 10 + 1);
                    }
                    let start = Instant::now();
                    for tgt in 1..=(n.min(10) as u64) {
                        let _ = fork.query_by_target(tgt);
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
);

criterion_group!(
    delete_benches,
    spacetimedb_delete,
    doublets_united_delete,
    doublets_split_delete,
);

criterion_group!(
    update_benches,
    spacetimedb_update,
    doublets_united_update,
    doublets_split_update,
);

criterion_group!(
    query_all_benches,
    spacetimedb_query_all,
    doublets_united_query_all,
    doublets_split_query_all,
);

criterion_group!(
    query_by_id_benches,
    spacetimedb_query_by_id,
    doublets_united_query_by_id,
    doublets_split_query_by_id,
);

criterion_group!(
    query_by_source_benches,
    spacetimedb_query_by_source,
    doublets_united_query_by_source,
    doublets_split_query_by_source,
);

criterion_group!(
    query_by_target_benches,
    spacetimedb_query_by_target,
    doublets_united_query_by_target,
    doublets_split_query_by_target,
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
