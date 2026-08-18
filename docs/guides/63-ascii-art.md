# 63 — ASCII art — drawing with characters

English | [한국어](63-ascii-art.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [05 — Repeat](05-repeat.md), [47 — Progress](47-progress.md)
- Topic: output & loops
- Result: building a right triangle, an upside-down triangle, and a diamond with nested loops and string multiplication

Guide [47](47-progress.md) drew one growing row of `#`. Here the rows keep
their shape, so a picture appears: shapes built from `*` and spaces. Two
tools do the work — string multiplication repeats a character, and a loop
repeats a row. Nested loops turn those rows into triangles and a diamond.

## Steps

1. `"*" * 5` is one string with five stars, and `" " * 2 + "*" * 3` glues two
   spaces to three stars. Strings can multiply and add like numbers:

   ```nme
   show "*" * 5
   show " " * 2 + "*" * 3
   ```

   It prints `*****` and then `  ***` (two spaces, three stars).

2. A `for` loop turns the repeated string into a triangle. Each pass prints
   one row, and the row grows with `i`:

   ```nme
   for i in range(1, 6):
       show "*" * i
   ```

   It prints a right triangle from one star to five.

3. `range(5, 0, -1)` counts down, so the rows shrink. The same loop body
   draws an upside-down triangle:

   ```nme
   for i in range(5, 0, -1):
       show "*" * i
   ```

   It prints five stars down to one.

4. A centered row needs spaces on the left. For row `i` of a five-row shape,
   `n - i` spaces push the stars to the middle, and `2 * i - 1` stars is an
   odd count — 1, 3, 5, and so on:

   ```nme
   n = 5
   for i in range(1, n + 1):
       spaces = " " * (n - i)
       stars = "*" * (2 * i - 1)
       show spaces + stars
   ```

   It prints the pointed top half of a diamond.

5. Now the whole picture in one program. The diamond reuses the centered row
   twice — once counting up for the top, once counting down for the bottom.
   Save `ascii.nme`:

   ```nme
   # ascii.nme — drawing shapes with characters.
   # Run: nme r ascii
   #
   # Shapes are built from two ideas: string multiplication
   # repeats a character, and nested loops repeat a row.
   # A right triangle, an upside-down triangle, and a diamond.

   n = 5

   show "Right triangle:"
   for i in range(1, n + 1):
       show "*" * i

   show ""
   show "Upside-down triangle:"
   for i in range(n, 0, -1):
       show "*" * i

   show ""
   show "Diamond:"
   for i in range(1, n + 1):
       spaces = " " * (n - i)
       stars = "*" * (2 * i - 1)
       show spaces + stars
   for i in range(n - 1, 0, -1):
       spaces = " " * (n - i)
       stars = "*" * (2 * i - 1)
       show spaces + stars
   ```

6. Run it:

   ```sh
   nme r ascii
   ```

   ```text
   Right triangle:
   *
   **
   ***
   ****
   *****

   Upside-down triangle:
   *****
   ****
   ***
   **
   *

   Diamond:
       *
      ***
     *****
    *******
   *********
    *******
     *****
      ***
       *
   ```

   The diamond starts with one star, widens to nine, and narrows back to one.
   The bottom loop starts at `n - 1` so the widest row is not printed twice.

7. Korean writes the same steps with `말해`, `공백`, and `별`. The full
   Korean program is in the [Korean guide](63-ascii-art.ko.md).

## Try it yourself

Change `n` from 5 to 8 and rerun — every shape grows. Then build a letter
`H`: two vertical bars of stars with a middle row of `n` stars, all in one
loop that chooses the middle row with an `if`. Replacing `"*"` with `"@"`
gives the same shapes in a different character.

## What you learned

- `"*" * 5` repeats a string; `" " * 2 + "*" * 3` builds a centered row.
- A loop repeats a row; the loop variable sets each row's length.
- `range(n, 0, -1)` counts down and shrinks the rows.
- A diamond is one centered row counted up and then down again.
- Nested loops — a loop that decides each row's shape — draw pictures.
