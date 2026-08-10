# 01 — Hello: say your first words

English | [한국어](01-hello.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★☆☆☆☆ (1/5)
- 선수 지식 (Prerequisites): none
- 주제 (Topic): 첫 프로그램과 출력 / first program and output
- 결과물 (Result): `nme run`으로 메시지를 출력하는 프로그램 / a program that prints a message with `nme run`

Your first program. NME prints whatever follows `show` — no quotes, no
parentheses.

## Steps

1. Create a file named `hello.nme` in an empty folder and write:

   ```text
   show Hello world!
   ```

2. Run it:

   ```sh
   nme run hello
   ```

   The console prints `Hello world!`. `show` is the action; the rest is the
   message.

3. The same idea works in Korean, and English and Korean can share one file.
   Replace the file with:

   ```text
   안녕하세요! 말해줘
   ```

   `말해줘` is the Korean action for `show`.

4. NME also understands plain speech without an action word when the line is a
   natural sentence:

   ```text
   Hello everyone!
   오늘도 반가워요!
   ```

## Try it yourself

Change the message to your own name or a favorite place, save, and run
`nme r hello` again. The shortcut `nme r` is the same as `nme run`.

## What you learned

- `nme run hello` runs `hello.nme`; `nme r hello` is the shortcut.
- `show message` prints the rest of the line; `말해줘` is the Korean action.
- No quotes, commas, or parentheses are needed for a sentence.
- A natural one-line sentence prints by itself.
