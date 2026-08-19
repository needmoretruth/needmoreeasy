# 23 — High score: a small project

English | [한국어](23-high-score.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [17 — Word guess](17-word-guess.md)
- Topic: a project
- Result: a dice game of three rounds that tells you your best

Three rounds, a score kept for each, and the best of them at the end. **A loop
inside a loop** appears here for the first time. No quotes, no brackets, no
equals sign in any of it.

## Steps

1. **One round** is five rolls, a point for every roll above three:

   ```nme
   set score to 0
   repeat 5 times
       set die to random number from 1 to 6
       if die is greater than 3
           add 1 to score
       end
   end
   show score
   ```

2. **To play three rounds**, wrap that in another loop. A loop inside a loop is
   nothing new — the inner one is simply indented further:

   ```nme
   repeat 3 times
       set score to 0
       repeat 5 times
           set die to random number from 1 to 6
           if die is greater than 3
               add 1 to score
           end
       end
       show score
   end
   ```

   `set score to 0` sitting **before** the inner loop is what matters: each
   round has to start again from nothing.

3. **Collect the scores.** At the end of a round, put it in a list:

   ```nme
   set scores to an empty list
   append 3 to scores
   append 1 to scores
   show scores joined by comma
   show the biggest of scores
   ```

4. **To use the best in a condition**, keep it in a name first:

   ```nme
   set scores to list of 3, 1, 2
   set best to the biggest of scores
   if best is greater than 4
       show very good
   else
       show there is room to improve
   end
   ```

5. All of it:

   ```nme
   ask name What is your name?
   show Hello name!
   set scores to an empty list
   repeat 3 times
       set score to 0
       repeat 5 times
           set die to random number from 1 to 6
           if die is greater than 3
               add 1 to score
           end
       end
       show one round done
       show score
       append score to scores
   end
   draw a line
   show your best
   show the biggest of scores
   show scores joined by comma
   set best to the biggest of scores
   if best is greater than 4
       show very good
   else
       show there is room to improve
   end
   ```

   End the program and the scores are gone. **Keeping them for next time**
   means writing them to a file, and that is [37 — Files](37-files.md).

## Try it yourself

Play five rounds instead of three, and show the lowest as well as the best
(`the smallest of scores`). Every round's score is already in the list.

## What you learned

- A loop can sit inside a loop; the inner one is indented further.
- A value that must reset each round is set to nought **before** the inner loop.
- With the scores in a list you can ask for the biggest, the smallest or the total.
- To use a value taken from a list in a condition, keep it in a name first.
- Everything is gone when the program ends. Keeping it needs a file.
