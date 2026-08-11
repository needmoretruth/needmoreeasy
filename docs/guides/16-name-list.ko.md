# 16 — 이름 목록: 파일에서 줄 읽기

[English](16-name-list.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [13 — 파일](13-files.ko.md), [14 — JSON](14-json.ko.md)
- 주제 (Topic): 파일과 목록 / files and lists
- 결과물 (Result): 파일에서 이름 목록을 읽어 고르는 프로그램 / a program that reads a list of names from a file and picks from it

파일이 목록이 될 수 있습니다. 이름이 줄마다 하나씩 있으면, Python 메서드
하나 `splitlines()`가 파일 전체를 이름 목록으로 바꿔 줍니다.

## 단계

1. `names.txt` 파일을 만들고 이름을 줄마다 하나씩 프로그램 옆에
   저장합니다:

   ```text
   Mina
   Sana
   준호
   Yuna
   ```

2. 파일 전체를 읽고 줄로 나눕니다:

   ```text
   파일 사용 최신
   이름들 = 파일읽기("names.txt").splitlines()
   이름들 보여줘
   ```

   `nme r names`를 실행하세요. 콘솔에 `['Mina', 'Sana', '준호', 'Yuna']`가
   보입니다. `파일읽기`가 글을 돌려주고 `.splitlines()`가 줄바꿈마다
   잘라냅니다. 이 줄은 보통의 Python이라 쓰인 그대로 남습니다.

3. `for` 블록으로 목록을 차례로 훑습니다. 그 안에서도 문장형 NME가
   됩니다:

   ```text
   파일 사용 최신
   이름들 = 파일읽기("names.txt").splitlines()
   for 이름 in 이름들:
       안녕하세요 이름! 말해줘
   ```

   이름마다 인사가 하나씩 출력됩니다.

4. `랜덤선택`으로 이름 하나를 무작위로 고릅니다. `랜덤 사용 최신`이
   선택기를 불러오고, `3번:`이 선택을 반복합니다:

   ```text
   파일 사용 최신
   랜덤 사용 최신
   이름들 = 파일읽기("names.txt").splitlines()
   3번:
       이름 = 랜덤선택(이름들)
       안녕하세요 이름! 말해줘
   ```

5. 영어는 `file_read(...).splitlines()`로 읽고 `random_pick`으로 고릅니다:

   ```text
   use file latest
   use random latest
   names = file_read("names.txt").splitlines()
   3 times:
       show random_pick(names)
   ```

## 직접 해보기

`names.txt`에 이름을 두 개 더 넣고 다시 실행해 보세요. 목록과 고른 이름이
함께 늘어납니다.

## 배운 것

- `파일읽기(경로).splitlines()` / `file_read(path).splitlines()`가 파일의
  모든 줄을 목록으로 읽습니다.
- `for 이름 in 이름들:` 블록이 항목마다 차례로 실행되고, 그 안에서 문장형
  NME를 쓸 수 있습니다.
- `랜덤선택(이름들)` / `random_pick(names)`이 항목 하나를 무작위로
  고릅니다.
- `3번:` / `3 times:`가 선택을 반복해서 게임이 다시 물어볼 수 있게
  합니다.
