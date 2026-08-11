# 40 — Report — writing a summary file

English | [한국어](40-report.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [35 — Diary](35-diary.md), [30 — Data](30-data.md)
- 주제 (Topic): 파일/프로젝트 / files
- 결과물 (Result): JSON 데이터 파일 몇 개를 읽어 file_write로 요약 보고서 하나를 쓰는 프로그램 / reading a few JSON data files and writing one text report with file_write

One JSON file holds one kind of data; a report combines several. This guide
reads a scores file and a players file, computes a short summary — average,
top scorer, players per team — and writes one `report.txt` with `file_write`
from guide [35](35-diary.md).

## Steps

1. Create two data files. `scores.json` pairs each player with a score, and
   `players.json` pairs each player with a team:

   ```text
   [
     {"name": "Mina", "score": 7},
     {"name": "Sana", "score": 9},
     {"name": "Junho", "score": 5}
   ]
   ```

   ```text
   [
     {"name": "Mina", "team": "Seoul"},
     {"name": "Sana", "team": "Busan"},
     {"name": "Junho", "team": "Seoul"}
   ]
   ```

   Each file is a list of dicts, exactly like the `questions.json` from guide
   [38](38-quiz.md). The `name` keys line up, which is how the two files stay
   about the same players.

2. Load both files with `json_load` from guide [14](14-json.md). Two calls,
   two lists:

   ```text
   use file latest
   scores = json_load("scores.json")
   players = json_load("players.json")
   show f"Loaded {len(scores)} scores and {len(players)} players"
   ```

   It prints `Loaded 3 scores and 3 players`.

3. Walk the scores with a `for` loop to total them and find the top scorer. The
   `best_score` check remembers the biggest value seen so far:

   ```text
   total = 0
   best_name = ""
   best_score = -1
   for p in scores:
       total = total + p["score"]
       if p["score"] > best_score:
           best_score = p["score"]
           best_name = p["name"]
   average = total / len(scores)
   ```

   This is the running-total and running-max pattern from guide [30](30-data.md),
   now walking dicts instead of plain numbers.

4. Count players per team with a dict. A team seen for the first time starts at
   1; a team seen again goes up by one:

   ```text
   teams = {}
   for p in players:
       team = p["team"]
       if team in teams:
           teams[team] = teams[team] + 1
       else:
           teams[team] = 1
   ```

5. Build the report as a list of lines and join them with `"\n".join(lines)`.
   `file_write("report.txt", text)` saves the whole text in one call — the same
   helper that saved each diary note in guide [35](35-diary.md):

   ```text
   lines = []
   lines.append("Match report")
   lines.append("Players: " + str(len(players)))
   lines.append("Average score: " + str(average))
   lines.append("Top scorer: " + best_name + " with " + str(best_score))
   for team in sorted(teams):
       lines.append(team + ": " + str(teams[team]) + " player(s)")
   text = "\n".join(lines)
   file_write("report.txt", text)
   ```

   `sorted(teams)` prints the teams in a stable order, just like the sorted
   numbers in guide [39](39-sorting.md).

6. The whole program in one file. Save `report.nme`:

   ```text
   # Report: read two JSON data files and write one text summary.
   # Run: nme r report
   # The files scores.json and players.json must exist in the same folder.

   use file latest

   scores = json_load("scores.json")
   players = json_load("players.json")

   show f"Loaded {len(scores)} scores and {len(players)} players"

   total = 0
   best_name = ""
   best_score = -1
   for p in scores:
       total = total + p["score"]
       if p["score"] > best_score:
           best_score = p["score"]
           best_name = p["name"]

   average = total / len(scores)

   teams = {}
   for p in players:
       team = p["team"]
       if team in teams:
           teams[team] = teams[team] + 1
       else:
           teams[team] = 1

   lines = []
   lines.append("Match report")
   lines.append("Players: " + str(len(players)))
   lines.append("Average score: " + str(average))
   lines.append("Top scorer: " + best_name + " with " + str(best_score))
   for team in sorted(teams):
       lines.append(team + ": " + str(teams[team]) + " player(s)")

   text = "\n".join(lines)
   file_write("report.txt", text)

   show "report.txt written:"
   show text
   ```

7. Run it, then look at the file it wrote:

   ```sh
   nme r report
   cat report.txt
   ```

   ```text
   Loaded 3 scores and 3 players
   report.txt written:
   Match report
   Players: 3
   Average score: 7.0
   Top scorer: Sana with 9
   Busan: 1 player(s)
   Seoul: 2 player(s)
   ```

   ```text
   Match report
   Players: 3
   Average score: 7.0
   Top scorer: Sana with 9
   Busan: 1 player(s)
   Seoul: 2 player(s)
   ```

   The console and the file show the same report: one program turned two data
   files into one readable summary.

8. Korean uses `파일 사용 최신`, `json읽기`, and `파일쓰기`. The full Korean
   program is in the [Korean guide](40-report.ko.md).

## Try it yourself

Add a fourth player to both JSON files and rerun `report.nme`; the counts and
the average update together. Then add a lowest-score line by tracking a
`worst_score` exactly like the `best_score` check.

## What you learned

- `json_load` reads each JSON file into its own list of dicts.
- A `for` loop with a running total and a running max summarizes the scores.
- A dict counts groups such as players per team.
- `file_write("report.txt", text)` saves a whole multi-line report at once.
- Data files stay data; the program turns them into a report.
