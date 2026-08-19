# 41 — Records: a small address book

English | [한국어](41-address-book.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [38 — Name list](38-name-list.md)
- Topic: working with data
- Result: an address book that finds where someone lives by their name

A list holds values **in order**. But what you want from an address book is
not "the third person" — it is **"where Mina lives"**. That is what a
**record** is for: every value in it has a name of its own.

## Steps

1. **Make a record and put something in it.** Putting takes a name and a
   value together:

   ```nme
   set book to an empty record
   put Mina at Seoul in book
   put Sana at Busan in book
   ```

   It is the same word as a list's `put`; when the name holds a record it
   takes the two things instead of one.

2. **Read a value by its name.** You never need to know where it is:

   ```nme
   set book to an empty record
   put Mina at Seoul in book
   show Mina in book
   ```

   You get `Seoul`. That is the whole difference between a list and a record.

3. **Asking how many is the same sentence as for a list:**

   ```nme
   set book to an empty record
   put Mina at Seoul in book
   put Sana at Busan in book
   show how many book
   ```

   You get `2`.

4. **Going through a record gives you the names.** Read each value back with
   the name it gave you:

   ```nme
   set book to an empty record
   put Mina at Seoul in book
   put Sana at Busan in book
   for each name in book
       show name
       show name in book
   end
   ```

   `Mina`, `Seoul`, `Sana`, `Busan`.

5. **Asking whether something is there, and taking it out** — both are the
   list sentences again:

   ```nme
   set book to an empty record
   put Mina at Seoul in book
   if book contains Mina
       show Mina in book
   end
   remove Mina from book
   show how many book
   ```

   `Seoul`, then `0`.

6. The whole thing:

   ```nme
   set book to an empty record
   put Mina at Seoul in book
   put Sana at Busan in book
   put Ada at Daegu in book
   show how many book
   for each name in book
       show name
       show name in book
   end
   if book contains Sana
       show found Sana
       show Sana in book
   end
   remove Mina from book
   show how many book
   ```

## Try it yourself

Add `sort book` and run it — it is refused. A record has **no order**: there
is no first and no last, only names to look things up by. Put things you want
in order in a list, and things you want to look up by name in a record.

## What you learned

- `set <name> to an empty record` makes one. Every value in it has a name.
- `put <name> at <value> in <record>` puts one in; `<name> in <record>` reads it back.
- `how many`, `contains`, `for each` and `remove` are the **same sentences** as
  for a list. What the name holds decides what they mean.
- Going through a record gives you the **names**; read each value with its name.
- A record has no order, so `sort` and `the first` are not for it.
