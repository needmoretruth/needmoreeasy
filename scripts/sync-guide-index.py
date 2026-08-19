#!/usr/bin/env python3
"""Rewrite the index from the guides, so the index cannot drift.

    python scripts/sync-guide-index.py

`check-guide-index.py` says when `docs/guides/index.md` and `index.ko.md`
disagree with the guides.  This writes the agreement instead of reporting it:
every guide's own five facts (number, difficulty, topic, title, result) are
copied into the table, into the "learn in order" list, and into the
browse-by-topic buckets.

The guide is the source; the index follows it.  Hand-editing the index was
what let thirteen rows drift and let the topic buckets keep saying `데이터`
for a guide whose own topic line had said `파일과 자료` for weeks.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GUIDES = ROOT / "docs/guides"

TOPIC_HEADING = {False: "## Topic lookup", True: "## 주제별 찾아보기"}
NEXT_HEADING = {False: "## All guides", True: "## 전체 목록"}


def facts(path: Path) -> dict[str, str]:
    text = path.read_text(encoding="utf-8")

    def field(*names: str) -> str:
        for name in names:
            found = re.search(rf"^- {name}:\s*(.+)$", text, re.M)
            if found:
                return found.group(1).strip()
        raise SystemExit(f"sync-guide-index: {path.name} has no {names[0]} line")

    title = re.search(r"^# (\d\d) — (.+)$", text, re.M)
    if not title:
        raise SystemExit(f"sync-guide-index: {path.name} has no `# NN — title` line")
    return {
        "number": title.group(1),
        "title": title.group(2).strip(),
        "stars": field("난이도", "Difficulty").split(" ")[0],
        "topic": field("주제", "Topic"),
        "result": field("결과물", "Result"),
    }


def sync(index_path: Path, korean: bool) -> int:
    lines = index_path.read_text(encoding="utf-8").split("\n")
    guides = {}
    for guide in sorted(GUIDES.glob("[0-9]*.md")):
        if guide.name.endswith(".ko.md") != korean:
            continue
        guides[guide.name[:2]] = (guide.name, facts(guide))

    changed = 0

    # 1. the table at the bottom.
    for at, line in enumerate(lines):
        if not re.match(r"^\| \d\d \|", line):
            continue
        number = line[2:4]
        if number not in guides:
            continue
        name, fact = guides[number]
        want = (
            f"| {number} | {fact['stars']} | {fact['topic']} | "
            f"[{fact['title']}]({name}) | {fact['result']} |"
        )
        if lines[at] != want:
            lines[at] = want
            changed += 1

    # 2. the numbered "learn in order" list.
    for at, line in enumerate(lines):
        found = re.match(r"^(\d\d)\. \[", line)
        if not found or found.group(1) not in guides:
            continue
        number = found.group(1)
        name, fact = guides[number]
        want = f"{number}. [{number} — {fact['title']}]({name})"
        if lines[at] != want:
            lines[at] = want
            changed += 1

    # 3. the browse-by-topic buckets, rebuilt whole.  Order follows the guide
    #    numbers, so a bucket appears where its first guide does.
    start = lines.index(TOPIC_HEADING[korean])
    stop = lines.index(NEXT_HEADING[korean])
    buckets: dict[str, list[str]] = {}
    for number in sorted(guides):
        name, fact = guides[number]
        buckets.setdefault(fact["topic"], []).append(f"[{number}]({name})")
    want = ["", *[f"- {topic}: {', '.join(rows)}" for topic, rows in buckets.items()], ""]
    if lines[start + 1 : stop] != want:
        lines[start + 1 : stop] = want
        changed += 1

    index_path.write_text("\n".join(lines), encoding="utf-8")
    print(f"sync-guide-index: {index_path.name} {changed} 곳 맞춤")
    return changed


def main() -> None:
    total = sum(
        sync(GUIDES / name, korean)
        for name, korean in (("index.md", False), ("index.ko.md", True))
    )
    if total == 0:
        print("sync-guide-index: 이미 맞습니다")


if __name__ == "__main__":
    main()
    sys.exit(0)
