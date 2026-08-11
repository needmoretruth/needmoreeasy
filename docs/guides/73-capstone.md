# 73 — Capstone: a language that compiles to Python

English | [한국어](73-capstone.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [58 — Bytecode](58-bytecode.md), [34 — Self-host](34-selfhost.md)
- 주제 (Topic): 컴파일러/캡스톤 / compiler capstone
- 결과물 (Result): 아주 작은 언어(say/set/add/while/end)를 읽어 Python 소스로 컴파일하고 파일로 저장한 뒤 실행하는 NME 프로그램 / an NME program that reads a small custom language (say/set/add/while/end), compiles it to Python source, writes it to a file, and runs it

Guide [34](34-selfhost.md) compiled NME words into Python and ran them in
memory. Guide [58](58-bytecode.md) turned instructions into data steps. The
capstone finishes the whole path: read a small custom language, compile it to
Python **source**, write that source to `out.py`, and run the file — a real
compiler project in one NME program.

## Steps

1. The input is a small custom language with five verbs. `say` prints, `set`
   stores a number, `add` adds, `while name N` repeats while `name < N`, and
   `end` closes the loop. The program lives as a list of lines:

   ```text
   [
       "set count 0",
       "while count 3",
       "  add count 1",
       "  say count",
       "end",
       "say done",
   ]
   ```

   This is source text, not yet run — exactly what a compiler reads.

2. The compiler turns each line into one Python line. `split()` splits a line
   into words; the first word is the verb, the rest are arguments. `indent`
   tracks block depth: `while` grows it by 4, `end` shrinks it, and
   `" " * indent` writes the leading spaces Python needs — the same string
   multiplication that drew the bars in guide [71](71-chart.md). The full
   compiler also writes the finished source to `out.py` with `file_write`
   (guide [13](13-files.md)) and runs it by reading the file back with `exec`
   — guide [34](34-selfhost.md)'s runner, now reading a real file. Save
   `capstone.nme`:

   ```text
   # capstone.nme — a language that compiles to Python.
   # Run: nme r capstone
   # Reads the mini language, compiles it to Python source,
   # writes out.py, then runs out.py with exec.

   use file latest

   program = [
       "set count 0",
       "while count 3",
       "  add count 1",
       "  say count",
       "end",
       "say done",
   ]

   known = []
   lines = []
   indent = 0
   for raw in program:
       parts = raw.split()
       verb = parts[0]
       if verb == "say":
           text = parts[1]
           if text in known:
               lines.append(" " * indent + f"print({text})")
           else:
               lines.append(" " * indent + f'print("{text}")')
       elif verb == "set":
           known.append(parts[1])
           lines.append(" " * indent + f"{parts[1]} = {parts[2]}")
       elif verb == "add":
           lines.append(" " * indent + f"{parts[1]} += {parts[2]}")
       elif verb == "while":
           lines.append(" " * indent + f"while {parts[1]} < {parts[2]}:")
           indent = indent + 4
       elif verb == "end":
           indent = indent - 4
       else:
           lines.append(" " * indent + "# unknown: " + raw)

   source = "\n".join(lines)
   file_write("out.py", source)

   show "compiled mini language:"
   show source
   show ""
   show "running out.py:"
   exec(open("out.py").read())
   ```

   The `known` list is a symbol table: `set` records its targets, and `say`
   checks it, so `say count` becomes `print(count)` while `say done` becomes
   `print("done")` — a tiny version of the name lists you imported in guide
   [72](72-project-files.md).

3. Run it — no server and no input needed:

   ```sh
   nme r capstone
   ```

   ```text
   compiled mini language:
   count = 0
   while count < 3:
       count += 1
       print(count)
   print("done")

   running out.py:
   1
   2
   3
   done
   ```

   The generated Python is real Python — indent-safe, saved as `out.py`, and
   runnable on its own. The loop counts 1, 2, 3, then `done` prints.

4. Korean writes the same compiler with `파일 사용 최신`, `말해`, and Korean
   report lines; the mini language keeps its English verbs. The full Korean
   program is in the [Korean guide](73-capstone.ko.md).

## Try it yourself

Add a `sub` verb that lowers to `-=`, and a `say text N` form that prints
`text` N times — guide [34](34-selfhost.md) hinted at the loop translation.
Then open `out.py`: it is plain Python, runnable on its own with
`python out.py`.

## What you learned

- A compiler maps each instruction of a source language to a target line.
- `indent` tracking turns `while`/`end` into indented Python blocks.
- A `known` list is a symbol table: `say` tells variables from plain words.
- `file_write` then `exec(open(...))` finishes the compile-and-run path.
- Five verbs, one pipeline — the whole compiler-project path in one program.
