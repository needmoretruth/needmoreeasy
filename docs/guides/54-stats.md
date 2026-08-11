# 54 — Stats: understanding data

English | [한국어](54-stats.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [30 — Data](30-data.md), [42 — Compare](42-compare.md)
- 주제 (Topic): 통계/데이터 / statistics
- 결과물 (Result): JSON 숫자 목록에서 count·평균·중앙값·최빈값·최소·최대·범위 보고하기 / loading a JSON list and reporting count, mean, median, mode, min, max, and range

Averages hide what one number cannot tell you. This guide loads a list of
numbers and reports several statistics, each answering a different question
about the data.

## Steps

1. Create `numbers.json` with a list of scores:

   ```text
   [5, 2, 9, 1, 7, 3, 5, 8]
   ```

2. `statistics` provides the library versions; the guide also computes the
   mean by hand so you can see what it means:

   ```text
   from statistics import mean, median, mode
   ```

   `sum(numbers) / len(numbers)` is the mean: total divided by count. When the
   scores are ordered, the middle one is the median; the value that appears
   most often is the mode.

3. The full program loads the list and prints every statistic. Save
   `stats.nme`:

   ```text
   # stats.nme — several statistics about one list.
   # Run: nme r stats
   # The file numbers.json must exist in the same folder.

   use file latest

   from statistics import mean, median, mode

   numbers = json_load("numbers.json")

   show f"count: {len(numbers)}"
   show f"total: {sum(numbers)}"
   show f"mean by hand: {sum(numbers) / len(numbers)}"
   show f"mean from statistics: {mean(numbers)}"
   show f"median: {median(numbers)}"
   show f"mode: {mode(numbers)}"
   show f"min: {min(numbers)}"
   show f"max: {max(numbers)}"
   show f"range: {max(numbers) - min(numbers)}"
   ```

4. Run it:

   ```sh
   nme r stats
   ```

   ```text
   count: 8
   total: 40
   mean by hand: 5.0
   mean from statistics: 5.0
   median: 5.0
   mode: 5
   min: 1
   max: 9
   range: 8
   ```

5. What each one tells you:

   - **count** — how many numbers there are.
   - **mean** — the balance point: if everyone got the total and split it
     evenly, each would get this.
   - **median** — the middle value when sorted; one unusually large or small
     score moves the mean but not the median.
   - **mode** — the most common value; the only statistic that works on
     categories, not just numbers.
   - **min/max** — the extremes.
   - **range** — how spread out the data is (max − min).

## Try it yourself

Replace `numbers.json` with `[10, 9, 9, 8]` and run again: the median stays 9
while the mean changes, showing why both matter.

## What you learned

- `statistics.mean/median/mode` summarize a list in one number each.
- `sum(...) / len(...)` is the mean by hand.
- `min`/`max` find the extremes; `range = max − min`.
- Mean, median, and mode answer different questions about the same data.
