# Comparisons.SpacetimeDBVSDoublets

Benchmark comparing [SpacetimeDB 2](https://github.com/clockworklabs/SpacetimeDB) vs [Doublets](https://github.com/linksplatform/doublets-rs) performance for basic CRUD operations with links.

SpacetimeDB is benchmarked via its SQLite storage backend (the same engine SpacetimeDB 2 uses internally). Doublets is benchmarked with its in-memory (volatile) storage variants.

## Benchmark Operations

| Operation | Description |
|---|---|
| Create | Create a self-referential point link (id == source == target) |
| Delete | Delete links by id |
| Update | Update link source and target |
| Query All | Retrieve all links (`[*, *, *]`) |
| Query by Id | Retrieve a link by id |
| Query by Source | Retrieve all links with a given source |
| Query by Target | Retrieve all links with a given target |

## Backends Benchmarked

### SpacetimeDB (SQLite backend)
- **SpacetimeDB Memory** — in-memory SQLite with B-tree indexes on `id`, `source`, `target`

SpacetimeDB 2 uses SQLite as its underlying data store for persistent tables. This benchmark measures the performance of SpacetimeDB's storage layer (SQLite + WAL mode) for link CRUD operations.

### Doublets
- **Doublets United Volatile** — in-memory store; links stored as contiguous `(index, source, target)` units
- **Doublets Split Volatile** — in-memory store; separate data and index memory regions

Doublets is a custom in-memory doublet link data structure with O(1) lookup by id and O(log n + k) traversal by source/target using balanced tree indexes.

## Benchmark Background

Each benchmark iteration pre-populates the database with background links to simulate a realistic database state:

- **Background links**: `BACKGROUND_LINK_COUNT` (default: 3000) — already present before measurement
- **Benchmark links**: `BENCHMARK_LINK_COUNT` (default: 1000) — the operations being measured

## Results

> _Benchmark results will be automatically generated and committed here by CI when changes are merged to main._

<!--RESULTS_TABLE_PLACEHOLDER-->

## Operation Complexity

| Operation | SpacetimeDB (SQLite) | Doublets United | Doublets Split |
|---|---|---|---|
| Create | O(log n) + disk I/O | O(log n) | O(log n) |
| Delete | O(log n) + disk I/O | O(log n) | O(log n) |
| Update | O(log n) + disk I/O | O(log n) | O(log n) |
| Query All | O(n) + disk I/O | O(n) | O(n) |
| Query by Id | O(log n) | O(1) | O(1) |
| Query by Source | O(log n + k) | O(log n + k) | O(log n + k) |
| Query by Target | O(log n + k) | O(log n + k) | O(log n + k) |

## Related Benchmarks

- [Neo4j vs Doublets](https://github.com/linksplatform/Comparisons.Neo4jVSDoublets)
- [PostgreSQL vs Doublets](https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets)
- [SQLite vs Doublets](https://github.com/linksplatform/Comparisons.SQLiteVSDoublets)

## Running Benchmarks

### Prerequisites

- Rust nightly-2022-08-22 (see `rust/rust-toolchain.toml`)

### Run benchmarks

```bash
cd rust

# Full benchmark run (1000 links, 3000 background)
cargo bench --bench bench -- --output-format bencher | tee out.txt

# Quick benchmark run (CI scale)
BENCHMARK_LINK_COUNT=10 BACKGROUND_LINK_COUNT=100 cargo bench --bench bench

# Generate charts from results
python3 out.py
```

### Run tests

```bash
cd rust
cargo test --release
```

### Code quality

```bash
cd rust
cargo fmt --all
cargo clippy --all-targets
```

## Project Structure

```
.
├── rust/
│   ├── Cargo.toml              # Package manifest with pinned dependencies
│   ├── rust-toolchain.toml     # Pinned Rust nightly toolchain
│   ├── rustfmt.toml            # Rust formatting config
│   ├── out.py                  # Chart generation script (matplotlib)
│   ├── src/
│   │   ├── lib.rs              # Links trait, constants (BENCHMARK_LINK_COUNT, BACKGROUND_LINK_COUNT)
│   │   ├── spacetimedb_impl.rs # SpacetimeDB SQLite client (implements Links)
│   │   ├── doublets_impl.rs    # Doublets store adapters (implements Links)
│   │   ├── exclusive.rs        # Exclusive<T> wrapper for interior mutability
│   │   ├── fork.rs             # Fork<B> — benchmark iteration isolation
│   │   └── benched/
│   │       ├── mod.rs          # Benched trait (setup/fork/unfork lifecycle)
│   │       ├── spacetimedb_benched.rs  # Benched impl for SpacetimeDB
│   │       └── doublets_benched.rs     # Benched impls for Doublets stores
│   └── benches/
│       └── bench.rs            # Criterion benchmark suite (7 operations x 3 backends)
└── .github/
    └── workflows/
        └── rust-benchmark.yml  # CI: test on 3 OS + benchmark + chart generation
```

## License

[Unlicense](LICENSE) — Public Domain
