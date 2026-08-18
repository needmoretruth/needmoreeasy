# 15 — Time: a stopwatch and cooldowns

English | [한국어](15-timer.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★☆☆☆ (2/5)
- Prerequisites: [14 — Screen](14-screen.md)
- Topic: stopwatch and cooldowns
- Result: a program that times itself and stops an action from repeating too soon

Games keep needing two things: how long did that take, and you cannot do that
again yet. Both are sentences.

## Steps

1. Type this into the writing box:

   ```nme
   start the timer
   wait 3 seconds
   show elapsed
   ```

   A number close to `3.0` appears. Korean is `시간 재기 시작해` and `잰시간 말해줘`.

2. `elapsed` is a value, so it goes wherever a value goes. Store it:

   ```nme
   start the timer
   wait 1 second
   set spent to elapsed
   show spent
   ```

   Or put it in a condition:

   ```nme
   start the timer
   if elapsed is greater than 10
       show That took too long
   end
   ```

3. Reading `elapsed` without starting the timer is refused while compiling. If
   you see error `E0226`, add `start the timer` on an earlier line.

4. A **cooldown** means "this cannot happen again for a few seconds". Give it a
   name:

   ```nme
   put attack on cooldown for 3 seconds
   ```

   Korean is `공격 쿨타임 3초 걸어`.

5. Ask whether it is over:

   ```nme
   put attack on cooldown for 3 seconds
   when attack is on cooldown
       show Not yet
   end
   ```

   The opposite is `when attack is ready`. Korean is `공격 쿨타임이 남았으면` and
   `공격 쿨타임이 끝났으면`.

6. Or wait it out:

   ```nme
   put attack on cooldown for 2 seconds
   wait for attack
   show You can attack now
   ```

   Korean is `공격 쿨타임 끝날때까지 기다려`.

## Try it

Put a door on cooldown, wait for it, and show how long it took:

```nme
start the timer
put door on cooldown for 2 seconds
wait for door
show The door opened
show elapsed
```

## What you learned

- `start the timer` / `시간 재기 시작해` starts the stopwatch.
- `elapsed` / `잰시간` is the seconds since it started. It is a value, so it goes
  anywhere a value goes.
- `put attack on cooldown for 3 seconds` / `공격 쿨타임 3초 걸어` sets a cooldown.
- `when attack is ready` / `is on cooldown` asks about it.
- `wait for attack` / `공격 쿨타임 끝날때까지 기다려` sleeps out what is left.
- Cooldowns are named, so several can run at once and each is counted separately.
