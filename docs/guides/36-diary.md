# 36 — Diary: writing it down with today's date

English | [한국어](36-diary.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [23 — High score](23-high-score.md), [12 — Random](12-random.md)
- Topic: files
- Result: saving a one-line diary entry stamped with today's date and weekday, then reading it back

A diary needs **when it was written**. Typing the date by hand goes wrong and
gets forgotten. The computer already knows it, so ask it.

> A program that writes files does not run on the site.

## Steps

1. **Open the date tools.** One line brings the words that ask about dates:

   ```nme
   use date latest
   say today()
   say weekday()
   say year()
   ```

   Something like `2026-08-19`, `Wednesday`, `2026`. **The clock is UTC** —
   with a different clock per country the same program would answer differently
   for different people.

2. **Put the date in a name** so it can be used more than once:

   ```nme
   use date latest
   set stamp to today()
   set day to weekday()
   show stamp
   show day
   ```

3. **Put the date, the weekday and the note into one line.** It is the shape
   from [guide 40](40-csv.md):

   ```nme
   set stamp to "2026-08-19"
   set day to Wednesday
   set note to it rained a little
   set lines to an empty list
   append stamp to lines
   append day to lines
   append note to lines
   set row to lines joined by comma
   show row
   ```

   `2026-08-19, Wednesday, it rained a little`. **A date typed by hand needs
   quotes** — `2026-08-19` on its own reads as a sum with subtraction in it.
   What `today()` hands you needs none.

4. **Save it, read it back, take the fields out:**

   ```nme
   set row to "2026-08-19, Wednesday, it rained a little"
   write row to "diary.txt"
   read "diary.txt" into memo
   set fields to memo split by comma
   show the first of fields
   show the last of fields
   ```

5. The whole thing:

   ```nme
   use date latest
   set stamp to today()
   set day to weekday()
   set note to it rained a little
   set lines to an empty list
   append stamp to lines
   append day to lines
   append note to lines
   set row to lines joined by comma
   write row to "diary.txt"
   read "diary.txt" into memo
   show memo
   ```

## Try it yourself

Add `days_after(7)` to write down what the date will be a week from now;
`days_after(-1)` is yesterday. Then `ask` for the note and it becomes a real
diary — except that it **overwrites**. Keeping entries means reading what is
there, adding to a list, and writing the whole thing back.

## What you learned

- One line of `use date latest` brings `today()`, `weekday()`, `year()` and `days_after(n)`.
- Put a date in a name and use it as often as you like.
- Date, weekday and note joined by commas make a one-line record.
- The clock is UTC, so the program answers the same for everyone.
