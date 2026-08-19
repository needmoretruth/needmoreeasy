# 52 — Sorting — putting a list in order

English | [한국어](52-sorting.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [38 — Name list](38-name-list.md)
- Topic: lists and text
- Result: standing a list up in both directions, smallest first and biggest first

A list stays in the order you wrote it. That is rarely the order you want to
show — scores read best from the top, names read best in alphabetical order.
There are only two sentences for putting a list in order.

## Steps

1. **Make a list and look at it.** `joined by comma` shows a list on one line:

   ```nme
   set scores to list of 5, 3, 9, 1
   show scores joined by comma
   ```

   `5, 3, 9, 1` — exactly as written.

2. **`sort` stands it up smallest first:**

   ```nme
   set scores to list of 5, 3, 9, 1
   sort scores
   show scores joined by comma
   ```

   You get `1, 3, 5, 9`. **The list itself changes.** From here on `scores`
   stays in that order.

3. **Once it is standing, the two ends are your answer.** Smallest first means
   the first item is the smallest and the last is the biggest:

   ```nme
   set scores to list of 5, 3, 9, 1
   sort scores
   show the first of scores
   show the last of scores
   ```

   `1` and `9`.

4. **`reverse` turns it around.** Reversing a sorted list gives biggest first:

   ```nme
   set scores to list of 5, 3, 9, 1
   sort scores
   reverse scores
   show scores joined by comma
   ```

   `9, 5, 3, 1`. That is the order a score table is written in.

5. **Words stand up too.** Numbers go smallest first, words go alphabetically:

   ```nme
   set names to list of Zoe, Mina, Ada
   sort names
   show names joined by comma
   ```

   You get `Ada, Mina, Zoe`.

6. The whole thing:

   ```nme
   set scores to list of 5, 3, 9, 1
   show as written
   show scores joined by comma
   sort scores
   show smallest first
   show scores joined by comma
   show the first of scores
   show the last of scores
   reverse scores
   show biggest first
   show scores joined by comma
   ```

## Try it yourself

Put `show the biggest of scores` at the very top, before any sorting. It finds
the biggest without standing the list up at all. Sorting is what you do when
you want to see **all of it** in order.

## What you learned

- `sort <list>` stands a list up smallest first, or alphabetically for words.
- `reverse <list>` turns the current order around. After sorting, that is biggest first.
- Both **change the list itself.** The original order is gone.
- After sorting, `the first` is the smallest and `the last` is the biggest.
