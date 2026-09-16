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

The numbers below represent the amount of time (ns) a single benchmark iteration takes.

- The first chart shows time in a pixel (linear) scale. Doublets bars are drawn with a
  minimum visible width, otherwise they would not be visible next to SpacetimeDB.
- The second chart shows time in a logarithmic scale, to see the difference clearly,
  because it is around 3-5 orders of magnitude.

Charts and the table are recalculated by the
[Rust Benchmark workflow](.github/workflows/rust-benchmark.yml) on every push to `main`
and committed back to this repository, so the results are visible here without running
the benchmark locally.

### Rust

![Image of Rust benchmark (pixel scale)](https://github.com/linksplatform/Comparisons.SpacetimeDBVSDoublets/blob/main/docs/benchmarks/bench_rust.png?raw=true)
![Image of Rust benchmark (log scale)](https://github.com/linksplatform/Comparisons.SpacetimeDBVSDoublets/blob/main/docs/benchmarks/bench_rust_log_scale.png?raw=true)

### Raw benchmark results (all numbers are in nanoseconds)

<!--BENCHMARK_RESULTS_START-->
_Generated 2026-09-15 22:53 UTC by [GitHub Actions run 35030102133](https://github.com/linksplatform/Comparisons.SpacetimeDBVSDoublets/actions/runs/35030102133) — 1000 benchmarked links, 3000 background links._

| Operation       | Doublets United Volatile | Doublets United NonVolatile | Doublets Split Volatile | Doublets Split NonVolatile | SpacetimeDB   |
|-----------------|--------------------------|-----------------------------|-------------------------|----------------------------|---------------|
| Create          | 73090 (34741.6x faster)  | 70181 (36181.6x faster)     | 48458 (52401.3x faster) | 48606 (52241.7x faster)    | 2539260579    |
| Update          | 214541 (12035.6x faster) | 214497 (12038.1x faster)    | 38161 (67664.3x faster) | 37990 (67968.9x faster)    | 2582139227    |
| Delete          | 159655 (7996.4x faster)  | 159342 (8012.1x faster)     | 96915 (13173.0x faster) | 96309 (13255.9x faster)    | 1276662577    |
| Query All       | 21170 (1.1x faster)      | 21611 (1.0x faster)         | 25503 (1.1x slower)     | 25649 (1.1x slower)        | 22522         |
| Query by Id     | 53 (211157.5x faster)    | 53 (211157.5x faster)       | 1019 (10982.7x faster)  | 1019 (10982.7x faster)     | 11191350      |
| Query by Source | 1463 (139.2x faster)     | 1462 (139.3x faster)        | 461 (441.8x faster)     | 464 (438.9x faster)        | 203651        |
| Query by Target | 1562 (116.7x faster)     | 1539 (118.4x faster)        | 392 (464.9x faster)     | 394 (462.6x faster)        | 182251        |
<!--BENCHMARK_RESULTS_END-->

Each Doublets cell is annotated with how many times faster (or slower) it is than
SpacetimeDB for the same operation.

## Conclusion

Doublets is an embedded store: an operation is a few pointer dereferences and tree
rotations in memory (or in a memory-mapped file), while every SpacetimeDB operation is a
reducer call over a WebSocket connection to a separate process, and every query is served
from the client-side subscription cache. The measured difference is dominated by that
architectural difference rather than by the data structures themselves.

To get fresh numbers, please fork the repository and rerun the benchmark in GitHub Actions.

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

- Rust nightly, pinned in `rust/rust-toolchain.toml` (`rustup` installs it automatically)
- SpacetimeDB CLI: `curl -sSf https://install.spacetimedb.com | sh`

### Start SpacetimeDB server and publish module

```bash
# Start the local SpacetimeDB server
spacetime start &

# Build and publish the links module
spacetime build --project-path rust/spacetime-module
spacetime publish --project-path rust/spacetime-module --yes benchmark-links
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

# Generate the results table and charts from out.txt
python3 out.py out.txt --results results.md

# Regenerate everything the CI publishes: results.md, docs/benchmarks/ charts
# and the results section of README.md
python3 out.py out.txt --results results.md --readme ../README.md \
  --docs-dir ../docs/benchmarks
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

# Unit tests for the results reporting pipeline (no benchmark run required)
python3 -m unittest test_out -v
```

## Project Structure

```
.
├── docs/
│   └── benchmarks/             # Benchmark charts published by CI and shown above
│       ├── bench_rust.png
│       └── bench_rust_log_scale.png
├── rust/
│   ├── spacetime-module/       # SpacetimeDB WASM module (links table + reducers)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs          # Table definition and reducers using `spacetimedb` crate
│   ├── Cargo.toml              # Package manifest with pinned dependencies
│   ├── doublets-patched/       # Local patches to doublets-rs for modern nightly compatibility
│   │   └── PATCHES.md          # Documents why patches are needed and what was changed
│   ├── rust-toolchain.toml     # Pinned Rust nightly toolchain
│   ├── rustfmt.toml            # Rust formatting config
│   ├── out.py                  # Results table, charts and README update
│   ├── test_out.py             # Unit tests for out.py
│   ├── results.md              # Generated results table (committed by CI)
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
        └── rust-benchmark.yml  # CI: test on Linux/macOS, benchmark, publish results
```

## License

[Unlicense](LICENSE) — Public Domain
