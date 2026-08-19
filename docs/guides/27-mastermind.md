# 27 — Game: guessing a hidden set of colours

English | [한국어](27-mastermind.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [23 — High score](23-high-score.md)
- Topic: games
- Result: guessing the three colours the computer hid, in five tries

The computer hides three colours and you name three. It tells you **only how
many you got**, never which ones, so finding them means changing one answer at a
time and watching the number move.

## Steps

1. **Choose the hidden colours.** Shuffle the list and take the first of it,
   three times over:

   ```nme
   set colors to list of red, blue, green, yellow
   set answer to an empty list
   repeat 3 times
       shuffle colors
       append the first of colors to answer
   end
   show answer joined by comma
   ```

   Printing the answer is how you check the program while building it. Delete
   that last line once it works.

2. **One try** takes three colours, keeps them in a list, and counts how many
   of them are in the hidden set:

   ```nme
   set answer to list of red, blue, green
   set right to 0
   set guess to an empty list
   repeat 3 times
       ask choice What colour?
       append choice to guess
       if answer contains choice
           add 1 to right
       end
   end
   show guess joined by comma
   show right
   ```

3. **Winning** is that one count:

   ```nme
   set right to 3
   if right equals 3
       show you found them all
   end
   ```

4. **Count the tries down**, and show the answer when they run out:

   ```nme
   set lives to 5
   subtract 1 from lives
   if lives equals 0
       show out of tries
   end
   ```

5. All of it:

   ```nme
   set colors to list of red, blue, green, yellow
   set answer to an empty list
   repeat 3 times
       shuffle colors
       append the first of colors to answer
   end
   set lives to 5
   repeat forever
       set right to 0
       set guess to an empty list
       repeat 3 times
           ask choice What colour?
           append choice to guess
           if answer contains choice
               add 1 to right
           end
       end
       show guess joined by comma
       show how many of them are hidden
       show right
       if right equals 3
           show you found them all
           stop
       end
       subtract 1 from lives
       if lives equals 0
           show out of tries
           show answer joined by comma
           stop
       end
   end
   ```

   **Position is not checked.** It only counts whether a colour is in there, so
   naming the same colour twice counts twice. Real Mastermind also says how many
   are in the right place, and that needs a loop that knows which position it is
   at — something sentence syntax does not have yet.

## Try it yourself

Use five colours and six tries. Count for yourself which of the two changes
makes it harder.

## What you learned

- Shuffling and taking the first, several times over, makes a random set.
- A value taken out of one list goes straight into another.
- "How many did I get" is one counting name and one `contains`.
- The whole game has no quotes, no brackets and no equals sign.
