# 56 — Log: an event record

English | [한국어](56-log.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [13 — Files](13-files.md), [35 — Diary](35-diary.md)
- Topic: files & logging
- Result: appending a dated line to a log file each time the program runs, using datetime and file_write

The diary in guide [35](35-diary.md) writes one file per day. A **log** is the
opposite: one growing file with a new line for every event, showing *when*
things happened.

## Steps

1. Ask `datetime` for now and shape it as text. `datetime.now()` is this
   moment; `strftime` (guide [24](24-python-packages.md)) formats it — `%Y`
   year, `%m` month, `%d` day, `%H` hour, `%M` minute:

   ```nme
   from datetime import datetime
   now = datetime.now()
   stamp = now.strftime("%Y-%m-%d %H:%M")
   show stamp
   ```

2. `file_write` from guide [13](13-files.md) replaces the whole file, so
   appending means read the old log, add one line, write it all back:

   ```nme
   use file latest
   log = file_read("log.txt")
   file_write("log.txt", log + stamp + " - program started\n")
   ```

   Python's `open(path, "a")` appends in one call — the `a` means append — but
   the read-then-write way also lets the program read the log back for a menu.

3. The full logger appends a dated line on every `add`. The first run must not
   fail when `log.txt` does not exist, so `os.path.exists` starts empty. Save
   it as `log.nme`:

   ```nme
   # log.nme — a small event logger.
   # Run: nme r log

   use file latest
   from datetime import datetime
   import os
   if os.path.exists("log.txt"):
       log = file_read("log.txt")
   else:
       log = ""

   while True:
       ask choice, "(add, show, quit) "
       if choice == "add":
           ask event, "what happened? "
           stamp = datetime.now().strftime("%Y-%m-%d %H:%M")
           log = log + stamp + " - " + event + "\n"
           file_write("log.txt", log)
           show "saved: " + stamp + " - " + event
       elif choice == "show":
           show "log.txt has " + str(len(log.splitlines())) + " line(s):"
           for line in log.splitlines():
               show line
       else:
           show "bye"
           break
   ```

4. Run it twice, adding a different event each time — the second run shows the
   first event still there, the log growing across runs:

   ```sh
   printf 'add\nwater the plants\nshow\nquit\n' | nme r log
   printf 'add\ncall mom\nshow\nquit\n' | nme r log
   ```

   ```text
   (add, show, quit) what happened? saved: 2026-08-11 14:05 - water the plants
   (add, show, quit) log.txt has 1 line(s):
   2026-08-11 14:05 - water the plants
   (add, show, quit) bye
   (add, show, quit) what happened? saved: 2026-08-11 14:05 - call mom
   (add, show, quit) log.txt has 2 line(s):
   2026-08-11 14:05 - water the plants
   2026-08-11 14:05 - call mom
   (add, show, quit) bye
   ```

   The timestamps are real — run it yourself and `log.txt` records the actual
   minute of each `add`.

## Try it yourself

Count events per day: change the timestamp to `strftime("%Y-%m-%d")`, then use
the dict counting from guide [36](36-word-count.md) for how many lines share
each date.

## What you learned

- `datetime.now().strftime(format)` shapes the current moment as text.
- Appending is read + one new line + `file_write`, because `file_write`
  replaces the whole file.
- `open(path, "a")` appends directly; `with` closes the file.
- `os.path.exists` lets the first run start with an empty log.
