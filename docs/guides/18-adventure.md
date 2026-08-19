# 18 — Adventure — a small text game

English | [한국어](18-adventure.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [11 — Break](11-break.md), [12 — Random](12-random.md)
- Topic: a large project
- Result: a room-by-room text adventure with choices

A text adventure describes a place and asks what you do next. Everything from
Part 1 meets here — asking, conditions, an endless loop, a list, and chance.
**There is not one quote, bracket or equals sign in it.**

## Steps

1. Start with the **loop that does not end** and a small menu:

   ```nme
   repeat forever
       ask action What now? look, quit
       if action equals quit
           show You leave the cave
           stop
       else if action equals look
           show Damp stone, and a tunnel leading east
       else
           show I did not understand that
       end
   end
   ```

   `repeat forever` never stops by itself. The only way out is `stop`, so
   leaving the game means answering `quit`.

2. **What you are carrying is a list** — the same list you made in
   [05 — Set](05-set.md):

   ```nme
   set things to an empty list
   append rope to things
   show how many things
   if things contains rope
       show You have a rope
   end
   ```

3. **Chance decides the risky room.** A roll above 3 finds the rope:

   ```nme
   set roll to random number from 1 to 6
   show roll
   if roll is greater than 3
       show You found a rope
   else
       show Nothing here
   end
   ```

4. **Winning is decided by what you carry.** With the rope the door opens;
   without it, it stays locked:

   ```nme
   set things to an empty list
   if things contains rope
       show The door opens
   else
       show The door is locked
   end
   ```

5. Put the four together and it is a game. Save the whole thing:

   ```nme
   set things to an empty list
   show You wake in a dark cave
   repeat forever
       ask action What now? look, east, dice, quit
       if action equals quit
           show You leave the cave
           stop
       else if action equals look
           show Damp stone, and a tunnel leading east
       else if action equals dice
           set roll to random number from 1 to 6
           show roll
           if roll is greater than 3
               show You found a rope
               append rope to things
           else
               show Nothing here
           end
       else if action equals east
           show You step into a stone room
           if things contains rope
               show The door opens
               stop
           else
               show The door is locked
           end
       else
           show I did not understand that
       end
   end
   ```

   Answer `look`, then `dice` until the rope turns up, then `east`, and you
   win. The die is random, so how many rolls it takes changes every time.

## Try it yourself

Add another room. Take a `west` answer that leads to a riverbank, roll there
for a `coin`, and then make the door need **both** the rope and the coin — the
`and` from [09 — And / Or](09-and-or.md) joins the two checks.

## What you learned

- `repeat forever` is a loop with no end, and `stop` is the only way out.
- An `if … else if …` chain over an answer is what a menu really is.
- A list is your inventory: `append` puts things in, `contains` asks.
- A random number gives every run a different story.
- The whole game has no quotes, no brackets and no equals sign.
