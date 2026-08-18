# 09 — And / Or: combine conditions

English | [한국어](09-and-or.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★☆☆☆ (2/5)
- Prerequisites: [08 — Break](08-break.md)
- Topic: conditions
- Result: a program that judges combined conditions

Real conditions are often two questions. `and` needs both to be true; `or`
needs at least one.

## Steps

1. Create `combine.nme`:

   ```nme
   set ready to True
   set score to 5
   if ready and score is greater than 2 then show Go
   ```

   Both parts are true, so it prints `Go`.

2. Korean connects conditions with `그리고` and `또는`:

   ```nme
   준비는 참
   점수는 5
   만약 준비 그리고 점수가 2보다 크면 성공 말해줘
   ```

   `or` only needs one true side:

   ```nme
   준비는 참
   기다림은 거짓
   만약 준비 또는 기다림 그러면 기다려 말해줘
   ```

3. `and` binds before `or`, exactly like Python:

   ```nme
   만약 준비 그리고 기다림 또는 점수가 2보다 크면 성공 말해줘
   ```

   This means `(준비 and 기다림) or (점수 > 2)`.

4. Combined conditions work in loops too:

   ```nme
   set ready to True
   set waiting to False
   while ready or waiting
       show Still working
       break
   end
   ```

## Try it yourself

Show a welcome only when a name exists and the hour is late:

```nme
set name to Mina
set hour to 21
if name exists and hour is greater than 18 then show Good evening name!
```

## What you learned

- `and` / `그리고` needs both conditions true.
- `or` / `또는` needs only one true.
- `and` binds before `or`, like ordinary Python.
- Conditions combine inside `if`, `만약`, and `while` alike.
