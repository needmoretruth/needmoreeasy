# 17 — Word guess: a hidden-word game

English | [한국어](17-word-guess.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [12 — Random](12-random.md), [11 — Break](11-break.md)
- Topic: game
- Result: a hangman-style game where you guess letters of a hidden word with limited tries

A hangman game hides a word and shows one blank per letter. A right guess
fills the blanks, a wrong one costs a try. The word comes from `random_pick`
(guide [12](12-random.md)), and a `while` loop keeps asking for letters until
the word is filled in or the tries run out.

## Steps

1. Keep a list of secret words and pick one at random with `random_pick`:

   ```nme
   use random latest
   words = ["apple", "grape", "melon", "mango", "orange"]
   secret = random_pick(words)
   show secret
   ```

   Run it a few times: a different word appears each time.

2. Show the word as one blank per letter. A list of `"_"` strings starts
   hidden, and `" ".join(shown)` prints them with spaces:

   ```nme
   secret = "grape"
   shown = []
   for ch in secret:
       shown.append("_")
   show " ".join(shown)
   ```

   `_ _ _ _ _` means five letters.

3. Reveal a guessed letter by walking the word with a `for` loop and writing
   the letter into the matching blank. `for i in range(len(secret))` visits
   every position, and `shown[i] = letter` fills the blank when it matches:

   ```nme
   secret = "grape"
   shown = []
   for ch in secret:
       shown.append("_")
   ask letter, "guess a letter: "
   for i in range(len(secret)):
       if secret[i] == letter:
           shown[i] = letter
   show " ".join(shown)
   ```

   Guessing `a` turns `_ _ _ _ _` into `_ _ a _ _`.

4. The full game adds a try counter and win/lose checks. `while remaining > 0`
   keeps asking, a wrong guess lowers `remaining`, `"_" not in shown` means
   every letter was found, and `break` leaves the loop. Save it as
   `word-guess.nme`:

   ```nme
   # A hidden-word game: guess the letters of a secret word.
   # Run: nme r word-guess

   use random latest

   words = ["apple", "grape", "melon", "mango", "orange"]
   secret = random_pick(words)

   shown = []
   for ch in secret:
       shown.append("_")

   remaining = 6
   guessed = []

   show "I picked a word with " + str(len(secret)) + " letters."

   while remaining > 0:
       show " ".join(shown)
       show "tries left: " + str(remaining)
       ask letter, "guess a letter (q to quit): "
       if letter == "q":
           show "the word was " + secret
           break
       if letter in guessed:
           show "you already guessed " + letter
           continue
       guessed.append(letter)
       if letter in secret:
           for i in range(len(secret)):
               if secret[i] == letter:
                   shown[i] = letter
           show "yes, " + letter + " is in the word"
           if "_" not in shown:
               show " ".join(shown)
               show "you won! the word was " + secret
               break
       else:
           remaining = remaining - 1
           show "no, " + letter + " is not in the word"

   if remaining == 0:
       show "you lost. the word was " + secret
   ```

5. Run it and guess four letters, then quit:

   ```sh
   printf 'e\na\nr\nq\n' | nme r word-guess
   ```

   ```text
   I picked a word with 5 letters.
   _ _ _ _ _
   tries left: 6
   guess a letter (q to quit): yes, e is in the word
   _ _ _ _ e
   tries left: 6
   guess a letter (q to quit): yes, a is in the word
   _ _ a _ e
   tries left: 6
   guess a letter (q to quit): yes, r is in the word
   _ r a _ e
   tries left: 6
   guess a letter (q to quit): the word was grape
   ```

   The hidden word is picked at random, so your run shows a different word and
   a different blank pattern.

## Try it yourself

Make the game accept capital letters. Right after `ask letter, ...` add
`letter = letter.lower()` so typing `A` guesses the same as `a`:

```nme
secret = "grape"
shown = []
for ch in secret:
    shown.append("_")
ask letter, "guess a letter: "
letter = letter.lower()
for i in range(len(secret)):
    if secret[i] == letter:
        shown[i] = letter
show " ".join(shown)
```

Then add the same line to the full game and test it with `A` and `G`.

## What you learned

- `random_pick(words)` picks the hidden word from a list.
- A list of `"_"` shows one blank per letter; `" ".join(shown)` prints it.
- A `for` loop over `range(len(secret))` reveals each matching letter.
- `while remaining > 0` counts down the tries and `break` leaves the loop.
