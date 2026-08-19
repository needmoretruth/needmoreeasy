#!/usr/bin/env python3
"""The version is `0.MINOR.PATCH`, it moves one step, and it never reaches 1.

    python scripts/check-version.py

The owner set this policy on 2026-08-19:

> 0.0.1 beta가 아니라 이제부터는 마이너와 패치를 버전업 해도 좋아. 무슨 버전
> 올릴지는 너가 정하고 버전업은 반드시 배포할 때만 올려. 그리고 1.0.0 정식출시는
> 절대 하지마.

So three rules, and this checks all three:

1. **The shape is `0.MINOR.PATCH`.** No `-beta.N` counter any more — the leading
   zero already says the language is still moving.
2. **The major number stays 0.** `1.0.0` is refused outright, and so is anything
   above it. This is not a soft convention: a release that says 1 promises that
   programs written today keep working, and the owner has said the opposite —
   *이전 코드와 호환 안되도 상관없으니까*.
3. **One step at a time.** From `0.a.b` the only next versions are `0.a.(b+1)`
   and `0.(a+1).0`.

It also checks that everything which quotes the version agrees with
`Cargo.toml`, and that the changelogs have a heading for it. Bumping is done by
`scripts/bump-version.py`, which the deploy script calls; nothing else should
edit these numbers by hand.
"""

from __future__ import annotations

import re
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
VERSION = re.compile(r"^(\d+)\.(\d+)\.(\d+)$")
ANY_VERSION = re.compile(r"\b0\.\d+\.\d+\b")

# Every document that tells a reader which version they should have. The
# prompts are generated, so they are checked but never edited here.
NAMES_VERSION = (
    "README.md", "README.ko.md",
    "docs/install.md", "docs/install.ko.md",
    "docs/ai-assistants.md", "docs/ai-assistants.ko.md",
    "docs/prompts/nme-sentence.md", "docs/prompts/nme-sentence.ko.md",
    "docs/prompts/nme-all-levels.md", "docs/prompts/nme-all-levels.ko.md",
    "docs/prompts/nme-complete.md", "docs/prompts/nme-complete.ko.md",
)

# Documents that talk *about* versions without telling anyone which one to
# install. They still may not be left holding the last release's number.
MENTIONS_VERSION = ("docs/versioning.md", "docs/versioning.ko.md")

CHANGELOGS = ("CHANGELOG.md", "CHANGELOG.ko.md")


def fail(message: str) -> None:
    print(f"check-version: {message}", file=sys.stderr)
    raise SystemExit(1)


def workspace_version(text: str) -> str:
    try:
        return tomllib.loads(text)["workspace"]["package"]["version"]
    except KeyError:
        fail("Cargo.toml has no workspace.package.version")
        raise


def parts(version: str, what: str) -> tuple[int, int, int]:
    found = VERSION.fullmatch(version)
    if not found:
        fail(f"{what} is not `0.MINOR.PATCH`: {version}")
        raise
    major, minor, patch = (int(g) for g in found.groups())
    if major != 0:
        fail(
            f"{what} is {version}: the major number must stay 0. "
            "1.0.0 promises that today's programs keep working, and the owner "
            "has said the opposite while the language is still moving."
        )
    return major, minor, patch


def main() -> None:
    current = workspace_version((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
    _, minor, patch = parts(current, "the version")

    # The step, when there is a previous commit to compare with.
    try:
        before = workspace_version(
            subprocess.check_output(["git", "show", "HEAD^:Cargo.toml"], cwd=ROOT, text=True)
        )
    except subprocess.CalledProcessError:
        before = None
    if before is not None and before != current:
        old = VERSION.fullmatch(before)
        if old:
            _, was_minor, was_patch = (int(g) for g in old.groups())
            allowed = {(was_minor, was_patch + 1), (was_minor + 1, 0)}
            if (minor, patch) not in allowed:
                fail(
                    f"the version jumped: {before} -> {current}. "
                    f"Only 0.{was_minor}.{was_patch + 1} and 0.{was_minor + 1}.0 follow."
                )

    problems: list[str] = []
    # Bundled modules carry their own versions (`use random 0.0.1`), so a
    # document is not asked to hold one version number — it is asked to hold
    # *this* one, and not to be left holding the one before it.
    for name in NAMES_VERSION + MENTIONS_VERSION:
        path = ROOT / name
        if not path.is_file():
            problems.append(f"missing: {name}")
            continue
        text = path.read_text(encoding="utf-8")
        if name in NAMES_VERSION and current not in ANY_VERSION.findall(text):
            problems.append(f"{name} never names the current version {current}")
        if before is not None and before != current and before in text:
            problems.append(f"{name} still names the last release {before}")

    for name in CHANGELOGS:
        text = (ROOT / name).read_text(encoding="utf-8")
        if f"## {current}" not in text:
            problems.append(f"{name} needs a `## {current}` heading")

    lock = tomllib.loads((ROOT / "Cargo.lock").read_text(encoding="utf-8"))
    wanted = {"nme-cli", "nme-core", "nme-native"}
    seen = set()
    for package in lock.get("package", []):
        if package.get("name") in wanted:
            seen.add(package["name"])
            if package.get("version") != current:
                problems.append(
                    f"Cargo.lock has {package['name']} at {package.get('version')}, "
                    f"expected {current}"
                )
    if seen != wanted:
        problems.append(f"Cargo.lock is missing {', '.join(sorted(wanted - seen))}")

    for problem in problems:
        print(f"check-version: {problem}", file=sys.stderr)
    if problems:
        raise SystemExit(1)
    print(f"check-version: {current} · 자리 맞음 · 1.0.0 아님 · 문서와 일치")


if __name__ == "__main__":
    main()
