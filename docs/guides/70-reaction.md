# 70 — Game: a reaction time test

English | [한국어](70-reaction.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [68 — Errors](68-errors.md), [10 — Random](10-random.md)
- Topic: game & timing
- Result: a game that measures reaction time with time.time() and reports the best of several rounds

A reaction test must know when GO appeared and when you pressed Enter; `time.time()` clocks seconds, so two readings measure the gap.

## Steps

1. `time.time()` returns seconds since a fixed moment; read it before and
   after `input()` and subtract:
   ```text
   import time
   start = time.time()
   input()
   elapsed = time.time() - start
   show f"You took {elapsed:.2f} seconds"
   ```
   Piped Enter is instant, so a piped run reports about `0.00`.
2. A fixed wait lets you time your click; a random delay fixes that:
   ```text
   use random latest
   import time
   show "waiting..."
   time.sleep(random_number(1, 3))
   show "GO!"
   ```
   `time.sleep(seconds)` pauses; `random_number(1, 3)` picks the wait.
3. The full program runs three rounds and keeps the fastest, with a `0.1`
   sleep for fast piped tests. Use `random_number(1, 3)` for a real game:
   ```text
   # reaction.nme — measure your reaction time over rounds.
   # Run: nme r reaction
   # time.time() clocks the moment; the best round wins.

   import time

   rounds = 3
   best = None
   total = 0

   show "Reaction test — press Enter as soon as you see GO."
   show f"We play {rounds} rounds; the fastest one wins."
   show "An early press is a false start — wait for GO."
   input()
   for round_number in range(1, rounds + 1):
       show f"Round {round_number} of {rounds}: get ready..."
       time.sleep(0.1)
       show "GO!"
       start = time.time()
       input()
       elapsed = time.time() - start
       total = total + elapsed
       show f"  You took {elapsed:.2f} seconds."
       if best is None or elapsed < best:
           best = elapsed
           show "  That is the new best time!"
       else:
           show f"  The best is still {best:.2f} seconds."

   show ""
   show "Final results:"
   show f"  Best time:   {best:.2f} seconds"
   average = total / rounds
   show f"  Average:     {average:.2f} seconds"
   ```
4. Run it, piping one empty line for the ready prompt plus one per round:
   ```sh
   printf '\n\n\n\n' | nme r reaction
   ```
   ```text
   Reaction test — press Enter as soon as you see GO.
   We play 3 rounds; the fastest one wins.
   An early press is a false start — wait for GO.
   Round 1 of 3: get ready...
   GO!
     You took 0.00 seconds.
     That is the new best time!
   Round 2 of 3: get ready...
   GO!
     You took 0.00 seconds.
     The best is still 0.00 seconds.
   Round 3 of 3: get ready...
   GO!
     You took 0.00 seconds.
     The best is still 0.00 seconds.

   Final results:
     Best time:   0.00 seconds
     Average:     0.00 seconds
   ```
5. `best = None` means "no round yet"; `if best is None or elapsed < best`
   keeps the smallest time; piped rounds all tie at `0.00` and vary by tiny fractions.

## Try it yourself

Raise `rounds` to 5, or print the average. Change the delay to `time.sleep(random_number(1, 3))` and run with a real keyboard.

## What you learned

- `time.time()` returns the current clock in seconds; subtract two readings to measure a gap.
- `time.sleep(seconds)` pauses the program.
- `random_number(1, 3)` / `1부터 3까지 랜덤정수` makes the delay unpredictable.
- `best = None` then `if best is None or elapsed < best` keeps the smallest time across rounds.
