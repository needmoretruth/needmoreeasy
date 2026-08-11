#!/usr/bin/env python3
from pathlib import Path

OLD_TARGET = "452312848583266388373324160190187140051835877600158453279131187530910662656"
NEW_TARGET = "7237005577332262213973186563042994240829374041602535252466099000494570602496"


def replace(path: str, old: str, new: str) -> None:
    file = Path(path)
    text = file.read_text(encoding="utf-8")
    if old not in text:
        raise SystemExit(f"anchor changed: {path}: {old!r}")
    file.write_text(text.replace(old, new, 1), encoding="utf-8")


for path in [
    "examples/needmorecoin-sentence.ko.nme",
    "examples/needmorecoin-sentence.en.nme",
    "examples/needmorecoin-beginner.ko.nme",
    "examples/needmorecoin-beginner.en.nme",
]:
    replace(path, OLD_TARGET, NEW_TARGET)

replace("examples/needmorecoin-advanced.ko.nme", '채굴접두사 = "00"', '채굴접두사 = "0"')
replace("examples/needmorecoin-advanced.en.nme", 'MINING_PREFIX = "00"', 'MINING_PREFIX = "0"')
replace("docs/guides/cryptocurrency.ko.md", '`00`으로 시작할 때까지', '`0`으로 시작할 때까지')
replace("docs/guides/cryptocurrency.md", 'starts with `00`', 'starts with `0`')

print("materialized four-bit learning proof of work")
