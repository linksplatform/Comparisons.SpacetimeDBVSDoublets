_Generated 2026-09-16 09:20 UTC by [GitHub Actions run 35075414684](https://github.com/linksplatform/Comparisons.SpacetimeDBVSDoublets/actions/runs/35075414684) — 1000 benchmarked links, 3000 background links._

| Operation       | Doublets United Volatile | Doublets United NonVolatile | Doublets Split Volatile | Doublets Split NonVolatile | SpacetimeDB   |
|-----------------|--------------------------|-----------------------------|-------------------------|----------------------------|---------------|
| Create          | 76689 (33385.0x faster)  | 76567 (33438.2x faster)     | 47102 (54355.7x faster) | 47395 (54019.7x faster)    | 2560262853    |
| Update          | 247101 (10703.3x faster) | 247021 (10706.8x faster)    | 35819 (73838.1x faster) | 36029 (73407.7x faster)    | 2644805261    |
| Delete          | 180271 (7169.4x faster)  | 181398 (7124.8x faster)     | 94116 (13732.3x faster) | 100143 (12905.9x faster)   | 1292432593    |
| Query All       | 21580 (1.0x faster)      | 21592 (1.0x faster)         | 28388 (1.3x slower)     | 28312 (1.3x slower)        | 22126         |
| Query by Id     | 55 (198863.3x faster)    | 59 (185381.0x faster)       | 1018 (10744.1x faster)  | 1018 (10744.1x faster)     | 10937480      |
| Query by Source | 1476 (110.8x faster)     | 1453 (112.5x faster)        | 480 (340.7x faster)     | 478 (342.1x faster)        | 163528        |
| Query by Target | 1514 (109.3x faster)     | 1478 (111.9x faster)        | 445 (371.8x faster)     | 393 (420.9x faster)        | 165432        |
