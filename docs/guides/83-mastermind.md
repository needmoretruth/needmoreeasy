# 83 — Game: Mastermind, guessing a secret code

English | [한국어](83-mastermind.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [10 — Random](10-random.md), [44 — Playlist](44-playlist.md)
- Topic: game & logic
- Result: a Mastermind-style game that hides a 4-color code and gives black/white feedback for each guess

The guessing games so far answered only "too big" or "too small".
Mastermind gives two clues at once: how many colors are right **and in
the right spot** (black), and how many colors are right but misplaced
(white). Turning those clues into correct feedback is a small piece of
real algorithm thinking.

## Steps

1. A secret code is four picks from the color list — the `random_pick`
   from guide [10](10-random.md), called four times:

   ```nme
   use random latest

   colors = ["red", "blue", "green", "yellow"]

   secret = []
   for i in range(4):
       secret.append(random_pick(colors))
   ```

   `append` grows the list one color at a time, the same list-building
   loop that made the playlist in guide [44](44-playlist.md).

2. Black counts exact matches — same color, same position. Compare each
   guess color with the secret color:

   ```nme
   black = 0
   for i in range(4):
       if parts[i] == secret[i]:
           black = black + 1
   ```

3. White is trickier: right colors that are in the wrong place. The
   honest way counts every color that appears in both lists, then removes
   the black ones:

   ```nme
   white = 0
   for color in colors:
       white = white + min(parts.count(color), secret.count(color))
   white = white - black
   ```

   `parts.count("red")` counts one list, `secret.count("red")` the other;
   `min` takes the smaller — a color guessed three times cannot match a
   secret that has it once. Subtracting `black` leaves only the misplaced
   ones. This is the standard Mastermind feedback algorithm.

4. The full game asks for guesses, validates them, and stops on four
   blacks. Save `mastermind.nme`:

   ```nme
   # mastermind.nme — guess a 4-color secret code.
   # Run: nme r mastermind

   use random latest

   colors = ["red", "blue", "green", "yellow"]

   secret = []
   for i in range(4):
       secret.append(random_pick(colors))

   turns = 0
   while True:
       ask guess, "guess 4 colors (red blue green yellow): "
       parts = guess.split()
       if len(parts) != 4:
           show "please type exactly 4 colors"
           continue
       turns = turns + 1
       black = 0
       for i in range(4):
           if parts[i] == secret[i]:
               black = black + 1
       white = 0
       for color in colors:
           white = white + min(parts.count(color), secret.count(color))
       white = white - black
       show f"black: {black}  white: {white}"
       if black == 4:
           show f"solved in {turns} turns"
           break
   ```

5. Run it. A scripted round (the three guesses are piped in, then the
   game ends) against a secret of `["yellow", "red", "blue", "red"]`:

   ```sh
   printf 'red blue green white\nblue yellow red white\nyellow red blue red\n' | nme r mastermind
   ```

   ```text
   guess 4 colors (red blue green yellow): black: 0  white: 2
   guess 4 colors (red blue green yellow): black: 0  white: 3
   guess 4 colors (red blue green yellow): black: 4  white: 0
   solved in 3 turns
   ```

   The first two guesses place yellow and red somewhere wrong; the third
   shows every position matching. Play against your own secret by running
   the program without the pipe — every run hides a new code.

## Try it yourself

Count invalid guesses (a wrong color word) as a miss instead of
re-asking, or add a `tries` limit and a "you lost" message when it runs
out. Add two more colors and make the game ask for 5 — the feedback
algorithm does not change at all.

## What you learned

- `random_pick` inside a loop builds a random secret of any length.
- Black feedback is a position-by-position comparison.
- `min(count, count)` per color is the honest way to count shared colors.
- Subtracting black from white leaves exactly the misplaced colors.
- A `continue` guard keeps invalid input out of the scoring loop.
