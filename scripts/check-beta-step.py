#!/usr/bin/env python3
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
