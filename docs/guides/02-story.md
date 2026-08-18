# 02 — Story: writing many lines without repeating yourself

English | [한국어](02-story.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★☆☆☆☆ (1/5)
- Prerequisites: [01 — Hello](01-hello.md)
- Topic: story blocks
- Result: a program that tells a story of several lines in one block

In guide 01 every line began with `say`. That is fine for a line or two, but a
story means writing the same word again and again. A `story:` block puts every
line inside it on the screen exactly as you wrote it.

## Steps

1. Type this into the writing box:

   ```nme
   story:
       The door opened slowly.
       The room was empty.
       One letter lay on the table.
   end
   ```

   `story:` opens the block and `end` closes it. Indent the lines inside by four
   spaces. Korean is `이야기:` … `끝`:

   ```nme
   이야기:
       문이 천천히 열렸습니다.
       방 안은 비어 있었습니다.
       탁자 위에 편지가 한 장 놓여 있었습니다.
   끝
   ```

2. A blank line inside the block is a blank line on the screen, so paragraphs
   need nothing special:

   ```nme
   story:
       It was the first day.

       Then the second day came.
   end
   ```

3. To have the letters arrive one at a time, put `slow` in front. It is what
   [guide 03](03-slow-story.md) does to a single line, done to a whole block:

   ```nme
   slow story:
       The wind knocked at the window.
       Nobody answered.
   end
   ```

4. Set the speed yourself by naming it:

   ```nme
   slow story every 0.15 seconds:
       One.
       Two.
       Three.
   end
   ```

   Korean is `천천히 이야기:`, `아주 천천히 이야기:`, `0.15초씩 천천히 이야기:`.

5. **Everything inside the block is text.** Nothing in it is a command. If you
   write `wait 3 seconds` inside a story, the words *wait 3 seconds* appear —
   the program does not wait. For a story that is the right answer, because a
   character may well say it. To really wait, close the block with `end` and
   write it outside:

   ```nme
   story:
       The night was late.
   end
   wait 1 second
   story:
       Then someone knocked.
   end
   ```

## Try it

Write a five-line story. Let the first three lines appear at normal speed and
the last two arrive letter by letter.

```nme
story:
    It was a very old library.
    The books stood at every thickness.
    On the bottom shelf one lay flat.
end
slow story:
    That book had no title.
    Its first line read like this.
end
```

## What you learned

- `story:` … `end` puts every line inside it on the screen as written.
- `slow story:` sends the letters out one at a time.
- `slow story every 0.15 seconds:` sets the speed yourself.
- Korean is `이야기:` · `천천히 이야기:` · `0.15초씩 천천히 이야기:`.
- Nothing inside the block is a command. It is all text.
