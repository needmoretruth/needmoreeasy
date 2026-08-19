# 63 — 도구: 명령 줄에서 말 받기

[English](63-argv.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★☆ (4/5)
- 선수 지식: [39 — JSON](39-json.ko.md), [31 — 미니 은행](31-bank.ko.md)
- 주제: 도구 다루기
- 결과물: `nme 실행 todo add "buy milk"`처럼 명령 줄에서 말을 받아 동작하는 할 일 도구

지금까지의 프로그램은 실행하는 동안 입력을 물었습니다. 진짜 도구는
시작할 때 지시를 읽습니다: `nme 실행 dice 6`은 아무것도 묻지 않고 육면체
주사위를 굴려야 합니다. 파일 이름 뒤에 적는 말들이 **명령 줄 인자**이고,
Python은 이것들을 `sys.argv` 목록에 넣습니다.

## 단계

1. `import sys`가 `sys.argv`를 줍니다: 프로그램 경로와 파일 뒤에 적은
   모든 단어. `greet.nme`로 저장하세요:

   ```nme
   # greet.nme — 명령 줄에서 받은 이름으로 인사하기.
   # 실행: nme 실행 greet 미나

   import sys

   name = sys.argv[1]
   show f"hello {name}"
   ```

   ```sh
   nme 실행 greet 미나
   ```

   ```text
   hello 미나
   ```

   `sys.argv[0]`은 프로그램 경로이고 `sys.argv[1]`부터가 진짜
   인자입니다 — 일반적인 `python greet.py 미나` 명령이 만드는 목록과
   같습니다.

2. 인자가 없을 때 혼란스러운 오류로 무너지면 안 됩니다. `len(sys.argv)`가
   단어 수를 세니, 읽기 전에 확인하세요. `dice.nme`로 저장하세요:

   ```nme
   # dice.nme — 명령 줄에서 원하는 면 수의 주사위 굴리기.
   # 실행: nme 실행 dice 6

   use random latest

   import sys

   if len(sys.argv) < 2:
       show "usage: nme 실행 dice <sides>"
   else:
       sides = int(sys.argv[1])
       show random_number(1, sides)
   ```

   ```sh
   nme 실행 dice 6
   ```

   ```text
   3
   ```

   `int(sys.argv[1])`이 단어 `"6"`을 숫자 6으로 바꿉니다([04](04-ask.ko.md)의
   변환과 같음). 확인이 없으면 `nme 실행 dice`는 빈 목록에서 오류로
   무너집니다. 올바른 명령을 보여 주는 "usage:" 줄이 인자 누락에
   친절하게 답하는 방법입니다.

3. 인자는 명령 줄 도구를 만듭니다: 프로그램 하나, 명령 여러 개. 할 일
   목록은 항목을 `todo.json`에 저장하고([39](39-json.ko.md)) `add`, `done`,
   `list` 명령을 받습니다. `todo.nme`로 저장하세요:

   ```nme
   # todo.nme — 명령 줄로 명령을 받는 할 일 도구.
   # 실행: nme 실행 todo add "buy milk"
   #       nme 실행 todo list
   #       nme 실행 todo done 1

   use file latest

   import sys

   todo_file = "todo.json"

   def load_todos():
       try:
           return json_load(todo_file)
       except Exception:
           return []

   def save_todos(todos):
       json_save(todo_file, todos)

   def show_todos(todos):
       for i, item in enumerate(todos):
           mark = "x" if item["done"] else " "
           show f"{i + 1}. [{mark}] {item['text']}"

   command = sys.argv[1] if len(sys.argv) > 1 else "list"
   todos = load_todos()

   if command == "add":
       text = sys.argv[2] if len(sys.argv) > 2 else "no text"
       todos.append({"text": text, "done": False})
       save_todos(todos)
       show f"added: {text}"
   elif command == "done":
       number = int(sys.argv[2]) - 1
       todos[number]["done"] = True
       save_todos(todos)
       show "marked done"
   else:
       show_todos(todos)
   ```

   `load_todos`는 파일이 아직 없으면 빈 목록을 돌려줍니다
   ([59](59-errors.ko.md)의 `try`/`except` 참고), `enumerate`는 목록에 1부터
   번호를 붙입니다. 항목마다 `text`와 `done` 깃발이 있는 dict이고 —
   `json_save`가 쓰고 `json_load`가 다시 읽는 바로 그 모양입니다.

4. 명령을 이어서 도구를 실행해 보세요:

   ```sh
   nme 실행 todo add "buy milk"
   nme 실행 todo add "learn argv"
   nme 실행 todo done 1
   nme 실행 todo
   ```

   ```text
   added: buy milk
   added: learn argv
   marked done
   1. [x] buy milk
   2. [ ] learn argv
   ```

   `"buy milk"`의 따옴표는 두 단어를 한 인자로 묶어 줍니다 — 따옴표가
   없으면 `buy`와 `milk`가 각각 `sys.argv[2]`, `sys.argv[3]`이 됩니다.
   `todo.json` 파일이 실행 사이에 기억하는 도구의 저장소입니다.

## 직접 해보기

목록을 비우는 `clear` 명령이나 번호 하나를 지우는 `rm` 명령
(`todos.pop(number)`)을 추가해 보세요. 주사위 도구를 면 수 목록을 받아
각각 굴리게 바꿔 보세요 (`for side in sys.argv[1:]`). 그다음 나만의
도구를 만들어 보세요 — 단위 변환기(`nme 실행 convert 100 km to mi`)가
좋은 첫 번째입니다.

## 배운 것

- `sys.argv`는 프로그램 경로와 파일 뒤에 적은 모든 단어를 담습니다.
- `len(sys.argv)`가 인자 누락을 막고, `usage:` 줄이 올바른 명령을
  알려 줍니다.
- `int(sys.argv[1])`이 인자 단어를 숫자로 바꿉니다.
- 명령 단어(`add`, `done`)와 데이터로 프로그램 하나가 여러 도구가
  됩니다 — [31](31-bank.ko.md) 미니 은행의 메뉴 루프를 시작 전에 미리
  받은 버전입니다.
- 따옴표가 여러 단어를 한 인자로 묶어 줍니다.
