# 29 — Playlist: shuffling the order once

English | [한국어](29-playlist.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [12 — Random](12-random.md), [21 — Progress](21-progress.md)
- Topic: random and chance
- Result: shuffling a list of songs and playing through it one at a time

Shuffle in a music app does not keep picking any song at random — that would
play the same song twice in a row. It **mixes the order once and then plays
through it**. One list of songs is all you need.

## Steps

1. **Make the list of songs:**

   ```nme
   set songs to list of Blue, Green, Red, Gold
   show songs joined by comma
   ```

   They come out in the order you wrote them.

2. **`shuffle` mixes the order.** It goes in the same place as `sort` from
   [guide 52](52-sorting.md), and it changes the list itself:

   ```nme
   set songs to list of Blue, Green, Red, Gold
   shuffle songs
   show songs joined by comma
   ```

   A different order every run. **No song is lost or repeated** — it is the
   same four songs in another order.

3. **Play through the shuffled order.** `with place` from
   [guide 21](21-progress.md) says which song you are on:

   ```nme
   set songs to list of Blue, Green, Red, Gold
   shuffle songs
   for each song in songs with place
       show place
       show song
   end
   ```

4. **Wait while a song plays.** A pause stands in for the real playing:

   ```nme
   set songs to list of Blue, Green
   for each song in songs
       show song
       wait 0.5 seconds
   end
   ```

5. The whole thing:

   ```nme
   set songs to list of Blue, Green, Red, Gold
   shuffle songs
   show todays running order
   for each song in songs with place
       clear the screen
       show place
       show song
       wait 0.5 seconds
   end
   show that was all of them
   ```

## Try it yourself

Delete `shuffle songs` and run it again — the order never changes. Put
`sort songs` there instead and it plays alphabetically. All three are one
sentence in the same place.

## What you learned

- `shuffle <list>` mixes the order at random, losing and repeating nothing.
- Shuffling **once** and playing through is what an app actually does.
- `shuffle`, `sort` and `reverse` are the same kind of sentence: they change the list itself.
- `with place` lets you show which song is playing.
