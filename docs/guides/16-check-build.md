# 16 — Check & Build: see the Python

English | [한국어](16-check-build.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★☆☆☆ (2/5)
- Prerequisites: [15 — Time](15-timer.md)
- Topic: using the tools
- Result: the habit of reading the Python your sentences become

This is the last guide of Part 1. Every sentence you have written so far was
turned into a language called **Python** and run as that. Python is a language
a great many people already use. This guide is about seeing that happen. **You
never have to read it** — your programs work either way. It is here for the day
you are curious, and for the day someone asks you for "the Python".

## Steps

1. Open the writing box on [needmoreeasy.com](https://needmoreeasy.com/) and
   type any program:

   ```nme
   show Hello world!
   ```

   Look at the **Python** panel on the right — on a phone, press the "Python"
   button above the box. It changes as you type:

   ```python
   print("Hello world!")
   ```

   One sentence always becomes **one line of Python**. The line numbers never
   shift, so when something goes wrong you can see which sentence caused it.

2. Put in something longer and read the two panels side by side:

   ```nme
   set score to 0
   3 times add 1 to score
   show score
   ```

   The Python panel shows `score = 0`, `for _ in range(3): score = score + 1`
   and `print(score)`.

3. **copy** and **download** above the Python panel take that Python away with
   you. A `.py` file runs anywhere Python is installed, with no NME at all.

4. Now get it wrong on purpose. Write `break` on its own, with no loop:

   <!-- nme-check: skip -->
   ```nme
   break
   ```

   The Python panel turns into an error:

   <!-- nme-check: skip -->
   ```text
   error[E0102]: `break` can only be used inside a loop
   오류[E0102]: `멈춰`는 반복문 안에서만 쓸 수 있어요
     --> program.nme:1:1
     |
   1 | break
     | ^^^^^
     = hint: put it inside `while ... end` or `repeat ... end`
     = 도움말: `동안 ... 끝` 또는 `반복 ... 끝` 안에 넣어 주세요
   ```

   There are three things to read: **what is wrong**, **which line** (the `^`
   marks), and **what to try instead**. It comes in both languages at once, so
   you can show it to anybody.

5. A number like `E0102` never changes. If the message does not help, **copy
   the whole thing and show it to an AI** — more accurately still if you first
   pasted one of [the messages at the top of the site](https://needmoreeasy.com/#ai-help).
   Every number is also listed, with a one-line explanation, at the bottom of
   the [syntax list](../syntax.md).

## Try it

Put each program you have written so far into the writing box and read the
Python panel. Once you can see which sentence becomes which line, you are
already half way into Python without having studied it.

## If you installed it on your own computer (skip this for now)

Once [install](../install.md) is done, the same things have commands.

```sh
nme check hello
nme build hello -o hello.py
nme en E0102
```

`nme check` looks without running, and prints **nothing at all** when the
program is fine — silence is success. `nme build` writes the Python to a file.
`nme en E0102` (or `nme ko E0102`) explains that number in full. The short
forms are `nme c` and `nme b`.

## What you learned

- The Python panel beside the writing box shows the Python as you type.
- One sentence is always one line of Python, and line numbers never shift.
- The Python can be copied or downloaded as a `.py` file.
- An error carries a number that never changes, a `^` under the exact place,
  a hint, and both languages at once.
- Installed, `nme check`, `nme build` and `nme en <code>` do the same jobs.
