# 78 — Compiler tier: a small expression language

English | [한국어](78-expressions.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [77 — Compiler](77-compiler.md), [61 — Modules](61-modules.md)
- Topic: compiler
- Result: a tiny calculator that evaluates 2 + 3 * 4 respecting precedence, as a step toward a real expression compiler

Guide [77](77-compiler.md) dispatched lines and guide [84](84-bootstrap.md) translates them; a real compiler also evaluates expressions, and the first rule is precedence — in `2 + 3 * 4` the multiplication happens first.

## Steps

1. Turn the line into a mixed list of numbers and operators — the token list
   for `2 + 3 * 4` is `[2, "+", 3, "*", 4]`:

   ```nme
   line = "2 + 3 * 4"
   values = []
   for word in line.split():
       if word in ["*", "+", "-"]:
           values.append(word)
       else:
           values.append(int(word))
   ```

2. Reading left to right gives `(2 + 3) * 4 = 20`, but multiplication binds
   tighter: `2 + (3 * 4) = 14`. The fix is two passes over the list —
   `multiply_pass` first, then `add_pass`. Save the calculator as `expr.nme`:

   ```nme
   # expr.nme — evaluate arithmetic like 2 + 3 * 4.
   # Run: nme r expr

   def multiply_pass(values):
       i = 0
       while i < len(values):
           if values[i] == "*":
               values[i - 1 : i + 2] = [values[i - 1] * values[i + 1]]
               i = i - 1
           i = i + 1
       return values

   def add_pass(values):
       i = 0
       while i < len(values):
           if values[i] == "+":
               values[i - 1 : i + 2] = [values[i - 1] + values[i + 1]]
               i = i - 1
           elif values[i] == "-":
               values[i - 1 : i + 2] = [values[i - 1] - values[i + 1]]
               i = i - 1
           i = i + 1
       return values

   def evaluate(line):
       values = []
       for word in line.split():
           if word in ["*", "+", "-"]:
               values.append(word)
           else:
               values.append(int(word))
       values = multiply_pass(values)
       values = add_pass(values)
       return values[0]

   show "Expression calculator — 2 + 3 * 4, or quit."
   while True:
       ask line, "> "
       if line == "quit":
           show "Bye!"
           break
       if line == "":
           continue
       answer = evaluate(line)
       show f"{line} = {answer}"
   ```

   The slice `values[i - 1 : i + 2] = [product]` replaces `a`, `*`, `b` with their product, folding the list to `[14]`.

3. Run it and feed expressions through a pipe:

   ```sh
   printf '2 + 3 * 4\n10 - 2 * 3\n2 * 3 + 4\n1 + 2 + 3\n5 * 2 - 3\nquit\n' | nme r expr
   ```

   ```text
   Expression calculator — 2 + 3 * 4, or quit.
   > 2 + 3 * 4 = 14
   > 10 - 2 * 3 = 4
   > 2 * 3 + 4 = 10
   > 1 + 2 + 3 = 6
   > 5 * 2 - 3 = 7
   > Bye!
   ```

   `2 + 3 * 4` is `14`, not `20` — the multiply pass ran first.

## Try it yourself

Add division to the first pass — accept `"/"` in `evaluate` and an `elif values[i] == "/":` branch in `multiply_pass` using `//`, so `10 - 8 / 2` is `6`.

## What you learned

- `line.split()` turns an expression into words; numbers and operators become one mixed list.
- `*` binds tighter than `+`, so `2 + 3 * 4` is `14`, not `20`.
- `multiply_pass` collapses `*` first; `add_pass` then adds and subtracts.
- Two passes over a list are the seed of an expression parser.
