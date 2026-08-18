#!/usr/bin/env python3
"""Measures how much of each guide is really sentence grammar.

The site tells a beginner that sentence grammar has "no quotes, no brackets, no
equals sign". This script applies exactly that test to every ```nme block in
the guides, so the claim can be checked instead of believed:

    python scripts/report-guide-tier.py            # a table, per part
    python scripts/report-guide-tier.py --guides   # one line per guide

It reports; it does not fail a build. Which guides are allowed to contain
Python is an editorial decision, not a rule the compiler can settle. What it
prevents is the index describing a part as "sentences alone" when it is not:
on 2026-08-18 Part 2 was titled that way and 796 of its 864 lines were Python.
"""

from __future__ import annotations

import argparse
import json
import re
import textwrap
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GUIDES = ROOT / "docs/guides"

# English NME genuinely uses the words `for` and `while`, and English prose in a
# `say` line may hold an apostrophe, so only punctuation and Python-only
# keywords count against a line here.
NOT_SENTENCE = re.compile(
    r'["()\[\]{}]|(?<![<>!=])=|:\s*$|\bdef\b|\bimport\b|\breturn\b|\blambda\b|\.\w+\('
)

PARTS = [
    (1, 16, "starting with sentences"),
    (17, 36, "small programs"),
    (37, 50, "files and data"),
    (51, 58, "lists and text"),
    (59, 66, "when a program grows"),
    (67, 76, "analysis and the internet"),
    (77, 86, "building a language"),
    (87, 90, "cryptocurrency"),
]


def blocks(text: str) -> list[str]:
    """Every ```nme block, dedented out of whatever list it sits in."""
    return [
        textwrap.dedent(match.group(2))
        for match in re.finditer(r"^([ \t]*)```nme\n(.*?)^\1```", text, re.S | re.M)
    ]


def measure(path: Path) -> tuple[int, int, list[str]]:
    total = hard = 0
    samples: list[str] = []
    for block in blocks(path.read_text(encoding="utf-8")):
        for line in block.split("\n"):
            stripped = line.strip()
            if not stripped or stripped.startswith("#"):
                continue
            total += 1
            if NOT_SENTENCE.search(stripped):
                hard += 1
                if len(samples) < 2:
                    samples.append(stripped[:60])
    return total, hard, samples


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--guides", action="store_true", help="one line per guide")
    parser.add_argument("--json", action="store_true")
    arguments = parser.parse_args()

    rows = []
    for path in sorted(GUIDES.glob("[0-9]*.md")):
        if path.name.endswith(".ko.md"):
            continue
        total, hard, samples = measure(path)
        rows.append(
            {
                "guide": path.name,
                "number": int(path.name[:2]),
                "lines": total,
                "not_sentence": hard,
                "examples": samples,
            }
        )

    if arguments.json:
        print(json.dumps(rows, ensure_ascii=False, indent=2))
        return

    if arguments.guides:
        for row in rows:
            mark = "  " if row["not_sentence"] == 0 else "* "
            print(
                f'{mark}{row["guide"]:<26}{row["lines"]:>5} lines'
                f'{row["not_sentence"]:>6} not sentences   '
                + " | ".join(row["examples"])[:52]
            )
        return

    print(f'{"part":<26}{"guides":>7}{"sentence-only":>15}{"lines":>7}{"not sentences":>15}')
    for low, high, name in PARTS:
        chosen = [row for row in rows if low <= row["number"] <= high]
        pure = sum(1 for row in chosen if row["not_sentence"] == 0 and row["lines"])
        lines = sum(row["lines"] for row in chosen)
        hard = sum(row["not_sentence"] for row in chosen)
        print(f'{low:02d}-{high:02d} {name:<20}{len(chosen):>7}{pure:>15}{lines:>7}{hard:>15}')


if __name__ == "__main__":
    main()
