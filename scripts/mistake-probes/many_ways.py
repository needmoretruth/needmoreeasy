#!/usr/bin/env python3
"""How many of the ways a beginner writes a thing actually do that thing.

    python scripts/mistake-probes/many_ways.py            # the count
    python scripts/mistake-probes/many_ways.py --all      # every variant

The owner watched a beginner use NME on 2026-08-19 and reported the problem in
one sentence: *무언가 문법을 사용할 때 순서를 바꾸거나 앞에 두거나 다른 문장
연결어를 쓰든 뭘 하든 제대로 작동해야 해. 지금 코드는 잘 받아주는 척하면서
정작 「왜 이게 작동 안 하지?」 싶은 게 너무 많아.*

Three things can go wrong, and they are not equally bad:

- **🤐 the line came back as its own text.** The writer used a command word and
  got a sentence. Nothing says anything is wrong. This is the failure the
  complaint is about, and the number that matters most.
- **🟡 refused.** The writer is told, which is far better, but the variant is
  one a person reasonably reaches for and should work.
- **❌ something else happened.** Rarest and worst when it happens.

This is a ratchet, like the prose corpora: the recorded numbers may improve and
may not get worse. It is not a gate — which spellings are worth accepting is an
editorial decision, and widening one costs prose safety somewhere else.
"""

from __future__ import annotations

import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from many_ways_corpus import GROUPS  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]


def nme_candidates(root):
    """The compiler binaries, newest build first.

    These used to be listed release-first, which meant a debug build of the
    change under test was checked against the previous release binary. Every
    number came back green because nothing under test was being run.
    """
    return sorted(
        (root / "target/release/nme", root / "target/debug/nme"),
        key=lambda path: -path.stat().st_mtime if path.is_file() else 0,
    )
BINARY = next(
    (c for c in nme_candidates(ROOT) if c.is_file()),
    None,
)

# Recorded on 2026-08-20 against the 211-variant corpus. Raise these when the
# compiler gets better; never lower them to make a run pass.
#
# 68 · 11 was the first reading, on the morning of 2026-08-19, over the first
# 103 variants. The round that followed drove the silent count to zero, which
# was the whole point: a writer who used a command word now either gets what
# they meant or is told what to write instead. 2026-08-20 doubled the corpus —
# fourteen more intentions, so the number measures the language rather than
# its opening lesson — and the first reading over all 211 was 163 · 5.
# The eight that do not work, and why each is left alone (2026-08-20):
#
#   name is 5                  E0604 — in Python that line compares. Reading it
#                              as a save would change what valid Python means.
#   set Mina to 90 in ages     already means *read `Mina` out of `ages`*, which
#                              `records_and_jobs.rs` pins. One spelling, one
#                              meaning.
#   raise score by 1           `raise` is a Python keyword and the line is
#                              handed to CPython, which answers it.
#   if not / in every other    four words or an opening `if`: both readings are
#   case                       live, and NME never picks one of two.
#   go through names as name   names the list first and the loop name last,
#                              which is the opposite order to every other
#                              header. A shape, not a word, and not written yet.
#   수가 3보다 작은 한 / 작으면 계속  `한` and `계속` are among the commonest words in
#                              Korean; claiming them would cost more prose than
#                              the two spellings are worth.
FLOOR_WORKS = 203
CEILING_SILENT = 0


def build(source: str) -> tuple[str, str]:
    with tempfile.TemporaryDirectory() as folder:
        nme = Path(folder) / "a.nme"
        nme.write_text(source + "\n", encoding="utf-8")
        result = subprocess.run(
            [str(BINARY), "build", str(nme), "-o", str(Path(folder) / "a.py")],
            capture_output=True, text=True, stdin=subprocess.DEVNULL,
        )
        if result.returncode != 0:
            message = result.stdout + result.stderr
            code = next(
                (l.split("error[")[1].split("]")[0] for l in message.splitlines()
                 if "error[" in l), "?")
            return "refused", code
        return "built", (Path(folder) / "a.py").read_text(encoding="utf-8")


def main() -> None:
    if BINARY is None:
        raise SystemExit("many_ways: no nme binary found")
    show_all = "--all" in sys.argv
    works = silent = refused = other = 0
    for group in GROUPS:
        if show_all:
            print(f"\n── {group['name']}   ({group['want']})")
        for way in group["ways"]:
            kind, out = build(group["setup"] + way + group["body"])
            if kind == "refused":
                refused += 1
                mark, shown = "🟡", f"거절 {out}"
            else:
                produced = out.splitlines()[len(group["setup"].splitlines()):]
                line = produced[0] if produced else ""
                if group["want"] in line:
                    works += 1
                    mark, shown = "✅", line
                elif line.startswith('print("') and way.split()[0] in line:
                    silent += 1
                    mark, shown = "🤐", line
                else:
                    other += 1
                    mark, shown = "❌", line
            if show_all:
                print(f"  {mark} {way:38s} → {shown[:62]}")

    total = sum(len(g["ways"]) for g in GROUPS)
    print(f"\n여러 갈래 표기 {total}개 — 뜻대로 {works} · 그대로 출력 {silent} "
          f"· 거절 {refused} · 다른 것 {other}")
    problems = []
    if works < FLOOR_WORKS:
        problems.append(f"뜻대로 되는 것이 {FLOOR_WORKS}개 아래로 내려갔습니다: {works}")
    if silent > CEILING_SILENT:
        problems.append(f"그대로 출력되는 것이 {CEILING_SILENT}개를 넘었습니다: {silent}")
    for problem in problems:
        print(f"many_ways: {problem}", file=sys.stderr)
    if problems:
        raise SystemExit(1)
    if works > FLOOR_WORKS or silent < CEILING_SILENT:
        print(f"many_ways: 기록을 갱신했습니다 — FLOOR_WORKS={works}, "
              f"CEILING_SILENT={silent} 로 올려 두세요")


if __name__ == "__main__":
    main()
