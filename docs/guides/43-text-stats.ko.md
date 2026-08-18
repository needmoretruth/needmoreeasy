# 43 — 텍스트 통계: 글자와 단어

[English](43-text-stats.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★☆ (4/5)
- 선수 지식: [42 — 단어 세기](42-word-count.ko.md), [38 — 이름 목록](38-name-list.ko.md)
- 주제: 텍스트/데이터
- 결과물: 텍스트 파일을 읽어 글자 수, 단어 수, 가장 긴 단어, 가장 흔한 단어를 보고하는 프로그램

이야기는 먼저 문자열이고, 그다음 단어 목록입니다. 이 가이드는 텍스트 파일을 읽어 글자와 단어를 세고, Python이 가장 긴 단어와 가장 흔한 단어를 찾게 합니다.

## 단계

1. 프로그램 옆에 `story.txt`를 만들고 읽어 봅니다:

   ```nme
   the sun rose over the quiet town
   the town woke and the birds sang
   the children ran to the playground
   ```

   ```nme
   파일 사용 최신
   글 = 파일읽기("story.txt")
   단어들 = 글.split()
   보여줘 f"글자 수: {len(글)}"
   보여줘 f"단어 수: {len(단어들)}"
   ```

   `글자 수: 101`과 `단어 수: 20`이 출력됩니다. `파일읽기`([37](37-files.ko.md))가
   글을 돌려주고 `.split()`이 단어 목록을 잘라냅니다.

2. 가장 긴 단어와 가장 흔한 단어. `max(단어들, key=len)`은 길이로 비교하고,
   `collections.Counter`([64](64-python-packages.ko.md))가 세며 `most_common(1)`이 최상위 짝을 돌려줍니다:

   ```nme
   파일 사용 최신
   from collections import Counter
   글 = 파일읽기("story.txt")
   단어들 = 글.split()
   긴단어 = max(단어들, key=len)
   세기 = Counter(단어들)
   보여줘 f"가장 긴 단어: {긴단어}"
   보여줘 세기.most_common(1)
   ```

   `가장 긴 단어: playground`과 `[('the', 6)]`이 출력됩니다.

3. 이제 전체 보고서를 한 파일에 씁니다. `텍스트통계.nme`로 저장하고 실행합니다:

   ```nme
   # 텍스트통계.nme — 텍스트 파일의 글자와 단어.
   # 실행: nme 실행 텍스트통계
   # story.txt 파일이 같은 폴더에 있어야 합니다.

   파일 사용 최신
   from collections import Counter

   # 파일 전체를 문자열 하나로 읽습니다.
   글 = 파일읽기("story.txt")
   보여줘 f"글자 수: {len(글)}"

   # 단어 목록으로 나눕니다.
   단어들 = 글.split()
   보여줘 f"단어 수: {len(단어들)}"

   # 길이로 비교한 가장 긴 단어.
   긴단어 = max(단어들, key=len)
   보여줘 f"가장 긴 단어: {긴단어} ({len(긴단어)}글자)"

   # 가장 흔한 단어와 그 횟수.
   세기 = Counter(단어들)
   흔한단어, 흔한횟수 = 세기.most_common(1)[0]
   보여줘 f"가장 흔한 단어: {흔한단어} ({흔한횟수}번)"

   # 평균 단어 길이를 풀어서 씁니다.
   합계 = 0
   for 단어 in 단어들:
       합계 = 합계 + len(단어)
   보여줘 f"평균 단어 길이: {합계 / len(단어들)}"
   ```

   `most_common(1)[0]`은 한 요소 목록에서 유일한 짝을 꺼내고, 마지막 반복은 각 단어의 길이를 모두 더합니다.

   ```sh
   nme 실행 텍스트통계
   ```

   ```text
   글자 수: 101
   단어 수: 20
   가장 긴 단어: playground (10글자)
   가장 흔한 단어: the (6번)
   평균 단어 길이: 4.05
   ```

   파일 하나, 질문 네 개의 답입니다. 영어 가이드는 `file_read`와 `show`로 같은 Python 호출을 씁니다 — 전체 쌍은 [43-text-stats.md](43-text-stats.md)에 있습니다.

## 직접 해보기

`min(단어들, key=len)`으로 `가장 짧은 단어` 줄을 더하고, `for` 반복 하나로 네 글자보다 긴 단어가 몇 개인지 세어 보세요.

## 배운 것

- `파일읽기(경로)`가 텍스트 파일 전체를 문자열 하나로 돌려줍니다.
- `len(글)`은 글자 수, `글.split()`은 단어 목록을 만듭니다.
- `max(단어들, key=len)`이 길이를 비교해 가장 긴 단어를 찾습니다.
- `Counter(단어들).most_common(1)`이 가장 흔한 단어와 그 횟수를 돌려줍니다.
