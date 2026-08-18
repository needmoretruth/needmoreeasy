# 15 — High score: a tiny project

English | [한국어](15-high-score.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [13 — Files](13-files.md), [14 — JSON](14-json.md), [06 — If](06-if.md)
- Topic: project
- Result: a dice game that remembers its best score

Everything you know fits into one small project: a dice game that remembers
its best score. `use random` rolls the die, `use file` saves the record.

## Steps

1. Create `best.nme` and save the whole game as one file:

   ```text
   use random latest
   use file latest
   import os
   ask name What is your name?
   score = 0
   3 times:
       roll = random_number(1, 6)
       if roll >= 4:
           score add 1
       say f"{name} rolled {roll} (score {score})"
   if os.path.exists("best.json"):
       best = json_load("best.json")
       say f"Last best was {best['score']}"
   else:
       best = {"name": "nobody", "score": 0}
   if score > best["score"]:
       json_save("best.json", {"name": name, "score": score})
       say f"New best: {score}"
   else:
       say f"Best stays {best['score']}"
   ```

   Three rounds roll a die; each roll of 4 or more adds 1 to the score. The
   `if` lines read a saved best and save a new best when the score beats it.

2. Run it and type a name. One run might print:

   ```sh
   nme run best
   ```

   ```text
   What is your name? Mina rolled 6 (score 1)
   Mina rolled 6 (score 2)
   Mina rolled 5 (score 3)
   New best: 3
   ```

   Your rolls and score will differ — the dice are random. The first run has
   no saved best, so the score always becomes the new best.

3. Run it again: the previous best comes back from the file before the game
   starts, and `best.json` holds `{"name": "Mina", "score": 3}`. Beating it
   saves a new best; otherwise the old one stays.

4. Korean writes the same project with `랜덤 사용 최신`, `파일 사용 최신`,
   `점수에 1 더해`, and `json저장`. The full Korean program is in the
   [Korean guide](15-high-score.ko.md); this snippet loads a saved best:

   ```text
   파일 사용 최신
   최고 = json읽기("best.json")
   말해 f"저장된 최고 점수: {최고['score']}점"
   ```

## Try it yourself

Make the game harder: change `3 times:` to `4 times:` for a longer game, or
change `roll >= 4` to `roll >= 5` for a stricter score.

## What you learned

- One project can combine `use random`, `use file`, `ask`, `3 times:`, `if`,
  and JSON in a single file.
- `os.path.exists(path)` checks whether a saved file is there yet.
- `json_load` restores the previous best; `json_save` writes a new one.
- `score add 1` / `점수에 1 더해` grows the score only inside the condition
  block.
