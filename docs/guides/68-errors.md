# 68 — Errors: handling problems

English | [한국어](68-errors.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [55 — Network](55-net.md), [13 — Files](13-files.md)
- Topic: exceptions
- Result: a program that reads a missing file and converts bad input without crashing

A missing file or a line that is not a number stops a program dead.
`try` / `except` catches it and runs a friendly fallback.

## Steps

1. A missing file makes `file_read` raise `FileNotFoundError`; the `except` block runs instead of crashing:
   ```nme
   use file latest
   try:
       text = file_read("notes.txt")
   except FileNotFoundError:
       show "notes.txt is not here yet."
   ```
2. `int()` raises `ValueError` on bad input; `else` after `except` runs only when nothing went wrong:
   ```nme
   ask answer, "A number to double: "
   try:
       number = int(answer)
   except ValueError:
       show "That is not a number."
   else:
       show f"Doubled, it is {number * 2}."
   ```
3. The full program reads a file that may be missing, then asks for a number until the answer converts. Create `notes.txt` with two lines first:
   ```nme
   # safe_read.nme — read a file that may be missing, safely.
   # Run: nme r safe_read
   # try / except keeps the program alive when the data is bad.

   use file latest

   ask file_name, "File to read (for example notes.txt): "

   try:
       text = file_read(file_name)
   except FileNotFoundError:
       show "That file is not here yet."
       show "Create it first with file_write, then run again."
   else:
       show f"Read {len(text)} characters."
       show f"That is {len(text.split())} words."
       lines = text.splitlines()
       show f"The first line is: {lines[0]}"
       show "Contents:"
       show text

   show ""
   show "Now give me a number to double."
   while True:
       ask answer, "A number to double: "
       try:
           number = int(answer)
       except ValueError:
           show f"'{answer}' is not a number — try again."
       else:
           break

   show f"Doubled, it is {number * 2}."
   ```
4. Run it with the file present:
   ```sh
   printf 'notes.txt\nseven\n12\n' | nme r safe_read
   ```
   ```text
   File to read (for example notes.txt): Read 30 characters.
   That is 6 words.
   The first line is: Today is sunny.
   Contents:
   Today is sunny.
   We study NME.

   Now give me a number to double.
   A number to double: 'seven' is not a number — try again.
   A number to double: Doubled, it is 24.
   ```
   With a file name that does not exist:
   ```sh
   printf 'nope.txt\n12\n' | nme r safe_read
   ```
   ```text
   File to read (for example notes.txt): That file is not here yet.
   Create it first with file_write, then run again.

   Now give me a number to double.
   A number to double: Doubled, it is 24.
   ```
5. Catch named errors (`FileNotFoundError`, `ValueError`), not a bare `except:`, so unexpected bugs still crash loudly.

## Try it yourself

Read a JSON file with `json_load` and catch `json.JSONDecodeError`, or change the number loop to add two numbers.

## What you learned

- `try:` runs risky code; `except SomeError:` catches exactly that error.
- `FileNotFoundError` means a missing file; `ValueError` means `int()`
  could not convert.
- `else:` after `except` runs only when no error happened.
- Catch named errors so unexpected bugs still crash loudly.
