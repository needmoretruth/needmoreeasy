#!/usr/bin/env python3
"""Move the version one step, everywhere it is written.

    python scripts/bump-version.py patch
    python scripts/bump-version.py minor

The owner's rule, 2026-08-19: **버전업은 반드시 배포할 때만.** So this is called
by the deploy script and by nothing else. It edits `Cargo.toml`, `Cargo.lock`,
every document that quotes the version, and opens a heading in both changelogs.

`patch` is for a deploy that fixes and polishes. `minor` is for one that adds
something a person can now write and could not before. There is no `major`:
`scripts/check-version.py` refuses anything that leaves 0, because 1.0.0 would
promise that today's programs keep working and the owner has said the opposite.
"""

from __future__ import annotations

import re
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
VERSION = re.compile(r"^(\d+)\.(\d+)\.(\d+)$")

# Everything that quotes the version, in both languages.
QUOTES_VERSION = (
    "README.md", "README.ko.md",
    "docs/install.md", "docs/install.ko.md",
    "docs/ai-assistants.md", "docs/ai-assistants.ko.md",
    "docs/versioning.md", "docs/versioning.ko.md",
)

HEADING = {
    "CHANGELOG.md": "## Unreleased",
    "CHANGELOG.ko.md": "## 미출시 (Unreleased)",
}


def main() -> None:
    if len(sys.argv) != 2 or sys.argv[1] not in ("minor", "patch"):
        raise SystemExit("usage: bump-version.py minor|patch")
    step = sys.argv[1]

    cargo = ROOT / "Cargo.toml"
    text = cargo.read_text(encoding="utf-8")
    current = tomllib.loads(text)["workspace"]["package"]["version"]
    found = VERSION.fullmatch(current)
    if not found:
        raise SystemExit(f"bump-version: {current} is not `0.MINOR.PATCH` yet")
    major, minor, patch = (int(g) for g in found.groups())
    nxt = f"{major}.{minor + 1}.0" if step == "minor" else f"{major}.{minor}.{patch + 1}"

    cargo.write_text(text.replace(f'version = "{current}"', f'version = "{nxt}"', 1),
                     encoding="utf-8")
    lock = ROOT / "Cargo.lock"
    lock.write_text(lock.read_text(encoding="utf-8").replace(f'version = "{current}"',
                                                            f'version = "{nxt}"'),
                    encoding="utf-8")
    for name in QUOTES_VERSION:
        path = ROOT / name
        path.write_text(path.read_text(encoding="utf-8").replace(current, nxt),
                        encoding="utf-8")

    # The unreleased notes become this version's notes, and a fresh unreleased
    # heading opens above them.
    for name, unreleased in HEADING.items():
        path = ROOT / name
        body = path.read_text(encoding="utf-8")
        if unreleased not in body:
            raise SystemExit(f"bump-version: {name} has no {unreleased!r} heading")
        body = body.replace(unreleased, f"{unreleased}\n\n## {nxt}", 1)
        path.write_text(body, encoding="utf-8")

    # The generated documents carry the version too; rebuild rather than edit.
    for script in ("scripts/build-syntax-reference.py", "scripts/build-ai-prompts.py"):
        subprocess.run([sys.executable, script], cwd=ROOT, check=True,
                       stdout=subprocess.DEVNULL)

    print(f"bump-version: {current} → {nxt}")


if __name__ == "__main__":
    main()
