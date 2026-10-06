#!/usr/bin/env bash
set -euo pipefail
mkdir -p ci-logs
nohup spacetime start > ci-logs/spacetimedb-server.log 2>&1 < /dev/null &
server_pid=$!
echo "$server_pid" > ci-logs/spacetimedb-server.pid
for attempt in $(seq 1 60); do
  if curl -sf http://localhost:3000/v1/ping >/dev/null; then
    spacetime publish --server http://localhost:3000 \
      --bin-path rust/spacetime-module/target/wasm32-unknown-unknown/release/spacetime_module.wasm \
      --yes benchmark-links
    exit 0
  fi
  if ! kill -0 "$server_pid" 2>/dev/null; then cat ci-logs/spacetimedb-server.log; exit 1; fi
  sleep 1
done
cat ci-logs/spacetimedb-server.log
exit 1
