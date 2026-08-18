# 04 — Update: change a value

English | [한국어](04-update.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★☆☆☆☆ (1/5)
- Prerequisites: [03 — Set](03-set.md)
- Topic: changing values
- Result: a program that adds to and subtracts from a score

Games need scores that change. `score = 0` starts a score, and `add` changes
it without `+` or `=`.

## Steps

1. Create `score.nme`:

   ```text
   score = 0
   score add 1
   show score
   ```

   `score = 0` stores the number 0; `score add 1` increases it. The program
   prints `1`.

2. The value can be changed in several natural word orders:

   ```text
   add 1 to score
   score increase by 1
   subtract 1 from score
   ```

   Add these lines one at a time and watch `show score` change.

3. Korean works the same way:

   ```text
   점수는 0
   점수에 1 더해
   점수 보여줘
   점수에서 1 빼줘
   ```

4. The advanced spelling `+=` means the same thing and is valid Python:

   ```python
   score += 1
   ```

## Try it yourself

Count up to 5 by adding, then count back down by subtracting:

```text
score = 0
score add 5
score subtract 2
show score
```

## What you learned

- `score = 0` stores a number; `점수는 0` is the Korean form.
- `score add 1`, `add 1 to score`, and `score increase by 1` all add.
- `subtract 1 from score` / `점수에서 1 빼줘` subtract.
- `score += 1` is the same idea in ordinary Python.
