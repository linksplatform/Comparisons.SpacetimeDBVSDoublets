//! Shared dataset for the source/target query benchmarks and differential tests.

/// Return a unique pair referencing existing background links.
///
/// The first ten background links are query keys. Subsequent links provide the
/// other endpoint, so pairs never duplicate the background points or each other.
/// Use actual IDs because SpacetimeDB's sequence survives `delete_all`.
#[must_use]
pub fn distributed_link(index: usize, background: &[u64], by_source: bool) -> (u64, u64) {
    let key = background[index % 10];
    let other = background[index / 10 + 10];
    if by_source {
        (key, other)
    } else {
        (other, key)
    }
}
