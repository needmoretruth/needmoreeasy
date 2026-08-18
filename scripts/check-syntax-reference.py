#!/usr/bin/env python3
"""Proves that docs/syntax.md and docs/syntax.ko.md tell the truth.

Two checks:

1. **Completeness** — every spelling in every keyword list in the compiler
   appears somewhere in both files. A new accepted word that nobody documented
   fails the build.
2. **Accuracy** — every `NME line → Python` row is actually compiled and
   compared against what the table claims.

    python scripts/check-syntax-reference.py [path/to/nme]

The compiler binary defaults to `target/release/nme`, then `target/debug/nme`.
Without one, the accuracy check is skipped and says so.
"""

from __future__ import annotations

import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PARSER = (ROOT / "crates/nme-core/src/parser.rs").read_text(encoding="utf-8")
SYNTAX = (ROOT / "crates/nme-core/src/syntax.rs").read_text(encoding="utf-8")
ENGLISH = ROOT / "docs/syntax.md"
KOREAN = ROOT / "docs/syntax.ko.md"

# Word lists whose contents must all be documented. Everything else in the
# compiler is either an internal helper or a Python keyword.
DOCUMENTED_LISTS = [
    "SAY_WORDS_EN", "SAY_WORDS_KO",
    "ASK_WORDS_EN", "ASK_WORDS_KO",
    "SET_WORDS_EN", "SET_WORDS_KO",
    "UPDATE_ADD_WORDS_EN", "UPDATE_ADD_WORDS_KO",
    "UPDATE_SUBTRACT_WORDS_EN", "UPDATE_SUBTRACT_WORDS_KO",
    "UPDATE_MULTIPLY_WORDS_EN", "UPDATE_MULTIPLY_WORDS_KO",
    "UPDATE_DIVIDE_WORDS_EN", "UPDATE_DIVIDE_WORDS_KO",
    "WAIT_WORDS_EN", "WAIT_WORDS_KO",
    "REPEAT_WORDS_EN", "REPEAT_WORDS_KO",
    "WHEN_WORDS_EN", "WHEN_WORDS_KO",
    "WHILE_WORDS_EN", "WHILE_WORDS_KO",
    "ELSE_WORDS_EN", "ELSE_WORDS_KO",
    "BREAK_WORDS_EN", "BREAK_WORDS_KO",
    "CONTINUE_WORDS_EN", "CONTINUE_WORDS_KO",
    "END_WORDS_EN", "END_WORDS_KO",
    "USE_WORDS_EN", "USE_WORDS_KO",
    "APPEND_WORDS_EN", "APPEND_WORDS_KO",
    "LIST_WORDS_EN", "LIST_WORDS_KO",
    "NUMBER_WORDS", "LATEST_WORDS", "SENTENCE_FILLERS",
    "KOREAN_PARTICLES",
    "NUMBER_WORDS_EN", "NUMBER_WORDS_KO",
    "TIMES_WORDS_EN", "TIMES_WORDS_KO",
    "BREAK_ALIAS_WORDS_EN", "CONTINUE_ALIAS_WORDS_EN",
    "RANDOM_CHOICE_WORDS", "UPDATE_CONNECTOR_WORDS_EN",
    "APPEND_CONNECTORS_EN", "APPEND_TARGET_PARTICLES_KO",
    "SET_TARGET_PARTICLES_KO", "VALUE_ENDINGS_KO",
    "FILE_READ_WORDS_EN", "FILE_READ_WORDS_KO",
    "FILE_WRITE_WORDS_EN", "FILE_WRITE_WORDS_KO",
    "SLOW_WORDS_EN", "SLOW_WORDS_KO",
    "VERY_WORDS_EN", "VERY_WORDS_KO",
    "SLOW_EVERY_WORDS_EN", "SLOW_EVERY_WORDS_KO",
    "CLEAR_SCREEN_WORDS_EN", "CLEAR_SCREEN_WORDS_KO",
    "CLEAR_SCREEN_ACTIONS_EN", "CLEAR_SCREEN_ACTIONS_KO",
    "DRAW_LINE_WORDS_EN", "DRAW_LINE_WORDS_KO",
    "DRAW_LINE_ACTIONS_EN", "DRAW_LINE_ACTIONS_KO",
    "BOX_WORDS_EN", "BOX_WORDS_KO",
    "MIDDLE_WORDS_EN", "MIDDLE_WORDS_KO",
    "START_TIMER_WORDS_EN", "START_TIMER_WORDS_KO", "TIMER_WORDS_EN",
    "ELAPSED_WORDS_EN", "ELAPSED_WORDS_KO",
    "COOLDOWN_WORDS_EN", "COOLDOWN_WORDS_KO",
    "COOLDOWN_SET_WORDS_EN", "COOLDOWN_SET_WORDS_KO",
    "COOLDOWN_READY_WORDS_EN", "COOLDOWN_READY_WORDS_KO",
    "COOLDOWN_BUSY_WORDS_KO", "COOLDOWN_UNTIL_WORDS_KO",
]

# Rows that name a shape rather than a compilable line.
SKIP_MARKS = ("…", "unchanged")

# Every table assumes these names already exist, so a row can be one line long
# and still compile. The preamble is not compared against anything.
PREAMBLE = {
    "en": [
        "set name to Mina",
        "set score to 0",
        "set total to 0",
        "set ready to True",
        "set waiting to False",
        "set pause_length to 3",
        "set friends to list of Mina",
        "set names to list of Mina",
        "set memo to empty",
        "start the timer",
        "put door on cooldown for 3 seconds",
    ],
    "ko": [
        "이름은 민수",
        "점수는 0",
        "총합은 0",
        "준비는 참",
        "대기는 거짓",
        "쉬는시간은 3",
        "친구들은 목록 민수",
        "이름들은 목록 민수",
        "memo는 비어있음",
        "시간 재기 시작해",
        "문 쿨타임 3초 걸어",
    ],
}

# What to wrap a row in so that a block header, a branch, or a loop control
# statement can be compiled on its own.
BODY = {"en": "show ok", "ko": "확인 말해줘"}
OPEN_IF = {"en": "if ready", "ko": "만약에 준비가 있으면"}
OPEN_LOOP = {"en": "repeat 2 times", "ko": "2번 반복해"}
CLOSE = {"en": "end", "ko": "끝"}


def wrap(nme: str, claimed: str, language: str) -> tuple[list[str], int]:
    """Returns (lines to compile after the preamble, index of the row's line)."""
    if claimed.endswith(":") and not claimed.startswith(("elif", "else")):
        return [nme, "    " + BODY[language]], 0
    if claimed.startswith(("elif", "else")):
        # The flat block form, which is what the branch rows document.
        return [OPEN_IF[language], BODY[language], nme, BODY[language], CLOSE[language]], 2
    if claimed in ("break", "continue"):
        return [OPEN_LOOP[language], nme, CLOSE[language]], 1
    return [nme], 0


def words(name: str) -> list[str]:
    for source in (PARSER, SYNTAX):
        match = re.search(
            rf"const\s+{name}\s*:\s*&\[&str\]\s*=\s*&\[(.*?)\];", source, re.S
        )
        if match:
            return re.findall(r'"([^"]*)"', match.group(1))
    raise SystemExit(f"check-syntax-reference: no word list named {name}")


def check_completeness(problems: list[str]) -> None:
    english = ENGLISH.read_text(encoding="utf-8")
    korean = KOREAN.read_text(encoding="utf-8")
    for name in DOCUMENTED_LISTS:
        for word in words(name):
            for label, text in (("docs/syntax.md", english), ("docs/syntax.ko.md", korean)):
                if f"`{word}`" not in text:
                    problems.append(f"{label}: {name} accepts `{word}`, which is not listed")


def table_rows(path: Path) -> list[tuple[int, str, str]]:
    rows = []
    for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
        match = re.match(r"^\| (?:문장형|초급|고급|Sentence|Beginner|Advanced) \| `(.+?)` \| (.+?) \|$", line)
        if not match:
            continue
        nme, python = match.groups()
        rows.append((number, nme, python.strip()))
    return rows


def check_accuracy(problems: list[str], binary: Path) -> int:
    checked = 0
    with tempfile.TemporaryDirectory() as folder:
        source = Path(folder) / "row.nme"
        output = Path(folder) / "row.py"
        for path, language in ((ENGLISH, "en"), (KOREAN, "ko")):
            preamble = PREAMBLE[language]
            for number, nme, claimed in table_rows(path):
                if any(mark in nme or mark in claimed for mark in SKIP_MARKS):
                    continue
                if not claimed.startswith("`"):
                    continue
                expected = claimed.strip("`")
                lines, row_at = wrap(nme, expected, language)
                source.write_text("\n".join(preamble + lines) + "\n", encoding="utf-8")
                output.unlink(missing_ok=True)
                result = subprocess.run(
                    [str(binary), "build", str(source), "-o", str(output)],
                    capture_output=True,
                    text=True,
                )
                if result.returncode != 0:
                    first = (result.stdout + result.stderr).strip().splitlines()
                    problems.append(
                        f"{path.name}:{number}: `{nme}` does not compile "
                        f"({first[0] if first else 'no output'})"
                    )
                    continue
                produced_lines = output.read_text(encoding="utf-8").splitlines()
                produced = produced_lines[len(preamble) + row_at].strip()
                if produced != expected:
                    problems.append(
                        f"{path.name}:{number}: `{nme}`\n"
                        f"    documented: {expected}\n"
                        f"    produced:   {produced}"
                    )
                checked += 1
    return checked


def main() -> None:
    problems: list[str] = []
    check_completeness(problems)

    if len(sys.argv) > 1:
        binary = Path(sys.argv[1])
    else:
        binary = next(
            (
                candidate
                for candidate in (ROOT / "target/release/nme", ROOT / "target/debug/nme")
                if candidate.is_file()
            ),
            None,
        )
    if binary is None:
        print("check-syntax-reference: no nme binary; skipping the accuracy check")
    else:
        checked = check_accuracy(problems, binary)
        print(f"check-syntax-reference: compiled {checked} documented rows")

    if problems:
        for problem in problems:
            print(problem, file=sys.stderr)
        raise SystemExit(f"check-syntax-reference: {len(problems)} problem(s)")
    print("check-syntax-reference: ok")


if __name__ == "__main__":
    main()
