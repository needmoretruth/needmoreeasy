# 42 — Compare: two groups of numbers

English | [한국어](42-compare.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [30 — Data](30-data.md), [14 — JSON](14-json.md)
- Topic: data & comparison
- Result: loading two JSON number lists and comparing their means and maxima

Guide [30](30-data.md) summarized one list of numbers. Comparing two lists
asks the same questions twice, then decides which group is bigger.

## Steps

1. Create and load the two data files, each holding one JSON list (the shape guide [14](14-json.md) saved with `json_save`):

   ```nme
   # a.json
   [5, 2, 9, 1, 7, 3]
   # b.json
   [8, 4, 6, 10, 3]
   ```

   ```nme
   use file latest
   a = json_load("a.json")
   b = json_load("b.json")
   show f"a.json: {a}"
   show f"b.json: {b}"
   ```

   It prints `a.json: [5, 2, 9, 1, 7, 3]` and `b.json: [8, 4, 6, 10, 3]`.

2. Mean by hand, mean by library, and max. `sum(a) / len(a)` is the mean; `statistics.mean` (guide [30](30-data.md)) checks it; `max` is a builtin:

   ```nme
   from statistics import mean
   a = [5, 2, 9, 1, 7, 3]
   a_mean = sum(a) / len(a)
   show f"a mean: {a_mean}"
   show f"a mean from statistics: {mean(a)}"
   show f"a max: {max(a)}"
   ```

   a is 4.5 (27 over 6), with max 9. For b the sum is 31 over 5, so its mean
   is 6.2 and its max is 10.

3. The comparison program. Save `compare.nme` with both JSON files, and run it:

   ```nme
   # compare.nme — which group of numbers is bigger?
   # Run: nme r compare
   # The files a.json and b.json must exist in the same folder.

   use file latest
   from statistics import mean

   a = json_load("a.json")
   b = json_load("b.json")

   # Mean by hand, max by builtin.
   a_mean = sum(a) / len(a)
   b_mean = sum(b) / len(b)
   a_max = max(a)
   b_max = max(b)
   show f"a: mean {a_mean}, max {a_max}"
   show f"b: mean {b_mean}, max {b_max}"

   # statistics.mean checks the hand-written answer.
   show f"a mean from statistics: {mean(a)}"
   show f"b mean from statistics: {mean(b)}"

   # Compare the two groups.
   if a_mean > b_mean:
       show "a has the higher mean"
   else:
       show "b has the higher mean"
   if a_max > b_max:
       show "a has the higher max"
   else:
       show "b has the higher max"
   ```

   ```sh
   nme r compare
   ```
   ```text
   a: mean 4.5, max 9
   b: mean 6.2, max 10
   a mean from statistics: 4.5
   b mean from statistics: 6.2
   a has the higher mean
   b has the higher max
   ```

   `b` wins both comparisons. The `statistics` lines confirm the hand-written
   means. The Korean guide uses `json읽기` and `보여줘` — full pair in [42-compare.ko.md](42-compare.ko.md).

## Try it yourself

Change `b.json` to `[3, 1, 4, 1, 5, 9]` and rerun — the answers flip.

## What you learned

- `json_load` turns each JSON list file into a Python list.
- `sum(numbers) / len(numbers)` is the mean written by hand.
- `statistics.mean` gives the same answer in one call.
- `max(numbers)` is a builtin; `if`/`else` turns the numbers into a sentence.
