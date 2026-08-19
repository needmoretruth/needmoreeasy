# 22 — Terminal menu — a small TUI

English | [한국어](22-terminal-menu.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [14 — Screen](14-screen.md), [18 — Adventure](18-adventure.md)
- Topic: projects
- Result: a menu program that clears the screen and draws it again

A menu shows what can be done, lets you choose one, does it, and comes back to
the menu. Because it clears the screen and draws it again, one window seems to
stay alive. The clearing, the box and the rule from
[14 — Screen](14-screen.md) all appear here. **No quotes, no brackets, no
equals sign.**

## Steps

1. Draw **just the menu screen** first:

   ```nme
   clear the screen
   say in a box A small menu
   show greet
   show dice
   show quit
   draw a line
   ```

   The screen goes clean, the title sits in a box, the three things you can do
   appear, and a rule closes them off.

2. **Let them choose, and branch:**

   ```nme
   ask choice What now?
   if choice equals greet
       show Hello there!
   else if choice equals quit
       show Goodbye
   else
       show There is no such thing
   end
   ```

   Words are easier answers than numbers. What someone types is always text, so
   an answer of `1` is the letter `1` and never equals the number 1.

3. **To come back to the menu**, wrap the whole thing in `repeat forever`. The
   only way out is `stop`, so it belongs in the `quit` branch alone:

   ```nme
   repeat forever
       clear the screen
       say in a box A small menu
       show quit
       draw a line
       ask choice What now?
       if choice equals quit
           show Goodbye
           stop
       end
   end
   ```

4. **Pause so the answer can be read.** Without it the screen is wiped before
   anyone sees what just appeared:

   ```nme
   show Hello there!
   wait 1 second
   ```

5. The whole thing is
   [`examples/terminal-menu.nme`](../../examples/terminal-menu.nme), and the
   Korean twin is
   [`examples/terminal-menu.ko.nme`](../../examples/terminal-menu.ko.nme).

   ```nme
   repeat forever
       clear the screen
       say in a box A small menu
       show greet
       show dice
       show quit
       draw a line
       ask choice What now?
       if choice equals greet
           show Hello there!
           wait 1 second
       else if choice equals dice
           set roll to random number from 1 to 6
           show roll
           wait 1 second
       else if choice equals quit
           show Goodbye
           stop
       else
           show There is no such thing
           wait 1 second
       end
   end
   ```

## Try it yourself

Add one more thing it can do. An answer of `time` could show how many seconds
the program has been running, using the stopwatch from [15 — Time](15-timer.md).

## What you learned

- A menu is `repeat forever` plus a chain of conditions. Nothing more.
- Clearing the screen at the top of the loop makes one window seem to live on.
- Words are better answers than numbers: what is typed is always text.
- `wait 1 second` before the screen is wiped is what makes it readable.
- `stop` is the only way out.
