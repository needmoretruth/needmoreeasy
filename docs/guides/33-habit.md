# 33 — Project: a habit tracker

English | [한국어](33-habit.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [32 — Grade book](32-grade-book.md), [30 — Shop](30-shop.md)
- Topic: projects
- Result: a JSON-persisted habit tracker with add, check, streak, list, quit, and a module file for the storage logic

Guide [35](35-todo.md) saves a todo list of dicts; guide [61](61-modules.md) splits a project across files. A habit tracker is one dict — each habit's days in a row — saved to JSON.

## Steps

1. Storage lives in a module, `store.nme`, exporting `load()` and `save(habits)`. A habit is a `{name: days}` dict entry; `load()` returns `{}` when no file exists yet:

   ```nme
   # store.nme — file storage for the habit tracker.
   import os
   use file latest
   def load():
       if os.path.exists("habits.json"):
           return json_load("habits.json")
       return {}

   def save(habits):
       json_save("habits.json", habits)
   ```

2. The whole project. Save `habit.nme` next to `store.nme`:

   ```nme
   # habit.nme — a habit tracker that survives between runs.
   # Run: nme r habit

   from "store.nme" import load, save
   habits = load()

   while True:
       show ""
       show "Commands: add, check, streak, list, quit"
       ask command, "? "
       if command == "add":
           ask name, "Habit? "
           habits[name] = 0
           save(habits)
           show f"Added: {name}"
       elif command == "check":
           ask name, "Habit? "
           if name in habits:
               habits[name] = habits[name] + 1
               save(habits)
               show f"Checked: {name} ({habits[name]} in a row)"
           else:
               show f"No habit named {name}"
       elif command == "streak":
           ask name, "Habit? "
           show f"{name}: {habits.get(name, 0)} days in a row"
       elif command == "list":
           show f"{len(habits)} habits"
           for name in habits:
               show f"{name}: {habits[name]}"
       elif command == "quit":
           show "Bye!"
           break
   ```

   `add` starts a habit at 0; `check` adds 1 and saves; `streak` reads it back; `list` visits every pair.

3. Run it and feed the commands through a pipe:

   ```sh
   printf 'add\nwater\ncheck\nwater\ncheck\nwater\nstreak\nwater\nlist\nquit\n' | nme r habit
   ```
   ```text

   Commands: add, check, streak, list, quit
   ? Habit? Added: water

   Commands: add, check, streak, list, quit
   ? Habit? Checked: water (1 in a row)

   Commands: add, check, streak, list, quit
   ? Habit? Checked: water (2 in a row)

   Commands: add, check, streak, list, quit
   ? Habit? water: 2 days in a row

   Commands: add, check, streak, list, quit
   ? 1 habits
   water: 2

   Commands: add, check, streak, list, quit
   ? Bye!
   ```

   The habit started at 0 and grew to 2 — `habits.json` holds `{"water": 2}`. Korean writes the same menu with `물어봐` and `말해` — full pair in [33-habit.ko.md](33-habit.ko.md).

## Try it yourself

Add a `reset` command that sets a habit back to 0 — one `elif` branch and a `save`.

## What you learned

- A habit is a dict of `{name: days}`; `habits[name] = habits[name] + 1` grows a streak.
- A module file owns `load()` and `save()`, and the main program imports them.
- `json_save` persists the whole dict after every change.
- A `while True` menu with `add`/`check`/`streak`/`list`/`quit` and `break` drives the tracker.
