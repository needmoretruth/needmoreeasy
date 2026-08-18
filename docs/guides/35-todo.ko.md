# 35 — 할 일: 커지는 프로젝트

[English](35-todo.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★☆ (4/5)
- 선수 지식: [33 — 습관 체크](33-habit.ko.md), [30 — 상점](30-shop.ko.md)
- 주제: 프로젝트
- 결과물: add, done, list를 지원하고 저장 로직을 모듈 파일로 둔 JSON 저장 할 일 목록

[41](41-address-book.ko.md)은 `json저장`으로 딕셔너리 목록을 저장하고,
[61](61-modules.ko.md)은 코드를 모듈로 나눕니다. 이 가이드는 둘을
프로젝트로 키웁니다: 실행 사이에도 남는 할 일 목록.

## 단계

1. 저장은 모듈에 둡니다. `store_ko.nme`이 `load()`와 `save(할일)`을 내보냅니다:

   ```nme
   # store_ko.nme — 할 일 목록의 파일 저장 모듈

   import os
   파일 사용 최신

   def load():
       if os.path.exists("todos.json"):
           return json읽기("todos.json")
       return []

   def save(할일):
       json저장("todos.json", 할일)
   ```

2. 프로젝트 전체입니다. `todo.ko.nme`을 `store_ko.nme` 옆에 저장합니다:

   ```nme
   # todo.ko.nme — 실행 사이에도 남는 할 일 목록
   # 실행: nme 실행 todo.ko

   from "store_ko.nme" import load, save
   할일들 = load()

   while True:
       말해 ""
       말해 "명령: add, done, list, quit"
       물어봐 명령, "? "
       if 명령 == "add":
           물어봐 내용, "할 일? "
           할일들.append({"text": 내용, "done": False})
           save(할일들)
           말해 f"{내용} 추가"
       elif 명령 == "done":
           물어봐 번호, "번호? "
           i = int(번호)
           if i >= 0 and i < len(할일들):
               할일들[i]["done"] = True
               save(할일들)
               말해 f"{할일들[i]['text']} 완료"
           else:
               말해 f"번호 {i} 없음"
       elif 명령 == "list":
           말해 f"할 일 {len(할일들)}개"
           for i in range(len(할일들)):
               if 할일들[i]["done"]:
                   말해 f"{i}: [x] {할일들[i]['text']}"
               else:
                   말해 f"{i}: [ ] {할일들[i]['text']}"
       elif 명령 == "quit":
           말해 "안녕!"
           break
       else:
           말해 "알 수 없는 명령"
   ```

   `add`는 딕셔너리를 추가하고 즉시 저장합니다. `done`은 번호로 할 일을
   표시하고, `and` 범위 검사로 번호가 목록 안에 있는지 확인합니다.

3. 파이프로 명령을 넣어 실행합니다:

   ```sh
   printf 'add\nbuy milk\ndone\n0\nlist\nquit\n' | nme 실행 todo.ko
   ```

   ```text
   명령: add, done, list, quit
   ? 할 일? buy milk 추가

   명령: add, done, list, quit
   ? 번호? buy milk 완료

   명령: add, done, list, quit
   ? 할 일 1개
   0: [x] buy milk

   명령: add, done, list, quit
   ? 안녕!
   ```

   할 일이 `todos.json`에 저장되어 다음 실행이 되살립니다. 영어는 같은 메뉴를 `ask`와 `show`로 씁니다; 전체 쌍은 [영어 가이드](35-todo.md)에 있습니다.

## 직접 해보기

`todos`를 `[]`로 바꾸고 저장하는 `clear` 명령을 더해 보세요 — `elif` 한 갈래. 그리고 `list`가 남은 할 일 개수를 세게 해 보세요.

## 배운 것

- 프로젝트는 주 프로그램과 저장 모듈로 나뉘고, 모듈은 명확한 인터페이스를 갖습니다.
- `json저장`이 딕셔너리 목록을 저장하고 `load()`가 다음 실행에서 되살립니다.
- `int(번호)`와 `and` 범위 검사가 번호 명령을 안전하게 합니다.
