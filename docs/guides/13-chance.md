# 13 — Chance: how often out of a hundred

English | [한국어](13-chance.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★☆☆☆ (2/5)
- Prerequisites: [12 — Random](12-random.md)
- Topic: chance
- Result: a program in which something happens only as often as you decided

The `random number` of [guide 12](12-random.md) rolls a die, and a die gives all
six faces equally often. Chance is different: **you decide how often** something
happens. Thirty times out of a hundred is written `30% chance`.

## Steps

1. Type this into the writing box:

   ```nme
   30% chance say You found a coin in the street
   ```

   Run it ten times and it happens about three times. Some days not at all.
   Korean is:

   ```nme
   30% 확률로 말해줘 길에서 동전을 주웠습니다
   ```

2. To hang several lines on the same chance, write it as a block and close it
   with `end`:

   ```nme
   10% chance
       say The sky darkens.
       say Rain comes down.
   end
   ```

3. A decimal point is allowed to **one** place:

   ```nme
   12.5% chance say This is a rare thing
   ```

   Two places are refused when the program is compiled. `12.25%` is an error,
   and it tells you to write something like `12.3%` instead. It does not quietly
   round, because a program must never decide something its author did not
   write.

   A whole number carries no decimal: `50%` is exactly fifty in a hundred. You
   never need to add `.0`.

4. A chance can be **kept as true or false**, so the same result can be used in
   several places:

   ```nme
   rain is a 40% chance
   if rain
       say Take an umbrella.
   else
       say It is clear today.
   end
   ```

   `rain` is decided once, when the program reaches that line, and does not
   change afterwards. Writing `40% chance` on two separate lines draws twice, so
   the two can disagree. When one decision has to hold everywhere, keep it in a
   name like this.

5. `0%` never happens and `100%` always does. Both are useful while you are
   still building: turn a line on and off without deleting it.

   ```nme
   100% chance say This is only a test
   ```

## Try it

Use a die and a chance together. The die decides *what*, the chance decides *how
often*.

```nme
set die to random number from 1 to 6
say You rolled the die
say die
20% chance say And a second die rolled out with it
```

## What you learned

- `30% chance …` happens thirty times in a hundred. Korean is `30% 확률로 …`.
- Open `30% chance` as a block and close it with `end` to cover several lines.
- One decimal place is allowed; two are refused as an error.
- `name is a 30% chance` decides once and keeps the answer as true or false.
- `0%` is never and `100%` is always.
