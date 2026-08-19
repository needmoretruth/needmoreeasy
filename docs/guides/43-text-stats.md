# 43 — Text stats: letters and words

English | [한국어](43-text-stats.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [42 — Word count](42-word-count.md), [38 — Name list](38-name-list.md)
- Topic: lists and text
- Result: reading a text file and reporting character count, word count, longest word, and most common word (with collections.Counter)

A story is a string first, then a list of words. This guide reads a text file, counts its characters and words, and lets Python find the longest and most common ones.

## Steps

1. Create `story.txt` next to your program and read it back:

   ```nme
   the sun rose over the quiet town
   the town woke and the birds sang
   the children ran to the playground
   ```

   ```nme
   use file latest
   text = file_read("story.txt")
   words = text.split()
   show f"characters: {len(text)}"
   show f"words: {len(words)}"
   ```

   It prints `characters: 101` and `words: 20`. `file_read` (guide
   [37](37-files.md)) returns the text; `.split()` cuts the word list.

2. Longest and most common word. `max(words, key=len)` compares by length and
   `collections.Counter` (guide [64](64-python-packages.md)) counts every word, with `most_common(1)` for the top pair:

   ```nme
   use file latest
   from collections import Counter
   text = file_read("story.txt")
   words = text.split()
   longest = max(words, key=len)
   counts = Counter(words)
   show f"longest word: {longest}"
   show counts.most_common(1)
   ```

   It prints `longest word: playground` and `[('the', 6)]`.

3. Now the whole report in one file. Save `text-stats.nme` and run it:

   ```nme
   # text-stats.nme — letters and words in a text file.
   # Run: nme r text-stats
   # The file story.txt must exist in the same folder.

   use file latest
   from collections import Counter

   # Read the whole file as one string.
   text = file_read("story.txt")
   show f"characters: {len(text)}"

   # Split into a list of words.
   words = text.split()
   show f"words: {len(words)}"

   # Longest word, compared by length.
   longest = max(words, key=len)
   show f"longest word: {longest} ({len(longest)} letters)"

   # Most common word and its count.
   counts = Counter(words)
   top_word, top_times = counts.most_common(1)[0]
   show f"most common word: {top_word} ({top_times} times)"

   # Average word length, written out long-hand.
   total = 0
   for word in words:
       total = total + len(word)
   show f"average word length: {total / len(words)}"
   ```

   `most_common(1)[0]` pulls the only pair out of the one-element list; the
   last loop adds every word's length.

   ```sh
   nme r text-stats
   ```
   ```text
   characters: 101
   words: 20
   longest word: playground (10 letters)
   most common word: the (6 times)
   average word length: 4.05
   ```

   One file, four questions answered. The Korean guide uses `파일읽기` and `보여줘` with the same Python calls — full pair in [43-text-stats.ko.md](43-text-stats.ko.md).

## Try it yourself

Add a `shortest` line with `min(words, key=len)`, then count the words longer than four letters with one `for` loop.

## What you learned

- `file_read(path)` returns the whole text file as one string.
- `len(text)` counts characters; `text.split()` makes a word list.
- `max(words, key=len)` finds the longest word by comparing lengths.
- `Counter(words).most_common(1)` returns the most common word and its count.
