---
bump: minor
---

### Added

- **SpacetimeDB 2 vs Doublets benchmark** (`rust/`): New Rust benchmark suite comparing SpacetimeDB 2.0 against Doublets in-memory link stores for basic CRUD operations.

  - **7 benchmark operations**: Create, Delete, Update, Query All, Query by Id, Query by Source, Query by Target
  - **3 backends**: SpacetimeDB 2.0 (via official `spacetimedb-sdk` Rust crate), Doublets United Volatile, Doublets Split Volatile
  - **Configurable scale**: `BENCHMARK_LINK_COUNT` and `BACKGROUND_LINK_COUNT` environment variables
  - **Criterion harness**: Uses criterion 0.3.6 with custom `iter_custom` timing to exclude setup/teardown from measurements
  - **Fork/unfork lifecycle**: Each iteration starts from a clean database state with pre-populated background links

- **`spacetime-module/`**: SpacetimeDB WebAssembly module using the official `spacetimedb` Rust crate, defining the `links` table and CRUD reducers.

- **`rust/src/module_bindings/`**: Client-side SDK bindings for the links module (equivalent to `spacetime generate --lang rust` output).

- **`rust/src/spacetimedb_impl.rs`**: SpacetimeDB 2.0 implementation using the official `spacetimedb-sdk` Rust crate. Connects to a running SpacetimeDB server, subscribes to the links table, and calls reducers for CRUD operations.

- **`rust/src/doublets_impl.rs`**: Doublets store adapters implementing the `Links` trait for both United Volatile and Split Volatile storage layouts.

- **`rust/benches/bench.rs`**: Criterion benchmark suite with 21 benchmark functions (7 operations × 3 backends).

- **`rust/out.py`**: Python script for generating comparison charts (linear and log scale PNG) and a Markdown results table from benchmark output.

- **`.github/workflows/rust-benchmark.yml`**: GitHub Actions CI workflow that installs the SpacetimeDB CLI, starts a local server, publishes the module, runs tests on Ubuntu/macOS/Windows, and generates benchmark charts on push to main.

- **Updated `README.md`**: Benchmark description, operation complexity table, and usage instructions.
