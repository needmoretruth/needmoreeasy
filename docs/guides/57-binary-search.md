# 57 — Binary search — halving the guess

English | [한국어](57-binary-search.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [56 — Bubble sort](56-bubble.md), [52 — Sorting](52-sorting.md)
- Topic: algorithm
- Result: finding a number in a sorted list by halving the search range each step, showing the step count and the found index

Guide [56](56-bubble.md) sorted a list; guide [52](52-sorting.md) let Python
sort. Once a list is sorted, a smarter way to search appears. Instead of
checking values left to right, binary search reads the middle value and uses
the comparison to throw away half of the remaining range — a 1,000-item list
needs at most 10 guesses.

## Steps

1. Two pointers, `low` and `high`, fence in the range that could still hold
   the target. `mid` is the middle of that range, and `//` divides and rounds
   down:

   ```nme
   numbers = [2, 5, 8, 12, 16, 23, 38, 56, 72, 91]
   low = 0
   high = len(numbers) - 1
   mid = (low + high) // 2
   show f"low={low} high={high} mid={mid} guess={numbers[mid]}"
   ```

   It prints `low=0 high=9 mid=4 guess=16`. With 10 values the middle lands on
   index 4, the number 16.

2. Compare the guess with the target. If it is too small, everything left of
   `mid` is too small too — so `low` jumps past it. If it is too big, `high`
   jumps below it. Repeating this halves the range every loop:

   ```nme
   numbers = [2, 5, 8, 12, 16, 23, 38, 56, 72, 91]
   target = 23
   low = 0
   high = len(numbers) - 1
   steps = 0
   found = -1

   while low <= high:
       steps = steps + 1
       mid = (low + high) // 2
       show f"step {steps}: mid={mid} guess={numbers[mid]}"
       if numbers[mid] == target:
           found = mid
           break
       if numbers[mid] < target:
           low = mid + 1
       else:
           high = mid - 1

   if found != -1:
       show f"Found {target} at index {found}"
   else:
       show f"{target} is not in the list"
   ```

   `found = -1` marks "not seen yet". A miss keeps halving; a hit records the
   index and `break`s out.

3. Follow the walkthrough for target 23. First `mid=4` guesses 16, and
   `16 < 23` — so 16 and everything left of it are too small, and `low`
   becomes 5. The range shrinks from ten values to five. Next `mid=7` guesses
   56, and `56 > 23` — everything right of it is too big, and `high` becomes
   6. Two values remain. `mid=5` guesses 23 exactly: found at index 5 after
   three steps.

4. The whole program. It prints every step, the found index, the step count,
   and a left-to-right search for comparison. Save `binary.nme`:

   ```nme
   # binary.nme — binary search: halving the guess.
   # Run: nme r binary
   #
   # The list is sorted. Keep two pointers — low and high — around
   # the range that could still hold the target. Each step reads the
   # middle, compares it with the target, and drops half of the range.
   # A left-to-right search is shown at the end for comparison.

   numbers = [2, 5, 8, 12, 16, 23, 38, 56, 72, 91]
   target = 23

   low = 0
   high = len(numbers) - 1
   steps = 0
   found = -1

   while low <= high:
       steps = steps + 1
       mid = (low + high) // 2
       guess = numbers[mid]
       show f"step {steps}: low={low} high={high} mid={mid} guess={guess}"
       if guess == target:
           found = mid
           break
       if guess < target:
           low = mid + 1
       else:
           high = mid - 1

   show ""
   if found != -1:
       show f"Found {target} at index {found} in {steps} step(s)"
   else:
       show f"{target} is not in the list ({steps} steps)"
   show f"The range shrank from {len(numbers)} values to 1"

   linear = 0
   for i in range(len(numbers)):
       linear = linear + 1
       if numbers[i] == target:
           break
   show f"A left-to-right search checks {linear} value(s)"
   ```

5. Run it:

   ```sh
   nme r binary
   ```

   ```text
   step 1: low=0 high=9 mid=4 guess=16
   step 2: low=5 high=9 mid=7 guess=56
   step 3: low=5 high=6 mid=5 guess=23

   Found 23 at index 5 in 3 step(s)
   The range shrank from 10 values to 1
   A left-to-right search checks 6 value(s)
   ```

   Three guesses found 23, while a left-to-right search needed six checks.
   Every loop the range keeps halving — a ten-value list, five, then two, then
   one. The step count is the cost of the search.

6. Korean writes the same steps with `동안`, `말해`, and Korean variable
   names like `숫자들`, `낮은`, and `높은`. The full Korean program is in the
   [Korean guide](57-binary-search.ko.md).

## Try it yourself

Change `target` to `5` (near the left edge) and rerun — it takes two steps.
Then try `40`, which is not in the list: the loop runs out of range, prints
`not in the list`, and reports how many steps it needed. Change the list to
the first 100 numbers, `list(range(1, 101))`, and keep `target = 23`: the
range still shrinks in halves, so the guess count barely grows.

## What you learned

- `low` and `high` fence in the range that could still hold the target.
- `mid = (low + high) // 2` picks the middle; `//` rounds down.
- Comparing the guess halves the range — a miss on the left throws away the left half.
- `break` leaves the loop as soon as the target is found.
- Binary search needs the list to be sorted first.
