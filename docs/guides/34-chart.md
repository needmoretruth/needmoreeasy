# 34 — Chart: seeing numbers as bars

English | [한국어](34-chart.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [21 — Progress](21-progress.md), [20 — ASCII art](20-ascii-art.md)
- Topic: screen and time
- Result: drawing how much was read each day as bars, so the days compare at a glance

A column of numbers takes a while to read. The same numbers as bar lengths are
understood immediately. It is the bar from [guide 21](21-progress.md) again,
except each one is drawn to **its own number** rather than to the step count.

## Steps

1. **Keep two lists side by side.** One holds the names, the other the
   numbers, and the same places belong together:

   ```nme
   set days to list of Monday, Tuesday, Wednesday
   set pages to list of 3, 7, 5
   show the first of days
   show the first of pages
   ```

   `Monday` and `3` — three pages read on Monday.

2. **Walk one list and take the same place out of the other.** `with place`
   says where you are, and that place finds the partner:

   ```nme
   set days to list of Monday, Tuesday, Wednesday
   set pages to list of 3, 7, 5
   for each day in days with place
       set read to item place of pages
       show day
       show read
   end
   ```

3. **That number of blocks is the bar:**

   ```nme
   set block to *
   set read to 7
   set bar to block repeated read times
   show bar
   ```

   Seven `*`.

4. The whole thing, ending with the busiest day's number:

   ```nme
   set days to list of Monday, Tuesday, Wednesday
   set pages to list of 3, 7, 5
   set block to *
   for each day in days with place
       set read to item place of pages
       set bar to block repeated read times
       show day
       show bar
   end
   show the biggest of pages
   ```

   You can see Tuesday is the longest without reading a single number.

## Try it yourself

Add another day and another number — **to both lists**, or the places stop
lining up and the program stops at the place with no partner. Keeping a name
and its number together in one place is what [guide 41](41-address-book.md)
is for.

## What you learned

- Two lists side by side pair a name with a number.
- `with place` says where you are; `item <place> of <list>` takes the partner out.
- `<block> repeated <n> times` turns that number into one bar.
- Seen as lengths, the biggest is obvious without reading any number.
