#!/usr/bin/env python3
"""Writes the six files in docs/prompts/.

Each one is a single self-contained Markdown file meant to be **pasted into a
chat** — ChatGPT, Claude, or anything else — so that the assistant on the other
side can write correct NME without ever having seen the language. They are not
agent instructions and they reference no local files.

    python scripts/build-ai-prompts.py [path/to/nme]

Three depths, each in Korean and English:

    nme-sentence      sentence level only — the recommended one
    nme-all-levels    + beginner and advanced syntax
    nme-complete      + worked example programs

The syntax tables come from `build-syntax-reference.py`, so the prompts, the
syntax list and the compiler can never disagree. Every example program in the
deepest prompt is compiled by the real compiler while this script runs, and the
Python shown is exactly what came out; if one stops compiling, this fails.
"""

from __future__ import annotations

import importlib.util
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


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
OUT = ROOT / "docs/prompts"

_spec = importlib.util.spec_from_file_location(
    "syntax_reference", ROOT / "scripts/build-syntax-reference.py"
)
_module = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_module)
S = _module

VERSION = next(
    line.split('"')[1]
    for line in (ROOT / "Cargo.toml").read_text(encoding="utf-8").splitlines()
    if line.startswith("version =")
)


def compiler() -> Path:
    if len(sys.argv) > 1:
        return Path(sys.argv[1])
    for candidate in nme_candidates(ROOT):
        if candidate.is_file():
            return candidate
    raise SystemExit("build-ai-prompts: no nme binary found")


# ---------------------------------------------------------------- examples
#
# (id, Korean program, English program). Every one is compiled below, and the
# Python in the prompt is whatever the compiler actually produced.

EXAMPLES = [
    (
        "인사 / hello",
        "안녕하세요! 말해줘\n3번 반복해서 반가워요 말해줘\n",
        "show Hello!\nrepeat 3 times and show Nice to meet you\n",
    ),
    (
        "이름 묻기 / ask a name",
        "이름이 뭐예요?\n안녕하세요 이름! 말해줘\n",
        "What is your name?\nshow Hello name!\n",
    ),
    (
        "이야기 묶음 / a story in one block",
        "이야기:\n문이 천천히 열렸습니다.\n방 안은 비어 있었습니다.\n끝\n"
        "천천히 이야기:\n탁자 위에 편지가 한 장 있었습니다.\n끝\n",
        "story:\nThe door opened slowly.\nThe room was empty.\nend\n"
        "slow story:\nOne letter lay on the table.\nend\n",
    ),
    (
        "확률 / chance",
        "비는 40% 확률\n만약에 비가 있으면\n우산을 챙기세요 말해줘\n아니면\n"
        "오늘은 맑습니다 말해줘\n끝\n10% 확률로 말해줘 무지개가 떴습니다\n",
        "rain is a 40% chance\nif rain\nshow Take an umbrella\nelse\n"
        "show It is clear today\nend\n10% chance show A rainbow appeared\n",
    ),
    (
        "숫자 맞히기 / guessing game",
        "정답은 1부터 10까지 랜덤정수\n추측을 숫자로 물어봐 1부터 10까지 골라 보세요\n"
        "만약에 추측이 정답과 같으면\n맞았어요 말해줘\n아니면 만약에 추측이 정답보다 작으면\n"
        "더 큰 수예요 말해줘\n아니면\n더 작은 수예요 말해줘\n끝\n",
        "set answer to random number from 1 to 10\nask number guess Pick a number from 1 to 10\n"
        "if guess equals answer\nshow Correct!\nelse if guess is less than answer\n"
        "show Go higher\nelse\nshow Go lower\nend\n",
    ),
    (
        "점수 세기 / counting",
        "점수는 0\n5번 반복해\n점수에 2 곱해\n점수에 1 더해\n끝\n점수 말해줘\n",
        "set score to 0\nrepeat 5 times\nmultiply score by 2\nadd 1 to score\nend\nshow score\n",
    ),
    (
        "목록 하나씩 / going through a list",
        "친구들은 목록 민수, 지안, 서준\n친구들의 친구마다 반복해\n안녕하세요 친구! 말해줘\n끝\n",
        "set friends to list of Mina, Ada and Grace\nfor each friend in friends\nshow Hello friend!\nend\n",
    ),
    (
        "표와 이름 붙인 일 / a record and a named job",
        "나이표는 빈 표\n나이표에 민수를 90으로 넣어\n나이표에 지안을 80으로 넣어\n"
        "결과보기라는 일:\n나이표의 이름마다 반복해\n이름 말해줘\n나이표의 이름 말해줘\n끝\n끝\n"
        "결과보기 해줘\n",
        "set ages to an empty record\nput Mina at 90 in ages\nput Ada at 80 in ages\n"
        "to report:\nfor each name in ages\nshow name\nshow name in ages\nend\nend\n"
        "do report\n",
    ),
    (
        "목록에 넣기 / building a list",
        "이름들은 목록\n3번 반복해\n이름을 물어봐 이름을 알려 주세요\n이름들에 이름 넣어\n끝\n이름들 말해줘\n",
        "set names to list of\nrepeat 3 times\nask name Tell me a name\nappend name to names\nend\nshow names\n",
    ),
    (
        "이야기 / a short story",
        "화면 지워\n줄 그어\n가운데 말해줘 겨울밤\n줄 그어\n"
        "천천히 말해줘 문이 천천히 열렸습니다.\n1초 기다려\n"
        "아주 천천히 말해줘 아무도 없었습니다.\n"
        "대답을 물어봐 나가 볼까요? 예 또는 아니오\n"
        "만약에 대답이 예와 같으면\n천천히 말해줘 눈이 내리고 있었습니다.\n"
        "아니면\n천천히 말해줘 문을 다시 닫았습니다.\n끝\n",
        "clear the screen\ndraw a line\nsay in the middle A winter night\ndraw a line\n"
        "say slowly The door opened slowly.\nwait 1 second\n"
        "say very slowly Nobody was there.\n"
        "ask answer Do you step outside? yes or no\n"
        "if answer equals yes\nsay slowly Snow was falling.\n"
        "else\nsay slowly You closed the door again.\nend\n",
    ),
    (
        "쿨타임 / a cooldown",
        "시간 재기 시작해\n문 쿨타임 2초 걸어\n만약 문 쿨타임이 남았으면\n"
        "아직 잠겨 있습니다 말해줘\n끝\n문 쿨타임 끝날때까지 기다려\n"
        "문이 열렸습니다 말해줘\n잰시간 말해줘\n",
        "start the timer\nput door on cooldown for 2 seconds\nwhen door is on cooldown\n"
        "show It is still locked\nend\nwait for door\n"
        "show The door opened\nshow elapsed\n",
    ),
    (
        "기다리기 / waiting",
        "출발합니다 말해줘\n3번 반복해\n1초 기다려\n하나 지났어요 말해줘\n끝\n끝났습니다 말해줘\n",
        "show Starting\nrepeat 3 times\nwait 1 second\nshow One second passed\nend\nshow Done\n",
    ),
    (
        "건너뛰기 / skipping a round",
        "점수는 0\n5번 반복해\n점수에 1 더해\n만약에 점수가 3과 같으면\n건너뛰어\n끝\n점수 말해줘\n끝\n",
        "set score to 0\nrepeat 5 times\nadd 1 to score\nif score equals 3\nskip\nend\nshow score\nend\n",
    ),
    (
        "조건 반복 / repeating while",
        "점수는 0\n점수가 3보다 작을 동안\n점수 말해줘\n점수에 1 더해\n끝\n",
        "set score to 0\nwhile score is less than 3\nshow score\nadd 1 to score\nend\n",
    ),
    (
        "두 조건 / two conditions",
        "준비는 참\n점수는 5\n만약 준비 그리고 점수가 2보다 크면 출발 말해줘\n",
        "set ready to True\nset score to 5\nif ready and score is greater than 2 then show Go\n",
    ),
    (
        "두 언어 섞기 / mixing the two languages",
        "이름을 물어봐 What is your name?\nshow 안녕하세요 이름!\n3번 반복해서 Welcome 말해줘\n",
        "ask name 이름이 뭐예요?\n안녕하세요 name! 말해줘\nrepeat 3 times and show 환영합니다\n",
    ),
    (
        "한 줄씩 Python으로 / growing into Python",
        "인사는 안녕하세요\nprint(인사)\nfor i in range(3):\n    인사 말해줘\n",
        "set greeting to Hello\nprint(greeting)\nfor i in range(3):\n    show greeting\n",
    ),
]


def compile_program(binary: Path, source: str) -> str:
    with tempfile.TemporaryDirectory() as folder:
        nme = Path(folder) / "program.nme"
        python = Path(folder) / "program.py"
        nme.write_text(source, encoding="utf-8")
        result = subprocess.run(
            [str(binary), "build", str(nme), "-o", str(python)],
            capture_output=True,
            text=True,
        )
        if result.returncode != 0:
            raise SystemExit(
                "build-ai-prompts: an example stopped compiling\n"
                f"{source}\n{result.stdout}{result.stderr}"
            )
        return python.read_text(encoding="utf-8").rstrip()


def compiled_examples(binary: Path) -> list[tuple[str, str, str, str, str]]:
    rows = []
    for name, korean, english in EXAMPLES:
        rows.append(
            (
                name,
                korean.rstrip(),
                compile_program(binary, korean),
                english.rstrip(),
                compile_program(binary, english),
            )
        )
    return rows


# ------------------------------------------------------------------ pieces


def sentence_tables(korean: bool) -> str:
    heads = {
        "say": ("출력 — 화면에 보여 주기", "Output — showing something"),
        "ask": ("입력 — 사람에게 묻기", "Input — asking the person"),
        "set": ("저장 — 값에 이름 붙이기", "Saving — giving a value a name"),
        "update": ("값 바꾸기 — 더하기·빼기·곱하기·나누기", "Changing a value — add, subtract, multiply, divide"),
        "wait": ("기다리기", "Waiting"),
        "times": ("정해진 횟수 반복", "Repeating a number of times"),
        "each": ("목록 하나씩 반복", "Repeating over a list"),
        "while": ("조건이 참인 동안 반복", "Repeating while a condition holds"),
        "when": ("조건 — 갈림길", "Conditions — choosing"),
        "compare": ("비교하는 말", "Comparison words"),
        "control": ("반복 안에서 멈추기·건너뛰기·닫기", "Stopping, skipping, and closing a block"),
        "list": ("목록 만들기와 넣기", "Making a list and adding to it"),
        "record": ("표 — 이름마다 값 하나씩", "Records — one value under each name"),
        "job": ("이름 붙인 일 — 프로그램 한 조각에 이름 붙이기",
                "Named jobs — giving a piece of program a name"),
        "text": ("글자 다루기 — 길이·대문자·소문자", "Working with text — length and case"),
        "numbers": ("숫자 나머지", "Number remainders"),
        "random": ("무작위", "Randomness"),
        "file": ("파일 읽기·쓰기", "Reading and writing files"),
        "slow": ("이야기 — 글자를 하나씩 내보내기", "Story — letters one at a time"),
        "story": ("이야기 묶음 — 여러 줄을 한 번에", "Story blocks — several lines at once"),
        "chance": ("확률 — 백 번에 몇 번", "Chance — how often out of a hundred"),
        "screen": ("화면 — 지우기·줄·상자·가운데", "Screen — clearing, ruling, boxing, centring"),
        "timer": ("시간 재기", "The stopwatch"),
        "cooldown": ("쿨타임", "Cooldowns"),
    }

    def sentence_only(rows):
        return [row for row in rows if row[0] == "문장형"]

    parts = []
    for key, rows in [
        ("say", S.SAY), ("ask", S.ASK), ("set", S.SET), ("update", S.UPDATE),
        ("wait", S.WAIT), ("times", S.TIMES), ("each", S.FOR_EACH),
        ("while", S.WHILE), ("when", S.WHEN),
    ]:
        title = heads[key][0] if korean else heads[key][1]
        parts.append(f"### {title}\n\n{S.level_table(sentence_only(rows), korean)}")
    title = heads["compare"][0] if korean else heads["compare"][1]
    parts.append(f"### {title}\n\n{S.compare_table(korean)}")
    for key, rows in [("control", S.LOOP_CONTROL), ("list", S.LISTS),
                      ("record", S.RECORDS), ("job", S.JOBS),
                      ("text", S.TEXT), ("numbers", S.NUMBERS),
                      ("slow", S.SLOW_TEXT), ("story", S.STORY),
                      ("chance", S.CHANCE), ("screen", S.SCREEN),
                      ("timer", S.TIMER), ("cooldown", S.COOLDOWN),
                      ("random", S.RANDOM), ("file", S.FILES)]:
        title = heads[key][0] if korean else heads[key][1]
        parts.append(f"### {title}\n\n{S.level_table(sentence_only(rows), korean)}")
    return "\n\n".join(parts)


def other_levels(korean: bool) -> str:
    def not_sentence(rows):
        return [row for row in rows if row[0] != "문장형"]

    groups = [
        ("출력", "Output", S.SAY), ("입력", "Input", S.ASK), ("저장", "Saving", S.SET),
        ("값 바꾸기", "Changing a value", S.UPDATE), ("기다리기", "Waiting", S.WAIT),
        ("횟수 반복", "Repeating a number of times", S.TIMES),
        ("목록 반복", "Repeating over a list", S.FOR_EACH),
        ("조건 반복", "Repeating while", S.WHILE), ("조건", "Conditions", S.WHEN),
        ("목록", "Lists", S.LISTS),
        ("표", "Records", S.RECORDS),
        ("이름 붙인 일", "Named jobs", S.JOBS),
        ("글자 다루기", "Working with text", S.TEXT),
        ("숫자 나머지", "Number remainders", S.NUMBERS),
        ("무작위", "Randomness", S.RANDOM),
        ("파일", "Files", S.FILES), ("모듈", "Modules", S.MODULES),
    ]
    parts = []
    for hangul, english, rows in groups:
        rows = not_sentence(rows)
        if not rows:
            continue
        parts.append(f"### {hangul if korean else english}\n\n{S.level_table(rows, korean)}")
    return "\n\n".join(parts)


RUN_KO = """## 설치하지 않고 바로 써 보기

프로그램을 쓰는 사람이 아무것도 설치하지 않아도 됩니다. **휴대폰에서도 됩니다.**

1. 브라우저에서 **needmoreeasy.com** 을 엽니다. (**nmelang.com** 을 쳐도 같은 곳으로 갑니다.)
2. 연습장 칸에 NME 프로그램을 붙여넣습니다.
3. 왼쪽에 쓰는 즉시 오른쪽에 그것이 되는 Python이 나타납니다.
4. **실행**을 누르면 그 자리에서 결과가 나옵니다. 프로그램이 사람에게 물어보는
   줄이 있으면 화면이 멈춰 서서 답을 기다립니다.

컴파일러와 파이썬 실행기가 **브라우저 안에서** 돌기 때문에, 쓴 프로그램은 그
탭 밖으로 나가지 않습니다. 브라우저에서 도는 실행기는 RustPython이라 파일·네트워크·
외부 꾸러미는 쓸 수 없습니다. 그 셋이 필요하면 내 컴퓨터에 설치해서 씁니다.

## 내 컴퓨터에 설치해서 쓰기

```sh
git clone --branch beta https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
nme --version
```

- `nme 실행 hello` — `hello.nme`를 실행합니다(`nme run hello`도 같습니다).
- `nme 검사 hello` — 실행하지 않고 문제만 봅니다. 아무 말이 없으면 정상입니다.
- `nme 빌드 hello -o hello.py` — 만들어진 Python을 파일로 받습니다.
- `nme ko E0102` — 오류 코드의 긴 설명을 한국어로 봅니다."""

RUN_EN = """## Trying it with nothing installed

The person writing the program does not have to install anything. **This works
on a phone.**

1. Open **needmoreeasy.com** in a browser. (**nmelang.com** goes to the same place.)
2. Paste an NME program into the playground box.
3. As you type on the left, the Python it becomes appears on the right.
4. Press **Run** and the result appears underneath. If the program has a line
   that asks the person something, it stops and waits for an answer.

The compiler and a Python engine both run **inside the browser**, so the program
never leaves that tab. The in-browser engine is RustPython, so files, the
network, and installed packages are not available there. Install NME locally if
the program needs those.

## Installing it locally

```sh
git clone --branch beta https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
nme --version
```

- `nme run hello` — runs `hello.nme`.
- `nme check hello` — reports problems without running. Silence means it is fine.
- `nme build hello -o hello.py` — writes out the Python it becomes.
- `nme en E0102` — the long explanation of an error code."""


RULES_KO = """## 답할 때 지켜야 할 것

1. **이 문서의 표에 나온 모양대로만 씁니다.** 없는 낱말을 지어내지 않습니다.
   문서에 없는 것을 해야 하면, 그 사실을 먼저 말하고 가장 가까운 표기를 제안합니다.
2. **문장형에서는 따옴표·쉼표·괄호·등호·콜론을 쓰지 않습니다.** 딱 두 군데만
   예외입니다: 파일 경로는 따옴표로 감싸고, 목록 항목은 쉼표로 나눕니다.
3. **한 줄에 한 가지 일만** 합니다. NME 문장 하나는 Python 한 줄이 됩니다.
4. **블록은 `끝`으로 닫습니다.** 들여쓰기를 써도 되지만, 처음 배우는 사람에게는
   `끝`이 더 쉽습니다. 반복 하나에 `끝` 하나이고, `만약에`·`아니면 만약에`·`아니면`
   한 묶음에도 `끝`은 맨 끝에 하나뿐입니다. 조건 뒤에 실행할 것을 한 줄로 붙여
   썼다면 `끝`이 필요 없습니다.
5. **이름은 미리 만들어 둡니다.** `점수에 1 더해`를 쓰기 전에 `점수는 0`이
   있어야 합니다. 문장 안에 값이 들어가는 이름도 마찬가지입니다.
6. **한국어와 영어를 섞어도 됩니다.** 한 줄 안에서도 됩니다. 선언할 것이 없습니다.
7. 프로그램을 보여 줄 때는 **NME 프로그램을 먼저**, 그것이 되는 Python은
   물어봤을 때만 보여 줍니다. 배우는 사람이 봐야 하는 것은 NME 쪽입니다.
8. 코딩을 처음 하는 사람에게 설명한다고 생각하고 씁니다. 전문 용어를 쓸 때는
   그 자리에서 한 줄로 풀어 줍니다."""

RULES_EN = """## Rules for your answers

1. **Use only the shapes shown in this document's tables.** Never invent a
   keyword. If something cannot be expressed, say so first and offer the nearest
   spelling.
2. **Sentence level uses no quotes, commas, parentheses, equals signs, or
   colons.** Exactly two exceptions: a file path is quoted, and list items are
   separated by commas.
3. **One thing per line.** One NME statement becomes one line of Python.
4. **Close a block with `end`.** Indentation also works, but `end` is easier for
   a first-time learner. One loop takes one `end`, and an `if` / `else if` /
   `else` chain takes a single `end` at the bottom. A body written on the same
   line as its condition needs no `end` at all.
5. **Create names before using them.** `add 1 to score` needs `set score to 0`
   above it. The same is true for a name you want substituted into a sentence.
6. **Korean and English may be mixed**, even on one line, with nothing to declare.
7. Show the **NME program first**; show the Python it becomes only when asked.
   The NME side is the one the learner needs to read.
8. Write as if explaining to someone who has never programmed. When a technical
   word is unavoidable, explain it in one line right where you use it."""


PITFALLS_KO = """## 문장 안에서 이름이 값으로 바뀌는 규칙

이 규칙 하나만 알면 문장형에서 헷갈릴 일이 거의 없습니다.

**출력 문장이나 질문 문구에 쓴 낱말이 앞에서 만들어 둔 이름과 똑같으면 그 자리에
값이 들어갑니다. 나머지 낱말은 쓴 그대로 찍힙니다.**

```text
안녕하세요 말해줘          → print("안녕하세요")
이름은 민수
안녕하세요 이름! 말해줘     → print("안녕하세요 " + str(이름) + "!")
```

여기서 나오는 실수는 하나뿐입니다. **문장 안에서 평범한 낱말로 쓰고 싶은 말을
이름으로도 쓰면 안 됩니다.** `점수는 3`을 만들어 둔 뒤 `당신의 점수 점수 말해줘`라고
쓰면 두 `점수`가 모두 값으로 바뀝니다. 이름을 `내점수`처럼 문장에 안 나올 말로
바꾸면 해결됩니다.

## 자주 걸리는 곳

- **`끝`은 갈림길 한 묶음에 하나입니다.** `만약에 …` · `아니면 만약에 …` ·
  `아니면`은 셋이 한 묶음이므로 맨 끝에 `끝` 하나만 씁니다. 두 개를 쓰면
  `error[E0101]`이 납니다. 반복 하나에는 `끝` 하나입니다.
- **이름은 동작을 나타내는 낱말과 겹치지 않게 짓습니다.** `말해`·`더해`·`반복`
  같은 낱말을 이름으로 쓰면 그 줄이 다른 뜻으로 읽힙니다.
- **빈 목록은 `친구들은 목록`입니다.** 뒤에 아무것도 쓰지 않으면 빈 목록이 됩니다.
  거기에 `친구들에 민수 넣어`로 하나씩 담습니다.
- **항목 이름이 `과`나 `와`로 끝나면 쉼표로 나눕니다.** `목록 사과, 바나나`처럼
  쉼표를 한 번이라도 쓰면 쉼표만 구분자로 봅니다.
- **`1초`도 `3초`도 됩니다.** 영어도 `wait 1 second`와 `wait 3 seconds` 둘 다 됩니다.
- **한 줄로 끝내려면 조건 뒤에 바로 씁니다.** `만약에 점수가 5보다 크면 성공 말해줘`
  처럼 쓰면 `끝`이 필요 없습니다. `건너뛰어`·`멈춰`·`점수에 1 더해`도 그 자리에
  올 수 있습니다."""

PITFALLS_EN = """## How a name becomes a value inside a sentence

This is the one rule that makes the sentence level predictable.

**A word in a message or a question is replaced by its value when a name of
exactly that spelling was created earlier. Every other word is printed as
written.**

```text
show hello                   → print("hello")
set name to Mina
show Hello name!             → print("Hello " + str(name) + "!")
```

Only one mistake follows from this. **Do not give a name a word you also want to
print as an ordinary word.** After `set score to 3`, the line
`show your score score` prints the number twice. Renaming it to `my_score` — a
word the message never uses — fixes it.

## Things that catch people out

- **One `end` closes a whole chain.** `if …`, `else if …` and `else` are one
  group, so they take a single `end` at the bottom. A second one raises
  `error[E0101]`. One loop takes one `end`.
- **Do not name something after an action word.** A name such as `show`, `add`
  or `repeat` makes the line read as that action instead.
- **An empty list is `set friends to list of` with nothing after it.** Fill it
  later with `append Mina to friends`.
- **Use commas when an item ends in a joining word.** Once a comma appears, only
  commas separate the items.
- **Both `wait 1 second` and `wait 3 seconds` work.**
- **A one-line body needs no `end`.** `if score is greater than 5 then show
  You won` is complete on its own, and `skip`, `break` and `add 1 to score` may
  all stand in that position."""


def what_is_ko() -> str:
    return f"""NME(NeedMoreEasy)는 **평범한 문장을 Python으로 바꾸는 작은 프로그래밍
언어**입니다. 한국어로 써도 되고 영어로 써도 되며, 한 줄 안에서 둘을 섞어도
됩니다. 이 문서가 설명하는 버전은 `{VERSION}`입니다.

**꼭 알아야 하는 규칙 세 가지.**

1. **올바른 Python은 언제나 Python입니다.** NME는 쉬운 표기를 찾기 전에 그 줄이
   올바른 Python인지 먼저 확인합니다. 그래서 Python 프로그램을 그대로 넣으면 한
   글자도 바뀌지 않습니다. 낱말 하나짜리 줄도 Python 이름으로 남습니다.
2. **NME 문장 하나는 Python 한 줄이 됩니다.** 줄 수가 변하지 않으므로 오류가 난
   줄 번호가 내가 쓴 파일과 정확히 맞습니다.
3. **문법 단계는 셋이고, 모드가 아닙니다.** 선언 없이 한 파일 안에서 섞입니다.
   이 문서는 그중 **문장형** 하나만 다룹니다. 코딩을 처음 하는 사람에게는 이것만
   있으면 충분합니다."""


def what_is_en() -> str:
    return f"""NME (NeedMoreEasy) is **a small programming language that turns ordinary
sentences into Python**. You can write it in English, in Korean, or mix the two
on one line. This document describes version `{VERSION}`.

**Three rules that matter.**

1. **Valid Python is always Python.** NME asks a real Python parser whether a
   line is valid before it looks for easier spellings, so a Python program passes
   through byte for byte. A one-word line also stays a Python name.
2. **One NME statement becomes one line of Python.** The line count never
   changes, so an error points at the line you actually wrote.
3. **There are three syntax levels, and they are not modes.** They mix in one
   file with nothing to declare. This document covers only the **sentence**
   level, which is all a first-time programmer needs."""


def header_ko(title: str, blurb: str) -> str:
    return f"""# {title}

> **쓰는 법.** 이 파일 전체를 복사해서 AI와의 대화 맨 앞에 붙여넣으세요.
> 그다음부터 "NME로 ○○하는 프로그램 만들어 줘"라고 말하면 됩니다.
> ChatGPT·Claude 같은 일반 대화창에서도 그대로 동작합니다.

{blurb}

---

당신은 지금부터 **NME(NeedMoreEasy) 프로그램을 써 주는 도우미**입니다.
아래는 NME에 대해 알아야 할 전부입니다. 여기 없는 문법은 존재하지 않습니다.

"""


def header_en(title: str, blurb: str) -> str:
    return f"""# {title}

> **How to use this.** Copy the whole file and paste it at the start of a chat
> with an AI. After that, ask for what you want: "write me an NME program that
> …". It works in an ordinary chat window such as ChatGPT or Claude.

{blurb}

---

From now on you are **an assistant that writes NME (NeedMoreEasy) programs**.
Everything you need to know about NME is below. Syntax that is not here does
not exist.

"""


def examples_section(rows, korean: bool) -> str:
    title = "## 예제 프로그램" if korean else "## Example programs"
    note = (
        "아래 프로그램은 모두 실제 컴파일러로 확인한 것입니다. "
        "오른쪽 Python은 컴파일러가 실제로 내놓은 글자 그대로입니다."
        if korean
        else "Every program below was compiled by the real compiler while this file was "
        "written. The Python shown is exactly what came out."
    )
    parts = [title, "", note, ""]
    for name, korean_source, korean_python, english_source, english_python in rows:
        source = korean_source if korean else english_source
        python = korean_python if korean else english_python
        parts.append(f"### {name}\n")
        parts.append("```nme\n" + source + "\n```\n")
        parts.append(("되는 Python:" if korean else "The Python it becomes:") + "\n")
        parts.append("```python\n" + python + "\n```\n")
    return "\n".join(parts)


def short_examples(rows, korean: bool, count: int) -> str:
    title = "## 짧은 예제" if korean else "## Short examples"
    parts = [title, ""]
    for name, korean_source, _, english_source, _ in rows[:count]:
        source = korean_source if korean else english_source
        parts.append(f"**{name}**\n")
        parts.append("```nme\n" + source + "\n```\n")
    return "\n".join(parts)


def checklist_ko(deep: bool) -> str:
    extra = "\n- 초급·고급 표기를 쓴 자리가 있다면, 문장형으로는 쓸 수 없는 이유를 한 줄로 밝혔는가?" if deep else ""
    return f"""## 답을 보내기 전 확인

- 표에 없는 낱말을 쓰지 않았는가?
- 문장형 줄에 따옴표·괄호·등호·콜론이 들어가지 않았는가?(파일 경로와 목록 쉼표는 예외)
- 반복마다, 그리고 갈림길 묶음마다 `끝`이 하나씩인가?(한 줄로 쓴 것은 필요 없음)
- 쓰기 전에 만들어 둔 이름만 쓰고 있는가?
- 코딩을 모르는 사람이 읽어도 무슨 뜻인지 알 수 있는가?{extra}"""


def checklist_en(deep: bool) -> str:
    extra = "\n- If a beginner or advanced spelling was used, did you say in one line why the sentence level could not do it?" if deep else ""
    return f"""## Before you send an answer

- Did you use only spellings from the tables?
- Do the sentence-level lines avoid quotes, parentheses, equals signs, and
  colons? (A file path and the commas between list items are the exceptions.)
- Does each loop, and each whole `if`/`else` chain, have exactly one `end`?
- Does every name exist before it is used?
- Would someone who has never programmed understand the answer?{extra}"""


ADVANCED_KO = """## 고급 단계 — 그냥 Python입니다

고급 단계에는 배울 문법이 없습니다. **올바른 Python은 무엇이든 그대로 통과합니다.**
함수(`def`), 클래스, `import`, 예외 처리, `for`, 컴프리헨션, PyPI에서 받은 꾸러미까지
전부 그대로 씁니다. NME는 그런 줄을 한 글자도 바꾸지 않습니다.

그래서 배우는 사람은 **한 줄씩** 옮겨 갈 수 있습니다. 같은 파일 안에서 오늘은
`점수는 0`, 내일은 `점수 = 0`으로 바꿔 써도 프로그램은 그대로 돕니다. 언어를
갈아타며 프로젝트를 다시 시작할 일이 없습니다.

⚠ 한 가지만 조심합니다. 낱말 하나짜리 줄과 `이름 = 값` 모양은 이미 올바른
Python이므로, NME 문장으로 읽히지 않습니다."""

ADVANCED_EN = """## The advanced level — it is just Python

There is no syntax to learn at the advanced level. **Any valid Python passes
through unchanged**: functions (`def`), classes, `import`, exception handling,
`for`, comprehensions, packages installed from PyPI. NME does not alter one byte
of such a line.

That is what lets a learner move across **one line at a time**. In the same file,
`set score to 0` today and `score = 0` tomorrow both work, so nobody has to
restart a project in a different language.

⚠ One thing to watch: a one-word line, and anything shaped like `name = value`,
is already valid Python and will not be read as an NME sentence."""


MODULES_KO = """## 딸려 오는 도구 일곱 가지

`랜덤 사용` 한 줄이면 무작위 도구가, `파일 사용` 한 줄이면 파일 도구가,
`영지식 사용` 한 줄이면 영지식 증명 도구가 준비됩니다. 한 번 부르면 한국어
이름과 영어 이름이 **둘 다** 생깁니다.

| 무작위 | 파일 | 뜻 |
| --- | --- | --- |
| `랜덤정수(1, 6)` / `random_number(1, 6)` | `파일읽기(경로)` / `file_read(path)` | 주사위 굴리기 · 파일 읽기 |
| `랜덤선택(목록)` / `random_pick(values)` | `파일쓰기(경로, 내용)` / `file_write(path, text)` | 하나 고르기 · 파일 쓰기 |
| `섞기(목록)` / `shuffle(values)` | `json읽기(경로)` / `json_load(path)` | 섞기 · JSON 읽기 |
| | `json저장(경로, 값)` / `json_save(path, value)` | JSON 쓰기 |

`랜덤`은 비밀번호나 보안에 쓰면 안 됩니다.

`목록 사용`, `글자 사용`, `수학 사용`, `날짜 사용` 네 가지가 더 있습니다. 안에 든
것은 전부 평범한 Python 기본 기능이라 브라우저에서도 그대로 돕니다.

| 목록 | 글자 | 수학 |
| --- | --- | --- |
| `개수(값들)` / `count(values)` | `대문자(글)` / `upper(text)` | `제곱근(x)` / `root(x)` |
| `정렬(값들)` / `sort(values)` | `소문자(글)` / `lower(text)` | `반올림(x, 자리)` / `round_to(x, places)` |
| `뒤집기(값들)` / `reverse(values)` | `공백없애기(글)` / `trim(text)` | `원주율` / `pi` |
| `빼기(값들, x)` / `remove(values, x)` | `나누기(글, 구분자)` / `split(text, sep)` | `거듭제곱(x, y)` / `power(x, y)` |
| `첫번째(값들)` / `first(values)` | `합치기(구분자, 값들)` / `join(sep, values)` | `절댓값(x)` / `absolute(x)` |
| `마지막(값들)` / `last(values)` | `바꾸기(글, a, b)` / `replace(text, a, b)` | `내림(x)` / `floor(x)` |
| `합계(값들)` / `sum(values)` | `로시작(글, a)` / `starts_with(text, a)` | `올림(x)` / `ceil(x)` |
| `최대(값들)` / `largest(values)` | `길이(글)` / `length(text)` | |
| `최소(값들)` / `smallest(values)` | | |

| 날짜 | 뜻 |
| --- | --- |
| `오늘()` / `today()` | 오늘 날짜, `"2026-08-19"` 같은 글자 |
| `지금()` / `now()` | 지금 시각, `"09:06"` 같은 글자 |
| `올해()` / `year()` | 올해 (숫자) |
| `이번달()` / `month()` | 이번 달 (숫자) |
| `오늘일자()` / `day_of_month()` | 오늘이 며칟날인지 (숫자) |
| `요일()` / `weekday()` | 한국어 이름은 `수요일`, 영어 이름은 `Wednesday` |
| `며칠뒤(n)` / `days_after(n)` | n일 뒤 날짜. 음수를 넣으면 n일 전 |

`정렬`·`뒤집기`·`빼기`는 새 목록을 돌려주고 원래 목록은 그대로 둡니다.
`목록`·`글자`·`수학`·`날짜`는 평범한 낱말이라, `사용` 바로 옆에 있고 그 줄에 다른
낱말이 없을 때만 모듈로 읽습니다. `요일`과 `weekday`는 두 이름이 서로 다른 값을
갖는 유일한 자리입니다. 요일 이름은 낱말이라서 어느 한 언어에 속하고, 적은 이름이
답의 언어를 정합니다. 브라우저가 Python에 주는 시계는 UTC입니다."""

MODULES_EN = """## Seven bundled toolboxes

One line of `use random` brings in the random tools, `use file` the file tools,
and `use zero_knowledge` a Schnorr proof-of-knowledge reference implementation.
One import binds **both** the English and the Korean names.

| Random | File | Meaning |
| --- | --- | --- |
| `random_number(1, 6)` / `랜덤정수(1, 6)` | `file_read(path)` / `파일읽기(경로)` | roll a die · read a file |
| `random_pick(values)` / `랜덤선택(목록)` | `file_write(path, text)` / `파일쓰기(경로, 내용)` | pick one · write a file |
| `shuffle(values)` / `섞기(목록)` | `json_load(path)` / `json읽기(경로)` | shuffle · read JSON |
| | `json_save(path, value)` / `json저장(경로, 값)` | write JSON |

`random` is not suitable for passwords or any security decision.

Four more: `use list`, `use text`, `use math` and `use date`. Everything inside
them is a plain Python builtin, so they run in the browser as they stand.

| List | Text | Maths |
| --- | --- | --- |
| `count(values)` / `개수(값들)` | `upper(text)` / `대문자(글)` | `root(x)` / `제곱근(x)` |
| `sort(values)` / `정렬(값들)` | `lower(text)` / `소문자(글)` | `round_to(x, places)` / `반올림(x, 자리)` |
| `reverse(values)` / `뒤집기(값들)` | `trim(text)` / `공백없애기(글)` | `pi` / `원주율` |
| `remove(values, x)` / `빼기(값들, x)` | `split(text, sep)` / `나누기(글, 구분자)` | `power(x, y)` / `거듭제곱(x, y)` |
| `first(values)` / `첫번째(값들)` | `join(sep, values)` / `합치기(구분자, 값들)` | `absolute(x)` / `절댓값(x)` |
| `last(values)` / `마지막(값들)` | `replace(text, a, b)` / `바꾸기(글, a, b)` | `floor(x)` / `내림(x)` |
| `sum(values)` / `합계(값들)` | `starts_with(text, a)` / `로시작(글, a)` | `ceil(x)` / `올림(x)` |
| `largest(values)` / `최대(값들)` | `length(text)` / `길이(글)` | |
| `smallest(values)` / `최소(값들)` | | |

| Date | Meaning |
| --- | --- |
| `today()` / `오늘()` | today's date, text such as `"2026-08-19"` |
| `now()` / `지금()` | the time now, text such as `"09:06"` |
| `year()` / `올해()` | which year it is (a number) |
| `month()` / `이번달()` | which month it is (a number) |
| `day_of_month()` / `오늘일자()` | which day of the month it is (a number) |
| `weekday()` / `요일()` | `Wednesday` in English, `수요일` in Korean |
| `days_after(n)` / `며칠뒤(n)` | the date n days from today; a negative n is days before |

`sort`, `reverse` and `remove` hand back a new list and leave the original
alone. `list`, `text`, `math` and `date` are ordinary words, so they name a
module only when they stand beside `use` and nothing else is left over on the
line. `weekday` and `요일` are the one place where two names hold different
values: a weekday name is a word, so it belongs to a language, and the name you
write chooses the language of the answer. The clock a browser hands Python is
UTC."""


def build(binary: Path) -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    rows = compiled_examples(binary)

    spelling_ko = S.spelling_table(True)
    spelling_en = S.spelling_table(False)
    codes_ko = S.diagnostics_table(True)
    codes_en = S.diagnostics_table(False)

    # ---------------------------------------------------------- prompt one
    sentence_ko = (
        header_ko(
            "NME 문장형 프롬프트 (추천)",
            "**추천하는 프롬프트입니다.** 코딩을 처음 하는 사람에게 필요한 것은 이것 하나뿐입니다. "
            "문장형 문법만 100% 담았고, 초급·고급 문법은 넣지 않았습니다.",
        )
        + what_is_ko()
        + "\n\n## 문장형 문법 전부\n\n"
        + sentence_tables(True)
        + "\n\n### 동작을 나타내는 낱말 전부\n\n"
        + spelling_ko
        + "\n\n같은 칸에 있는 낱말은 서로 바꿔 써도 뜻이 같습니다. 이 표는 **동작을 나타내는 "
          "낱말**만 모은 것입니다. `부터`·`까지`·`초`·`마다`·`보다 크면`·`랜덤정수`처럼 문장을 "
          "이루는 나머지 말은 위 표들에 나온 모양 그대로 쓰면 됩니다.\n\n"
        + PITFALLS_KO
        + "\n\n"
        + short_examples(rows, True, 6)
        + "\n"
        + RUN_KO
        + "\n\n"
        + RULES_KO
        + "\n\n"
        + checklist_ko(False)
        + "\n"
    )
    sentence_en = (
        header_en(
            "NME sentence-level prompt (recommended)",
            "**This is the one to use.** It is all a first-time programmer needs: "
            "100% of the sentence syntax and none of the beginner or advanced syntax.",
        )
        + what_is_en()
        + "\n\n## All of the sentence syntax\n\n"
        + sentence_tables(False)
        + "\n\n### Every action word\n\n"
        + spelling_en
        + "\n\nWords in the same cell mean the same thing. This table lists the **action "
          "words** only; the rest of a sentence — `from`, `to`, `seconds`, `for each`, "
          "`greater than`, `random number` — is written exactly as the tables above "
          "show it.\n\n"
        + PITFALLS_EN
        + "\n\n"
        + short_examples(rows, False, 6)
        + "\n"
        + RUN_EN
        + "\n\n"
        + RULES_EN
        + "\n\n"
        + checklist_en(False)
        + "\n"
    )

    # ---------------------------------------------------------- prompt two
    all_ko = (
        header_ko(
            "NME 전체 문법 프롬프트",
            "문장형에 **초급·고급 문법까지** 더한 프롬프트입니다. Python을 아는 사람과 "
            "일할 때, 또는 문장형만으로는 표현되지 않는 프로그램을 만들 때 쓰세요.",
        )
        + what_is_ko()
        + "\n\n## 문장형 문법 전부\n\n"
        + sentence_tables(True)
        + "\n\n## 초급 문법\n\n"
        + other_levels(True)
        + "\n\n"
        + ADVANCED_KO
        + "\n\n"
        + MODULES_KO
        + "\n\n### 쓸 수 있는 낱말 전부\n\n"
        + spelling_ko
        + "\n\n## 오류 코드\n\n"
        + codes_ko
        + "\n\n"
        + PITFALLS_KO
        + "\n\n"
        + short_examples(rows, True, 6)
        + "\n"
        + RUN_KO
        + "\n\n"
        + RULES_KO
        + "\n\n"
        + checklist_ko(True)
        + "\n"
    )
    all_en = (
        header_en(
            "NME full-syntax prompt",
            "The sentence syntax **plus the beginner and advanced levels**. Use this "
            "when working with someone who knows Python, or for a program the "
            "sentence level cannot express.",
        )
        + what_is_en()
        + "\n\n## All of the sentence syntax\n\n"
        + sentence_tables(False)
        + "\n\n## Beginner syntax\n\n"
        + other_levels(False)
        + "\n\n"
        + ADVANCED_EN
        + "\n\n"
        + MODULES_EN
        + "\n\n### Every word you may use\n\n"
        + spelling_en
        + "\n\n## Error codes\n\n"
        + codes_en
        + "\n\n"
        + PITFALLS_EN
        + "\n\n"
        + short_examples(rows, False, 6)
        + "\n"
        + RUN_EN
        + "\n\n"
        + RULES_EN
        + "\n\n"
        + checklist_en(True)
        + "\n"
    )

    # -------------------------------------------------------- prompt three
    complete_ko = (
        header_ko(
            "NME 전체 문법 + 예제 프롬프트",
            "전체 문법에 **예제 프로그램까지** 더한 가장 긴 프롬프트입니다. "
            "대화창에 붙여넣을 수 있는 길이라면 이것을 쓰는 편이 결과가 가장 정확합니다.",
        )
        + what_is_ko()
        + "\n\n## 문장형 문법 전부\n\n"
        + sentence_tables(True)
        + "\n\n## 초급 문법\n\n"
        + other_levels(True)
        + "\n\n"
        + ADVANCED_KO
        + "\n\n"
        + MODULES_KO
        + "\n\n### 쓸 수 있는 낱말 전부\n\n"
        + spelling_ko
        + "\n\n## 오류 코드\n\n"
        + codes_ko
        + "\n\n"
        + PITFALLS_KO
        + "\n\n"
        + examples_section(rows, True)
        + "\n"
        + RUN_KO
        + "\n\n"
        + RULES_KO
        + "\n\n"
        + checklist_ko(True)
        + "\n"
    )
    complete_en = (
        header_en(
            "NME full-syntax prompt with examples",
            "The full syntax **plus worked example programs**. This is the longest "
            "of the three; if it fits in the chat window, it gives the most "
            "accurate results.",
        )
        + what_is_en()
        + "\n\n## All of the sentence syntax\n\n"
        + sentence_tables(False)
        + "\n\n## Beginner syntax\n\n"
        + other_levels(False)
        + "\n\n"
        + ADVANCED_EN
        + "\n\n"
        + MODULES_EN
        + "\n\n### Every word you may use\n\n"
        + spelling_en
        + "\n\n## Error codes\n\n"
        + codes_en
        + "\n\n"
        + PITFALLS_EN
        + "\n\n"
        + examples_section(rows, False)
        + "\n"
        + RUN_EN
        + "\n\n"
        + RULES_EN
        + "\n\n"
        + checklist_en(True)
        + "\n"
    )

    files = {
        "nme-sentence.ko.md": sentence_ko,
        "nme-sentence.md": sentence_en,
        "nme-all-levels.ko.md": all_ko,
        "nme-all-levels.md": all_en,
        "nme-complete.ko.md": complete_ko,
        "nme-complete.md": complete_en,
    }
    for name, text in files.items():
        (OUT / name).write_text(text, encoding="utf-8")
        print(f"wrote docs/prompts/{name} ({len(text):,} characters)")


if __name__ == "__main__":
    build(compiler())
