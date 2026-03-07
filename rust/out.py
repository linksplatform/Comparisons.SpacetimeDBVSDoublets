#!/usr/bin/env python3
"""
Benchmark result visualization for SpacetimeDB vs Doublets.

Reads Criterion bencher-format output from out.txt and generates:
- bench_rust.png: Linear scale comparison chart
- bench_rust_log_scale.png: Logarithmic scale comparison chart
- results.md: Markdown table with speedup ratios

Usage:
    python3 out.py [out.txt]
"""

import re
import sys
import os

try:
    import matplotlib
    matplotlib.use('Agg')
    import matplotlib.pyplot as plt
    import numpy as np
    HAS_MATPLOTLIB = True
except ImportError:
    print("Warning: matplotlib/numpy not installed, skipping chart generation")
    HAS_MATPLOTLIB = False

# Bencher output line format:
# test <benchmark>/<operation>/<variant>/<size> ... bench: <ns_per_iter> ns/iter (+/- <variance>)
BENCHER_PATTERN = re.compile(
    r'test (\w+)/(\w+)/(\w+)/(\d+)\s+\.\.\.\s+bench:\s+([\d,]+)\s+ns/iter'
)

OPERATIONS = [
    'create',
    'delete',
    'update',
    'query_all',
    'query_by_id',
    'query_by_source',
    'query_by_target',
]

OPERATION_LABELS = {
    'create': 'Create',
    'delete': 'Delete',
    'update': 'Update',
    'query_all': 'Query All',
    'query_by_id': 'Query by Id',
    'query_by_source': 'Query by Source',
    'query_by_target': 'Query by Target',
}

VARIANTS = {
    'SpacetimeDB': 'SpacetimeDB 2.0',
    'Doublets_United_Volatile': 'Doublets (United/Volatile)',
    'Doublets_Split_Volatile': 'Doublets (Split/Volatile)',
}

COLORS = {
    'SpacetimeDB': '#e74c3c',
    'Doublets_United_Volatile': '#2ecc71',
    'Doublets_Split_Volatile': '#3498db',
}


def parse_results(filename='out.txt'):
    """Parse bencher-format output into a nested dict: operation -> variant -> ns_per_iter."""
    results = {op: {} for op in OPERATIONS}

    if not os.path.exists(filename):
        print(f"Warning: {filename} not found")
        return results

    with open(filename, 'r') as f:
        content = f.read()

    for line in content.splitlines():
        m = BENCHER_PATTERN.search(line)
        if m:
            group, op, variant, size, ns_str = m.groups()
            ns = int(ns_str.replace(',', ''))
            if op in results:
                results[op][variant] = ns

    # Also handle Criterion's default output format
    # test group::benchmark/variant/size ... bench: X ns/iter (+/- Y)
    CRITERION_PATTERN = re.compile(
        r'test (\w+)::(\w+)/(\w+)/\d+\s+\.\.\.\s+bench:\s+([\d,]+)\s+ns/iter'
    )
    for line in content.splitlines():
        m = CRITERION_PATTERN.search(line)
        if m:
            _group, op, variant, ns_str = m.groups()
            ns = int(ns_str.replace(',', ''))
            if op in results:
                results[op][variant] = ns

    return results


def generate_charts(results):
    """Generate PNG comparison charts."""
    if not HAS_MATPLOTLIB:
        return

    variant_keys = list(VARIANTS.keys())
    op_labels = [OPERATION_LABELS.get(op, op) for op in OPERATIONS]
    n_ops = len(OPERATIONS)
    n_variants = len(variant_keys)

    # Collect data
    data = {}
    for variant in variant_keys:
        data[variant] = []
        for op in OPERATIONS:
            ns = results[op].get(variant, 0)
            data[variant].append(ns)

    x = np.arange(n_ops)
    width = 0.8 / n_variants

    def make_chart(ax, log_scale=False):
        for i, variant in enumerate(variant_keys):
            vals = [v if v > 0 else float('nan') for v in data[variant]]
            offset = (i - n_variants / 2 + 0.5) * width
            bars = ax.bar(
                x + offset, vals, width,
                label=VARIANTS[variant],
                color=COLORS[variant],
                alpha=0.85
            )

        ax.set_xlabel('Operation')
        ax.set_ylabel('Time (ns/iter)')
        title = 'SpacetimeDB vs Doublets — Link CRUD Benchmark'
        if log_scale:
            title += ' (Log Scale)'
            ax.set_yscale('log')
        ax.set_title(title)
        ax.set_xticks(x)
        ax.set_xticklabels(op_labels, rotation=30, ha='right')
        ax.legend()
        ax.grid(axis='y', alpha=0.3)
        plt.tight_layout()

    # Linear scale
    fig, ax = plt.subplots(figsize=(12, 6))
    make_chart(ax, log_scale=False)
    fig.savefig('bench_rust.png', dpi=150, bbox_inches='tight')
    plt.close(fig)
    print("Generated bench_rust.png")

    # Log scale
    fig, ax = plt.subplots(figsize=(12, 6))
    make_chart(ax, log_scale=True)
    fig.savefig('bench_rust_log_scale.png', dpi=150, bbox_inches='tight')
    plt.close(fig)
    print("Generated bench_rust_log_scale.png")


def generate_markdown_table(results):
    """Generate a Markdown results table with speedup ratios."""
    baseline = 'SpacetimeDB'
    doublets_variants = ['Doublets_United_Volatile', 'Doublets_Split_Volatile']

    header = (
        '| Operation | SpacetimeDB (ns/iter) '
        '| Doublets United (ns/iter) | Doublets United Speedup '
        '| Doublets Split (ns/iter) | Doublets Split Speedup |'
    )
    sep = '|---|---|---|---|---|---|'

    rows = [header, sep]
    for op in OPERATIONS:
        label = OPERATION_LABELS.get(op, op)
        baseline_ns = results[op].get(baseline, 0)
        baseline_str = f'{baseline_ns:,}' if baseline_ns else 'N/A'

        row = f'| {label} | {baseline_str} '
        for variant in doublets_variants:
            ns = results[op].get(variant, 0)
            ns_str = f'{ns:,}' if ns else 'N/A'
            if baseline_ns > 0 and ns > 0:
                speedup = baseline_ns / ns
                speedup_str = f'{speedup:.0f}x'
            else:
                speedup_str = 'N/A'
            row += f'| {ns_str} | {speedup_str} '
        row += '|'
        rows.append(row)

    table = '\n'.join(rows)
    with open('results.md', 'w') as f:
        f.write('# Benchmark Results\n\n')
        f.write(f'> Background: {os.environ.get("BACKGROUND_LINK_COUNT", "3000")} links\n')
        f.write(f'> Operations: {os.environ.get("BENCHMARK_LINK_COUNT", "1000")} links\n\n')
        f.write(table)
        f.write('\n')

    print("Generated results.md")
    print('\n' + table)


def main():
    filename = sys.argv[1] if len(sys.argv) > 1 else 'out.txt'
    results = parse_results(filename)

    if all(not v for v in results.values()):
        print(f"No benchmark data found in {filename}")
        return

    generate_charts(results)
    generate_markdown_table(results)


if __name__ == '__main__':
    main()
