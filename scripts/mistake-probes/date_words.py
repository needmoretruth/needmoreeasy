# -*- coding: utf-8 -*-
"""Ordinary sentences made of the words the date module takes, three ways each.

    python3 date_words.py [path-to-nme]

`use date` / `날짜 사용` binds `today`, `now`, `year`, `month`, `weekday`,
`오늘`, `지금`, `올해`, `이번달`, `요일` and their friends. Every one of those
is a word people write in ordinary sentences, and a bound name is a name the
compiler can see on later lines. So the question here is not "does the module
work" — it is "did opening the module change what an ordinary sentence means".

Each of the 310 sentences is compiled three ways:

    alone             the sentence on its own
    after the module  a program that already opened the date module
    beside a name     a program where the writer keeps their own `date`/`날짜`

**The gate** is the middle one, and it allows nothing: a sentence must compile
to exactly the same Python whether or not the date module has been opened. A
single difference fails the run.

The other two are ratchets. `alone` records how much of the corpus prints
itself character for character, and `beside a name` records how much survives
the writer having taken the word as a variable — where NME interpolates the
value on purpose, so `날짜를 잘못 적었습니다` becoming
`print(str(날짜) + "를 잘못 적었습니다")` is the documented behaviour and not a
defect. Both fail when the numbers get worse.

Measured 2026-08-19, on the round that added the date module:

    alone             printed 306 · refused 0 · changed 4
    beside a name     printed 222 · refused 3 · changed 85
    after the module  identical to `alone` for all 310

The four sentences that do not print themselves alone are all readings NME
already took before the date module existed; none of them changed on this
round. `What is the date today?` is the same shape as `지금 몇 시예요?` in
`korean_prose_corpus.py` — a line ending in a question mark asks something.
"""
import os
import subprocess
import sys
import tempfile

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from date_words_corpus import ENGLISH, KOREAN  # noqa: E402

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
NME = sys.argv[1] if len(sys.argv) > 1 else os.path.join(ROOT, "target", "release", "nme")

# The three programs each sentence is dropped into. `{}` is the sentence.
CONTEXTS = {
    "en": (
        ("alone", "{}\n", 0),
        ("after the module", "use date\n{}\n", 1),
        ("beside a name", "set date to 5\n{}\n", 1),
    ),
    "ko": (
        ("alone", "{}\n", 0),
        ("after the module", "날짜 사용\n{}\n", 1),
        ("beside a name", "날짜는 5\n{}\n", 1),
    ),
}

# What was true before the date module landed. The run fails if it gets worse.
RATCHET = {"alone": (0, 4), "beside a name": (3, 85)}  # (refused, changed)


def compile_source(folder, source):
    nme_path = os.path.join(folder, "probe.nme")
    py_path = os.path.join(folder, "probe.py")
    with open(nme_path, "w", encoding="utf-8") as fh:
        fh.write(source)
    if os.path.exists(py_path):
        os.remove(py_path)
    result = subprocess.run(
        [NME, "build", nme_path, "-o", py_path], capture_output=True, text=True
    )
    if result.returncode != 0:
        first = (result.stdout + result.stderr).strip().splitlines()
        return "거절: " + (first[0] if first else "출력 없음")
    with open(py_path, encoding="utf-8") as fh:
        produced = fh.read().splitlines()
    return produced


def outcome(folder, shape, at, sentence):
    produced = compile_source(folder, shape.format(sentence))
    if isinstance(produced, str):
        return produced
    return produced[at] if at < len(produced) else "<빈 줄>"


def main():
    if not os.path.exists(NME):
        raise SystemExit(f"date-words: no compiler at {NME}")
    counts = {}
    notes = {}
    changed_by_the_module = []
    with tempfile.TemporaryDirectory() as folder:
        for language, sentences in (("en", ENGLISH), ("ko", KOREAN)):
            for sentence in sentences:
                wanted = 'print("%s")' % sentence
                alone = None
                for label, shape, at in CONTEXTS[language]:
                    produced = outcome(folder, shape, at, sentence)
                    if label == "alone":
                        alone = produced
                    elif label == "after the module" and produced != alone:
                        changed_by_the_module.append(
                            f"  [{language}] {sentence}\n"
                            f"      alone           : {alone}\n"
                            f"      after the module: {produced}"
                        )
                    kind = (
                        "printed" if produced == wanted
                        else "refused" if produced.startswith("거절")
                        else "changed"
                    )
                    counts[(label, kind)] = counts.get((label, kind), 0) + 1
                    if label != "after the module" and kind != "printed":
                        notes.setdefault(label, []).append(
                            f"  [{language}] {sentence}\n      -> {produced}"
                        )

    total = len(ENGLISH) + len(KOREAN)
    print(f"날짜 낱말 {total}문장 × 3가지 = {total * 3}회")
    for label in ("alone", "beside a name", "after the module"):
        line = "  ".join(
            f"{kind} {counts.get((label, kind), 0)}"
            for kind in ("printed", "refused", "changed")
        )
        print(f"  {label:<17} {line}")

    problems = []
    if changed_by_the_module:
        print("\n모듈을 연 것만으로 뜻이 바뀐 문장:")
        for note in changed_by_the_module:
            print(note)
        problems.append(f"{len(changed_by_the_module)} sentence(s) changed meaning after `use date`")

    for label, (refused, changed) in RATCHET.items():
        now_refused = counts.get((label, "refused"), 0)
        now_changed = counts.get((label, "changed"), 0)
        if now_refused > refused or now_changed > changed:
            problems.append(
                f"{label}: refused {now_refused} (was {refused}), "
                f"changed {now_changed} (was {changed})"
            )
        for note in notes.get(label, []):
            print(note)

    if problems:
        for problem in problems:
            print(problem, file=sys.stderr)
        raise SystemExit(f"date-words: {len(problems)} problem(s)")
    print("date-words: ok")


if __name__ == "__main__":
    main()
