#!/usr/bin/env python3
"""Writes the third-party notices that travel with a prebuilt `nme` binary.

    python3 scripts/third-party-notices.py <rust-target> <output.md>

The `nme` executable statically links every crate it depends on, so a copy of
the binary has to carry their licence notices. This reads the exact set from
Cargo (normal dependency edges only, resolved for the given target, so build
tools and test-only crates are left out), then copies each crate's own licence
files out of Cargo's download cache. Identical texts are written once, with
every crate that uses them listed above it.

The release workflow runs this after `cargo build`, when every dependency's
source is already on disk.
"""

from __future__ import annotations

import hashlib
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LICENCE_FILE_PREFIXES = ("LICENSE", "LICENCE", "COPYING", "NOTICE", "UNLICENSE")


def cargo(*args: str) -> str:
    # Cargo writes UTF-8; Windows would otherwise decode it as the ANSI code page.
    return subprocess.run(
        ["cargo", *args], cwd=ROOT, check=True, capture_output=True, encoding="utf-8"
    ).stdout


def linked_crates(target: str) -> set[tuple[str, str]]:
    tree = cargo(
        "tree", "-p", "nme-cli", "--locked", "--target", target,
        "-e", "normal", "--prefix", "none", "--format", "{p}",
    )
    found = set()
    for line in tree.splitlines():
        parts = line.split()
        if len(parts) >= 2 and parts[1].startswith("v"):
            found.add((parts[0], parts[1][1:]))
    return found


def main() -> int:
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    target, output = sys.argv[1], Path(sys.argv[2])
    wanted = linked_crates(target)
    metadata = json.loads(cargo("metadata", "--format-version", "1", "--locked"))
    ours = set(metadata["workspace_members"])

    rows, texts = [], {}
    for package in sorted(metadata["packages"], key=lambda p: (p["name"], p["version"])):
        if (package["name"], package["version"]) not in wanted or package["id"] in ours:
            continue
        licence = package.get("license") or "see licence file"
        rows.append(f"| `{package['name']}` | {package['version']} | {licence} |")
        folder = Path(package["manifest_path"]).parent
        for path in sorted(folder.iterdir()):
            if path.is_file() and path.name.upper().startswith(LICENCE_FILE_PREFIXES):
                text = path.read_text(encoding="utf-8", errors="replace").strip()
                key = hashlib.sha256(text.encode()).hexdigest()
                texts.setdefault(key, (text, []))[1].append(
                    f"{package['name']} {package['version']} ({path.name})"
                )

    parts = [
        "# Third-party notices",
        "",
        "NME itself is Apache-2.0 (see `LICENSE`). This `nme` executable also contains",
        f"the {len(rows)} crates below, compiled for `{target}`. Their licence texts follow.",
        "",
        "## malachite — LGPL-3.0-only",
        "",
        "`rustpython-parser` uses the malachite crates for Python's arbitrary-precision",
        "integer literals, so this binary statically links LGPL-3.0-only code. The LGPL",
        "asks that you can rebuild the program against your own version of the library.",
        "The complete source of NME is at https://github.com/needmoretruth/needmoreeasy;",
        "check out the tag that matches `nme --version`, point Cargo at a modified",
        "malachite with a `[patch.crates-io]` entry, and run",
        f"`cargo build --release --locked -p nme-cli --target {target}`.",
        "`Cargo.lock` pins every other dependency, so the rest of the binary is the same.",
        "The LGPL-3.0 and the GPL-3.0 it builds on are included in `licenses/`.",
        "",
        "## Crates compiled into this binary",
        "",
        "| Crate | Version | Licence |",
        "| --- | --- | --- |",
        *rows,
        "",
        "## Licence texts",
    ]
    for text, users in texts.values():
        parts += ["", "---", "", "Used by: " + ", ".join(users), "", "```text", text, "```"]
    output.write_text("\n".join(parts) + "\n", encoding="utf-8")
    print(f"third-party-notices: {len(rows)} crates, {len(texts)} distinct licence texts -> {output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
