---
bump: patch
---

### Fixed

- **Rust nightly compatibility** (`rust/`): the benchmark no longer fails to compile on current nightly toolchains.

  - `ethnum` bumped `1.5.2` → `1.5.3` in `rust/Cargo.lock` and `rust/spacetime-module/Cargo.lock`. `ethnum 1.5.2` built `core::num::TryFromIntError` with `unsafe { mem::transmute(()) }`, which now fails with `error[E0512]: cannot transmute between types of different sizes` because `TryFromIntError` is no longer zero-sized. `1.5.3` constructs the error via `u8::try_from(-1i8).unwrap_err()` instead.
  - Vendored `platform-data` (`rust/doublets-patched/dev-deps/data-rs`): added `impl Residual<()> for Flow` plus the `try_trait_v2_residual` feature gate. Nightly's `Try::Residual` gained a `Residual<Self::Output>` bound, so the custom `Flow` try-type failed with `error[E0277]: the trait bound `Flow: Residual<()>` is not satisfied`.

- **Benchmark chart generation** (`rust/out.py`): the bencher-output parser matched nothing and silently produced no charts. The pattern expected four slash-separated components, but the benchmarks build ids as `BenchmarkId::new("<operation>/<variant>", <size>)`, which Criterion renders as three (`<operation>/<variant>/<size>`). `out.py` printed `No benchmark data found in out.txt` and exited `0`, so the CI "Generate charts" step produced empty artifacts without failing. The `Doublets_United_NonVolatile` and `Doublets_Split_NonVolatile` variants were also missing from the chart label/colour maps, so they would have been dropped even once parsing worked.

### Added

- **`rust/test_out_py.py`**: regression tests for the benchmark result parser, covering three-component benchmark ids, operations containing underscores, the non-volatile variants, and the invariant that every measured variant has a chart label and colour. Wired into the `test` job of `.github/workflows/rust-benchmark.yml` as a pure-stdlib `unittest` run.

### Changed

- **Dependency updates** (`rust/Cargo.lock`, `rust/spacetime-module/Cargo.lock`, `Cargo.lock`): all dependencies refreshed to their latest compatible versions, clearing every open Dependabot security alert.

  - `anymap 0.12.1` → `anymap3 1.1.0` (critical advisory)
  - `protobuf 2.28.0` → `3.7.2`
  - `rand 0.9.2` → `0.9.5`, `rand 0.8.6` → `0.8.8`
  - `atty` removed entirely (pulled in transitively by the old criterion)
  - `spacetimedb-sdk` / `spacetimedb` `2.0.3` → `2.10.1`
  - `tokio 1.48.0` → `1.53.1`, `syn 2.0.111` → `3.0.5`

- **criterion `=0.3.6` → `0.8`** (`rust/Cargo.toml`), with the `cargo_bench_support` feature enabled explicitly since criterion 0.5 no longer implies it under `default-features = false`.

- **`rust/src/module_bindings/`** regenerated with `spacetime generate` 2.10.1. The generated table module is now `links_table` (`LinksTableAccess` / `LinksTableHandle`) instead of `link_table`, and all binding types gained `Debug` derives — required by the 2.10.1 SDK, which bounds `RemoteModule` on `Debug`. `rust/src/spacetimedb_impl.rs` updated to the new import path.
