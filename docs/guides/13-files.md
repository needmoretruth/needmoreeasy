# 13 — Files: save and read text

English | [한국어](13-files.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★☆ (4/5)
- 선수 지식 (Prerequisites): [10 — Random](10-random.md), [03 — Set](03-set.md)
- 주제 (Topic): 파일 / files
- 결과물 (Result): 글을 파일에 저장하고 다시 읽는 프로그램 / a program that saves text to a file and reads it back

A value lives until the program ends. A file lives on after it. NME's bundled
`file` helper writes text to a file and reads it back with two easy lines.

## Steps

1. Create `diary.nme`:

   ```text
   use file latest
   file_write("diary.txt", "Today is a good day")
   show file_read("diary.txt")
   ```

   `use file latest` loads the helper. `file_write` puts the text into a new
   file, and `file_read` reads it back.

2. Run it:

   ```sh
   nme run diary
   ```

   The console prints `Today is a good day`. Look at the folder: a `diary.txt`
   file now sits next to your program.

3. Files are written in the folder you run `nme` in, so keep the program and
   its files together. `nme r` is the shortcut for `nme run`:

   ```sh
   nme r diary
   ```

4. Korean loads the helper with `파일 사용 최신`:

   ```text
   파일 사용 최신
   파일쓰기("일기.txt", "오늘은 좋은 날이에요")
   말해 파일읽기("일기.txt")
   ```

   `파일쓰기` writes and `파일읽기` reads, exactly like the English names.
   Both vocabularies work in one file.

## Try it yourself

Write your favorite food to `food.txt`, then read it back:

```text
use file latest
file_write("food.txt", "Kimchi stew")
show file_read("food.txt")
```

## What you learned

- `use file latest` / `파일 사용 최신` loads the bundled file helper.
- `file_write(path, text)` / `파일쓰기(경로, 내용)` saves text to a file.
- `file_read(path)` / `파일읽기(경로)` reads it back.
- The file appears in the folder you run `nme` in, next to the program.
