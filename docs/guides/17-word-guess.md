# 17 — Word guess: a hidden-word game

English | [한국어](17-word-guess.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [16 — Check & Build](16-check-build.md)
- Topic: your first project
- Result: a game of guessing a hidden word one letter at a time

This is where Part 2 begins. Everything from Part 1 comes together into **a game
of guessing a hidden word in six tries**. Lists, chance, loops and conditions
all appear, and **there is not one quote, bracket or equals sign in it.**

## Steps

1. **Pick the hidden word at random.** Shuffle a list and take the first of it:

   ```nme
   set words to list of apple, grape, melon
   shuffle words
   set secret to the first of words
   ```

2. **A word can be walked through letter by letter.** `for each letter in`
   hands them over one at a time:

   ```nme
   set secret to apple
   for each letter in secret
       show letter
   end
   ```

3. **Build the masked word.** A letter already guessed stays; one still hidden
   becomes `_`. Collect them in a list and join it with spaces:

   ```nme
   set secret to apple
   set found to list of a
   set shown to an empty list
   for each letter in secret
       if found contains letter
           append letter to shown
       else
           append _ to shown
       end
   end
   show shown joined by space
   ```

   `apple` is a, p, p, l, e, and only `a` has been found, so it prints
   `a _ _ _ _`. Change the word and watch the mask follow it.

4. **Winning is one question.** If nothing is masked any more, it is all found:

   ```nme
   set shown to list of a, p
   if shown does not contain _
       show you got it
   end
   ```

5. Here is the whole game. Put it in `word-guess.nme`:

   ```nme
   set words to list of apple, grape, melon
   shuffle words
   set secret to the first of words
   set found to an empty list
   set lives to 6
   repeat forever
       set shown to an empty list
       for each letter in secret
           if found contains letter
               append letter to shown
           else
               append _ to shown
           end
       end
       show shown joined by space
       if shown does not contain _
           show you got it
           stop
       end
       if lives equals 0
           show out of guesses
           show secret
           stop
       end
       show lives
       ask letter Guess a letter
       if secret contains letter
           show yes
       else
           show no
           subtract 1 from lives
       end
       append letter to found
   end
   ```

## Try it yourself

Add more words to the list. Then stop a repeated guess from costing a life —
one `if found contains letter` does it.

## What you learned

- Shuffling a list and taking the first of it is a random pick.
- `for each letter in` walks a word letter by letter.
- The masked word is **a list you build and then join**.
- `does not contain` asks whether you have won, in one line.
- The whole game has no quotes, no brackets and no equals sign.
