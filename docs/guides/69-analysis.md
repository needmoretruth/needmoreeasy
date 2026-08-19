# 69 — Data: analyzing a month of temperatures

English | [한국어](69-analysis.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [67 — Stats](67-stats.md), [40 — CSV](40-csv.md)
- Topic: working with data
- Result: loading a month of temperatures, computing statistics and a histogram, and saving a report file

Averages are a start, not the whole story. This guide analyzes a real-ish
dataset: 30 daily temperatures. The numbers describe it, the histogram
draws its shape, and a report file keeps the findings — the same three
steps a data analysis project does at any size.

## Steps

1. Create `temps.json` with 30 daily high temperatures (Celsius):

   ```nme
   [22, 24, 21, 25, 27, 26, 23, 20, 19, 24,
    26, 28, 29, 30, 27, 25, 24, 23, 22, 26,
    28, 31, 32, 29, 26, 24, 23, 21, 20, 25]
   ```

2. Load it with `json_load` (guide [39](39-json.md)) and compute the
   statistics from guide [67](67-stats.md):

   ```nme
   use file latest

   from statistics import mean, median

   temps = json_load("temps.json")
   average = round(mean(temps), 1)
   middle = median(temps)
   hottest = max(temps)
   coolest = min(temps)
   ```

   `round(mean(temps), 1)` keeps one decimal, so the report does not print
   25.333333333333332.

3. A histogram shows the shape of the data: count how many days fall into
   each range, then draw one bar per range with string multiplication
   (guide [34](34-chart.md)):

   ```nme
   ranges = ["under 20", "20-24", "25-29", "30+"]
   counts = [0, 0, 0, 0]
   for t in temps:
       if t < 20:
           counts[0] = counts[0] + 1
       elif t < 25:
           counts[1] = counts[1] + 1
       elif t < 30:
           counts[2] = counts[2] + 1
       else:
           counts[3] = counts[3] + 1

   for i in range(4):
       show f"{ranges[i]:8s} {'#' * counts[i]}"
   ```

   The `elif` chain places each day in exactly one range, so the four
   counts always add up to 30 — a check worth making.

4. The full program also writes the findings into `report.txt`. Save
   `analysis.nme`:

   ```nme
   # analysis.nme — analyze a month of temperatures and write a report.
   # Run: nme r analysis
   # The file temps.json must exist in the same folder.

   use file latest

   from statistics import mean, median

   temps = json_load("temps.json")
   average = round(mean(temps), 1)
   middle = median(temps)
   hottest = max(temps)
   coolest = min(temps)

   lines = []
   lines.append(f"temperature report ({len(temps)} days)")
   lines.append(f"  average: {average}")
   lines.append(f"  median:  {middle}")
   lines.append(f"  hottest: {hottest}")
   lines.append(f"  coolest: {coolest}")

   ranges = ["under 20", "20-24", "25-29", "30+"]
   counts = [0, 0, 0, 0]
   for t in temps:
       if t < 20:
           counts[0] = counts[0] + 1
       elif t < 25:
           counts[1] = counts[1] + 1
       elif t < 30:
           counts[2] = counts[2] + 1
       else:
           counts[3] = counts[3] + 1

   lines.append("distribution:")
   for i in range(4):
       lines.append(f"  {ranges[i]:8s} {'#' * counts[i]}")

   report = "\n".join(lines)
   show report
   file_write("report.txt", report)
   show "saved report.txt"
   ```

   Every finding is appended to `lines` first, then printed and saved with
   the same text — the report cannot drift between screen and file.

5. Run it:

   ```sh
   nme r analysis
   ```

   ```text
   temperature report (30 days)
     average: 25
     median:  25.0
     hottest: 32
     coolest: 19
   distribution:
     under 20 #
     20-24    #############
     25-29    ##############
     30+      ##
   ```

   The numbers and the histogram agree: most days sit between 20 and 29,
   with one cool day and two hot ones.

## Try it yourself

Add a second file with last year's temperatures and compare the averages,
or count "hot days" (`>= 30`) and put that number into the report too.
The `ranges` list is data — try `["under 18", "18-27", "28+"]` and watch
the `elif` chain still place every day.

## What you learned

- One `elif` chain places every value into exactly one range.
- A histogram is just counts per range drawn with `'#' * count`.
- `round(value, 1)` keeps report numbers short.
- Building the report as a `lines` list keeps screen and file identical.
- Statistics plus a histogram together describe a dataset better than
  either alone.
