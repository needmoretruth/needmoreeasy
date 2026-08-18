# 22 — Terminal menu — a small TUI

English | [한국어](22-terminal-menu.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [10 — Random](10-random.md), [06 — If](06-if.md), [07 — While](07-while.md)
- Topic: terminal menus
- Result: a loop-driven terminal menu

A TUI (text user interface) is a menu you drive with the keyboard. The example
`examples/terminal-menu.nme` shows three choices, waits for an answer, does
what you picked, and then shows the menu again. It is a learning project, not
advice.

Run it and feed it two answers — `1` greets you, then `3` quits. Because the
loop goes back to the menu after the first answer, the menu appears twice:

```sh
printf '1\n3\n' | nme r examples/terminal-menu
```

```text
1) greet
2) dice
3) quit
choose: hello!
1) greet
2) dice
3) quit
choose: bye
```

## Steps

1. The `use random latest` line loads the dice function, and a text value holds
   the menu. `\n` means "new line" — it is how one string becomes three rows:

   ```nme
   # part of examples/terminal-menu.nme
   use random latest

   menu = "1) greet\n2) dice\n3) quit"
   ```

2. `while True:` makes an endless loop; `show menu` prints the choices and
   `ask choice, "choose: "` stores your answer:

   ```nme
   # part of examples/terminal-menu.nme
   while True:
       show menu
       ask choice, "choose: "
   ```

   The block starts at the indentation, exactly like Python.

3. The `if`/`elif`/`else` from guide [06](06-if.md) runs one branch per
   answer. The plain Python headers `if choice == "1":` mix freely with the
   NME lines `show`/`break` inside the same block:

   ```nme
   # part of examples/terminal-menu.nme
   while True:
       show menu
       ask choice, "choose: "
       if choice == "1":
           show "hello!"
       elif choice == "2":
           show random_number(1, 6)
       else:
           show "bye"
           break
   ```

   `show random_number(1, 6)` rolls the die from guide [10](10-random.md) on
   the spot; any other answer falls into `else`, prints `bye`, and leaves the
   loop with `break` — the one way out of `while True:`.

4. `nme check` verifies the syntax without running the loop:

   ```sh
   nme check examples/terminal-menu
   ```

   The Korean twin `examples/terminal-menu.ko.nme` uses the same `while True:`
   loop with `ask 선택, "고르세요: "`; `nme r examples/terminal-menu.ko` picks
   the same numbers and gets the same flow in Korean.

## Try it yourself

Add a fourth row `4) coin` to `menu`, then a new `elif choice == "4":` branch
that shows a random pick between two sides — guide [10](10-random.md) shows
how. `break` still works; the extra number just adds another branch.

## What you learned

- `while True:` loops forever; `break` is the way out.
- A menu is show, ask, branch, then loop back.
- `show`/`ask` NME lines mix with plain Python `if choice == "1":` headers.
- `\n` inside a string makes a new line.
