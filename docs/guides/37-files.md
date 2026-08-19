# 37 — Files: saving and reading again

English | [한국어](37-files.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [23 — High score](23-high-score.md)
- Topic: files
- Result: a program that writes text to a file and reads it back

Everything so far disappeared when the program ended. A file stays. Part 3
begins here.

> **From this guide on, the programs do not run on the site.** There is nowhere
> inside a browser to keep a file, so a program that writes one fails on
> needmoreeasy.com. Try these on your own computer, after
> [install](../install.md).

## Steps

1. **Write it.** Name the file and the text to put in it:

   ```nme
   write "today was a good day" to "diary.txt"
   ```

   `diary.txt` appears in the folder you ran the program from.

2. **Read it back** into a name, then show that name:

   ```nme
   read "diary.txt" into memo
   show memo
   ```

3. Put them together and it is one program:

   ```nme
   write "today was a good day" to "diary.txt"
   read "diary.txt" into memo
   show memo
   ```

   Korean is `"일기.txt" 파일에 "…"를 저장해` and `메모에 "일기.txt" 읽어서`.

4. **This is the one place quotes appear.** Sentence syntax has no quotes
   anywhere else. A file name is the exception because `diary.txt` is not a word
   of your sentence — it is **the name of something outside the program**, and
   the quotes are what say "take this exactly as it stands".

5. **What somebody types can be saved too.** Put the answer straight in:

   ```nme
   ask today What happened today?
   write today to "diary.txt"
   read "diary.txt" into memo
   show what was saved
   show memo
   ```

   Run it a second time and the first entry is **overwritten**. Adding to a file
   instead of replacing it is [44 — Log](44-log.md).

## Try it yourself

Take the game from [23 — High score](23-high-score.md) and write the best score
to a file at the end. Then stop the program, start it again, and check the file
is still there.

## What you learned

- `write … to "name.txt"` puts text in a file.
- `read "name.txt" into …` takes it back out.
- File names are wrapped in quotes — the only place sentence syntax uses them.
- The file appears in the folder you ran the program from.
- Writing the same name again replaces what was there.
- A program that uses files does not run in a browser.
