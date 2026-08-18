# 11 — Check & Build: see the Python

English | [한국어](11-check-build.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★☆☆☆ (2/5)
- Prerequisites: [08 — Break](08-break.md)
- Topic: using the tools
- Result: the habit of verifying programs and reading the generated Python

`nme check` and `nme build` show what your NME really is. `check` asks Python
whether the generated program is valid; `build` shows the generated Python
itself.

## Steps

1. Verify a program without running it:

   ```sh
   nme check hello
   nme c hello
   ```

   `check` prints nothing when the program is fine — silence means success.
   `nme c` is the shortcut.

2. See the Python your program becomes:

   ```sh
   nme build hello -o hello.py
   python3 hello.py
   ```

   For `show Hello world!`, `hello.py` contains:

   ```python
   print("Hello world!")
   ```

   Reading this is how you grow into Python one line at a time.

3. Every error has a stable code. Deliberately put `break` outside a loop
   (a line at the left edge, not inside the block) and check it:

   ```sh
   nme c broken.nme
   ```

   The compiler prints the code, the exact line, and a hint:

   ```text
   error[E0102]: `break` can only be used inside a loop
     --> broken.nme:1:1
     |
   1 | break
     | ^^^^^
     = hint: put it inside `while ... end` or `repeat ... end`
   ```

4. Read the long explanation of a code in Korean or English:

   ```sh
   nme ko E0102
   nme en E0102
   ```

   `nme ko` alone lists every code.

## Try it yourself

Check every guide program you wrote so far with `nme c <file>`, then build one
and read its Python.

## What you learned

- `nme check` / `nme c` verifies without running; silence means success.
- `nme build` / `nme b` prints the generated Python; `-o` saves it.
- Error messages carry codes like `E0102` with an exact caret and hint.
- `nme ko <CODE>` / `nme en <CODE>` explain each code.
