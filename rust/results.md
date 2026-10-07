_Generated 2026-09-16 09:20 UTC by [GitHub Actions run 35075414684](https://github.com/linksplatform/Comparisons.SpacetimeDBVSDoublets/actions/runs/35075414684) — 1000 benchmarked links, 3000 background links. SpacetimeDB server/CLI unknown (not recorded); Rust SDK 2.10.1 (lockfile); Doublets (patched) 0.1.0-pre+beta.15; CPU — historical run: unknown (not recorded)._

| Operation       | Doublets United Volatile | Doublets United NonVolatile | Doublets Split Volatile | Doublets Split NonVolatile | SpacetimeDB   |
|-----------------|--------------------------|-----------------------------|-------------------------|----------------------------|---------------|
| Create          | 76.7 µs (33,385× faster) | 76.6 µs (33,438× faster)    | 47.1 µs (54,356× faster) | 47.4 µs (54,020× faster)   | 2.56 s        |
| Update          | 247 µs (10,703× faster)  | 247 µs (10,707× faster)     | 35.8 µs (73,838× faster) | 36.0 µs (73,408× faster)   | 2.64 s        |
| Delete          | 180 µs (7,169× faster)   | 181 µs (7,125× faster)      | 94.1 µs (13,732× faster) | 100 µs (12,906× faster)    | 1.29 s        |
| Query All       | 21.6 µs (≈ same)         | 21.6 µs (≈ same)            | 28.4 µs (1.28× slower)  | 28.3 µs (1.28× slower)     | 22.1 µs       |
| Query by Id     | 55.0 ns (198,863× faster) | 59.0 ns (185,381× faster)   | 1.02 µs (10,744× faster) | 1.02 µs (10,744× faster)   | 10.9 ms       |
| Query by Source | 1.48 µs (111× faster)    | 1.45 µs (113× faster)       | 480 ns (341× faster)    | 478 ns (342× faster)       | 164 µs        |
| Query by Target | 1.51 µs (109× faster)    | 1.48 µs (112× faster)       | 445 ns (372× faster)    | 393 ns (421× faster)       | 165 µs        |
