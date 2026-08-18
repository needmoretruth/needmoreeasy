# 50 — Editor — a tiny text editor

English | [한국어](50-editor.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [21 — Progress](21-progress.md), [37 — Files](37-files.md)
- Topic: a terminal app
- Result: a line-based editor with a buffer and add, list, remove, save, and quit commands

A text file is one long string. An editor works on a buffer — the same text
split into a list of lines, one per row. You add, remove, and list those rows,
then `save` writes the whole buffer back to the file.

## Steps

1. `"\n".join(lines)` merges lines into one text with newlines between them;
   `.splitlines()` is the exact opposite. It prints `Hello`, `World!`, then
   `['Hello', 'World!']`:

   ```nme
   lines = ["Hello", "World!"]
   text = "\n".join(lines)
   show text
   back = text.splitlines()
   show back
   ```

2. The buffer lives in the file between runs. As in guide [37](37-files.md), a
   saved file becomes a list of lines, and a first run starts empty:

   ```nme
   use file latest
   import os

   if os.path.exists("notes.txt"):
       buffer = file_read("notes.txt").splitlines()
   else:
       buffer = []
   ```

3. Commands arrive as whole lines. `line.split()` splits on spaces, `parts[0]`
   is the command word, and `" ".join(parts[1:])` reconnects the rest — the
   text for `add`. It prints `add`, then `Hello world`; guide
   [79](79-tokens.md) splits tokens the same way:

   ```nme
   line = "add Hello world"
   parts = line.split()
   show parts[0]
   show " ".join(parts[1:])
   ```

4. `remove <n>` needs a number. `int(parts[1])` turns typed text into an
   integer, `- 1` makes it zero-based, and `buffer.pop(i)` deletes that row.
   It prints `['a', 'c']` — line 2 was removed:

   ```nme
   buffer = ["a", "b", "c"]
   n = 2
   i = int(n) - 1
   buffer.pop(i)
   show buffer
   ```

5. The whole editor. Save `editor.nme`:

   ```nme
   # editor.nme — a tiny line-based text editor.
   # Run: nme r editor
   # Type add <text>, list, remove <n>, save, or quit.

   use file latest
   import os

   # Load the saved buffer, or start with an empty one.
   if os.path.exists("notes.txt"):
       buffer = file_read("notes.txt").splitlines()
   else:
       buffer = []

   show "Tiny editor — notes.txt"
   while True:
       show "Commands: add, list, remove, save, quit"
       ask line, "> "
       parts = line.split()
       command = parts[0] if parts else ""
       if command == "add":
           # The text after the command word is the new line.
           text = " ".join(parts[1:])
           buffer.append(text)
           show f"Added line {len(buffer)}"
       elif command == "list":
           show f"{len(buffer)} lines"
           for i in range(len(buffer)):
               show f"{i + 1}: {buffer[i]}"
       elif command == "remove":
           # Remove line N: 1 is the first line.
           i = int(parts[1]) - 1
           if i >= 0 and i < len(buffer):
               buffer.pop(i)
               show "Removed"
           else:
               show "No such line"
       elif command == "save":
           file_write("notes.txt", "\n".join(buffer))
           show "Saved"
       elif command == "quit":
           show "Bye!"
           break
       else:
           show "Unknown command"
   ```

   `add` appends to the list, `list` walks it with `f"{i + 1}:"` so line
   numbers start at 1, and `save` does the join from step 1 in reverse —
   `"\n".join(buffer)` into the file.

6. Run it and feed the commands through a pipe. `add Hello` and `add World!`
   fill the buffer, `remove 1` deletes `Hello`, and `save` keeps `World!`:

   ```sh
   printf 'add Hello\nadd World!\nlist\nremove 1\nlist\nsave\nquit\n' | nme r editor
   ```

   ```text
   Tiny editor — notes.txt
   Commands: add, list, remove, save, quit
   > Added line 1
   Commands: add, list, remove, save, quit
   > Added line 2
   Commands: add, list, remove, save, quit
   > 2 lines
   1: Hello
   2: World!
   Commands: add, list, remove, save, quit
   > Removed
   Commands: add, list, remove, save, quit
   > 1 lines
   1: World!
   Commands: add, list, remove, save, quit
   > Saved
   Commands: add, list, remove, save, quit
   > Bye!
   ```

   `cat notes.txt` shows the saved buffer — a single `World!` line. Run the
   editor again and `list` loads that line back, so the file and the buffer
   stay in sync across sessions.

## Try it yourself

Add an `upper` command that uppercases the whole buffer before saving
(`[l.upper() for l in buffer]`), or a `clear` command that empties the list.
Make `save` print how many lines it wrote.

## What you learned

- A buffer is a list of lines; `"\n".join(buffer)` saves it and
  `.splitlines()` loads it.
- `line.split()` separates a command word from its text; `" ".join(parts[1:])`
  reconnects the text.
- `int(...) - 1` turns a typed line number into a list index; `buffer.pop(i)`
  removes a row.
- `save` writes the file, so the buffer survives between runs.
