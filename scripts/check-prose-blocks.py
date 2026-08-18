#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Catches a guide line that reads as a sentence but compiles to something else.

A program that fails to compile announces itself. A program that compiles into
a *different* program does not: it runs, prints nothing, and the reader spends
an afternoon on it. The shape this catches is the one that actually shipped —

    터미널은 한 번에 작은 프로그램 하나씩만 물었습니다.
        ->  터미널 = "한 번에 작은 프로그램 하나씩만 물었습니다."

a line of story in a documented ```nme block, silently stored in a variable
named after its first word rather than printed.

    python3 scripts/check-prose-blocks.py [path-to-nme]

It reads every ```nme block in docs/ and every examples/*.nme, keeps the lines
that end the way a Korean or English sentence ends, and fails if any of them
compiles to a bare assignment or to a bare name. Exit code 1 on any hit.

A line shown inside a story block (`이야기:` / `story:`) is compiled together
with that header, because that is the program the document actually shows, and
it has to come out as one `print`. A story without a closing `end` runs to the
end of its fenced block, which is the reading the compiler itself takes.
"""
import glob
import os
import re
import subprocess
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
NME = sys.argv[1] if len(sys.argv) > 1 else os.path.join(ROOT, "target", "release", "nme")

# The endings that mean "this was a sentence, not a value".
SENTENCE_END = re.compile(
    r"(습니다|읍니다|입니다|이었습니다|였습니다|해요|예요|이에요|었다|았다)[.!?]?$"
    r"|[a-z][.!?]$"
)
ASSIGNMENT = re.compile(r'^[^\W\d]\w*\s*=\s*["\']')
BARE_NAME = re.compile(r"^[^\W\d]\w*$")

# A story block opens with a colon and closes with `end`/`끝`. Its lines are
# text by construction, so compiling one of them on its own would test
# something the document never claims. They are compiled *inside* their own
# header instead, and must still come out as one `print`.
STORY_HEADER = re.compile(r"(?:\bstory\b|\btale\b|이야기|얘기).*[:\uff1a]\s*$", re.I)
STORY_END = ("end", "finish", "done", "끝", "종료", "마침")
PRINTS = ("print(", "[print(")


def compiled(source: str) -> str | None:
    """The Python `source` becomes, or None when the compiler refuses it."""
    with tempfile.TemporaryDirectory() as work:
        nme = os.path.join(work, "p.nme")
        out = os.path.join(work, "p.py")
        open(nme, "w", encoding="utf-8").write(source)
        run = subprocess.run(
            [NME, "build", nme, "-o", out], capture_output=True, text=True
        )
        if run.returncode != 0:
            return None
        return open(out, encoding="utf-8").read().strip()


def blocks(path: str):
    text = open(path, encoding="utf-8").read()
    if path.endswith(".nme"):
        return [text]
    return re.findall(r"```nme\n(.*?)```", text, re.S)


def main() -> int:
    if not os.path.isfile(NME):
        print(f"check-prose-blocks: no compiler at {NME}", file=sys.stderr)
        return 2

    hits = []
    checked = 0
    sources = sorted(
        glob.glob(os.path.join(ROOT, "docs", "**", "*.md"), recursive=True)
        + glob.glob(os.path.join(ROOT, "examples", "*.nme"))
    )
    for path in sources:
        for block in blocks(path):
            story = None
            for line in block.splitlines():
                text = line.strip()
                if story is not None and text.lower() in STORY_END:
                    story = None
                    continue
                if story is None and text.endswith((":", "\uff1a")) and STORY_HEADER.search(text):
                    story = line
                    continue
                if not text or text.startswith("#") or not SENTENCE_END.search(text):
                    continue
                checked += 1
                if story is not None:
                    # Inside a story block the line must still be one print.
                    python = compiled(f"{story}\n{line}\n")
                    if python is None:
                        continue
                    last = python.splitlines()[-1].strip()
                    if not last.startswith(PRINTS):
                        hits.append((os.path.relpath(path, ROOT), text, last))
                    continue
                python = compiled(text + "\n")
                if python is None:
                    continue
                if ASSIGNMENT.match(python) or BARE_NAME.match(python):
                    hits.append((os.path.relpath(path, ROOT), text, python))

    print(f"check-prose-blocks: checked {checked} sentence-shaped lines")
    for path, text, python in hits:
        print(f"check-prose-blocks: {path}: {text!r} -> {python[:70]}")
    if hits:
        print(f"check-prose-blocks: {len(hits)} line(s) compile to something else")
        return 1
    print("check-prose-blocks: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
