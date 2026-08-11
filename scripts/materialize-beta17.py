#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path

OLD = "0.0.1-beta.16"
NEW = "0.0.1-beta.17"

version_files = [
    Path("Cargo.toml"),
    Path("Cargo.lock"),
    Path("README.md"),
    Path("README.ko.md"),
    Path("docs/install.md"),
    Path("docs/install.ko.md"),
    Path("docs/versioning.md"),
    Path("docs/versioning.ko.md"),
    Path("crates/nme-cli/tests/cli.rs"),
]

changed: list[str] = []
for path in version_files:
    text = path.read_text(encoding="utf-8")
    if OLD in text:
        path.write_text(text.replace(OLD, NEW), encoding="utf-8")
        changed.append(str(path))

required = {"Cargo.toml", "Cargo.lock", "crates/nme-cli/tests/cli.rs"}
assert required <= set(changed), changed

Path("scripts/check-beta-step.py").write_text(
    r'''#!/usr/bin/env python3
from __future__ import annotations

import re
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
VERSION_RE = re.compile(r"^(?P<base>.+-beta\.)(?P<number>\d+)$")


def fail(message: str) -> None:
    print(f"beta-step: {message}", file=sys.stderr)
    raise SystemExit(1)


def version(text: str) -> str:
    try:
        return tomllib.loads(text)["workspace"]["package"]["version"]
    except KeyError as error:
        fail(f"Cargo.toml has no workspace.package.version: {error}")


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


current = version((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
current_match = VERSION_RE.fullmatch(current)
if not current_match:
    fail(f"current version is not a beta version: {current}")
try:
    parent = version(git("show", "HEAD^:Cargo.toml"))
except subprocess.CalledProcessError:
    fail("beta commit needs a first parent with Cargo.toml")
parent_match = VERSION_RE.fullmatch(parent)
if not parent_match:
    fail(f"parent version is not a beta version: {parent}")
if current_match.group("base") != parent_match.group("base"):
    fail(f"beta base changed unexpectedly: {parent} -> {current}")
if int(current_match.group("number")) != int(parent_match.group("number")) + 1:
    fail(f"beta must advance exactly one step: {parent} -> {current}")
subject = git("log", "-1", "--format=%s")
if current not in subject:
    fail(f"commit subject must contain {current!r}: {subject!r}")
lock = tomllib.loads((ROOT / "Cargo.lock").read_text(encoding="utf-8"))
expected = {"nme-cli", "nme-core", "nme-native"}
seen: set[str] = set()
for package in lock.get("package", []):
    if package.get("name") in expected:
        seen.add(package["name"])
        if package.get("version") != current:
            fail(f"Cargo.lock {package['name']} is {package.get('version')}, expected {current}")
if seen != expected:
    fail(f"Cargo.lock workspace packages mismatch: {sorted(seen)}")
print(f"beta-step: ok {parent} -> {current}")
''',
    encoding="utf-8",
)

en = Path("CHANGELOG.md")
text = en.read_text(encoding="utf-8")
marker = "## Unreleased\n"
entry = """
## 0.0.1-beta.17 — 2026-08-12

- Make `beta` the enforced next-generation release line. Every public beta push must advance the workspace beta number by exactly one, name that version in the commit subject, and keep the workspace package versions in `Cargo.lock` synchronized.
- Upgrade CI to `actions/checkout@v6` and `actions/setup-python@v6`, and run Cargo checks and tests with `--locked`.
- Add CPython 3.10, 3.12, and 3.14 compatibility jobs for beta and pull requests while retaining the full Ubuntu, Windows, and macOS quality gate.
"""
assert marker in text and "## 0.0.1-beta.17" not in text
en.write_text(text.replace(marker, marker + entry, 1), encoding="utf-8")

ko = Path("CHANGELOG.ko.md")
text = ko.read_text(encoding="utf-8")
marker = "## 미출시 (Unreleased)\n"
entry = """
## 0.0.1-beta.17 — 2026-08-12

- `beta`를 규칙으로 강제되는 차세대 릴리스 선으로 만들었습니다. 공개 beta push마다 workspace beta 번호가 부모보다 정확히 1 증가하고, 커밋 제목과 `Cargo.lock`의 workspace 패키지 버전이 모두 일치해야 합니다.
- CI를 `actions/checkout@v6`, `actions/setup-python@v6`로 올리고 Cargo 검사와 테스트에 `--locked`를 적용했습니다.
- beta와 PR에 CPython 3.10, 3.12, 3.14 호환성 검증을 추가하면서 Ubuntu, Windows, macOS 전체 품질 게이트도 유지합니다.
"""
assert marker in text and "## 0.0.1-beta.17" not in text
ko.write_text(text.replace(marker, marker + entry, 1), encoding="utf-8")

Path("docs/release-beta.17.md").write_text(
    """# NME 0.0.1-beta.17

`beta` is the repository's next-generation release line.

## Public beta invariant

Every commit on public `beta` must advance `beta.N` to exactly `beta.N+1`. The release commit subject, workspace manifest, and the three workspace package entries in `Cargo.lock` must agree. The CI guard checks the real first parent with a two-commit checkout.

## Compatibility gate

The release gate runs format, locked check, Clippy with warnings denied, locked full workspace tests, CLI installation, and smoke tests on Ubuntu, Windows, and macOS. Beta and pull requests also test with CPython 3.10, 3.12, and 3.14.
""",
    encoding="utf-8",
)

print("materialized", NEW, "files:", ", ".join(changed))
