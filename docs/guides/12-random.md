# 12 — Random: dice and picks

English | [한국어](12-random.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★☆☆☆ (2/5)
- Prerequisites: [11 — Break](11-break.md)
- Topic: random and chance
- Result: a program that rolls a die and picks a color

A game needs a surprise. Rolling a die, or picking one of several things, is a
single sentence. There is nothing to load first and nothing to switch on.

## Steps

1. Type this into the writing box:

   ```nme
   set die to random number from 1 to 6
   show die
   ```

   Run it a few times. Any number from 1 to 6 comes out. Korean is
   `주사위는 1부터 6까지 무작위 숫자`.

2. Picking one of the things you name:

   ```nme
   set color to pick from red or green or blue
   show color
   ```

   Korean is `색은 빨강 또는 초록 또는 파랑 중에서 골라`.

3. Mix it with what you already know and it is a game:

   ```nme
   set answer to random number from 1 to 3
   ask number guess Pick 1, 2 or 3
   if guess equals answer
       show Correct!
   else
       show Not this time
       show answer
   end
   ```

4. Roll several times and keep them:

   ```nme
   set rolls to list of
   repeat 3 times
   set one to random number from 1 to 6
   append one to rolls
   end
   show rolls
   ```

5. (**Skip this for now.**) In beginner syntax the same jobs have helper names.
   You load them with `use random` and then write `random_number(1, 6)`. Written
   as sentences, that line is not needed.

## Try it yourself

Roll the die twice and show both:

```nme
set first to random number from 1 to 6
set second to random number from 1 to 6
show first
show second
```

## What you learned

- `random number from 1 to 6` / `1부터 6까지 무작위 숫자` rolls a die.
- `pick from red or green or blue` / `... 중에서 골라` picks one of several.
- Written as sentences, nothing has to be loaded first.
- Chance mixes straight into conditions, loops and lists — mix them and you
  have a game.
- Beginner syntax calls the same jobs by name, after `use random`. That comes
  later, in [61 — Modules](61-modules.md).
