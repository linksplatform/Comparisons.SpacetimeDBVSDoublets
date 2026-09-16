_Generated 2026-09-15 22:53 UTC by [GitHub Actions run 35030102133](https://github.com/linksplatform/Comparisons.SpacetimeDBVSDoublets/actions/runs/35030102133) — 1000 benchmarked links, 3000 background links._

| Operation       | Doublets United Volatile | Doublets United NonVolatile | Doublets Split Volatile | Doublets Split NonVolatile | SpacetimeDB   |
|-----------------|--------------------------|-----------------------------|-------------------------|----------------------------|---------------|
| Create          | 73090 (34741.6x faster)  | 70181 (36181.6x faster)     | 48458 (52401.3x faster) | 48606 (52241.7x faster)    | 2539260579    |
| Update          | 214541 (12035.6x faster) | 214497 (12038.1x faster)    | 38161 (67664.3x faster) | 37990 (67968.9x faster)    | 2582139227    |
| Delete          | 159655 (7996.4x faster)  | 159342 (8012.1x faster)     | 96915 (13173.0x faster) | 96309 (13255.9x faster)    | 1276662577    |
| Query All       | 21170 (1.1x faster)      | 21611 (1.0x faster)         | 25503 (1.1x slower)     | 25649 (1.1x slower)        | 22522         |
| Query by Id     | 53 (211157.5x faster)    | 53 (211157.5x faster)       | 1019 (10982.7x faster)  | 1019 (10982.7x faster)     | 11191350      |
| Query by Source | 1463 (139.2x faster)     | 1462 (139.3x faster)        | 461 (441.8x faster)     | 464 (438.9x faster)        | 203651        |
| Query by Target | 1562 (116.7x faster)     | 1539 (118.4x faster)        | 392 (464.9x faster)     | 394 (462.6x faster)        | 182251        |
