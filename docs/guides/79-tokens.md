# 79 — Compiler tier: reading tokens

English | [한국어](79-tokens.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [78 — Expressions](78-expressions.md), [77 — Compiler](77-compiler.md)
- Topic: compiler
- Result: splitting a command line into tokens and dispatching them, a step toward a real tokenizer and parser

Guide [78](78-expressions.md) split an expression into words; guide [84](84-bootstrap.md)
splits instruction lines. A real compiler starts the same way: it reads source
text and cuts it into **tokens** — the smallest meaningful pieces. This guide
builds a command reader that splits a line into tokens and dispatches each
command, like a compiler reading input one token at a time.

## Steps

1. `split()` cuts a line at the spaces; each word is a token, in order —
   `"move 3"` becomes `["move", "3"]`:

   ```nme
   line = "move 3"
   tokens = line.split()
   show f"{len(tokens)} tokens: {tokens}"
   ```

   It prints `2 tokens: ['move', '3']`. The first token is the command; the
   rest are its arguments.

2. A function dispatches on the first token — the `if`/`elif` chain from guide
   [08](08-if.md), comparing tokens instead of answers. `tokens[1]` is the
   first argument:

   ```nme
   def run_tokens(tokens):
       command = tokens[0]
       if command == "move":
           amount = int(tokens[1])
           show f"Move {amount} steps"
       elif command == "turn":
           show f"Turn {tokens[1]}"
       elif command == "say":
           show tokens[1]
       else:
           show f"Unknown: {command}"
   ```

3. `int(tokens[1])` turns the second token into a number, so `move 3` can add
   real steps. State lives in a dict — `robot = {"direction": "north",
   "steps": 0}` — and `move` adds to `robot["steps"]`. The whole program,
   saved as `tokens.nme`:

   ```nme
   # tokens.nme — split a command line into tokens and dispatch them.
   # Run: nme r tokens
   # A token is one word of a line; split() makes the list.
   # The first token is the command, the rest are its arguments.
   # Real compilers read text, cut tokens, and dispatch on the first.

   def run_tokens(tokens, robot):
       command = tokens[0]
       if command == "move":
           amount = int(tokens[1])
           robot["steps"] = robot["steps"] + amount
           show f"Move {amount} — total {robot['steps']} steps"
       elif command == "turn":
           robot["direction"] = tokens[1]
           show f"Turn {robot['direction']}"
       elif command == "say":
           show tokens[1]
       elif command == "where":
           show f"{robot['direction']} {robot['steps']} steps"
       elif command == "help":
           show "move <n>, turn <dir>, say <text>, where, help, quit"
       else:
           show f"Unknown: {command}"

   robot = {"direction": "north", "steps": 0}
   show "Toy robot: type help for commands."
   while True:
       ask line, "> "
       if line == "":
           continue
       if line == "quit":
           show "Bye!"
           break
       tokens = line.split()
       run_tokens(tokens, robot)
   ```

   `move` adds to `robot["steps"]`, `turn` changes direction, `say` prints the
   rest of the line, `where` reports the state. Empty lines `continue`, and
   `quit` breaks the loop.

4. Run it and feed commands through a pipe:

   ```sh
   printf 'move 3\nturn left\nsay hi\nwhere\nquit\n' | nme r tokens
   ```

   ```text
   Toy robot: type help for commands.
   > Move 3 — total 3 steps
   > Turn left
   > hi
   > left 3 steps
   > Bye!
   ```

   Split → dispatch → loop is the front end of every compiler.

## Try it yourself

Add a `reset` branch that zeroes `robot["steps"]`, or check the argument
count before reading `tokens[1]`.

## What you learned

- A token is one meaningful word; `line.split()` makes the token list.
- Dispatch on `tokens[0]`, the command; the rest are arguments.
- `int(tokens[1])` converts an argument into a number.
- Split → dispatch → loop is the seed of a real tokenizer and parser.
