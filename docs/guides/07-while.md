# 07 — While: keep going

English | [한국어](07-while.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★☆☆☆ (2/5)
- Prerequisites: [06 — If](06-if.md)
- Topic: loops and conditions
- Result: a block that loops while a condition is true

`repeat` runs a fixed number of times. `while` keeps running as long as a
condition stays true — and stops when it becomes false.

## Steps

1. Create `count.nme`:

   ```nme
   set score to 0
   while score is less than 3
   show score
   add 1 to score
   end
   ```

   The block closes with `end`, so indentation is optional. The program prints
   `0`, `1`, `2` and stops when `score` reaches 3.

2. Korean uses `동안` and `끝`:

   ```nme
   점수는 0
   점수가 3보다 작은 동안
       점수 보여줘
       점수에 1 더해
   끝
   ```

3. English and Korean can mix in one condition. The Korean ending `동안`
   follows the subject:

   ```nme
   점수는 0
   while 점수가 3보다 작을 동안
   show 점수
   add 1 to 점수
   end
   ```

4. A spoken one-line loop puts the ending on the subject:

   ```nme
   준비는 참
   준비하는동안 성공 말해줘
   준비는 거짓
   ```

   The last line makes the condition false, so the loop stops after one
   round. Without it the loop never ends.

## Try it yourself

Loop while a name is missing, asking until one appears:

```nme
set name to empty
while name equals empty
    ask name What is your name?
end
```

## What you learned

- `while condition` ... `end` loops while the condition is true.
- `동안 조건` ... `끝` is the Korean spelling.
- The loop must change its condition, or it never stops.
- Spoken Korean can end the loop on the subject: `준비하는동안`.
