# 43 — Text stats: letters, words, and the longest one

English | [한국어](43-text-stats.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [37 — Files](37-files.md), [40 — CSV](40-csv.md)
- Topic: lists and text
- Result: reading a saved file back and reporting its letters, its words, and its longest word

What comes out of a file is one piece of text. There are usually three things
you want from it — **how long it is**, **how many words**, and **which word is
the longest**.

> A program that writes files does not run on the site.

## Steps

1. **Save it and read it back**, exactly as in [guide 37](37-files.md):

   ```nme
   set text to the weather is very good today
   write text to "memo.txt"
   read "memo.txt" into memo
   show memo
   ```

2. **Letters and words are two sentences:**

   ```nme
   set memo to the weather is very good today
   show the length of memo
   set words to memo split by space
   show how many words
   ```

   `30` and `6`.

3. **The longest word is found by comparing one at a time.** Keep the best so
   far in a name and replace it when something longer turns up:

   ```nme
   set words to list of the, weather, is, good
   set longest to the first of words
   set best to the length of longest
   for each word in words
       set size to the length of word
       if size is greater than best
           set longest to word
           set best to size
       end
   end
   show longest
   ```

   You get `weather`. **The lengths go into names first** — a reading cannot
   be written straight into the right-hand side of a comparison.

4. The whole thing:

   ```nme
   set text to the weather is very good today
   write text to "memo.txt"
   read "memo.txt" into memo
   show the length of memo
   set words to memo split by space
   show how many words
   set longest to the first of words
   set best to the length of longest
   for each word in words
       set size to the length of word
       if size is greater than best
           set longest to word
           set best to size
       end
   end
   show longest
   show best
   ```

## Try it yourself

Find the **shortest** word too — change `is greater than` to `is less than`.
Then count the lines: `set lines to memo split by line`, then
`show how many lines`.

## What you learned

- What comes out of a file is one piece of text; splitting it makes a list.
- `the length of` is letters; `how many` on the split list is words.
- The longest is found by keeping the best so far and comparing one at a time.
- A reading goes into a name before it can be compared against.
