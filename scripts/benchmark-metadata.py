#!/usr/bin/env python3
"""Record provenance on the VM performing the measurement, before artifact upload."""
import argparse
import json
import os
import platform
import re
import subprocess
import tomllib
from datetime import datetime, timezone
from pathlib import Path
from xml.etree import ElementTree

ROOT = Path(__file__).resolve().parents[1]


def package_version(lock, name):
    packages = tomllib.loads(lock.read_text())["package"]
    return next(package["version"] for package in packages if package["name"] == name)


def collect(language, backend):
    if language == "rust":
        versions = {"Rust SDK": package_version(ROOT / "rust/Cargo.lock", "spacetimedb-sdk"),
                    "Doublets (patched)": package_version(ROOT / "rust/Cargo.lock", "doublets")}
    else:
        project = ElementTree.parse(ROOT / "csharp/SpacetimeDBVSDoublets/SpacetimeDBVSDoublets.csproj")
        versions = {item.attrib["Include"]: item.attrib["Version"] for item in project.iter("PackageReference")}
    versions["SpacetimeDB module"] = package_version(ROOT / "rust/spacetime-module/Cargo.lock", "spacetimedb")
    # The configured pin is useful on Doublets VMs, which never start a server.
    versions["SpacetimeDB server/CLI"] = (ROOT / "scripts/spacetimedb-version").read_text().strip()
    if backend == "spacetimedb":
        actual = subprocess.check_output(["spacetime", "--version"], text=True)
        if f"tool version {versions['SpacetimeDB server/CLI']};" not in actual:
            raise ValueError(f"Unexpected SpacetimeDB version: {actual}")
    cpu = platform.processor() or "unknown"
    if Path("/proc/cpuinfo").exists():
        match = re.search(r"model name\s*:\s*(.+)", Path("/proc/cpuinfo").read_text())
        if match:
            cpu = match[1]
    repository, run = os.environ.get("GITHUB_REPOSITORY"), os.environ.get("GITHUB_RUN_ID")
    source = f"[GitHub Actions run {run}](https://github.com/{repository}/actions/runs/{run})" if repository and run else "a local benchmark run"
    return {"date": datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M UTC"),
            "source": source, "benchmark_links": os.environ.get("BENCHMARK_LINK_COUNT", "1000"),
            "background_links": os.environ.get("BACKGROUND_LINK_COUNT", "3000"),
            "versions": versions, "cpus": {backend: cpu}}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("language", choices=["rust", "csharp"])
    parser.add_argument("backend", choices=["spacetimedb", "doublets"])
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    args.output.write_text(json.dumps(collect(args.language, args.backend), indent=2) + "\n")
