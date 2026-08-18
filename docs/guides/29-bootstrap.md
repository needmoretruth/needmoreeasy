# 29 — Bootstrap: NME compiling a tiny language

English | [한국어](29-bootstrap.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [28 — Compiler](28-compiler.md), [23 — Modules](23-modules.md)
- Topic: bootstrap
- Result: a tiny compiler written in NME

Guide [28](28-compiler.md) interpreted lines directly. The next step is the
seed of a real compiler: a program that **translates** a tiny language into
another language and runs the result. Writing a compiler in NME is called
bootstrapping, and it is how every real compiler grows. The example
`examples/bootstrap.nme` does exactly this.

## Steps

1. Define a tiny language with five instructions. BML (beginner mini
   language) keeps one instruction per line:

   ```text
   set count 0
   while count 3
     show hello
     add count 1
   end
   show done
   ```

2. Read it line by line and translate each instruction into Python. The
   `show` instruction becomes a `print`, `set` becomes an assignment, and
   `while` opens a block. Indentation tracks the block depth:

   ```text
   # part of examples/bootstrap.nme
   lines = []
   indent = 0
   for raw in program:
       parts = raw.split()
       verb = parts[0]
       if verb == "set":
           lines.append(" " * indent + f"{parts[1]} = {parts[2]}")
       elif verb == "show":
           lines.append(" " * indent + f'print("{parts[1]}")')
       elif verb == "while":
           lines.append(" " * indent + f"while {parts[1]} < {parts[2]}:")
           indent = indent + 4
       elif verb == "end":
           indent = indent - 4
   ```

3. Join the translated lines and run them. Because NME itself runs on
   CPython, the compiler can execute what it produced:

   ```text
   # part of examples/bootstrap.nme
   source = "\n".join(lines)
   exec(source)
   ```

4. Run the whole program:

   ```sh
   nme r examples/bootstrap
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

   A compiler written in NME translated BML to Python, and Python ran it.

## Try it yourself

Add a `sub <name> <int>` instruction to the translator (`-=` in Python), then
a BML program that counts down from 5 to 1.

## What you learned

- A compiler translates source text into another language, then runs it.
- `split()` turns a line into words; `f"..."` builds the output line.
- Indentation depth tracks nested blocks.
- Writing a compiler inside NME is bootstrapping — the seed of self-hosting.
