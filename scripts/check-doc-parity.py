#!/usr/bin/env python3
"""Check bilingual Markdown navigation and local link parity."""

from __future__ import annotations

import re
import sys
from pathlib import Path
from urllib.parse import unquote


ROOT = Path(__file__).resolve().parents[1]
LINK_RE = re.compile(r"\[([^\]]+)\]\(([^)]+)\)")

EXPECTED_GUIDE_NAVIGATION = {
    ".md": (
        "../../README.md",
        "../install.md",
        "../getting-started.md",
        "../tutorial.md",
        "../language.md",
        "index.md",
    ),
    ".ko.md": (
        "../../README.ko.md",
        "../install.ko.md",
        "../getting-started.ko.md",
        "../tutorial.ko.md",
        "../language.ko.md",
        "index.ko.md",
    ),
}

EXPECTED_GUIDE_NAVIGATION_LINES = {
    ".md": "[Home](../../README.md) | [Install](../install.md) | "
    "[Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | "
    "[Language reference](../language.md) | [Guides](index.md)",
    ".ko.md": "[README](../../README.ko.md) | [설치](../install.ko.md) | "
    "[시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | "
    "[문법 안내](../language.ko.md) | [가이드](index.ko.md)",
}

GUIDE_METADATA = {
    ".md": ("Difficulty", "Prerequisites", "Topic", "Result"),
    ".ko.md": ("난이도", "선수 지식", "주제", "결과물"),
}


def local_markdown_target(path: Path, raw_target: str) -> Path | None:
    target = raw_target.split("#", 1)[0].strip().strip("<>")
    if not target or target.startswith(("http:", "https:", "mailto:")):
        return None
    candidate = (path.parent / target).resolve()
    return candidate if candidate.suffix == ".md" else None


def markdown_anchor(text: str) -> str:
    text = unquote(text).strip().lower()
    text = re.sub(r"<[^>]+>", "", text)
    text = re.sub(r"[^\w\s-]", "", text, flags=re.UNICODE)
    return re.sub(r"\s+", "-", text)


def markdown_anchors(path: Path) -> set[str]:
    anchors: set[str] = set()
    occurrences: dict[str, int] = {}
    in_fence = False
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.strip().startswith(("```", "~~~")):
            in_fence = not in_fence
            continue
        if in_fence:
            continue
        match = re.match(r"^#{1,6}\s+(.+?)\s*#*\s*$", line)
        if not match:
            continue
        base = markdown_anchor(match.group(1))
        occurrence = occurrences.get(base, 0)
        occurrences[base] = occurrence + 1
        anchors.add(base if occurrence == 0 else f"{base}-{occurrence}")
    return anchors


def is_explicit_english_reference(
    line: str, previous_line: str, line_number: int
) -> bool:
    # A Korean page must retain its language switch and may deliberately point
    # readers to the English twin when it explicitly says that it is doing so.
    return (
        (line_number <= 4 and "English" in line)
        or "영어" in line
        or "영어" in previous_line
    )


def check_korean_links(problems: list[str]) -> None:
    for path in ROOT.rglob("*.ko.md"):
        if any(part in {".git", "target"} for part in path.parts):
            continue
        lines = path.read_text(encoding="utf-8").splitlines()
        for line_number, line in enumerate(lines, start=1):
            previous_line = lines[line_number - 2] if line_number > 1 else ""
            if is_explicit_english_reference(line, previous_line, line_number):
                continue
            for _label, raw_target in LINK_RE.findall(line):
                candidate = local_markdown_target(path, raw_target)
                if candidate is None or candidate.name.endswith(".ko.md"):
                    continue
                korean_twin = candidate.with_name(f"{candidate.stem}.ko.md")
                if korean_twin.is_file():
                    relative = Path(raw_target.split("#", 1)[0].strip())
                    problems.append(
                        f"{path.relative_to(ROOT)}:{line_number}: "
                        f"link to English page {relative}; use {korean_twin.name}"
                    )


def check_local_markdown_links(problems: list[str]) -> None:
    anchor_cache: dict[Path, set[str]] = {}
    for path in ROOT.rglob("*.md"):
        if any(part in {".git", "target"} for part in path.parts):
            continue
        lines = path.read_text(encoding="utf-8").splitlines()
        for line_number, line in enumerate(lines, start=1):
            for _label, raw_target in LINK_RE.findall(line):
                candidate = local_markdown_target(path, raw_target)
                if candidate is not None and not candidate.is_file():
                    problems.append(
                        f"{path.relative_to(ROOT)}:{line_number}: "
                        f"missing local Markdown link target {raw_target}"
                    )
                if candidate is None or not candidate.is_file() or "#" not in raw_target:
                    continue
                fragment = raw_target.split("#", 1)[1].strip()
                if not fragment:
                    continue
                if candidate not in anchor_cache:
                    anchor_cache[candidate] = markdown_anchors(candidate)
                if markdown_anchor(fragment) not in anchor_cache[candidate]:
                    problems.append(
                        f"{path.relative_to(ROOT)}:{line_number}: "
                        f"missing Markdown link anchor {raw_target}"
                    )


def check_guide_navigation(problems: list[str]) -> None:
    guides = ROOT / "docs" / "guides"
    for path in guides.iterdir():
        if not path.is_file() or not re.match(r"\d+-", path.name):
            continue
        suffix = ".ko.md" if path.name.endswith(".ko.md") else ".md"
        expected = EXPECTED_GUIDE_NAVIGATION[suffix]
        head = "\n".join(path.read_text(encoding="utf-8").splitlines()[:8])
        expected_line = EXPECTED_GUIDE_NAVIGATION_LINES[suffix]
        if expected_line not in head:
            problems.append(
                f"{path.relative_to(ROOT)}: missing standard navigation row"
            )
        for target in expected:
            if target not in head:
                problems.append(
                    f"{path.relative_to(ROOT)}: missing navigation target {target}"
                )


def check_numbered_guide_pairs(problems: list[str]) -> None:
    guides = ROOT / "docs" / "guides"
    for path in guides.iterdir():
        if not path.is_file() or not re.match(r"\d+-", path.name):
            continue
        if path.name.endswith(".ko.md"):
            twin = path.with_name(f"{path.name[:-6]}.md")
            language = "English"
        else:
            twin = path.with_name(f"{path.stem}.ko.md")
            language = "Korean"
        if not twin.is_file():
            problems.append(
                f"{path.relative_to(ROOT)}: missing {language} guide twin {twin.name}"
            )


def check_guide_metadata(problems: list[str]) -> None:
    guides = ROOT / "docs" / "guides"
    for path in guides.iterdir():
        if not path.is_file() or not re.match(r"\d+-", path.name):
            continue
        suffix = ".ko.md" if path.name.endswith(".ko.md") else ".md"
        head = "\n".join(path.read_text(encoding="utf-8").splitlines()[:14])
        for field in GUIDE_METADATA[suffix]:
            if field not in head:
                problems.append(
                    f"{path.relative_to(ROOT)}: missing guide metadata {field}"
                )


def check_guide_code_block_parity(problems: list[str]) -> None:
    guides = ROOT / "docs" / "guides"
    for english in guides.iterdir():
        if (
            not english.is_file()
            or english.name.endswith(".ko.md")
        ):
            continue
        korean = english.with_name(f"{english.stem}.ko.md")
        if not korean.is_file():
            continue
        english_blocks = sum(
            line.strip().startswith("```")
            for line in english.read_text(encoding="utf-8").splitlines()
        )
        korean_blocks = sum(
            line.strip().startswith("```")
            for line in korean.read_text(encoding="utf-8").splitlines()
        )
        if english_blocks != korean_blocks:
            problems.append(
                f"{english.relative_to(ROOT)} and {korean.relative_to(ROOT)}: "
                f"code-block markers differ ({english_blocks}/{korean_blocks})"
            )


def check_example_template_loop(problems: list[str]) -> None:
    loops = ("while value < 3:", "동안 값 < 3:")
    for path in (
        ROOT / "docs" / "guides" / "example-template.md",
        ROOT / "docs" / "guides" / "example-template.ko.md",
    ):
        text = path.read_text(encoding="utf-8")
        for loop in loops:
            if loop not in text:
                problems.append(
                    f"{path.relative_to(ROOT)}: beginner skeleton needs bounded loop {loop}"
                )


problems: list[str] = []
check_local_markdown_links(problems)
check_korean_links(problems)
check_guide_navigation(problems)
check_numbered_guide_pairs(problems)
check_guide_metadata(problems)
check_guide_code_block_parity(problems)
check_example_template_loop(problems)
if problems:
    print(f"doc-parity: {len(problems)} problem(s)", file=sys.stderr)
    for problem in problems:
        print(f"doc-parity: {problem}", file=sys.stderr)
    raise SystemExit(1)
print("doc-parity: ok")
