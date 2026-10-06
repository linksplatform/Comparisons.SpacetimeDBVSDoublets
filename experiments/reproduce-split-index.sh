#!/usr/bin/env bash
# Bounded reproduction: at most 130 live links, 8 GiB address space limit (including the linker).
set -euo pipefail
ulimit -v 8388608
root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root/rust"
BENCHMARK_TRACE=1 RUST_BACKTRACE=1 cargo test --locked --test same_behavior \
  all_doublets_variants_return_same_results -- --test-threads=1 --nocapture
