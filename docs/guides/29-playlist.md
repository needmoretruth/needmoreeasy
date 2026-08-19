# 29 — Playlist: a random music player

English | [한국어](29-playlist.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [12 — Random](12-random.md), [24 — Quiz](24-quiz.md)
- Topic: random and chance
- Result: a playlist loaded from JSON with shuffle, next, and a loop of songs

A music player walks a list of songs — load it from JSON, mix it with the random helper, and loop a menu until you quit.

## Steps

1. Save five songs in `songs.json` — a JSON list of dicts with `title` and
   `artist`:

   ```nme
   [
     {"title": "Hello", "artist": "Adele"},
     {"title": "Dynamite", "artist": "BTS"},
     {"title": "Love Dive", "artist": "IVE"},
     {"title": "Life Goes On", "artist": "BTS"},
     {"title": "Butter", "artist": "BTS"}
   ]
   ```

2. Load the list with `json_load`, mix it with `shuffle`, and jump around with
   `random_pick`. Save the whole player as `playlist.nme`:

   ```nme
   # playlist.nme — a random music player.
   # Run: nme r playlist

   use random latest
   use file latest

   songs = json_load("songs.json")
   shuffle(songs)

   current = 0

   show f"Playlist loaded: {len(songs)} songs"

   while True:
       show ""
       show "Commands: next, prev, list, quit"
       ask command, "? "
       if command == "next":
           current = current + 1
           if current >= len(songs):
               current = 0
           song = songs[current]
           show f"Now playing: {song['title']} by {song['artist']}"
       elif command == "prev":
           current = current - 1
           if current < 0:
               current = len(songs) - 1
           song = songs[current]
           show f"Now playing: {song['title']} by {song['artist']}"
       elif command == "list":
           show f"Playlist ({len(songs)} songs):"
           for i in range(len(songs)):
               mark = "> " if i == current else "  "
               show f"{mark}{i + 1}. {songs[i]['title']} by {songs[i]['artist']}"
       elif command == "quit":
           show "Bye!"
           break
   ```

3. Run it and feed the commands through a pipe:

   ```sh
   printf 'next\nnext\nlist\nquit\n' | nme r playlist
   ```

   ```text
   Playlist loaded: 5 songs

   Commands: next, prev, list, quit
   ? Now playing: Butter by BTS

   Commands: next, prev, list, quit
   ? Now playing: Hello by Adele

   Commands: next, prev, list, quit
   ? Playlist (5 songs):
     1. Life Goes On by BTS
     2. Butter by BTS
   > 3. Hello by Adele
     4. Love Dive by IVE
     5. Dynamite by BTS

   Commands: next, prev, list, quit
   ? Bye!
   ```

   `next` wraps past the end and `prev` wraps to the last song; `shuffle` mixes the order.

## Try it yourself

Add a `count` command that asks for an artist and prints how many of their songs are in the playlist — loop over `songs` and grow a counter.

## What you learned

- `use random latest` and `use file latest` load both helpers together.
- `json_load("songs.json")` reads the song list; `shuffle(songs)` mixes it; `random_pick(songs)` jumps around.
- `songs[current]` reads one song; wrapping the index loops the playlist.
- A `while True` menu with `ask`, `show`, and `break` drives the player.
