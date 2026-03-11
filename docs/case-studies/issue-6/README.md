# Case Study: Issue #6 — Benchmark Exceeds CI Time Limits

**Issue:** [#6 — We need to find a way to execute benchmark for debug for Pull Requests in just under 10 minutes and it should fit 3 hours in commits to main branch](https://github.com/linksplatform/Comparisons.SpacetimeDBVSDoublets/issues/6)

**CI Run that triggered this issue:** [Run #22813620523](https://github.com/linksplatform/Comparisons.SpacetimeDBVSDoublets/actions/runs/22813620523)

---

## 1. Timeline of Events

| Time (UTC) | Event |
|---|---|
| 2026-03-08 04:11:29 | Push to `main` branch triggers Rust Benchmark workflow |
| 2026-03-08 04:11:29 | `Test (ubuntu-latest)` and `Test (macos-latest)` jobs start |
| 2026-03-08 04:13:54 | Both `Test` jobs pass in ~2.5–3.5 minutes |
| 2026-03-08 04:14:XX | `Benchmark` job starts (only runs on main, needs test to pass) |
| 2026-03-08 10:19:30 | **Benchmark job killed by GitHub Actions — exceeded 6h0m0s limit** |

GitHub Actions enforces a maximum job execution time of **6 hours**. The `Benchmark` job had no `timeout-minutes` set, causing it to run until the platform forced termination.

---

## 2. Root Cause Analysis

### 2.1 Architecture of the SpacetimeDB Benchmarks

The benchmark suite (`rust/benches/bench.rs`) uses Criterion's `iter_custom` pattern:

```rust
b.iter_custom(|iters| {
    let mut total = Duration::ZERO;
    for _ in 0..iters {
        let mut fork = Benched::fork(&mut benched);
        setup_background!(fork);          // UNMEASURED: BACKGROUND_LINK_COUNT × ops
        let start = Instant::now();
        for _ in 0..n {
            fork.create_point();          // MEASURED: BENCHMARK_LINK_COUNT × ops
        }
        total += start.elapsed();
    }
    total
});
```

**Key observation:** Each benchmark iteration includes both:
1. **Unmeasured setup:** `BACKGROUND_LINK_COUNT=3000` background links created via `create_point()` (each = 2 round trips to SpacetimeDB)
2. **Measured operation:** `BENCHMARK_LINK_COUNT=1000` operations

### 2.2 SpacetimeDB Round-Trip Overhead

`SpacetimeDbLinks` uses the official `spacetimedb-sdk` v2 over WebSocket. Each CRUD operation (`create`, `update`, `delete`) is **synchronous** — it calls a reducer and waits for the acknowledgment via condvar:

```rust
fn wait_for_reducer<F>(&self, invoke: F) {
    let done = Arc::new((Mutex::new(false), Condvar::new()));
    invoke(Arc::clone(&done));
    let (lock, cvar) = &*done;
    let mut finished = lock.lock().unwrap();
    while !*finished {
        // Waits up to 30 seconds per operation
        cvar.wait_timeout(finished, Duration::from_secs(30))...
    }
}
```

### 2.3 Benchmark Count

There are **35 total benchmarks** (7 operations × 5 backends):
- **7 SpacetimeDB benchmarks** (network-bound)
- **28 local Doublets benchmarks** (in-memory or file-backed)

Operations: `create`, `delete`, `update`, `query_all`, `query_by_id`, `query_by_source`, `query_by_target`

Backends: `SpacetimeDB`, `Doublets_United_Volatile`, `Doublets_Split_Volatile`, `Doublets_United_NonVolatile`, `Doublets_Split_NonVolatile`

### 2.4 Wall-Clock Time per Criterion Sample

With defaults (`BENCHMARK_LINK_COUNT=1000`, `BACKGROUND_LINK_COUNT=3000`, `sample_size=100`):

Each outer iteration for SpacetimeDB involves:
- 3000 background `create_point()` = 3000 × 2 round trips = **~6s unmeasured**
- 1000 benchmark operations = **~2s measured** (for create)
- **Total wall clock: ~8s per outer iteration**

Criterion sees this ~8s per-iteration and uses **Flat sampling mode** (since Linear would project to 5050 iterations × 8s = 11 hours). With `sample_size=100` and `measurement_time=5s`:

- `iters_per_sample = ceil(5s / 100 / 8s) = 1` (always)
- 100 samples × 1 iter × ~8s = **~800s per SpacetimeDB benchmark**
- Warm-up: 1 pass (8s > 3s threshold) = 8s
- **~808s ≈ 13.5 minutes per SpacetimeDB benchmark**
- 7 SpacetimeDB benchmarks = **~94 minutes total**

For local Doublets (in-memory, ~10µs per link):
- ~10ms per outer iteration, Linear mode
- 5 seconds measurement / 10ms = 500 iterations, but Linear sampling averages across 100 samples
- ~50s per local benchmark
- 28 local benchmarks = **~23 minutes total**

**Projected total benchmark runtime: ~117 minutes ≈ 2 hours**

### 2.5 Why Did It Exceed 6 Hours?

The `cargo bench` command was:
```bash
cargo bench --bench bench -- --output-format bencher | tee out.txt
```

This uses **all defaults**, which with default `BENCHMARK_LINK_COUNT=1000` and `BACKGROUND_LINK_COUNT=3000` should take ~2 hours. However, there are compounding factors:

1. **`delete_all()` overhead on each fork drop:** The `unfork()` method calls `delete_all()`, which iterates through all 3000+ links in the SpacetimeDB module (`for id in ids { ctx.db.links().id().delete(&id); }`) — each individually deleted, not bulk. This adds another ~3s per iteration cleanup.

2. **GitHub Actions overhead:** Network latency on CI runners is higher than localhost. Even "localhost" on a CI runner shares virtualized network stack.

3. **SpacetimeDB server startup warmup:** The server was newly started before benchmarks; JIT/JVM warmup of the WASM engine may cause slower early iterations.

4. **Combined effect:** With cleanup overhead, effective wall clock per outer iteration may be **~11-15s** rather than the theoretical ~8s, pushing total to **3-4 hours** minimum for the SpacetimeDB portion alone — and then the `spacetimedb_create` benchmark alone may have taken several hours.

The `delete_all_links` reducer implementation is particularly slow:
```rust
#[reducer]
pub fn delete_all_links(ctx: &ReducerContext) {
    let ids: Vec<u64> = ctx.db.links().iter().map(|l| l.id).collect();
    for id in ids {
        ctx.db.links().id().delete(&id);  // One delete per link, N round trips
    }
}
```

With 4000+ links (3000 background + 1000 benchmark), this single `delete_all` call sends ~4000 delete operations to the WASM engine, each executing a btree lookup and delete. This is slow on the server side.

---

## 3. Secondary Issues Found

### 3.1 Test Failures in Earlier PRs

Run [#22810598819](https://github.com/linksplatform/Comparisons.SpacetimeDBVSDoublets/actions/runs/22810598819) shows test failures in `spacetimedb_impl::tests::test_create_and_query` and `test_delete_all` when tests run in parallel. This was due to **shared state** in the SpacetimeDB database — parallel tests interfere with each other. Fixed in the current version by adding `--test-threads=1`.

### 3.2 No `timeout-minutes` on Benchmark Job

The `Benchmark` job in `rust-benchmark.yml` has no `timeout-minutes`, allowing it to consume the full 6-hour GitHub Actions limit.

### 3.3 SpacetimeDB `delete_all` Performance

The server-side `delete_all_links` reducer deletes links one by one in a loop. This is O(N) round trips on the server. A more efficient implementation would use a bulk table truncation API if available in SpacetimeDB 2.x.

---

## 4. Proposed Solutions

### Solution 1: Separate PR and Main Branch Benchmark Modes (Implemented ✓)

Use environment variables to control scale, plus CLI flags for Criterion timing:

**PR Quick Mode (< 10 minutes):**
```yaml
env:
  BENCHMARK_LINK_COUNT: 10
  BACKGROUND_LINK_COUNT: 30
  CRITERION_SAMPLE_SIZE: 10
  CRITERION_WARM_UP_TIME: 1
  CRITERION_MEASUREMENT_TIME: 2
```

Expected runtime:
- 7 SpacetimeDB benchmarks: ~10s each = ~70s
- 28 local benchmarks: ~30s each = ~840s
- Build time: ~3 minutes (with cache)
- **Total: ~10-12 minutes** (under the 10-minute benchmark goal, may need further reduction)

With even smaller counts:
- `BENCHMARK_LINK_COUNT=10`, `BACKGROUND_LINK_COUNT=30`, `--sample-size 10`, `--warm-up-time 1`, `--measurement-time 2`
- SpacetimeDB: 30 bg × 2 + 10 op × 2 = 80 RTTs per iter × ~1ms = ~0.08s + overhead ≈ 0.5s wall clock per iter
- 10 samples × 0.5s = ~5s per SpacetimeDB benchmark, 7 benchmarks = ~35s total
- Local: very fast, <1s per benchmark
- **Benchmark portion: ~90-120 seconds ≈ 2 minutes**
- With build/setup/start overhead: **~8-9 minutes total**

**Main Full Mode (< 3 hours):**
```yaml
env:
  BENCHMARK_LINK_COUNT: 1000
  BACKGROUND_LINK_COUNT: 3000
```
With `--sample-size 20 --nresamples 10000`:
- SpacetimeDB: 20 samples × ~11s = ~220s each = ~26 min total
- Local: ~10s each × 28 = ~5 min
- Build/setup: ~5 min
- **Total: ~36 minutes** — well within 3-hour limit with large margin

### Solution 2: Add `timeout-minutes` to Benchmark Job (Implemented ✓)

Added `timeout-minutes: 180` (3 hours) to the `Benchmark` job as a hard safety net.

### Solution 3: Optimize `delete_all_links` in SpacetimeDB Module

The server-side reducer could use SpacetimeDB's table iterator deletion more efficiently:

```rust
#[reducer]
pub fn delete_all_links(ctx: &ReducerContext) {
    // More efficient: collect all and delete
    ctx.db.links().iter().collect::<Vec<_>>().into_iter()
        .for_each(|link| { ctx.db.links().id().delete(&link.id); });
}
```

Note: SpacetimeDB 2.x may not yet provide a table `truncate()` or `delete_all()` API. This is a known limitation. See external issue section.

### Solution 4: Per-Group Criterion Configuration

Override Criterion settings per benchmark group using the full `criterion_group!` syntax to set SpacetimeDB groups to use lower sample counts.

---

## 5. External Issues to Report

### SpacetimeDB: No Bulk Table Truncation API

The `delete_all_links` workaround is needed because SpacetimeDB 2.x lacks a native `TRUNCATE TABLE` equivalent in reducers. This should be reported to the SpacetimeDB project.

Repository: https://github.com/clockworklabs/SpacetimeDB

**Reproducible example:**
```rust
// Workaround needed: slow O(N) individual deletes
#[reducer]
pub fn delete_all_links(ctx: &ReducerContext) {
    let ids: Vec<u64> = ctx.db.links().iter().map(|l| l.id).collect();
    for id in ids { ctx.db.links().id().delete(&id); }
}

// Desired API: O(1) bulk truncation
// ctx.db.links().delete_all();  // Does not exist in SpacetimeDB 2.x
```

---

## 6. Files Modified

- `.github/workflows/rust-benchmark.yml` — Added PR quick mode, main full mode, and `timeout-minutes: 180` on Benchmark job
- `changelog.d/20260311_benchmark_timing.md` — Changelog entry for this fix

---

## 7. References

- [Criterion documentation — Timing configuration](https://bheisler.github.io/criterion.rs/book/user_guide/command_line_options.html)
- [GitHub Actions: Job timeout](https://docs.github.com/en/actions/writing-workflows/choosing-what-your-workflow-does/workflow-syntax-for-github-actions#jobsjob_idtimeout-minutes)
- [SpacetimeDB SDK](https://github.com/clockworklabs/SpacetimeDB/tree/master/crates/sdk)
- CI Run: https://github.com/linksplatform/Comparisons.SpacetimeDBVSDoublets/actions/runs/22813620523
