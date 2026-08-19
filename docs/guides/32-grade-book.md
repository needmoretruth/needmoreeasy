# 32 — Grade book: a score per name, and an average

English | [한국어](32-grade-book.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [30 — Shop](30-shop.md), [31 — Bank](31-bank.md)
- Topic: projects
- Result: a grade book holding a score per name, reporting the average and the best

A grade book is **one score under each name**. But working out an average
needs the scores on their own — a record has no order and no total. So the
record is walked and the scores are copied into a list.

## Steps

1. **Put the names and scores in a record:**

   ```nme
   set marks to an empty record
   put Mina at 88 in marks
   put Sana at 92 in marks
   show Mina in marks
   ```

   You get `88`.

2. **Walk it and collect the scores into a list.** Only a list can be asked
   for a total and a count:

   ```nme
   set marks to an empty record
   put Mina at 88 in marks
   put Sana at 92 in marks
   set scores to an empty list
   for each name in marks
       set score to name in marks
       append score to scores
   end
   show how many scores
   show the total of scores
   ```

   `2` and `180`.

3. **The average is the total divided by the number of people.** Put the
   divisor in a name first:

   ```nme
   set scores to list of 88, 92
   set people to how many scores
   set average to the total of scores
   divide average by people
   show average
   ```

   You get `90.0`. **What you divide by has to be a name or a number** —
   `divide average by how many scores` is not a thing, which is why `people`
   holds it first.

4. **The best score the list answers directly:**

   ```nme
   set scores to list of 88, 92, 75
   show the biggest of scores
   ```

   You get `92`.

5. The whole thing:

   ```nme
   set marks to an empty record
   put Mina at 88 in marks
   put Sana at 92 in marks
   put Ada at 75 in marks
   set scores to an empty list
   for each name in marks
       set score to name in marks
       append score to scores
       show name
       show score
   end
   set people to how many scores
   set average to the total of scores
   divide average by people
   show average
   show the biggest of scores
   ```

## Try it yourself

Pick out everyone above the average — walk the record a second time and show
the name when `if score is greater than average`. It has to be a **second**
walk: during the first one the average does not exist yet.

## What you learned

- A record is for looking up by name; a list is for totals, counts and the biggest.
- Walking a record and copying its values into a list gets you both.
- The average is the total divided by the count, and the divisor must be a name or a number.
- A comparison against an average happens **after** the average is finished.
