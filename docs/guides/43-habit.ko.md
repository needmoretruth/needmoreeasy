# 43 — 프로젝트: 습관 체크

[English](43-habit.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [33 — 할 일](33-todo.ko.md), [23 — 모듈](23-modules.ko.md)
- 주제 (Topic): 프로젝트 / a project
- 결과물 (Result): add, check, streak, list, quit를 지원하고 저장 로직을 모듈 파일로 둔 JSON 저장 습관 추적기 / a JSON-persisted habit tracker with add, check, streak, list, quit, and a module file for the storage logic

[33](33-todo.ko.md) 가이드는 딕셔너리 목록의 할 일을 저장했고, [23](23-modules.ko.md) 가이드는 프로젝트를 여러 파일로 나눴습니다. 습관 추적기는 딕셔너리 하나 — 습관마다 연속 날수 — 를 JSON으로 저장합니다.

## 단계

1. 저장은 모듈에 둡니다. `store_ko.nme`이 `load()`와 `save(습관들)`을 내보냅니다. 습관은 `{이름: 날수}` 딕셔너리 항목이고, 파일이 없으면 `load()`가 `{}`를 돌려줍니다:

   ```text
   # store_ko.nme — 습관 추적기의 파일 저장 모듈
   import os
   파일 사용 최신
   def load():
       if os.path.exists("habits.json"):
           return json읽기("habits.json")
       return {}

   def save(습관들):
       json저장("habits.json", 습관들)
   ```

2. 프로젝트 전체입니다. `store_ko.nme` 옆에 `habit.ko.nme`을 저장합니다:

   ```text
   # habit.ko.nme — 실행 사이에도 남는 습관 추적기
   # 실행: nme r habit.ko

   from "store_ko.nme" import load, save
   습관들 = load()

   while True:
       말해 ""
       말해 "명령: add, check, streak, list, quit"
       물어봐 명령, "? "
       if 명령 == "add":
           물어봐 이름, "습관? "
           습관들[이름] = 0
           save(습관들)
           말해 f"{이름} 추가"
       elif 명령 == "check":
           물어봐 이름, "습관? "
           if 이름 in 습관들:
               습관들[이름] = 습관들[이름] + 1
               save(습관들)
               말해 f"{이름} 체크 ({습관들[이름]}일 연속)"
           else:
               말해 f"{이름} 습관 없음"
       elif 명령 == "streak":
           물어봐 이름, "습관? "
           말해 f"{이름}: {습관들.get(이름, 0)}일 연속"
       elif 명령 == "list":
           말해 f"습관 {len(습관들)}개"
           for 이름 in 습관들:
               말해 f"{이름}: {습관들[이름]}"
       elif 명령 == "quit":
           말해 "안녕!"
           break
   ```

   `add`는 습관을 0에서 시작하고, `check`는 1을 더해 저장하며, `streak`는 값을 읽고, `list`는 모든 짝을 방문합니다.

3. 파이프로 명령을 넣어 실행합니다:

   ```sh
   printf 'add\n물 마시기\ncheck\n물 마시기\ncheck\n물 마시기\nstreak\n물 마시기\nlist\nquit\n' | nme r habit.ko
   ```
   ```text

   명령: add, check, streak, list, quit
   ? 습관? 물 마시기 추가

   명령: add, check, streak, list, quit
   ? 습관? 물 마시기 체크 (1일 연속)

   명령: add, check, streak, list, quit
   ? 습관? 물 마시기 체크 (2일 연속)

   명령: add, check, streak, list, quit
   ? 습관? 물 마시기: 2일 연속

   명령: add, check, streak, list, quit
   ? 습관 1개
   물 마시기: 2

   명령: add, check, streak, list, quit
   ? 안녕!
   ```

   습관이 0에서 2로 자랐습니다 — `habits.json`에 `{"물 마시기": 2}`가 저장됩니다. 영어는 같은 메뉴를 `ask`와 `show`로 씁니다 — 전체 쌍은 [43-habit.md](43-habit.md)에 있습니다.

## 직접 해보기

습관을 0으로 되돌리는 `reset` 명령을 더해 보세요 — `elif` 한 갈래와 `save` 하나입니다.

## 배운 것

- 습관은 `{이름: 날수}` 딕셔너리이고, `습관들[이름] = 습관들[이름] + 1`이 연속을 늘립니다.
- 모듈 파일이 `load()`와 `save()`를 담고 주 프로그램이 그것을 가져옵니다.
- `json저장`이 변경마다 딕셔너리 전체를 저장합니다.
- `add`/`check`/`streak`/`list`/`quit` 메뉴의 `while True` 반복과 `break`가 추적기 전체를 움직입니다.
