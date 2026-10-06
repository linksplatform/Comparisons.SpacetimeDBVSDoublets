#!/usr/bin/env bash
# Install the exact release; no mutable install.spacetimedb.com bootstrap.
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd)
version=$(cat "$root/scripts/spacetimedb-version")
case "$(uname -s)/$(uname -m)" in
  Linux/x86_64) target=x86_64-unknown-linux-gnu ;;
  Darwin/arm64) target=aarch64-apple-darwin ;;
  Darwin/x86_64) target=x86_64-apple-darwin ;;
  *) echo "Unsupported platform" >&2; exit 1 ;;
esac
install_dir=${SPACETIMEDB_INSTALL_DIR:-$HOME/.local/bin}
mkdir -p "$install_dir"
archive=$(mktemp)
trap 'rm -f "$archive"' EXIT
curl --fail --location --silent --show-error \
  "https://github.com/clockworklabs/SpacetimeDB/releases/download/v${version}/spacetime-${target}.tar.gz" -o "$archive"
tar -xzf "$archive" -C "$install_dir"
ln -sf "$install_dir/spacetimedb-cli" "$install_dir/spacetime"
"$install_dir/spacetime" --version
if [[ -n "${GITHUB_PATH:-}" ]]; then echo "$install_dir" >> "$GITHUB_PATH"; fi
