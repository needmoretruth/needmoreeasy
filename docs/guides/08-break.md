# 08 — Break: stop a loop

English | [한국어](08-break.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★☆☆☆ (2/5)
- Prerequisites: [07 — While](07-while.md)
- Topic: loops
- Result: a program that leaves a loop early

Sometimes a loop should stop in the middle. `break` leaves the nearest loop
immediately and continues with the lines after it.

## Steps

1. Create `stop.nme`:

   ```text
   set ready to True
   set waiting to False
   while ready or waiting
   show Still working
   break
   end
   ```

   The first round runs and prints `Still working`; the `break` stops the
   loop, so it runs exactly once.

2. Korean uses `동안`, `멈춰`, and `끝`:

   ```text
   준비 = True
   동안 준비
       작동 중이에요 말해줘
       멈춰
   끝
   ```

3. `break here` is the longer sentence spelling, and `skip` jumps straight to
   the next round instead of leaving the loop:

   ```text
   set score to 0
   while score is less than 10
   add 1 to score
   if score equals 1
   skip
   end
   break here
   end
   ```

4. A `break` outside a loop is an error with code `E0102`; guide 11 shows how
   to read the message. Use `break` only inside `while` or `repeat` blocks.

## Try it yourself

Stop a loop as soon as a name exists:

```text
set name to empty
while name equals empty
    ask name What is your name?
    if name exists
        break
    end
end
```

## What you learned

- `break` / `멈춰` leaves the nearest loop at once.
- `break here` / `여기서 멈춰` are the longer sentence spellings.
- `skip` / `건너뛰어` goes to the next round instead of leaving the loop.
- A loop that always breaks still runs once.
- `break` outside a loop is the `E0102` error.
