# 21 — Progress — showing how far along you are

English | [한국어](21-progress.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [14 — Screen](14-screen.md), [20 — ASCII art](20-ascii-art.md)
- Topic: screen and time
- Result: working through a list of jobs while a bar grows to show the progress

A long job should show how far along it is. With nothing on screen it looks
like the program has stopped. One bar is enough — a bar that gets one block
longer with every step.

## Steps

1. **A bar is one block repeated.** It is the same sentence as in
   [guide 20](20-ascii-art.md):

   ```nme
   set block to *
   show block repeated 3 times
   ```

   You get `***`.

2. **To choose the length you need to know which step you are on.** Adding
   `with place` to a list loop tells you:

   ```nme
   set jobs to list of reading, adding up, saving
   for each job in jobs with place
       show place
       show job
   end
   ```

   You get `1` and `reading`, then `2` and `adding up`, then `3` and `saving`.
   **Places are counted from 1.**

3. **Use that place as the length.** One block on the first step, three on the
   third:

   ```nme
   set block to *
   set jobs to list of reading, adding up, saving
   for each job in jobs with place
       set bar to block repeated place times
       show bar
   end
   ```

   Each bar is one longer than the last, but they pile up.

4. **Clear the screen before each one so they do not pile up.** That is
   `clear the screen` from [guide 14](14-screen.md), and now the bar looks
   like it is growing:

   ```nme
   set block to *
   set jobs to list of reading, adding up, saving
   for each job in jobs with place
       set bar to block repeated place times
       clear the screen
       show job
       show bar
       wait 0.2 seconds
   end
   ```

   Without `wait 0.2 seconds` it is over before you can see it. A real program
   does the real work there instead of waiting.

5. The whole thing:

   ```nme
   set block to *
   set jobs to list of reading, adding up, drawing, saving
   for each job in jobs with place
       set bar to block repeated place times
       clear the screen
       show job
       show bar
       wait 0.2 seconds
   end
   show all done
   ```

## Try it yourself

Change `set block to *` to `set block to #` or `set block to =`. Then add more
jobs to the list — the bar gets longer to match, and there is no length to fix
anywhere.

## What you learned

- `<block> repeated <n> times` makes one row of a bar.
- `with place` on a list loop tells you which step you are on, counting from 1.
- Using that place as the length makes the bar grow one block per step.
- `clear the screen` before each draw keeps it growing in place instead of piling up.
