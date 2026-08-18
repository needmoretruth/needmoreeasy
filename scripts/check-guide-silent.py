#!/usr/bin/env python3
"""Finds a line in a guide that was meant as a command and quietly became text.

    python scripts/check-guide-silent.py

The compiler's rule is that a line it does not recognise as NME is left alone
as Python, and a line that reads like an ordinary sentence is printed. Both are
right. What is wrong is a line the *author* meant as a command that falls
through to being printed, because nothing says so: the guide compiles, the
example runs, and the output is a sentence instead of the thing it promised.

That is not hypothetical. On 2026-08-18 guide 05 taught this:

    할 일은 목록          →  print("할 일은 목록")
    할 일에 청소 넣어      →  print("할 일에 청소 넣어")

A name cannot contain a space, so `할 일` is not a name and the two lines were
printed word for word. `할일은 목록` is the working form. Nothing caught it —
the block compiled, and `check-guide-code.py` only asks whether it compiles.

The signal used here is exact. When a line is understood, its action word is
consumed: `준비물 보여줘` becomes `print("준비물")`, without the `보여줘`. So a
line that comes back as `print("<the whole line, action word and all>")` was
not understood. Ordinary prose does that too, which is fine and expected — so
only lines carrying a word that no one writes by accident are reported.
"""

from __future__ import annotations

import re
import subprocess
import sys
import tempfile
import textwrap
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GUIDES = ROOT / "docs/guides"

# Words that appear only when a command was meant. Keep this list conservative:
# a word that also reads as ordinary prose belongs in `check-prose-blocks.py`,
# which asks the opposite question.
ACTION_WORDS = [
    "넣어", "물어봐", "기다려", "반복해서", "목록", "저장", "지워", "그어",
    "쿨타임", "시간 재기", "골라", "뽑아", "멈춰", "건너뛰어",
    "append", "wait", "times", "ask", "set ", "clear", "draw", "pick",
    "choose", "list of", "cooldown", "shuffle", "sort", "remove",
]


def binary() -> Path:
    for candidate in (ROOT / "target/release/nme", ROOT / "target/debug/nme"):
        if candidate.is_file():
            return candidate
    raise SystemExit(
        "check-guide-silent: build the compiler first (cargo build --release -p nme-cli)"
    )


def blocks(text: str):
    """Every ```nme block, dedented, skipping the ones marked as failing."""
    for match in re.finditer(r"^([ \t]*)```nme\n(.*?)^\1```", text, re.S | re.M):
        before = text[max(0, match.start() - 80) : match.start()]
        if "nme-check: skip" in before:
            continue
        yield textwrap.dedent(match.group(2))


def main() -> None:
    nme = binary()
    problems: list[tuple[str, str, str]] = []

    for guide in sorted(GUIDES.glob("[0-9]*.md")):
        source = guide.read_text(encoding="utf-8")
        for block in blocks(source):
            with tempfile.TemporaryDirectory() as folder:
                nme_file = Path(folder) / "guide.nme"
                nme_file.write_text(block, encoding="utf-8")
                python_file = Path(folder) / "guide.py"
                finished = subprocess.run(
                    [str(nme), "build", str(nme_file), "-o", str(python_file)],
                    capture_output=True,
                    text=True,
                    stdin=subprocess.DEVNULL,
                )
                if finished.returncode != 0:
                    # A block that does not compile is `check-guide-code.py`'s job.
                    continue
                produced = python_file.read_text(encoding="utf-8").split("\n")
            for written, became in zip(block.split("\n"), produced):
                line = written.strip()
                if not line or line.startswith("#"):
                    continue
                if became.strip() != f'print("{line}")':
                    continue
                if any(word in line for word in ACTION_WORDS):
                    problems.append((guide.name, line, became.strip()))

    for name, line, became in problems:
        print(f"{name}: this looks like a command but was printed as words")
        print(f"    {line}")
        print(f"    -> {became}")
    if problems:
        print(f"check-guide-silent: {len(problems)} line(s) quietly became text")
        sys.exit(1)
    print("check-guide-silent: ok")


if __name__ == "__main__":
    main()
