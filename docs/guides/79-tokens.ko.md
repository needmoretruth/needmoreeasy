# 79 — 컴파일러: 토큰 읽기

[English](79-tokens.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★☆ (4/5)
- 선수 지식: [78 — 표현식](78-expressions.ko.md), [77 — 첫 컴파일러](77-compiler.ko.md)
- 주제: 언어 만들기
- 결과물: 명령 줄을 토큰으로 나누고 실행을 보내는 프로그램

[78](78-expressions.ko.md)은 식을 단어로 나누었고,
[84](84-bootstrap.ko.md)은 명령 줄을 나눕니다. 진짜 컴파일러도 같은
방식으로 시작합니다: 소스 글을 읽고 **토큰**으로 자릅니다 — 의미의 가장
작은 조각. 이 가이드는 줄을 토큰으로 나누고 명령마다 실행을 보내는 명령
리더를 만듭니다. 컴파일러가 입력을 한 토큰씩 읽는 방식과 같습니다.

## 단계

1. `split()`이 줄을 공백에서 자릅니다. 단어 하나가 토큰 하나이고, 목록이
   순서를 지킵니다 — `"move 3"`이 `["move", "3"]`이 됩니다:

   ```nme
   line = "move 3"
   토큰들 = line.split()
   말해 f"토큰 {len(토큰들)}개: {토큰들}"
   ```

   `토큰 2개: ['move', '3']`이 출력됩니다. 첫 토큰이 명령이고, 나머지
   토큰이 인자입니다.

2. 함수가 토큰을 받아 첫 토큰으로 실행을 보냅니다 — [08](08-if.ko.md)
   가이드의 `if`/`elif` 사슬과 같지만, 답 대신 토큰을 비교합니다.
   `토큰들[1]`이 첫 인자입니다:

   ```nme
   def 토큰실행(토큰들):
       command = 토큰들[0]
       if command == "move":
           amount = int(토큰들[1])
           말해 f"{amount}만큼 이동"
       elif command == "turn":
           말해 f"{토큰들[1]}쪽으로 회전"
       elif command == "say":
           말해 토큰들[1]
       else:
           말해 f"알 수 없음: {command}"
   ```

3. `int(토큰들[1])`이 두 번째 토큰을 숫자로 바꿔 `move 3`이 진짜 걸음을
   더하게 합니다. 상태는 딕셔너리에 둡니다 — `로봇 = {"direction": "north",
   "steps": 0}` — 그리고 `move`가 `로봇["steps"]`에 더합니다. 전체
   프로그램입니다. `tokens.ko.nme`으로 저장합니다:

   ```nme
   # tokens.ko.nme — 명령 줄을 토큰으로 나누고 실행합니다.
   # 실행: nme 실행 tokens.ko
   # 토큰은 줄의 한 단어이고, split()이 목록을 만듭니다.
   # 첫 토큰이 명령이고, 나머지 토큰이 인자입니다.
   # 진짜 컴파일러도 글을 읽고, 토큰으로 자르고, 첫 토큰으로 갈라집니다.

   def 토큰실행(토큰들, 로봇):
       command = 토큰들[0]
       if command == "move":
           amount = int(토큰들[1])
           로봇["steps"] = 로봇["steps"] + amount
           말해 f"{amount}만큼 이동 — 총 {로봇['steps']}걸음"
       elif command == "turn":
           로봇["direction"] = 토큰들[1]
           말해 f"{로봇['direction']}쪽으로 회전"
       elif command == "say":
           말해 토큰들[1]
       elif command == "where":
           말해 f"{로봇['direction']} {로봇['steps']}걸음"
       elif command == "help":
           말해 "move <n>, turn <dir>, say <text>, where, help, quit"
       else:
           말해 f"알 수 없음: {command}"

   로봇 = {"direction": "north", "steps": 0}
   말해 "장난감 로봇: help를 입력하세요."
   while True:
       물어봐 line, "> "
       if line == "":
           continue
       if line == "quit":
           말해 "안녕!"
           break
       토큰들 = line.split()
       토큰실행(토큰들, 로봇)
   ```

   `move`는 `로봇["steps"]`에 더하고, `turn`은 방향을 바꾸며, `say`는 줄의
   나머지를 출력하고, `where`는 상태를 알려 줍니다. 빈 줄은 `continue`로
   반복을 계속하고, `quit`이 멈춥니다.

4. 파이프로 명령을 넣어 실행합니다:

   ```sh
   printf 'move 3\nturn left\nsay hi\nwhere\nquit\n' | nme 실행 tokens.ko
   ```

   ```text
   장난감 로봇: help를 입력하세요.
   > 3만큼 이동 — 총 3걸음
   > left쪽으로 회전
   > hi
   > left 3걸음
   > 안녕!
   ```

   나누기 → 실행 보내기 → 반복이 모든 컴파일러의 프런트엔드입니다.

## 직접 해보기

`로봇["steps"]`을 `0`으로 되돌리는 `reset` 가지를 추가해 보세요. 또는
`토큰들[1]`을 읽기 전에 인자 개수를 확인하게 해 보세요.

## 배운 것

- 토큰은 의미 있는 단어 하나이고, `line.split()`이 토큰 목록을 만듭니다.
- `토큰들[0]`이 명령이라 실행을 보내고, 나머지는 인자입니다.
- `int(토큰들[1])`이 인자를 숫자로 바꿉니다.
- 나누기 → 실행 보내기 → 반복이 진짜 토크나이저와 파서의 씨앗입니다.
