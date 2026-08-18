# 67 — Project — a grade book

English | [한국어](67-grade-book.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [61 — Bank](61-bank.md), [48 — Shop](48-shop.md)
- Topic: a capstone project
- Result: a JSON-persisted grade book with add-student, add-grade, report-averages, and a storage module

The bank in guide [61](61-bank.md) saved a dict through a storage module, and
the shop in guide [48](48-shop.md) ran a command menu. A grade book puts both
together: one dict mapping a student name to a list of grades, a
`gradebook.nme` module that loads and saves it, and a menu with `add`, `grade`,
`report`, and `quit`. `statistics.mean` from guide [54](54-stats.md) turns a
grade list into an average.

## Steps

1. A grade book is one dict: each student name maps to a list of grades.
   `statistics.mean` averages the list:

   ```nme
   from statistics import mean
   books = {"Mina": [88, 90], "Jun": [95]}
   show mean(books["Mina"])
   ```

   It prints `89` — the average of 88 and 90.

2. Storage lives in a module, `gradebook.nme`, exporting `load()` and `save`,
   exactly like the bank module in guide [61](61-bank.md). `load()` returns an
   empty dict when no file exists yet:

   ```nme
   # gradebook.nme — file storage for the grade book.

   import os
   use file latest

   def load():
       if os.path.exists("gradebook.json"):
           return json_load("gradebook.json")
       return {}

   def save(books):
       json_save("gradebook.json", books)
   ```

3. The whole program. Save `grade-book.nme` next to `gradebook.nme`:

   ```nme
   # grade-book.nme — a JSON-persisted grade book.
   # Run: nme r grade-book
   # Type add, grade, report, or quit.

   from "gradebook.nme" import load, save
   from statistics import mean

   books = load()

   show "Grade book — kept in gradebook.json"
   while True:
       show "Commands: add, grade, report, quit"
       ask command, "? "
       if command == "add":
           ask name, "Name? "
           if name in books:
               show "Already added"
           else:
               books[name] = []
               save(books)
               show f"Added {name}"
       elif command == "grade":
           ask name, "Name? "
           if name in books:
               ask score_text, "Score? "
               score = int(score_text)
               books[name].append(score)
               save(books)
               show f"Added {score} for {name}"
           else:
               show "No such student"
       elif command == "report":
           ask name, "Name? "
           if name in books:
               scores = books[name]
               if len(scores) > 0:
                   show f"{name}: {mean(scores):.1f} average, {len(scores)} grades"
               else:
                   show f"{name}: no grades yet"
           else:
               show "No such student"
       elif command == "quit":
           show "Bye!"
           break
       else:
           show "Unknown command"
   ```

   `add` checks the student is new, then creates an empty grade list and saves.
   `grade` checks the student exists, appends one score, and saves. `report`
   averages that student's grades and shows the count — or says there are none
   yet. Every change calls `save`, so the book survives between runs.

4. Run it and feed the commands through a pipe:

   ```sh
   printf 'add\nMina\nreport\nMina\ngrade\nMina\n90\nreport\nMina\nquit\n' | nme r grade-book
   ```

   ```text
   Grade book — kept in gradebook.json
   Commands: add, grade, report, quit
   ? Name? Added Mina
   Commands: add, grade, report, quit
   ? Name? Mina: no grades yet
   Commands: add, grade, report, quit
   ? Name? Score? Added 90 for Mina
   Commands: add, grade, report, quit
   ? Name? Mina: 90.0 average, 1 grades
   Commands: add, grade, report, quit
   ? Bye!
   ```

   The report before any grade says `no grades yet`; after one grade it
   averages 90 with the `:.1f` format, showing one decimal place.

5. Look at `gradebook.json` — it holds the whole book:

   ```nme
   {"Mina": [90]}
   ```

   Add a grade for Mina (`grade` then `Mina` then `100`) and the file becomes
   `{"Mina": [90, 100]}` with an average of 95.0.

## Try it yourself

Add a `list` command that reports every student's average, or a `top` command
that sorts the students by average with the `sorted(..., key=...)` trick from
guide [66](66-top-ten.md). Or make `report` print the highest grade too with
`max(scores)`.

## What you learned

- A grade book is a dict mapping a name to a list of grades, saved as JSON.
- `load()` / `save()` in `gradebook.nme` keep the file format in one module.
- `statistics.mean(scores)` turns a grade list into an average.
- Commands `add`, `grade`, `report`, `quit` drive a `while True:` menu.
