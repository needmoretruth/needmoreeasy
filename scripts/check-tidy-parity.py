#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Proves the tidier lands one program in one place, whichever cell it came from.

`check-tier-parity.py` proves a capability can be *written* at every level and
in both languages. This script proves the other half of the owner's rule: that
a program written in one cell can be **turned into** any other cell and come
back written the way that cell writes it.

    한국어 → 영어든 그 반대든 완전히 다 변환되어야 하고,
    고급 → 문장이든 반대든 초급이든 뭐든 전부 다 바뀌어야 한다.

The check is a sameness check, so nothing here has to say what the right answer
looks like:

    two programs that mean the same thing, tidied into the same cell,
    must come out as the same text.

Where a program came from may not show in where it lands. A statement the
tidier cannot write in Korean stays English and is caught; one it cannot write
at beginner level stays a sentence and is caught; one it writes two ways
depending on the way in is caught as well.

"Mean the same thing" is decided by the compiler: two cells of a row are
compared only when they produce the same Python. Message text and names are
never translated — turning `안녕` into `Hello` would change what the program
prints — so a row's own `names` map is applied to both sides first.

The capability table is the one in `check-tier-parity.py`, so a statement added
there is carried here without anybody editing this file.

    python scripts/check-tidy-parity.py [path/to/nme]

Cells that cannot be closed today live in `KNOWN_GAPS`, one line of reason
each. Everything else has to pass, so a new gap fails the build.
"""

from __future__ import annotations

import importlib.util
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

spec = importlib.util.spec_from_file_location(
    "tier_parity", ROOT / "scripts/check-tier-parity.py"
)
tier = importlib.util.module_from_spec(spec)
sys.modules["tier_parity"] = tier
spec.loader.exec_module(tier)

MATRIX = tier.MATRIX
PREAMBLE = tier.PREAMBLE
CELLS = [
    ("sentence", "en"), ("sentence", "ko"),
    ("beginner", "en"), ("beginner", "ko"),
    ("advanced", "en"),
]

# Rows whose two spellings are the same program written in files that close
# their blocks differently, one line of reason each. NME closes a block with
# `end` or with the indentation under a `:`, and which one a file uses is not
# the tidier's to change: a `:` put on a header whose block is closed by `end`
# leaves the `end` closing nothing. So these rows come out with the mark the
# file they came from had, which is right and is still a difference.
#
# The key is (row id, target level, target language).
KNOWN_GAPS: dict[tuple[str, str, str], str] = {
    (row, level, language): "the two files close their blocks differently"
    for row in ("end", "chance_block")
    for level in ("sentence", "beginner", "advanced")
    for language in ("en", "ko")
}
# A story block lowers to `if True:` and its lines to `print(...)`, which is
# what an ordinary condition on `True` lowers to as well. Reading `if True:`
# back as a story would turn every line under it into prose, so it is read as
# the condition it is. One Python shape, two NME meanings; the tidier keeps
# the one the writer can check.
KNOWN_GAPS.update(
    {
        (row, level, language): "`if True:` is a condition, not a story"
        for row in ("story", "story_slow", "story_very_slow", "story_slow_every")
        for level in ("sentence", "beginner", "advanced")
        for language in ("en", "ko")
    }
)


def program(row, cell):
    """The row's own program for one cell, on top of that cell's preamble."""
    level, language = cell
    lines, _ = row["cells"][cell]
    return "\n".join(PREAMBLE["en" if level == "advanced" else language]
                     + list(lines)) + "\n"


def one_language(text, row, language):
    """The text with this row's Korean words put in their English twins.

    Message text is never translated by the tidier, and it should not be: the
    program would print something else. So the two sides are brought together
    here instead, and only for the words the row itself introduces.
    """
    if language != "ko":
        return text
    for korean, english in row["names"].items():
        text = text.replace(korean, english)
    return text


def statements(text):
    return [line.rstrip() for line in text.split("\n")
            if line.strip() and not line.strip().startswith("#")]


def run(nme, source, arguments, out_name):
    with tempfile.TemporaryDirectory() as folder:
        a = Path(folder) / "a.nme"
        b = Path(folder) / out_name
        a.write_text(source, encoding="utf-8")
        subprocess.run([str(nme), *arguments, str(a), "-o", str(b)],
                       capture_output=True, text=True)
        return b.read_text(encoding="utf-8") if b.is_file() else None


def convert(nme, source, level, language):
    return run(nme, source, ["convert", "--level", level, "--language", language],
               "b.nme")


def python_of(nme, source):
    return run(nme, source, ["build"], "b.py")


def main():
    nme = Path(sys.argv[1]) if len(sys.argv) > 1 else None
    if nme is None:
        for candidate in tier.nme_candidates(ROOT):
            if candidate.is_file():
                nme = candidate
                break
    if nme is None or not nme.is_file():
        print("check-tidy-parity: 컴파일러를 찾지 못했습니다", file=sys.stderr)
        return 1

    compared = 0
    problems = []
    for row in MATRIX:
        sources = {}
        for cell in CELLS:
            if row["cells"].get(cell) is None:
                continue
            text = program(row, cell)
            python = python_of(nme, text)
            if python is None:
                continue
            sources[cell] = (text, one_language(python, row, cell[1]))
        # Cells that mean the same thing, and so must tidy to the same text.
        families: dict[str, list] = {}
        for cell, (text, python) in sources.items():
            families.setdefault(python, []).append(cell)
        for target in CELLS:
            for family in families.values():
                if len(family) < 2:
                    continue
                answers = {}
                for cell in family:
                    got = convert(nme, sources[cell][0], target[0], target[1])
                    answers[cell] = (one_language(got, row, cell[1])
                                     if got is not None else None)
                compared += len(family)
                first = family[0]
                for other in family[1:]:
                    key = (row["id"], target[0], target[1])
                    if key in KNOWN_GAPS:
                        continue
                    if answers[first] is None or answers[other] is None:
                        problems.append((key, first, other, "변환이 거절되었습니다",
                                         "", ""))
                        continue
                    if statements(answers[first]) != statements(answers[other]):
                        pair = next(
                            ((a, b) for a, b in zip(statements(answers[first]),
                                                    statements(answers[other]))
                             if a != b),
                            (" / ".join(statements(answers[first])),
                             " / ".join(statements(answers[other]))),
                        )
                        problems.append((key, first, other, "서로 다르게 나왔습니다",
                                         pair[0], pair[1]))

    # One broken statement shows up in every row that uses it, so the report is
    # by statement rather than by row: that is the list somebody works through.
    seen: dict[tuple[str, str, str, str], list[str]] = {}
    for (rid, level, language), first, other, why, a, b in problems:
        key = (f"{first[0]}/{first[1]} vs {other[0]}/{other[1]}"
               f" → {level}/{language}", why, a, b)
        seen.setdefault(key, []).append(rid)
    for (direction, why, a, b), rows in sorted(seen.items(),
                                               key=lambda item: -len(item[1])):
        print(f"[{len(rows):>3}] {direction}: {why}")
        if a or b:
            print(f"      한쪽: {a[:200]}")
            print(f"      다른쪽: {b[:200]}")
        print(f"      해당 항목: {', '.join(sorted(set(rows))[:8])}")
    print(f"\ncheck-tidy-parity: {compared}개를 견주어 "
          f"{len(problems)}개가 어긋났습니다")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
