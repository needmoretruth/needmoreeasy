# 16 — Name list: read lines from a file

English | [한국어](16-name-list.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [13 — Files](13-files.md), [14 — JSON](14-json.md)
- Topic: files and lists
- Result: a program that reads a list of names from a file and picks from it

A file can be a list. When each name sits on its own line, one Python method,
`splitlines()`, turns the whole file into a list of names.

## Steps

1. Create `names.txt` with one name per line, next to your program:

   ```nme
   Mina
   Sana
   준호
   Yuna
   ```

2. Read the whole file and split it into lines:

   ```nme
   use file latest
   names = file_read("names.txt").splitlines()
   show names
   ```

   Run `nme r names`. The console shows `['Mina', 'Sana', '준호', 'Yuna']`.
   `file_read` returns the text and `.splitlines()` cuts it at every line
   break. This line is ordinary Python, so it stays exactly as written.

3. Loop over the list with a `for` block. Sentence NME works inside it:

   ```nme
   use file latest
   names = file_read("names.txt").splitlines()
   for name in names:
       show Hello name
   ```

   This prints one `Hello` per name.

4. Pick a random name with `random_pick`. `use random latest` loads the
   picker; `3 times:` repeats the pick:

   ```nme
   use file latest
   use random latest
   names = file_read("names.txt").splitlines()
   3 times:
       show random_pick(names)
   ```

5. Korean reads with `파일읽기(...).splitlines()` and picks with `랜덤선택`:

   ```nme
   파일 사용 최신
   랜덤 사용 최신
   이름들 = 파일읽기("names.txt").splitlines()
   3번:
       이름 = 랜덤선택(이름들)
       안녕하세요 이름! 말해줘
   ```

## Try it yourself

Add two names to `names.txt`, rerun, and watch the list and picks grow.

## What you learned

- `file_read(path).splitlines()` / `파일읽기(경로).splitlines()` reads every
  line of a file into a list.
- A `for name in names:` block visits each entry; sentence NME works inside
  it.
- `random_pick(names)` / `랜덤선택(이름들)` chooses one entry at random.
- `3 times:` / `3번:` repeats the pick so a game can ask again.
