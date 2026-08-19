# 19 — Game: how fast are you?

English | [한국어](19-reaction.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [15 — Time](15-timer.md)
- Topic: a game, and timing
- Result: a program that measures how fast you react

The stopwatch from [15 — Time](15-timer.md) is enough for a game. Start the
clock the moment the signal appears, and read it when the key comes down.

## Steps

1. **Starting the clock and reading it** is the whole idea:

   ```nme
   start the timer
   ask answer Press enter
   show elapsed
   ```

   `elapsed` is the seconds since the clock was started.

2. **Give the signal late.** Asking straight away lets someone press early, so
   pause first:

   ```nme
   show Get ready
   wait 3 seconds
   show Press enter now
   ```

3. **Keeping the reading in a name** lets you use it more than once:

   ```nme
   start the timer
   ask answer Enter
   set taken to elapsed
   show taken
   ```

4. **Say how fast that was:**

   ```nme
   set taken to 0.4
   if taken is less than 1
       show very fast
   else if taken is less than 2
       show not bad
   else
       show a little slow
   end
   ```

5. All of it together:

   ```nme
   show A reaction test
   show Get ready
   wait 3 seconds
   show Press enter now
   start the timer
   ask answer Enter
   set taken to elapsed
   show taken
   if taken is less than 1
       show very fast
   else if taken is less than 2
       show not bad
   else
       show a little slow
   end
   ```

   The clock gives the time up to the moment `elapsed` is read, so where you
   start it is exactly where the measuring begins.

## Try it yourself

Measure three times and show the best of them. Put each reading in a list and
ask for `the smallest of times` — the list is from [05 — Set](05-set.md) and
the smallest is from [17 — Word guess](17-word-guess.md).

## What you learned

- `start the timer` starts the clock and `elapsed` gives the seconds since.
- `set taken to elapsed` keeps a reading so it can be used again.
- A `wait` before the signal is what stops an early press.
- The reading is an ordinary number, so a condition takes it as it is.
