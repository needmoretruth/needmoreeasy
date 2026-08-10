# 09 — And / Or: combine conditions

English | [한국어](09-and-or.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★☆☆ (3/5)
- 선수 지식 (Prerequisites): [08 — Break](08-break.md)
- 주제 (Topic): 조건 / conditions
- 결과물 (Result): 조건을 합쳐 판단하는 프로그램 / a program that judges combined conditions

Real conditions are often two questions. `and` needs both to be true; `or`
needs at least one.

## Steps

1. Create `combine.nme`:

   ```text
   ready = True
   score = 5
   if ready and score > 2 then show Go
   ```

   Both parts are true, so it prints `Go`.

2. Korean connects conditions with `그리고` and `또는`:

   ```text
   만약 준비 그리고 점수가 2보다 크면 성공 말해줘
   ```

   `or` only needs one true side:

   ```text
   준비 = True
   기다림 = False
   만약 준비 또는 기다림 그러면 기다려 말해줘
   ```

3. `and` binds before `or`, exactly like Python:

   ```text
   만약 준비 그리고 기다림 또는 점수 > 2 그러면 성공 말해줘
   ```

   This means `(준비 and 기다림) or (점수 > 2)`.

4. Combined conditions work in loops too:

   ```text
   while ready or waiting
   show Still working
   break
   end
   ```

## Try it yourself

Show a welcome only when a name exists and the hour is late:

```text
name = "Mina"
hour = 21
if name exists and hour > 18 then show Good evening name!
```

## What you learned

- `and` / `그리고` needs both conditions true.
- `or` / `또는` needs only one true.
- `and` binds before `or`, like ordinary Python.
- Conditions combine inside `if`, `만약`, and `while` alike.
