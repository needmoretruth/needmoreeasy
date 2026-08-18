# 53 — Sets: unique values

English | [한국어](53-sets.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [36 — Word count](36-word-count.md), [39 — Sorting](39-sorting.md)
- Topic: sets & data
- Result: using a Python set to find unique words in a text file, then unique letters in a sentence

Guide [36](36-word-count.md) counted every word, repeats and all. A set keeps
only the unique values, so `set(words)` answers a different question: how many
*different* words are there?

## Steps

1. Create a small text file, `story.txt`, next to your program:

   ```nme
   the quick brown fox jumps over the lazy dog
   the fox and the dog are friends
   quick and lazy are fun words to say
   ```

2. Read it and split it (guide [36](36-word-count.md)); `set(words)` drops the
   repeats, so `len(set(words))` counts the different words:

   ```nme
   use file latest
   text = file_read("story.txt")
   words = text.split()
   show f"total words: {len(words)}"
   show f"different words: {len(set(words))}"
   ```

   The story has 24 words but only 15 different ones. A set has no order and
   no repeats, so the same word can never sit inside twice.

3. Sets have no positions, so `my_set[0]` fails. `sorted(...)` (guide
   [39](39-sorting.md)) converts one back to an ordered list, and `in` checks
   membership:

   ```nme
   unique_words = set(["the", "dog", "the", "cat"])
   for word in sorted(unique_words):
       show "  " + word
   show f"dog present? {'dog' in unique_words}"
   ```

   It prints `cat`, `dog`, `the` — alphabetical, each once — then `True`.

4. Strings become sets of characters too, and the space counts as a member:

   ```nme
   letters = set("hello world")
   show sorted(letters)
   ```

   It prints `[' ', 'd', 'e', 'h', 'l', 'o', 'r', 'w']` — eight members.

5. The full program reads the story, builds both sets, and reports them. Save
   `sets.nme`:

   ```nme
   # sets.nme — sets keep only unique values.
   # Run: nme r sets
   # The file story.txt must exist in the same folder.

   use file latest

   text = file_read("story.txt")
   words = text.split()

   show f"total words in story.txt: {len(words)}"

   unique_words = set(words)
   show f"different words: {len(unique_words)}"
   show ""

   show "the different words, sorted:"
   show sorted(unique_words)

   show f"is 'fox' in the story? {'fox' in unique_words}"
   show f"is 'zebra' in the story? {'zebra' in unique_words}"
   show ""

   sentence = "hello world"
   letters = set(sentence)
   show f"sentence: {sentence}"
   show f"members inside set(sentence): {len(letters)}"
   show sorted(letters)

   show ""
   show "list vs set:"
   show f"  list length (with repeats): {len(words)}"
   show f"  set length (no repeats):   {len(unique_words)}"
   ```

   Run `nme r sets` with `story.txt` in the folder:

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

   The list holds every word; the set holds one copy of each. Membership checks
   use the same `in` a dict uses for keys.

6. Korean writes the same program with `파일 사용 최신`, `파일읽기`, and `말해`;
   the full Korean program is in the [Korean guide](53-sets.ko.md).

## Try it yourself

Add a line to `story.txt` that reuses an old word, like `the dog and the fox
jump`; the total word count grows, but the different-word count may not. Then
build a letter set from your own name with `set("your name")` and count it.

## What you learned

- `set(words)` keeps one copy of each value, with no order and no repeats.
- `len(set(...))` counts the different values, not all of them.
- `sorted(a_set)` converts the set back into an ordered list.
- `'fox' in a_set` asks whether a value is inside, like a dict's keys.
- `set("hello world")` works on a string, where the space is a member too.
