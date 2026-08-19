# 56 — Bubble sort — your first algorithm

English | [한국어](56-bubble.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [52 — Sorting](52-sorting.md), [55 — Grid](55-grid.md)
- Topic: lists and text
- Result: implementing bubble sort by hand with nested loops and a swap, then comparing the result with Python's built-in sort

Guide [52](52-sorting.md) let Python sort for you. Here you build the sort
yourself. Bubble sort is an algorithm — a fixed recipe of steps that always
ends with the list in order. It compares neighbor pairs, swaps them when they
are out of order, and repeats until a pass makes no swap.

## Steps

1. Compare the first two numbers. If the left one is larger, swap them. A
   swap needs a third variable: `temp` holds the first value while
   `numbers[0]` takes the second value's place:

   ```nme
   numbers = [5, 2, 9, 1, 7, 3]
   if numbers[0] > numbers[1]:
       temp = numbers[0]
       numbers[0] = numbers[1]
       numbers[1] = temp
   show numbers
   ```

   It prints `[2, 5, 9, 1, 7, 3]` — the `5` and `2` changed places.

2. One loop can walk the whole list, comparing each neighbor pair and
   swapping when needed. This is one pass, and it moves the largest value to
   the end:

   ```nme
   numbers = [5, 2, 9, 1, 7, 3]
   n = len(numbers)
   for j in range(0, n - 1):
       if numbers[j] > numbers[j + 1]:
           temp = numbers[j]
           numbers[j] = numbers[j + 1]
           numbers[j + 1] = temp
   show numbers
   ```

   It prints `[2, 5, 1, 7, 3, 9]`. The `9` bubbled to the end — that is where
   the name comes from.

3. One pass is not enough: the `2` and `1` are still out of order. An outer
   loop repeats the pass. After each pass the largest remaining value is
   already in place, so the inner loop stops one cell earlier — `n - i - 1`:

   ```nme
   numbers = [5, 2, 9, 1, 7, 3]
   n = len(numbers)
   for i in range(n):
       for j in range(0, n - i - 1):
           if numbers[j] > numbers[j + 1]:
               temp = numbers[j]
               numbers[j] = numbers[j + 1]
               numbers[j + 1] = temp
   show numbers
   ```

   It prints `[1, 2, 3, 5, 7, 9]` — sorted.

4. Sorting a list that is already sorted should stop fast. A flag records
   whether a pass made any swap; if it made none, the list is done and the
   loop can `break`:

   ```nme
   numbers = [5, 2, 9, 1, 7, 3]
   n = len(numbers)
   for i in range(n):
       swapped = False
       for j in range(0, n - 1):
           if numbers[j] > numbers[j + 1]:
               temp = numbers[j]
               numbers[j] = numbers[j + 1]
               numbers[j + 1] = temp
               swapped = True
       if not swapped:
           break
   show numbers
   ```

   It prints `[1, 2, 3, 5, 7, 9]`. The flag is the early-exit trick.

5. Now the whole thing in one program. It sorts the list by hand, prints
   every pass, and checks the result against Python's `sorted()`. Save
   `bubble.nme`:

   ```nme
   # bubble.nme — bubble sort by hand, compared with the built-in sort.
   # Run: nme r bubble
   #
   # Each pass compares neighbor pairs and swaps them when they are
   # out of order, so the largest remaining value bubbles to the end.
   # The flag exits early when a pass makes no swaps.

   numbers = [5, 2, 9, 1, 7, 3]
   built_in = sorted(numbers)

   show f"Before: {numbers}"
   show ""

   n = len(numbers)
   comparisons = 0
   swaps = 0

   for i in range(n):
       swapped = False
       for j in range(0, n - i - 1):
           comparisons = comparisons + 1
           if numbers[j] > numbers[j + 1]:
               temp = numbers[j]
               numbers[j] = numbers[j + 1]
               numbers[j + 1] = temp
               swapped = True
               swaps = swaps + 1
       show f"Pass {i + 1}: {numbers}"
       if not swapped:
           show "  no swaps — already sorted, stopping early"
           break

   show ""
   show f"After my loop:  {numbers}"
   show f"After sorted(): {built_in}"

   if numbers == built_in:
       show "Both orders agree."
   show f"{comparisons} comparisons and {swaps} swaps for {n} values"
   ```

6. Run it:

   ```sh
   nme r bubble
   ```

   ```text
   Before: [5, 2, 9, 1, 7, 3]

   Pass 1: [2, 5, 1, 7, 3, 9]
   Pass 2: [2, 1, 5, 3, 7, 9]
   Pass 3: [1, 2, 3, 5, 7, 9]
   Pass 4: [1, 2, 3, 5, 7, 9]
     no swaps — already sorted, stopping early

   After my loop:  [1, 2, 3, 5, 7, 9]
   After sorted(): [1, 2, 3, 5, 7, 9]
   Both orders agree.
   14 comparisons and 8 swaps for 6 values
   ```

   Pass 4 makes no swap, so the flag stops the sort early — the loop never
   ran a fifth or sixth pass. `sorted()` is Python's built-in sort; matching
   its order is a good sanity check for a hand-written algorithm.

7. Korean writes the same steps with `말해`, Korean variable names, and a
   Korean list. The full Korean program is in the [Korean
   guide](56-bubble.ko.md).

## Try it yourself

Change the list to `[9, 8, 7, 6, 5]` — reversed order — and rerun. Almost
every pass swaps, so the flag never stops the sort early and all five passes
run. Then try `[1, 2, 3, 4, 5]`, already sorted: the first pass swaps
nothing, and the program stops after a single pass.

## What you learned

- Bubble sort compares neighbor pairs and swaps them when they are out of order.
- A swap needs a temporary variable so no value is lost.
- An outer loop repeats the pass; each pass needs one fewer comparison (`n - i - 1`).
- A `swapped` flag lets the loop stop early once the list is sorted.
- Checking your result against `sorted()` verifies the algorithm.
