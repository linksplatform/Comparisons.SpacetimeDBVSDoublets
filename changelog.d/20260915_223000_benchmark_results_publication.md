---
bump: patch
---

### Fixed
- Pinned the Rust nightly toolchain (`nightly-2026-04-14`), unbreaking the Rust Benchmark workflow that had been failing on every branch since April with `E0512` in `ethnum` and `E0277` in the patched `doublets` dev dependency
- Added `set -o pipefail` around `cargo bench ... | tee out.txt`, so a failing benchmark is no longer masked by `tee`

### Added
- Benchmark results are now published automatically: `rust/out.py` writes `rust/results.md`, copies both charts into `docs/benchmarks/` and replaces the results section of `README.md`, which CI commits back to `main`
- `rust/test_out.py` — unit tests for the results reporting pipeline, run by a dedicated `results-pipeline` CI job that gates the benchmark jobs
- Benchmark results are written to the GitHub Actions job summary for both the pull request and the full run

### Changed
- `rust/out.py` now reports all five benchmarked backends (the two NonVolatile Doublets variants were previously missing) and annotates every Doublets result relative to the SpacetimeDB baseline
- `README.md` documents the results in the same style as the sibling Neo4j and PostgreSQL comparisons

### Removed
- `rust/rust_out` — a 4.2 MB compiled binary committed by accident
