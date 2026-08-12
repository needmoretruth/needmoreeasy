# 24 — Python packages: the standard library and installed libraries

English | [한국어](24-python-packages.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [23 — Modules](23-modules.md), [13 — Files](13-files.md)
- 주제 (Topic): Python 패키지 / Python packages
- 결과물 (Result): 표준 라이브러리와 설치된 라이브러리 사용하기 / using the standard library and installed libraries

Python ships with many ready-made packages. Because advanced NME is ordinary
Python, any of them is available inside a `.nme` file — that is the third
syntax level. The example `examples/birthday.nme` uses the `datetime` package
to count down to a birthday.

## Steps

1. Import the part of the package you need with a Python import line:

   ```text
   # part of examples/birthday.nme
   from datetime import date

   today = date.today()
   show today.year
   ```

   `date` now works like any value: `date(2026, 12, 25)` builds a date and
   `today.year` reads the year.

2. Ask for input with beginner syntax and combine it with the package:

   ```text
   # part of examples/birthday.nme
   ask month, "your birth month (1-12): "
   ask day, "your birth day (1-31): "

   today = date.today()
   this_year = date(today.year, int(month), int(day))

   if this_year < today:
       this_year = date(today.year + 1, int(month), int(day))

   show "your next birthday is in " + str((this_year - today).days) + " days"
   ```

   Run it and answer with your birth month and day:

   ```sh
   nme run birthday
   ```

   ```text
   your next birthday is in 136 days
   ```

3. Other standard packages work the same way. `statistics` can average a
   list, `collections` can count things, and `json` (already used by the
   `use file` module) reads and writes data.

4. Third-party libraries are installed with NME's package command, a small
   wrapper around Python's pip. It chooses the usual Python command for your
   operating system and installs one package at a time:

   ```sh
   nme install requests
   ```

   The Korean command form is equivalent:

   ```sh
   nme 설치 requests
   ```

   The command needs internet access. If pip fails, NME reports E9025; check
   the package name, your connection, and that pip is installed before trying
   again. After a successful install, import the package the same way:

   ```text
   import requests
   ```

   An installed package is used exactly like a standard one. Package
   installation is not part of this offline compiler.

## Try it yourself

Change `birthday.nme` to count down to a favorite holiday instead of a
birthday, or print the weekday of a date with `date(2026, 12, 25).strftime("%A")`.

## What you learned

- `from datetime import date` brings a package name into your program.
- The standard library is always available inside NME.
- Beginner `ask` and Python package calls mix freely on one file.
- `nme install` / `nme 설치` wraps pip, then third-party packages import the same way.
