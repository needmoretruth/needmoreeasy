# 59 — 편집기 — 아주 작은 텍스트 에디터

[English](59-editor.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★★ (5/5)
- 선수 지식: [47 — 진행](47-progress.ko.md), [50 — 문자열](50-strings.ko.md)
- 주제: TUI/터미널 앱
- 결과물: 버퍼를 두고 줄을 추가·삭제·목록·저장하는 줄 단위 에디터

텍스트 파일은 하나의 긴 문자열입니다. 에디터는 버퍼로 일합니다 — 같은
글을 줄 목록으로 나눈 것, 각 줄이 한 행입니다. 그 행들을 추가하고,
삭제하고, 나열한 뒤 `save`로 버퍼 전체를 파일에 다시 씁니다.

## 단계

1. `"\n".join(줄들)`은 줄들을 줄바꿈으로 이어 하나의 글을 만들고,
   `.splitlines()`은 정확히 반대입니다. `안녕`, `세계!`, `['안녕', '세계!']`가
   출력됩니다:

   ```text
   줄들 = ["안녕", "세계!"]
   글 = "\n".join(줄들)
   말해 글
   다시 = 글.splitlines()
   말해 다시
   ```

2. 버퍼는 실행 사이에도 파일에 남습니다. [13](13-files.ko.md)에서처럼
   저장된 파일이 줄 목록이 되고, 첫 실행은 빈 목록으로 시작합니다:

   ```text
   파일 사용 최신
   import os

   if os.path.exists("notes.txt"):
       줄들 = 파일읽기("notes.txt").splitlines()
   else:
       줄들 = []
   ```

3. 명령은 한 줄로 들어옵니다. `줄.split()`이 공백으로 나누고, `parts[0]`이
   명령어이고, `" ".join(parts[1:])`이 나머지를 다시 잇습니다 — `add`의
   글입니다. `add`를 출력하고, 그다음 `안녕 세계`를 출력합니다.
   [49](49-tokens.ko.md)도 같은 방식으로 토큰을 나눴습니다:

   ```text
   줄 = "add 안녕 세계"
   parts = 줄.split()
   말해 parts[0]
   말해 " ".join(parts[1:])
   ```

4. `remove <번호>`에는 숫자가 필요합니다. `int(parts[1])`이 입력한 글을
   정수로 바꾸고, `- 1`이 0부터 세는 번호로 만들며, `줄들.pop(i)`이 그
   행을 지웁니다. `['가', '다']`가 출력됩니다 — 2번 줄이 지워졌습니다:

   ```text
   줄들 = ["가", "나", "다"]
   n = 2
   i = int(n) - 1
   줄들.pop(i)
   말해 줄들
   ```

5. 에디터 전체입니다. `editor.ko.nme`으로 저장합니다:

   ```text
   # editor.ko.nme — 아주 작은 줄 단위 텍스트 에디터.
   # 실행: nme 실행 editor.ko
   # add <글>, list, remove <번호>, save, quit 중 하나를 입력하세요.

   파일 사용 최신
   import os

   # 저장된 버퍼를 불러오거나, 빈 버퍼로 시작합니다.
   if os.path.exists("notes.txt"):
       줄들 = 파일읽기("notes.txt").splitlines()
   else:
       줄들 = []

   말해 "작은 에디터 — notes.txt"
   while True:
       말해 "명령: add, list, remove, save, quit"
       물어봐 줄, "> "
       parts = 줄.split()
       명령 = parts[0] if parts else ""
       if 명령 == "add":
           # 명령어 뒤의 글자가 새 줄입니다.
           글 = " ".join(parts[1:])
           줄들.append(글)
           말해 f"{len(줄들)}번 줄 추가"
       elif 명령 == "list":
           말해 f"줄 {len(줄들)}개"
           for i in range(len(줄들)):
               말해 f"{i + 1}: {줄들[i]}"
       elif 명령 == "remove":
           # N번 줄 삭제: 1이 첫 줄입니다.
           i = int(parts[1]) - 1
           if i >= 0 and i < len(줄들):
               줄들.pop(i)
               말해 "삭제"
           else:
               말해 "그런 줄 없음"
       elif 명령 == "save":
           파일쓰기("notes.txt", "\n".join(줄들))
           말해 "저장"
       elif 명령 == "quit":
           말해 "안녕!"
           break
       else:
           말해 "알 수 없는 명령"
   ```

   `add`는 목록에 더하고, `list`는 `f"{i + 1}:"`으로 1번부터 번호를 매겨
   훑으며, `save`는 1단계의 join을 거꾸로 합니다 — `"\n".join(줄들)`을
   파일에 씁니다.

6. 파이프로 명령을 넣어 실행합니다. `add 안녕`과 `add 세계!`가 버퍼를
   채우고, `remove 1`이 `안녕`을 지우며, `save`가 `세계!`를 남깁니다:

   ```sh
   printf 'add 안녕\nadd 세계!\nlist\nremove 1\nlist\nsave\nquit\n' | nme 실행 editor.ko
   ```

   ```text
   작은 에디터 — notes.txt
   명령: add, list, remove, save, quit
   > 1번 줄 추가
   명령: add, list, remove, save, quit
   > 2번 줄 추가
   명령: add, list, remove, save, quit
   > 줄 2개
   1: 안녕
   2: 세계!
   명령: add, list, remove, save, quit
   > 삭제
   명령: add, list, remove, save, quit
   > 줄 1개
   1: 세계!
   명령: add, list, remove, save, quit
   > 저장
   명령: add, list, remove, save, quit
   > 안녕!
   ```

   `cat notes.txt`는 저장된 버퍼를 보여 줍니다 — `세계!` 한 줄입니다.
   에디터를 다시 실행하고 `list`를 입력하면 그 줄이 다시 불러와집니다.
   파일과 버퍼가 실행 사이에도 같게 유지됩니다.

## 직접 해보기

버퍼 전체를 대문자로 바꿔 저장하는 `upper` 명령(`[l.upper() for l in 줄들]`)
을 추가해 보세요. 또는 목록을 비우는 `clear` 명령을 추가해 보세요.
`save`가 몇 줄을 썼는지 출력하게 해 보세요.

## 배운 것

- 버퍼는 줄 목록이고, `"\n".join(버퍼)`로 저장하고 `.splitlines()`으로
  불러옵니다.
- `줄.split()`이 명령어와 글을 나누고, `" ".join(parts[1:])`이 글을 다시
  잇습니다.
- `int(...) - 1`이 입력한 줄 번호를 목록 번호로 바꾸고, `줄들.pop(i)`이
  행을 지웁니다.
- `save`가 파일을 쓰므로 버퍼가 실행 사이에도 남습니다.
