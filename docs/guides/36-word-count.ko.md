# 36 — 단어 세기: 단어 빈도

[English](36-word-count.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★☆ (4/5)
- 선수 지식: [16 — 이름 목록](16-name-list.ko.md), [13 — 파일](13-files.ko.md)
- 주제: 데이터
- 결과물: 텍스트 파일을 읽어 단어가 각각 몇 번 나오는지 세는 프로그램

단어 세기는 많은 글 프로그램의 첫걸음입니다. 파일을 읽고 단어로 나누고
셉니다: 보통 dict로 직접 셀 수 있고, `collections.Counter`가 한 번에 같은
일을 해 줍니다 — 가장 흔한 다섯 단어까지 포함해서요.

## 단계

1. 프로그램 옆에 작은 텍스트 파일 `story.txt`를 만듭니다:

   ```text
   the cat sat on the mat
   the dog ran past the cat
   a bird sang on the roof
   ```

2. 파일 전체를 읽고 단어로 나눕니다. `파일읽기`가 글을 돌려주고
   ([13](13-files.ko.md) 가이드) `.split()`이 빈칸마다 잘라냅니다:

   ```text
   파일 사용 최신
   글 = 파일읽기("story.txt")
   단어들 = 글.split()
   보여줘 "단어 수: " + str(len(단어들))
   ```

   파일에는 단어가 18개 있습니다.

3. dict로 직접 셉니다. 빈 dict `{}`와 `for` 반복 하나: 단어가 이미 있으면
   1을 더하고, 없으면 1에서 시작합니다. `셈표["the"]`가 개수를 읽습니다:

   ```text
   파일 사용 최신
   글 = 파일읽기("story.txt")
   단어들 = 글.split()
   셈표 = {}
   for 단어 in 단어들:
       if 단어 in 셈표:
           셈표[단어] = 셈표[단어] + 1
       else:
           셈표[단어] = 1
   보여줘 "'the' 개수: " + str(셈표["the"])
   ```

   `셈표`가 단어마다 개수를 담습니다. [16](16-name-list.ko.md) 가이드는
   목록을 썼습니다. dict는 이름이 붙은 칸이 있는 목록입니다.

4. `collections.Counter`가 한 줄로 같은 셈을 하고 `most_common(5)`로 가장
   흔한 다섯 단어를 더해 줍니다. [24](24-python-packages.ko.md) 가이드의
   `date` 가져오기처럼 불러옵니다:

   ```text
   파일 사용 최신
   from collections import Counter
   글 = 파일읽기("story.txt")
   단어들 = 글.split()
   세기 = Counter(단어들)
   보여줘 세기.most_common(5)
   ```

   항목마다 `단어, 횟수` 짝이 나옵니다 — `the`는 5번, `cat`과 `on`은 각각
   2번, 나머지 단어는 각각 1번입니다.

5. 전체 프로그램은 파일을 읽고 두 방식으로 세고 작은 보고서를 출력합니다.
   `daneo.nme`로 저장하세요:

   ```text
   # 텍스트 파일에서 단어가 각각 몇 번 나오는지 셉니다.
   # 실행: nme 실행 daneo

   파일 사용 최신
   from collections import Counter

   글 = 파일읽기("story.txt")
   단어들 = 글.split()

   보여줘 "단어 수: " + str(len(단어들))

   셈표 = {}
   for 단어 in 단어들:
       if 단어 in 셈표:
           셈표[단어] = 셈표[단어] + 1
       else:
           셈표[단어] = 1

   보여줘 "서로 다른 단어 수: " + str(len(셈표))
   보여줘 "'the' 개수: " + str(셈표.get("the", 0))

   세기 = Counter(단어들)
   보여줘 "가장 많은 다섯 단어:"
   for 단어, 횟수 in 세기.most_common(5):
       보여줘 f"{단어}: {횟수}"
   ```

   `셈표.get("the", 0)`도 개수를 읽지만, 단어가 없어도 오류 대신 0을
   돌려줍니다.

6. `story.txt`가 있는 폴더에서 실행합니다:

   ```sh
   nme 실행 daneo
   ```

   ```text
   단어 수: 18
   서로 다른 단어 수: 12
   'the' 개수: 5
   가장 많은 다섯 단어:
   the: 5
   cat: 2
   on: 2
   sat: 1
   mat: 1
   ```

   `for` 반복 두 개는 똑같이 셉니다: dict가 아이디어를 보여 주고,
   `Counter`가 그것을 한 번의 호출로 바꿉니다.

## 직접 해보기

`The`와 `the`가 한 단어로 세어지게 대소문자 무시 셈으로 바꿔 보세요:
전체 프로그램에서 `단어들 = 글.split()`을 `단어들 = 글.lower().split()`로
고치면 `the`가 두 가지 철자가 아니라 5번으로 세어집니다.

## 배운 것

- `파일읽기(경로)`가 텍스트 파일 전체를 문자열 하나로 돌려줍니다.
- `글.split()`이 그 문자열을 단어 목록으로 나눕니다.
- dict가 단어를 셉니다: `if 단어 in 셈표`면 1을 더하고, 아니면 1에서
  시작합니다.
- `Counter(단어들).most_common(5)`가 모든 단어를 세고 상위 다섯을
  돌려줍니다.
