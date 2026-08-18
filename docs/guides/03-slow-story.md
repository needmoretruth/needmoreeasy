# 03 — Story: letters arriving one at a time

English | [한국어](03-slow-story.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★☆☆☆☆ (1/5)
- Prerequisites: [02 — Story](02-story.md)
- Topic: stories and slow output
- Result: a program whose text appears one letter at a time, the way a novel unfolds

`show` puts a whole line on screen at once. When you are telling a story, letters
arriving one at a time read better. `say slowly` does that.

## Steps

1. Make `story.nme`:

   ```nme
   say slowly The door opened slowly.
   ```

   The letters arrive one by one. Korean is the same:

   ```nme
   천천히 말해줘 문이 천천히 열렸습니다.
   ```

2. For something slower still, add `very`:

   ```nme
   say very slowly Nobody was there.
   ```

   Korean is `아주 천천히 말해줘 아무도 없었습니다.`

3. You can set the speed yourself:

   ```nme
   say slowly every 0.2 seconds One. Two. Three.
   ```

   Korean is `0.2초씩 천천히 말해줘 하나. 둘. 셋.`

4. A story is usually several lines. `wait` between them lets it breathe:

   ```nme
   say slowly The night was late.
   wait 1 second
   say slowly Then someone knocked.
   ```

5. Come back to this step after [04 — Ask](04-ask.md) and [08 — If](08-if.md).
   Mixing the two lets the reader choose the story:

   ```nme
   say slowly Do you open the door?
   ask answer yes or no
   if answer equals yes
       say slowly Outside, snow was falling.
   else
       say slowly The footsteps faded away.
   end
   ```

## Try it

Write a three-line story: the first line at normal speed, the middle line slowly,
the last line very slowly.

```nme
show It was a winter night
say slowly Something moved outside the window.
say very slowly It had waited a very long time.
```

## What you learned

- `say slowly …` sends the letters out one at a time.
- `say very slowly …` does the same, slower.
- `say slowly every 0.2 seconds …` sets the speed yourself.
- Korean is `천천히 말해줘`, `아주 천천히 말해줘`, `0.2초씩 천천히 말해줘`.
- `wait 1 second` between lines lets a story breathe.
