# 56 — 기록: 사건 기록

[English](56-log.md) | 한국어

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [13 — Files](13-files.md), [35 — Diary](35-diary.md)
- 주제 (Topic): 파일/기록 / files & logging
- 결과물 (Result): 프로그램이 실행될 때마다 datetime과 file_write로 로그 파일에 날짜 줄을 덧붙이기 / appending a dated line to a log file each time the program runs, using datetime and file_write

가이드 [35](35-diary.md)의 일기는 날마다 파일 하나를 씁니다. **로그**는
반대입니다: 사건마다 줄 하나가 쌓이는, 커지는 파일 하나. 언제 무슨 일이
있었는지 보여 줍니다.

## 단계

1. `datetime`에게 지금을 물어보고 텍스트로 만드세요. `datetime.now()`는 바로
   이 순간이고, `strftime`(가이드 [24](24-python-packages.md))이 형식을
   정합니다 — `%Y` 연도, `%m` 달, `%d` 날짜, `%H` 시, `%M` 분:

   ```text
   from datetime import datetime
   now = datetime.now()
   stamp = now.strftime("%Y-%m-%d %H:%M")
   말해 stamp
   ```

2. 가이드 [13](13-files.md)의 `file_write`는 파일 전체를 덮어씁니다. 그래서
   덧붙이기는 예전 로그를 읽고, 줄 하나를 더하고, 전부 다시 씁니다:

   ```text
   use file latest
   log = file_read("log.txt")
   file_write("log.txt", log + stamp + " - program started\n")
   ```

   Python의 `open(경로, "a")`은 한 번에 덧붙입니다 — `a`는 쓰기가 아니라
   추가 — 하지만 읽고-쓰는 방법은 프로그램이 로그를 다시 읽어 메뉴에 보여 줄
   수 있게 합니다.

3. 전체 기록기는 `add`할 때마다 날짜 줄을 덧붙입니다. `log.txt`가 아직 없어도
   첫 실행이 실패하면 안 되므로 `os.path.exists`로 빈 문자열에서 시작합니다.
   `log.ko.nme`로 저장하세요:

   ```text
   # log.ko.nme — 작은 사건 기록기.
   # 실행: nme r log.ko

   use file latest
   from datetime import datetime
   import os
   if os.path.exists("log.txt"):
       log = file_read("log.txt")
   else:
       log = ""

   while True:
       물어봐 choice, "(add, show, quit) "
       if choice == "add":
           물어봐 event, "무슨 일이 있었나요? "
           stamp = datetime.now().strftime("%Y-%m-%d %H:%M")
           log = log + stamp + " - " + event + "\n"
           file_write("log.txt", log)
           말해 "기록: " + stamp + " - " + event
       elif choice == "show":
           말해 "log.txt에 " + str(len(log.splitlines())) + "줄이 있습니다:"
           for line in log.splitlines():
               말해 line
       else:
           말해 "안녕!"
           break
   ```

4. 두 번 실행하며 각각 다른 사건을 추가하세요 — 두 번째 실행에도 첫 사건이
   남아 있어 로그가 커지는 모습이 보입니다:

   ```sh
   printf 'add\n물 주기\nshow\nquit\n' | nme r log.ko
   printf 'add\n엄마에게 전화\nshow\nquit\n' | nme r log.ko
   ```

   ```text
   (add, show, quit) 무슨 일이 있었나요? 기록: 2026-08-11 14:05 - 물 주기
   (add, show, quit) log.txt에 1줄이 있습니다:
   2026-08-11 14:05 - 물 주기
   (add, show, quit) 안녕!
   (add, show, quit) 무슨 일이 있었나요? 기록: 2026-08-11 14:05 - 엄마에게 전화
   (add, show, quit) log.txt에 2줄이 있습니다:
   2026-08-11 14:05 - 물 주기
   2026-08-11 14:05 - 엄마에게 전화
   (add, show, quit) 안녕!
   ```

   시간표시는 진짜입니다 — 직접 실행하면 `log.txt`에 각 `add`의 실제 시각이
   기록됩니다.

## 직접 해보기

날짜별 사건 수를 세어 보세요: 시간표시를 `strftime("%Y-%m-%d")`로 바꾸고,
가이드 [36](36-word-count.md)의 dict 세기로 같은 날짜 줄이 몇 개인지 세세요.

## 배운 것

- `datetime.now().strftime(형식)`이 지금 순간을 텍스트로 만듭니다.
- 덧붙이기는 읽기 + 새 줄 + `file_write`입니다. `file_write`가 파일 전체를
  덮어쓰기 때문입니다.
- `open(경로, "a")`이 바로 덧붙입니다.
- `os.path.exists`가 첫 실행을 빈 로그로 시작하게 해 줍니다.
