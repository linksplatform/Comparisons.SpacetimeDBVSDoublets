### Added
- C# SpacetimeDB SDK and four Platform.Data.Doublets benchmarks matching the Rust workloads and sizes.
- Differential backend tests in both languages, and C# sampling and iteration reset tests.
- Per-backend, per-language benchmark VMs with recorded CPU, server/module and SDK/library versions.

### Fixed
- Preserve Rust Doublets create endpoints and classify Split self-references before detaching index trees.
- Use unique query datasets with actual background IDs and consume query results to prevent optimization.
- Report differences within 5% or overlapping standard deviations as approximately the same.
- Format measured durations with three significant digits and human-readable units.
- Pin SpacetimeDB 2.10.1, fail CI on Clippy warnings, and cancel superseded workflow runs.

### Removed
- Unrelated template crate, tests, example and crate release workflow.
