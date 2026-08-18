# 38 — Quiz — questions from a file

English | [한국어](38-quiz.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [14 — JSON](14-json.md), [26 — Adventure](26-adventure.md)
- Topic: game & data
- Result: a multiple-choice quiz that loads questions from a JSON file, scores answers, and reports the result

A quiz is a loop over data: each round shows one question, reads an answer, and
adds to a score. Guide [14](14-json.md) saved a record; this guide keeps the
questions in a JSON file so changing the quiz means editing data, not code. The
menu loop is the one from guide [26](26-adventure.md), aimed at options instead
of directions.

## Steps

1. Create `questions.json` — a list of question dicts. Each dict has the
   `question`, the `options`, and the `answer` as the position of the correct
   option, counting from 0:

   ```nme
   [
     {"question": "What is 5 + 3?", "options": ["6", "7", "8", "9"], "answer": 2},
     {"question": "What color is the sky?", "options": ["Green", "Blue", "Red", "Yellow"], "answer": 1},
     {"question": "How many legs does a dog have?", "options": ["2", "3", "4", "5"], "answer": 2}
   ]
   ```

   `"answer": 2` means the third option, `"8"`. Keeping the file data-only means
   a new question is just one new line in JSON.

2. Load the questions with `json_load` from guide [14](14-json.md). The file
   holds a list, so `json_load` returns a list of dicts:

   ```nme
   use file latest
   questions = json_load("questions.json")
   show f"Loaded {len(questions)} questions"
   ```

   Run `nme r quiz`; it prints `Loaded 3 questions`.

3. A `for q in questions:` loop visits each dict, and `enumerate` numbers the
   options for display. `ask` reads the answer, and the `if` compares it with
   the stored position:

   ```nme
   use file latest
   questions = json_load("questions.json")
   score = 0
   for q in questions:
       show q["question"]
       for i, option in enumerate(q["options"]):
           show str(i + 1) + ". " + option
       ask answer, "Your answer (number): "
       if answer == str(q["answer"] + 1):
           score = score + 1
   ```

   The answer is stored counting from 0, but the player counts from 1, so the
   stored `answer` needs a `+ 1` on both sides of the comparison.

4. The full game wraps that loop in `while play_again == "yes":` so a finished
   quiz can start over, and counts the missed questions. Save it as `quiz.nme`:

   ```nme
   # A multiple-choice quiz: the questions come from a JSON file.
   # Run: nme r quiz
   # The file questions.json must exist in the same folder.

   use file latest

   show "Welcome to the NME quiz!"
   ask name, "Your name: "
   questions = json_load("questions.json")

   play_again = "yes"
   while play_again == "yes":
       score = 0
       number = 0
       missed = []

       for q in questions:
           number = number + 1
           show ""
           show f"Question {number} of {len(questions)}: {q['question']}"
           for i, option in enumerate(q["options"]):
               show f"  {i + 1}. {option}"
           ask answer, "Your answer (number): "
           if answer == str(q["answer"] + 1):
               show "Correct!"
               score = score + 1
           else:
               show "Wrong. The correct answer was " + str(q["answer"] + 1) + "."
               missed.append(number)

       percent = score * 100 // len(questions)
       show ""
       show f"{name}: {score} of {len(questions)} correct ({percent}%)."
       if missed:
           show "Missed questions: " + str(missed)
       if percent == 100:
           show "Perfect score!"
       elif percent >= 70:
           show "Well done!"
       elif percent >= 50:
           show "Not bad. Keep practicing!"
       else:
           show "Try again - read each question carefully."

       ask play_again, "Play again? (yes/no): "

   show "Thanks for playing, " + name + "!"
   ```

5. Run it. The piped input plays one whole round — a name, then `1` for each
   question, then `no`:

   ```sh
   printf 'Mina\n1\n1\n1\nno\n' | nme r quiz
   ```

   ```text
   Welcome to the NME quiz!
   Your name: 
   Question 1 of 3: What is 5 + 3?
     1. 6
     2. 7
     3. 8
     4. 9
   Your answer (number): Wrong. The correct answer was 3.

   Question 2 of 3: What color is the sky?
     1. Green
     2. Blue
     3. Red
     4. Yellow
   Your answer (number): Wrong. The correct answer was 2.

   Question 3 of 3: How many legs does a dog have?
     1. 2
     2. 3
     3. 4
     4. 5
   Your answer (number): Wrong. The correct answer was 3.

   Mina: 0 of 3 correct (0%).
   Missed questions: [1, 2, 3]
   Try again - read each question carefully.
   Play again? (yes/no): Thanks for playing, Mina!
   ```

   All three `1` answers are wrong here, so the score stays 0 and the missed
   list shows every question. Answering `3`, `2`, `3` would give a perfect
   score.

6. The Korean twin `quiz.ko.nme` writes the same game with `파일 사용 최신`,
   `json읽기`, `물어봐`, and `말해`. The answers are numbers, so the same
   piped input works in both languages.

## Try it yourself

Change `questions.json` to hold four questions of your own, and make `answer`
point at the right option for each. Then rerun `quiz.nme` and answer with the
correct numbers; the score should reach 100%.

## What you learned

- A JSON list of dicts is a whole question bank; `json_load` returns it as a list.
- `enumerate(q["options"])` numbers the options for display.
- The stored `answer` counts from 0; the player's choice counts from 1.
- `while play_again == "yes":` lets the player replay the whole quiz.
