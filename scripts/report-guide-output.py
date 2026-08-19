#!/usr/bin/env python3
"""Runs the guides' programs and compares what they print with what the guide says.

    python scripts/report-guide-output.py            # only the mismatches
    python scripts/report-guide-output.py --all      # every program it ran

`check-guide-code.py` proves a program compiles. It cannot tell that the guide
promises one thing and the program prints another — which is how guide 13 came
to say a die roll printed a sentence it never printed, and how guide 05 taught
a line that silently turned into `print` of itself.

Only self-contained programs are run: no input, no randomness, no clock, no
network. Anything else is skipped and counted, because a report that quietly
covers a third of the guides is worse than one that says so.

It reports; it does not fail a build. Some ```text blocks are data files or an
abridged run, and deciding which is editorial.
"""

from __future__ import annotations

import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GUIDES = ROOT / "docs/guides"
FENCE = re.compile(r"^([ \t]*)```(\w*)\s*$")
SKIP = "<!-- nme-check: skip"

# A program whose output changes between runs cannot be compared with a fixed
# block. These are the words that make one.
UNSTABLE = re.compile(
    r"무작위|확률|섞어|기다려|시간 재기|잰시간|물어봐|입력|"
    r"\brandom\b|\bchance\b|shuffle|\bwait\b|timer|elapsed|\bask\b|\binput\b|"
    r"today|now\(|date\b|오늘|지금"
)

# A program that reads something it did not make in the same block needs the
# guide's earlier steps to have run. Reading a folder, or taking a command-line
# argument, is the same thing: the run here is not the run the guide describes.
NEEDS_SETUP = re.compile(
    r"읽어서|불러|열어서|폴더|argv|listdir|glob|walk|"
    r"\bread\b|\bload\b|json_load|file_read|folder|arguments"
)


def blocks(path: Path) -> list[tuple[int, int, str, str]]:
    """Every fenced block: (first body line, line after the closing fence,
    language, body). Line numbers are 1-based, as an editor shows them."""
    lines = path.read_text(encoding="utf-8").splitlines()
    found, index = [], 0
    while index < len(lines):
        match = FENCE.match(lines[index])
        if not match:
            index += 1
            continue
        skipped = index > 0 and SKIP in lines[index - 1]
        indent, language = len(match.group(1)), match.group(2)
        body, index = [], index + 1
        start = index
        while index < len(lines) and not FENCE.match(lines[index]):
            body.append(lines[index][indent:])
            index += 1
        index += 1
        if body and not skipped:
            found.append((start, index, language, "\n".join(body)))
    return found


# The sentence guides do not fence their output; they say it in the sentence
# right after the program — "`1, 3, 5, 9`가 나옵니다", "You get `Ada, Mina, Zoe`".
# Those backticked spans are a promise too, and they are the only promise the
# first three parts make.
SAYS_OUTPUT = re.compile(r"나옵니다|나와|출력|\bprints\b|You get|you get")
SPAN = re.compile(r"`([^`\n]+)`")
# Backticks in these sentences hold two different things: what comes out, and
# the piece of code being talked about. Printed output almost always carries a
# digit, a comma or a space; a bare identifier, a call and a file name never
# do. Under-reporting here is the cheaper mistake — a report that cries wolf
# gets skimmed and then ignored.
LOOKS_LIKE_OUTPUT = re.compile(r"[\d, ]")
LOOKS_LIKE_CODE = re.compile(r"[()\[\]{}]|\.(?:nme|py|txt|json|csv|md)\b|^[a-z_]+\(")


def promised_inline(lines: list[str], opens: int, after: int) -> list[str]:
    """The backticked spans in the sentence that carries a program's output.

    Guides put it on either side. The Python-heavy ones lead with it and end
    in a colon — "`hello`, `NME` … 가 출력됩니다:" — and the sentence guides put
    it underneath: "`1, 3, 5, 9`가 나옵니다". Above wins when both are there,
    because the sentence under a block is usually the *next* step's lead-in.
    """
    def spans(window: list[str]) -> list[str]:
        said = []
        for line in window:
            if line.strip().startswith("```"):
                break
            if SAYS_OUTPUT.search(line):
                said += [
                    span for span in SPAN.findall(line)
                    if LOOKS_LIKE_OUTPUT.search(span) and not LOOKS_LIKE_CODE.search(span)
                ]
        return said

    lead = [line for line in lines[max(0, opens - 3) : opens] if line.strip()]
    above = spans(list(reversed(lead))) if lead and lead[-1].rstrip().endswith(":") else []
    return above or spans(lines[after : after + 3])


def pairs(path: Path) -> list[tuple[int, str, list[str]]]:
    """Each ```nme block together with what the guide promises it prints.

    Three shapes carry that promise. `nme` then `text`, and `nme` then `sh`
    then `text` — "here is the program, here is the command, here is what it
    prints" — are the fenced ones, used 22 and 118 times. The third is a
    sentence: the first three parts of the book never fence their output, they
    write "`1, 3, 5, 9`가 나옵니다" underneath, and without this shape the
    report would cover only the Python-heavy end of the book.

    A ```text block that opens with a bracket or a brace is a data file the
    guide is about to read, not something the program printed.
    """
    lines = path.read_text(encoding="utf-8").splitlines()
    found = blocks(path)
    # Every line of NME the guide shows anywhere. A span quoting one of them is
    # naming the code, not promising output.
    # Both languages' blocks: a Korean guide quotes the English spelling and
    # the other way round, and either way it is code being named.
    twin = path.with_name(
        path.name.replace(".ko.md", ".md") if path.name.endswith(".ko.md")
        else path.name.replace(".md", ".ko.md")
    )
    both = found + (blocks(twin) if twin.exists() else [])
    code = "\n".join(entry[3] for entry in both if entry[2] == "nme")
    out = []
    for at, (line, after, language, body) in enumerate(found):
        if language != "nme":
            continue
        following = [entry[2] for entry in found[at + 1 : at + 3]]
        if following[:1] == ["text"]:
            shown = found[at + 1][3]
        elif following[:2] == ["sh", "text"]:
            shown = found[at + 2][3]
        else:
            # A span that also appears in the program is quoting the code to
            # explain it — `break`, `min(numbers)`, the file's own name — not
            # promising what comes out.
            said = [w for w in promised_inline(lines, line - 1, after) if w not in code]
            if said:
                out.append((line, body, said))
            continue
        if shown.lstrip()[:1] in ("[", "{"):
            continue
        out.append((line, body, [w for w in shown.splitlines() if w.strip()]))
    return out


def main() -> None:
    show_all = "--all" in sys.argv
    binary = next(
        (c for c in (ROOT / "target/release/nme", ROOT / "target/debug/nme") if c.is_file()),
        None,
    )
    if binary is None:
        raise SystemExit("report-guide-output: no nme binary found")

    ran = skipped = mismatched = 0
    for guide in sorted(GUIDES.glob("[0-9]*.md")):
        for line, program, promised in pairs(guide):
            if UNSTABLE.search(program) or NEEDS_SETUP.search(program):
                skipped += 1
                continue
            with tempfile.TemporaryDirectory() as folder:
                source = Path(folder) / "block.nme"
                source.write_text(program + "\n", encoding="utf-8")
                built = subprocess.run(
                    [str(binary), "build", str(source), "-o", str(Path(folder) / "block.py")],
                    capture_output=True, text=True, cwd=folder,
                )
                if built.returncode != 0:
                    skipped += 1
                    continue
                try:
                    got = subprocess.run(
                        [sys.executable, str(Path(folder) / "block.py")],
                        capture_output=True, text=True, cwd=folder,
                        stdin=subprocess.DEVNULL, timeout=10,
                    )
                except subprocess.TimeoutExpired:
                    skipped += 1
                    continue
            if got.returncode != 0:
                skipped += 1
                continue
            printed = got.stdout
            # Nothing came out, but the guide promises something: the program
            # was waiting on a file, a folder or an argument the guide made in
            # an earlier step. That is a skip, not a broken promise.
            if not printed.strip() and promised:
                skipped += 1
                continue
            ran += 1
            missing = [want for want in promised if want.strip() not in printed]
            if missing:
                mismatched += 1
                print(f"{guide.name}:{line}: the guide says it prints")
                for want in missing[:3]:
                    print(f"    {want.strip()}")
                first = printed.strip().splitlines()[:3] or ["(nothing)"]
                print("  but it printed")
                for line_out in first:
                    print(f"    {line_out}")
            elif show_all:
                print(f"{guide.name}:{line}: ok")

    print(f"\n돌려 본 프로그램 {ran}개 · 건너뛴 것 {skipped}개 · 어긋난 것 {mismatched}개")


if __name__ == "__main__":
    main()
