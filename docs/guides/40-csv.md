# 40 — CSV: several fields on one line

English | [한국어](40-csv.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [38 — Name list](38-name-list.md)
- Topic: files and data
- Result: a program that reads a file of comma-separated fields

[Guide 38](38-name-list.md) put one thing on each line. To put **two or more**
on a line — a name and a count, say — separate them with a comma. A file made
that way is called a CSV.

> A program that uses files does not run on the site.

## Steps

1. **Split one line by comma:**

   ```nme
   set text to apples,3
   set fields to text split by comma
   show the first of fields
   show the last of fields
   ```

   `apples` and `3` come out separately.

2. **Several lines go in a list.** A comma separates items when a list is
   written out, so a line that itself contains a comma is kept in a name first
   and appended:

   ```nme
   set first to apples,3
   set second to grapes,5
   set rows to an empty list
   append first to rows
   append second to rows
   ```

3. **Save it and read it back**, joined by newline and split by line:

   ```nme
   set rows to list of a, b
   set text to rows joined by newline
   write text to "stock.csv"
   read "stock.csv" into memo
   set lines to memo split by line
   ```

4. **Split each line by comma and you have the fields:**

   ```nme
   set lines to list of apples,3
   for each row in lines
       set fields to row split by comma
       show the first of fields
       show the last of fields
   end
   ```

5. All of it:

   ```nme
   set first to apples,3
   set second to grapes,5
   set rows to an empty list
   append first to rows
   append second to rows
   set text to rows joined by newline
   write text to "stock.csv"
   read "stock.csv" into memo
   set lines to memo split by line
   for each row in lines
       set fields to row split by comma
       show the first of fields
       show the last of fields
   end
   ```

   **The comma you split on and the comma you join with are different.**
   Splitting cuts on a bare comma; joining puts a comma and a space (`, `)
   between. Text a person reads wants the space; a file being cut apart does
   not have one.

## Try it yourself

Add a third field — `apples,3,red` — and take the middle one with
`item 2 of fields`.

## What you learned

- `<text> split by comma` turns one line into a list of fields.
- A comma separates items when a list is written out, so a line containing one
  goes into a name first.
- Join by newline to save, split by line to read, split by comma for the fields.
- Splitting uses `,`; joining uses `, `.
