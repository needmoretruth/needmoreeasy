# -*- coding: utf-8 -*-
"""How much ordinary English survives the compiler unchanged.

    python3 english_prose.py [path-to-nme]

`prose_corpus.py` is a gate: thirty sentences, all of which must print, and it
is 30/30. This is its English counterpart and it is **not** a gate, because on
the day it was written only 61% of it printed. It is a ratchet: it records what
was true then and fails when the numbers get worse.

Three outcomes, and only the third is a defect:

    printed   the Python is print("<the sentence>"), word for word
    refused   the compiler said no — a bad minute for the writer
    changed   it compiled into something else — a bad afternoon

Baseline, measured 2026-08-18 against one build:

    printed 184 · refused 74 · changed 44   (302 sentences)

and again on 2026-08-19, after the three causes below were closed:

    printed 249 · refused 41 · changed 12   (302 sentences)

Nine of the remaining 12 are the language working as designed: a line
beginning with `show`/`say`/`tell` prints what follows it, which is guide 01,
and `show Hello world` must not print the word `show`. They are counted here
anyway, because the honest number is the number a reader would get — and
because warning on them is worth doing one day.

The three causes closed on 2026-08-19 were a typo-tolerant match firing on
ordinary words (`day`→`say`, `shop`→`show`, `road`→`load`), a digit switching
the prose path off, and a one-word line becoming a bare name that raised
`NameError`. Three of the 12 that are left are:

    Well done                      `Well` is one letter from `tell`, and one
                                   word of message is exactly what a real
                                   misspelling looks like — the same shape as
                                   `shoe hello`, which must keep working.
    There is nothing left to say.  `Hello world show` is a documented
                                   spelling: an output word written exactly,
                                   after the message, prints the message.
    ask Mum about the recipe       the same shape as `ask name What is your
                                   name?`, which is guide 03.
"""
import os
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from english_prose_corpus import GROUPS, PROSE  # noqa: E402

def newest_nme():
    """The compiler binary that was built last, release or debug.

    Defaulting to `target/release/nme` meant a debug build of the change under
    test was measured against the previous release binary; every number came
    back green because nothing under test was being run.
    """
    root = Path("/home/user/nmt/needmoreeasy")
    found = sorted(
        (root / "target/release/nme", root / "target/debug/nme"),
        key=lambda path: -path.stat().st_mtime if path.is_file() else 0,
    )
    return str(found[0])


NME = sys.argv[1] if len(sys.argv) > 1 else newest_nme()

# The numbers this file refuses to let get worse.
BASELINE_PRINTED = 254
BASELINE_CHANGED = 10


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
    "영어 산문 %d문장 — 그대로 출력 %d · 거절 %d · 다른 프로그램 %d"
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
    print("english-prose: 나빠졌습니다 — " + ", ".join(worse))
    sys.exit(1)
if counts["printed"] > BASELINE_PRINTED or counts["changed"] < BASELINE_CHANGED:
    print(
        "english-prose: 좋아졌습니다. BASELINE_PRINTED=%d, BASELINE_CHANGED=%d 로 올려 두세요."
        % (counts["printed"], counts["changed"])
    )
print("english-prose: ok")
