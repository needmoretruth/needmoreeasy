#!/usr/bin/env python3
"""Writes docs/syntax.md and docs/syntax.ko.md.

The two files are twins: every table row is written once here and emitted into
both, so the Korean reference can never be less complete than the English one
again. The prose around the tables is the only thing that differs.

Run after changing any keyword list in the compiler:

    python scripts/build-syntax-reference.py

`scripts/check-syntax-reference.py` fails the build when a spelling that the
compiler accepts is missing from these files.
"""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PARSER = (ROOT / "crates/nme-core/src/parser.rs").read_text(encoding="utf-8")
SYNTAX = (ROOT / "crates/nme-core/src/syntax.rs").read_text(encoding="utf-8")
DIAGNOSTICS = (ROOT / "crates/nme-core/src/diagnostics.rs").read_text(encoding="utf-8")


def words(name: str) -> list[str]:
    """Every string in one `const NAME: &[&str] = &[...]` list."""
    for source in (PARSER, SYNTAX):
        match = re.search(
            rf"const\s+{name}\s*:\s*&\[&str\]\s*=\s*&\[(.*?)\];", source, re.S
        )
        if match:
            return re.findall(r'"([^"]*)"', match.group(1))
    raise SystemExit(f"build-syntax-reference: no word list named {name}")


def code_table() -> list[tuple[str, str, str]]:
    """(code, English one-liner, Korean one-liner) for every diagnostic."""
    rows = []
    for match in re.finditer(
        r'\(\s*"(E\d{4})",\s*"((?:[^"\\]|\\.)*)",\s*"((?:[^"\\]|\\.)*)",',
        DIAGNOSTICS,
    ):
        rows.append(match.groups())
    seen, unique = set(), []
    for row in rows:
        if row[0] in seen:
            continue
        seen.add(row[0])
        unique.append(row)
    return sorted(unique)


def spellings(items: list[str]) -> str:
    return " · ".join(f"`{item}`" for item in items)


# --------------------------------------------------------------- statements
#
# Each entry is one row of the big per-statement tables. `en` and `ko` are the
# NME line a reader would write; `py` is exactly what the compiler emits.

SAY = [
    ("문장형", "Sentence", "show Hello world!", "안녕하세요! 말해줘", 'print("Hello world!")', 'print("안녕하세요!")'),
    ("문장형", "Sentence", "Hello world show", "보여줘 안녕하세요", 'print("Hello world")', 'print("안녕하세요")'),
    ("문장형", "Sentence", "Hello everyone!", "오늘도 반가워요!", 'print("Hello everyone!")', 'print("오늘도 반가워요!")'),
    ("문장형", "Sentence", "show Hello name!", "안녕하세요 이름! 말해줘", 'print("Hello " + str(name) + "!")', 'print("안녕하세요 " + str(이름) + "!")'),
    ("초급", "Beginner", 'say "Hello"', '말해 "안녕"', 'print("Hello")', 'print("안녕")'),
    ("초급", "Beginner", "say total + 1", "말해 총합 + 1", "print(total + 1)", 'print(총합 + 1)'),
    ("고급", "Advanced", 'print("Hello")', 'print("안녕")', "unchanged"),
]

ASK = [
    ("문장형", "Sentence", "ask name What is your name?", "이름을 물어봐 이름이 뭐예요?", 'name = input("What is your name?" + " ")', '이름 = input("이름이 뭐예요?" + " ")'),
    ("문장형", "Sentence", "ask number age How old are you?", "나이를 숫자로 물어봐 몇 살이에요?", 'age = int(input("How old are you?" + " "))', '나이 = int(input("몇 살이에요?" + " "))'),
    ("문장형", "Sentence", "What is your name?", "이름이 뭐예요?", 'name = input("What is your name?" + " ")', '이름 = input("이름이 뭐예요?" + " ")'),
    ("문장형", "Sentence", "How old are you?", "몇 살이에요?", 'age = int(input("How old are you?" + " "))', '나이 = int(input("몇 살이에요?" + " "))'),
    ("문장형", "Sentence", "name ask", "이름을 물어봐", "name = input()", '이름 = input()'),
    ("초급", "Beginner", 'ask name, "Name? "', '물어봐 이름, "이름? "', 'name = input("Name? ")', '이름 = input("이름? ")'),
    ("고급", "Advanced", "name = input()", "이름 = input()", "unchanged"),
]

SET = [
    ("문장형", "Sentence", "set greeting to Hello", "인사는 안녕하세요", 'greeting = "Hello"', '인사 = "안녕하세요"'),
    ("문장형", "Sentence", "set answer to 7", "정답은 7", "answer = 7", '정답 = 7'),
    ("문장형", "Sentence", "greeting save Hello", "인사 저장 안녕하세요", 'greeting = "Hello"', '인사 = "안녕하세요"'),
    ("문장형", "Sentence", "remember score to 0", "점수를 0으로 설정해", "score = 0", '점수 = 0'),
    ("문장형", "Sentence", "set score to 0.", "점수는 0이다", "score = 0", '점수 = 0'),
    ("초급", "Beginner", "save total to 1 + 2", "저장 총합 1 + 2", "total = 1 + 2", '총합 = 1 + 2'),
    ("고급", "Advanced", "greeting = 'Hello'", "인사 = '안녕'", "unchanged"),
]

UPDATE = [
    ("문장형", "Sentence", "score add 1", "점수에 1 더해", "score = score + 1", '점수 = 점수 + 1'),
    ("문장형", "Sentence", "add 1 to score", "점수에 1 더해줘", "score = score + 1", '점수 = 점수 + 1'),
    ("문장형", "Sentence", "score increase by 1", "점수 1 올려", "score = score + 1", '점수 = 점수 + 1'),
    ("문장형", "Sentence", "to score add 1", "1을 점수에 더해", "score = score + 1", '점수 = 점수 + 1'),
    ("문장형", "Sentence", "subtract 1 from score", "점수에서 1 빼줘", "score = score - 1", '점수 = 점수 - 1'),
    ("문장형", "Sentence", "multiply score by 2", "점수에 2 곱해", "score = score * 2", '점수 = 점수 * 2'),
    ("문장형", "Sentence", "divide score by 2", "점수를 2로 나눠", "score = score / 2", '점수 = 점수 / 2'),
    ("문장형", "Sentence", "subtract 1 + 2 from score", "점수에서 1 + 2 빼줘", "score = score - (1 + 2)", '점수 = 점수 - (1 + 2)'),
    ("고급", "Advanced", "score += 1", "점수 += 1", "unchanged"),
]

WAIT = [
    ("문장형", "Sentence", "wait 3 seconds", "3초 기다려", '__import__("time").sleep(3)'),
    ("문장형", "Sentence", "pause 3", "3초 쉬어", '__import__("time").sleep(3)'),
    ("문장형", "Sentence", "wait 1 second", "1초 기다려", '__import__("time").sleep(1)'),
    ("문장형", "Sentence", "wait two seconds", "이초 기다려", '__import__("time").sleep(2)'),
    ("문장형", "Sentence", "wait for 5 seconds", "5 초 기다려주세요", '__import__("time").sleep(5)'),
    ("문장형", "Sentence", "sleep pause_length", "쉬는시간 기다려", '__import__("time").sleep(pause_length)', '__import__("time").sleep(쉬는시간)'),
    ("고급", "Advanced", "import time; time.sleep(3)", "import time; time.sleep(3)", "unchanged"),
]

TIMES = [
    ("문장형", "Sentence", "repeat 3 times and show Again", "3번 반복해서 다시 말해줘", 'for _ in range(3): print("Again")', 'for _ in range(3): print("다시")'),
    ("문장형", "Sentence", "3 times Welcome", "3번 환영합니다", 'for _ in range(3): print("Welcome")', 'for _ in range(3): print("환영합니다")'),
    ("문장형", "Sentence", "repeat three times and show Again", "세 번 반복해서 다시 말해줘", 'for _ in range(3): print("Again")', 'for _ in range(3): print("다시")'),
    ("문장형", "Sentence", "repeat 3 rounds and show Again", "3회 반복해서 다시 말해줘", 'for _ in range(3): print("Again")', 'for _ in range(3): print("다시")'),
    ("문장형", "Sentence", "repeat 3 times … end", "3번 반복해 … 끝", "for _ in range(3):", 'for _ in range(3):'),
    ("초급", "Beginner", '3 times: say "Hi"', '3번: 말해 "안녕"', 'for _ in range(3): print("Hi")', 'for _ in range(3): print("안녕")'),
    ("고급", "Advanced", "for i in range(3):", "for i in range(3):", "unchanged"),
]

FOR_EACH = [
    ("문장형", "Sentence", "for each friend in friends", "친구들의 친구마다 반복해", "for friend in friends:", 'for 친구 in 친구들:'),
    ("문장형", "Sentence", "for each friend in friends and show friend", "친구들의 친구마다 반복해서 친구 말해줘", "for friend in friends: print(friend)", 'for 친구 in 친구들: print(친구)'),
    ("문장형", "Sentence", "repeat for each name in names", "이름들에서 이름마다 반복해", "for name in names:", 'for 이름 in 이름들:'),
    ("문장형", "Sentence", "foreach friend in friends", "친구들의 친구 마다 반복해", "for friend in friends:", 'for 친구 in 친구들:'),
    ("초급", "Beginner", "for each friend in friends:", "친구들의 친구마다:", "for friend in friends:", 'for 친구 in 친구들:'),
    ("고급", "Advanced", "for friend in friends:", "for 친구 in 친구들:", "unchanged"),
]

WHILE = [
    ("문장형", "Sentence", "while score is less than 3", "점수가 3보다 작을 동안", "while (score < 3):", 'while (점수 < 3):'),
    ("문장형", "Sentence", "while ready and waiting", "준비 그리고 대기 동안", "while (ready and waiting):", 'while (준비 and 대기):'),
    ("문장형", "Sentence", "while score is greater than 0", "점수가 0보다 큰 동안", "while (score > 0):", 'while (점수 > 0):'),
    ("문장형", "Sentence", "while ready then show working", "준비하는동안 확인 말해줘", 'while (ready): print("working")', 'while (준비): print("확인")'),
    ("초급", "Beginner", "while score < 3", "동안 점수 < 3", "while (score < 3):", 'while (점수 < 3):'),
    ("고급", "Advanced", "while score < 3:", "while 점수 < 3:", "unchanged"),
]

WHEN = [
    ("문장형", "Sentence", "if score is greater than 10 then show You won", "만약에 점수가 10보다 크면 성공 말해줘", 'if (score > 10): print("You won")', 'if (점수 > 10): print("성공")'),
    ("문장형", "Sentence", "if name exists", "만약에 이름이 있으면", "if (name):", 'if (이름):'),
    ("문장형", "Sentence", "if score > 10 then show You won", "만약 점수 > 10 이면 성공 말해줘", 'if (score > 10): print("You won")', 'if (점수 > 10): print("성공")'),
    ("문장형", "Sentence", "if score is above 10 then show You won", "만약에 점수가 10 초과면 성공 말해줘", 'if (score > 10): print("You won")', 'if (점수 > 10): print("성공")'),
    ("문장형", "Sentence", "score is greater than 5 then show high", "점수가 5보다 크면 높음 말해줘", 'if (score > 5): print("high")', 'if (점수 > 5): print("높음")'),
    ("문장형", "Sentence", "else if score equals 0", "아니면 만약에 점수가 0과 같으면", "elif (score == 0):", 'elif (점수 == 0):'),
    ("문장형", "Sentence", "else", "아니면", "else:", 'else:'),
    ("초급", "Beginner", 'when score == 1: say "one"', '만약 점수 == 1: 말해 "하나"', 'if (score == 1): print("one")', 'if (점수 == 1): print("하나")'),
    ("고급", "Advanced", "if score == 1:", "if 점수 == 1:", "unchanged"),
]

COMPARE = [
    ("if name exists", "만약에 이름이 있으면", "name", "참인 값 / truthy", "이름"),
    ("if name missing", "만약에 이름이 없으면", "not (name)", "거짓인 값 / falsey", "not (이름)"),
    ("if score equals 10", "만약에 점수가 10과 같으면", "score == 10", "==", "점수 == 10"),
    ("if score is not equal to 10", "만약에 점수가 10과 같지 않으면", "score != 10", "!=", "점수 != 10"),
    ("if score is greater than 10", "만약에 점수가 10보다 크면", "score > 10", ">", "점수 > 10"),
    ("if score is less than 10", "만약에 점수가 10보다 작으면", "score < 10", "<", "점수 < 10"),
    ("if score is greater than or equal to 10", "만약에 점수가 10보다 크거나 같으면", "score >= 10", ">=", "점수 >= 10"),
    ("if score is less than or equal to 10", "만약에 점수가 10보다 작거나 같으면", "score <= 10", "<=", "점수 <= 10"),
    ("if ready and score > 2", "만약 준비 그리고 점수가 2보다 크면", "ready and score > 2", "and / 그리고", "준비 and 점수 > 2"),
    ("if ready or waiting", "만약 준비 또는 대기", "ready or waiting", "or / 또는", "준비 or 대기"),
]

LOOP_CONTROL = [
    ("문장형", "Sentence", "break", "멈춰", "break"),
    ("문장형", "Sentence", "break here", "여기서 멈춰", "break"),
    ("문장형", "Sentence", "stop", "그만해", "break"),
    ("문장형", "Sentence", "exit loop", "정지해", "break"),
    ("문장형", "Sentence", "quit", "멈춰줘", "break"),
    ("문장형", "Sentence", "skip", "건너뛰어", "continue"),
    ("문장형", "Sentence", "keep going", "계속해", "continue"),
    ("문장형", "Sentence", "end", "끝", "(closes the block)"),
    ("문장형", "Sentence", "finish", "종료", "(closes the block)"),
]

LISTS = [
    ("문장형", "Sentence", "set friends to list of Mina, Ada", "친구들은 목록 민수, 지안", 'friends = ["Mina", "Ada"]', '친구들 = ["민수", "지안"]'),
    ("문장형", "Sentence", "set friends to list of Mina and Ada", "친구들은 목록 민수와 지안", 'friends = ["Mina", "Ada"]', '친구들 = ["민수", "지안"]'),
    ("문장형", "Sentence", "set scores to list of 1, 2, 3", "점수들은 목록 1, 2, 3", "scores = [1, 2, 3]", '점수들 = [1, 2, 3]'),
    ("문장형", "Sentence", "set friends to list of", "친구들은 목록", "friends = []", '친구들 = []'),
    ("문장형", "Sentence", "append Mina to friends", "친구들에 민수 넣어", 'friends.append("Mina")', '친구들.append("민수")'),
    ("문장형", "Sentence", "push Mina to friends", "친구들에 민수 추가해", 'friends.append("Mina")', '친구들.append("민수")'),
    ("문장형", "Sentence", "add Mina to friends", "친구들에 민수 더해", 'friends.append("Mina")', '친구들.append("민수")'),
    ("문장형", "Sentence", "to friends append Mina", "민수를 친구들에 넣어", 'friends.append("Mina")', '친구들.append("민수")'),
    ("고급", "Advanced", 'friends = ["Mina"]', '친구들 = ["민수"]', "unchanged"),
]

RANDOM = [
    ("문장형", "Sentence", "set die to random number from 1 to 6", "주사위는 1부터 6까지 랜덤정수", 'die = __import__("random").randint(1, 6)', '주사위 = __import__("random").randint(1, 6)'),
    ("문장형", "Sentence", "set color to pick from red or green", "색은 빨강 또는 초록 중에서 랜덤선택", 'color = __import__("random").choice(("red", "green",))', '색 = __import__("random").choice(("빨강", "초록",))'),
    ("문장형", "Sentence", "set color to choose from red or green", "색은 빨강 또는 초록 중에서 뽑아", 'color = __import__("random").choice(("red", "green",))', '색 = __import__("random").choice(("빨강", "초록",))'),
    ("초급", "Beginner", "use random", "랜덤 사용", "(binds the random helpers)"),
    ("초급", "Beginner", "say random_number(1, 6)", "말해 랜덤정수(1, 6)", "print(random_number(1, 6))", 'print(랜덤정수(1, 6))'),
]

FILES = [
    ("문장형", "Sentence", 'read "notes.txt" into memo', 'memo에 "notes.txt" 읽어서', 'memo = __import__("pathlib").Path("notes.txt").read_text()', 'memo = __import__("pathlib").Path("notes.txt").read_text()'),
    ("문장형", "Sentence", 'write "hello" to "out.txt"', '"out.txt" 파일에 "hello"를 저장해', '__import__("pathlib").Path("out.txt").write_text("hello")', '__import__("pathlib").Path("out.txt").write_text("hello")'),
    ("초급", "Beginner", "use file", "파일 사용", "(binds the file helpers)"),
    ("초급", "Beginner", 'say file_read("notes.txt")', '말해 파일읽기("notes.txt")', 'print(file_read("notes.txt"))', 'print(파일읽기("notes.txt"))'),
]

MODULES = [
    ("초급", "Beginner", "use random", "랜덤 사용", "(binds random, random_number, random_pick, shuffle and their Korean twins)"),
    ("초급", "Beginner", "use file", "파일 사용", "(binds file_read, file_write, json_load, json_save and their Korean twins)"),
    ("초급", "Beginner", "use zero_knowledge", "영지식 사용", "(binds zk_secret, zk_public, zk_nizk_prove, … and their Korean twins)"),
    ("초급", "Beginner", "use random latest", "랜덤 사용 최신", "(the newest bundled adapter)"),
    ("초급", "Beginner", 'use random version "0.0.1"', '랜덤 사용 버전 "0.0.1"', "(that exact adapter)"),
    ("고급", "Advanced", 'from "helper.nme" import greet', 'from "helper.nme" import greet', "(from helper import greet — needs helper.nme next to the program)"),
]


# The stopwatch and every cooldown bind one Python name each
# (`_nme_clock`, `_nme_cool_<name>`); nothing else in a program may use
# those names, which is why they start with an underscore.
SLOW_TEXT = [
    ('문장형', 'Sentence', 'say slowly Hello', '천천히 말해줘 안녕', '[print(_ch, end="", flush=True) or __import__("time").sleep(0.04) for _ch in "Hello"]; print()', '[print(_ch, end="", flush=True) or __import__("time").sleep(0.04) for _ch in "안녕"]; print()'),
    ('문장형', 'Sentence', 'show slowly Hello', '천천히 보여줘 안녕', '[print(_ch, end="", flush=True) or __import__("time").sleep(0.04) for _ch in "Hello"]; print()', '[print(_ch, end="", flush=True) or __import__("time").sleep(0.04) for _ch in "안녕"]; print()'),
    ('문장형', 'Sentence', 'say very slowly Hello', '아주 천천히 말해줘 안녕', '[print(_ch, end="", flush=True) or __import__("time").sleep(0.12) for _ch in "Hello"]; print()', '[print(_ch, end="", flush=True) or __import__("time").sleep(0.12) for _ch in "안녕"]; print()'),
    ('문장형', 'Sentence', 'say slowly every 3 seconds Hello', '3초씩 천천히 말해줘 안녕', '[print(_ch, end="", flush=True) or __import__("time").sleep(3) for _ch in "Hello"]; print()', '[print(_ch, end="", flush=True) or __import__("time").sleep(3) for _ch in "안녕"]; print()'),
]

SCREEN = [
    ('문장형', 'Sentence', 'clear the screen', '화면 지워', 'print("\\033[2J\\033[3J\\033[H", end="")', 'print("\\033[2J\\033[3J\\033[H", end="")'),
    ('문장형', 'Sentence', 'clear screen', '화면 비워줘', 'print("\\033[2J\\033[3J\\033[H", end="")', 'print("\\033[2J\\033[3J\\033[H", end="")'),
    ('문장형', 'Sentence', 'draw a line', '줄 그어', 'print("─" * 40)', 'print("─" * 40)'),
    ('문장형', 'Sentence', 'draw line', '가로줄 그어줘', 'print("─" * 40)', 'print("─" * 40)'),
    ('문장형', 'Sentence', 'say in a box Hello', '상자로 말해줘 안녕', 'print((lambda _t: (lambda _w: "┌" + "─" * (_w + 2) + "┐\\n│ " + _t + " │\\n└" + "─" * (_w + 2) + "┘")(sum(2 if __import__("unicodedata").east_asian_width(_c) in "WF" else 1 for _c in _t)))("Hello"))', 'print((lambda _t: (lambda _w: "┌" + "─" * (_w + 2) + "┐\\n│ " + _t + " │\\n└" + "─" * (_w + 2) + "┘")(sum(2 if __import__("unicodedata").east_asian_width(_c) in "WF" else 1 for _c in _t)))("안녕"))'),
    ('문장형', 'Sentence', 'say in the middle Hello', '가운데 말해줘 안녕', 'print((lambda _t: " " * max(0, (40 - sum(2 if __import__("unicodedata").east_asian_width(_c) in "WF" else 1 for _c in _t)) // 2) + _t)("Hello"))', 'print((lambda _t: " " * max(0, (40 - sum(2 if __import__("unicodedata").east_asian_width(_c) in "WF" else 1 for _c in _t)) // 2) + _t)("안녕"))'),
]

TIMER = [
    ('문장형', 'Sentence', 'start the timer', '시간 재기 시작해', '_nme_clock = __import__("time").time()', '_nme_clock = __import__("time").time()'),
    ('문장형', 'Sentence', 'start timer', '시간재기 시작해', '_nme_clock = __import__("time").time()', '_nme_clock = __import__("time").time()'),
    ('문장형', 'Sentence', 'show elapsed', '잰시간 말해줘', 'print(round(__import__("time").time() - _nme_clock, 2))', 'print(round(__import__("time").time() - _nme_clock, 2))'),
    ('문장형', 'Sentence', 'set spent to elapsed', '걸린시간은 잰시간', 'spent = round(__import__("time").time() - _nme_clock, 2)', '걸린시간 = round(__import__("time").time() - _nme_clock, 2)'),
]

COOLDOWN = [
    ('문장형', 'Sentence', 'put door on cooldown for 3 seconds', '문 쿨타임 3초 걸어', '_nme_cool_door = __import__("time").time() + 3', '_nme_cool_문 = __import__("time").time() + 3'),
    ('문장형', 'Sentence', 'when door is ready', '만약 문 쿨타임이 끝났으면', 'if (__import__("time").time() >= _nme_cool_door):', 'if (__import__("time").time() >= _nme_cool_문):'),
    ('문장형', 'Sentence', 'if door is ready', '문 쿨타임 끝났으면', 'if (__import__("time").time() >= _nme_cool_door):', 'if (__import__("time").time() >= _nme_cool_문):'),
    ('문장형', 'Sentence', 'when door is on cooldown', '만약 문 쿨타임이 남았으면', 'if (__import__("time").time() < _nme_cool_door):', 'if (__import__("time").time() < _nme_cool_문):'),
    ('문장형', 'Sentence', 'wait for door', '문 쿨타임 끝날때까지 기다려', '__import__("time").sleep(max(0, _nme_cool_door - __import__("time").time()))', '__import__("time").sleep(max(0, _nme_cool_문 - __import__("time").time()))'),
    ('문장형', 'Sentence', 'pause for door', '문 쿨타임 끝날 때까지 기다려', '__import__("time").sleep(max(0, _nme_cool_door - __import__("time").time()))', '__import__("time").sleep(max(0, _nme_cool_문 - __import__("time").time()))'),
]

LITERALS = [
    ("`True` · `true`", "`참`", "True"),
    ("`False` · `false`", "`거짓`", "False"),
    ("`None` · `none` · `null`", "`없음`", "None"),
]


def table(header: list[str], rows: list[tuple], columns: list[int]) -> str:
    out = ["| " + " | ".join(header) + " |", "| " + " | ".join("---" for _ in header) + " |"]
    for row in rows:
        cells = []
        for index in columns:
            value = row[index]
            cells.append(value if value.startswith("(") or value == "unchanged" else f"`{value}`")
        out.append("| " + " | ".join(cells) + " |")
    return "\n".join(out)


def level_table(rows: list[tuple], korean: bool) -> str:
    """Rows are (ko level, en level, en line, ko line, en Python[, ko Python]).

    The sixth element is only needed when the two languages produce different
    Python, which happens whenever the example contains Korean words.
    """
    header = ["단계", "NME", "만들어지는 Python"] if korean else ["Level", "NME", "Python produced"]
    lines = ["| " + " | ".join(header) + " |", "| --- | --- | --- |"]
    for row in rows:
        level = row[0] if korean else row[1]
        nme = row[3] if korean else row[2]
        python = (row[5] if korean and len(row) > 5 else row[4])
        python = python if python.startswith("(") or python == "unchanged" else f"`{python}`"
        lines.append(f"| {level} | `{nme}` | {python} |")
    return "\n".join(lines)


def korean_condition_endings() -> str:
    """Every Korean comparison ending the parser matches exactly."""
    body = re.search(r"fn condition_connector_exact\(.*?\n\}\n", PARSER, re.S)
    if body is None:
        raise SystemExit("build-syntax-reference: condition_connector_exact not found")
    endings = sorted(set(re.findall(r'"([가-힣]{1,8})"', body.group(0))))
    return spellings(endings)


def compare_table(korean: bool) -> str:
    header = ["NME", "Python", "뜻"] if korean else ["NME", "Python", "Meaning"]
    lines = ["| " + " | ".join(header) + " |", "| --- | --- | --- |"]
    for english, hangul, python, meaning, korean_python in COMPARE:
        nme = hangul if korean else english
        shown = korean_python if korean else python
        lines.append(f"| `{nme}` | `{shown}` | {meaning} |")
    return "\n".join(lines)


def spelling_table(korean: bool) -> str:
    header = ["하는 일", "영어 표기", "한국어 표기"] if korean else ["Action", "English spellings", "Korean spellings"]
    groups = [
        ("출력 / Output", "SAY_WORDS_EN", "SAY_WORDS_KO"),
        ("입력 / Input", "ASK_WORDS_EN", "ASK_WORDS_KO"),
        ("저장 / Save", "SET_WORDS_EN", "SET_WORDS_KO"),
        ("더하기 / Add", "UPDATE_ADD_WORDS_EN", "UPDATE_ADD_WORDS_KO"),
        ("빼기 / Subtract", "UPDATE_SUBTRACT_WORDS_EN", "UPDATE_SUBTRACT_WORDS_KO"),
        ("곱하기 / Multiply", "UPDATE_MULTIPLY_WORDS_EN", "UPDATE_MULTIPLY_WORDS_KO"),
        ("나누기 / Divide", "UPDATE_DIVIDE_WORDS_EN", "UPDATE_DIVIDE_WORDS_KO"),
        ("기다리기 / Wait", "WAIT_WORDS_EN", "WAIT_WORDS_KO"),
        ("반복 / Repeat", "REPEAT_WORDS_EN", "REPEAT_WORDS_KO"),
        ("조건 / If", "WHEN_WORDS_EN", "WHEN_WORDS_KO"),
        ("조건 반복 / While", "WHILE_WORDS_EN", "WHILE_WORDS_KO"),
        ("다른 갈래 / Else", "ELSE_WORDS_EN", "ELSE_WORDS_KO"),
        ("반복 중단 / Break", "BREAK_WORDS_EN", "BREAK_WORDS_KO"),
        ("건너뛰기 / Skip", "CONTINUE_WORDS_EN", "CONTINUE_WORDS_KO"),
        ("블록 닫기 / End", "END_WORDS_EN", "END_WORDS_KO"),
        ("모듈 쓰기 / Use", "USE_WORDS_EN", "USE_WORDS_KO"),
        ("목록에 넣기 / Append", "APPEND_WORDS_EN", "APPEND_WORDS_KO"),
        ("목록 표시 / List", "LIST_WORDS_EN", "LIST_WORDS_KO"),
        ("숫자로 / As a number", "NUMBER_WORDS", None),
        ("숫자 낱말 / Number words", "NUMBER_WORDS_EN", "NUMBER_WORDS_KO"),
        ("횟수 단위 / Count unit", "TIMES_WORDS_EN", "TIMES_WORDS_KO"),
        ("반복 중단(블록 안) / Break inside a block", "BREAK_ALIAS_WORDS_EN", None),
        ("건너뛰기(블록 안) / Skip inside a block", "CONTINUE_ALIAS_WORDS_EN", None),
        ("무작위 고르기 / Random pick", "RANDOM_CHOICE_WORDS", None),
        ("값 바꾸기 연결어 / Value-change connector", "UPDATE_CONNECTOR_WORDS_EN", None),
        ("목록 연결어 / List connector", "APPEND_CONNECTORS_EN", "APPEND_TARGET_PARTICLES_KO"),
        ("저장 대상 조사 / Saved-name particle", "SET_TARGET_PARTICLES_KO", None),
        ("문장 어미 / Sentence ending", "VALUE_ENDINGS_KO", None),
        ("최신판 / Latest", "LATEST_WORDS", None),
        ("파일 읽기 / File read", "FILE_READ_WORDS_EN", "FILE_READ_WORDS_KO"),
        ("파일 쓰기 / File write", "FILE_WRITE_WORDS_EN", "FILE_WRITE_WORDS_KO"),
        ("천천히 / Slowly", "SLOW_WORDS_EN", "SLOW_WORDS_KO"),
        ("아주 / Very", "VERY_WORDS_EN", "VERY_WORDS_KO"),
        ("글자 간격 / Interval", "SLOW_EVERY_WORDS_EN", "SLOW_EVERY_WORDS_KO"),
        ("화면 / Clear screen", "CLEAR_SCREEN_WORDS_EN", "CLEAR_SCREEN_WORDS_KO"),
        ("화면 지우기 / Clear screen action", "CLEAR_SCREEN_ACTIONS_EN", "CLEAR_SCREEN_ACTIONS_KO"),
        ("줄 / Draw line", "DRAW_LINE_WORDS_EN", "DRAW_LINE_WORDS_KO"),
        ("줄 긋기 / Draw line action", "DRAW_LINE_ACTIONS_EN", "DRAW_LINE_ACTIONS_KO"),
        ("상자 / Box", "BOX_WORDS_EN", "BOX_WORDS_KO"),
        ("가운데 / Middle", "MIDDLE_WORDS_EN", "MIDDLE_WORDS_KO"),
        ("시간 재기 / Start timer", "START_TIMER_WORDS_EN", "START_TIMER_WORDS_KO"),
        ("시계 / Timer", "TIMER_WORDS_EN", None),
        ("잰 시간 / Elapsed", "ELAPSED_WORDS_EN", "ELAPSED_WORDS_KO"),
        ("쿨타임 / Cooldown", "COOLDOWN_WORDS_EN", "COOLDOWN_WORDS_KO"),
        ("쿨타임 걸기 / Put on cooldown", "COOLDOWN_SET_WORDS_EN", "COOLDOWN_SET_WORDS_KO"),
        ("쿨타임 끝남 / Ready", "COOLDOWN_READY_WORDS_EN", "COOLDOWN_READY_WORDS_KO"),
        ("쿨타임 남음 / On cooldown", "COOLDOWN_BUSY_WORDS_KO", None),
        ("쿨타임 끝날 때까지 / Until ready", "COOLDOWN_UNTIL_WORDS_KO", None),
        ("군말 / Filler", "SENTENCE_FILLERS", None),
    ]
    lines = ["| " + " | ".join(header) + " |", "| --- | --- | --- |"]
    for label, english_list, korean_list in groups:
        english_words = words(english_list)
        if korean_list is None:
            latin = [w for w in english_words if w.isascii()]
            hangul = [w for w in english_words if not w.isascii()]
        else:
            latin, hangul = english_words, words(korean_list)
        lines.append(f"| {label} | {spellings(latin) or '—'} | {spellings(hangul) or '—'} |")
    return "\n".join(lines)


def diagnostics_table(korean: bool) -> str:
    header = ["코드", "뜻"] if korean else ["Code", "Meaning"]
    lines = ["| " + " | ".join(header) + " |", "| --- | --- |"]
    for code, english, hangul in code_table():
        lines.append(f"| `{code}` | {hangul if korean else english} |")
    return "\n".join(lines)


NAV_KO = "[README](../README.ko.md) | [설치](install.ko.md) | [5분 시작](getting-started.ko.md) | [학습 과정](tutorial.ko.md) | [문법 안내](language.ko.md) | [가이드](guides/index.ko.md)"
NAV_EN = "[Home](../README.md) | [Install](install.md) | [Getting started](getting-started.md) | [Tutorial](tutorial.md) | [Language reference](language.md) | [Guides](guides/index.md)"


def korean_document() -> str:
    return f"""# NME 문법 목록

[English](syntax.md) | 한국어

{NAV_KO}

NME가 **실제로 받아들이는 표기를 빠짐없이** 모아 둔 목록입니다. 설명이 필요하면
[문법 안내](language.ko.md)를 보세요. 이 문서는 예제 모음이 아니라 표입니다.

이 파일은 컴파일러 소스에서 자동으로 만들어집니다
(`python scripts/build-syntax-reference.py`). 컴파일러가 받아들이는 표기가 여기에
빠져 있으면 검사가 실패하므로, 목록과 구현이 어긋날 수 없습니다.

## 읽는 법

- **단계**는 셋입니다. **문장형**은 따옴표·쉼표·괄호·등호·콜론이 없는 말이고,
  **초급**은 짧고 정확한 표기이며, **고급**은 그냥 Python입니다. 선언 없이 한 파일,
  한 줄 안에서 섞어 쓸 수 있습니다.
- **한 줄은 Python이 먼저입니다.** 어떤 줄이든 올바른 Python이면 NME는 손대지
  않습니다. 그래서 낱말 하나짜리 줄(`skip`, `멈춰`)은 Python 이름으로 남고,
  반복 블록 안에서만 NME 명령이 됩니다.
- **모든 NME 문장은 Python 한 줄이 됩니다.** 줄 수가 변하지 않으므로 오류 메시지의
  줄 번호가 내가 쓴 파일과 정확히 맞습니다.
- 표의 `만들어지는 Python`은 컴파일러가 내놓는 글자 그대로입니다.

## 1. 출력

{level_table(SAY, True)}

**글이 되는가, 코드가 되는가.** 동작 단어 뒤가 올바른 Python 표현식이고 그 안의
이름을 프로그램이 이미 알고 있으면 **코드**로, 그렇지 않으면 **글**로 다룹니다.
`말해`와 `say`만은 반대로, 표현식을 먼저 시도합니다. 앞에서 만든 이름이 글 안에
있으면 그 자리에 값이 들어갑니다(`안녕하세요 이름!` → `"안녕하세요 " + str(이름) + "!"`).

## 2. 입력

{level_table(ASK, True)}

`숫자로`/`ask number`를 붙이면 `int(input(...))`이 됩니다. 나이나 개수를 묻는
질문(`몇 살이에요?`, `How old are you?`)은 붙이지 않아도 숫자로 읽습니다.
질문 뒤에 공백이 없으면 하나 붙여 줍니다.

## 3. 값 저장

{level_table(SET, True)}

한국어는 조사 `은`/`는`만으로 저장이 됩니다. 영어에는 그런 조사가 없으므로
`set … to …`를 씁니다.

## 4. 값 바꾸기

{level_table(UPDATE, True)}

⚠ 여러 낱말짜리 값은 괄호로 묶입니다. `점수에서 1 + 2 빼줘`는
`점수 - (1 + 2)`이지 `점수 - 1 + 2`가 아닙니다.

## 5. 기다리기

{level_table(WAIT, True)}

단위 낱말(`초`, `seconds`)은 생략해도 됩니다. 숫자가 없는 줄
(`잠깐 기다려`)은 그냥 문장으로 출력됩니다.

## 6. 횟수 반복

{level_table(TIMES, True)}

블록은 세 가지로 닫습니다: 들여쓰기, `:` 뒤 한 줄, 또는 `끝`/`end` 한 줄.

## 7. 목록 반복

{level_table(FOR_EACH, True)}

`마다` 앞의 이름이 항목을 하나씩 받는 이름이고, 그 이름은 블록 안에서 바로
쓸 수 있습니다.

## 8. 조건 반복

{level_table(WHILE, True)}

## 9. 조건

{level_table(WHEN, True)}

## 10. 비교 어휘

{compare_table(True)}

한국어 어미는 줄여 써도 됩니다: {korean_condition_endings()}

## 11. 반복 제어

{level_table(LOOP_CONTROL, True)}

`멈춰`와 `건너뛰어`는 그 자체로 올바른 Python 이름이므로, **반복 블록 안에서만**
NME 명령이 됩니다. 블록 밖에서는 Python 그대로 남습니다.

## 12. 목록

{level_table(LISTS, True)}

`목록`/`list of` 표시가 없으면 쉼표가 들어간 줄은 그냥 글입니다.
`친구들에 민수 넣어`(목록에 넣기)와 `점수에 1 더해`(값 바꾸기)는 다른 명령입니다.

## 13. 값과 리터럴

| 영어 표기 | 한국어 표기 | Python |
| --- | --- | --- |
""" + "\n".join(f"| {english} | {hangul} | `{python}` |" for english, hangul, python in LITERALS) + f"""

## 14. 무작위

{level_table(RANDOM, True)}

숫자로 적은 선택지는 숫자로 남습니다(`1 또는 2 중에서 랜덤선택` → `choice((1, 2,))`).

## 15. 파일

{level_table(FILES, True)}

파일 경로는 항상 따옴표로 감쌉니다. 이것만은 문장형에서도 예외입니다.

## 16. 모듈

{level_table(MODULES, True)}

## 17. 천천히 말하기

{level_table(SLOW_TEXT, True)}

글자를 하나씩 내보내고 사이에 잠깐 쉽니다. 쉬는 시간은 기본 0.04초, `아주`를
붙이면 0.12초이며, `3초씩`처럼 직접 정할 수도 있습니다.

## 18. 화면

{level_table(SCREEN, True)}

화면 지우기는 터미널에 보내는 제어 문자입니다. 터미널이 아닌 곳에서는 글자
그대로 보일 수 있습니다. 상자와 가운데 맞춤은 한글을 두 칸으로 세기 때문에
한국어 문장도 반듯하게 나오며, 가로 폭은 40칸입니다.

## 19. 시간 재기

{level_table(TIMER, True)}

`시간 재기 시작해`가 시계를 켜고, `잰시간`/`걸린시간`/`elapsed`는 켠 뒤로 흐른
초를 소수 둘째 자리까지 돌려줍니다. 값이므로 출력·저장·조건 어디에나 쓸 수
있습니다(`만약 잰시간이 3보다 크면`). 켜지 않고 읽으면 컴파일할 때 `E0226`으로
알려 줍니다. 프로그램이 `잰시간`이라는 이름을 직접 만들었으면 그 이름이 이깁니다.

## 20. 쿨타임

{level_table(COOLDOWN, True)}

쿨타임은 이름마다 하나씩 걸립니다. `문 쿨타임 3초 걸어`는 지금부터 3초 뒤를
기억해 두고, `쿨타임이 끝났으면`/`is ready`는 그 시각이 지났는지 봅니다.
조건이므로 `만약`·`동안`·`아니면 만약`과 한 줄 형태에서 모두 쓸 수 있습니다.
`wait for door`는 영어 문장으로도 읽히므로, 그 이름이 이미 다른 값으로 저장돼
있으면 쿨타임으로 읽지 않습니다.

## 21. 동작 단어 전수 목록

같은 뜻으로 받아들이는 표기를 하나도 빠뜨리지 않고 적은 표입니다.

{spelling_table(True)}

## 22. 조사 부록

이름 뒤에 붙어도 이름의 일부로 보지 않는 조사입니다.

{spellings(words("KOREAN_PARTICLES"))}

## 23. 오타 복구

동작 단어와 연결어에 한해, Python이 그 줄을 거부한 다음에만 한 글자 오타를
고쳐서 다시 읽어 봅니다(넣기·빼기·바꾸기·이웃 자리 바꿈 한 번). 고칠 방법이
둘 이상이면 고치지 않고 그 자리를 짚어 알려 줍니다. **문자열과 주석은 절대
건드리지 않습니다.**

## 24. 오류 코드

{diagnostics_table(True)}

`nme ko E0102`처럼 부르면 그 코드의 긴 설명을 한국어로 보여 줍니다
(영어는 `nme en E0102`).
"""


def english_document() -> str:
    return f"""# NME syntax list

English | [한국어](syntax.ko.md)

{NAV_EN}

**Every spelling NME actually accepts**, in one place. For explanations, read the
[language reference](language.md); this file is a table, not a tour.

It is generated from the compiler source
(`python scripts/build-syntax-reference.py`), and a check fails the build if the
compiler accepts a spelling that is missing here — so the list cannot drift away
from the implementation.

## How to read this

- **Three levels.** *Sentence* needs no quotes, commas, parentheses, equals
  signs or colons; *beginner* is short and exact; *advanced* is ordinary Python.
  They mix freely in one file and on one line, with nothing to declare.
- **Python wins.** If a line is valid Python, NME leaves it alone. That is why a
  one-word line (`skip`, `멈춰`) stays a Python name and only becomes an NME
  command inside a loop block.
- **One NME statement is one Python line.** The line count never changes, so a
  traceback points at the line you actually wrote.
- The `Python produced` column is exactly what the compiler emits.

## 1. Output

{level_table(SAY, False)}

**Text or code.** After an action word, a valid Python expression whose names the
program already knows is treated as **code**; anything else is **text**. `say`
and `말해` are the exception: they try the expression first. A name introduced
earlier is substituted into text (`show Hello name!` →
`"Hello " + str(name) + "!"`).

## 2. Input

{level_table(ASK, False)}

`ask number` / `숫자로` produces `int(input(...))`. A question about an age or a
count (`How old are you?`, `몇 살이에요?`) is read as a number without it. A
prompt that does not end in a space gets one.

## 3. Save a value

{level_table(SET, False)}

Korean marks the target with the particle `은`/`는` and needs no action word.
English has no such particle, so it uses `set … to …`.

## 4. Change a value

{level_table(UPDATE, False)}

⚠ A multi-word amount is parenthesised: `subtract 1 + 2 from score` is
`score - (1 + 2)`, not `score - 1 + 2`.

## 5. Wait

{level_table(WAIT, False)}

The unit word (`seconds`, `초`) is optional. A line with no number in it
(`잠깐 기다려`) stays ordinary output.

## 6. Repeat a number of times

{level_table(TIMES, False)}

A block closes three ways: by indentation, by one statement after `:`, or by a
line containing only `end` / `끝`.

## 7. Repeat over a list

{level_table(FOR_EACH, False)}

The name before `in` / `마다` holds each item in turn, and the block body can use
it immediately.

## 8. Repeat while a condition holds

{level_table(WHILE, False)}

## 9. Conditions

{level_table(WHEN, False)}

## 10. Comparison vocabulary

{compare_table(False)}

English accepts synonyms for the comparison words: `greater`, `above`, `great`,
`larger`, `bigger`, `higher` all mean `>`; `less`, `below`, `small`, `smaller`,
`lower` all mean `<`; `equals`, `equal`, `same` mean `==`.

## 11. Loop control

{level_table(LOOP_CONTROL, False)}

`break` and `skip` (`멈춰`, `건너뛰어`) are valid Python names on their own, so
they are read as NME **only inside a loop block**. Outside one they stay Python.

## 12. Lists

{level_table(LISTS, False)}

Without the `list of` / `목록` marker a comma-separated line is ordinary text.
`append Mina to friends` (a list) and `add 1 to score` (a number) are different
commands and keep their own meanings.

## 13. Values and literals

| English | Korean | Python |
| --- | --- | --- |
""" + "\n".join(f"| {english} | {hangul} | `{python}` |" for english, hangul, python in LITERALS) + f"""

## 14. Randomness

{level_table(RANDOM, False)}

A choice written as a number stays a number (`pick from 1 or 2` →
`choice((1, 2,))`).

## 15. Files

{level_table(FILES, False)}

A file path is always quoted. This is the one place the sentence level asks for
a quote character.

## 16. Modules

{level_table(MODULES, False)}

## 17. Slow text

{level_table(SLOW_TEXT, False)}

Each character is printed on its own with a short pause after it. The pause is
0.04 seconds by default, 0.12 with `very`, and whatever you name with
`every 3 seconds` / `3초씩`.

## 18. Screen

{level_table(SCREEN, False)}

Clearing the screen sends a terminal control sequence, so somewhere that is not
a terminal it may show up as text. The box and the centred line count a Korean
character as two columns, so a Korean sentence comes out straight; the width is
40 columns.

## 19. The stopwatch

{level_table(TIMER, False)}

`start the timer` starts the clock and `elapsed` / `잰시간` / `걸린시간` reads
how many seconds have passed, to two decimal places. It is a value, so it works
in output, in a saved name, and in a condition (`if elapsed is greater than 3`).
Reading it without starting the clock is reported at compile time as `E0226`. A
name the program made itself always wins over the word.

## 20. Cooldowns

{level_table(COOLDOWN, False)}

One cooldown belongs to one name. `put door on cooldown for 3 seconds` remembers
the moment three seconds from now, and `is ready` / `쿨타임이 끝났으면` asks
whether that moment has passed. They are conditions, so they work with `when`,
`while`, `else if`, and the one-line form of all three. `wait for door` also
reads as an ordinary English sentence, so a name the program already saved as
something else is not read as a cooldown.

## 21. Every action word

Every spelling accepted for each action, with nothing left out.

{spelling_table(False)}

## 22. Korean particles

These endings are not treated as part of the name they follow.

{spellings(words("KOREAN_PARTICLES"))}

## 23. Typo recovery

For action words and connectors only, and only after Python has rejected the
line, NME retries once with a single edit repaired (one insertion, deletion,
substitution, or adjacent swap). If more than one repair is possible it repairs
nothing and points at the exact span instead. **Strings and comments are never
touched.**

## 24. Error codes

{diagnostics_table(False)}

Run `nme en E0102` for the long explanation of a code (`nme ko E0102` in
Korean).
"""


def main() -> None:
    (ROOT / "docs/syntax.ko.md").write_text(korean_document(), encoding="utf-8")
    (ROOT / "docs/syntax.md").write_text(english_document(), encoding="utf-8")
    print("wrote docs/syntax.md and docs/syntax.ko.md")


if __name__ == "__main__":
    main()
