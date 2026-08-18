# 74 — Merge: joining two lists

English | [한국어](74-merge.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [66 — Top ten](66-top-ten.md), [31 — Records](31-address-book.md)
- Topic: data & merging
- Result: loading two JSON lists and joining records by name key into one report

Real data often lives in more than one file. A school keeps students in one
list and their scores in another. Joining them means looking a name up in the
second list — exactly what a dict is for.

## Steps

1. Create two data files. `students.json` has one dict per student:

   ```text
   [
     {"name": "Mina", "class": "A"},
     {"name": "Jun", "class": "A"},
     {"name": "Sora", "class": "B"},
     {"name": "Tom", "class": "B"}
   ]
   ```

2. `scores.json` has the same names with a score:

   ```text
   [
     {"name": "Mina", "score": 92},
     {"name": "Jun", "score": 88},
     {"name": "Tom", "score": 75}
   ]
   ```

3. Build a lookup dict from the scores: each name maps to its score, so
   finding a student's score is one fast `[]` lookup instead of a scan:

   ```text
   scores_by_name = {}
   for record in scores:
       scores_by_name[record["name"]] = record["score"]
   ```

4. The full program loads both lists, joins them, and prints the combined
   report. Save `merge.nme`:

   ```text
   # merge.nme — join students with scores by name.
   # Run: nme r merge
   # students.json and scores.json must be in the same folder.

   use file latest

   students = json_load("students.json")
   scores = json_load("scores.json")

   scores_by_name = {}
   for record in scores:
       scores_by_name[record["name"]] = record["score"]

   show f"class report ({len(students)} students):"
   for student in students:
       name = student["name"]
       score = scores_by_name.get(name, 0)
       show f"  {name} in class {student['class']}: {score} points"
   ```

   `scores_by_name.get(name, 0)` returns the score, or `0` when a student has
   no score yet — so Sora still appears in the report.

5. Run it:

   ```sh
   nme r merge
   ```

   ```text
   class report (4 students):
     Mina in class A: 92 points
     Jun in class A: 88 points
     Sora in class B: 0 points
     Tom in class B: 75 points
   ```

## Try it yourself

Add a second score for Mina in `scores.json` and change the lookup to keep
the highest score, or add a `grade` field derived from the score in the loop.

## What you learned

- A dict turns a name into a value for fast lookup.
- Joining two lists means building a lookup dict from one and looping the
  other.
- `dict.get(key, default)` handles missing keys without crashing.
- Merging is how programs combine data that lives in separate files.
