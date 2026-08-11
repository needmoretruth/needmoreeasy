# 47 — Progress — a bar in the terminal

English | [한국어](47-progress.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [22 — Terminal menu](22-terminal-menu.md), [05 — Repeat](05-repeat.md)
- 주제 (Topic): TUI/출력 / terminal output
- 결과물 (Result): 반복이 돌면서 한 줄을 채우는 터미널 진행 막대 / a terminal progress bar that fills a row as a loop runs

Long tasks deserve feedback. A progress bar is a row of `#` that grows while a
loop runs. Instead of printing new lines, it redraws the same line — so the
terminal shows one row filling up. This guide builds one with `print` and `\r`
(a carriage return).

## Steps

1. `print()` normally ends with a newline, so a second `print` drops to a new
   row. `end="\r"` replaces the newline with a carriage return, which sends the
   cursor back to the start of the same row — the next `print` overwrites it:

   ```text
   print("Hello", end="\r")
   print("World")
   ```

   Only `World` is visible at the end; the second call overwrote the first row.

2. Repeat the overwrite with a `for` loop and a growing row of `#`. `"#" * i`
   is a string of `i` hashes — `"#" * 3` is `"###"`:

   ```text
   for i in range(1, 11):
       print("#" * i, end="\r")
   print()
   ```

   Each pass redraws the row one hash longer, so the bar fills left to right.
   The bare `print()` after the loop ends with a newline, so later output
   starts on a fresh row.

3. The bar moves too fast to watch. `import time` loads the clock, and
   `time.sleep(0.2)` pauses for two tenths of a second each pass:

   ```text
   import time

   for i in range(1, 11):
       print("#" * i, end="\r")
       time.sleep(0.2)
   print()
   ```

4. The whole program. Save `progress.nme`:

   ```text
   # progress.nme — a terminal progress bar that fills a row.
   # Run: nme r progress
   #
   # A row of # grows from 1 to 10. The \r returns to the start
   # of the row, so each longer row overwrites the one before it,
   # and time.sleep pauses so you can watch the bar fill.
   # The bar runs 10 steps, then a final print() moves to a new row.

   import time

   steps = 10
   show "Working..."
   # Each pass builds one row and redraws it in place.
   for i in range(1, steps + 1):
       filled = "#" * i
       percent = i * 10
       print(f"{filled} {percent}%", end="\r")
       time.sleep(0.2)
   print()
   show "Done!"
   ```

   The bar grew a percent column: `percent = i * 10` turns step 1 into `10%`
   and step 10 into `100%`.

5. Run it:

   ```sh
   nme r progress
   ```

   ```text
   Working...
   ########## 100%
   Done!
   ```

   On a real terminal the ten rows `# 10%` through `########## 100%` replace
   each other on the same line, so you see the bar fill one hash at a time. In
   printed output only the final full bar survives.

## Try it yourself

Change the loop to count down: `range(10, 0, -1)` draws a full bar that empties
to one hash. Or replace `"#"` with `"="` and watch a different character fill
the row.

## What you learned

- `print(..., end="\r")` replaces the newline with a carriage return.
- `"#" * i` repeats a string — the loop draws a growing bar on one row.
- `import time` and `time.sleep(0.2)` pause between steps.
- A progress bar is just a loop that redraws one row until it is full.
