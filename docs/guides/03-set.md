# 03 — Set: store a value

English | [한국어](03-set.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★☆☆☆☆ (1/5)
- Prerequisites: [02 — Ask](02-ask.md)
- Topic: storing values
- Result: a program that keeps text or numbers in named values

A value is a box with a name. `set` puts something in the box, and `show`
takes it out later.

## Steps

1. Create `store.nme`:

   ```nme
   set greeting to Hello
   show greeting
   ```

   This prints `Hello`. The name `greeting` now stands for that text.

2. Numbers work the same way:

   ```nme
   set answer to 7
   set answer to 10
   show answer
   ```

   `answer` is a box: putting `10` in replaces `7`, so the program prints
   `10`.

3. Korean puts the name first:

   ```nme
   인사는 안녕하세요
   정답은 7
   ```

   `인사는 ...` and `정답은 ...` store text and numbers exactly like the
   English forms.

4. The action word can come after the name too:

   ```nme
   name save Mina
   이름 저장 민수
   ```

## Try it yourself

Store your age as a number and a favorite place as text, then show both:

```nme
set age to 12
set place to Seoul
show place
```

## What you learned

- `set name to value` stores a value; `이름은 값` is the Korean form.
- Putting a new value in a name replaces the old one.
- Text and numbers are both stored without quotes.
- `name save value` / `이름 저장 값` are accepted word orders.
