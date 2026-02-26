---
bump: minor
---

### Added

- **SpacetimeDB 2 vs Doublets benchmark** (`rust/`): New Rust benchmark suite comparing the real SpacetimeDB 2.0 engine against Doublets in-memory link stores for basic CRUD operations.

  - **7 benchmark operations**: Create, Delete, Update, Query All, Query by Id, Query by Source, Query by Target
  - **3 backends**: SpacetimeDB 2.0 in-memory (via `RelationalDB`/`TestDB`), Doublets United Volatile, Doublets Split Volatile
  - **Configurable scale**: `BENCHMARK_LINK_COUNT` and `BACKGROUND_LINK_COUNT` environment variables
  - **Criterion harness**: Uses criterion 0.3.6 with custom `iter_custom` timing to exclude setup/teardown from measurements
  - **Fork/unfork lifecycle**: Each iteration starts from a clean database state with pre-populated background links

- **`rust/src/lib.rs`**: `Links` trait as the shared interface for both SpacetimeDB and Doublets backends; `Benched` trait for benchmark lifecycle management.

- **`rust/src/spacetimedb_impl.rs`**: Real SpacetimeDB 2.0 engine implementation using `spacetimedb-core` with `TestDB` (in-memory `RelationalDB`, no SQLite). Verifies engine version at startup and fails if backend ≠ SpacetimeDB 2.0+.

- **`rust/src/doublets_impl.rs`**: Doublets store adapters implementing the `Links` trait for both United Volatile and Split Volatile storage layouts.

- **`rust/benches/bench.rs`**: Criterion benchmark suite with 21 benchmark functions (7 operations × 3 backends).

- **`rust/out.py`**: Python script for generating comparison charts (linear and log scale PNG) and a Markdown results table from benchmark output.

- **`.github/workflows/rust-benchmark.yml`**: GitHub Actions CI workflow that runs tests on Ubuntu/macOS/Windows and generates benchmark charts on push to main.

- **Updated `README.md`**: Benchmark description, operation complexity table, and usage instructions.
