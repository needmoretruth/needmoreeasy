# 45 — CSV: rows of data

English | [한국어](45-csv.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [13 — Files](13-files.md), [30 — Data](30-data.md)
- 주제 (Topic): 데이터 / data
- 결과물 (Result): 쉼표로 나뉜 텍스트 파일을 split(",")로 읽고 한 열의 평균을 구해 요약 CSV를 쓰기 / reading a comma-separated text file with split(","), computing a column's average, and writing a summary CSV

A CSV file is plain text — one row per line, fields separated by commas; a small file only needs `split(",")`.

## Steps

1. Create `data.csv` with one `name,score` row per line:

   ```text
   Mina,90
   Yuna,70
   Sora,85
   Jun,75
   ```

2. Read the file and cut it into rows: `splitlines()` splits the text into
   lines, and `line.split(",")` splits one row into fields:

   ```text
   use file latest
   lines = file_read("data.csv").splitlines()
   parts = lines[0].split(",")
   show parts[0]
   show int(parts[1])
   ```

3. Total the scores and divide by the row count for the average. Save it as
   `scores.nme` — it collects every row and writes a summary:

   ```text
   # scores.nme — read a CSV, average the score column, write a summary.
   # Run: nme r scores

   use file latest

   raw = file_read("data.csv")
   lines = raw.splitlines()

   names = []
   scores = []
   for line in lines:
       parts = line.split(",")
       names.append(parts[0])
       scores.append(int(parts[1]))

   total = 0
   biggest = scores[0]
   for score in scores:
       total = total + score
       if score > biggest:
           biggest = score
   average = total / len(scores)

   show f"Read {len(lines)} rows from data.csv"
   for i in range(len(names)):
       show f"{names[i]}: {scores[i]}"
   show f"Total: {total}"
   show f"Average: {average}"
   show f"Highest: {biggest}"

   summary = f"rows,{len(lines)}\ntotal,{total}\naverage,{average}\nhighest,{biggest}\n"
   file_write("summary.csv", summary)
   show "Wrote summary.csv"
   ```

4. Run it next to `data.csv`, then look at the new `summary.csv`:

   ```sh
   nme r scores
   ```

   ```text
   Read 4 rows from data.csv
   Mina: 90
   Yuna: 70
   Sora: 85
   Jun: 75
   Total: 320
   Average: 80.0
   Highest: 90
   Wrote summary.csv
   ```

   ```text
   rows,4
   total,320
   average,80.0
   highest,90
   ```

## Try it yourself

Track the lowest score too — a `lowest = scores[0]` start, an `if score < lowest` check, and a `lowest,<value>` line in `summary`.

## What you learned

- `file_read(...).splitlines()` cuts a file into rows.
- `line.split(",")` splits a row into fields; `parts[1]` is the second field.
- `int(parts[1])` turns text into a number before adding.
- `file_write` writes the summary back out as CSV.
