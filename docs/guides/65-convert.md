# 65 — Convert: turn Python into NME

English | [한국어](65-convert.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [16 — Check & Build](16-check-build.md), [64 — Python packages](64-python-packages.md)
- Topic: using the tools
- Result: a small Python file converted into NME

`nme convert` goes the other way: it rewrites safe Python patterns into the
level and language you choose. Lines it cannot convert safely stay Python.

## Steps

1. Create `old.py` with a few familiar lines:

   ```python
   print("hi")
   name = input("Name: ")
   if name:
       print("hi", name)
   ```

2. Convert it to English sentence syntax:

   ```sh
   nme convert old.py --level sentence --language en -o easy.nme
   ```

   The result keeps the `if` (Python) but rewrites what it can:

   ```nme
   show "hi"
   ask name "Name: "
   if name:
       print("hi", name)
   ```

3. The Korean command uses `변환`; beginner syntax keeps the quotes, as the
   converter always does:

   ```sh
   nme 변환 old.py --level beginner --language ko -o easy.ko.nme
   ```

   The result:

   ```nme
   말해 "hi"
   물어봐 name, "Name: "
   if name:
       print("hi", name)
   ```

4. Check and run the converted programs:

   ```sh
   nme c easy
   nme run easy
   ```

   The converter never guesses: anything that could change the meaning stays
   ordinary Python.

## Try it yourself

Add a `for i in range(3): print(i)` line to `old.py` and convert again at
`--level beginner`. The loop becomes `3 times: say i` — but only after you
read the result.

## What you learned

- `nme convert` rewrites safe `print`, `input`, and simple patterns.
- `--level sentence|beginner|advanced` and `--language en|ko` pick the target.
- `nme 변환` is the Korean command.
- Everything uncertain stays Python — no lossy guesses.
