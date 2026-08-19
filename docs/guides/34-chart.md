# 34 — Chart: a bar chart in the terminal

English | [한국어](34-chart.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [20 — ASCII art](20-ascii-art.md), [24 — Quiz](24-quiz.md)
- Topic: screen and time
- Result: drawing a horizontal bar chart with # blocks from a JSON list, scaled to the largest value

Guide [20](20-ascii-art.md) multiplied strings into rows, and guide
[46](46-top-ten.md) sorts records by a score. A bar chart puts both
together: read values from JSON, then turn each value into a row of `#`
blocks whose length is scaled to the largest value. Numbers become a picture
you can read at a glance.

## Steps

1. Create `chart.json` — a list of records, each with a `label` and a `value`:

   ```nme
   [
     {"label": "Jan", "value": 12},
     {"label": "Feb", "value": 29},
     {"label": "Mar", "value": 18},
     {"label": "Apr", "value": 41},
     {"label": "May", "value": 24},
     {"label": "Jun", "value": 35}
   ]
   ```

   The labels become row names; the values become bar lengths.

2. Load the list with `json_load` from guide [39](39-json.md). Collecting the
   values into their own list makes `max` easy to call:

   ```nme
   use file latest
   rows = json_load("chart.json")
   values = []
   for row in rows:
       values.append(row["value"])
   show max(values)
   ```

   It prints `41` — the largest value, which sets the scale.

3. A bar is a fraction of the longest bar. `max_value` is the largest value;
   `int(value / max_value * 20)` is that value's share of 20 blocks:

   ```nme
   value = 29
   max_value = 41
   length = int(value / max_value * 20)
   show length
   ```

   It prints `14`, because 29 is roughly 14/20 of 41.

4. `"#" * length` builds the bar itself — the string multiplication from guide
   [20](20-ascii-art.md):

   ```nme
   length = 14
   show "#" * length
   ```

   It prints fourteen `#` blocks. Repeating a string is how a number becomes a
   picture.

5. The full program loads the data, finds the scale, and draws one labeled bar
   per record. Save `chart.nme`:

   ```nme
   # chart.nme — a horizontal bar chart in the terminal.
   # Run: nme r chart
   # chart.json must be in the same folder.
   # Each bar is # blocks scaled to the largest value.

   use file latest

   rows = json_load("chart.json")

   values = []
   for row in rows:
       values.append(row["value"])

   max_value = max(values)
   show f"largest value: {max_value}"
   show ""

   for row in rows:
       label = row["label"]
       length = int(row["value"] / max_value * 20)
       bar = "#" * length
       show f"{label:>3}: {bar} {row['value']}"
   ```

   `{label:>3}` pads every label to three columns so all bars start at the
   same place. `length` and `bar` are recalculated for every row.

6. Run it with the data file present:

   ```sh
   nme r chart
   ```

   ```text
   largest value: 41

   Jan: ##### 12
   Feb: ############## 29
   Mar: ######## 18
   Apr: #################### 41
   May: ########### 24
   Jun: ################# 35
   ```

   April, the largest value, fills all 20 blocks; January, the smallest,
   fills 5. Every bar is a fraction of the same scale, so the chart is honest.

7. Korean writes the same steps with `파일 사용 최신`, `json읽기`, `말해`, and
   `막대`. The full Korean program is in the [Korean guide](34-chart.ko.md).

## Try it yourself

Add `{"label": "Jul", "value": 47}` to `chart.json` and rerun — the scale
moves to 47 and every other bar shrinks. Then change 20 to 40 in the length
formula for a wider chart, or add a second `"@"` bar under each row.

## What you learned

- A record's `value` becomes a bar length; `max(values)` sets the scale.
- `int(value / max_value * 20)` turns a value into a count of blocks.
- `"#" * length` repeats one character into a bar.
- `{label:>3}` aligns labels so all bars start at the same column.
