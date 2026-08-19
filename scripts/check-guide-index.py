#!/usr/bin/env python3
"""The index table must say what the guides say.

    python scripts/check-guide-index.py

`docs/guides/index.md` and `index.ko.md` end in a table of every guide: its
number, difficulty, topic, title and what you end up with. Those five facts are
also written at the top of each guide. Two copies of a fact drift, and on
2026-08-19 thirteen rows had: guide 19 had been rewritten to use the stopwatch
and the index still promised `time.time()`.

The guide is the source; the index follows it.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GUIDES = ROOT / "docs/guides"


def facts(path: Path) -> tuple[str, str, str]:
    text = path.read_text(encoding="utf-8")

    def field(*names: str) -> str:
        for name in names:
            found = re.search(rf"^- {name}:\s*(.+)$", text, re.M)
            if found:
                return found.group(1).strip()
        raise SystemExit(f"check-guide-index: {path.name} has no {names[0]} line")

    stars = field("난이도", "Difficulty").split(" ")[0]
    return stars, field("주제", "Topic"), field("결과물", "Result")


def main() -> None:
    problems: list[str] = []
    for index_path, korean in (
        (GUIDES / "index.md", False),
        (GUIDES / "index.ko.md", True),
    ):
        rows = {
            line[2:4]: line
            for line in index_path.read_text(encoding="utf-8").split("\n")
            if re.match(r"^\| \d\d \|", line)
        }
        for guide in sorted(GUIDES.glob("[0-9]*.md")):
            if guide.name.endswith(".ko.md") != korean:
                continue
            number = guide.name[:2]
            row = rows.get(number)
            if row is None:
                problems.append(f"{index_path.name}: no row for guide {number}")
                continue
            cells = [cell.strip() for cell in row.strip("|").split("|")]
            stars, topic, result = facts(guide)
            for got, want, what in (
                (cells[1], stars, "difficulty"),
                (cells[2], topic, "topic"),
                (cells[4], result, "result"),
            ):
                if got != want:
                    problems.append(
                        f"{index_path.name} row {number}: {what} says {got!r}, "
                        f"{guide.name} says {want!r}"
                    )

    for problem in problems:
        print(problem)
    if problems:
        print(f"check-guide-index: {len(problems)} row(s) disagree with their guide")
        sys.exit(1)
    print("check-guide-index: ok")


if __name__ == "__main__":
    main()
