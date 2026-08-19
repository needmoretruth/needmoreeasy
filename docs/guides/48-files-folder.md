# 48 — Files: processing many files

English | [한국어](48-files-folder.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [37 — Files](37-files.md), [42 — Word count](42-word-count.md)
- Topic: files
- Result: listing the files in a folder with os.listdir, reading each with file_read, and reporting total words and letters across all of them

Guide [37](37-files.md) read one file you named in the code. Real folders hold
many files, and you usually do not know their names ahead of time.
`os.listdir(".")` lists every name in a folder, and `file_read` from guide
[37](37-files.md) reads whichever one you point at. Together they turn a whole
folder into a list you can loop over.

## Steps

1. Create two small text files next to your program. `one.txt`:

   ```nme
   the cat sat on the mat
   the dog ran past the cat
   ```

   And `two.txt`:

   ```nme
   a bird sang on the roof
   ```

2. `import os` loads Python's folder tools, and `os.listdir(".")` returns a
   list of every name in the current folder — `.` means "this folder". Count
   the list with `len`:

   ```nme
   import os
   names = os.listdir(".")
   show f"{len(names)} names in this folder"
   ```

   With the program and the two text files, that prints
   `3 names in this folder`. The list contains every name, including
   `files-folder.nme` — not just the text files.

3. `.endswith(".txt")` keeps only the text files. A `for` loop reads each
   kept name with `file_read` and splits it into words, exactly as guide
   [42](42-word-count.md) did for one file:

   ```nme
   import os
   use file latest
   for name in os.listdir("."):
       if name.endswith(".txt"):
           words = file_read(name).split()
           show f"{name}: {len(words)} words"
   ```

   ```text
   two.txt: 6 words
   one.txt: 12 words
   ```

4. Letters count the same way per file: remove the spaces and the line breaks,
   then take `len`:

   ```nme
   use file latest
   text = file_read("one.txt")
   letters = len(text.replace(" ", "").replace("\n", ""))
   show letters
   ```

   `one.txt` has 36 letters.

5. The full program collects the `.txt` names first, reports each file, and
   then the totals and the biggest file across all of them. Save
   `files-folder.nme`:

   ```nme
   # files-folder.nme — words and letters across every .txt file.
   # Run: nme r files-folder
   # Create one.txt and two.txt in the same folder first.

   import os
   use file latest

   files = os.listdir(".")
   txt_files = []
   for name in files:
       if name.endswith(".txt"):
           txt_files.append(name)

   show f"{len(txt_files)} txt files found:"

   total_words = 0
   total_letters = 0
   biggest = ""
   biggest_words = 0
   for name in txt_files:
       text = file_read(name)
       words = text.split()
       letters = len(text.replace(" ", "").replace("\n", ""))
       total_words = total_words + len(words)
       total_letters = total_letters + letters
       if len(words) > biggest_words:
           biggest_words = len(words)
           biggest = name
       show f"  {name}: {len(words)} words, {letters} letters"

   show "all files:"
   show f"  words: {total_words}"
   show f"  letters: {total_letters}"
   show f"  biggest: {biggest} with {biggest_words} words"
   ```

   The `if len(words) > biggest_words:` check remembers the file with the most
   words, the same "best so far" idea as the high score in guide
   [23](23-high-score.md).

6. Run it with the two text files in the folder:

   ```sh
   nme r files-folder
   ```

   ```text
   2 txt files found:
     two.txt: 6 words, 18 letters
     one.txt: 12 words, 36 letters
   all files:
     words: 18
     letters: 54
     biggest: one.txt with 12 words
   ```

   The program never names a file in advance: `os.listdir` found them, the
   `if` kept the `.txt` ones, and the totals add the per-file counts.

7. Korean writes the same steps with `import os`, `파일 사용 최신`, and
   `파일읽기`; the full Korean program is in the
   [Korean guide](48-files-folder.ko.md).

## Try it yourself

Add a third file `three.txt` and rerun — it appears in the report with no
code change. Or change the filter to `if name.endswith(".csv"):` and count
lines instead of words: `text.splitlines()`.

## What you learned

- `os.listdir(".")` returns every name in the current folder as a list.
- `.endswith(".txt")` filters the list to the files you want.
- `file_read(name)` reads the file named by any string, not just a literal.
- A `for` loop over a folder turns one-file handling into many-file handling.
