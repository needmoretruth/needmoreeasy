# 35 — 일기: 날짜별 메모

[English](35-diary.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [24 — Python 패키지](24-python-packages.ko.md), [13 — 파일](13-files.ko.md)
- 주제 (Topic): 파일 / files
- 결과물 (Result): 각 날의 메모를 날짜별 파일에 저장하고 다시 읽는 일기 / a diary that saves each day's note to a dated file and can read it back

일기는 하루에 파일 하나입니다. Python의 `datetime` 패키지가 오늘 날짜를
알려 주고, [13](13-files.ko.md) 가이드의 `파일` 도우미가 메모를 저장하고
읽습니다. 둘을 합치면 메모마다 그 날짜로 이름 지은 파일에 들어갑니다.

## 단계

1. 오늘 날짜를 글자로 받습니다. `datetime` 패키지의 `date.today()`
   ([24](24-python-packages.ko.md) 가이드)가 진짜 날짜를 알고, `str()`이
   그것을 `2026-08-11` 같은 글자로 바꿉니다:

   ```text
   from datetime import date
   오늘 = str(date.today())
   보여줘 오늘
   ```

   출력은 진짜 오늘 날짜라서 어떤 날에 실행해도 그날 날짜가 나옵니다.

2. 메모를 날짜가 들어간 파일에 저장합니다. `f"..."` 안의 `f`가 `{오늘}`을
   파일 이름에 채워 넣고, [13](13-files.ko.md) 가이드의 `파일쓰기`와
   `파일읽기`가 메모를 저장하고 불러옵니다:

   ```text
   파일 사용 최신
   from datetime import date
   오늘 = str(date.today())
   파일쓰기(f"일기-{오늘}.txt", "친구와 커피를 마셨어요.")
   보여줘 파일읽기(f"일기-{오늘}.txt")
   ```

   일기를 쓴 날마다 폴더에 새 파일이 생깁니다.

3. 일기 전체는 [22](22-terminal-menu.ko.md) 가이드의 터미널 메뉴처럼 메뉴
   반복 하나입니다: `add`는 메모를 저장하고, `read`는 오늘이나 지난 날짜를
   보여 주며, `quit`는 반복을 끝냅니다. `ilgi.nme`로 저장하세요:

   ```text
   # 일기: 하루 메모가 날짜별 파일에 저장됩니다.
   # 실행: nme r ilgi

   파일 사용 최신
   from datetime import date

   보여줘 "일기 메뉴 (add, read, quit)"
   while True:
       물어봐 action, "고르기 (add, read, quit): "
       만약 action == "add":
           물어봐 note, "메모: "
           오늘 = str(date.today())
           파일쓰기(f"일기-{오늘}.txt", note)
           보여줘 "저장됨: " + f"일기-{오늘}.txt"
       elif action == "read":
           물어봐 when, "언제 글을 읽을까요? (today, date): "
           if when == "today":
               오늘 = str(date.today())
               보여줘 파일읽기(f"일기-{오늘}.txt")
           else:
               물어봐 day, "날짜 (YYYY-MM-DD): "
               보여줘 파일읽기("일기-" + day + ".txt")
       else:
           보여줘 "안녕"
           break
   ```

4. 실행하고 메뉴에 답 세 개를 넣어 보세요 — 메모 추가, 오늘 읽기, 끝내기:

   ```sh
   printf 'add\n친구와 커피를 마셨어요.\nread\ntoday\nquit\n' | nme r ilgi
   ```

   ```text
   일기 메뉴 (add, read, quit)
   고르기 (add, read, quit): 메모: 저장됨: 일기-2026-08-11.txt
   고르기 (add, read, quit): 언제 글을 읽을까요? (today, date): 친구와 커피를 마셨어요.
   고르기 (add, read, quit): 안녕
   ```

   파일 이름에 진짜 날짜가 들어갑니다 — 여러분이 실행하면 그날 날짜가
   출력됩니다.

5. 지난 날짜를 읽는 것은 반대로 똑같습니다: `물어봐`가 날짜를 받고
   `파일읽기`가 그 정확한 파일을 엽니다. 그것이 `read date` 갈래입니다:

   ```text
   파일 사용 최신
   물어봐 when, "언제 글을 읽을까요? (today, date): "
   if when == "today":
       오늘 = str(date.today())
       보여줘 파일읽기(f"일기-{오늘}.txt")
   else:
       물어봐 day, "날짜 (YYYY-MM-DD): "
       보여줘 파일읽기("일기-" + day + ".txt")
   ```

   이 갈래는 `when`을 확인해서 오늘이 아닐 때만 `일기-<날짜>.txt`를 엽니다.

## 직접 해보기

폴더에 있는 모든 일기 파일을 보여 주는 `list` 선택지를 추가해 보세요.
`from pathlib import Path`와 `Path(".").glob("일기-*.txt")`를 도는 `for`
반복이 날짜별 파일을 나열합니다:

```text
from pathlib import Path
for p in sorted(Path(".").glob("일기-*.txt")):
    보여줘 p.name
```

메뉴 안내에 `list`를 넣고 이 반복을 실행하는 `elif action == "list":`
갈래를 새로 만드세요.

## 배운 것

- `from datetime import date`와 `str(date.today())`가 오늘 날짜를 글자로
  알려 줍니다.
- `f"일기-{오늘}.txt"`가 날짜로 파일 이름을 만듭니다.
- `파일쓰기`가 그 파일에 메모를 저장하고 `파일읽기`가 다시 읽습니다.
- `while True:` 메뉴가 하루 한 메모를 커지는 일기로 만듭니다.
