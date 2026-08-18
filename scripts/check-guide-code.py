#!/usr/bin/env python3
"""Compiles every NME program in the guides.

A guide that shows a program which does not compile teaches the wrong thing, so
every ```nme block in `docs/guides/` is run through `nme check`. Blocks fenced
as ```text are what the program prints, or a data file, or the mini-language a
guide is building — never NME, and never checked.

    python scripts/check-guide-code.py [path/to/nme] [--only 01,02,…]

Blocks that are deliberately fragments — a lone `else`, a template with `<…>`
placeholders, a line shown as an example of an error — opt out with a marker
comment on the line above the fence:

    <!-- nme-check: skip -->

The compiler binary defaults to `target/release/nme`, then `target/debug/nme`.
"""

from __future__ import annotations

import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GUIDES = ROOT / "docs/guides"
SKIP = "<!-- nme-check: skip -->"
FENCE = re.compile(r"^\s*```(\w*)\s*$")


def blocks(path: Path):
    lines = path.read_text(encoding="utf-8").splitlines()
    index = 0
    while index < len(lines):
        match = FENCE.match(lines[index])
        if not match or match.group(1) != "nme":
            index += 1
            continue
        skipped = index > 0 and SKIP in lines[index - 1]
        indent = len(lines[index]) - len(lines[index].lstrip())
        body, index = [], index + 1
        start = index
        while index < len(lines) and not FENCE.match(lines[index]):
            body.append(lines[index][indent:])
            index += 1
        index += 1
        if not skipped and body:
            yield start, "\n".join(body)


def main() -> None:
    arguments = [a for a in sys.argv[1:] if not a.startswith("--")]
    only = None
    for argument in sys.argv[1:]:
        if argument.startswith("--only"):
            only = set(argument.split("=", 1)[1].split(","))
    binary = Path(arguments[0]) if arguments else next(
        (
            candidate
            for candidate in (ROOT / "target/release/nme", ROOT / "target/debug/nme")
            if candidate.is_file()
        ),
        None,
    )
    if binary is None:
        raise SystemExit("check-guide-code: no nme binary found")

    problems, checked = [], 0
    with tempfile.TemporaryDirectory() as folder:
        source = Path(folder) / "block.nme"
        for path in sorted(GUIDES.glob("*.md")):
            if only and path.name.split("-")[0] not in only:
                continue
            for line_number, body in blocks(path):
                source.write_text(body + "\n", encoding="utf-8")
                result = subprocess.run(
                    [str(binary), "check", str(source)],
                    capture_output=True,
                    text=True,
                )
                checked += 1
                # A program that imports a sibling `.nme` file cannot be
                # compiled on its own; the guide shows the sibling in another
                # block. Everything up to the import was still accepted.
                if "error[E9007]" in result.stdout + result.stderr:
                    continue
                if result.returncode != 0:
                    message = (result.stdout + result.stderr).strip().splitlines()
                    first = next((m for m in message if "error" in m or "오류" in m), "")
                    problems.append(f"{path.name}:{line_number}: {first}")

    print(f"check-guide-code: compiled {checked} blocks")
    for problem in problems:
        print(problem, file=sys.stderr)
    if problems:
        raise SystemExit(f"check-guide-code: {len(problems)} block(s) do not compile")
    print("check-guide-code: ok")


if __name__ == "__main__":
    main()
