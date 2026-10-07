_Generated 2026-10-07 00:04 UTC by [GitHub Actions run 37549640595](https://github.com/linksplatform/Comparisons.SpacetimeDBVSDoublets/actions/runs/37549640595) — 1000 benchmarked links, 3000 background links. Rust SDK 2.10.1; Doublets (patched) 0.1.0-pre+beta.15; SpacetimeDB module 2.10.1; SpacetimeDB server/CLI 2.10.1; CPU — spacetimedb: AMD EPYC 7763 64-Core Processor; doublets: AMD EPYC 7763 64-Core Processor._

| Operation       | Doublets United Volatile | Doublets United NonVolatile | Doublets Split Volatile | Doublets Split NonVolatile | SpacetimeDB   |
|-----------------|--------------------------|-----------------------------|-------------------------|----------------------------|---------------|
| Create          | 76.9 µs (33,383× faster) | 76.5 µs (33,540× faster)    | 49.9 µs (51,405× faster) | 51.2 µs (50,116× faster)   | 2.57 s        |
| Update          | 244 µs (10,581× faster)  | 242 µs (10,697× faster)     | 40.4 µs (63,999× faster) | 41.5 µs (62,213× faster)   | 2.58 s        |
| Delete          | 179 µs (7,243× faster)   | 177 µs (7,286× faster)      | 99.9 µs (12,942× faster) | 134 µs (9,679× faster)     | 1.29 s        |
| Query All       | 21.6 µs (≈ same)         | 21.5 µs (≈ same)            | 28.3 µs (≈ same)        | 28.5 µs (≈ same)           | 28.9 µs       |
| Query by Id     | 2.44 µs (4,602× faster)  | 2.45 µs (4,582× faster)     | 2.67 µs (4,194× faster) | 3.07 µs (3,651× faster)    | 11.2 ms       |
| Query by Source | 11.8 µs (17.3× faster)   | 11.6 µs (17.6× faster)      | 11.7 µs (17.6× faster)  | 11.8 µs (17.4× faster)     | 205 µs        |
| Query by Target | 11.6 µs (16.3× faster)   | 11.8 µs (16.1× faster)      | 11.5 µs (16.5× faster)  | 11.5 µs (16.5× faster)     | 190 µs        |
