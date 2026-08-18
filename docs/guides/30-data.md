# 30 — Data: statistics on a list

English | [한국어](30-data.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [24 — Python packages](24-python-packages.md), [14 — JSON](14-json.md)
- Topic: data
- Result: loading numbers from a JSON file and computing mean/median/max with the statistics standard library

A saved JSON list is ready-made data for a program. This guide loads a list of
numbers from a file, computes a summary by hand and with the Python standard
library, and prints one report. `use file` reads the data; `statistics` does
the math.

## Steps

1. Create the data file `numbers.json` with one JSON list:

   ```nme
   [5, 2, 9, 1, 7, 3]
   ```

   Six numbers stored as a JSON list. Guide [14](14-json.md) saved a dict with
   `json_save`; a JSON list is the same idea, one value per entry.

2. Load the list with `use file latest` and `json_load`. The loaded value is a
   Python list, so `len(numbers)` counts it. Save this as `load.nme`:

   ```nme
   use file latest
   numbers = json_load("numbers.json")
   show f"Loaded {len(numbers)} numbers"
   ```

   Run `nme r load`; it prints `Loaded 6 numbers`. Because the file holds a
   list, `json_load` returns a list rather than the dict from guide [14](14-json.md).

3. Import `mean` and `median` from the standard library. Guide
   [24](24-python-packages.md) showed `from datetime import date`; importing
   two names at once works the same way:

   ```nme
   use file latest
   from statistics import mean, median

   numbers = json_load("numbers.json")
   show f"Mean: {mean(numbers)}"
   show f"Median: {median(numbers)}"
   ```

   Run it; you see `Mean: 4.5` and `Median: 4.0`. `mean` adds the values and
   divides by the count; `median` is the middle value once the list is sorted.

4. `max(...)` is a Python builtin, so no import is needed:

   ```nme
   numbers = [5, 2, 9, 1, 7, 3]
   show f"Max: {max(numbers)}"
   ```

   It prints `Max: 9`. `min(numbers)` would print `1`.

5. Now the whole report in one file. Save `numbers.nme`:

   ```nme
   # numbers.nme — statistics on a list of numbers saved as JSON.
   # Run: nme r numbers
   # The file numbers.json must exist in the same folder.

   use file latest
   from statistics import mean, median

   # Load the saved list of numbers.
   numbers = json_load("numbers.json")

   # Show what we loaded, one row per number.
   show f"Loaded {len(numbers)} numbers from numbers.json:"
   for n in numbers:
       show f"  {n}"

   # Count and total by hand with a for loop.
   count = 0
   total = 0
   for n in numbers:
       count = count + 1
       total = total + n

   # Find the biggest value by hand.
   biggest = numbers[0]
   for n in numbers:
       if n > biggest:
           biggest = n

   # The hand-written average...
   average = total / count
   show ""
   show f"Count: {count}"
   show f"Total: {total}"
   show f"Average by hand: {average}"

   # ...then the standard library does the same jobs in one call.
   show f"Mean from statistics: {mean(numbers)}"
   show f"Median from statistics: {median(numbers)}"

   # max() is a Python builtin, so no import is needed.
   show f"Max by hand: {biggest}"
   show f"Max from max(): {max(numbers)}"
   ```

   Run it with the data file present:

   ```sh
   nme r numbers
   ```

   ```text
   Loaded 6 numbers from numbers.json:
     5
     2
     9
     1
     7
     3

   Count: 6
   Total: 27
   Average by hand: 4.5
   Mean from statistics: 4.5
   Median from statistics: 4.0
   Max by hand: 9
   Max from max(): 9
   ```

   The hand-written loop shows what `mean` and `max` do inside: a running
   total and a running biggest value. The `statistics` lines give the same
   answers in one call each.

6. Korean writes the same steps with `파일 사용 최신` and `json읽기`. The full
   Korean program is in the [Korean guide](30-data.ko.md); this snippet loads
   the list:

   ```nme
   파일 사용 최신
   숫자들 = json읽기("numbers.json")
   말해 f"숫자 {len(숫자들)}개를 불러왔습니다"
   ```

## Try it yourself

Change `numbers.json` to `[10, 20, 30]` and rerun `numbers.nme`; the mean, the
median, and the max all change together. Then add `show f"Min: {min(numbers)}"`
to the report.

## What you learned

- `json_load` returns a list when the file holds a JSON list.
- `from statistics import mean, median` imports two standard-library names.
- `mean(numbers)` and `median(numbers)` summarize a whole list in one call.
- `max(numbers)` and `min(numbers)` are Python builtins, so no import is needed.
- A hand-written loop can find the same total and max one step at a time.
