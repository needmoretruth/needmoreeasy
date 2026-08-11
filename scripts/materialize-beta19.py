#!/usr/bin/env python3
from pathlib import Path

old = "0.0.1-beta.18"
new = "0.0.1-beta.19"

for name in [
    "Cargo.toml",
    "Cargo.lock",
    "README.md",
    "README.ko.md",
    "docs/install.md",
    "docs/install.ko.md",
    "docs/versioning.md",
    "docs/versioning.ko.md",
    "crates/nme-cli/tests/cli.rs",
]:
    path = Path(name)
    text = path.read_text(encoding="utf-8")
    if old in text:
        path.write_text(text.replace(old, new), encoding="utf-8")

for path_name, marker, entry in [
    (
        "CHANGELOG.md",
        "## Unreleased\n",
        """\n## 0.0.1-beta.19 — 2026-08-12\n\n- Converge the public beta Git topology with `main`: the final beta.19 release commit keeps beta.18 as its first parent and records the current main tip as its second parent. The beta first-parent release line still advances exactly one version per public commit, while `main` becomes an actual ancestor of the next-generation `beta` branch.\n- Keep the beta.17 release guard, locked Cargo validation, three-OS gate, and CPython 3.10/3.12/3.14 compatibility matrix unchanged.\n""",
    ),
    (
        "CHANGELOG.ko.md",
        "## 미출시 (Unreleased)\n",
        """\n## 0.0.1-beta.19 — 2026-08-12\n\n- 공개 beta의 Git 토폴로지를 `main`과 수렴시킵니다. 최종 beta.19 릴리스 커밋은 beta.18을 첫 부모로 유지하고 현재 main tip을 둘째 부모로 기록합니다. 따라서 beta의 first-parent 릴리스 선은 공개 커밋마다 버전이 정확히 1씩 증가하면서도, `main`이 차세대 `beta`의 실제 조상이 됩니다.\n- beta.17에서 추가한 버전 증가 가드, locked Cargo 검증, 3개 OS 게이트, CPython 3.10/3.12/3.14 호환성 매트릭스는 그대로 유지합니다.\n""",
    ),
]:
    path = Path(path_name)
    text = path.read_text(encoding="utf-8")
    assert marker in text and "## 0.0.1-beta.19" not in text
    path.write_text(text.replace(marker, marker + entry, 1), encoding="utf-8")

Path("docs/release-beta.19.md").write_text(
    """# NME 0.0.1-beta.19\n\nBeta.19 makes the repository topology match the operating model: `beta` is the next-generation branch and `main` is an actual ancestor of it.\n\nThe final public release is intentionally a two-parent commit:\n\n1. first parent: public beta.18, preserving the beta first-parent release line;\n2. second parent: the current `main` tip, making main part of beta's ancestry without modifying main.\n\nThe release tree is validated before that merge commit is created. The public beta-version guard still checks the first parent, so beta.18 -> beta.19 remains an exact +1 transition.\n\nNo feature from beta.18 is removed; this release is a topology and release-engineering convergence point for future beta-first development.\n""",
    encoding="utf-8",
)

print("materialized", new)
