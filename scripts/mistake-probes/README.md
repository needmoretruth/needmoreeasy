# Mistake probes — how forgiving is the compiler?

804 short programs, in English and Korean: mostly things a beginner would
plausibly write **wrongly** — keyword typos, a different word order, missing or
extra spaces, a full stop at the end, Korean particles and endings, number
words, capitals, and the synonyms someone guesses before they have read
anything — beside the correct spellings of each grammar form, so that a
capability cannot quietly stop working either.

The number that matters is the last one `probe.py` prints: **how many were
accepted and compiled to something the writer plainly did not mean.** It is 10,
and it must not rise.

The ten that remain are all deliberate. Four are dictionary words a typo repair
must not claim (`shoe hello`, `sett score to 0`); two are Korean output verbs
that only mean the verb at the end of a line, because `말하기 연습` is speaking
practice; three are a full stop kept as part of the text (`say hello.` prints
`hello.`); and one is a line beginning `then`, where telling it from *Then show
me the way* would cost more prose than the probe is worth.

There are two corpora of ordinary sentences beside the mistakes:

| file | what it is |
| --- | --- |
| `prose_corpus.py` | 30 ordinary sentences, half English half Korean, that **must** print. A gate: 30/30 today, and it stays there. |
| `english_prose.py` + `english_prose_corpus.py` | 302 ordinary **English** sentences. A ratchet, not a gate — only 61% printed on the day it was written, so it records the three numbers and fails when they get worse. |
| `korean_prose.py` + `korean_prose_corpus.py` | 353 ordinary **Korean** sentences. The same ratchet: 83% printed on the day it was written, and 38 of them compiled into a different program. |

    python3 scripts/mistake-probes/english_prose.py
    영어 산문 302문장 — 그대로 출력 249 · 거절 41 · 다른 프로그램 12

    python3 scripts/mistake-probes/korean_prose.py
    한국어 산문 353문장 — 그대로 출력 350 · 거절 0 · 다른 프로그램 3

English prose was the worse of the two first, and the gap was the compiler's,
not the corpus's: a typo-tolerant match fired on ordinary English words (`day`
read as `say`, `shop` as `show`, `road` as `load`), a digit anywhere switched
the prose path off, and a one-word line became a bare name that raised
`NameError`. All three were closed on 2026-08-19 and 61% became 82%. Nine of
the twelve that are left are a line beginning with `show`/`say`/`tell`, which
is the language working as designed.

Korean was measured the same day and was three times worse at the thing that
matters most: 38 of 353 sentences compiled into a different program, and
**every one** of the twenty-three polite `… 주세요` requests was wrong. One
shape was behind nearly all of it — a matcher claimed the line before anything
asked whether the line was a written Korean sentence — and closing it took
83% to 99%. The three that are left are each a reading somebody could defend;
`korean_prose.py` names them one by one.

This is a **measurement**, not a pass/fail gate. Run it, read the three numbers,
and decide whether the direction is right:

```sh
cargo build --release -p nme-cli --locked
python scripts/mistake-probes/probe.py           # writes results.tsv + results.json
python scripts/mistake-probes/analyse.py         # prints the totals
```

Three numbers come out, and the third is the one that matters:

| | meaning |
| --- | --- |
| accepted | the compiler read the line and produced Python |
| rejected | the compiler said no — with an error the writer can act on, one hopes |
| **mis-compiled** | **accepted, but the Python does something the writer plainly did not mean** |

A rejection is a bad minute. A mis-compile is a bad afternoon: the program runs,
prints the wrong thing or dies later with a `NameError` pointing at a line that
is not the mistake. Watch that number above the other two.

There is a second corpus beside this one, `prose_corpus.py`: thirty ordinary
sentences — a greeting, a line of a story, a message — that must print
themselves and nothing else. It is the other half of the same question. Making
the compiler stricter is easy if you are allowed to refuse prose; that file is
what stops you.

```sh
python scripts/mistake-probes/prose_corpus.py     # must say 30/30
python scripts/check-prose-blocks.py              # and no guide line may drift
```

### The record so far

| date | accepted | rejected | mis-compiled | prose |
| --- | --- | --- | --- | --- |
| 2026-08-18, start | 442 | 124 | 141 | — |
| 2026-08-18, after accepting more | 496 | 70 | 46 | 20/30 |
| 2026-08-18, after refusing loudly | 461 | 105 | 11 | 30/30 |
| 2026-08-19, after the English-prose round | 628 | 122 | 11 | 30/30 |
| 2026-08-19, after the Korean-prose round | 635 | 122 | 11 | 30/30 |
| 2026-08-19, after the modules-and-text round | 680 | 124 | 11 | 30/30 |
| 2026-08-20, after the many-ways round | 708 | 96 | 10 | 30/30 |
| 2026-08-20, reading the verb a beginner wrote | 727 | 77 | 10 | 30/30 |

The first three rows were measured over 566 probes, then 750, 757 and 804, so
read the mis-compile column across them and the other two only within a row.

Accepting more and refusing loudly pull in opposite directions, which is why
both numbers are here: the second round gave back 35 acceptances to remove 35
silent mis-compiles, and that was the right trade.

### Adding a probe

Append to `probes3.py`: `(id, family, lang, source, intent, expect_regex)`.
`expect_regex` is searched against the generated Python; `None` means "I only
care whether it was accepted". Keep each probe to one mistake, so a failure
names one thing.
