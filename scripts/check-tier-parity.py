#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Proves that the three syntax levels and the two languages can do the same things.

The owner's rule: *what is possible in sentence grammar, beginner grammar and
advanced grammar must not differ at all, and the same goes for the Korean
grammar and the English grammar.*

Advanced NME is ordinary Python, so "everything in Python" is not the bar. The
bar is that **every capability NME itself offers is reachable from every level
and from both natural languages**. This script proves that four ways, and every
one of them reads its facts out of the compiler rather than out of a list
somebody has to remember to update:

1. **Inventory** — the capability list comes from the sixteen `enum`s named in
   `INVENTORY_ENUMS`, read out of `crates/nme-core/src/syntax.rs`. A variant
   that no probe row claims to cover fails the build, so a new statement cannot
   be added without saying how it is written at each level in each language.
   A variant that is declared but that no parser code can reach is named in
   `UNWIRED` and printed on every run.
2. **Word-list symmetry** — every `X_WORDS_EN` list in the compiler must have an
   `X_WORDS_KO` twin and the other way round, so a spelling cannot be added to
   one language alone.
3. **Reachability** — every cell of the matrix is compiled with the real
   compiler. A cell that does not compile is a missing cell.
4. **Cross-language identity** — within one level the English cell and the
   Korean cell must produce **the same Python**, once Korean names are mapped
   to their English twins. This is the check that catches a Korean form that
   silently means something else, which is worse than a form that is missing.

On top of those four, `DIAGNOSTIC_PARITY` checks the refusals that are part of
a capability's contract: a capability is not at parity if one language gets a
clear error and the other quietly accepts the same mistake.

Cells that cannot be closed today live in `KNOWN_GAPS` with a one-line reason
each. Everything not listed there must pass, so a *new* gap still fails CI.

    python scripts/check-tier-parity.py [path/to/nme]

The compiler binary defaults to `target/release/nme`, then `target/debug/nme`.
Without one, only the inventory and symmetry checks run, and it says so.
"""

from __future__ import annotations

import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PARSER = (ROOT / "crates/nme-core/src/parser.rs").read_text(encoding="utf-8")
SYNTAX = (ROOT / "crates/nme-core/src/syntax.rs").read_text(encoding="utf-8")
LOWER = (ROOT / "crates/nme-core/src/lower.rs").read_text(encoding="utf-8")

HANGUL = re.compile(r"[\uac00-\ud7a3]")


def bilingual_helper_names() -> dict[str, str]:
    """Korean helper name -> its English twin, read out of the bundled modules.

    A bundled module binds both spellings of every helper, so a Korean beginner
    program and an English one produce the same Python except for these names.
    They are picked out of `lower.rs` three ways, all of them mechanical, so a
    helper added later is paired without anybody editing this file:

    * `zk_secret = 영지식비밀만들기 = lambda…` — an explicit chain.
    * `파일읽기 = file_read;` — a direct alias.
    * `random_number = 랜덤.randint;` beside `랜덤정수 = 랜덤.randint;` — two
      names bound to one right-hand side.
    """
    pairs: dict[str, str] = {}
    for english, korean in re.findall(
        r'"\s*([A-Za-z_]\w*) = ([^\s=";]+) = ', LOWER
    ):
        if HANGUL.search(korean):
            pairs[korean] = english
    for korean, english in re.findall(
        r'"\s*([^\s=";]*[\uac00-\ud7a3][^\s=";]*) = ([A-Za-z_]\w*); ', LOWER
    ):
        pairs.setdefault(korean, english)
    by_value: dict[str, list[str]] = {}
    for name, value in re.findall(r'"\s*([^\s=";]+) = ([^;"]+); ', LOWER):
        by_value.setdefault(value.strip(), []).append(name)
    for names in by_value.values():
        english = [n for n in names if not HANGUL.search(n)]
        korean = [n for n in names if HANGUL.search(n)]
        if len(english) == 1 and len(korean) == 1:
            pairs.setdefault(korean[0], english[0])
    # Longest first, so `영지식비밀만들기` is replaced before `영지식비밀`.
    return dict(sorted(pairs.items(), key=lambda item: -len(item[0])))


HELPER_NAMES = bilingual_helper_names()

LEVELS = ("sentence", "beginner", "advanced")
LANGUAGES = ("en", "ko")


# --------------------------------------------------------------------------
# Known gaps.
#
# Each key is "<capability id>/<level>-<language>" and each value is the one
# reason it is not closed yet. Everything absent from this table must compile.
# Remove a line here the moment the spelling lands; do not add one without a
# reason a reader can act on.
# --------------------------------------------------------------------------
KNOWN_GAPS = {
    # The English zero-knowledge sentence grammar stops after the
    # non-interactive forms. Korean has all thirteen. Worse than missing: the
    # English attempt is silently read as text, so `set z to r s e zero
    # knowledge response make` saves a sentence instead of a response.
    "zk_challenge_except/sentence-en": "no English sentence spelling; the attempt becomes text",
    "zk_response/sentence-en": "no English sentence spelling; the attempt becomes text",
    "zk_verify/sentence-en": "no English sentence spelling; the attempt becomes text",
    "zk_sim_response/sentence-en": "no English sentence spelling; the attempt becomes text",
    "zk_sim_commitment/sentence-en": "no English sentence spelling; the attempt becomes text",
    # `CompareOp::Contains` is declared in syntax.rs and lowered in lower.rs,
    # but parser.rs never builds it, so neither documented spelling parses.
    "cmp_contains/sentence-en": "parser never produces CompareOp::Contains",
    "cmp_contains/sentence-ko": "parser never produces CompareOp::Contains",
    # `아니면` / `아니면 만약에` are only read in the flat `끝`-terminated block.
    # Inside an indented block the line falls through to Python, where English
    # gets `else:` for free because Python spells it the same way and Korean
    # gets nothing at all.
    "else_indented/beginner-ko": "`아니면:` is not read inside an indented block",
    "elif_indented/beginner-ko": "`아니면 만약에 …:` is not read inside an indented block",
    # Splitting a program across files exists only in the Python-shaped form.
    "nme_import/sentence-en": "no sentence spelling for a .nme import",
    "nme_import/sentence-ko": "no sentence spelling for a .nme import",
    "nme_import/beginner-ko": "the import line is spelled with the English words `from` and `import`",
    # Inside a beginner colon block a skip is unreachable in both languages:
    # `skip` / `건너뛰어` stay bare Python names that do nothing, and Python's
    # own `continue` is refused with E0107 even though `break` is accepted.
    "continue/beginner-en": "`skip` is a no-op and `continue` is refused inside a `times:` block",
    "continue/beginner-ko": "`건너뛰어` is a no-op and `continue` is refused inside a `번:` block",
    # Sentence grammar has no spelling for two of the bundled helpers; they are
    # reachable only after `use random` / `use file`.
    "shuffle/sentence-en": "no sentence spelling; reachable after `use random`",
    "shuffle/sentence-ko": "no sentence spelling; reachable after `랜덤 사용`",
    "json/sentence-en": "no sentence spelling; reachable after `use file`",
    "json/sentence-ko": "no sentence spelling; reachable after `파일 사용`",
}

# Word lists that legitimately exist in one language only, because the two
# languages carry the same meaning with different machinery. Anything not
# listed here must have a twin.
WORD_LIST_EXCEPTIONS = {
    "APPEND_CONNECTORS_EN": "Korean marks the list with a particle (APPEND_TARGET_PARTICLES_KO)",
    "UPDATE_CONNECTOR_WORDS_EN": "Korean marks the amount with a particle (UPDATE_AMOUNT_PARTICLES_KO)",
    "EACH_WORDS_EN": "Korean uses the attached ending EACH_SUFFIX_KO",
    "TIMER_WORDS_EN": "Korean writes the whole phrase joined in START_TIMER_WORDS_KO",
    "BREAK_ALIAS_WORDS_EN": "aliases that are also Python names; Korean has no such collision",
    "CONTINUE_ALIAS_WORDS_EN": "aliases that are also Python names; Korean has no such collision",
    "APPEND_TARGET_PARTICLES_KO": "particles; English uses APPEND_CONNECTORS_EN",
    "UPDATE_AMOUNT_PARTICLES_KO": "particles; English uses UPDATE_CONNECTOR_WORDS_EN",
    "SET_TARGET_PARTICLES_KO": "particles; English uses `set … to …`",
    "EACH_CONTAINER_PARTICLES_KO": "particles; English uses `in`",
    "VALUE_ENDINGS_KO": "spoken endings; English has none",
    "SENTENCE_ENDINGS_KO": "spoken endings; English has none",
    "COOLDOWN_BUSY_WORDS_KO": "English spells it `is on cooldown`, reusing COOLDOWN_WORDS_EN",
    "COOLDOWN_UNTIL_WORDS_KO": "English spells it `wait for <name>`, reusing WAIT_WORDS_EN",
    "CHANCE_LEAD_WORDS_EN": "Korean marks the chance with a particle (CHANCE_PARTICLES_KO)",
    "CHANCE_TIME_WORDS_EN": "`one time in ten`; Korean counts with CHANCE_PERCENT_WORDS_KO",
    "CHANCE_IS_WORDS_EN": "`the chance is 30 percent`; Korean uses the verb ending",
    "CHANCE_PARTICLES_KO": "particles; English uses CHANCE_LEAD_WORDS_EN",
}


# Statements the compiler declares but no code can reach yet. They are named
# here so the inventory check stays useful while they are being built; delete a
# line the moment its parser lands, and the matrix will then demand real cells.
#
# Empty, and meant to stay that way. `Story` and `Chance` were the last two
# entries and their parsers have landed, so they are ordinary matrix rows now.
UNWIRED: dict[str, str] = {}


# --------------------------------------------------------------------------
# The matrix.
#
# One row per capability:
#   id, human name, {(level, language): (lines, index)}, names, covers
#
# `lines` are appended after that language's preamble; `index` picks the line
# whose Python is compared. `names` maps a Korean name in the Korean cells to
# the English name in the English cells, so the two can be compared literally.
# `covers` names the compiler enum variants this row proves, and every variant
# has to be named by some row.
#
# Advanced cells are written once: advanced NME is Python, which has no
# natural-language variant beyond the names, so the same program stands for
# both advanced columns.
# --------------------------------------------------------------------------

PREAMBLE = {
    "en": [
        "set score to 0",
        "set ready to True",
        "set waiting to False",
        "set gap to 3",
        "set friends to list of Mina",
        "start the timer",
        "put door on cooldown for 3 seconds",
    ],
    "ko": [
        "score는 0",
        "ready는 참",
        "waiting은 거짓",
        "gap은 3",
        "friends는 목록 Mina",
        "시간 재기 시작해",
        "door 쿨타임 3초 걸어",
    ],
}
# Both preambles bind the same Python names on purpose: the cross-language
# check then only has to map the names a row introduces itself.


def row(rid, name, covers, sentence_en, sentence_ko, beginner_en, beginner_ko,
        advanced, names=None):
    return {
        "id": rid,
        "name": name,
        "covers": tuple(covers),
        "names": names or {},
        "cells": {
            ("sentence", "en"): sentence_en,
            ("sentence", "ko"): sentence_ko,
            ("beginner", "en"): beginner_en,
            ("beginner", "ko"): beginner_ko,
            ("advanced", "en"): advanced,
            ("advanced", "ko"): advanced,
        },
    }


def line(text):
    return ([text], 0)


def block(header, body, closer):
    return ([header, body, closer], 0)


SLOW_PLAIN = ('[print(_ch, end="", flush=True) or __import__("time").sleep(0.04) '
              'for _ch in "Hello"]; print()')
SLOW_VERY = ('[print(_ch, end="", flush=True) or __import__("time").sleep(0.12) '
             'for _ch in "Hello"]; print()')
SLOW_EVERY = ('[print(_ch, end="", flush=True) or __import__("time").sleep(3) '
              'for _ch in "Hello"]; print()')


def whole(*lines):
    """Compare every produced line, not just one.

    A one-line comparison is enough for a statement, but a block statement is
    only half proved by its header: `story:` lowering to `if True:` says
    nothing about whether the lines inside it were kept as prose. `whole` asks
    for the finished block, indentation included.
    """
    return (list(lines), slice(0, len(lines)))


MATRIX = [
    # ---------------------------------------------------------------- output
    row("say", "Say a value", ["NmeStmt::Say", "Value::Text"],
        line("show Hello"), line("Hello 말해줘"),
        line('say "Hello"'), line('말해 "Hello"'),
        line('print("Hello")')),
    row("say_python", "Say the result of an expression", ["Value::Python"],
        line("show score + 1"), line("score + 1 말해줘"),
        line("say score + 1"), line("말해 score + 1"),
        line("print(score + 1)")),
    row("say_literal", "Say a true/false/none literal", ["Literal::True", "Value::Literal"],
        line("show true"), line("참 말해줘"),
        line("say True"), line("말해 True"),
        line("print(True)")),

    # ---------------------------------------------------------------- input
    row("ask_text", "Ask for text", ["NmeStmt::Ask", "InputKind::Text"],
        line("ask friend What is your name?"), line("friend을 물어봐 이름이 뭐예요?"),
        line('ask friend, "Name? "'), line('물어봐 friend, "Name? "'),
        line('friend = input("Name? ")'),
        names={"이름이 뭐예요?": "What is your name?"}),
    row("ask_number", "Ask for a number", ["InputKind::Number"],
        line("ask number age How old are you?"), line("age를 숫자로 물어봐 몇 살이에요?"),
        line('ask number age, "Age? "'), line('물어봐 숫자로 age, "Age? "'),
        line('age = int(input("Age? "))'),
        names={"몇 살이에요?": "How old are you?"}),

    # ---------------------------------------------------------------- save
    row("set", "Save a value", ["NmeStmt::Set"],
        line("set greeting to Hello"), line("greeting은 Hello"),
        line("save greeting to 1 + 2"), line("저장 greeting 1 + 2"),
        line("greeting = 1 + 2")),

    # ---------------------------------------------------------------- update
    row("add", "Add to a saved number", ["NmeStmt::Update", "UpdateOp::Add"],
        line("add 1 to score"), line("score에 1 더해"),
        line("add 1 + 2 to score"), line("score에 1 + 2 더해"),
        line("score = score + 1")),
    row("subtract", "Subtract from a saved number", ["UpdateOp::Subtract"],
        line("subtract 1 from score"), line("score에서 1 빼줘"),
        line("subtract 1 + 2 from score"), line("score에서 1 + 2 빼줘"),
        line("score = score - 1")),
    row("multiply", "Multiply a saved number", ["UpdateOp::Multiply"],
        line("multiply score by 2"), line("score에 2 곱해"),
        line("multiply score by 2 + 1"), line("score에 2 + 1 곱해"),
        line("score = score * 2")),
    row("divide", "Divide a saved number", ["UpdateOp::Divide"],
        line("divide score by 2"), line("score를 2로 나눠"),
        line("divide score by 2 + 1"), line("score를 2 + 1로 나눠"),
        line("score = score / 2")),

    # ---------------------------------------------------------------- wait
    row("wait", "Wait a number of seconds", ["NmeStmt::Wait"],
        line("wait 3 seconds"), line("3초 기다려"),
        line("wait gap + 1"), line("gap + 1 기다려"),
        line('import time; time.sleep(3)')),

    # ---------------------------------------------------------------- loops
    row("times", "Repeat a number of times", ["NmeStmt::Times"],
        block("repeat 3 times", "show hi", "end"),
        block("3번 반복해", "hi 말해줘", "끝"),
        (["3 times:", "    print(1)"], 0),
        (["3번:", "    print(1)"], 0),
        (["for _ in range(3):", "    print(1)"], 0)),
    row("times_inline", "Repeat a number of times on one line", ["InlineStmt::Nme"],
        line("repeat 3 times and show hi"), line("3번 반복해서 hi 말해줘"),
        line('3 times: say "hi"'), line('3번: 말해 "hi"'),
        line('for _ in range(3): print("hi")')),
    row("for_each", "Repeat over a list", ["NmeStmt::ForEach"],
        block("for each friend in friends", "show friend", "end"),
        block("friends의 friend마다 반복해", "friend 말해줘", "끝"),
        (["for each friend in friends:", "    print(friend)"], 0),
        (["friends의 friend마다:", "    print(friend)"], 0),
        (["for friend in friends:", "    print(friend)"], 0)),
    row("while", "Repeat while a condition holds", ["NmeStmt::While"],
        block("while score is less than 3", "show hi", "end"),
        block("score가 3보다 작을 동안", "hi 말해줘", "끝"),
        (["while score < 3", "    print(1)", "end"], 0),
        (["동안 score < 3", "    print(1)", "끝"], 0),
        (["while score < 3:", "    print(1)"], 0)),

    # ---------------------------------------------------------------- branches
    row("if", "A condition", ["NmeStmt::When", "Condition::Python"],
        block("if score is greater than 10", "show hi", "end"),
        block("만약에 score가 10보다 크면", "hi 말해줘", "끝"),
        (["when score > 10", "    print(1)", "end"], 0),
        (["만약 score > 10", "    print(1)", "끝"], 0),
        (["if score > 10:", "    print(1)"], 0)),
    row("elif", "Another branch, flat block", ["NmeStmt::ElseIf"],
        (["if score > 10", "show a", "else if score == 0", "show b", "end"], 2),
        (["만약 score > 10", "a 말해줘", "아니면 만약에 score == 0", "b 말해줘", "끝"], 2),
        (["when score > 10", "print(1)", "else if score == 0", "print(2)", "end"], 2),
        (["만약 score > 10", "print(1)", "아니면만약에 score == 0", "print(2)", "끝"], 2),
        (["if score > 10:", "    print(1)", "elif score == 0:", "    print(2)"], 2)),
    row("else", "The remaining branch, flat block", ["NmeStmt::Else"],
        (["if score > 10", "show a", "else", "show b", "end"], 2),
        (["만약 score > 10", "a 말해줘", "아니면", "b 말해줘", "끝"], 2),
        (["when score > 10", "print(1)", "else", "print(2)", "end"], 2),
        (["만약 score > 10", "print(1)", "아니면", "print(2)", "끝"], 2),
        (["if score > 10:", "    print(1)", "else:", "    print(2)"], 2)),
    row("elif_indented", "Another branch inside an indented block", [],
        (["if score > 10", "    print(1)", "else if score == 0", "    print(2)", "end"], 2),
        (["만약 score > 10", "    print(1)", "아니면 만약에 score == 0", "    print(2)", "끝"], 2),
        (["when score > 10:", "    print(1)", "elif score == 0:", "    print(2)"], 2),
        (["만약 score > 10:", "    print(1)", "아니면 만약에 score == 0:", "    print(2)"], 2),
        (["if score > 10:", "    print(1)", "elif score == 0:", "    print(2)"], 2)),
    row("else_indented", "The remaining branch inside an indented block", [],
        (["if score > 10", "    print(1)", "else", "    print(2)", "end"], 2),
        (["만약 score > 10", "    print(1)", "아니면", "    print(2)", "끝"], 2),
        (["when score > 10:", "    print(1)", "else:", "    print(2)"], 2),
        (["만약 score > 10:", "    print(1)", "아니면:", "    print(2)"], 2),
        (["if score > 10:", "    print(1)", "else:", "    print(2)"], 2)),
    row("end", "Close a block by hand", ["NmeStmt::End"],
        (["repeat 2 times", "show hi", "end"], 2),
        (["2번 반복해", "hi 말해줘", "끝"], 2),
        (["2 times", "print(1)", "end"], 2),
        (["2번", "print(1)", "끝"], 2),
        (["for _ in range(2):", "    print(1)", "# end"], 2)),

    # ---------------------------------------------------------------- loop control
    row("break", "Leave a loop", ["NmeStmt::Break"],
        (["repeat 2 times", "break", "end"], 1),
        (["2번 반복해", "멈춰", "끝"], 1),
        (["2 times:", "    break"], 1),
        (["2번:", "    break"], 1),
        (["for _ in range(2):", "    break"], 1)),
    row("continue", "Skip to the next round", ["NmeStmt::Continue"],
        (["repeat 2 times", "skip", "end"], 1),
        (["2번 반복해", "건너뛰어", "끝"], 1),
        (["2 times:", "    continue"], 1),
        (["2번:", "    continue"], 1),
        (["for _ in range(2):", "    continue"], 1)),

    # ---------------------------------------------------------------- conditions
    row("cmp_truthy", "Condition: a value is set", ["Condition::Truthy"],
        block("if ready exists", "show hi", "end"),
        block("만약에 ready가 있으면", "hi 말해줘", "끝"),
        (["when ready", "    print(1)", "end"], 0),
        (["만약 ready", "    print(1)", "끝"], 0),
        (["if ready:", "    print(1)"], 0)),
    row("cmp_falsey", "Condition: a value is not set", [],
        block("if ready missing", "show hi", "end"),
        block("만약에 ready가 없으면", "hi 말해줘", "끝"),
        (["when not ready", "    print(1)", "end"], 0),
        (["만약 not ready", "    print(1)", "끝"], 0),
        (["if not ready:", "    print(1)"], 0)),
    row("cmp_eq", "Condition: equal", ["Condition::Compare", "CompareOp::Equal"],
        block("if score equals 10", "show hi", "end"),
        block("만약에 score가 10과 같으면", "hi 말해줘", "끝"),
        (["when score == 10", "    print(1)", "end"], 0),
        (["만약 score == 10", "    print(1)", "끝"], 0),
        (["if score == 10:", "    print(1)"], 0)),
    row("cmp_gt", "Condition: greater than", ["CompareOp::Greater"],
        block("if score is greater than 10", "show hi", "end"),
        block("만약에 score가 10보다 크면", "hi 말해줘", "끝"),
        (["when score > 10", "    print(1)", "end"], 0),
        (["만약 score > 10", "    print(1)", "끝"], 0),
        (["if score > 10:", "    print(1)"], 0)),
    row("cmp_lt", "Condition: less than", ["CompareOp::Less"],
        block("if score is less than 10", "show hi", "end"),
        block("만약에 score가 10보다 작으면", "hi 말해줘", "끝"),
        (["when score < 10", "    print(1)", "end"], 0),
        (["만약 score < 10", "    print(1)", "끝"], 0),
        (["if score < 10:", "    print(1)"], 0)),
    row("cmp_ge", "Condition: greater than or equal", ["CompareOp::GreaterOrEqual"],
        block("if score is greater than or equal to 10", "show hi", "end"),
        block("만약에 score가 10보다 크거나 같으면", "hi 말해줘", "끝"),
        (["when score >= 10", "    print(1)", "end"], 0),
        (["만약 score >= 10", "    print(1)", "끝"], 0),
        (["if score >= 10:", "    print(1)"], 0)),
    row("cmp_le", "Condition: less than or equal", ["CompareOp::LessOrEqual"],
        block("if score is less than or equal to 10", "show hi", "end"),
        block("만약에 score가 10보다 작거나 같으면", "hi 말해줘", "끝"),
        (["when score <= 10", "    print(1)", "end"], 0),
        (["만약 score <= 10", "    print(1)", "끝"], 0),
        (["if score <= 10:", "    print(1)"], 0)),
    row("cmp_contains", "Condition: a list holds a value", ["CompareOp::Contains"],
        block("if friends contains Mina", "show hi", "end"),
        block("만약에 friends에 Mina가 있으면", "hi 말해줘", "끝"),
        (['when "Mina" in friends', "    print(1)", "end"], 0),
        (['만약 "Mina" in friends', "    print(1)", "끝"], 0),
        (['if "Mina" in friends:', "    print(1)"], 0)),
    row("cmp_and", "Condition: both", ["Condition::Logical", "LogicalOp::And"],
        block("if ready and waiting", "show hi", "end"),
        block("만약 ready 그리고 waiting", "hi 말해줘", "끝"),
        (["when ready and waiting", "    print(1)", "end"], 0),
        (["만약 ready and waiting", "    print(1)", "끝"], 0),
        (["if ready and waiting:", "    print(1)"], 0)),
    row("cmp_or", "Condition: either", ["LogicalOp::Or"],
        block("if ready or waiting", "show hi", "end"),
        block("만약 ready 또는 waiting", "hi 말해줘", "끝"),
        (["when ready or waiting", "    print(1)", "end"], 0),
        (["만약 ready or waiting", "    print(1)", "끝"], 0),
        (["if ready or waiting:", "    print(1)"], 0)),

    # ---------------------------------------------------------------- lists
    row("list_make", "Make a list", ["Value::List"],
        line("set pals to list of Mina, Ada"), line("pals는 목록 Mina, Ada"),
        line('save pals to ["Mina", "Ada"]'), line('저장 pals ["Mina", "Ada"]'),
        line('pals = ["Mina", "Ada"]')),
    row("append", "Add an item to a list", ["NmeStmt::Append"],
        line("append Mina to friends"), line("friends에 Mina 넣어"),
        line('append "Mina" to friends'), line('friends에 "Mina" 넣어'),
        line('friends.append("Mina")')),

    # ---------------------------------------------------------------- randomness
    row("random_int", "A random whole number in a range", ["Value::RandomInteger"],
        line("set die to random number from 1 to 6"),
        line("die는 1부터 6까지 랜덤정수"),
        (["use random", "save die to random_number(1, 6)"], 1),
        (["랜덤 사용", "저장 die 랜덤정수(1, 6)"], 1),
        (["import random", "die = random.randint(1, 6)"], 1)),
    row("random_pick", "Pick one of several values", ["Value::RandomChoice"],
        line("set colour to pick from red or green"),
        line("colour는 red 또는 green 중에서 랜덤선택"),
        (["use random", 'save colour to random_pick(["red", "green"])'], 1),
        (["랜덤 사용", '저장 colour 랜덤선택(["red", "green"])'], 1),
        (["import random", 'colour = random.choice(["red", "green"])'], 1)),
    row("shuffle", "Shuffle a list", [],
        None, None,
        (["use random", "shuffle(friends)"], 1),
        (["랜덤 사용", "섞기(friends)"], 1),
        (["import random", "random.shuffle(friends)"], 1)),

    # --------------------------------------------------------------- chance
    # A percentage is stored as permille, so `30%` is 300 and `30.5%` is 305.
    # There is no colon form: `30% chance:` is not read, so the beginner cells
    # keep the one header spelling and differ in the body, which is where the
    # beginner register actually lives.
    row("chance", "Do something some of the time", ["NmeStmt::Chance"],
        line("30% chance show You win"),
        line("30% 확률로 말해줘 당첨"),
        line('30% chance say "You win"'),
        line('30% 확률로 말해 "당첨"'),
        line('if __import__("random").randrange(1000) < 300: print("You win")'),
        names={"당첨": "You win"}),
    row("chance_block", "Do a whole block some of the time", [],
        whole("30% chance", "print(1)", "end"),
        whole("30% 확률로", "print(1)", "끝"),
        whole("with a 30% chance", "    print(1)"),
        whole("30%의 확률로", "    print(1)"),
        whole('if __import__("random").randrange(1000) < 300:', "    print(1)")),
    row("chance_value", "Save a true-some-of-the-time value", ["Value::Chance"],
        line("luck is a 30% chance"),
        line("luck은 30% 확률"),
        line("save luck to a 30% chance"),
        line("저장 luck 30% 확률"),
        line('luck = __import__("random").randrange(1000) < 300')),

    # ---------------------------------------------------------------- files
    row("file_read", "Read a file into a name", ["NmeStmt::FileRead"],
        line('read "notes.txt" into memo'), line('memo에 "notes.txt" 읽어서'),
        (["use file", 'save memo to file_read("notes.txt")'], 1),
        (["파일 사용", '저장 memo 파일읽기("notes.txt")'], 1),
        (["import pathlib", 'memo = pathlib.Path("notes.txt").read_text()'], 1)),
    row("file_write", "Write a value into a file", ["NmeStmt::FileWrite"],
        line('write "hello" to "out.txt"'), line('"out.txt" 파일에 "hello"를 저장해'),
        (["use file", 'file_write("out.txt", "hello")'], 1),
        (["파일 사용", '파일쓰기("out.txt", "hello")'], 1),
        (["import pathlib", 'pathlib.Path("out.txt").write_text("hello")'], 1)),
    row("json", "Read and write JSON", [],
        None, None,
        (["use file", 'save data to json_load("a.json")'], 1),
        (["파일 사용", '저장 data json읽기("a.json")'], 1),
        (["import json", 'data = json.loads("{}")'], 1)),

    # ---------------------------------------------------------------- modules
    row("use_module", "Load a bundled module",
        ["NmeStmt::UseModule", "ModuleVersion::Bundled", "BundledModuleId::Random"],
        line("use random"), line("랜덤 사용"),
        line("use random"), line("랜덤 사용"),
        line("import random")),
    row("use_file_module", "Load the bundled file module", ["BundledModuleId::File"],
        line("use file"), line("파일 사용"),
        line("use file"), line("파일 사용"),
        line("import pathlib")),
    row("use_zk_module", "Load the bundled zero-knowledge module",
        ["BundledModuleId::ZeroKnowledge"],
        line("use zero_knowledge"), line("영지식 사용"),
        line("use zero_knowledge"), line("영지식 사용"),
        line("import secrets")),
    row("use_latest", "Load the newest bundled module", ["ModuleVersion::Latest"],
        line("use random latest"), line("랜덤 사용 최신"),
        line("use random latest"), line("랜덤 사용 최신"),
        line("import random")),
    row("use_version", "Load one exact module version", ["ModuleVersion::Exact"],
        line('use random version "0.0.1"'), line('랜덤 사용 버전 "0.0.1"'),
        line('use random version "0.0.1"'), line('랜덤 사용 버전 "0.0.1"'),
        line("import random")),
    row("nme_import", "Use names from another .nme file", ["NmeStmt::ModuleImport"],
        None, None,
        line('from "helper.nme" import greet'), None,
        line("from helper import greet")),

    # ---------------------------------------------------------------- slow text
    row("slow", "Say text one character at a time", ["NmeStmt::SaySlowly"],
        line("say slowly Hello"), line("천천히 말해줘 Hello"),
        line('say slowly "Hello"'), line('천천히 말해줘 "Hello"'),
        line('[print(_ch, end="", flush=True) or __import__("time").sleep(0.04) for _ch in "Hello"]; print()')),

    row("slow_very", "Say text very slowly", [],
        line("say very slowly Hello"), line("아주 천천히 말해줘 Hello"),
        line('say very slowly "Hello"'), line('아주 천천히 말해줘 "Hello"'),
        line('[print(_ch, end="", flush=True) or __import__("time").sleep(0.12) for _ch in "Hello"]; print()')),

    row("slow_every", "Say text with a chosen pause", [],
        line("say slowly every 3 seconds Hello"), line("3초씩 천천히 말해줘 Hello"),
        line('say slowly every 3 seconds "Hello"'), line('3초씩 천천히 말해줘 "Hello"'),
        line('[print(_ch, end="", flush=True) or __import__("time").sleep(3) for _ch in "Hello"]; print()')),


    # ---------------------------------------------------------------- story
    # A story block has no beginner vocabulary of its own: nothing inside it is
    # ever read as code, so there is no quoted or exact spelling to offer. The
    # beginner cells therefore differ only in how the block is closed — a colon
    # and indentation rather than `end` / `끝`, the same shape `3 times:` uses.
    row("story", "Tell a block of prose, never code", ["NmeStmt::Story"],
        whole("story:", "Once upon a time", "end"),
        whole("이야기:", "옛날 옛적에", "끝"),
        whole("story:", "    Once upon a time"),
        whole("이야기:", "    옛날 옛적에"),
        whole("if True:", '    print("Once upon a time")'),
        names={"옛날 옛적에": "Once upon a time"}),
    row("story_slow", "Tell a story one character at a time", [],
        whole("slow story:", "Hello", "end"),
        whole("천천히 이야기:", "Hello", "끝"),
        whole("slow story:", "    Hello"),
        whole("천천히 이야기:", "    Hello"),
        whole("if True:", "    " + SLOW_PLAIN)),
    row("story_very_slow", "Tell a story very slowly", [],
        whole("very slow story:", "Hello", "end"),
        whole("아주 천천히 이야기:", "Hello", "끝"),
        whole("very slow story:", "    Hello"),
        whole("아주 천천히 이야기:", "    Hello"),
        whole("if True:", "    " + SLOW_VERY)),
    row("story_slow_every", "Tell a story with a chosen pause", [],
        whole("slow story every 3 seconds:", "Hello", "end"),
        whole("3초씩 천천히 이야기:", "Hello", "끝"),
        whole("slow story every 3 seconds:", "    Hello"),
        whole("3초씩 천천히 이야기:", "    Hello"),
        whole("if True:", "    " + SLOW_EVERY)),

    # ---------------------------------------------------------------- screen
    row("clear", "Clear the screen", ["NmeStmt::ClearScreen"],
        line("clear the screen"), line("화면 지워"),
        line("clear screen"), line("화면 비워"),
        line('print("\\033[2J\\033[3J\\033[H", end="")')),

    row("draw_line", "Draw a line across the screen", ["NmeStmt::DrawLine"],
        line("draw a line"), line("줄 그어"),
        line("draw line"), line("가로줄 그어줘"),
        line('print("─" * 40)')),

    row("box", "Say text inside a box", ["NmeStmt::SayInBox"],
        line("say in a box Hello"), line("상자로 말해줘 Hello"),
        line('say in a box "Hello"'), line('상자로 말해줘 "Hello"'),
        line('print((lambda _t: (lambda _w: "┌" + "─" * (_w + 2) + "┐\\n│ " + _t + " │\\n└" + "─" * (_w + 2) + "┘")(sum(2 if __import__("unicodedata").east_asian_width(_c) in "WF" else 1 for _c in _t)))("Hello"))')),

    row("middle", "Say text in the middle", ["NmeStmt::SayInMiddle"],
        line("say in the middle Hello"), line("가운데 말해줘 Hello"),
        line('say in the middle "Hello"'), line('가운데 말해줘 "Hello"'),
        line('print((lambda _t: " " * max(0, (40 - sum(2 if __import__("unicodedata").east_asian_width(_c) in "WF" else 1 for _c in _t)) // 2) + _t)("Hello"))')),


    # ---------------------------------------------------------------- stopwatch
    row("timer_start", "Start the stopwatch", ["NmeStmt::StartTimer"],
        line("start the timer"), line("시간 재기 시작해"),
        line("start timer"), line("시간재기 시작해"),
        line('_nme_clock = __import__("time").time()')),

    row("elapsed", "Read the stopwatch", ["Value::Elapsed"],
        line("show elapsed"), line("잰시간 말해줘"),
        line("say elapsed"), line("말해 잰시간"),
        line('print(round(__import__("time").time() - _nme_clock, 2))')),


    # ---------------------------------------------------------------- cooldowns
    row("cool_set", "Put a name on cooldown", ["NmeStmt::Cooldown"],
        line("put gate on cooldown for 3 seconds"), line("gate 쿨타임 3초 걸어"),
        line("put gate on cooldown for gap seconds"), line("gate 쿨타임 gap 초 걸어"),
        line('_nme_cool_gate = __import__("time").time() + 3')),

    row("cool_ready", "Condition: the cooldown has ended", [],
        block("if door is ready", "show hi", "end"),
        block("만약 door 쿨타임이 끝났으면", "hi 말해줘", "끝"),
        block("when door is ready", "show hi", "end"),
        block("만약 door 쿨타임 끝났으면", "hi 말해줘", "끝"),
        (['if (__import__("time").time() >= _nme_cool_door):', '    print(1)'], 0)),

    row("cool_busy", "Condition: the cooldown is still running", [],
        block("if door is on cooldown", "show hi", "end"),
        block("만약 door 쿨타임이 남았으면", "hi 말해줘", "끝"),
        block("when door is on cooldown", "show hi", "end"),
        block("만약 door 쿨타임 남았으면", "hi 말해줘", "끝"),
        (['if (__import__("time").time() < _nme_cool_door):', '    print(1)'], 0)),

    row("cool_wait", "Wait until a cooldown ends", ["NmeStmt::WaitForCooldown"],
        line("wait for door"), line("door 쿨타임 끝날때까지 기다려"),
        line("pause for door"), line("door 쿨타임 끝날 때까지 기다려"),
        line('__import__("time").sleep(max(0, _nme_cool_door - __import__("time").time()))')),


    # ------------------------------------------------------- zero knowledge
    row("zk_secret", "Zero knowledge: make a secret",
        ["ZeroKnowledgeValue::Secret", "Value::ZeroKnowledge"],
        (["use zero_knowledge", "set s to zero knowledge secret make"], 1),
        (["영지식 사용", "s는 영지식 비밀 만들기"], 1),
        (["use zero_knowledge", "save s to zk_secret()"], 1),
        (["영지식 사용", "저장 s 영지식비밀만들기()"], 1),
        (['import secrets', 's = secrets.randbelow(10) + 1'], 1)),

    row("zk_public", "Zero knowledge: make a public value", ["ZeroKnowledgeValue::Public"],
        (["use zero_knowledge", "set s to zero knowledge secret make",
          "set p to s zero knowledge public make"], 2),
        (["영지식 사용", "s는 영지식 비밀 만들기", "p는 s로 영지식 공개값 만들기"], 2),
        (["use zero_knowledge", "save p to zk_public(1)"], 1),
        (["영지식 사용", "저장 p 영지식공개값(1)"], 1),
        (['import secrets', 'p = pow(2, 1, 23)'], 1)),

    row("zk_nonce", "Zero knowledge: make a one-time value", ["ZeroKnowledgeValue::Nonce"],
        (["use zero_knowledge", "set r to zero knowledge nonce make"], 1),
        (["영지식 사용", "r는 영지식 일회값 만들기"], 1),
        (["use zero_knowledge", "save r to zk_nonce()"], 1),
        (["영지식 사용", "저장 r 영지식일회값만들기()"], 1),
        (['import secrets', 'r = secrets.randbelow(11)'], 1)),

    row("zk_commitment", "Zero knowledge: make a commitment", ["ZeroKnowledgeValue::Commitment"],
        (["use zero_knowledge", "set r to zero knowledge nonce make",
          "set c to r zero knowledge commitment make"], 2),
        (["영지식 사용", "r는 영지식 일회값 만들기", "c는 r로 영지식 약속 만들기"], 2),
        (["use zero_knowledge", "save c to zk_commitment(1)"], 1),
        (["영지식 사용", "저장 c 영지식약속(1)"], 1),
        (['import secrets', 'c = pow(2, 1, 23)'], 1)),

    row("zk_challenge", "Zero knowledge: make a challenge", ["ZeroKnowledgeValue::Challenge"],
        (["use zero_knowledge", "set e to zero knowledge challenge make"], 1),
        (["영지식 사용", "e는 영지식 도전 만들기"], 1),
        (["use zero_knowledge", "save e to zk_challenge()"], 1),
        (["영지식 사용", "저장 e 영지식도전만들기()"], 1),
        (['import secrets', 'e = secrets.randbelow(256)'], 1)),

    row("zk_challenge_except", "Zero knowledge: make a different challenge",
        ["ZeroKnowledgeValue::ChallengeExcept"],
        None,
        (["영지식 사용", "e는 영지식 도전 만들기", "f는 e와 다른 영지식 도전 만들기"], 2),
        (["use zero_knowledge", "save f to zk_challenge_except(1)"], 1),
        (["영지식 사용", "저장 f 영지식다른도전(1)"], 1),
        (['import secrets', 'f = secrets.randbelow(255) + 1'], 1)),

    row("zk_response", "Zero knowledge: make a response", ["ZeroKnowledgeValue::Response"],
        None,
        (["영지식 사용", "r는 영지식 일회값 만들기", "s는 영지식 비밀 만들기",
          "e는 영지식 도전 만들기", "z는 r와 s와 e로 영지식 응답 만들기"], 4),
        (["use zero_knowledge", "save z to zk_response(1, 2, 3)"], 1),
        (["영지식 사용", "저장 z 영지식응답(1, 2, 3)"], 1),
        (['import secrets', 'z = (1 - 2 * 3) % 11'], 1)),

    row("zk_verify", "Zero knowledge: check a proof", ["ZeroKnowledgeValue::Verify"],
        None,
        (["영지식 사용", "p는 1", "c는 2", "e는 3", "z는 4",
          "ok는 p와 c와 e와 z로 영지식 검증"], 5),
        (["use zero_knowledge", "save ok to zk_verify(1, 2, 3, 4)"], 1),
        (["영지식 사용", "저장 ok 영지식검증(1, 2, 3, 4)"], 1),
        (['import secrets', 'ok = 2 == (pow(2, 4, 23) * pow(1, 3, 23)) % 23'], 1)),

    row("zk_sim_response", "Zero knowledge: make a simulated response",
        ["ZeroKnowledgeValue::SimulatedResponse"],
        None,
        (["영지식 사용", "z는 영지식 모의 응답 만들기"], 1),
        (["use zero_knowledge", "save z to zk_simulated_response()"], 1),
        (["영지식 사용", "저장 z 영지식모의응답만들기()"], 1),
        (['import secrets', 'z = secrets.randbelow(11)'], 1)),

    row("zk_sim_commitment", "Zero knowledge: make a simulated commitment",
        ["ZeroKnowledgeValue::SimulatedCommitment"],
        None,
        (["영지식 사용", "p는 1", "e는 2", "z는 3", "c는 p와 e와 z로 영지식 모의 약속 만들기"], 4),
        (["use zero_knowledge", "save c to zk_simulated_commitment(1, 2, 3)"], 1),
        (["영지식 사용", "저장 c 영지식모의약속(1, 2, 3)"], 1),
        (['import secrets', 'c = (pow(2, 3, 23) * pow(1, 2, 23)) % 23'], 1)),

    row("zk_nizk_challenge", "Zero knowledge: bind a challenge to a context",
        ["ZeroKnowledgeValue::NizkChallenge"],
        (["use zero_knowledge", "set p to 1", "set c to 2", "set ctx to 3",
          "set e to p c ctx zero knowledge challenge make"], 4),
        (["영지식 사용", "p는 1", "c는 2", "ctx는 3",
          "e는 p와 c와 ctx로 영지식 비대화 도전 만들기"], 4),
        (["use zero_knowledge", "save e to zk_nizk_challenge(1, 2, 3)"], 1),
        (["영지식 사용", "저장 e 영지식비대화도전(1, 2, 3)"], 1),
        (['import hashlib', 'e = int.from_bytes(hashlib.sha256(b"1 2 3").digest(), "big")'], 1)),

    row("zk_nizk_prove", "Zero knowledge: make a shareable proof",
        ["ZeroKnowledgeValue::NizkProof"],
        (["use zero_knowledge", "set s to 1", "set ctx to 2",
          "set proof to s ctx zero knowledge proof make"], 3),
        (["영지식 사용", "s는 1", "ctx는 2", "proof는 s와 ctx로 영지식 비대화 증명 만들기"], 3),
        (["use zero_knowledge", "save proof to zk_nizk_prove(1, 2)"], 1),
        (["영지식 사용", "저장 proof 영지식비대화증명(1, 2)"], 1),
        (['import secrets', 'proof = [pow(2, 1, 23), (1 - 2 * 3) % 11]'], 1)),

    row("zk_nizk_verify", "Zero knowledge: check a shareable proof",
        ["ZeroKnowledgeValue::NizkVerify"],
        (["use zero_knowledge", "set p to 1", "set proof to 2", "set ctx to 3",
          "set ok to p proof ctx zero knowledge verify"], 4),
        (["영지식 사용", "p는 1", "proof는 2", "ctx는 3",
          "ok는 p와 proof와 ctx로 영지식 비대화 검증"], 4),
        (["use zero_knowledge", "save ok to zk_nizk_verify(1, 2, 3)"], 1),
        (["영지식 사용", "저장 ok 영지식비대화검증(1, 2, 3)"], 1),
        (['import secrets', 'ok = isinstance([1, 2], list) and len([1, 2]) == 2'], 1)),

]


# --------------------------------------------------------------------------
# Refusals that are part of a capability's contract.
#
# A capability is not at parity if one language gets a clear error and the
# other quietly accepts the same mistake. These are the refusals a writer is
# meant to meet, one English program and one Korean program each; both must be
# refused, and with the same code.
# --------------------------------------------------------------------------
DIAGNOSTIC_PARITY = [
    ("E0227", "a chance goes to at most one decimal place",
     "30.55% chance show hi", "30.55% 확률로 말해줘 당첨"),
    ("E0228", "a chance stays between 0% and 100%",
     "150% chance show hi", "150% 확률로 말해줘 당첨"),
]


# --------------------------------------------------------------------------
# 1. Inventory, read out of the compiler.
# --------------------------------------------------------------------------

# Variants the parity matrix does not have to name one by one, with the reason.
INVENTORY_EXCEPTIONS = {
    "Code::Source": "how a span is stored, not something a program writes",
    "Code::Generated": "how a span is stored, not something a program writes",
    "TextPart::Literal": "a piece of a text template, covered by say/say_interp",
    "TextPart::Variable": "a piece of a text template, covered by say/say_interp",
    "Literal::False": "covered together with Literal::True by say_literal",
    "Literal::None": "covered together with Literal::True by say_literal",
    "Spelling::English": "which language the parser matched, not a capability",
    "Spelling::Korean": "which language the parser matched, not a capability",
    "ConditionValue::Python": "how one side of a condition is stored",
    "ConditionValue::Name": "how one side of a condition is stored",
    "ConditionValue::Text": "how one side of a condition is stored",
    "ConditionValue::Literal": "how one side of a condition is stored",
    "CompareOp::NotEqual": "the negated form of CompareOp::Equal",
    "InlineStmt::Python": "a Python line inside an NME block, covered by times",
}

INVENTORY_ENUMS = (
    "NmeStmt", "ZeroKnowledgeValue", "CompareOp", "UpdateOp", "InputKind",
    "BundledModuleId", "ModuleVersion", "LogicalOp", "Condition", "Value",
    "Literal", "InlineStmt", "TextPart", "ConditionValue", "Code", "Spelling",
)


def enum_variants(name: str) -> list[str]:
    match = re.search(rf"pub enum {name} \{{(.*?)\n\}}", SYNTAX, re.S)
    if match is None:
        raise SystemExit(f"check-tier-parity: no enum named {name} in syntax.rs")
    body = re.sub(r"//.*", "", match.group(1))
    return re.findall(r"^\s{4}([A-Z]\w*)", body, re.M)


def check_inventory(problems: list[str]) -> int:
    covered = {name for entry in MATRIX for name in entry["covers"]}
    total = 0
    for enum in INVENTORY_ENUMS:
        for variant in enum_variants(enum):
            key = f"{enum}::{variant}"
            total += 1
            if key in covered or key in INVENTORY_EXCEPTIONS:
                continue
            if key in UNWIRED:
                print(f"check-tier-parity: {key} is declared but unreachable "
                      f"({UNWIRED[key]})")
                continue
            problems.append(
                f"inventory: {key} exists in the compiler but no parity row covers it; "
                f"add a row to MATRIX saying how it is written at each level in each language"
            )
    known = {
        f"{enum}::{variant}" for enum in INVENTORY_ENUMS for variant in enum_variants(enum)
    }
    for key in sorted(set(UNWIRED) - known):
        problems.append(f"UNWIRED names {key}, which no longer exists in syntax.rs")
    stale = covered - known
    for key in sorted(stale):
        problems.append(f"inventory: a parity row claims to cover {key}, which no longer exists")
    return total


# --------------------------------------------------------------------------
# 2. Word-list symmetry, read out of the compiler.
# --------------------------------------------------------------------------

def check_word_lists(problems: list[str]) -> int:
    source = PARSER + SYNTAX
    names = set(re.findall(r"const\s+([A-Z0-9_]+)\s*:\s*&\[&str\]", source))
    checked = 0
    for name in sorted(names):
        for mine, theirs in (("_EN", "_KO"), ("_KO", "_EN")):
            if not name.endswith(mine):
                continue
            checked += 1
            twin = name[: -len(mine)] + theirs
            if twin in names or name in WORD_LIST_EXCEPTIONS:
                continue
            problems.append(
                f"word lists: {name} has no {twin}; one language accepts a spelling "
                f"the other cannot (add the twin, or explain it in WORD_LIST_EXCEPTIONS)"
            )
    return checked


# --------------------------------------------------------------------------
# 3 and 4. Reachability and cross-language identity.
# --------------------------------------------------------------------------

def shorten(text: str, limit: int = 160) -> str:
    """One line, or one block, short enough to read in a build log."""
    text = text.replace("\n", " ⏎ ")
    return text if len(text) <= limit else text[:limit] + " …"


def compile_cell(binary: Path, folder: Path, entry: dict, level: str, language: str):
    """Returns (python_line, None) or (None, first error line)."""
    cell = entry["cells"][(level, language)]
    if cell is None:
        return None, "declared missing in the matrix"
    lines, at = cell
    preamble = PREAMBLE["en" if level == "advanced" else language]
    source = folder / "row.nme"
    output = folder / "row.py"
    source.write_text("\n".join(preamble + list(lines)) + "\n", encoding="utf-8")
    if output.exists():
        output.unlink()
    result = subprocess.run(
        [str(binary), "build", str(source), "-o", str(output)],
        capture_output=True, text=True,
    )
    if result.returncode != 0:
        first = (result.stdout + result.stderr).strip().splitlines()
        return None, (first[0] if first else "no output")
    produced = output.read_text(encoding="utf-8").splitlines()
    if isinstance(at, slice):
        start = len(preamble) + (at.start or 0)
        stop = len(preamble) + at.stop
        if stop > len(produced):
            return None, "the compiler produced fewer lines than the program has"
        # Keep the indentation: it is what proves the body sits inside the block.
        return "\n".join(row.rstrip() for row in produced[start:stop]), None
    index = len(preamble) + at
    if index >= len(produced):
        return None, "the compiler produced fewer lines than the program has"
    return produced[index].strip(), None


def check_diagnostics(problems: list[str], binary: Path) -> int:
    checked = 0
    with tempfile.TemporaryDirectory() as name:
        folder = Path(name)
        source = folder / "refused.nme"
        for code, meaning, english, korean in DIAGNOSTIC_PARITY:
            for language, program in (("en", english), ("ko", korean)):
                checked += 1
                source.write_text(program + "\n", encoding="utf-8")
                result = subprocess.run(
                    [str(binary), "check", str(source)],
                    capture_output=True, text=True,
                )
                output = result.stdout + result.stderr
                if result.returncode == 0:
                    problems.append(
                        f"refusal {code}-{language} ({meaning}): "
                        f"`{program}` was accepted; the other language refuses it"
                    )
                elif f"error[{code}]" not in output:
                    first = output.strip().splitlines()
                    problems.append(
                        f"refusal {code}-{language} ({meaning}): `{program}` was refused, "
                        f"but with {first[0] if first else 'no output'}"
                    )
    return checked


def check_matrix(problems: list[str], binary: Path) -> tuple[int, int]:
    cells = 0
    green_rows = 0
    with tempfile.TemporaryDirectory() as name:
        folder = Path(name)
        # `from "helper.nme" import greet` needs a real neighbour to import.
        (folder / "helper.nme").write_text('set greet to Hi\n', encoding="utf-8")
        for entry in MATRIX:
            produced: dict[tuple[str, str], str] = {}
            row_ok = True
            for level in LEVELS:
                for language in LANGUAGES:
                    key = f"{entry['id']}/{level}-{language}"
                    expected_gap = key in KNOWN_GAPS
                    text, error = compile_cell(binary, folder, entry, level, language)
                    cells += 1
                    if text is None:
                        if not expected_gap:
                            row_ok = False
                            problems.append(
                                f"missing cell {key} ({entry['name']}): {error}"
                            )
                        continue
                    if expected_gap:
                        row_ok = False
                        problems.append(
                            f"stale gap {key} ({entry['name']}): this now compiles; "
                            f"delete its line from KNOWN_GAPS"
                        )
                        continue
                    produced[(level, language)] = text

            for level in LEVELS:
                english = produced.get((level, "en"))
                korean = produced.get((level, "ko"))
                if english is None or korean is None:
                    continue
                mapped = korean
                if mapped != english:
                    for korean_name, english_name in HELPER_NAMES.items():
                        mapped = mapped.replace(korean_name, english_name)
                    for korean_name, english_name in entry["names"].items():
                        mapped = mapped.replace(korean_name, english_name)
                if mapped != english:
                    row_ok = False
                    problems.append(
                        f"languages differ for {entry['id']}/{level} ({entry['name']}):\n"
                        f"    English writes: {shorten(english)}\n"
                        f"    Korean  writes: {shorten(mapped)}"
                    )
            if row_ok:
                green_rows += 1
    return cells, green_rows


def main() -> None:
    problems: list[str] = []
    variants = check_inventory(problems)
    lists = check_word_lists(problems)
    print(
        f"check-tier-parity: {len(MATRIX)} capabilities, "
        f"{variants} compiler variants, {lists} one-language word lists"
    )

    if len(sys.argv) > 1:
        binary = Path(sys.argv[1])
    else:
        binary = next(
            (
                candidate
                for candidate in (ROOT / "target/release/nme", ROOT / "target/debug/nme")
                if candidate.is_file()
            ),
            None,
        )
    if binary is None:
        print("check-tier-parity: no nme binary; skipping the compile checks")
    else:
        cells, green = check_matrix(problems, binary)
        refusals = check_diagnostics(problems, binary)
        print(
            f"check-tier-parity: compiled {cells} cells and checked {refusals} "
            f"refusals; {green} of {len(MATRIX)} capabilities are complete; "
            f"{len(KNOWN_GAPS)} known gaps allowed"
        )

    if problems:
        for problem in problems:
            print(problem, file=sys.stderr)
        raise SystemExit(f"check-tier-parity: {len(problems)} problem(s)")
    print("check-tier-parity: ok")


if __name__ == "__main__":
    main()
