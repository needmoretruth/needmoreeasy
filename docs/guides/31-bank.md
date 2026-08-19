# 31 — Bank: paying in, taking out, keeping a history

English | [한국어](31-bank.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [25 — Calculator](25-calculator.md), [05 — Set](05-set.md)
- Topic: projects
- Result: an account that pays in, takes out, protects the balance and records what happened

An account remembers two things — **how much there is** and **what happened**.
The first is one number, the second is a list. And there is one rule: **you
cannot take out more than is there.**

## Steps

1. **Make the balance and the history:**

   ```nme
   set balance to 0
   set history to an empty list
   show balance
   show how many history
   ```

   `0` and `0`.

2. **Paying in is one line.** Write down what happened after it:

   ```nme
   set balance to 0
   set history to an empty list
   add 100 to balance
   append paid in to history
   show balance
   show how many history
   ```

   `100` and `1`.

3. **Look at the balance before taking anything out.** Without that check the
   balance goes below nothing:

   ```nme
   set balance to 50
   set wanted to 80
   if wanted is greater than balance
       show there is not enough
   else
       subtract wanted from balance
   end
   show balance
   ```

   `there is not enough`, then `50`. Nothing happened at all.

4. **The history reads as one line, joined by commas:**

   ```nme
   set history to an empty list
   append paid in to history
   append took out to history
   show history joined by comma
   ```

   `paid in, took out`.

5. The whole thing:

   ```nme
   set balance to 0
   set history to an empty list
   ask number paying how much to pay in
   add paying to balance
   append paid in to history
   ask number wanted how much to take out
   if wanted is greater than balance
       show there is not enough
   else
       subtract wanted from balance
       append took out to history
   end
   show balance
   show history joined by comma
   show how many history
   ```

## Try it yourself

Ask for more than the balance — the balance stays as it was and nothing is
written to the history either. **Refusing means doing nothing at all.** Then
wrap the whole thing in `repeat forever` so you can pay in and out again and
again.

## What you learned

- One number and one list make an account.
- `add` and `subtract` move the balance; `append` builds the history.
- The check before taking out is where the rule lives.
- When a program refuses, it leaves the value alone and writes nothing down.
