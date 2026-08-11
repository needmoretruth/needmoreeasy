# 36 — Word count: how often each word appears

English | [한국어](36-word-count.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [16 — Name list](16-name-list.md), [13 — Files](13-files.md)
- 주제 (Topic): 데이터 / data
- 결과물 (Result): 텍스트 파일을 읽어 단어가 각각 몇 번 나오는지 세는 프로그램 / reading a text file and counting how often each word appears, using a dict and collections.Counter

Counting words is the first step of many text programs. Read a file, split it
into words, and count: a plain dict does it by hand, and `collections.Counter`
does the same job in one call — including the five most common words.

## Steps

1. Make a small text file, `story.txt`, next to your program:

   ```text
   the cat sat on the mat
   the dog ran past the cat
   a bird sang on the roof
   ```

2. Read the whole file and split it into words. `file_read` returns the text
   (guide [13](13-files.md)) and `.split()` cuts it at every space:

   ```text
   use file latest
   text = file_read("story.txt")
   words = text.split()
   show "words: " + str(len(words))
   ```

   The file has 18 words.

3. A dict counts by hand. An empty dict `{}` plus one `for` loop: add 1 when
   the word is already in it, start at 1 otherwise. `tally["the"]` reads the
   count back:

   ```text
   use file latest
   text = file_read("story.txt")
   words = text.split()
   tally = {}
   for word in words:
       if word in tally:
           tally[word] = tally[word] + 1
       else:
           tally[word] = 1
   show "the appears " + str(tally["the"]) + " times"
   ```

   `tally` maps each word to its count. Guide [16](16-name-list.md) used a
   list; a dict is a list with named slots instead.

4. `collections.Counter` does the same counting in one line and adds
   `most_common(5)` for the five most frequent words. Import it like the
   `date` import in guide [24](24-python-packages.md):

   ```text
   use file latest
   from collections import Counter
   text = file_read("story.txt")
   words = text.split()
   counts = Counter(words)
   show counts.most_common(5)
   ```

   Each entry is a `word, times` pair — `the` appears 5 times, `cat` and `on`
   twice each, and every other word once.

5. The full program reads the file, counts both ways, and prints a small
   report. Save it as `word-count.nme`:

   ```text
   # Count how often each word appears in a text file.
   # Run: nme r word-count

   use file latest
   from collections import Counter

   text = file_read("story.txt")
   words = text.split()

   show "total words: " + str(len(words))

   tally = {}
   for word in words:
       if word in tally:
           tally[word] = tally[word] + 1
       else:
           tally[word] = 1

   show "different words: " + str(len(tally))
   show "the word 'the': " + str(tally.get("the", 0))

   counts = Counter(words)
   show "most common five:"
   for word, times in counts.most_common(5):
       show f"{word}: {times}"
   ```

   `tally.get("the", 0)` reads the count too, but returns 0 instead of failing
   when the word is missing.

6. Run it with `story.txt` in the folder:

   ```sh
   nme r word-count
   ```

   ```text
   total words: 18
   different words: 12
   the word 'the': 5
   most common five:
   the: 5
   cat: 2
   on: 2
   sat: 1
   mat: 1
   ```

   The two `for` loops count the same way: the dict shows the idea, and
   `Counter` turns it into one call.

## Try it yourself

Make the count case-insensitive so `The` and `the` are one word: change
`words = text.split()` in the full program to `words = text.lower().split()`.
Then `the` appears 5 times instead of two separate spellings.

## What you learned

- `file_read(path)` returns the whole text file as one string.
- `text.split()` cuts the string into a list of words.
- A dict counts each word: `if word in tally` adds 1, otherwise starts at 1.
- `Counter(words).most_common(5)` counts all words and returns the top five.
