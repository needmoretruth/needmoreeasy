# 63 — Tools: reading command-line arguments

English | [한국어](63-argv.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [39 — JSON](39-json.md), [31 — Mini bank](31-bank.md)
- Topic: using the tools
- Result: a todo tool that takes commands like `nme r todo add "buy milk"` on the command line

So far every program asked for its input while running. A real tool reads
its instructions up front: `nme r dice 6` should roll a six-sided die
without asking anything. Those words after the file name are **command-line
arguments** — Python puts them in the list `sys.argv`.

## Steps

1. `import sys` gives you `sys.argv`: the program path plus every word
   typed after it. Save `greet.nme`:

   ```nme
   # greet.nme — greet whoever is named on the command line.
   # Run: nme r greet Mina

   import sys

   name = sys.argv[1]
   show f"hello {name}"
   ```

   ```sh
   nme r greet Mina
   ```

   ```text
   hello Mina
   ```

   `sys.argv[0]` is the program's path and `sys.argv[1]` is the first real
   argument — the same list a normal `python greet.py Mina` command makes.

2. A missing argument must not crash confusingly. `len(sys.argv)` counts
   the words; check it before reading one. Save `dice.nme`:

   ```nme
   # dice.nme — roll a die with the number of sides you ask for.
   # Run: nme r dice 6

   use random latest

   import sys

   if len(sys.argv) < 2:
       show "usage: nme r dice <sides>"
   else:
       sides = int(sys.argv[1])
       show random_number(1, sides)
   ```

   ```sh
   nme r dice 6
   ```

   ```text
   3
   ```

   `int(sys.argv[1])` converts the word `"6"` into the number 6, as in
   guide [04](04-ask.md); without the check, `nme r dice` would crash on
   an empty list. A "usage:" line that shows the right command is the
   friendly way to answer a missing argument.

3. Arguments make a command-line tool: one program, several commands. A
   todo list stores its items in `todo.json` (guide [39](39-json.md)) and
   takes `add`, `done`, and `list` commands. Save `todo.nme`:

   ```nme
   # todo.nme — a todo tool that takes commands on the command line.
   # Run: nme r todo add "buy milk"
   #      nme r todo list
   #      nme r todo done 1

   use file latest

   import sys

   todo_file = "todo.json"

   def load_todos():
       try:
           return json_load(todo_file)
       except Exception:
           return []

   def save_todos(todos):
       json_save(todo_file, todos)

   def show_todos(todos):
       for i, item in enumerate(todos):
           mark = "x" if item["done"] else " "
           show f"{i + 1}. [{mark}] {item['text']}"

   command = sys.argv[1] if len(sys.argv) > 1 else "list"
   todos = load_todos()

   if command == "add":
       text = sys.argv[2] if len(sys.argv) > 2 else "no text"
       todos.append({"text": text, "done": False})
       save_todos(todos)
       show f"added: {text}"
   elif command == "done":
       number = int(sys.argv[2]) - 1
       todos[number]["done"] = True
       save_todos(todos)
       show "marked done"
   else:
       show_todos(todos)
   ```

   `load_todos` returns an empty list when the file does not exist yet
   (guide [59](59-errors.md) explains `try`/`except`), and
   `enumerate` numbers the list starting at 1. Each item is a dict with a
   `text` and a `done` flag — the shape that `json_save` writes and
   `json_load` reads back.

4. Run the tool as a chain of commands:

   ```sh
   nme r todo add "buy milk"
   nme r todo add "learn argv"
   nme r todo done 1
   nme r todo
   ```

   ```text
   added: buy milk
   added: learn argv
   marked done
   1. [x] buy milk
   2. [ ] learn argv
   ```

   The quotes around `"buy milk"` keep the two words together in one
   argument — without them, `buy` and `milk` would be `sys.argv[2]` and
   `sys.argv[3]`. The file `todo.json` is the tool's memory between runs.

## Try it yourself

Add a `clear` command that empties the list, or a `rm` command that
removes one number (`todos.pop(number)`). Change the dice tool to accept
a list of sides and roll each (`for side in sys.argv[1:]`). Then make a
tool of your own: a unit converter (`nme r convert 100 km to mi`) is a
good first one.

## What you learned

- `sys.argv` holds the program path and every word typed after the file.
- `len(sys.argv)` guards against missing arguments; a `usage:` line tells
  the user the right command.
- `int(sys.argv[1])` turns an argument word into a number.
- A command word (`add`, `done`) plus data makes one program into many
  tools, like the mini bank's looped menu in guide [31](31-bank.md) but
  given up front.
- Quotes group several words into a single argument.
