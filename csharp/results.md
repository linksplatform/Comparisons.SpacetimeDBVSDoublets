_Generated 2026-10-07 00:04 UTC by [GitHub Actions run 37549640595](https://github.com/linksplatform/Comparisons.SpacetimeDBVSDoublets/actions/runs/37549640595) — 1000 benchmarked links, 3000 background links. SpacetimeDB.ClientSDK 2.10.1; Platform.Data.Doublets 0.18.1; SpacetimeDB module 2.10.1; SpacetimeDB server/CLI 2.10.1; CPU — spacetimedb: AMD EPYC 7763 64-Core Processor; doublets: AMD EPYC 7763 64-Core Processor._

| Operation       | Doublets United Volatile | Doublets United NonVolatile | Doublets Split Volatile | Doublets Split NonVolatile | SpacetimeDB   |
|-----------------|--------------------------|-----------------------------|-------------------------|----------------------------|---------------|
| Create          | 593 µs (4,304× faster)   | 595 µs (4,294× faster)      | 195 µs (13,070× faster) | 203 µs (12,565× faster)    | 2.55 s        |
| Update          | 788 µs (3,275× faster)   | 791 µs (3,264× faster)      | 129 µs (20,002× faster) | 128 µs (20,108× faster)    | 2.58 s        |
| Delete          | 325 µs (3,927× faster)   | 329 µs (3,881× faster)      | 180 µs (7,088× faster)  | 189 µs (6,754× faster)     | 1.28 s        |
| Query All       | 139 µs (≈ same)          | 173 µs (≈ same)             | 119 µs (≈ same)         | 119 µs (≈ same)            | 337 µs        |
| Query by Id     | 114 µs (108× faster)     | 114 µs (108× faster)        | 126 µs (97.7× faster)   | 126 µs (98.3× faster)      | 12.3 ms       |
| Query by Source | 28.7 µs (≈ same)         | 28.6 µs (≈ same)            | 23.3 µs (≈ same)        | 22.7 µs (≈ same)           | 342 µs        |
| Query by Target | 38.5 µs (10.5× faster)   | 38.6 µs (10.5× faster)      | 25.5 µs (15.8× faster)  | 25.7 µs (15.7× faster)     | 404 µs        |
