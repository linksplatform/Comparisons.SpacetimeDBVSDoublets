---
bump: patch
---

### Fixed

- **Rust nightly compatibility** (`rust/`): the two errors that forced the `nightly-2026-04-14` pin are fixed at the source, so the pin moves forward to `nightly-2026-09-16` (`rust/rust-toolchain.toml` and `.github/workflows/rust-benchmark.yml`, kept in sync). The toolchain stays pinned — a dated nightly, not a rolling one — so issue #14 cannot recur.

  - `ethnum` bumped `1.5.2` → `1.5.3` in `rust/Cargo.lock` and `rust/spacetime-module/Cargo.lock`. `ethnum 1.5.2` builds `core::num::TryFromIntError` with `unsafe { mem::transmute(()) }`, which now fails with `error[E0512]: cannot transmute between types of different sizes` because `TryFromIntError` is no longer zero-sized. `1.5.3` constructs the error via `u8::try_from(-1i8).unwrap_err()` instead.
  - Vendored `platform-data` (`rust/doublets-patched/dev-deps/data-rs`): added `impl Residual<()> for Flow` plus the `try_trait_v2_residual` feature gate. Nightly's `Try::Residual` gained a `Residual<Self::Output>` bound, so the unchanged `Flow` try-type failed with ``error[E0277]: the trait bound `Flow: Residual<()>` is not satisfied``. Documented in `rust/doublets-patched/PATCHES.md`.

### Changed

- **Dependencies updated to their latest compatible versions** (`rust/Cargo.lock`, `rust/spacetime-module/Cargo.lock`, `Cargo.lock`), clearing all five open Dependabot security alerts.

  - `serde_with 3.17.0` → `3.22.0` (the original Dependabot bump)
  - `anymap 0.12.1` → `anymap3 1.1.0` (critical advisory)
  - `protobuf 2.28.0` → `3.7.2`
  - `rand 0.9.2` → `0.9.5` and `rand 0.8.6` → `0.8.8`
  - `atty` removed entirely — it was pulled in transitively by the old criterion
  - `spacetimedb-sdk` / `spacetimedb` `2.0.3` → `2.10.1`
  - `tokio 1.48.0` → `1.53.1`, `syn 2.0.111` → `3.0.5`

- **criterion `=0.3.6` → `0.8`** (`rust/Cargo.toml`). The `cargo_bench_support` feature is now requested explicitly, because criterion no longer implies it under `default-features = false`.

- **`rust/src/module_bindings/`** regenerated with `spacetime generate` 2.10.1. The generated table module is now `links_table` (`LinksTableAccess` / `LinksTableHandle`) instead of `link_table`, and the binding types gained `Debug` derives, required by the 2.10.1 SDK which bounds `RemoteModule` on `Debug`. `rust/src/spacetimedb_impl.rs` follows the new import path.
