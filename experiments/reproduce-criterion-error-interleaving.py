#!/usr/bin/env python3
"""Reproduces the `No benchmark data found in out.txt` CI failure (issue #14).

Criterion prints its own error messages to *stdout* (see
criterion-0.3.6/src/macros_private.rs:36 `println!("Criterion.rs ERROR: {}", ...)`)
between `print!("test {} ... ")` and `println!("bench: ...")`
(criterion-0.3.6/src/report.rs:740 and :756).

When a stale/partial `target/criterion/<id>/<size>/base` directory is present —
which happens in CI because the Rust cache restores `target/` — loading
`base/sample.json` fails and the error text is spliced into the middle of the
bencher line, so the measurement ends up split across two lines:

    test query_by_id/Doublets_Split_NonVolatile/10 ... Criterion.rs ERROR: error: Failed to access file "...": No such file or directory (os error 2)
    bench:          37 ns/iter (+/- 0)

This script feeds exactly that output (copied from CI run 35028280108) into
out.py's parser and reports how many measurements survive.
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "rust"))

import out  # noqa: E402

POLLUTED = """\
test create/SpacetimeDB/10 ... Criterion.rs ERROR: error: Failed to access file "/home/runner/work/rust/target/criterion/create_SpacetimeDB/10/base/sample.json": No such file or directory (os error 2)
bench:    24992765 ns/iter (+/- 1234567)

test create/Doublets_United_Volatile/10 ... Criterion.rs ERROR: error: Failed to access file "/home/runner/work/rust/target/criterion/create_Doublets_United_Volatile/10/base/sample.json": No such file or directory (os error 2)
bench:         464 ns/iter (+/- 12)
"""

CLEAN = """\
test create/SpacetimeDB/10 ... bench:    24992765 ns/iter (+/- 1234567)

test create/Doublets_United_Volatile/10 ... bench:         464 ns/iter (+/- 12)
"""


def main() -> int:
    clean = out.parse_text(CLEAN)
    polluted = out.parse_text(POLLUTED)
    print(f"clean output    -> {sum(len(v) for v in clean.values())} measurements: {clean}")
    print(f"polluted output -> {sum(len(v) for v in polluted.values())} measurements: {polluted}")
    if clean == polluted:
        print("OK: the parser tolerates interleaved Criterion error messages")
        return 0
    print("REPRODUCED: interleaved Criterion errors make the parser lose every measurement")
    return 1


if __name__ == "__main__":
    sys.exit(main())
