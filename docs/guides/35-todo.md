# 35 — Todo: a growing project

English | [한국어](35-todo.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [33 — Habit](33-habit.md), [30 — Shop](30-shop.md)
- Topic: projects
- Result: a JSON-persisted todo list with add, done, list, and a module file for the storage logic

Guide [41](41-address-book.md) saves a list of dicts with `json_save`; [61](61-modules.md)
moves code into modules. This guide grows both into a todo list project.

## Steps

1. Storage lives in a module, `store.nme`, exporting `load()` and `save(todos)`:

   ```nme
   # store.nme — file storage for the todo list.

   import os
   use file latest

   def load():
       if os.path.exists("todos.json"):
           return json_load("todos.json")
       return []

   def save(todos):
       json_save("todos.json", todos)
   ```

2. The whole project. Save `todo.nme` next to `store.nme`:

   ```nme
   # todo.nme — a todo list that survives between runs.
   # Run: nme r todo

   from "store.nme" import load, save
   todos = load()

   while True:
       show ""
       show "Commands: add, done, list, quit"
       ask command, "? "
       if command == "add":
           ask text, "Todo? "
           todos.append({"text": text, "done": False})
           save(todos)
           show f"Added: {text}"
       elif command == "done":
           ask num, "Number? "
           i = int(num)
           if i >= 0 and i < len(todos):
               todos[i]["done"] = True
               save(todos)
               show f"Done: {todos[i]['text']}"
           else:
               show f"No todo number {i}"
       elif command == "list":
           show f"{len(todos)} todos"
           for i in range(len(todos)):
               if todos[i]["done"]:
                   show f"{i}: [x] {todos[i]['text']}"
               else:
                   show f"{i}: [ ] {todos[i]['text']}"
       elif command == "quit":
           show "Bye!"
           break
       else:
           show "Unknown command"
   ```

   `add` appends a dict and saves immediately; `done` marks a todo by number
   with an `and` range check.

3. Run it and feed the commands through a pipe:

   ```sh
   printf 'add\nbuy milk\ndone\n0\nlist\nquit\n' | nme r todo
   ```

   ```text
   Commands: add, done, list, quit
   ? Todo? Added: buy milk

   Commands: add, done, list, quit
   ? Number? Done: buy milk

   Commands: add, done, list, quit
   ? 1 todos
   0: [x] buy milk

   Commands: add, done, list, quit
   ? Bye!
   ```

   The todo is saved to `todos.json`, so the next run loads it back. Korean
   writes the same menu with `물어봐` and `말해`; the full pair is in the [Korean guide](35-todo.ko.md).

## Try it yourself

Add a `clear` command that resets `todos` to `[]` and saves — one `elif` branch. Then make `list` count the open items.

## What you learned

- A project splits into a main program and a storage module with a clear interface.
- `json_save` persists a list of dicts; `load()` restores it on the next run.
- `int(num)` and the `and` range check keep a number command safe.
