# -*- coding: utf-8 -*-
"""How much ordinary Korean survives the compiler unchanged.

    python3 korean_prose.py [path-to-nme]

`prose_corpus.py` is a gate: thirty sentences, half of them Korean, all of
which must print. This is the Korean counterpart of `english_prose.py` and it
is a ratchet rather than a gate: it records what was true on the day it was
written and fails when the numbers get worse.

Three outcomes, and only the third is a defect:

    printed   the Python is print("<the sentence>"), character for character
    refused   the compiler said no — a bad minute for the writer
    changed   it compiled into something else — a bad afternoon

Measured 2026-08-19 against build md5 49e6015b…f847, before that day's round:

    printed 294 (83.3%) · refused 21 · changed 38   (353 sentences)

and again the same day, after it:

    printed 350 (99.2%) · refused 0 · changed 3

The 38 fell into six causes. Every one of them came from the same shape: a
matcher claimed the line before anything asked whether the line was a written
Korean sentence.

    16  an assignment nobody wrote      `이유는 저도 잘 모릅니다`
                                        -> `이유 = "저도 잘 모릅니다"`
                                        runs, prints nothing, says nothing
    10  a word eaten out of the line    `물 좀 주세요` -> `print("물")`
                                        prints, but not what was written
     6  the program waits for typing    `물어봐 주셔서 감사합니다`
                                        -> `주셔서 = input("감사합니다" + " ")`
     3  a comparison nobody wrote       `3층에서 내리면 됩니다`
                                        -> `if (3 == "층에서 내리"): ...`
     2  a loop nobody wrote             `1번 출구에서 만납시다`
                                        -> `for _ in range(1): ...`
     1  arithmetic nobody wrote         `카드를 잘 섞어 나눠 주세요`
                                        -> `카드 = 카드 / 주세요`

Three things are left, and each is a reading somebody could defend:

    지금 몇 시예요?             `지금 = int(input(...))`. It is the same shape as
                              `나이가 몇 살이에요?`, which is guide 03.
    print 함수를 처음 배웠습니다  a line beginning with an output word prints what
    show me 라고 적혀 있었습니다  follows it. That is guide 01, and the English
                              corpus counts the same nine cases against itself.
"""
import os
import subprocess
import sys
import tempfile

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from korean_prose_corpus import GROUPS, PROSE  # noqa: E402

NME = sys.argv[1] if len(sys.argv) > 1 else "/home/user/nmt/needmoreeasy/target/release/nme"

# The numbers this file refuses to let get worse.
BASELINE_PRINTED = 350
BASELINE_CHANGED = 3


def outcome(sentence):
    """('printed'|'refused'|'changed', what happened)"""
    with tempfile.TemporaryDirectory() as folder:
        nme = os.path.join(folder, "p.nme")
        python = os.path.join(folder, "p.py")
        with open(nme, "w", encoding="utf-8") as handle:
            handle.write(sentence + "\n")
        run = subprocess.run(
            [NME, "build", nme, "-o", python],
            capture_output=True, text=True, stdin=subprocess.DEVNULL,
        )
        if run.returncode != 0:
            codes = [line for line in run.stdout.splitlines() if line.startswith("error[")]
            return "refused", (codes[0] if codes else run.stdout.strip()[:60])
        with open(python, encoding="utf-8") as handle:
            produced = handle.read().strip()
    # `print("...")` holding the sentence character for character.
    quoted = sentence.replace("\\", "\\\\").replace('"', '\\"')
    if produced == 'print("%s")' % quoted:
        return "printed", produced
    return "changed", produced


counts = {"printed": 0, "refused": 0, "changed": 0}
changed = []
for group_name, sentences in GROUPS:
    for sentence in sentences:
        kind, detail = outcome(sentence)
        counts[kind] += 1
        if kind == "changed":
            changed.append((group_name, sentence, detail))

print(
    "한국어 산문 %d문장 — 그대로 출력 %d · 거절 %d · 다른 프로그램 %d"
    % (len(PROSE), counts["printed"], counts["refused"], counts["changed"])
)
for group_name, sentence, detail in changed:
    print("  [%s] %s\n      -> %s" % (group_name, sentence, detail[:90]))

worse = []
if counts["printed"] < BASELINE_PRINTED:
    worse.append("printed %d < %d" % (counts["printed"], BASELINE_PRINTED))
if counts["changed"] > BASELINE_CHANGED:
    worse.append("changed %d > %d" % (counts["changed"], BASELINE_CHANGED))
if worse:
    print("korean-prose: 나빠졌습니다 — " + ", ".join(worse))
    sys.exit(1)
if counts["printed"] > BASELINE_PRINTED or counts["changed"] < BASELINE_CHANGED:
    print(
        "korean-prose: 좋아졌습니다. BASELINE_PRINTED=%d, BASELINE_CHANGED=%d 로 올려 두세요."
        % (counts["printed"], counts["changed"])
    )
print("korean-prose: ok")
