# 28 — Your first compiler — a tiny language

English | [한국어](28-compiler.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [26 — Adventure](26-adventure.md), [23 — Modules](23-modules.md)
- 주제 (Topic): 컴파일러 / compiler
- 결과물 (Result): `add 2 3` 같은 줄을 읽고 답을 출력하는 아주 작은 언어 / a tiny language that reads lines like `add 2 3` and prints the answer

A compiler reads text and decides what it means. You already wrote the hardest
part of one in [27](27-calculator.md): split a line, read its first word, and
branch on it. A compiler just repeats that for every line of a program. This
guide builds the seed of a real compiler — a tiny language with `add` and
`mul` commands — and compares it with the example `examples/tiny-compiler.nme`.

## Steps

1. See the pipeline. Every compiler, however big, does three steps in a loop:
   read a line of text, split it into words, then interpret the words. Your
   language's only job is to answer each line:

   ```text
   # the pipeline: read, split, interpret
   line = "add 2 3"
   parts = line.split()
   show int(parts[1]) + int(parts[2])
   ```

   Running this prints `5` — the same answer your language will give for the
   line `add 2 3`.

2. Save the whole interpreter in one file, `mini.nme`:

   ```text
   # A tiny calculator language: add 2 3, mul 4 5, or quit.
   # Run: nme r mini

   def run_command(parts):
       verb = parts[0]
       if verb == "add":
           return int(parts[1]) + int(parts[2])
       elif verb == "mul":
           return int(parts[1]) * int(parts[2])
       else:
           return "I do not know that command"


   show "Mini language — add 2 3, mul 4 5, or quit."

   while True:
       ask line, "Next line? "
       if line == "quit":
           show "Goodbye!"
           break
       parts = line.split()
       if len(parts) == 3:
           result = run_command(parts)
           show result
       else:
           show "Use: add 2 3 or mul 4 5"
   ```

3. Run it and feed it a small program:

   ```sh
   printf 'add 2 3\nmul 4 5\nadd 1 2\nsub 9 2\nquit\n' | nme r mini
   ```

   ```text
   Mini language — add 2 3, mul 4 5, or quit.
   Next line? 5
   Next line? 20
   Next line? 3
   Next line? I do not know that command
   Next line? Goodbye!
   ```

   `add 2 3` answers `5`, `mul 4 5` answers `20`, and `sub 9 2` is not a
   command yet, so it gets the unknown-command message.

4. Interpreting is a function. `verb = parts[0]` names the first word, and
   `if` on the verb dispatches one branch per command. The second and third
   words become numbers with `int(parts[1])` and `int(parts[2])`:

   ```text
   def run_command(parts):
       verb = parts[0]
       if verb == "add":
           return int(parts[1]) + int(parts[2])
       elif verb == "mul":
           return int(parts[1]) * int(parts[2])
       else:
           return "I do not know that command"
   ```

   The last `else` is your error handling: an unknown verb comes back as a
   message instead of crashing.

5. The loop is the pipeline. Read a line with `ask`, stop on `quit`, split
   with `.split()`, guard the shape with `len(parts) == 3`, then interpret and
   print:

   ```text
   while True:
       ask line, "Next line? "
       if line == "quit":
           show "Goodbye!"
           break
       parts = line.split()
       if len(parts) == 3:
           result = run_command(parts)
           show result
       else:
           show "Use: add 2 3 or mul 4 5"
   ```

6. The example `examples/tiny-compiler.nme` is a real (tiny) compiler: it
   reads source lines from a list instead of the terminal, and it generates
   Python code as its output:

   ```text
   # part of examples/tiny-compiler.nme
   python_lines = []
   for line in tiny_source:
       words = line.split()
       if words[0] == "말하기":
           text = " ".join(words[1:])
           python_lines.append(f"print({text!r})")
   ```

   It splits each line, checks the first word, and builds `print(...)` lines —
   the same read → split → interpret pipeline, with generated Python as the
   answer instead of a number. Add a generator branch that writes `answer =
   5` instead of printing and your interpreter has turned into a compiler.

7. The Korean twin `mini.ko.nme` keeps the same `def` and writes the loop with
   `물어봐`, `만약`, and `말해`:

   ```text
   def run_command(parts):
       verb = parts[0]
       if verb == "add":
           return int(parts[1]) + int(parts[2])


   while True:
       물어봐 line, "다음 줄? "
       만약 line == "quit":
           말해 "안녕히 가세요!"
           break
       parts = line.split()
       result = run_command(parts)
       말해 result
   ```

   The same piped input answers in both languages.

## Try it yourself

Add `sub` and `div` commands to `run_command`: `sub` subtracts
(`int(parts[1]) - int(parts[2])`) and `div` divides. Then feed the program
`sub 9 2` and `div 12 3` and watch the unknown-command message disappear.

## What you learned

- A compiler reads text, splits it into words, and interprets each line.
- A `run_command(parts)` function with `if` on the verb is the dispatch.
- `split()` cuts a line; `int()` turns text into a number.
- The read → split → interpret pipeline is the seed of a real compiler.
