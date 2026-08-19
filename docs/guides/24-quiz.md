# 24 — Quiz: asking questions and marking them

English | [한국어](24-quiz.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [21 — Progress](21-progress.md), [23 — High score](23-high-score.md)
- Topic: games
- Result: a quiz that asks each question in turn, marks the answers, and reports a score

A quiz is one thing done once per question — ask it, compare the reply with the
answer, add a point if it matches. Keeping **the questions and the answers in
two lists side by side** makes adding a question one more line in each list.

## Steps

1. **Put the questions and answers side by side.** The same places belong
   together:

   ```nme
   set questions to list of the capital of Korea, the largest ocean
   set answers to list of Seoul, Pacific
   show the first of questions
   show the first of answers
   ```

   `the capital of Korea` and `Seoul`.

2. **Go through the questions and take the answer from the same place:**

   ```nme
   set questions to list of the capital of Korea, the largest ocean
   set answers to list of Seoul, Pacific
   for each question in questions with place
       set answer to item place of answers
       show question
       show answer
   end
   ```

3. **Ask, then compare.** A name after `ask` is used as the question itself:

   ```nme
   set question to the capital of Korea
   set answer to Seoul
   ask reply question
   if reply equals answer
       show that is right
   else
       show that is not right
       show answer
   end
   ```

4. **Add a point for each right answer.** The score starts at nothing:

   ```nme
   set score to 0
   add 1 to score
   add 1 to score
   show score
   ```

   You get `2`.

5. The whole thing:

   ```nme
   set questions to list of the capital of Korea, the largest ocean
   set answers to list of Seoul, Pacific
   set score to 0
   for each question in questions with place
       set answer to item place of answers
       ask reply question
       if reply equals answer
           show that is right
           add 1 to score
       else
           show that is not right
           show answer
       end
   end
   show score
   ```

## Try it yourself

Add another question and another answer — **to both lists**, or the places stop
lining up. Then put `show how many questions` at the end so the score reads as
"so many out of so many".

## What you learned

- Two lists side by side pair each question with its answer by place.
- A name after `ask` becomes the question that is shown.
- `if … equals` and `else` split what happens on a right and a wrong answer.
- A score starts at `0` and goes up with `add 1 to score`.
