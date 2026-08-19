# 51 — Text: counting, splitting, joining again

English | [한국어](51-strings.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [05 — Set](05-set.md), [40 — CSV](40-csv.md)
- Topic: lists and text
- Result: counting the letters in a sentence, splitting it into words, and joining it back into one line

Text is what programs handle most. What a person types is text and what comes
out of a file is text. There are three things to do with it — **count it**,
**split it**, **join it**.

## Steps

1. **The number of letters is its `length`.** Spaces count too:

   ```nme
   set line to the weather is very good today
   show the length of line
   ```

   You get `30`.

2. **Splitting by space gives a list of words.** It is the same sentence as
   splitting by comma in [guide 40](40-csv.md):

   ```nme
   set line to the weather is very good today
   set words to line split by space
   show how many words
   show the first of words
   show the last of words
   ```

   `6`, `the`, `today`. **What comes out is a list**, so every list sentence
   works on it straight away.

3. **It joins back into one line too:**

   ```nme
   set words to list of the, weather, is, good
   show words joined by comma
   show words joined by space
   ```

   `the, weather, is, good` and `the weather is good`.

4. **One star per word is a way to see the length.** Put the count in a name
   first:

   ```nme
   set words to list of the, weather, is, good
   set count to how many words
   set star to *
   set bar to star repeated count times
   show bar
   ```

   You get `****`.

5. **Letters can be turned into capitals or small letters:**

   ```nme
   set line to the weather is very good today
   show line in capitals
   show line in small letters
   ```

   `THE WEATHER IS VERY GOOD TODAY`, then the line as it was.

6. The whole thing:

   ```nme
   set line to the weather is very good today
   show the length of line
   set words to line split by space
   show how many words
   show the first of words
   show the last of words
   show words joined by comma
   set count to how many words
   set star to *
   set bar to star repeated count times
   show bar
   ```

## Try it yourself

Split by comma instead of by space — there are no commas in the line, so the
whole thing comes back as one piece. What you split on has to actually be in
the text. Then `sort` the words you split out and you have them alphabetically.

## What you learned

- `the length of <text>` is how many letters it has, spaces included.
- `<text> split by space` makes a list of words, and it is a list like any other.
- `<list> joined by <separator>` puts it back into one line.
- `in capitals` and `in small letters` change letters that have two cases.
