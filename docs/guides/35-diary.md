# 35 — Diary: notes saved by date

English | [한국어](35-diary.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [24 — Python packages](24-python-packages.md), [13 — Files](13-files.md)
- 주제 (Topic): 파일 / files
- 결과물 (Result): 각 날의 메모를 날짜별 파일에 저장하고 다시 읽는 일기 / a diary that saves each day's note to a dated file and can read it back

A diary is one file per day. Python's `datetime` package tells you today's
date, and the `file` helper from guide [13](13-files.md) saves and reads the
note. Put the two together and every note lands in a file named after its day.

## Steps

1. Get today's date as text. `date.today()` from the `datetime` package
   (guide [24](24-python-packages.md)) knows the real date, and `str()` turns
   it into the text `2026-08-11`:

   ```text
   from datetime import date
   today = str(date.today())
   show today
   ```

   The output is the real current date, so it shows today's date on any day.

2. Save a note into a file whose name contains the date. The `f` in `f"..."`
   fills `{today}` into the filename, then `file_write` and `file_read` from
   guide [13](13-files.md) store and load the note:

   ```text
   use file latest
   from datetime import date
   today = str(date.today())
   file_write(f"diary-{today}.txt", "Had coffee with a friend.")
   show file_read(f"diary-{today}.txt")
   ```

   A new file appears in the folder for each day the diary is used.

3. The whole diary is one menu loop, like the terminal menu from guide
   [22](22-terminal-menu.md): `add` saves a note, `read` shows today or a past
   date, and `quit` breaks the loop. Save it as `diary.nme`:

   ```text
   # A diary: each day's note goes to its own dated file.
   # Run: nme r diary

   use file latest
   from datetime import date

   show "diary menu (add, read, quit)"
   while True:
       ask action, "choice (add, read, quit): "
       if action == "add":
           ask note, "note: "
           today = str(date.today())
           file_write(f"diary-{today}.txt", note)
           show "saved to " + f"diary-{today}.txt"
       elif action == "read":
           ask when, "which day (today, date): "
           if when == "today":
               today = str(date.today())
               show file_read(f"diary-{today}.txt")
           else:
               ask day, "date (YYYY-MM-DD): "
               show file_read("diary-" + day + ".txt")
       else:
           show "bye"
           break
   ```

4. Run it and feed the menu three answers — add a note, read today, quit:

   ```sh
   printf 'add\nHad coffee with a friend.\nread\ntoday\nquit\n' | nme r diary
   ```

   ```text
   diary menu (add, read, quit)
   choice (add, read, quit): note: saved to diary-2026-08-11.txt
   choice (add, read, quit): which day (today, date): Had coffee with a friend.
   choice (add, read, quit): bye
   ```

   The filename shows the real date; yours prints today's date instead.

5. A past date is read the same way in reverse: `ask` collects the date and
   `file_read` opens that exact file. That is the `read date` branch:

   ```text
   use file latest
   ask when, "which day (today, date): "
   if when == "today":
       today = str(date.today())
       show file_read(f"diary-{today}.txt")
   else:
       ask day, "date (YYYY-MM-DD): "
       show file_read("diary-" + day + ".txt")
   ```

   The branch checks `when`, and only opens `diary-<date>.txt` when the day is
   not today.

## Try it yourself

Add a `list` choice that shows every diary file in the folder. `from pathlib
import Path` and a `for` loop over `Path(".").glob("diary-*.txt")` lists the
dated files:

```text
from pathlib import Path
for p in sorted(Path(".").glob("diary-*.txt")):
    show p.name
```

Add `list` to the menu prompt and a new `elif action == "list":` branch that
runs this loop.

## What you learned

- `from datetime import date` and `str(date.today())` give today's date as text.
- `f"diary-{today}.txt"` builds a filename from the date.
- `file_write` saves a note to that file and `file_read` reads it back.
- A `while True:` menu turns one note per day into a growing diary.
