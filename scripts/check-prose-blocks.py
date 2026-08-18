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
            for line in block.splitlines():
                text = line.strip()
                if not text or text.startswith("#") or not SENTENCE_END.search(text):
                    continue
                checked += 1
                with tempfile.TemporaryDirectory() as work:
                    nme = os.path.join(work, "p.nme")
                    out = os.path.join(work, "p.py")
                    open(nme, "w", encoding="utf-8").write(text + "\n")
                    run = subprocess.run(
                        [NME, "build", nme, "-o", out], capture_output=True, text=True
                    )
                    if run.returncode != 0:
                        continue
                    python = open(out, encoding="utf-8").read().strip()
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
