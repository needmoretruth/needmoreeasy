# 30 — Shop: stock and a balance

English | [한국어](30-shop.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [24 — Quiz](24-quiz.md), [25 — Calculator](25-calculator.md)
- Topic: projects
- Result: a small shop where every sale takes one off the stock and ten off the money

A shop program has two things to remember — **how many of each thing are
left**, and **how much money there is**. The first is one number under each
name, which is the shape of a [record](41-address-book.md); the second is one
number.

## Steps

1. **Keep the stock in a record.** The thing's name is the name, how many are
   left is the value:

   ```nme
   set stock to an empty record
   put apple at 5 in stock
   put bread at 2 in stock
   show apple in stock
   ```

   You get `5`.

2. **Asking for something that is not there stops the program**, so ask
   whether it is there first:

   ```nme
   set stock to an empty record
   put apple at 5 in stock
   if stock contains milk
       show milk in stock
   else
       show we do not have that
   end
   ```

3. **Lowering a number in a record takes three steps** — take it out, change
   it, put it back:

   ```nme
   set stock to an empty record
   put apple at 5 in stock
   set left to apple in stock
   subtract 1 from left
   put apple at left in stock
   show apple in stock
   ```

   You get `4`. There is no sentence yet for changing a value inside a record
   where it sits, and these three lines stand in for it.

4. **Nothing may be sold when none are left**, so look at the count before
   selling:

   ```nme
   set left to 0
   if left is greater than 0
       show selling one
   else
       show that one is gone
   end
   ```

5. The whole thing. One sale takes one off the stock and ten off the money:

   ```nme
   set stock to an empty record
   put apple at 5 in stock
   put bread at 2 in stock
   set money to 100
   ask item what would you like
   if stock contains item
       set left to item in stock
       if left is greater than 0
           subtract 1 from left
           put item at left in stock
           subtract 10 from money
           show sold
           show item
           show left
       else
           show that one is gone
       end
   else
       show we do not have that
   end
   show money
   ```

## Try it yourself

Give each thing its own price — a second record called `prices`, keyed by the
same names as `stock`. Then write the other direction: putting stock back in
is the same shape with `add` where `subtract` is.

## What you learned

- A record is the right shape for one number under each name.
- Reading a name that is not there stops the program, so **ask first**.
- Changing a number inside a record is take-out, change, put-back.
- Looking at the count before selling is what keeps it from going below zero.
