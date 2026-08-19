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
    ("문장형", "Sentence", "repeat forever", "계속 반복해", "while True:"),
    ("문장형", "Sentence", "repeat forever and show Again", "계속 반복해서 다시 말해줘", 'while True: print("Again")', 'while True: print("다시")'),
    ("초급", "Beginner", '3 times: say "Hi"', '3번: 말해 "안녕"', 'for _ in range(3): print("Hi")', 'for _ in range(3): print("안녕")'),
    ("고급", "Advanced", "for i in range(3):", "for i in range(3):", "unchanged"),
]

FOR_EACH = [
    ("문장형", "Sentence", "for each friend in friends", "친구들의 친구마다 반복해", "for friend in friends:", 'for 친구 in 친구들:'),
    ("문장형", "Sentence", "for each friend in friends and show friend", "친구들의 친구마다 반복해서 친구 말해줘", "for friend in friends: print(friend)", 'for 친구 in 친구들: print(친구)'),
    ("문장형", "Sentence", "repeat for each name in names", "이름들에서 이름마다 반복해", "for name in names:", 'for 이름 in 이름들:'),
    ("문장형", "Sentence", "foreach friend in friends", "친구들의 친구 마다 반복해", "for friend in friends:", 'for 친구 in 친구들:'),
    # `친구들마다` on its own: Korean drops the singular name when the plural
    # already says it, and NME calls each one by the plural without its `들`.
    ("문장형", "Sentence", "for every friend in friends", "친구들마다 반복해", "for friend in friends:", 'for 친구 in 친구들:'),
    ("문장형", "Sentence", "for each friend in friends with place", "친구들의 친구마다 순서와 함께 반복해", "for place, friend in enumerate(friends, 1):", 'for 순서, 친구 in enumerate(친구들, 1):'),
    ("초급", "Beginner", "for each friend in friends:", "친구들의 친구마다:", "for friend in friends:", 'for 친구 in 친구들:'),
    ("초급", "Beginner", "for each friend in friends with place:", "친구들의 친구마다 순서와 함께:", "for place, friend in enumerate(friends, 1):", 'for 순서, 친구 in enumerate(친구들, 1):'),
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
    ("문장형", "Sentence", "set friends to an empty list", "친구들은 빈 목록", "friends = []", '친구들 = []'),
    ("문장형", "Sentence", "remove Mina from friends", "친구들에서 민수 빼", 'friends.remove("Mina")', '친구들.remove("민수")'),
    ("문장형", "Sentence", "show how many friends", "친구들 개수 말해줘", "print(len(friends))", 'print(len(친구들))'),
    ("문장형", "Sentence", "set total to how many friends", "총합은 친구들 개수", "total = len(friends)", '총합 = len(친구들)'),
    # A reading standing inside a sentence rather than being the whole of it.
    ("문장형", "Sentence", "show You have how many friends left", "친구들 개수 명 남았습니다 말해줘",
     'print("You have " + str(len(friends)) + " left")', 'print(str(len(친구들)) + " 명 남았습니다")'),
    ("문장형", "Sentence", "sort friends", "친구들 정렬해", "friends.sort()", '친구들.sort()'),
    ("문장형", "Sentence", "reverse friends", "친구들 거꾸로 해", "friends.reverse()", '친구들.reverse()'),
    ("문장형", "Sentence", "shuffle friends", "친구들 섞어", '__import__("random").shuffle(friends)', '__import__("random").shuffle(친구들)'),
    ("문장형", "Sentence", "show the first of friends", "친구들 첫 번째 말해줘", "print(friends[0])", 'print(친구들[0])'),
    ("문장형", "Sentence", "show the last of friends", "친구들 마지막 말해줘", "print(friends[-1])", 'print(친구들[-1])'),
    ("문장형", "Sentence", "show item 2 of friends", "친구들 2번째 말해줘", "print(friends[1])", 'print(친구들[1])'),
    ("문장형", "Sentence", "show the total of scores", "점수들 합 말해줘", "print(sum(scores))", 'print(sum(점수들))'),
    ("문장형", "Sentence", "show the biggest of scores", "점수들 중 가장 큰 것 말해줘", "print(max(scores))", 'print(max(점수들))'),
    ("문장형", "Sentence", "show the smallest of scores", "점수들 중 가장 작은 것 말해줘", "print(min(scores))", 'print(min(점수들))'),
    ("문장형", "Sentence", "show friends joined by comma", "친구들을 쉼표로 이어 말해줘", 'print(", ".join(map(str, friends)))', 'print(", ".join(map(str, 친구들)))'),
    ("문장형", "Sentence", "show friends joined by space", "친구들을 빈칸으로 이어 말해줘", 'print(" ".join(map(str, friends)))', 'print(" ".join(map(str, 친구들)))'),
    ("문장형", "Sentence", "show friends joined together", "친구들을 붙여 말해줘", 'print("".join(map(str, friends)))', 'print("".join(map(str, 친구들)))'),
    ("문장형", "Sentence", "show friends joined by nothing", "친구들을 그대로 이어 말해줘", 'print("".join(map(str, friends)))', 'print("".join(map(str, 친구들)))'),
    ("문장형", "Sentence", "if friends contains Mina", "만약에 친구들에 민수가 있으면", 'if ("Mina" in friends):', 'if ("민수" in 친구들):'),
    ("문장형", "Sentence", "if friends does not contain Ada", "만약에 친구들에 지안이 없으면", 'if ("Ada" not in friends):', 'if ("지안" not in 친구들):'),
    ("문장형", "Sentence", "if friends is empty", "만약에 친구들이 비었으면", "if (not (friends)):", 'if (not (친구들)):'),
    ("초급", "Beginner", "say len(friends)", "말해 len(친구들)", "print(len(friends))", 'print(len(친구들))'),
    ("고급", "Advanced", 'friends = ["Mina"]', '친구들 = ["민수"]', "unchanged"),
]

RECORDS = [
    ("문장형", "Sentence", "set ages to an empty record", "나이표는 빈 표", "ages = {}", '나이표 = {}'),
    ("문장형", "Sentence", "set ages to record", "나이표는 표", "ages = {}", '나이표 = {}'),
    # Korean says `표` for both `record` and `table`, so pairing the English
    # `table` row with `빈 표` printed the same Korean row twice. `비어있는` is a
    # real second spelling and keeps the row worth reading.
    ("문장형", "Sentence", "set ages to an empty table", "나이표는 비어있는 표", "ages = {}", '나이표 = {}'),
    ("문장형", "Sentence", "put Mina at 90 in ages", "나이표에 민수를 90으로 넣어", 'ages["Mina"] = 90', '나이표["민수"] = 90'),
    ("문장형", "Sentence", "put Mina as 90 in ages", "나이표에 민수를 90으로 두어", 'ages["Mina"] = 90', '나이표["민수"] = 90'),
    ("문장형", "Sentence", "show Mina in ages", "나이표의 민수 말해줘", 'print(ages["Mina"])', 'print(나이표["민수"])'),
    ("문장형", "Sentence", "set best to Mina in ages", "제일은 나이표에서 민수", 'best = ages["Mina"]', '제일 = 나이표["민수"]'),
    ("문장형", "Sentence", "show how many ages", "나이표 개수 말해줘", "print(len(ages))", 'print(len(나이표))'),
    ("문장형", "Sentence", "if ages contains Mina", "만약에 나이표에 민수가 있으면", 'if ("Mina" in ages):', 'if ("민수" in 나이표):'),
    ("문장형", "Sentence", "for each name in ages", "나이표의 이름마다 반복해", "for name in ages:", 'for 이름 in 나이표:'),
    ("문장형", "Sentence", "remove Mina from ages", "나이표에서 민수 빼", 'del ages["Mina"]', 'del 나이표["민수"]'),
    ("초급", "Beginner", "save rates to {}", "저장 나이표2 {}", "rates = {}", '나이표2 = {}'),
    ("초급", "Beginner", 'say ages["Mina"]', '말해 나이표["민수"]', 'print(ages["Mina"])', 'print(나이표["민수"])'),
    ("고급", "Advanced", 'ages["Mina"] = 90', '나이표["민수"] = 90', "unchanged"),
]

JOBS = [
    ("문장형", "Sentence", "to greet:", "인사하기라는 일:", "def greet():", 'def 인사하기():'),
    ("문장형", "Sentence", "to greet:", "계산이라는 작업:", "def greet():", 'def 계산():'),
    ("문장형", "Sentence", "do greet", "인사하기 해줘", "greet()", '인사하기()'),
    ("문장형", "Sentence", "run greet", "인사하기 해", "greet()", '인사하기()'),
    ("문장형", "Sentence", "do greet", "인사하기 실행해", "greet()", '인사하기()'),
    ("문장형", "Sentence", "to hail someone:", "이름에게 부르기라는 일:", "def hail(someone):", 'def 부르기(이름):'),
    ("문장형", "Sentence", "do hail with Mina", "민수에게 부르기 해줘", 'hail("Mina")', '부르기("민수")'),
    ("문장형", "Sentence", "run hail with Mina", "민수한테 부르기 실행해", 'hail("Mina")', '부르기("민수")'),
    ("고급", "Advanced", "def greet():", "def 인사하기():", "unchanged"),
]

NUMBERS = [
    ("문장형", "Sentence", "show the remainder of score divided by 4", "점수를 4로 나눈 나머지 말해줘", "print(score % 4)", 'print(점수 % 4)'),
    ("문장형", "Sentence", "set left to the remainder of score divided by 4", "남은것은 점수를 4로 나눈 나머지", "left = score % 4", '남은것 = 점수 % 4'),
    ("문장형", "Sentence", "if the remainder of score divided by 4 equals 0", "만약에 점수를 4로 나눈 나머지가 0과 같으면", "if (score % 4 == 0):", 'if (점수 % 4 == 0):'),
    ("초급", "Beginner", "say score % 4", "말해 점수 % 4", "print(score % 4)", 'print(점수 % 4)'),
    ("고급", "Advanced", "left = score % 4", "남은것 = 점수 % 4", "unchanged"),
]

TEXT = [
    ("문장형", "Sentence", "show the length of name", "이름 길이 말해줘", "print(len(name))", 'print(len(이름))'),
    ("문장형", "Sentence", "show name in capitals", "이름 대문자로 말해줘", "print(str(name).upper())", 'print(str(이름).upper())'),
    ("문장형", "Sentence", "show name in small letters", "이름 소문자로 말해줘", "print(str(name).lower())", 'print(str(이름).lower())'),
    ("문장형", "Sentence", "set names to memo split by line", "이름들은 memo를 줄마다 나눈 것", "names = str(memo).splitlines()", '이름들 = str(memo).splitlines()'),
    ("문장형", "Sentence", "set fields to memo split by comma", "칸들은 memo를 쉼표로 나눈 것", 'fields = str(memo).split(",")', '칸들 = str(memo).split(",")'),
    ("문장형", "Sentence", "set words to memo split by space", "말들은 memo를 빈칸으로 나눈 것", 'words = str(memo).split(" ")', '말들 = str(memo).split(" ")'),
    ("문장형", "Sentence", "set bar to name repeated 5 times", "막대는 이름을 5개 붙인 것", "bar = str(name) * 5", '막대 = str(이름) * 5'),
    ("문장형", "Sentence", "show name repeated 3 times", "이름을 3번 붙인 것 말해줘", "print(str(name) * 3)", 'print(str(이름) * 3)'),
    ("초급", "Beginner", "say len(name)", "말해 len(이름)", "print(len(name))", 'print(len(이름))'),
    ("고급", "Advanced", "name.upper()", "이름.upper()", "unchanged"),
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
    ("초급", "Beginner", "use list", "목록 사용", "(binds count, sort, reverse, remove, first, last, sum, largest, smallest and their Korean twins)"),
    ("초급", "Beginner", "use text", "글자 사용", "(binds upper, lower, trim, split, join, replace, starts_with, length and their Korean twins)"),
    ("초급", "Beginner", "use math", "수학 사용", "(binds math, root, round_to, pi, power, absolute, floor, ceil and their Korean twins)"),
    ("초급", "Beginner", "use date", "날짜 사용", "(binds today, now, year, month, day_of_month, weekday, days_after and their Korean twins)"),
    ("초급", "Beginner", "use random latest", "랜덤 사용 최신", "(the newest bundled adapter)"),
    ("초급", "Beginner", 'use random version "0.0.1"', '랜덤 사용 버전 "0.0.1"', "(that exact adapter)"),
    ("문장형", "Sentence", 'use greet from "helper.nme"', '"helper.nme"에서 greet 가져와', "(from helper import greet — needs helper.nme next to the program)"),
    ("초급", "Beginner", 'from "helper.nme" import greet', '"helper.nme"에서 greet 가져오기', "(from helper import greet — needs helper.nme next to the program)"),
    ("고급", "Advanced", 'from "helper.nme" import greet', 'from "helper.nme" import greet', "(from helper import greet — needs helper.nme next to the program)"),
]


# Every name the four newest bundled adapters bind, written once and emitted
# into both reference files. These are Python names rather than NME statements,
# so they are a plain table rather than a level table; `check-tier-parity.py`
# is what compiles each of them at all three levels in both languages.
MODULE_HELPERS = [
    ("use list", "목록 사용", [
        ("count(values)", "개수(값들)", "`len(values)`"),
        ("sort(values)", "정렬(값들)", "`sorted(values)` — a new list; the original is left alone"),
        ("reverse(values)", "뒤집기(값들)", "`list(reversed(values))`"),
        ("remove(values, x)", "빼기(값들, x)", "every item that is not `x`, as a new list"),
        ("first(values)", "첫번째(값들)", "`values[0]`"),
        ("last(values)", "마지막(값들)", "`values[-1]`"),
        ("sum(values)", "합계(값들)", "`sum(values)`"),
        ("largest(values)", "최대(값들)", "`max(values)`"),
        ("smallest(values)", "최소(값들)", "`min(values)`"),
        ("list_version", "목록버전", '`"0.0.1"`'),
    ]),
    ("use text", "글자 사용", [
        ("upper(text)", "대문자(글)", "`str(text).upper()`"),
        ("lower(text)", "소문자(글)", "`str(text).lower()`"),
        ("trim(text)", "공백없애기(글)", "`str(text).strip()`"),
        ("split(text, sep)", "나누기(글, 구분자)", "`str(text).split(sep)`"),
        ("join(sep, values)", "합치기(구분자, 값들)", "`str(sep).join(map(str, values))`"),
        ("replace(text, a, b)", "바꾸기(글, a, b)", "`str(text).replace(a, b)`"),
        ("starts_with(text, a)", "로시작(글, a)", "`str(text).startswith(a)`"),
        ("length(text)", "길이(글)", "`len(text)`"),
        ("text_version", "글자버전", '`"0.0.1"`'),
    ]),
    ("use math", "수학 사용", [
        ("root(x)", "제곱근(x)", "`math.sqrt(x)`"),
        ("round_to(x, places)", "반올림(x, 자리)", "`round(x, places)` — `places` may be left out"),
        ("pi", "원주율", "`math.pi`"),
        ("power(x, y)", "거듭제곱(x, y)", "`pow(x, y)` — whole numbers stay whole"),
        ("absolute(x)", "절댓값(x)", "`abs(x)`"),
        ("floor(x)", "내림(x)", "`math.floor(x)`"),
        ("ceil(x)", "올림(x)", "`math.ceil(x)`"),
        ("math_version", "수학버전", '`"0.0.1"`'),
    ]),
    ("use date", "날짜 사용", [
        ("today()", "오늘()", '`date.today().isoformat()` — text such as `"2026-08-19"`'),
        ("now()", "지금()", '`datetime.now().strftime("%H:%M")` — the clock a browser gives is UTC'),
        ("year()", "올해()", "`date.today().year`"),
        ("month()", "이번달()", "`date.today().month`"),
        ("day_of_month()", "오늘일자()", "`date.today().day`"),
        ("weekday()", "요일()", '`strftime("%A")`, an English name such as `Wednesday`; the Korean name answers `수요일`'),
        ("days_after(n)", "며칠뒤(n)", "`(date.today() + timedelta(days=n)).isoformat()`; a negative `n` reads as days before"),
        ("date_version", "날짜버전", '`"0.0.1"`'),
    ]),
]


def module_helper_tables(korean_first: bool) -> str:
    """The helper tables for `use list`, `use text`, `use math` and `use date`."""
    blocks = []
    for english_line, korean_line, rows in MODULE_HELPERS:
        lead = f"`{korean_line}`" if korean_first else f"`{english_line}`"
        other = f"`{english_line}`" if korean_first else f"`{korean_line}`"
        header = ("| 한국어 표기 | 영어 표기 | Python |"
                  if korean_first else
                  "| English | Korean | Python |")
        body = []
        for english, hangul, python in rows:
            left, right = (hangul, english) if korean_first else (english, hangul)
            body.append(f"| `{left}` | `{right}` | {python} |")
        blocks.append(
            f"{lead} ({other}):\n\n{header}\n| --- | --- | --- |\n"
            + "\n".join(body)
        )
    return "\n\n".join(blocks)


# The stopwatch and every cooldown bind one Python name each
# (`_nme_clock`, `_nme_cool_<name>`); nothing else in a program may use
# those names, which is why they start with an underscore.
SLOW_TEXT = [
    ('문장형', 'Sentence', 'say slowly Hello', '천천히 말해줘 안녕', '[print(_ch, end="", flush=True) or __import__("time").sleep(0.04) for _ch in "Hello"]; print()', '[print(_ch, end="", flush=True) or __import__("time").sleep(0.04) for _ch in "안녕"]; print()'),
    ('문장형', 'Sentence', 'show slowly Hello', '천천히 보여줘 안녕', '[print(_ch, end="", flush=True) or __import__("time").sleep(0.04) for _ch in "Hello"]; print()', '[print(_ch, end="", flush=True) or __import__("time").sleep(0.04) for _ch in "안녕"]; print()'),
    ('문장형', 'Sentence', 'say very slowly Hello', '아주 천천히 말해줘 안녕', '[print(_ch, end="", flush=True) or __import__("time").sleep(0.12) for _ch in "Hello"]; print()', '[print(_ch, end="", flush=True) or __import__("time").sleep(0.12) for _ch in "안녕"]; print()'),
    ('문장형', 'Sentence', 'say slowly every 3 seconds Hello', '3초씩 천천히 말해줘 안녕', '[print(_ch, end="", flush=True) or __import__("time").sleep(3) for _ch in "Hello"]; print()', '[print(_ch, end="", flush=True) or __import__("time").sleep(3) for _ch in "안녕"]; print()'),
]

CHANCE = [
    ("문장형", "Sentence", "30% chance show You win", "30% 확률로 말해줘 당첨", 'if __import__("random").randrange(1000) < 300: print("You win")', 'if __import__("random").randrange(1000) < 300: print("당첨")'),
    ("문장형", "Sentence", "30.5% chance show You win", "30.5% 확률로 말해줘 당첨", 'if __import__("random").randrange(1000) < 305: print("You win")', 'if __import__("random").randrange(1000) < 305: print("당첨")'),
    ("문장형", "Sentence", "with a 30% chance show You win", "30%의 확률로 말해줘 당첨", 'if __import__("random").randrange(1000) < 300: print("You win")', 'if __import__("random").randrange(1000) < 300: print("당첨")'),
    ("문장형", "Sentence", "30 percent chance show You win", "30퍼센트 확률로 말해줘 당첨", 'if __import__("random").randrange(1000) < 300: print("You win")', 'if __import__("random").randrange(1000) < 300: print("당첨")'),
    ("문장형", "Sentence", "30% of the time show You win", "확률 30%로 말해줘 당첨", 'if __import__("random").randrange(1000) < 300: print("You win")', 'if __import__("random").randrange(1000) < 300: print("당첨")'),
    ("문장형", "Sentence", "30% chance", "30% 확률로", 'if __import__("random").randrange(1000) < 300:'),
    ("문장형", "Sentence", "luck is a 30% chance", "운은 30% 확률", 'luck = __import__("random").randrange(1000) < 300', '운 = __import__("random").randrange(1000) < 300'),
]

# Every line inside a story block is text. The block header is the only NME
# form with a colon, because a bare `story:` is a syntax error in Python and
# can therefore be claimed without disturbing the Python-wins rule.
STORY = [
    ("문장형", "Sentence", "story:", "이야기:", "if True:"),
    ("문장형", "Sentence", "slow story:", "천천히 이야기:", "if True:"),
    ("문장형", "Sentence", "very slow story:", "아주 천천히 이야기:", "if True:"),
    ("문장형", "Sentence", "slow story every 3 seconds:", "3초씩 천천히 이야기:", "if True:"),
    ("문장형", "Sentence", "The door opened.", "문이 열렸습니다.", 'print("The door opened.")', 'print("문이 열렸습니다.")'),
    ("문장형", "Sentence", "wait 3 seconds", "3초 기다려", '(inside a story: print("wait 3 seconds"))', '(이야기 안에서는 print("3초 기다려"))'),
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
        ("말끝 출력 / Output written last", "SAY_TRAILING_WORDS_KO", None),
        ("목적어가 없을 때만 말끝 출력 / Output written last, with no object",
         "SAY_TRAILING_OBJECT_FREE_WORDS_KO", None),
        ("한 낱말만 보여 주기 / Show the one word after it", "SAY_ONE_WORD_WORDS_EN", None),
        ("한 글자 출력 / Short output word", "SAY_SHORT_WORDS_KO", None),
        ("화면에 / On the screen", "SCREEN_TAIL_WORDS_EN", "SCREEN_TAIL_WORDS_KO"),
        ("화면 동사 / Screen verb", "SCREEN_VERB_WORDS_EN", "SCREEN_VERB_WORDS_KO"),
        ("물음표가 있을 때만 묻기 / Ask, only with a question mark",
         "ASK_QUESTION_WORDS_EN", "ASK_QUESTION_WORDS_KO"),
        ("짧은 물어보기 / Short asking word", "ASK_SHORT_WORDS_KO", None),
        ("~로 해 / Save with the everyday verb", "SET_MAKE_WORDS_EN", "SET_MAKE_WORDS_KO"),
        ("~로 / ~라고 표시 / What the name becomes", "SET_MAKE_ENDINGS_KO", None),
        ("횟수가 있을 때만 반복 / Repeat, only with a count",
         "REPEAT_COUNT_WORDS_EN", "REPEAT_COUNT_WORDS_KO"),
        ("건너뛰기 / Skip", "CONTINUE_WORDS_EN", "CONTINUE_WORDS_KO"),
        ("블록 닫기 / End", "END_WORDS_EN", "END_WORDS_KO"),
        ("모듈 쓰기 / Use", "USE_WORDS_EN", "USE_WORDS_KO"),
        ("다른 파일에서 / Import from a file", "NME_IMPORT_WORDS_EN", "NME_IMPORT_WORDS_KO"),
        ("목록에 넣기 / Append", "APPEND_WORDS_EN", "APPEND_WORDS_KO"),
        ("목록 표시 / List", "LIST_WORDS_EN", "LIST_WORDS_KO"),
        ("빈 목록 / Empty list", "EMPTY_WORDS_EN", "EMPTY_WORDS_KO"),
        ("개수 / How many", "COUNT_WORDS_EN", "COUNT_WORDS_KO"),
        ("개수 앞말 / Reading lead", "READING_LEAD_WORDS_EN", None),
        ("길이 / Length", "LENGTH_WORDS_EN", "LENGTH_WORDS_KO"),
        ("합 / Total", "TOTAL_WORDS_EN", "TOTAL_WORDS_KO"),
        ("최댓값 / Biggest", "LARGEST_WORDS_EN", "LARGEST_WORDS_KO"),
        ("최솟값 / Smallest", "SMALLEST_WORDS_EN", "SMALLEST_WORDS_KO"),
        ("최댓값 앞말 / Extreme scope", "EXTREME_SCOPE_WORDS_KO", None),
        ("가장 / Most", "EXTREME_MOST_WORDS_KO", None),
        ("것 / Thing", "EXTREME_THING_WORDS_KO", None),
        ("첫 번째 / First", "FIRST_WORDS_EN", "FIRST_WORDS_KO"),
        ("마지막 / Last", "LAST_WORDS_EN", "LAST_WORDS_KO"),
        ("몇 번째 / Item", "ITEM_WORDS_EN", "ITEM_WORDS_KO"),
        ("대문자 / Capitals", "CAPITALS_WORDS_EN", "CAPITALS_WORDS_KO"),
        ("소문자 / Small letters", "SMALL_LETTERS_WORDS_EN", "SMALL_LETTERS_WORDS_KO"),
        ("이어 붙이기 / Join", "JOIN_WORDS_EN", "JOIN_WORDS_KO"),
        ("사이 없이 이어 붙이기 / Join together", "JOIN_TOGETHER_WORDS_EN", "JOIN_TOGETHER_WORDS_KO"),
        ("빈 이음말 / Empty separator", "EMPTY_SEPARATOR_WORDS_EN", "EMPTY_SEPARATOR_WORDS_KO"),
        ("나누기(글) / Split", "SPLIT_WORDS_EN", "SPLIT_WORDS_KO"),
        ("줄마다 / By line", "SPLIT_LINE_WORDS_EN", "SPLIT_LINE_WORDS_KO"),
        ("나눈 것 / Split thing", "SPLIT_THING_WORDS_KO", None),
        ("붙인 것 / Repeated text", "REPEAT_TEXT_WORDS_EN", "REPEAT_TEXT_WORDS_KO"),
        ("몇 개 / Copies", "COPIES_WORDS_EN", "COPIES_WORDS_KO"),
        ("몇 번째와 함께 / With its position", "POSITION_WORDS_EN", "POSITION_WORDS_KO"),
        ("나머지 / Remainder", "REMAINDER_WORDS_EN", "REMAINDER_WORDS_KO"),
        ("나누기 말 / Divided", "DIVIDED_WORDS_EN", "DIVIDED_WORDS_KO"),
        ("이음말 / Separator", "SEPARATOR_WORDS_EN", "SEPARATOR_WORDS_KO"),
        ("정렬 / Sort", "SORT_WORDS_EN", "SORT_WORDS_KO"),
        ("거꾸로 / Reverse", "REVERSE_WORDS_EN", "REVERSE_WORDS_KO"),
        ("섞기 / Shuffle", "SHUFFLE_WORDS_EN", "SHUFFLE_WORDS_KO"),
        ("들어있는지 / Contains", "CONTAINS_WORDS_EN", "CONTAINS_WORDS_KO"),
        ("표 표시 / Record", "RECORD_WORDS_EN", "RECORD_WORDS_KO"),
        ("표에 넣기 / Put in a record", "RECORD_PUT_WORDS_EN", "RECORD_PUT_WORDS_KO"),
        ("표 값 앞말 / Record value connector", "RECORD_AT_WORDS_EN", "RECORD_VALUE_PARTICLES_KO"),
        ("표 이름 앞말 / Record container connector", "RECORD_IN_WORDS_EN", "RECORD_KEY_PARTICLES_KO"),
        ("표에서 읽기 조사 / Record reading particle", "RECORD_OF_PARTICLES_KO", None),
        ("일 표시 / Job", "JOB_LEAD_WORDS_EN", "JOB_WORDS_KO"),
        ("일 이름 어미 / Job name ending", "JOB_NAME_SUFFIXES_KO", None),
        ("일 실행 / Run a job", "RUN_JOB_WORDS_EN", "RUN_JOB_WORDS_KO"),
        ("일이 받는 것 / What a job is given", "JOB_WITH_WORDS_EN", "JOB_PARAMETER_PARTICLES_KO"),
        ("무한 반복 / Forever", "FOREVER_WORDS_EN", "FOREVER_WORDS_KO"),
        ("읽기 조사 / Reading particle", "READING_PARTICLES_KO", None),
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
        ("이야기 / Story", "STORY_WORDS_EN", "STORY_WORDS_KO"),
        ("이야기 천천히 / Story, slowly", "STORY_SLOW_WORDS_EN", "STORY_SLOW_WORDS_KO"),
        ("확률 / Chance", "CHANCE_WORDS_EN", "CHANCE_WORDS_KO"),
        ("퍼센트 / Percent", "CHANCE_PERCENT_WORDS_EN", "CHANCE_PERCENT_WORDS_KO"),
        ("확률 앞뒤 말 / Chance connector", "CHANCE_LEAD_WORDS_EN", "CHANCE_PARTICLES_KO"),
        ("확률의 다른 말 / Chance, other wording", "CHANCE_TIME_WORDS_EN", None),
        ("확률 저장 / Chance saved in a name", "CHANCE_IS_WORDS_EN", None),
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

**몇 번째인지는 1부터 셉니다.** `친구들 첫 번째`가 곧 `친구들 1번째`이고,
Python으로는 `친구들[0]`이 됩니다. 0번째는 없으며 적으면 `E0229`로 알려 줍니다.

목록을 읽고 바꾸는 문장(`개수`·`정렬해`·`섞어`·`첫 번째`·`합`·`빼`…)은
**프로그램이 이미 목록으로 만든 이름에만** 씁니다. 그래야 `친구들 이야기를
들었습니다`처럼 그 낱말이 들어간 평범한 문장이 명령으로 바뀌지 않습니다.
목록이 아닌 이름에 쓰면 `E0231`로 알려 줍니다.

`쉼표`/`comma`는 쉼표와 빈칸(`", "`), `빈칸`·`공백`/`space`는 빈칸 하나,
`줄바꿈`/`newline`은 줄바꿈입니다. 항목이 숫자여도 되도록 `map(str, …)`을
거쳐서 잇습니다.

## 13. 표

{level_table(RECORDS, True)}

표는 이름이 붙은 값을 한 번에 여럿 담습니다. Python으로는 사전(dictionary)이고,
처음 배우는 사람에게는 「이름과 그에 딸린 값을 적어 둔 표」입니다.

`빈 표`/`an empty record`는 **값을 저장하는 자리에서만** 표가 됩니다.
`표를 보여 주세요`처럼 그 낱말이 들어간 평범한 문장은 그대로 글로 남습니다.

목록과 겹치는 표기 — `개수`·`빼`·`넣어`·`…에 …가 있으면`·`…마다 반복해` — 는
**모두 같은 낱말**이고, 그 이름이 목록인지 표인지에 따라 하는 일이 갈립니다.
읽는 사람이 어느 쪽인지 외울 필요가 없도록 일부러 그렇게 했습니다. 목록인
이름에 표 문장을 쓰면 `E0234`로 알려 줍니다.

표에는 순서가 없어서 `합`·`가장 큰 것`·`첫 번째`·`정렬해`는 쓰지 않습니다.
`나이표의 이름마다 반복해`는 값이 아니라 **이름**을 하나씩 돌려주므로, 값은
`나이표의 이름`처럼 다시 꺼내 읽습니다.

Python으로 `나이표 = {{}}`라고 쓴 이름도 표로 봅니다.

## 14. 글자 다루기

{level_table(TEXT, True)}

`길이`/`the length of`는 글자 수를, `대문자로`/`in capitals`와
`소문자로`/`in small letters`는 같은 글을 대·소문자로 바꾼 것을 돌려줍니다.
셋 다 값이므로 출력·저장·조건 어디에나 쓸 수 있습니다. 앞에서 만든 이름에만
쓸 수 있고, 그래서 그 낱말이 들어간 평범한 문장은 그대로 글로 남습니다.

## 15. 숫자 나머지

{level_table(NUMBERS, True)}

`나머지`/`the remainder of`는 나눗셈에서 남는 수입니다. 값이라서 출력·저장·조건
어디에나 쓸 수 있고, 나누는 수는 숫자이거나 앞에서 만든 이름이어야 합니다.

## 16. 값과 리터럴

| 영어 표기 | 한국어 표기 | Python |
| --- | --- | --- |
""" + "\n".join(f"| {english} | {hangul} | `{python}` |" for english, hangul, python in LITERALS) + f"""

## 17. 무작위

{level_table(RANDOM, True)}

숫자로 적은 선택지는 숫자로 남습니다(`1 또는 2 중에서 랜덤선택` → `choice((1, 2,))`).

## 18. 확률

{level_table(CHANCE, True)}

`30%`는 천 번 중 300번입니다. 소수점은 첫째 자리까지만 쓸 수 있고(`30.5%`),
그보다 잘게 적으면 반올림하지 않고 `E0227`로 알려 줍니다. 쓴 사람이 적지 않은
뜻으로 프로그램이 돌아가는 일은 없어야 하기 때문입니다. 범위는 `0%`부터
`100%`까지이며 벗어나면 `E0228`입니다. `100%`는 항상 일어나고 `0%`는 절대
일어나지 않습니다. 비교는 천분율 정수로만 하므로 소수 비교에서 생기는 오차가
없습니다.

이름에 저장한 확률은 참·거짓 값이라서 `만약에 운이 있으면`처럼 그대로 물어볼 수
있습니다.

## 19. 파일

{level_table(FILES, True)}

파일 경로는 항상 따옴표로 감쌉니다. 이것만은 문장형에서도 예외입니다.

## 20. 모듈

{level_table(MODULES, True)}

`랜덤 사용`·`파일 사용`·`영지식 사용` 다음으로 자주 쓰는 네 가지가 `목록 사용`,
`글자 사용`, `수학 사용`, `날짜 사용`입니다. 한 줄이면 아래 이름들이 한국어와 영어
두 가지로 모두 준비되고, 안에 들어 있는 것은 전부 평범한 Python 기본 기능이라
브라우저에서도 그대로 돕니다. `목록`·`글자`·`수학`·`날짜`는 평범한 낱말이라서,
`사용`/`use` 바로 옆에 있고 그 줄에 다른 낱말이 없을 때만 모듈로 읽습니다.
`장 볼 목록을 사용해 보세요`와 `이 날짜 사용법을 알려 주세요`는 그대로 출력되는
문장입니다.

수요일이면 `요일()`은 `수요일`을, 영어 이름 `weekday()`는 `Wednesday`를
돌려줍니다. 두 이름이 서로 다른 값을 갖는 곳은 내장 모듈을 통틀어 여기 하나이며,
요일 이름은 숫자가 아니라 낱말이라서 어느 한 언어에 속할 수밖에 없기 때문입니다.
적은 이름이 답의 언어를 정합니다. 브라우저가 Python에 주는 시계는 UTC이므로
브라우저에서 `오늘()`은 UTC 기준의 오늘입니다.

{module_helper_tables(True)}

`정렬`·`뒤집기`·`빼기`는 새 목록을 돌려주고 원래 목록은 그대로 둡니다. 원래 목록을
직접 바꾸려면 문장형 `친구들 정렬해`·`친구들 거꾸로 해`·`친구들에서 민수 빼`를
쓰세요.

## 21. 천천히 말하기

{level_table(SLOW_TEXT, True)}

글자를 하나씩 내보내고 사이에 잠깐 쉽니다. 쉬는 시간은 기본 0.04초, `아주`를
붙이면 0.12초이며, `3초씩`처럼 직접 정할 수도 있습니다.

## 22. 이야기

{level_table(STORY, True)}

`이야기:` 안에서는 **모든 줄이 글**입니다. `3초 기다려`도 `만약에`도 명령이
아니라 그대로 출력됩니다. 소설처럼 쓰는 것이 목적이라서, 글 한 줄이 조용히
명령으로 바뀌는 일을 아예 막아 두었습니다. 블록은 `끝`/`end`으로 닫고, 들여쓰기로
열었으면 들여쓰기가 끝나는 곳에서도 닫힙니다. 빈 줄은 `print()`가 되어 한 줄을
띄우고, 앞에서 만든 이름은 글 안에서 값으로 바뀝니다. 콜론은 반각 `:`과 전각
`：` 둘 다 됩니다.

## 23. 이름 붙인 일

{level_table(JOBS, True)}

이름 붙인 일은 프로그램의 한 조각에 이름을 붙여 두었다가 나중에 그 이름으로
실행하는 것입니다. Python으로는 함수(`def`)입니다.

`일`·`하기`·`해줘`는 평범한 낱말이라서, 이름 붙인 일은 **낱말이 아니라 모양**으로
알아봅니다. 이름 뒤의 `라는`(또는 `이라는`), 그 뒤의 `일`(또는 `작업`), 줄 끝의
`:`, 그리고 **그 아래에 오는 묶음**까지 넷이 모두 있어야 합니다.
`할 일이 많습니다`에는 그중 하나도 없습니다.

**묶음이 없으면 일이 아닙니다.** `오늘의 할 일:`처럼 제목만 있는 줄은 그대로 글로
나옵니다. 한 줄로 붙여 쓰는 형태도 없습니다.

일을 실행하는 줄은 **그 프로그램이 이미 만든 일 이름**일 때만 실행이 됩니다.
`해`·`해줘`는 그 자체로는 아무것도 정하지 못합니다. Python으로 쓴 `def`도 받는
값이 없으면 같은 방법으로 실행할 수 있습니다.

일 안에서 저장한 이름은 일 안에만 있습니다. Python 함수와 같습니다.

일은 **하나를 받을 수 있습니다.** 여는 줄에서 이름 앞에 받을 것을 `에게`·`한테`·
`을`·`를`로 표시하고(`이름에게 인사하기라는 일:`), 실행할 때 같은 방법으로
건넵니다(`민수에게 인사하기 해줘`). 영어는 `to greet someone:`과
`do greet with Mina`입니다. 받는 개수가 맞지 않으면 `E0235`로 알려 줍니다.
실행할 때 나는 Python `TypeError`는 줄이 멀쩡해 보여서 찾기 어렵기 때문입니다.

아직 **둘 이상을 받는 일**과 **값을 돌려주는 일**은 문장 문법에 없습니다. 그 둘이
필요하면 Python `def`를 그대로 씁니다.

## 24. 화면

{level_table(SCREEN, True)}

화면 지우기는 터미널에 보내는 제어 문자입니다. 터미널이 아닌 곳에서는 글자
그대로 보일 수 있습니다. 상자와 가운데 맞춤은 한글을 두 칸으로 세기 때문에
한국어 문장도 반듯하게 나오며, 가로 폭은 40칸입니다.

## 25. 시간 재기

{level_table(TIMER, True)}

`시간 재기 시작해`가 시계를 켜고, `잰시간`/`걸린시간`/`elapsed`는 켠 뒤로 흐른
초를 소수 둘째 자리까지 돌려줍니다. 값이므로 출력·저장·조건 어디에나 쓸 수
있습니다(`만약 잰시간이 3보다 크면`). 켜지 않고 읽으면 컴파일할 때 `E0226`으로
알려 줍니다. 프로그램이 `잰시간`이라는 이름을 직접 만들었으면 그 이름이 이깁니다.

## 26. 쿨타임

{level_table(COOLDOWN, True)}

쿨타임은 이름마다 하나씩 걸립니다. `문 쿨타임 3초 걸어`는 지금부터 3초 뒤를
기억해 두고, `쿨타임이 끝났으면`/`is ready`는 그 시각이 지났는지 봅니다.
조건이므로 `만약`·`동안`·`아니면 만약`과 한 줄 형태에서 모두 쓸 수 있습니다.
`wait for door`는 영어 문장으로도 읽히므로, 그 이름이 이미 다른 값으로 저장돼
있으면 쿨타임으로 읽지 않습니다.

## 27. 동작 단어 전수 목록

같은 뜻으로 받아들이는 표기를 하나도 빠뜨리지 않고 적은 표입니다.

{spelling_table(True)}

## 28. 조사 부록

이름 뒤에 붙어도 이름의 일부로 보지 않는 조사입니다.

{spellings(words("KOREAN_PARTICLES"))}

## 29. 오타 복구

동작 단어와 연결어에 한해, Python이 그 줄을 거부한 다음에만 한 글자 오타를
고쳐서 다시 읽어 봅니다(넣기·빼기·바꾸기·이웃 자리 바꿈 한 번). 고칠 방법이
둘 이상이면 고치지 않고 그 자리를 짚어 알려 줍니다. **문자열과 주석은 절대
건드리지 않습니다.**

## 30. 오류 코드

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

**Items are counted from one.** `the first of friends` is `item 1 of friends`
and becomes `friends[0]`. There is no item 0; writing one is `E0229`.

The statements that read or rearrange a list (`how many`, `sort`, `shuffle`,
`the first of`, `the total of`, `remove`, …) only work on a name the program
already made a list. That is what keeps `sort out your things` and `the first
of many` ordinary sentences. Using one on a name that is not a list is
`E0231`.

`comma` joins with a comma and a space (`", "`), `space` with one space, and
`newline` with a line break. Items go through `map(str, …)`, so a list of
numbers joins as readily as a list of words.

## 13. Records

{level_table(RECORDS, False)}

A record holds many named values at once. Python calls it a dictionary; a
beginner calls it a list of names and what goes with each.

`an empty record` / `빈 표` is only a record **where a value is being saved**.
An ordinary sentence holding the same word — `I keep a record of everything I
read` — stays a sentence.

The spellings a record shares with a list — `how many`, `remove`, `contains`,
`for each`, `개수`, `빼`, `넣어` — are **the same words**, and what they do
depends on which kind the name holds. That is deliberate: a reader should not
have to remember which is which. Using a record line on a list is `E0234`.

A record has no order, so `the total of`, `the biggest of`, `the first of` and
`sort` are not written for one. `for each name in ages` hands back the
**names**, so the value is read back with `name in ages`.

A name Python wrote as `ages = {{}}` is a record too.

## 14. Working with text

{level_table(TEXT, False)}

`the length of` gives how many characters a piece of text has, and
`in capitals` / `in small letters` give the same text with its letters
changed. All three are values, so they work in output, in a saved name, and in
a condition. They only read a name the program already made, which is what
keeps an ordinary sentence containing one of those words a sentence.

## 15. Number remainders

{level_table(NUMBERS, False)}

`the remainder of` is what is left over after a division. It is a value, so it
works in output, in a saved name, and in a condition; the number being divided
by must be a number or a name the program already made.

## 16. Values and literals

| English | Korean | Python |
| --- | --- | --- |
""" + "\n".join(f"| {english} | {hangul} | `{python}` |" for english, hangul, python in LITERALS) + f"""

## 17. Randomness

{level_table(RANDOM, False)}

A choice written as a number stays a number (`pick from 1 or 2` →
`choice((1, 2,))`).

## 18. Chance

{level_table(CHANCE, False)}

`30%` means 300 times in a thousand. One decimal place is the finest you may
write (`30.5%`); anything finer is reported as `E0227` rather than rounded,
because a program must never quietly mean something you did not write. The
range is `0%` to `100%`, and outside it you get `E0228`. `100%` always happens
and `0%` never does. The comparison is between whole thousandths, so no
floating-point rounding can creep in.

A chance saved in a name is an ordinary true/false value, so the usual
condition words can ask about it: `if luck then show You win`.

## 19. Files

{level_table(FILES, False)}

A file path is always quoted. This is the one place the sentence level asks for
a quote character.

## 20. Modules

{level_table(MODULES, False)}

After `use random`, `use file` and `use zero_knowledge`, the four most asked
for are `use list`, `use text`, `use math` and `use date`. One line makes every
name below ready in both languages, and everything inside them is plain Python
builtins, so they run in the browser as they stand. `list`, `text`, `math` and
`date` are ordinary words, so they name a module only when they stand beside
`use`/`사용` and nothing else is left over on the line; `get the list of names`
and `What is the date today?` are sentences and print.

On a Wednesday, `weekday()` answers `Wednesday` and its Korean name `요일()`
answers `수요일`.
This is the one place in any bundled module where the two names hold different
values: a weekday name is a word rather than a number, so it has to be in some
language, and the name you write chooses which. The clock a browser hands
Python is UTC, so in the browser `today()` is today in UTC.

{module_helper_tables(False)}

`sort`, `reverse` and `remove` hand back a new list and leave the original
alone. To change the list itself, use the sentence statements `sort friends`,
`reverse friends` and `remove Mina from friends`.

## 21. Slow text

{level_table(SLOW_TEXT, False)}

Each character is printed on its own with a short pause after it. The pause is
0.04 seconds by default, 0.12 with `very`, and whatever you name with
`every 3 seconds` / `3초씩`.

## 22. Stories

{level_table(STORY, False)}

Inside `story:` **every line is text**. `wait 3 seconds` and `if ready` are not
commands there; they print, exactly as written. Writing something novel-like is
the whole point of the form, so a line of prose can never quietly turn into a
statement. Close the block with `end` / `끝`, or, if you opened it by
indenting, by ending the indentation. A blank line prints an empty line, names
you made earlier are still substituted into the text, and the colon may be the
plain `:` or the full-width `：` a Korean keyboard writes.

## 23. Named jobs

{level_table(JOBS, False)}

A named job gives a piece of program a name, so it can be run later by that
name. Python calls it a function (`def`).

`to`, `do`, `일` and `하기` are ordinary words, so a named job is recognized by
**structure and never by a word**: the opening `to` (or, in Korean, the `라는`
on the name and the `일` after it), the closing `:`, **and a block underneath**.
`to be honest` and `할 일이 많습니다` have none of that.

**Without a block it is not a job.** A heading such as `To do:` on its own is
printed as the line it is, and there is no one-line form.

The line that runs a job only runs one when the name is **a job this program
already made**. `do` and `run` decide nothing on their own. A Python `def` that
takes no arguments can be run the same way.

A name saved inside a job stays inside it, exactly as in a Python function.

A job **may be given one thing.** The header names it in front of the job name
(`to greet someone:`, `이름에게 인사하기라는 일:`), and the line that runs the
job hands it over the same way (`do greet with Mina`, `민수에게 인사하기 해줘`).
Giving a job the wrong number of things is refused with `E0235`, because the
Python `TypeError` it would otherwise cause happens at run time on a line that
looks right.

Sentence grammar has **no job that takes two things** and **no job that hands
something back** yet. Write a Python `def` when you need either.

## 24. Screen

{level_table(SCREEN, False)}

Clearing the screen sends a terminal control sequence, so somewhere that is not
a terminal it may show up as text. The box and the centred line count a Korean
character as two columns, so a Korean sentence comes out straight; the width is
40 columns.

## 25. The stopwatch

{level_table(TIMER, False)}

`start the timer` starts the clock and `elapsed` / `잰시간` / `걸린시간` reads
how many seconds have passed, to two decimal places. It is a value, so it works
in output, in a saved name, and in a condition (`if elapsed is greater than 3`).
Reading it without starting the clock is reported at compile time as `E0226`. A
name the program made itself always wins over the word.

## 26. Cooldowns

{level_table(COOLDOWN, False)}

One cooldown belongs to one name. `put door on cooldown for 3 seconds` remembers
the moment three seconds from now, and `is ready` / `쿨타임이 끝났으면` asks
whether that moment has passed. They are conditions, so they work with `when`,
`while`, `else if`, and the one-line form of all three. `wait for door` also
reads as an ordinary English sentence, so a name the program already saved as
something else is not read as a cooldown.

## 27. Every action word

Every spelling accepted for each action, with nothing left out.

{spelling_table(False)}

## 28. Korean particles

These endings are not treated as part of the name they follow.

{spellings(words("KOREAN_PARTICLES"))}

## 29. Typo recovery

For action words and connectors only, and only after Python has rejected the
line, NME retries once with a single edit repaired (one insertion, deletion,
substitution, or adjacent swap). If more than one repair is possible it repairs
nothing and points at the exact span instead. **Strings and comments are never
touched.**

## 30. Error codes

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
