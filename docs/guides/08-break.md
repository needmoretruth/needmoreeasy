# 08 — Break: stop a loop

English | [한국어](08-break.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★☆☆ (3/5)
- 선수 지식 (Prerequisites): [07 — While](07-while.md)
- 주제 (Topic): 반복 / loops
- 결과물 (Result): 반복을 일찍 끝내는 프로그램 / a program that leaves a loop early

Sometimes a loop should stop in the middle. `break` leaves the nearest loop
immediately and continues with the lines after it.

## Steps

1. Create `stop.nme`:

   ```text
   ready = True
   waiting = False
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
       show working
       멈춰
   끝
   ```

3. The sentence spellings `break here` and `여기서 멈춰` mean the same thing
   inside a block:

   ```text
   score = 0
   while score < 10
   add 1 to score
   여기서 멈춰
   end
   ```

4. A `break` outside a loop is an error with code `E0102`; guide 11 shows how
   to read the message. Use `break` only inside `while` or `repeat` blocks.

## Try it yourself

Stop a loop as soon as a name exists:

```text
name = ""
동안 name == ""
    이름을 물어봐 이름이 뭐예요?
    만약에 이름이 있으면
        멈춰
    끝
끝
```

## What you learned

- `break` / `멈춰` leaves the nearest loop at once.
- `break here` / `여기서 멈춰` are the sentence spellings.
- A loop that always breaks still runs once.
- `break` outside a loop is the `E0102` error.
