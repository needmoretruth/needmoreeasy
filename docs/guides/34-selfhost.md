# 34 — Self-host: NME running NME

English | [한국어](34-selfhost.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [29 — Bootstrap](29-bootstrap.md)
- Topic: self-hosting
- Result: an NME program that compiles a tiny NME-like subset (say/set/while) to Python

Guide [29](29-bootstrap.md) compiled a tiny language called BML. This guide
compiles a subset whose words are already NME words — `say`, `set`, `while`, `end` — the seed of NME running NME.

## Steps

1. The mini-language program — one instruction per line, in NME words:

   ```text
   set count 0
   while count 3
     say hello
     add count 1
   end
   say done
   ```

   `while count 3` means "repeat while count < 3"; `end` closes the block.

2. The whole compiler. Save `selfhost.nme`:

   ```text
   # selfhost.nme — NME compiling a tiny NME-like language.
   # The mini language uses NME words: say <text>, set <name> <int>,
   # add <name> <int>, while <name> <int>, end.
   # Run: nme r selfhost

   program = [
       "set count 0",
       "while count 3",
       "  say hello",
       "  add count 1",
       "end",
       "say done",
   ]

   lines = []
   indent = 0
   for raw in program:
       parts = raw.split()
       verb = parts[0]
       if verb == "say":
           lines.append(" " * indent + f'print("{parts[1]}")')
       elif verb == "set":
           lines.append(" " * indent + f"{parts[1]} = {parts[2]}")
       elif verb == "add":
           lines.append(" " * indent + f"{parts[1]} += {parts[2]}")
       elif verb == "while":
           lines.append(" " * indent + f"while {parts[1]} < {parts[2]}:")
           indent = indent + 4
       elif verb == "end":
           indent = indent - 4
       else:
           lines.append(" " * indent + "# unknown instruction: " + raw)

   source = "\n".join(lines)
   show "generated Python:"
   show source
   show ""
   show "running it:"
   exec(source)
   ```

   `split()` turns each line into words; `indent` tracks block depth, and `exec(source)` runs the generated Python — the trick from guide [29].

3. Run it — no server and no input needed:

   ```sh
   nme r selfhost
   ```

   ```text
   generated Python:
   count = 0
   while count < 3:
       print("hello")
       count += 1
   print("done")

   running it:
   hello
   hello
   hello
   done
   ```

   A compiler written in NME read NME words and ran the result on CPython —
   the seed of NME compiling NME itself.

4. Korean writes the same compiler; only the comments and report lines change. The full program is in the [Korean guide](34-selfhost.ko.md).

## Try it yourself

Add a `say hi 3` form that prints `hi` three times. Hint: translate it to `for _ in range(3): print("hi")`.

## What you learned

- A mini language whose words are NME words is closer to NME itself.
- `say`, `set`, `add`, `while`, and `end` map to Python one line each.
- A compiler that reads NME-like source is the seed of NME running NME.
