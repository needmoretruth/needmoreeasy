# 01 — Hello: say your first words

English | [한국어](01-hello.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★☆☆☆☆ (1/5)
- Prerequisites: none
- Topic: your first program, and output
- Result: a program that puts what you wrote on the screen

This is your first program. Nothing to install, and no knowledge of computers
needed. NME puts whatever follows `show` on the screen. No quotes, no brackets.

## Steps

1. Open [needmoreeasy.com](https://needmoreeasy.com/). There is a writing box.
   Type this into it:

   ```nme
   show Hello world!
   ```

   Press **Run**. `Hello world!` appears below. That is all of it — you have
   just written a program and run it.

   Nothing here can break anything, and what you write never leaves the tab.

2. `show` is the thing to do, and the rest of the line is what to say. Change
   the words:

   ```nme
   show I wrote my first program today
   ```

3. Korean works the same way, with the action at the end of the line instead:

   ```nme
   안녕하세요! 말해줘
   ```

   You may mix the two languages in one file. There is nothing to set.

4. A line that reads as an ordinary sentence needs no action word at all:

   ```nme
   Hello everyone!
   오늘도 반가워요!
   ```

5. Several lines run from top to bottom, in order:

   ```nme
   show This is the first line
   show This is the second line
   ```

## Try it

Change the words to your own name, or a place you like, and press **Run**
again. A three-line introduction of yourself is a good first program.

To keep what you wrote, move it into **File 1** with the button above the
writing box. That file stays in your browser, so opening an example to look at
it will not take your work away.

## If you installed it on your own computer (skip this for now)

Once [install](../install.md) is done, the same program runs from a file. Put
the lines in `hello.nme` and:

```sh
nme run hello
```

`nme r hello` is the short form of the same command. You do not need any of
this while you are learning — the result is the same as on the site.

## What you learned

- Whatever follows `show` appears on the screen; Korean puts `말해줘` at the end.
- Sentences need no quotes, no commas and no brackets.
- A line that reads as an ordinary sentence prints itself.
- Several lines run from top to bottom.
- Everything works on needmoreeasy.com with nothing installed.
