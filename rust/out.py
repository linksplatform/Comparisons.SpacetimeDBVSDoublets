#!/usr/bin/env python3
"""Benchmark result visualization and documentation for SpacetimeDB vs Doublets.

Reads Criterion bencher-format output (produced by
`cargo bench --bench bench -- --output-format bencher`) and generates:

- ``bench_rust.png``          — linear ("pixel") scale comparison chart
- ``bench_rust_log_scale.png``— logarithmic scale comparison chart
- ``results.md``              — Markdown results table with speedup ratios
- optionally updates the results section of ``README.md`` in place

The chart and table style follows the sibling benchmarks
(Comparisons.Neo4jVSDoublets, Comparisons.PostgreSQLVSDoublets) so that all
LinksPlatform comparisons are documented the same way.

Usage:
    python3 out.py [out.txt] [--readme ../README.md] [--docs-dir ../docs/benchmarks]
"""

import argparse
import os
import re
import shutil
import sys
from datetime import datetime, timezone

try:
    import matplotlib

    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    import numpy as np

    HAS_MATPLOTLIB = True
except ImportError:  # pragma: no cover - exercised only without matplotlib
    print("Warning: matplotlib/numpy not installed, skipping chart generation")
    HAS_MATPLOTLIB = False

# Bencher output format emitted by criterion:
# test <operation>/<variant>/<size> ... bench: <ns_per_iter> ns/iter (+/- <variance>)
#
# The two halves are printed by separate statements (`print!("test {} ... ")` and
# `println!("bench: ...")`, criterion-0.3.6/src/report.rs), and criterion logs its
# own errors to stdout in between (`println!("Criterion.rs ERROR: ...")`,
# criterion-0.3.6/src/macros_private.rs). A stale `target/criterion/<id>/<size>/base`
# directory is enough to splice such an error into the middle of the record and push
# `bench:` onto the next line, so the pattern skips anything that is not the start of
# the next `test ` record.
BENCHER_PATTERN = re.compile(
    r"test\s+(\w+)/(\w+)/(\d+)\s+\.\.\.\s*"
    r"(?:(?!\btest\s)[\s\S])*?"
    r"bench:\s+([\d,]+)\s+ns/iter"
)

# Operations in the order they are reported, mapped to human readable labels.
OPERATIONS = [
    ("create", "Create"),
    ("update", "Update"),
    ("delete", "Delete"),
    ("query_all", "Query All"),
    ("query_by_id", "Query by Id"),
    ("query_by_source", "Query by Source"),
    ("query_by_target", "Query by Target"),
]

# Benchmarked backends. The first entry is the baseline every other variant is
# compared against in the results table.
BASELINE = "SpacetimeDB"

VARIANTS = [
    ("Doublets_United_Volatile", "Doublets United Volatile", "salmon"),
    ("Doublets_United_NonVolatile", "Doublets United NonVolatile", "red"),
    ("Doublets_Split_Volatile", "Doublets Split Volatile", "lightgreen"),
    ("Doublets_Split_NonVolatile", "Doublets Split NonVolatile", "green"),
    (BASELINE, "SpacetimeDB", "royalblue"),
]

README_START_MARKER = "<!--BENCHMARK_RESULTS_START-->"
README_END_MARKER = "<!--BENCHMARK_RESULTS_END-->"


def parse_text(content):
    """Parse bencher-format text into ``{operation: {variant: ns_per_iter}}``."""
    results = {op: {} for op, _ in OPERATIONS}

    for match in BENCHER_PATTERN.finditer(content):
        operation, variant, _size, ns_str = match.groups()
        if operation in results:
            results[operation][variant] = int(ns_str.replace(",", ""))

    return results


def parse_results(filename="out.txt"):
    """Parse a bencher-format output file."""
    if not os.path.exists(filename):
        print(f"Warning: {filename} not found")
        return {op: {} for op, _ in OPERATIONS}

    with open(filename, "r", encoding="utf-8") as handle:
        return parse_text(handle.read())


def report_input_excerpt(filename, lines=20):
    """Describe the tail of ``filename`` so an unparsable run can be diagnosed."""
    if not os.path.exists(filename):
        return f"{filename} does not exist"

    with open(filename, "r", encoding="utf-8") as handle:
        content = handle.read()

    if not content.strip():
        return f"{filename} is empty"

    tail = content.splitlines()[-lines:]
    return "\n".join([f"Last {len(tail)} line(s) of {filename}:", *tail])


def has_any_results(results):
    """Return ``True`` when at least one measurement was parsed."""
    return any(measurements for measurements in results.values())


def format_speedup(value, baseline):
    """Annotate ``value`` with how it compares to the ``baseline`` measurement."""
    if not value:
        return "N/A"
    if not baseline:
        return f"{value}"
    if value <= baseline:
        return f"{value} ({baseline / value:.1f}x faster)"
    return f"{value} ({value / baseline:.1f}x slower)"


def format_results_table(results):
    """Render the Markdown results table (all numbers in nanoseconds)."""
    labels = [label for _, label, _ in VARIANTS]
    widths = [max(len(label), 13) for label in labels]
    operation_width = max(len(label) for _, label in OPERATIONS)

    header = "| " + "Operation".ljust(operation_width) + " | "
    header += " | ".join(label.ljust(width) for label, width in zip(labels, widths))
    header += " |"
    separator = "|" + "-" * (operation_width + 2)
    separator += "".join("|" + "-" * (width + 2) for width in widths) + "|"

    lines = [header, separator]
    for op, op_label in OPERATIONS:
        baseline = results[op].get(BASELINE, 0)
        cells = []
        for key, _label, _color in VARIANTS:
            value = results[op].get(key, 0)
            if key == BASELINE:
                cells.append(str(value) if value else "N/A")
            else:
                cells.append(format_speedup(value, baseline))
        row = "| " + op_label.ljust(operation_width) + " | "
        row += " | ".join(cell.ljust(width) for cell, width in zip(cells, widths))
        row += " |"
        lines.append(row)

    return "\n".join(lines)


def build_provenance(benchmark_links=None, background_links=None, generated_at=None):
    """Describe how and when the committed results were produced."""
    benchmark_links = benchmark_links or os.environ.get("BENCHMARK_LINK_COUNT", "1000")
    background_links = background_links or os.environ.get(
        "BACKGROUND_LINK_COUNT", "3000"
    )
    generated_at = generated_at or datetime.now(timezone.utc).strftime(
        "%Y-%m-%d %H:%M UTC"
    )

    source = "a local benchmark run"
    repository = os.environ.get("GITHUB_REPOSITORY")
    run_id = os.environ.get("GITHUB_RUN_ID")
    if repository and run_id:
        source = (
            f"[GitHub Actions run {run_id}]"
            f"(https://github.com/{repository}/actions/runs/{run_id})"
        )

    return (
        f"_Generated {generated_at} by {source} — "
        f"{benchmark_links} benchmarked links, "
        f"{background_links} background links._"
    )


def render_results_section(results, provenance=None):
    """Render the README section: provenance line plus the results table."""
    provenance = provenance if provenance is not None else build_provenance()
    return f"{provenance}\n\n{format_results_table(results)}"


def update_readme(readme_path, section):
    """Replace the marked results section of the README.

    Returns ``True`` when the file was modified.
    """
    with open(readme_path, "r", encoding="utf-8") as handle:
        readme = handle.read()

    if README_START_MARKER not in readme or README_END_MARKER not in readme:
        raise ValueError(
            f"{readme_path} does not contain the "
            f"{README_START_MARKER} / {README_END_MARKER} markers"
        )

    pattern = re.compile(
        re.escape(README_START_MARKER) + r".*?" + re.escape(README_END_MARKER),
        re.DOTALL,
    )
    replacement = f"{README_START_MARKER}\n{section}\n{README_END_MARKER}"
    updated = pattern.sub(lambda _match: replacement, readme, count=1)

    if updated == readme:
        return False

    with open(readme_path, "w", encoding="utf-8") as handle:
        handle.write(updated)
    return True


def _series(results, variant):
    """Measurements of one variant across all operations, 0 when missing."""
    return [results[op].get(variant, 0) for op, _ in OPERATIONS]


def _ensure_min_visible(values, minimum):
    """Keep non-zero bars at least ``minimum`` wide so they stay visible."""
    return [max(value, minimum) if value > 0 else 0 for value in values]


def _plot(results, filename, log_scale, output_dir):
    positions = np.arange(len(OPERATIONS))
    width = 0.15
    figure, axes = plt.subplots(figsize=(12, 8))

    series = {key: _series(results, key) for key, _, _ in VARIANTS}

    if log_scale:
        plotted = series
    else:
        # On a linear scale Doublets bars are invisible next to SpacetimeDB, so
        # give every non-zero measurement a minimum visible width (~0.5% of the
        # maximum), matching the sibling benchmark charts.
        all_values = [value for values in series.values() for value in values]
        max_value = max(all_values) if all_values else 1
        minimum = max_value * 0.005
        plotted = {
            key: _ensure_min_visible(values, minimum) for key, values in series.items()
        }

    offset_base = (len(VARIANTS) - 1) / 2
    for index, (key, label, color) in enumerate(VARIANTS):
        offset = (index - offset_base) * width
        axes.barh(positions + offset, plotted[key], width, label=label, color=color)

    axes.set_xlabel("Time (ns) – log scale" if log_scale else "Time (ns)")
    axes.set_title("Benchmark Comparison: SpacetimeDB vs Doublets (Rust)")
    axes.set_yticks(positions)
    axes.set_yticklabels([label for _, label in OPERATIONS])
    if log_scale:
        axes.set_xscale("log")
    axes.legend()
    figure.tight_layout()

    path = os.path.join(output_dir, filename) if output_dir else filename
    figure.savefig(path)
    plt.close(figure)
    print(f"Generated {path}")


def generate_charts(results, output_dir=""):
    """Generate the linear and logarithmic comparison charts."""
    if not HAS_MATPLOTLIB:
        return
    _plot(results, "bench_rust.png", log_scale=False, output_dir=output_dir)
    _plot(results, "bench_rust_log_scale.png", log_scale=True, output_dir=output_dir)


def copy_charts(docs_dir, output_dir=""):
    """Copy generated charts into the documentation directory."""
    os.makedirs(docs_dir, exist_ok=True)
    for chart in ("bench_rust.png", "bench_rust_log_scale.png"):
        source = os.path.join(output_dir, chart) if output_dir else chart
        if os.path.exists(source):
            shutil.copyfile(source, os.path.join(docs_dir, chart))
            print(f"Copied {source} -> {os.path.join(docs_dir, chart)}")


def parse_args(argv):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "input",
        nargs="?",
        default="out.txt",
        help="criterion bencher output file (default: out.txt)",
    )
    parser.add_argument(
        "--results",
        default="results.md",
        help="path of the generated Markdown table (default: results.md)",
    )
    parser.add_argument(
        "--readme",
        default=None,
        help="README to update in place between the benchmark result markers",
    )
    parser.add_argument(
        "--docs-dir",
        default=None,
        help="directory the generated charts are copied to (for example ../docs/benchmarks)",
    )
    parser.add_argument(
        "--output-dir",
        default="",
        help="directory the charts are written to (default: working directory)",
    )
    return parser.parse_args(argv)


def main(argv=None):
    args = parse_args(argv if argv is not None else sys.argv[1:])
    results = parse_results(args.input)

    if not has_any_results(results):
        print(f"No benchmark data found in {args.input}")
        print(report_input_excerpt(args.input))
        return 1

    section = render_results_section(results)
    table = format_results_table(results)
    print(table)

    with open(args.results, "w", encoding="utf-8") as handle:
        handle.write(section + "\n")
    print(f"Generated {args.results}")

    generate_charts(results, args.output_dir)

    if args.docs_dir:
        copy_charts(args.docs_dir, args.output_dir)

    if args.readme:
        changed = update_readme(args.readme, section)
        print(
            f"{'Updated' if changed else 'No changes needed in'} {args.readme}",
        )

    return 0


if __name__ == "__main__":
    sys.exit(main())
