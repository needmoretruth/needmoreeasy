# 87 — Screen: clearing, ruling and boxing

English | [한국어](87-screen.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★☆☆☆ (2/5)
- Prerequisites: [86 — Story](86-story.md)
- Topic: arranging the screen
- Result: a program that clears the screen, centres a title and draws a box

Output piles up and the screen gets messy. Four sentences tidy it.

## Steps

1. Make `screen.nme`:

   ```nme
   clear the screen
   ```

   Everything printed before it disappears. Korean is `화면 지워`.

2. Rule a line to separate one part from the next:

   ```nme
   draw a line
   ```

   Korean is `줄 그어`.

3. A centred line looks like a title:

   ```nme
   say in the middle Today's menu
   ```

   Korean is `가운데 말해줘 오늘의 차림표`.

4. Put a box around anything that has to be noticed:

   ```nme
   say in a box Three minutes left
   ```

   Korean is `상자로 말해줘 남은 시간은 3분입니다`. A Korean letter takes two
   columns in a terminal, and the box counts that width so it never comes out
   crooked.

5. Together they make one screen:

   ```nme
   clear the screen
   draw a line
   say in the middle Today's menu
   draw a line
   say in a box Coffee, tea, water
   ```

## Try it

Combine it with a question, and redraw the screen after the answer:

```nme
clear the screen
say in the middle Tell me your name
draw a line
ask name Your name
clear the screen
say in a box Nice to meet you
say in the middle name
```

## What you learned

- `clear the screen` / `화면 지워` empties the screen.
- `draw a line` / `줄 그어` rules one horizontal line.
- `say in the middle …` / `가운데 말해줘 …` centres a line.
- `say in a box …` / `상자로 말해줘 …` draws a box around it.
- Both the box and the centring count Korean letters as two columns wide.
