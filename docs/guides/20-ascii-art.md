# 20 — ASCII art — drawing with characters

English | [한국어](20-ascii-art.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [14 — Screen](14-screen.md)
- Topic: drawing
- Result: a program that draws shapes by repeating a character

The easiest way to draw on a screen is **to repeat one character**. Five stars
make a line; growing the count line by line makes a triangle.

## Steps

1. **Repeat one character.** Keep the character in a name first:

   ```nme
   set star to *
   set line to star repeated 5 times
   show line
   ```

   That prints `*****`.

2. **The count can be a name too:**

   ```nme
   set star to *
   set n to 3
   set line to star repeated n times
   show line
   ```

3. **Add one each time round and it is a triangle:**

   ```nme
   set star to *
   set n to 1
   repeat 5 times
       set line to star repeated n times
       show line
       add 1 to n
   end
   ```

4. **Counting back down gives the other half.** Together they are a diamond:

   ```nme
   set star to *
   set n to 1
   repeat 5 times
       set line to star repeated n times
       show line
       add 1 to n
   end
   set n to 4
   repeat 4 times
       set line to star repeated n times
       show line
       subtract 1 from n
   end
   ```

5. Mix in [14 — Screen](14-screen.md) and the drawing gets a frame:

   ```nme
   clear the screen
   say in a box Drawing stars
   draw a line
   set star to *
   set n to 1
   repeat 4 times
       set line to star repeated n times
       show line
       add 1 to n
   end
   draw a line
   ```

## Try it yourself

Use `#` or `♥` instead of `*`. Then grow by two each time — change
`add 1 to n` to `add 2 to n`.

## What you learned

- `<name> repeated <count> times` repeats one character; Korean is
  `<이름>을 <개수>개 붙인 것`.
- The count may be a number or a name.
- Growing or shrinking the count inside a loop is what makes a shape.
- Mixed with the screen sentences, a drawing gets a frame.
