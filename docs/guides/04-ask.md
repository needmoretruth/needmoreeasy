# 04 — Ask: have a conversation

English | [한국어](04-ask.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★☆☆☆☆ (1/5)
- Prerequisites: [03 — Story](03-slow-story.md)
- Topic: input
- Result: a program that asks a name and greets it in a sentence

A program that listens. `ask` reads what the user types and stores it in a
name.

## Steps

1. Create `greet.nme`:

   ```nme
   ask name What is your name?
   show Hello name!
   ```

2. Run it and type a name when the program waits:

   ```sh
   nme run greet
   ```

   NME inserts the stored answer into the second sentence, so typing `Mina`
   prints `Hello Mina!`.

3. The same conversation works in Korean:

   ```nme
   이름을 물어봐 이름이 뭐예요?
   안녕하세요 이름! 말해줘
   ```

4. For the gentlest first input, an ordinary question is enough — NME infers
   the name from the question:

   ```nme
   What is your name?
   Hello name!
   ```

## Try it yourself

Ask for a city and show it in a sentence:

```nme
ask city Which city do you live in?
show I love city!
```

## What you learned

- `ask name question` stores the typed answer as `name`; `물어봐` is the Korean
  action.
- A known name is inserted into a later sentence automatically.
- An ordinary question like `What is your name?` also creates the name.
- `show Hello name!` prints `Hello` plus the stored value.
