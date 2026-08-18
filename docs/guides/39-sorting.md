# 39 — Sorting — putting a list in order

English | [한국어](39-sorting.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [16 — Name list](16-name-list.md), [30 — Data](30-data.md)
- Topic: data
- Result: loading numbers from a JSON file and showing sorted ascending and descending orders with the Python sort method

A loaded list arrives in the order it was written, which is rarely the order
you want to show. Python's `sort` method puts the list in order and `sorted()`
builds a sorted copy; this guide loads numbers from JSON, sorts them both ways,
and explains the difference between the two.

## Steps

1. Create `numbers.json` with one JSON list of numbers:

   ```text
   [5, 2, 9, 1, 7, 3]
   ```

   Guide [30](30-data.md) used the same file; the numbers here are deliberately
   out of order.

2. Load the list with `json_load` from guide [14](14-json.md):

   ```text
   use file latest
   numbers = json_load("numbers.json")
   show f"Loaded {len(numbers)} numbers: {numbers}"
   ```

   Run it; it prints `Loaded 6 numbers: [5, 2, 9, 1, 7, 3]`.

3. `sorted(numbers)` returns a NEW list in ascending order and leaves the
   original untouched:

   ```text
   numbers = [5, 2, 9, 1, 7, 3]
   ascending = sorted(numbers)
   show ascending
   show numbers
   ```

   It prints `[1, 2, 3, 5, 7, 9]` and then `[5, 2, 9, 1, 7, 3]` — the original
   stays exactly as it was.

4. `numbers.sort()` changes the list ITSELF, in place. After the call the
   variable holds the sorted list and the old order is gone:

   ```text
   numbers = [5, 2, 9, 1, 7, 3]
   numbers.sort()
   show numbers
   ```

   It prints `[1, 2, 3, 5, 7, 9]`. This is the "ascending" order — the smallest
   first, the largest last.

5. Descending order flips it, the largest first. `sorted(numbers, reverse=True)`
   makes a new list without touching `numbers`:

   ```text
   numbers = [1, 2, 3, 5, 7, 9]
   descending = sorted(numbers, reverse=True)
   show descending
   ```

   It prints `[9, 7, 5, 3, 2, 1]`.

6. Now the whole report in one file. Save `sorting.nme`:

   ```text
   # Sorting: putting a saved list of numbers in order.
   # Run: nme r sorting
   # The file numbers.json must exist in the same folder.

   use file latest

   numbers = json_load("numbers.json")

   show f"Loaded {len(numbers)} numbers: {numbers}"
   show ""

   ascending = sorted(numbers)
   show "sorted(numbers) makes a NEW list:"
   show ascending
   show "The original list is unchanged:"
   show numbers
   show ""

   numbers.sort()
   show "numbers.sort() changes the list IN PLACE:"
   show numbers
   show ""

   descending = sorted(numbers, reverse=True)
   show "Descending order with reverse=True:"
   show descending

   show "Smallest: " + str(numbers[0])
   show "Largest: " + str(numbers[-1])

   show ""
   show "Sorted list, one number per line:"
   for n in numbers:
       show "  " + str(n)
   ```

7. Run it with the data file present:

   ```sh
   nme r sorting
   ```

   ```text
   Loaded 6 numbers: [5, 2, 9, 1, 7, 3]

   sorted(numbers) makes a NEW list:
   [1, 2, 3, 5, 7, 9]
   The original list is unchanged:
   [5, 2, 9, 1, 7, 3]

   numbers.sort() changes the list IN PLACE:
   [1, 2, 3, 5, 7, 9]

   Descending order with reverse=True:
   [9, 7, 5, 3, 2, 1]
   Smallest: 1
   Largest: 9

   Sorted list, one number per line:
     1
     2
     3
     5
     7
     9
   ```

   `numbers.sort()` changed the list for good, which is why the descending line
   starts from the sorted order and the one-per-line list uses it too.

8. Korean writes the same steps with `파일 사용 최신` and `json읽기`. The full
   Korean program is in the [Korean guide](39-sorting.ko.md).

## Try it yourself

Change `numbers.json` to `[100, 3, 42, 17]` and rerun `sorting.nme`; every
order updates. Then add `numbers.reverse()` after the sort and rerun — the list
now runs largest to smallest.

## What you learned

- `sorted(numbers)` returns a new ascending list and leaves the original alone.
- `numbers.sort()` reorders the list in place; the old order is lost.
- `sorted(numbers, reverse=True)` builds a descending copy.
- After sorting, `numbers[0]` is the smallest and `numbers[-1]` the largest.
- Ascending means smallest first; descending means largest first.
