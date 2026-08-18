# 07 — Repeat: do something many times

English | [한국어](07-repeat.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★☆☆☆ (2/5)
- Prerequisites: [06 — Update](06-update.md)
- Topic: loops
- Result: a program that repeats lines, walks a list, and pauses

Computers are great at doing the same thing many times. `repeat` runs a line
or a block again and again.

## Steps

1. Type this into the writing box:

   ```nme
   repeat 3 times and show Again
   ```

   This prints `Again` three times. Korean is equally valid:

   ```nme
   3번 반복해서 다시 말해줘
   ```

2. When the count comes first, the rest of the line is repeated output:

   ```nme
   3 times Welcome to NME
   3번 안녕하세요
   ```

3. Several lines use indentation and no colon:

   ```nme
   repeat 3 times
       show First sentence
       둘째 문장 말해줘
   ```

4. Indentation can be optional: put `end` (or `끝`) on its own line to close
   the block:

   ```nme
   repeat 3 times
   show one line
   show another line
   end
   ```

   The Korean twin uses `3번 반복해` and `끝`.

5. Instead of a fixed count you can walk **through a list**, the one you made
   in [05 — Save](05-set.md):

   ```nme
   set friends to list of Mina, Ada
   for each friend in friends
   show Hello friend!
   end
   ```

   `friend` becomes the next value on every round. The Korean form is
   `친구들의 친구마다 반복해`.

6. Sometimes a loop should **wait** between rounds:

   ```nme
   repeat 3 times
   wait 1 second
   show tick
   end
   ```

   Korean is `1초 기다려`, and `wait 3 seconds` waits longer.

## Try it yourself

Repeat a greeting five times, mixing both languages:

```nme
repeat 5 times
show Hello!
반가워요! 말해줘
end
```

## What you learned

- `repeat 3 times and show Again` prints one line three times.
- `3 times Welcome to NME` / `3번 안녕하세요` repeat the rest of the line.
- An indented block repeats every indented line.
- `repeat 3 times` ... `end` closes a flat block without indentation.
- `for each friend in friends` walks through a list, one value per round.
- `wait 1 second` pauses right there.
