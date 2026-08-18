# 53 — 집합: 중복 없는 값

[English](53-sets.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★★ (5/5)
- 선수 지식: [42 — 단어 세기](42-word-count.ko.md), [52 — 정렬](52-sorting.ko.md)
- 주제: 집합/데이터
- 결과물: Python 집합으로 텍스트 파일의 고유 단어와 문장의 고유 글자 찾기

가이드 [42](42-word-count.ko.md)은 반복까지 포함해 모든 단어를 셌습니다. 집합은
고유한 값만 남기므로 `set(words)`는 다른 질문에 답합니다: 서로 *다른* 단어가
몇 개인가?

## 단계

1. 프로그램 옆에 작은 텍스트 파일 `story.txt`를 만드세요:

   ```nme
   the quick brown fox jumps over the lazy dog
   the fox and the dog are friends
   quick and lazy are fun words to say
   ```

2. 읽고 나누세요([42](42-word-count.ko.md) 참고). `set(words)`가 반복을
   제거하므로 `len(set(words))`가 서로 다른 단어 수를 셉니다:

   ```nme
   use file latest
   text = file_read("story.txt")
   words = text.split()
   show f"total words: {len(words)}"
   show f"different words: {len(set(words))}"
   ```

   이 이야기는 단어 24개지만 서로 다른 것은 15개입니다. 집합은 순서가 없고
   중복이 없으므로 같은 단어가 두 번 들어갈 수 없습니다.

3. 집합은 위치가 없어 `my_set[0]`은 실패합니다. `sorted(...)`(가이드
   [52](52-sorting.ko.md))는 집합을 순서 있는 목록으로 되돌려 주고, `in`은
   포함 여부를 확인합니다:

   ```nme
   unique_words = set(["the", "dog", "the", "cat"])
   for word in sorted(unique_words):
       show "  " + word
   show f"dog present? {'dog' in unique_words}"
   ```

   `cat`, `dog`, `the`가 알파벳 순으로 각각 한 번씩 출력된 뒤 `True`가
   출력됩니다.

4. 문자열도 글자 집합이 됩니다. 공백도 멤버로 셉니다:

   ```nme
   letters = set("hello world")
   show sorted(letters)
   ```

   `[' ', 'd', 'e', 'h', 'l', 'o', 'r', 'w']` — 여덟 멤버가 출력됩니다.

5. 전체 프로그램은 이야기를 읽고 두 집합을 만든 뒤 보고합니다. `sets.nme`로
   저장하세요:

   ```nme
   # sets.nme — 집합은 고유한 값만 남깁니다.
   # 실행: nme 실행 sets
   # 같은 폴더에 story.txt가 있어야 합니다.

   use file latest

   text = file_read("story.txt")
   words = text.split()

   말해 f"total words in story.txt: {len(words)}"

   unique_words = set(words)
   말해 f"different words: {len(unique_words)}"
   말해 ""

   말해 "the different words, sorted:"
   말해 sorted(unique_words)

   말해 f"is 'fox' in the story? {'fox' in unique_words}"
   말해 f"is 'zebra' in the story? {'zebra' in unique_words}"
   말해 ""

   sentence = "hello world"
   letters = set(sentence)
   말해 f"sentence: {sentence}"
   말해 f"members inside set(sentence): {len(letters)}"
   말해 sorted(letters)

   말해 ""
   말해 "list vs set:"
   말해 f"  list length (with repeats): {len(words)}"
   말해 f"  set length (no repeats):   {len(unique_words)}"
   ```

   `story.txt`가 있는 폴더에서 `nme 실행 sets`를 실행하세요:

   ```text
   total words in story.txt: 24
   different words: 15

   the different words, sorted:
   ['and', 'are', 'brown', 'dog', 'fox', 'friends', 'fun', 'jumps', 'lazy', 'over', 'quick', 'say', 'the', 'to', 'words']
   is 'fox' in the story? True
   is 'zebra' in the story? False

   sentence: hello world
   members inside set(sentence): 8
   [' ', 'd', 'e', 'h', 'l', 'o', 'r', 'w']

   list vs set:
     list length (with repeats): 24
     set length (no repeats):   15
   ```

   목록은 모든 단어를, 집합은 각 단어를 한 번씩 담습니다. `in`으로 포함
   여부를 확인합니다.

## 직접 해보기

`story.txt`의 단어들로만 이루어진 `unique_letters` 집합을 만들고, 글자 수가
몇 개인지 보고하세요. 문장 부호를 제거하고 싶으면 `text.replace(",", "")`
를 먼저 적용하세요.

## 배운 것

- `set(list)`는 반복을 제거해 고유한 값만 남깁니다.
- 집합에는 순서가 없고 `set[0]` 같은 위치 접근이 없습니다.
- `sorted(set)`는 순서 있는 목록으로, `x in set`은 포함 여부를 확인합니다.
- 문자열도 글자 집합이 되며 공백도 멤버입니다.
