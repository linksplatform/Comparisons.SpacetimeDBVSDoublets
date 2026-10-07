# Contributing

Work in a pull request branch. The repository contains benchmark applications in
`rust/` and `csharp/`; run Cargo commands from `rust/` to select the pinned nightly.
Install .NET 10 and Python 3.11 or later, then install `matplotlib` and `numpy`.

Before committing:

```bash
(cd rust && cargo fmt --all -- --check)
(cd rust && cargo clippy --locked --all-targets -- -D warnings)
(cd rust && python3 -m unittest test_out -v)
node scripts/check-file-size.mjs
dotnet restore csharp/SpacetimeDBVSDoublets.slnx --locked-mode
dotnet format csharp/SpacetimeDBVSDoublets.slnx --no-restore --verify-no-changes
dotnet build csharp/SpacetimeDBVSDoublets.slnx --no-restore -c Release --warnaserror
```

For integration checks, install the pinned CLI using `scripts/install-spacetimedb.sh`,
add its directory to `PATH`, then build and publish the shared Rust module:

```bash
(cd rust/spacetime-module && cargo build --locked --release --target wasm32-unknown-unknown)
scripts/start-spacetimedb.sh
(cd rust && cargo test --locked -- --test-threads=1 --include-ignored)
SPACETIMEDB_URI=http://localhost:3000 \
  dotnet run --project csharp/SpacetimeDBVSDoublets.Tests -c Release -- -parallelMode none
```

Run tests sequentially because they reset the shared database. Without a server,
run ordinary `cargo test` and the C# tests without `SPACETIMEDB_URI`; those cover
Doublets and sampling. CI always includes live server tests.

Keep operations, setup, reset and dataset sizes equivalent across languages. Add
regression tests before bug fixes and exercise all four Doublets variants. Generated
C# bindings come from SpacetimeDB 2.10.1 and the module WASM; regenerate them with the
command in the README. Keep code formatting consistent and public APIs documented.

Add a fragment to `changelog.d/` for user-facing changes. There is no package release
pipeline: this repository publishes benchmark reports and charts through CI.

PR validation uses 10 measured links and 30 background links. Full default-branch
runs use 1000 and 3000. Each language/backend is measured on a separate VM. Preserve
raw output and metadata alongside any reported results, and describe validation in
the pull request.
