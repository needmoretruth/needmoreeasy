# Mistake probes — how forgiving is the compiler?

723 short programs, in English and Korean: mostly things a beginner would
plausibly write **wrongly** — keyword typos, a different word order, missing or
extra spaces, a full stop at the end, Korean particles and endings, number
words, capitals, and the synonyms someone guesses before they have read
anything — beside the correct spellings of each grammar form, so that a
capability cannot quietly stop working either.

The number that matters is the last one `probe.py` prints: **how many were
accepted and compiled to something the writer plainly did not mean.** It is 11,
and it must not rise.

There are two corpora of ordinary sentences beside the mistakes:

| file | what it is |
| --- | --- |
| `prose_corpus.py` | 30 ordinary sentences, half English half Korean, that **must** print. A gate: 30/30 today, and it stays there. |
| `english_prose.py` + `english_prose_corpus.py` | 302 ordinary **English** sentences. A ratchet, not a gate — only 61% printed on the day it was written, so it records the three numbers and fails when they get worse. |

    python3 scripts/mistake-probes/english_prose.py
    영어 산문 302문장 — 그대로 출력 184 · 거절 74 · 다른 프로그램 44

Korean prose is in far better shape than English prose, and the gap is the
compiler's, not the corpus's: a typo-tolerant match fires on ordinary English
words (`day` reads as `say`, `shop` as `show`), a digit anywhere switches the
prose path off, and a one-word line becomes a bare name that raises `NameError`.

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

Accepting more and refusing loudly pull in opposite directions, which is why
both numbers are here: the second round gave back 35 acceptances to remove 35
silent mis-compiles, and that was the right trade.

### Adding a probe

Append to `probes3.py`: `(id, family, lang, source, intent, expect_regex)`.
`expect_regex` is searched against the generated Python; `None` means "I only
care whether it was accepted". Keep each probe to one mistake, so a failure
names one thing.
