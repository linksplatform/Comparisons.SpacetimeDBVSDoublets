# Fix benchmark CI timing: add PR quick mode and full mode with timeout

## Problem

The `Benchmark` job in `rust-benchmark.yml` exceeded GitHub Actions' 6-hour limit
when pushed to `main`. Root cause: Criterion's default settings (100 samples, 5s
measurement) combined with SpacetimeDB's synchronous round-trip per operation
(~8000 round trips × ~1ms each per iteration) caused each SpacetimeDB benchmark
to run for ~13 minutes, totalling ~2+ hours for all 7 SpacetimeDB benchmarks —
and the cleanup `delete_all` overhead pushed it past 6 hours.

Additionally, there was no benchmark validation for pull requests at all.

## Solution

- **PR quick mode** (`benchmark-pr` job): runs on `pull_request` events with reduced
  scale (`BENCHMARK_LINK_COUNT=10`, `BACKGROUND_LINK_COUNT=30`) and tighter Criterion
  settings (`--sample-size 10 --warm-up-time 1 --measurement-time 2`). Expected
  runtime: 3–5 minutes total for all 35 benchmarks. Results uploaded as artifacts
  but not committed to the repository.

- **Full mode** (`benchmark` job): runs on `push` to `main`/`master` with full scale
  (`BENCHMARK_LINK_COUNT=1000`, `BACKGROUND_LINK_COUNT=3000`) and reduced sample count
  (`--sample-size 20 --nresamples 10000`) to finish in ~30–45 minutes (well under
  3 hours) while still producing statistically meaningful results.

- **Safety timeout**: `timeout-minutes: 180` added to the `benchmark` job and
  `timeout-minutes: 30` to `test` jobs (was `360` = 6 hours).

- **Case study**: Deep analysis of the root cause documented in
  `docs/case-studies/issue-6/README.md`.

Fixes #6.
