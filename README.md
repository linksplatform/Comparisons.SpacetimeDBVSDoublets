# Comparisons.SpacetimeDBVSDoublets

Benchmark comparing [SpacetimeDB 2](https://github.com/clockworklabs/SpacetimeDB) vs [Doublets](https://github.com/linksplatform/doublets-rs) performance for basic CRUD operations with links.

SpacetimeDB is benchmarked using the official `spacetimedb-sdk` Rust crate connected to a running SpacetimeDB 2.0 server. Doublets is benchmarked with both in-memory (volatile) and file-backed (non-volatile) storage variants.

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

### SpacetimeDB
- **SpacetimeDB** — connects to a running SpacetimeDB 2.0 server via the official `spacetimedb-sdk` Rust crate; uses the `links` table defined in the `spacetime-module` WebAssembly module

The benchmark uses the official SpacetimeDB Rust client SDK, calling reducers to mutate data and reading from the client-side subscription cache.

### Doublets
- **Doublets United Volatile** — in-memory store; links stored as contiguous `(index, source, target)` units
- **Doublets Split Volatile** — in-memory store; separate data and index memory regions
- **Doublets United NonVolatile** — file-backed store; same contiguous layout but memory-mapped to a single file; data persists to disk
- **Doublets Split NonVolatile** — file-backed store; separate data and index files; both memory-mapped; data persists to disk

Doublets uses a recursive-less size-balanced tree for O(1) lookup by id and O(log n + k) traversal by source/target. The file-backed variants use `memmap2` for memory-mapped file I/O, flushing changes to disk on drop via `sync_all()`. See [`rust/doublets-patched/PATCHES.md`](rust/doublets-patched/PATCHES.md) for why a local patched copy is used instead of the published crates.io version.

## Benchmark Background

Each benchmark iteration pre-populates the database with background links to simulate a realistic database state:

- **Background links**: `BACKGROUND_LINK_COUNT` (default: 3000) — already present before measurement
- **Benchmark links**: `BENCHMARK_LINK_COUNT` (default: 1000) — the operations being measured

## Results

> _Benchmark results will be automatically generated and committed here by CI when changes are merged to main._

<!--RESULTS_TABLE_PLACEHOLDER-->

## Operation Complexity

| Operation | SpacetimeDB | Doublets United | Doublets Split |
|---|---|---|---|
| Create | O(log n) + network | O(log n) | O(log n) |
| Delete | O(log n) + network | O(log n) | O(log n) |
| Update | O(log n) + network | O(log n) | O(log n) |
| Query All | O(n) cache read | O(n) | O(n) |
| Query by Id | O(n) cache scan | O(1) | O(1) |
| Query by Source | O(n) cache scan | O(log n + k) | O(log n + k) |
| Query by Target | O(n) cache scan | O(log n + k) | O(log n + k) |

The algorithmic complexity is the same for volatile and non-volatile Doublets variants. The non-volatile variants have additional I/O overhead due to memory-mapped file writes (flushed to disk on drop).

## Related Benchmarks

- [Neo4j vs Doublets](https://github.com/linksplatform/Comparisons.Neo4jVSDoublets)
- [PostgreSQL vs Doublets](https://github.com/linksplatform/Comparisons.PostgreSQLVSDoublets)
- [SQLite vs Doublets](https://github.com/linksplatform/Comparisons.SQLiteVSDoublets)

## Running Benchmarks

### Prerequisites

- Rust nightly (see `rust/rust-toolchain.toml`)
- SpacetimeDB CLI: `curl -sSf https://install.spacetimedb.com | sh`

### Start SpacetimeDB server and publish module

```bash
# Start the local SpacetimeDB server
spacetime start &

# Build and publish the links module
spacetime build --project-path spacetime-module
spacetime publish --project-path spacetime-module benchmark-links
```

### Run benchmarks

```bash
cd rust

# Full benchmark run (1000 links, 3000 background)
SPACETIMEDB_URI=http://localhost:3000 SPACETIMEDB_DB=benchmark-links \
  cargo bench --bench bench -- --output-format bencher | tee out.txt

# Quick benchmark run (CI scale)
BENCHMARK_LINK_COUNT=10 BACKGROUND_LINK_COUNT=100 \
SPACETIMEDB_URI=http://localhost:3000 SPACETIMEDB_DB=benchmark-links \
  cargo bench --bench bench

# Generate charts from results
python3 out.py
```

### Run tests

```bash
cd rust
SPACETIMEDB_URI=http://localhost:3000 SPACETIMEDB_DB=benchmark-links cargo test
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
├── spacetime-module/           # SpacetimeDB WASM module (links table + reducers)
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs              # Table definition and reducers using `spacetimedb` crate
├── rust/
│   ├── Cargo.toml              # Package manifest with pinned dependencies
│   ├── doublets-patched/       # Local patches to doublets-rs for modern nightly compatibility
│   │   └── PATCHES.md          # Documents why patches are needed and what was changed
│   ├── rust-toolchain.toml     # Pinned Rust nightly toolchain
│   ├── rustfmt.toml            # Rust formatting config
│   ├── out.py                  # Chart generation script (matplotlib)
│   ├── src/
│   │   ├── lib.rs              # Links trait, constants (BENCHMARK_LINK_COUNT, BACKGROUND_LINK_COUNT)
│   │   ├── module_bindings/    # spacetimedb-sdk client bindings for the links module
│   │   ├── spacetimedb_impl.rs # SpacetimeDB SDK client (implements Links)
│   │   ├── doublets_impl.rs    # Doublets store adapters (implements Links)
│   │   ├── exclusive.rs        # Exclusive<T> wrapper for interior mutability
│   │   ├── fork.rs             # Fork<B> — benchmark iteration isolation
│   │   └── benched/
│   │       ├── mod.rs          # Benched trait (setup/fork/unfork lifecycle)
│   │       ├── spacetimedb_benched.rs  # Benched impl for SpacetimeDB
│   │       └── doublets_benched.rs     # Benched impls for Doublets stores
│   └── benches/
│       └── bench.rs            # Criterion benchmark suite (7 operations x 5 backends)
└── .github/
    └── workflows/
        └── rust-benchmark.yml  # CI: test on 3 OS + benchmark + chart generation
```

## License

[Unlicense](LICENSE) — Public Domain
