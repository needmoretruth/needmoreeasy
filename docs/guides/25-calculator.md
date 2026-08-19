# 25 — Calculator: asking and working it out

English | [한국어](25-calculator.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [08 — If](08-if.md), [24 — Quiz](24-quiz.md)
- Topic: projects
- Result: a calculator that asks for two numbers and a operation, and says the answer

Everything asked for so far has been **text**. Text cannot be added up —
`"12"` and `"4"` together make `124`. To get a number you have to say so.

## Steps

1. **Ask for a number.** `ask number` reads the answer as a number:

   ```nme
   ask number first the first number
   ask number second the second number
   set result to first
   add second to result
   show result
   ```

   With `12` and `4` you get `16`. Without `number` you get `124` — two pieces
   of text stuck together.

2. **The four operations are four sentences**, all of the shape "do this to
   that name":

   ```nme
   set result to 12
   add 4 to result
   subtract 2 from result
   multiply result by 3
   divide result by 2
   show result
   ```

   Each line changes the result in turn: 12 becomes 16, then 14, then 42, then
   21. Only the last one reaches the screen, and it prints `21.0`.
   **Only division brings a decimal point.** A division rarely comes out even,
   so it always answers with one.

3. **Which operation to do is something you ask.** Four ways means
   `else if`:

   ```nme
   set sign to times
   set result to 12
   if sign equals plus
       add 4 to result
   else if sign equals times
       multiply result by 4
   else
       show that is not an operation I know
   end
   show result
   ```

4. The whole thing:

   ```nme
   ask number first the first number
   ask number second the second number
   ask sign one of plus minus times over
   set result to first
   if sign equals plus
       add second to result
   else if sign equals minus
       subtract second from result
   else if sign equals times
       multiply result by second
   else
       divide result by second
   end
   show result
   ```

## Try it yourself

Add a fifth operation for the remainder — `set result to the remainder of
first divided by second` is that one. Then answer `0` for the second number
and choose the division: the program stops with `ZeroDivisionError`. Guarding
it with `if second equals 0` before dividing is the fix.

## What you learned

- `ask number <name>` reads the answer as a number; plain `ask` reads text.
- The four operations are the sentences `add`, `subtract`, `multiply`, `divide`.
- Only division always answers with a decimal point.
- Three or more ways to go means `else if` between the first and the last.
