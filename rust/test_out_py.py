#!/usr/bin/env python3
"""Regression tests for the benchmark result parser in out.py.

Run with:
    python3 -m unittest discover -s rust -p 'test_*.py'

These guard the bencher-format parsing, which silently produced empty charts
when the pattern expected one more slash-separated component than Criterion
actually emits.
"""

import os
import sys
import tempfile
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import out as out_py  # noqa: E402

# Verbatim lines from `cargo bench --bench bench -- --output-format bencher`.
# Criterion renders BenchmarkId::new("<operation>/<variant>", <size>) as exactly
# three slash-separated components: <operation>/<variant>/<size>.
SAMPLE_OUTPUT = """\
test create/SpacetimeDB/5 ... bench: 162,933,984 ns/iter (+/- 60,235,870)
test create/Doublets_United_Volatile/5 ... bench:         465 ns/iter (+/- 49)
test create/Doublets_Split_Volatile/5 ... bench:         552 ns/iter (+/- 124)
test create/Doublets_United_NonVolatile/5 ... bench:         462 ns/iter (+/- 110)
test create/Doublets_Split_NonVolatile/5 ... bench:         573 ns/iter (+/- 118)
test query_by_source/SpacetimeDB/5 ... bench:         962 ns/iter (+/- 80)
test query_by_source/Doublets_United_Volatile/5 ... bench:         439 ns/iter (+/- 21)
"""


class ParseResultsTest(unittest.TestCase):
    def setUp(self):
        handle, self.path = tempfile.mkstemp(suffix=".txt")
        with os.fdopen(handle, "w") as f:
            f.write(SAMPLE_OUTPUT)
        self.addCleanup(os.unlink, self.path)

    def test_parses_three_component_benchmark_ids(self):
        results = out_py.parse_results(self.path)
        self.assertEqual(results["create"]["SpacetimeDB"], 162_933_984)
        self.assertEqual(results["create"]["Doublets_United_Volatile"], 465)
        self.assertEqual(results["create"]["Doublets_Split_Volatile"], 552)

    def test_parses_operations_with_underscores(self):
        results = out_py.parse_results(self.path)
        self.assertEqual(results["query_by_source"]["SpacetimeDB"], 962)
        self.assertEqual(results["query_by_source"]["Doublets_United_Volatile"], 439)

    def test_parses_non_volatile_variants(self):
        results = out_py.parse_results(self.path)
        self.assertEqual(results["create"]["Doublets_United_NonVolatile"], 462)
        self.assertEqual(results["create"]["Doublets_Split_NonVolatile"], 573)

    def test_every_measured_variant_is_chartable(self):
        """Parsed variants must all have a label and a colour, or they vanish
        from the generated charts."""
        results = out_py.parse_results(self.path)
        measured = {v for per_op in results.values() for v in per_op}
        self.assertTrue(measured, "sample output produced no results")
        self.assertLessEqual(measured, set(out_py.VARIANTS))
        self.assertLessEqual(measured, set(out_py.COLORS))

    def test_missing_file_yields_empty_results(self):
        results = out_py.parse_results(os.path.join(tempfile.gettempdir(), "no-such-out.txt"))
        self.assertEqual(results, {op: {} for op in out_py.OPERATIONS})


if __name__ == "__main__":
    unittest.main()
