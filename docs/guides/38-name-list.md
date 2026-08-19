# 38 — Name list: reading a file line by line

English | [한국어](38-name-list.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [37 — Files](37-files.md)
- Topic: files
- Result: a program that saves names one per line and reads them back as a list

[Guide 37](37-files.md) saved one piece of text. When there are several names,
it is easier to keep **one per line** and turn them back into a list on the way
in.

> A program that uses files does not run on the site. Try this on your own
> computer.

## Steps

1. **Join the list into one piece of text.** A newline between them puts one on
   each line:

   ```nme
   set friends to list of Mina, Ada, Grace
   set text to friends joined by newline
   show text
   ```

2. **Save that text to a file:**

   ```nme
   set friends to list of Mina, Ada, Grace
   set text to friends joined by newline
   write text to "names.txt"
   ```

3. **Read it back and turn it into a list again.** `split by line` does that:

   ```nme
   read "names.txt" into memo
   set names to memo split by line
   show how many names
   ```

4. **You can walk it knowing which one you are on:**

   ```nme
   set names to list of Mina, Ada
   for each name in names with place
       show place
       show name
   end
   ```

   `place` counts from 1.

5. All of it:

   ```nme
   set friends to list of Mina, Ada, Grace
   set text to friends joined by newline
   write text to "names.txt"
   read "names.txt" into memo
   set names to memo split by line
   show how many names
   for each name in names with place
       show place
       show name
   end
   ```

   `joined` and `split` are opposites. A list becomes text and the text becomes
   a list again, with a file in between.

## Try it yourself

Ask for names, put each in a list, and save the file when you are done. Next
time, read that file first and add to it, so the list grows.

## What you learned

- `<list> joined by newline` makes text with one item per line.
- `<text> split by line` turns it back into a list.
- A file between the two makes the list outlive the program.
- `with place` gives you which one you are on, counting from 1.
