# 37 — 파일: 글 저장하고 읽기

[English](37-files.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★☆☆ (3/5)
- 선수 지식: [12 — 랜덤](12-random.ko.md), [05 — 저장](05-set.ko.md)
- 주제: 파일
- 결과물: 글을 파일에 저장하고 다시 읽는 프로그램

값은 프로그램이 끝나면 사라집니다. 파일은 그 후에도 남습니다. NME에 들어
있는 `파일` 도우미는 두 줄로 파일에 글을 쓰고 다시 읽습니다.

## 단계

1. `diary.nme` 파일을 만듭니다:

   ```nme
   파일 사용 최신
   파일쓰기("일기.txt", "오늘은 좋은 날이에요")
   말해 파일읽기("일기.txt")
   ```

   `파일 사용 최신`이 도우미를 불러옵니다. `파일쓰기`는 새 파일에 글을
   넣고, `파일읽기`는 그 글을 다시 읽습니다.

2. 실행합니다:

   ```sh
   nme 실행 diary
   ```

   콘솔에 `오늘은 좋은 날이에요`가 출력됩니다. 폴더를 보면 프로그램 옆에
   `일기.txt` 파일이 생겨 있습니다.

3. 파일은 `nme`를 실행하는 폴더에 만들어집니다. 프로그램과 파일을 한
   폴더에 두세요. `nme r`은 `nme 실행`의 짧은 명령입니다:

   ```sh
   nme 실행 diary
   ```

4. 영어는 `use file latest`로 불러옵니다:

   ```nme
   use file latest
   file_write("diary.txt", "Today is a good day")
   show file_read("diary.txt")
   ```

   `file_write`는 쓰고 `file_read`는 읽습니다. 두 언어의 이름을 한 파일에
   섞어 써도 됩니다.

## 직접 해보기

좋아하는 음식을 `food.txt`에 저장하고 다시 읽어 보세요:

```nme
파일 사용 최신
파일쓰기("food.txt", "김치찌개")
말해 파일읽기("food.txt")
```

## 배운 것

- `파일 사용 최신` / `use file latest`가 내장 파일 도우미를 불러옵니다.
- `파일쓰기(경로, 내용)` / `file_write(path, text)`가 파일에 글을
  저장합니다.
- `파일읽기(경로)` / `file_read(path)`가 그 글을 다시 읽습니다.
- 파일은 `nme`를 실행하는 폴더, 프로그램 옆에 만들어집니다.
