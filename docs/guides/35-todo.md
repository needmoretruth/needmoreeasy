# 35 — Todo list: adding, dropping, listing

English | [한국어](35-todo.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [22 — Terminal menu](22-terminal-menu.md), [31 — Bank](31-bank.md)
- Topic: projects
- Result: a list that keeps taking commands to add, drop and show what needs doing

Every program so far asked once and stopped. A program people actually use
keeps taking commands **until it is told to stop**. One list and one loop that
keeps going is all it takes.

## Steps

1. **A loop with no count is `repeat forever`.** Getting out needs `break`:

   ```nme
   repeat forever
       ask command say quit to stop
       if command equals quit
           break
       end
       show got it
   end
   show that is everything
   ```

   Without the `break` the program never ends.

2. **Adding is one line into a list:**

   ```nme
   set jobs to an empty list
   append buy milk to jobs
   append read a book to jobs
   show how many jobs
   show jobs joined by comma
   ```

   `2`, then `buy milk, read a book`.

3. **Dropping something that is not there stops the program**, so ask first:

   ```nme
   set jobs to an empty list
   append buy milk to jobs
   if jobs contains buy milk
       remove buy milk from jobs
   else
       show there is no such job
   end
   show how many jobs
   ```

   You get `0`.

4. **Four commands means three `else if`s.** The last `else` is the way out:

   ```nme
   set command to list
   if command equals add
       show adding
   else if command equals drop
       show dropping
   else if command equals list
       show showing
   else
       show stopping
   end
   ```

5. The whole thing:

   ```nme
   set jobs to an empty list
   repeat forever
       ask command one of add, drop, list, quit
       if command equals add
           ask job what needs doing
           append job to jobs
       else if command equals drop
           ask job what is done
           if jobs contains job
               remove job from jobs
           else
               show there is no such job
           end
       else if command equals list
           show how many jobs
           show jobs joined by comma
       else
           break
       end
   end
   show that is everything
   ```

## Try it yourself

Make `list` show one job per line — inside `for each job in jobs with place`,
showing the place and the job gives you a numbered list. Closing the program
loses everything; keeping it in a file is what [guide 37](37-files.md) is for.

## What you learned

- `repeat forever` runs until it meets `break`. Always leave a way out.
- Several commands means `else if` in a row, with the last `else` as the exit.
- Asking whether something is there before dropping it is what keeps the program running.
- One list is enough to be a program — it just disappears when you close it.
