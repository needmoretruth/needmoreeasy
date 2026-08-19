# 42 — Word count: how often each word appears

English | [한국어](42-word-count.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [40 — CSV](40-csv.md), [41 — Records](41-address-book.md)
- Topic: lists and text
- Result: splitting text into words and counting how many times each one appears

"How often each word appears" is **one number under each name**, which makes it
a [record](41-address-book.md). There is only one hard part: a word seen for
the first time and a word seen again have to be handled differently.

## Steps

1. **Split the text into words:**

   ```nme
   set text to apple pear apple plum
   set words to text split by space
   show how many words
   ```

   You get `4`. A word that appears twice is in the list twice.

2. **A word seen for the first time starts at one:**

   ```nme
   set counts to an empty record
   put apple at 1 in counts
   show apple in counts
   ```

3. **A word seen again is taken out, raised, and put back.** Three steps:

   ```nme
   set counts to an empty record
   put apple at 1 in counts
   set seen to apple in counts
   add 1 to seen
   put apple at seen in counts
   show apple in counts
   ```

   You get `2`.

4. **What tells the two apart is whether it is already there:**

   ```nme
   set counts to an empty record
   set word to apple
   if counts contains word
       show this is not the first time
   else
       show this is the first time
   end
   ```

   `this is the first time`.

5. The whole thing — one word at a time, one of the two things each time:

   ```nme
   set text to apple pear apple plum pear apple
   set words to text split by space
   set counts to an empty record
   for each word in words
       if counts contains word
           set seen to word in counts
           add 1 to seen
           put word at seen in counts
       else
           put word at 1 in counts
       end
   end
   for each word in counts
       show word
       show word in counts
   end
   show how many counts
   ```

   `apple 3`, `pear 2`, `plum 1`, and the final `3` is **how many different
   words there were**.

## Try it yourself

The same word in capitals is counted separately. Putting `word in small
letters` into a name and counting that instead brings them together. Then show
only the words seen more than once by testing the count in the last loop.

## What you learned

- One number under each name is a record.
- `contains` tells a first sighting from a repeat.
- Raising a number inside a record is take-out, add, put-back.
- `how many` on a record is the number of **different names**, not of items counted.
