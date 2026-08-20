# 33 — Habit tracker: a mark for each day

English | [한국어](33-habit.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [32 — Grade book](32-grade-book.md), [12 — Random](12-random.md)
- Topic: projects
- Result: keeping a yes or no for each date and counting the days it was done

A habit tracker is **one answer per date**, which makes it a
[record](41-address-book.md) whose names are dates. And the dates are not typed
by hand — the computer is asked for them.

## Steps

1. **Open the date tools and work out today, yesterday and the day before.**
   A negative number in `days_after` goes backwards:

   ```nme
   use date latest
   set stamp to today
   set before to days_after(-1)
   set earlier to days_after(-2)
   show stamp
   show before
   show earlier
   ```

2. **Use the date as the name the answer goes under:**

   ```nme
   use date latest
   set log to an empty record
   set stamp to today
   put stamp at yes in log
   show stamp in log
   ```

   You get `yes`. **A name in the name position means the date inside it
   becomes the name.**

3. **Today's answer is asked for:**

   ```nme
   use date latest
   set log to an empty record
   set stamp to today
   ask reply did you do it today
   put stamp at reply in log
   show how many log
   ```

4. **Counting the days is a walk with a comparison.** Take the value out and
   compare it:

   ```nme
   set log to an empty record
   put Monday at yes in log
   put Tuesday at no in log
   set done to 0
   for each day in log
       set mark to day in log
       if mark equals yes
           add 1 to done
       end
   end
   show done
   ```

   You get `1`.

5. The whole thing:

   ```nme
   use date latest
   set log to an empty record
   set stamp to today
   set before to days_after(-1)
   set earlier to days_after(-2)
   put earlier at yes in log
   put before at no in log
   ask reply did you do it today
   put stamp at reply in log
   for each day in log
       show day
       show day in log
   end
   set done to 0
   for each day in log
       set mark to day in log
       if mark equals yes
           add 1 to done
       end
   end
   show done
   ```

## Try it yourself

Count the run of days in a row — walk backwards from today and `break` at the
first day that is not `yes`. Then keep it in a file the way
[guide 36](36-diary.md) does, so the record survives closing the program.

## What you learned

- One answer per date is a record whose names are dates.
- `days_after(-1)` is yesterday; a negative number goes backwards.
- A name in the name position puts the value inside it there instead.
- Counting is walking, taking the value out, comparing, and adding one.
