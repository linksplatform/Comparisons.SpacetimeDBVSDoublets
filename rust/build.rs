// Build script: inject SpacetimeDB core version as a compile-time env variable.
// This version is embedded in the binary and printed at benchmark startup.

fn main() {
    // spacetimedb-core version at tag v2.0.1
    // The workspace version is the single source of truth in SpacetimeDB's Cargo.toml.
    // We set it here as a build-time constant for runtime verification.
    println!("cargo:rustc-env=SPACETIMEDB_CORE_VERSION=2.0.1");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
}
