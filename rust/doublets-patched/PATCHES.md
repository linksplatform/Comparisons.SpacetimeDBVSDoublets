# Why Patched Versions Are Needed

This directory contains local patched copies of `doublets-rs` and its dependencies (`mem-rs`, `data-rs`, `trees-rs`). These patches are necessary because the published versions on crates.io use removed or renamed Rust nightly features that are incompatible with modern Rust nightly (≥ 1.80.0).

## Background

The `doublets` crate (and its internal dependencies `platform-mem`, `platform-data`, `platform-trees`) require Rust nightly due to heavy use of unstable features such as `allocator_api`, `fn_traits`, `try_trait_v2`, and others. The last published release on crates.io was pinned to a specific nightly toolchain from 2022–2023. Since then, several nightly features used by these crates have been stabilized, renamed, or removed from nightly entirely.

## Specific Issues Fixed

### `doublets` crate (`doublets-patched/doublets/`)

- **Removed `#![feature(generators)]`**: The upstream crate declared `#![feature(generators)]` in `src/lib.rs`. Rust nightly removed the `generators` feature in [this change](https://github.com/rust-lang/rust/pull/116958) (replaced by `coroutines`). The feature gate no longer exists and causes a compilation error. Since generators were not actually used in any code paths exercised by this benchmark, the feature flag was simply removed.

### `platform-data` crate (`doublets-patched/dev-deps/data-rs/`)

- **Removed `~const` syntax**: The upstream code used `~const Fn` bounds (the "maybe-const" trait bound syntax) in several places. This experimental syntax was removed from nightly in 2023 (see [rust-lang/rust#110395](https://github.com/rust-lang/rust/issues/110395)). The affected functions were changed to use regular `Fn` bounds instead, which is correct since they are never called in a `const` context in this project.

### `platform-mem` crate (`doublets-patched/dev-deps/mem-rs/`)

- **Removed obsolete `#![feature(...)]` flags**: The upstream crate declared several feature flags that have since been stabilized or renamed. Specifically, `nonnull_slice_from_raw_parts` and `slice_ptr_get` were stabilized in Rust 1.70.0 and 1.74.0 respectively. Keeping them as `#![feature(...)]` flags causes errors on modern nightly because nightly rejects `#![feature]` declarations for already-stable features.

## Why Not Use the Upstream Version?

The `doublets` crate is not actively maintained for compatibility with current Rust nightly. Attempts to use the published crates.io version (or the upstream git repository) result in compilation failures on Rust nightly ≥ 1.80.0. Upstream issues have been filed but not addressed.

The patches are minimal and surgical — they do not change any algorithmic behavior, data structures, or storage semantics. The only changes are the removal of feature flags and the replacement of removed syntax with equivalent stable alternatives.

## Future Resolution

Once the upstream `doublets-rs` crates are updated for modern Rust nightly compatibility, the `doublets-patched/` directory can be removed and the dependency replaced with a direct crates.io reference. This is tracked in the [doublets-rs repository](https://github.com/linksplatform/doublets-rs).
