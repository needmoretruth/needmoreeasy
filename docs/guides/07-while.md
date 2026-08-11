# 07 — While: keep going

English | [한국어](07-while.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★☆☆ (3/5)
- 선수 지식 (Prerequisites): [06 — If](06-if.md)
- 주제 (Topic): 반복과 조건 / loops and conditions
- 결과물 (Result): 조건이 참인 동안 반복하는 블록 / a block that loops while a condition is true

`repeat` runs a fixed number of times. `while` keeps running as long as a
condition stays true — and stops when it becomes false.

## Steps

1. Create `count.nme`:

   ```text
   score = 0
   while score < 3
   show score
   add 1 to score
   end
   ```

   The block closes with `end`, so indentation is optional. The program prints
   `0`, `1`, `2` and stops when `score` reaches 3.

2. Korean uses `동안` and `끝`:

   ```text
   점수는 0
   동안 점수 < 3
       점수 보여줘
       점수에 1 더해
   끝
   ```

3. English and Korean can mix in one condition. The Korean ending `동안`
   follows the subject:

   ```text
   점수는 0
   while 점수가 3보다 작을 동안
   show 점수
   add 1 to 점수
   end
   ```

4. A spoken one-line loop puts the ending on the subject. This one runs
   forever, so stop it with Ctrl+C in the terminal:

   ```text
   준비 = True
   준비하는동안 성공 말해줘
   ```

## Try it yourself

Loop while a name is missing, asking until one appears:

```text
name = ""
동안 name == ""
    name을 물어봐 이름이 뭐예요?
끝
```

## What you learned

- `while condition` ... `end` loops while the condition is true.
- `동안 조건` ... `끝` is the Korean spelling.
- The loop must change its condition, or it never stops.
- Spoken Korean can end the loop on the subject: `준비하는동안`.
