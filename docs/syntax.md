# NME syntax list

English | [한국어](syntax.ko.md)

[Home](../README.md) | [Install](install.md) | [Getting started](getting-started.md) | [Tutorial](tutorial.md) | [Language reference](language.md) | [Guides](guides/index.md)

**Every spelling NME actually accepts**, in one place. For explanations, read the
[language reference](language.md); this file is a table, not a tour.

It is generated from the compiler source
(`python scripts/build-syntax-reference.py`), and a check fails the build if the
compiler accepts a spelling that is missing here — so the list cannot drift away
from the implementation.

## How to read this

- **Three levels.** *Sentence* needs no quotes, commas, parentheses, equals
  signs or colons; *beginner* is short and exact; *advanced* is ordinary Python.
  They mix freely in one file and on one line, with nothing to declare.
- **Python wins.** If a line is valid Python, NME leaves it alone. That is why a
  one-word line (`skip`, `멈춰`) stays a Python name and only becomes an NME
  command inside a loop block.
- **One NME statement is one Python line.** The line count never changes, so a
  traceback points at the line you actually wrote.
- The `Python produced` column is exactly what the compiler emits.

## 1. Output

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `show Hello world!` | `print("Hello world!")` |
| Sentence | `Hello world show` | `print("Hello world")` |
| Sentence | `Hello everyone!` | `print("Hello everyone!")` |
| Sentence | `show Hello name!` | `print("Hello " + str(name) + "!")` |
| Beginner | `say "Hello"` | `print("Hello")` |
| Beginner | `say total + 1` | `print(total + 1)` |
| Advanced | `print("Hello")` | unchanged |

**Text or code.** After an action word, a valid Python expression whose names the
program already knows is treated as **code**; anything else is **text**. `say`
and `말해` are the exception: they try the expression first. A name introduced
earlier is substituted into text (`show Hello name!` →
`"Hello " + str(name) + "!"`).

## 2. Input

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `ask name What is your name?` | `name = input("What is your name?" + " ")` |
| Sentence | `ask number age How old are you?` | `age = int(input("How old are you?" + " "))` |
| Sentence | `What is your name?` | `name = input("What is your name?" + " ")` |
| Sentence | `How old are you?` | `age = int(input("How old are you?" + " "))` |
| Sentence | `name ask` | `name = input()` |
| Beginner | `ask name, "Name? "` | `name = input("Name? ")` |
| Advanced | `name = input()` | unchanged |

`ask number` / `숫자로` produces `int(input(...))`. A question about an age or a
count (`How old are you?`, `몇 살이에요?`) is read as a number without it. A
prompt that does not end in a space gets one.

## 3. Save a value

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `set greeting to Hello` | `greeting = "Hello"` |
| Sentence | `set answer to 7` | `answer = 7` |
| Sentence | `greeting save Hello` | `greeting = "Hello"` |
| Sentence | `remember score to 0` | `score = 0` |
| Sentence | `set score to 0.` | `score = 0` |
| Beginner | `save total to 1 + 2` | `total = 1 + 2` |
| Advanced | `greeting = 'Hello'` | unchanged |

Korean marks the target with the particle `은`/`는` and needs no action word.
English has no such particle, so it uses `set … to …`.

## 4. Change a value

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `score add 1` | `score = score + 1` |
| Sentence | `add 1 to score` | `score = score + 1` |
| Sentence | `score increase by 1` | `score = score + 1` |
| Sentence | `to score add 1` | `score = score + 1` |
| Sentence | `subtract 1 from score` | `score = score - 1` |
| Sentence | `multiply score by 2` | `score = score * 2` |
| Sentence | `divide score by 2` | `score = score / 2` |
| Sentence | `subtract 1 + 2 from score` | `score = score - (1 + 2)` |
| Advanced | `score += 1` | unchanged |

⚠ A multi-word amount is parenthesised: `subtract 1 + 2 from score` is
`score - (1 + 2)`, not `score - 1 + 2`.

## 5. Wait

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `wait 3 seconds` | `__import__("time").sleep(3)` |
| Sentence | `pause 3` | `__import__("time").sleep(3)` |
| Sentence | `wait 1 second` | `__import__("time").sleep(1)` |
| Sentence | `wait two seconds` | `__import__("time").sleep(2)` |
| Sentence | `wait for 5 seconds` | `__import__("time").sleep(5)` |
| Sentence | `sleep pause_length` | `__import__("time").sleep(pause_length)` |
| Advanced | `import time; time.sleep(3)` | unchanged |

The unit word (`seconds`, `초`) is optional. A line with no number in it
(`잠깐 기다려`) stays ordinary output.

## 6. Repeat a number of times

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `repeat 3 times and show Again` | `for _ in range(3): print("Again")` |
| Sentence | `3 times Welcome` | `for _ in range(3): print("Welcome")` |
| Sentence | `repeat three times and show Again` | `for _ in range(3): print("Again")` |
| Sentence | `repeat 3 rounds and show Again` | `for _ in range(3): print("Again")` |
| Sentence | `repeat 3 times … end` | `for _ in range(3):` |
| Sentence | `repeat forever` | `while True:` |
| Sentence | `repeat forever and show Again` | `while True: print("Again")` |
| Beginner | `3 times: say "Hi"` | `for _ in range(3): print("Hi")` |
| Advanced | `for i in range(3):` | unchanged |

A block closes three ways: by indentation, by one statement after `:`, or by a
line containing only `end` / `끝`.

## 7. Repeat over a list

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `for each friend in friends` | `for friend in friends:` |
| Sentence | `for each friend in friends and show friend` | `for friend in friends: print(friend)` |
| Sentence | `repeat for each name in names` | `for name in names:` |
| Sentence | `foreach friend in friends` | `for friend in friends:` |
| Sentence | `for every friend in friends` | `for friend in friends:` |
| Sentence | `for each friend in friends with place` | `for place, friend in enumerate(friends, 1):` |
| Beginner | `for each friend in friends:` | `for friend in friends:` |
| Beginner | `for each friend in friends with place:` | `for place, friend in enumerate(friends, 1):` |
| Advanced | `for friend in friends:` | unchanged |

The name before `in` / `마다` holds each item in turn, and the block body can use
it immediately.

## 8. Repeat while a condition holds

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `while score is less than 3` | `while (score < 3):` |
| Sentence | `while ready and waiting` | `while (ready and waiting):` |
| Sentence | `while score is greater than 0` | `while (score > 0):` |
| Sentence | `while ready then show working` | `while (ready): print("working")` |
| Beginner | `while score < 3` | `while (score < 3):` |
| Advanced | `while score < 3:` | unchanged |

## 9. Conditions

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `if score is greater than 10 then show You won` | `if (score > 10): print("You won")` |
| Sentence | `if name exists` | `if (name):` |
| Sentence | `if score > 10 then show You won` | `if (score > 10): print("You won")` |
| Sentence | `if score is above 10 then show You won` | `if (score > 10): print("You won")` |
| Sentence | `score is greater than 5 then show high` | `if (score > 5): print("high")` |
| Sentence | `else if score equals 0` | `elif (score == 0):` |
| Sentence | `else` | `else:` |
| Beginner | `when score == 1: say "one"` | `if (score == 1): print("one")` |
| Advanced | `if score == 1:` | unchanged |

## 10. Comparison vocabulary

| NME | Python | Meaning |
| --- | --- | --- |
| `if name exists` | `name` | 참인 값 / truthy |
| `if name missing` | `not (name)` | 거짓인 값 / falsey |
| `if score equals 10` | `score == 10` | == |
| `if score is not equal to 10` | `score != 10` | != |
| `if score is greater than 10` | `score > 10` | > |
| `if score is less than 10` | `score < 10` | < |
| `if score is greater than or equal to 10` | `score >= 10` | >= |
| `if score is less than or equal to 10` | `score <= 10` | <= |
| `if ready and score > 2` | `ready and score > 2` | and / 그리고 |
| `if ready or waiting` | `ready or waiting` | or / 또는 |

English accepts synonyms for the comparison words: `greater`, `above`, `great`,
`larger`, `bigger`, `higher` all mean `>`; `less`, `below`, `small`, `smaller`,
`lower` all mean `<`; `equals`, `equal`, `same` mean `==`.

## 11. Loop control

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `break` | `break` |
| Sentence | `break here` | `break` |
| Sentence | `stop` | `break` |
| Sentence | `exit loop` | `break` |
| Sentence | `quit` | `break` |
| Sentence | `skip` | `continue` |
| Sentence | `keep going` | `continue` |
| Sentence | `end` | (closes the block) |
| Sentence | `finish` | (closes the block) |

`break` and `skip` (`멈춰`, `건너뛰어`) are valid Python names on their own, so
they are read as NME **only inside a loop block**. Outside one they stay Python.

## 12. Lists

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `set friends to list of Mina, Ada` | `friends = ["Mina", "Ada"]` |
| Sentence | `set friends to list of Mina and Ada` | `friends = ["Mina", "Ada"]` |
| Sentence | `set scores to list of 1, 2, 3` | `scores = [1, 2, 3]` |
| Sentence | `set friends to list of` | `friends = []` |
| Sentence | `append Mina to friends` | `friends.append("Mina")` |
| Sentence | `push Mina to friends` | `friends.append("Mina")` |
| Sentence | `add Mina to friends` | `friends.append("Mina")` |
| Sentence | `to friends append Mina` | `friends.append("Mina")` |
| Sentence | `set friends to an empty list` | `friends = []` |
| Sentence | `remove Mina from friends` | `friends.remove("Mina")` |
| Sentence | `show how many friends` | `print(len(friends))` |
| Sentence | `set total to how many friends` | `total = len(friends)` |
| Sentence | `show You have how many friends left` | `print("You have " + str(len(friends)) + " left")` |
| Sentence | `sort friends` | `friends.sort()` |
| Sentence | `reverse friends` | `friends.reverse()` |
| Sentence | `shuffle friends` | `__import__("random").shuffle(friends)` |
| Sentence | `show the first of friends` | `print(friends[0])` |
| Sentence | `show the last of friends` | `print(friends[-1])` |
| Sentence | `show item 2 of friends` | `print(friends[1])` |
| Sentence | `show the total of scores` | `print(sum(scores))` |
| Sentence | `show the biggest of scores` | `print(max(scores))` |
| Sentence | `show the smallest of scores` | `print(min(scores))` |
| Sentence | `show friends joined by comma` | `print(", ".join(map(str, friends)))` |
| Sentence | `show friends joined by space` | `print(" ".join(map(str, friends)))` |
| Sentence | `show friends joined together` | `print("".join(map(str, friends)))` |
| Sentence | `show friends joined by nothing` | `print("".join(map(str, friends)))` |
| Sentence | `if friends contains Mina` | `if ("Mina" in friends):` |
| Sentence | `if friends does not contain Ada` | `if ("Ada" not in friends):` |
| Sentence | `if friends is empty` | `if (not (friends)):` |
| Beginner | `say len(friends)` | `print(len(friends))` |
| Advanced | `friends = ["Mina"]` | unchanged |

Without the `list of` / `목록` marker a comma-separated line is ordinary text.
`append Mina to friends` (a list) and `add 1 to score` (a number) are different
commands and keep their own meanings.

**Items are counted from one.** `the first of friends` is `item 1 of friends`
and becomes `friends[0]`. There is no item 0; writing one is `E0229`.

The statements that read or rearrange a list (`how many`, `sort`, `shuffle`,
`the first of`, `the total of`, `remove`, …) only work on a name the program
already made a list. That is what keeps `sort out your things` and `the first
of many` ordinary sentences. Using one on a name that is not a list is
`E0231`.

`comma` joins with a comma and a space (`", "`), `space` with one space, and
`newline` with a line break. Items go through `map(str, …)`, so a list of
numbers joins as readily as a list of words.

## 13. Records

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `set ages to an empty record` | `ages = {}` |
| Sentence | `set ages to record` | `ages = {}` |
| Sentence | `set ages to an empty table` | `ages = {}` |
| Sentence | `put Mina at 90 in ages` | `ages["Mina"] = 90` |
| Sentence | `put Mina as 90 in ages` | `ages["Mina"] = 90` |
| Sentence | `set Mina to 90 in ages` | `ages["Mina"] = 90` |
| Sentence | `show Mina in ages` | `print(ages["Mina"])` |
| Sentence | `set best to Mina in ages` | `best = ages["Mina"]` |
| Sentence | `show how many ages` | `print(len(ages))` |
| Sentence | `if ages contains Mina` | `if ("Mina" in ages):` |
| Sentence | `for each name in ages` | `for name in ages:` |
| Sentence | `remove Mina from ages` | `del ages["Mina"]` |
| Beginner | `save rates to {}` | `rates = {}` |
| Beginner | `say ages["Mina"]` | `print(ages["Mina"])` |
| Advanced | `ages["Mina"] = 90` | unchanged |

A record holds many named values at once. Python calls it a dictionary; a
beginner calls it a list of names and what goes with each.

`an empty record` / `빈 표` is only a record **where a value is being saved**.
An ordinary sentence holding the same word — `I keep a record of everything I
read` — stays a sentence.

The spellings a record shares with a list — `how many`, `remove`, `contains`,
`for each`, `개수`, `빼`, `넣어` — are **the same words**, and what they do
depends on which kind the name holds. That is deliberate: a reader should not
have to remember which is which. Using a record line on a list is `E0234`.

A record has no order, so `the total of`, `the biggest of`, `the first of` and
`sort` are not written for one. `for each name in ages` hands back the
**names**, so the value is read back with `name in ages`.

`set Mina to 90 in ages` and `set best to Mina in ages` are the same six words
in the same order, and they mean opposite things. What follows `to` decides: a
number or a quoted string is a **value being written**, and a word is a **name
being read**. A record kept under numbers is still readable — put the number in
a name first (`set key to 90`, then `set who to key in ages`).

A name Python wrote as `ages = {}` is a record too.

## 14. Working with text

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `show the length of name` | `print(len(name))` |
| Sentence | `show name in capitals` | `print(str(name).upper())` |
| Sentence | `show name in small letters` | `print(str(name).lower())` |
| Sentence | `set names to memo split by line` | `names = str(memo).splitlines()` |
| Sentence | `set fields to memo split by comma` | `fields = str(memo).split(",")` |
| Sentence | `set words to memo split by space` | `words = str(memo).split(" ")` |
| Sentence | `set bar to name repeated 5 times` | `bar = str(name) * 5` |
| Sentence | `show name repeated 3 times` | `print(str(name) * 3)` |
| Beginner | `say len(name)` | `print(len(name))` |
| Advanced | `name.upper()` | unchanged |

`the length of` gives how many characters a piece of text has, and
`in capitals` / `in small letters` give the same text with its letters
changed. All three are values, so they work in output, in a saved name, and in
a condition. They only read a name the program already made, which is what
keeps an ordinary sentence containing one of those words a sentence.

## 15. Number remainders

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `show the remainder of score divided by 4` | `print(score % 4)` |
| Sentence | `set left to the remainder of score divided by 4` | `left = score % 4` |
| Sentence | `if the remainder of score divided by 4 equals 0` | `if (score % 4 == 0):` |
| Beginner | `say score % 4` | `print(score % 4)` |
| Advanced | `left = score % 4` | unchanged |

`the remainder of` is what is left over after a division. It is a value, so it
works in output, in a saved name, and in a condition; the number being divided
by must be a number or a name the program already made.

## 16. Values and literals

| English | Korean | Python |
| --- | --- | --- |
| `True` · `true` | `참` | `True` |
| `False` · `false` | `거짓` | `False` |
| `None` · `none` · `null` | `없음` | `None` |

## 17. Randomness

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `set die to random number from 1 to 6` | `die = __import__("random").randint(1, 6)` |
| Sentence | `set color to pick from red or green` | `color = __import__("random").choice(("red", "green",))` |
| Sentence | `set color to choose from red or green` | `color = __import__("random").choice(("red", "green",))` |
| Beginner | `use random` | (binds the random helpers) |
| Beginner | `say random_number(1, 6)` | `print(random_number(1, 6))` |

A choice written as a number stays a number (`pick from 1 or 2` →
`choice((1, 2,))`).

## 18. Chance

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `30% chance show You win` | `if __import__("random").randrange(1000) < 300: print("You win")` |
| Sentence | `30.5% chance show You win` | `if __import__("random").randrange(1000) < 305: print("You win")` |
| Sentence | `with a 30% chance show You win` | `if __import__("random").randrange(1000) < 300: print("You win")` |
| Sentence | `30 percent chance show You win` | `if __import__("random").randrange(1000) < 300: print("You win")` |
| Sentence | `30% of the time show You win` | `if __import__("random").randrange(1000) < 300: print("You win")` |
| Sentence | `30% chance` | `if __import__("random").randrange(1000) < 300:` |
| Sentence | `luck is a 30% chance` | `luck = __import__("random").randrange(1000) < 300` |

`30%` means 300 times in a thousand. One decimal place is the finest you may
write (`30.5%`); anything finer is reported as `E0227` rather than rounded,
because a program must never quietly mean something you did not write. The
range is `0%` to `100%`, and outside it you get `E0228`. `100%` always happens
and `0%` never does. The comparison is between whole thousandths, so no
floating-point rounding can creep in.

A chance saved in a name is an ordinary true/false value, so the usual
condition words can ask about it: `if luck then show You win`.

## 19. Files

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `read "notes.txt" into memo` | `memo = __import__("pathlib").Path("notes.txt").read_text()` |
| Sentence | `write "hello" to "out.txt"` | `__import__("pathlib").Path("out.txt").write_text("hello")` |
| Beginner | `use file` | (binds the file helpers) |
| Beginner | `say file_read("notes.txt")` | `print(file_read("notes.txt"))` |

A file path is always quoted. This is the one place the sentence level asks for
a quote character.

## 20. Modules

| Level | NME | Python produced |
| --- | --- | --- |
| Beginner | `use random` | (binds random, random_number, random_pick, shuffle and their Korean twins) |
| Beginner | `use file` | (binds file_read, file_write, json_load, json_save and their Korean twins) |
| Beginner | `use zero_knowledge` | (binds zk_secret, zk_public, zk_nizk_prove, … and their Korean twins) |
| Beginner | `use list` | (binds count, sort, reverse, remove, first, last, sum, largest, smallest and their Korean twins) |
| Beginner | `use text` | (binds upper, lower, trim, split, join, replace, starts_with, length and their Korean twins) |
| Beginner | `use math` | (binds math, root, round_to, pi, power, absolute, floor, ceil and their Korean twins) |
| Beginner | `use date` | (binds today, now, year, month, day_of_month, weekday, days_after and their Korean twins) |
| Beginner | `use random latest` | (the newest bundled adapter) |
| Beginner | `use random version "0.0.1"` | (that exact adapter) |
| Sentence | `use greet from "helper.nme"` | (from helper import greet — needs helper.nme next to the program) |
| Beginner | `from "helper.nme" import greet` | (from helper import greet — needs helper.nme next to the program) |
| Advanced | `from "helper.nme" import greet` | (from helper import greet — needs helper.nme next to the program) |

After `use random`, `use file` and `use zero_knowledge`, the four most asked
for are `use list`, `use text`, `use math` and `use date`. One line makes every
name below ready in both languages, and everything inside them is plain Python
builtins, so they run in the browser as they stand. `list`, `text`, `math` and
`date` are ordinary words, so they name a module only when they stand beside
`use`/`사용` and nothing else is left over on the line; `get the list of names`
and `What is the date today?` are sentences and print.

`yesterday`, `tomorrow`, `3 days ago` and `3 days from now` — Korean `어제`,
`내일`, `3일 전`, `3일 뒤` — are the sentence way to step off today. They mean a
date only after the toolbox is open, so `3 days ago I saw her` and
`약속은 3일 전이었습니다` are sentences and print.

Six of the date names answer with nothing written after them, so they may be
written without brackets and a whole date program stays sentences:
`show today`, `show weekday`, `show year`, `오늘 말해줘`, `요일 말해줘`,
`올해 말해줘` — and `now`/`지금`, `month`/`이번달`, `day_of_month`/`오늘일자`
the same way. `days_after(n)` is not one of them: it needs the number written
after it, and what is missing there is the writer's, not the compiler's.

On a Wednesday, `weekday()` answers `Wednesday` and its Korean name `요일()`
answers `수요일`.
This is the one place in any bundled module where the two names hold different
values: a weekday name is a word rather than a number, so it has to be in some
language, and the name you write chooses which. The clock a browser hands
Python is UTC, so in the browser `today()` is today in UTC.

`use list` (`목록 사용`):

| English | Korean | Python |
| --- | --- | --- |
| `count(values)` | `개수(값들)` | `len(values)` |
| `sort(values)` | `정렬(값들)` | `sorted(values)` — a new list; the original is left alone |
| `reverse(values)` | `뒤집기(값들)` | `list(reversed(values))` |
| `remove(values, x)` | `빼기(값들, x)` | every item that is not `x`, as a new list |
| `first(values)` | `첫번째(값들)` | `values[0]` |
| `last(values)` | `마지막(값들)` | `values[-1]` |
| `sum(values)` | `합계(값들)` | `sum(values)` |
| `largest(values)` | `최대(값들)` | `max(values)` |
| `smallest(values)` | `최소(값들)` | `min(values)` |
| `list_version` | `목록버전` | `"0.0.1"` |

`use text` (`글자 사용`):

| English | Korean | Python |
| --- | --- | --- |
| `upper(text)` | `대문자(글)` | `str(text).upper()` |
| `lower(text)` | `소문자(글)` | `str(text).lower()` |
| `trim(text)` | `공백없애기(글)` | `str(text).strip()` |
| `split(text, sep)` | `나누기(글, 구분자)` | `str(text).split(sep)` |
| `join(sep, values)` | `합치기(구분자, 값들)` | `str(sep).join(map(str, values))` |
| `replace(text, a, b)` | `바꾸기(글, a, b)` | `str(text).replace(a, b)` |
| `starts_with(text, a)` | `로시작(글, a)` | `str(text).startswith(a)` |
| `length(text)` | `길이(글)` | `len(text)` |
| `text_version` | `글자버전` | `"0.0.1"` |

`use math` (`수학 사용`):

| English | Korean | Python |
| --- | --- | --- |
| `root(x)` | `제곱근(x)` | `math.sqrt(x)` |
| `round_to(x, places)` | `반올림(x, 자리)` | `round(x, places)` — `places` may be left out |
| `pi` | `원주율` | `math.pi` |
| `power(x, y)` | `거듭제곱(x, y)` | `pow(x, y)` — whole numbers stay whole |
| `absolute(x)` | `절댓값(x)` | `abs(x)` |
| `floor(x)` | `내림(x)` | `math.floor(x)` |
| `ceil(x)` | `올림(x)` | `math.ceil(x)` |
| `math_version` | `수학버전` | `"0.0.1"` |

`use date` (`날짜 사용`):

| English | Korean | Python |
| --- | --- | --- |
| `today()` | `오늘()` | `date.today().isoformat()` — text such as `"2026-08-19"` |
| `now()` | `지금()` | `datetime.now().strftime("%H:%M")` — the clock a browser gives is UTC |
| `year()` | `올해()` | `date.today().year` |
| `month()` | `이번달()` | `date.today().month` |
| `day_of_month()` | `오늘일자()` | `date.today().day` |
| `weekday()` | `요일()` | `strftime("%A")`, an English name such as `Wednesday`; the Korean name answers `수요일` |
| `days_after(n)` | `며칠뒤(n)` | `(date.today() + timedelta(days=n)).isoformat()`; a negative `n` reads as days before |
| `date_version` | `날짜버전` | `"0.0.1"` |

`sort`, `reverse` and `remove` hand back a new list and leave the original
alone. To change the list itself, use the sentence statements `sort friends`,
`reverse friends` and `remove Mina from friends`.

## 21. Slow text

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `say slowly Hello` | `[print(_ch, end="", flush=True) or __import__("time").sleep(0.04) for _ch in "Hello"]; print()` |
| Sentence | `show slowly Hello` | `[print(_ch, end="", flush=True) or __import__("time").sleep(0.04) for _ch in "Hello"]; print()` |
| Sentence | `say very slowly Hello` | `[print(_ch, end="", flush=True) or __import__("time").sleep(0.12) for _ch in "Hello"]; print()` |
| Sentence | `say slowly every 3 seconds Hello` | `[print(_ch, end="", flush=True) or __import__("time").sleep(3) for _ch in "Hello"]; print()` |

Each character is printed on its own with a short pause after it. The pause is
0.04 seconds by default, 0.12 with `very`, and whatever you name with
`every 3 seconds` / `3초씩`.

## 22. Stories

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `story:` | `if True:` |
| Sentence | `slow story:` | `if True:` |
| Sentence | `very slow story:` | `if True:` |
| Sentence | `slow story every 3 seconds:` | `if True:` |
| Sentence | `The door opened.` | `print("The door opened.")` |
| Sentence | `wait 3 seconds` | (inside a story: print("wait 3 seconds")) |

Inside `story:` **every line is text**. `wait 3 seconds` and `if ready` are not
commands there; they print, exactly as written. Writing something novel-like is
the whole point of the form, so a line of prose can never quietly turn into a
statement. Close the block with `end` / `끝`, or, if you opened it by
indenting, by ending the indentation. A blank line prints an empty line, names
you made earlier are still substituted into the text, and the colon may be the
plain `:` or the full-width `：` a Korean keyboard writes.

## 23. Named jobs

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `to greet:` | `def greet():` |
| Sentence | `to greet:` | `def greet():` |
| Sentence | `do greet` | `greet()` |
| Sentence | `run greet` | `greet()` |
| Sentence | `do greet` | `greet()` |
| Sentence | `to hail someone:` | `def hail(someone):` |
| Sentence | `do hail with Mina` | `hail("Mina")` |
| Sentence | `run hail with Mina` | `hail("Mina")` |
| Advanced | `def greet():` | unchanged |

A named job gives a piece of program a name, so it can be run later by that
name. Python calls it a function (`def`).

`to`, `do`, `일` and `하기` are ordinary words, so a named job is recognized by
**structure and never by a word**: the opening `to` (or, in Korean, the `라는`
on the name and the `일` after it), the closing `:`, **and a block underneath**.
`to be honest` and `할 일이 많습니다` have none of that.

**Without a block it is not a job.** A heading such as `To do:` on its own is
printed as the line it is, and there is no one-line form.

The line that runs a job only runs one when the name is **a job this program
already made**. `do` and `run` decide nothing on their own. A Python `def` that
takes no arguments can be run the same way.

A name saved inside a job stays inside it, exactly as in a Python function.

A job **may be given one thing.** The header names it in front of the job name
(`to greet someone:`, `이름에게 인사하기라는 일:`), and the line that runs the
job hands it over the same way (`do greet with Mina`, `민수에게 인사하기 해줘`).
Giving a job the wrong number of things is refused with `E0235`, because the
Python `TypeError` it would otherwise cause happens at run time on a line that
looks right.

Sentence grammar has **no job that takes two things** and **no job that hands
something back** yet. Write a Python `def` when you need either.

## 24. Screen

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `clear the screen` | `print("\033[2J\033[3J\033[H", end="")` |
| Sentence | `clear screen` | `print("\033[2J\033[3J\033[H", end="")` |
| Sentence | `draw a line` | `print("─" * 40)` |
| Sentence | `draw line` | `print("─" * 40)` |
| Sentence | `say in a box Hello` | `print((lambda _t: (lambda _w: "┌" + "─" * (_w + 2) + "┐\n│ " + _t + " │\n└" + "─" * (_w + 2) + "┘")(sum(2 if __import__("unicodedata").east_asian_width(_c) in "WF" else 1 for _c in _t)))("Hello"))` |
| Sentence | `say in the middle Hello` | `print((lambda _t: " " * max(0, (40 - sum(2 if __import__("unicodedata").east_asian_width(_c) in "WF" else 1 for _c in _t)) // 2) + _t)("Hello"))` |

Clearing the screen sends a terminal control sequence, so somewhere that is not
a terminal it may show up as text. The box and the centred line count a Korean
character as two columns, so a Korean sentence comes out straight; the width is
40 columns.

## 25. The stopwatch

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `start the timer` | `_nme_clock = __import__("time").time()` |
| Sentence | `start timer` | `_nme_clock = __import__("time").time()` |
| Sentence | `show elapsed` | `print(round(__import__("time").time() - _nme_clock, 2))` |
| Sentence | `set spent to elapsed` | `spent = round(__import__("time").time() - _nme_clock, 2)` |

`start the timer` starts the clock and `elapsed` / `잰시간` / `걸린시간` reads
how many seconds have passed, to two decimal places. It is a value, so it works
in output, in a saved name, and in a condition (`if elapsed is greater than 3`).
Reading it without starting the clock is reported at compile time as `E0226`. A
name the program made itself always wins over the word.

## 26. Cooldowns

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `put door on cooldown for 3 seconds` | `_nme_cool_door = __import__("time").time() + 3` |
| Sentence | `when door is ready` | `if (__import__("time").time() >= _nme_cool_door):` |
| Sentence | `if door is ready` | `if (__import__("time").time() >= _nme_cool_door):` |
| Sentence | `when door is on cooldown` | `if (__import__("time").time() < _nme_cool_door):` |
| Sentence | `wait for door` | `__import__("time").sleep(max(0, _nme_cool_door - __import__("time").time()))` |
| Sentence | `pause for door` | `__import__("time").sleep(max(0, _nme_cool_door - __import__("time").time()))` |

One cooldown belongs to one name. `put door on cooldown for 3 seconds` remembers
the moment three seconds from now, and `is ready` / `쿨타임이 끝났으면` asks
whether that moment has passed. They are conditions, so they work with `when`,
`while`, `else if`, and the one-line form of all three. `wait for door` also
reads as an ordinary English sentence, so a name the program already saved as
something else is not read as a cooldown.

## 27. Every action word

Every spelling accepted for each action, with nothing left out.

| Action | English spellings | Korean spellings |
| --- | --- | --- |
| 출력 / Output | `say` · `show` · `display` · `tell` · `print` | `말해` · `말해줘` · `말해주세요` · `보여줘` · `보여주세요` · `출력해` · `출력해줘` · `출력해주세요` · `해줘` · `해주세요` · `읽어줘` |
| 입력 / Input | `ask` · `prompt` · `question` | `물어봐` · `물어봐줘` · `물어보세요` · `질문해` · `질문해줘` · `입력받아` · `입력받아줘` · `입력받아주세요` · `물어봐요` · `물어봐주세요` · `질문해주세요` |
| 저장 / Save | `set` · `save` · `remember` · `store` · `let` · `make` | `저장` · `저장해` · `저장해줘` · `기억해` · `기억해줘` · `설정` · `설정해` · `설정해줘` · `지정` · `지정해` · `정해` · `만들어` |
| 더하기 / Add | `add` · `increase` · `increment` · `plus` · `up` · `goesup` · `grow` · `bump` · `boost` | `더해` · `더해줘` · `올려` · `올려줘` · `늘려` · `늘려줘` · `더하기` · `증가` · `증가해` · `증가시켜` |
| 빼기 / Subtract | `subtract` · `decrease` · `decrement` · `minus` · `remove` · `down` · `goesdown` | `빼` · `빼줘` · `내려` · `내려줘` · `줄여` · `줄여줘` · `빼기` · `감소` · `감소해` · `감소시켜` |
| 곱하기 / Multiply | `multiply` · `multiplied` | `곱해` · `곱해줘` · `곱하기해` |
| 나누기 / Divide | `divide` · `divided` | `나눠` · `나눠줘` · `나누어줘` |
| 기다리기 / Wait | `wait` · `pause` · `sleep` · `hold` · `delay` · `rest` | `기다려` · `기다려줘` · `기다리세요` · `기다려주세요` · `쉬어` · `쉬어줘` · `쉬세요` · `대기해` · `대기해줘` · `대기` |
| 반복 / Repeat | `repeat` · `again` · `do` | `반복` · `반복해` · `반복해줘` · `반복해주세요` · `반복하세요` · `반복해서` · `반복하고` · `반복한다음` · `다시해` · `다시해주세요` |
| 조건 / If | `when` · `if` · `should` · `incase` · `whenever` | `만약` · `만약에` · `만일` · `혹시` |
| 조건 반복 / While | `while` · `aslongas` · `repeatwhile` · `keepgoingwhile` | `동안` · `하는동안` · `할동안` |
| 다른 갈래 / Else | `else` · `otherwise` · `orelse` · `elseinstead` | `아니면` · `아니면은` · `아니라면` · `그렇지않으면` · `그렇지않다면` · `안그러면` · `안그렇다면` · `그외에는` · `그외에` · `아니면만약` · `아니면만약에` · `그렇지않으면만약` · `그렇지않으면만약에` |
| 반복 중단 / Break | `break` · `breakhere` | `멈춰` · `멈춰줘` · `멈춰라` · `멈추기` · `그만해` · `정지해` · `종료해` · `중단` · `반복멈춰` · `여기서멈춰` |
| 말끝 출력 / Output written last | — | `말하기` · `말해라` · `알려줘` · `알려주세요` · `알려줘요` · `얘기해` · `얘기해줘` · `얘기해주세요` · `표시해` · `표시해줘` · `출력하기` · `보여주기` · `프린트해` · `프린트` · `프린트해줘` · `프린트해주세요` · `표시하기` |
| 목적어가 없을 때만 말끝 출력 / Output written last, with no object | — | `띄워` · `띄워줘` · `띄워주세요` · `나타내` · `나타내줘` · `나타내주세요` · `써줘` · `써주세요` · `적어줘` · `적어주세요` |
| 한 낱말만 보여 주기 / Show the one word after it | `output` · `write` · `echo` · `reveal` · `report` · `give` · `list` · `present` · `announce` · `speak` · `puts` | — |
| 한 글자 출력 / Short output word | — | `말` |
| 화면에 / On the screen | `screen` | `화면에` · `화면에다` · `화면에다가` · `스크린에` |
| 화면 동사 / Screen verb | `put` · `write` · `print` · `show` · `display` · `say` · `tell` · `output` · `draw` | `띄워` · `띄워줘` · `보여줘` · `출력해` · `말해` · `말해줘` · `표시해` |
| 물음표가 있을 때만 묻기 / Ask, only with a question mark | `read` · `get` · `request` · `enter` · `input` | `받아` · `받아줘` · `여쭤봐` · `여쭤봐줘` · `여쭈어봐` · `요청해` · `요청해줘` · `요청해주세요` · `달라고해` · `달라고해줘` |
| 짧은 물어보기 / Short asking word | — | `물어` |
| ~로 해 / Save with the everyday verb | `becomes` · `become` · `call` | `해` · `하자` · `합시다` · `하죠` · `부르자` · `두어` · `둬` · `두자` |
| ~로 / ~라고 표시 / What the name becomes | — | `으로` · `로` · `이라고` · `라고` |
| 횟수가 있을 때만 반복 / Repeat, only with a count | `loop` · `iterate` · `cycle` · `rep` · `goround` · `runthrough` | `돌려` · `돌려줘` · `돌려주세요` · `되풀이` · `되풀이해` · `되풀이해줘` · `되풀이해서` · `되풀이하기` |
| 반복할 것을 가리키는 말 / What is being repeated | `it` · `this` · `that` | `그거` · `그걸` · `그것` · `그것을` · `이거` · `이걸` · `이것` · `이것을` |
| 조건을 닫는 말 / Word that closes a condition | — | `때` · `때에` · `때는` · `경우` · `경우에` · `경우에는` |
| 건너뛰기 / Skip | `skip` · `skipthis` · `skipit` · `nextone` | `건너뛰어` · `건너뛰어줘` · `건너뛰기` · `건너뛰자` · `넘어가` · `넘어가줘` · `계속해` · `넘겨` · `다음` |
| 블록 닫기 / End | `end` · `finish` · `done` | `끝` · `종료` · `마침` |
| 모듈 쓰기 / Use | `use` · `load` · `get` · `import` | `사용` · `사용해` · `사용해줘` · `사용해주세요` · `불러와` · `불러와줘` · `가져와` · `가져와줘` · `받아` · `받아줘` |
| 다른 파일에서 / Import from a file | `use` · `take` · `borrow` | `가져와` · `가져와줘` · `가져오기` · `불러와` · `불러오기` |
| 목록에 넣기 / Append | `append` · `push` · `insert` · `put` · `place` | `넣어` · `넣어줘` · `넣기` · `추가해` · `추가해줘` · `추가하기` · `붙여` · `붙여줘` |
| 목록 표시 / List | `list` | `목록` · `리스트` |
| 빈 목록 / Empty list | `empty` · `blank` | `빈` · `비어있는` · `새` |
| 개수 / How many | `count` · `number` · `many` | `개수` · `갯수` |
| 개수 앞말 / Reading lead | `how` | — |
| 길이 / Length | `length` · `size` | `길이` · `글자수` |
| 합 / Total | `total` · `sum` | `합` · `합계` · `총합` |
| 최댓값 / Biggest | `biggest` · `largest` · `highest` · `maximum` | `최댓값` · `최대값` · `큰` |
| 최솟값 / Smallest | `smallest` · `lowest` · `minimum` | `최솟값` · `최소값` · `작은` |
| 최댓값 앞말 / Extreme scope | — | `중` · `중에서` · `가운데` |
| 가장 / Most | — | `가장` · `제일` |
| 것 / Thing | — | `것` · `값` |
| 첫 번째 / First | `first` | `첫번째` · `첫째` · `처음` · `첫` |
| 마지막 / Last | `last` | `마지막` · `맨뒤` |
| 몇 번째 / Item | `item` · `element` | `번째` · `째` |
| 대문자 / Capitals | `capitals` · `capital` · `uppercase` | `대문자로` · `대문자` |
| 소문자 / Small letters | `lowercase` · `small` | `소문자로` · `소문자` |
| 이어 붙이기 / Join | `joined` · `join` | `이어` · `이어서` · `이어붙여` |
| 사이 없이 이어 붙이기 / Join together | `together` | `붙여` · `붙여서` · `붙여줘` · `이어붙여` · `이어붙여줘` |
| 빈 이음말 / Empty separator | `nothing` | `그대로` |
| 나누기(글) / Split | `split` | `나눈` · `쪼갠` · `자른` |
| 줄마다 / By line | `line` · `lines` | `줄마다` · `줄별로` · `한줄씩` |
| 나눈 것 / Split thing | — | `것` · `거` · `것들` |
| 붙인 것 / Repeated text | `repeated` | `붙인` · `이어붙인` |
| 몇 개 / Copies | `times` | `개` · `번` |
| 몇 번째와 함께 / With its position | `with` | `함께` · `같이` |
| 나머지 / Remainder | `remainder` · `rest` · `leftover` | `나머지` |
| 나누기 말 / Divided | `divided` · `shared` · `split` | `나눈` · `나눈뒤` · `나누고` |
| 이음말 / Separator | `comma` · `space` · `newline` | `쉼표` · `빈칸` · `공백` · `줄바꿈` |
| 정렬 / Sort | `sort` · `order` · `arrange` · `sortout` | `정렬해` · `정렬해줘` · `정렬` · `정렬하기` · `순서대로` · `순서대로해` · `차례대로` · `차례대로해` · `오름차순` · `오름차순으로` · `오름차순으로해` |
| 거꾸로 / Reverse | `reverse` · `flip` · `invert` | `거꾸로` · `거꾸로해` · `거꾸로해줘` · `뒤집어` · `뒤집어줘` · `뒤집기` · `반대로` · `반대로해` · `역순으로` · `역순으로해` |
| 섞기 / Shuffle | `shuffle` · `mix` · `jumble` · `scramble` · `randomise` · `randomize` | `섞어` · `섞어줘` · `섞어주세요` · `섞기` · `랜덤하게` · `랜덤하게해` · `무작위로해` |
| 들어있는지 / Contains | `contains` · `contain` · `includes` · `include` · `holds` | `안에는` · `속에는` · `안에` · `속에` · `에는` · `에` |
| 표 표시 / Record | `record` · `table` | `표` |
| 표에 넣기 / Put in a record | `put` · `set` · `store` · `save` · `record` · `add` | `넣어` · `넣어줘` · `넣어주세요` · `넣기` · `두어` · `두어줘` · `저장해` · `저장해줘` · `기억해` · `기억해줘` · `적어` · `적어줘` |
| 표 값 앞말 / Record value connector | `at` · `as` | `으로` · `로` |
| 표 이름 앞말 / Record container connector | `in` · `into` · `to` | `을` · `를` |
| 표에서 읽기 조사 / Record reading particle | — | `에서` · `의` |
| 일 표시 / Job | `to` | `일` · `작업` |
| 일 이름 어미 / Job name ending | — | `이라는` · `라는` |
| 일 실행 / Run a job | `do` · `run` | `해` · `해줘` · `해주세요` · `실행해` · `실행해줘` |
| 일이 받는 것 / What a job is given | `with` | `에게` · `한테` · `을` · `를` |
| 무한 반복 / Forever | `forever` · `always` | `계속` · `무한` · `끝없이` |
| 읽기 조사 / Reading particle | — | `이` · `가` · `은` · `는` · `을` · `를` |
| 숫자로 / As a number | `number` · `numeric` | `숫자` · `숫자로` · `수로` |
| 숫자 낱말 / Number words | `zero` · `one` · `two` · `three` · `four` · `five` · `six` · `seven` · `eight` · `nine` · `ten` · `once` · `twice` | `하나` · `한` · `둘` · `두` · `셋` · `세` · `넷` · `네` · `다섯` · `여섯` · `일곱` · `여덟` · `아홉` · `열` · `일` · `이` · `삼` · `사` · `오` · `육` · `칠` · `팔` · `구` · `십` |
| 횟수 단위 / Count unit | `times` · `time` · `loops` · `loop` · `rounds` · `round` | `번` · `회` · `차례` · `판` |
| 반복 중단(블록 안) / Break inside a block | `stop` · `stophere` · `exitloop` · `quit` | — |
| 건너뛰기(블록 안) / Skip inside a block | `keepgoing` · `carryon` | — |
| 무작위 고르기 / Random pick | `randomchoice` · `pick` · `choose` | `랜덤선택` · `하나골라` · `골라` · `하나뽑아` · `뽑아` |
| 값 바꾸기 연결어 / Value-change connector | `to` · `by` · `from` · `of` · `into` · `onto` | — |
| 목록 연결어 / List connector | `to` · `into` · `onto` · `in` | `에다가` · `에다` · `에` · `한테` · `에게` |
| 저장 대상 조사 / Saved-name particle | — | `을` · `를` · `이` · `가` · `에` |
| 문장 어미 / Sentence ending | — | `입니다` · `이에요` · `예요` · `이다` · `으로` · `로` · `라고` · `이라고` |
| 최신판 / Latest | `latest` · `newest` | `최신` · `최신판` · `최신버전` |
| 파일 읽기 / File read | `read` | `읽어서` · `읽고` · `읽어` |
| 파일 쓰기 / File write | `write` | `저장해` · `저장해줘` · `써줘` · `적어` |
| 천천히 / Slowly | `slowly` | `천천히` |
| 아주 / Very | `very` | `아주` |
| 글자 간격 / Interval | `every` | `초씩` |
| 화면 / Clear screen | `clear` | `화면` |
| 화면 지우기 / Clear screen action | `screen` | `지워` · `지워줘` · `비워` · `비워줘` |
| 줄 / Draw line | `draw` | `줄` · `가로줄` |
| 줄 긋기 / Draw line action | `line` | `그어` · `그어줘` |
| 상자 / Box | `box` | `상자로` |
| 가운데 / Middle | `middle` | `가운데` |
| 시간 재기 / Start timer | `start` | `시간재기시작해` · `시간재기시작` |
| 시계 / Timer | `timer` | — |
| 잰 시간 / Elapsed | `elapsed` | `잰시간` · `걸린시간` |
| 쿨타임 / Cooldown | `cooldown` | `쿨타임` · `쿨타임을` · `쿨타임은` · `쿨타임이` |
| 쿨타임 걸기 / Put on cooldown | `put` | `걸어` · `걸어줘` |
| 쿨타임 끝남 / Ready | `ready` | `끝났으면` |
| 쿨타임 남음 / On cooldown | — | `남았으면` |
| 쿨타임 끝날 때까지 / Until ready | — | `끝날때까지` |
| 이야기 / Story | `story` · `tale` | `이야기` · `얘기` |
| 이야기 천천히 / Story, slowly | `slow` · `slowly` | `천천히` |
| 확률 / Chance | `chance` · `chances` · `probability` | `확률로` · `확률` |
| 퍼센트 / Percent | `percent` · `percentage` | `퍼센트` · `프로` |
| 확률 앞뒤 말 / Chance connector | `with` | `의` · `로` · `으로` |
| 확률의 다른 말 / Chance, other wording | `time` | — |
| 확률 저장 / Chance saved in a name | `is` · `equals` | — |
| 군말 / Filler | `please` | `좀` · `혹시` · `제발` |

## 28. Korean particles

These endings are not treated as part of the name they follow.

`에게서는` · `한테서는` · `에게서` · `한테서` · `으로는` · `로는` · `에게` · `한테` · `에서` · `으로` · `까지` · `부터` · `처럼` · `보다` · `이라도` · `라도` · `만큼` · `밖에` · `에는` · `에서` · `은` · `는` · `이` · `가` · `을` · `를` · `와` · `과` · `도` · `의` · `에` · `로` · `아` · `야` · `랑` · `이랑` · `예요` · `이에요` · `님` · `님께` · `님은` · `님이`

## 29. Typo recovery

For action words and connectors only, and only after Python has rejected the
line, NME retries once with a single edit repaired (one insertion, deletion,
substitution, or adjacent swap). If more than one repair is possible it repairs
nothing and points at the exact span instead. **Strings and comments are never
touched.**

## 30. Error codes

| Code | Meaning |
| --- | --- |
| `E0001` | this line is not valid Python or NME |
| `E0101` | an `end` with no open block |
| `E0102` | `break` outside a loop |
| `E0103` | an `else` or `elif` with no open condition |
| `E0104` | two `else` branches in one condition |
| `E0105` | a block without its closing `end` |
| `E0106` | `return` outside a function |
| `E0107` | `continue` outside a loop |
| `E0108` | `yield` outside a function |
| `E0109` | `await` outside an async function |
| `E0110` | `yield from` inside an async function |
| `E0111` | `async for` outside an async function |
| `E0112` | `async with` outside an async function |
| `E0113` | `nonlocal` with no enclosing function |
| `E0114` | star import outside module scope |
| `E0115` | control flow inside an `except*` block |
| `E0116` | `yield` inside a comprehension |
| `E0117` | async comprehension outside an async function |
| `E0118` | return value inside an async generator |
| `E0119` | conflicting `global` declaration |
| `E0120` | conflicting `nonlocal` declaration |
| `E0201` | a sum after `say` with a piece missing |
| `E0202` | the value after `say` is not one Python can read |
| `E0203` | the sentence to show is not valid |
| `E0204` | `say` has nothing to show |
| `E0211` | the question after the comma is missing |
| `E0212` | NME could not read the question |
| `E0213` | no name to hold the answer |
| `E0221` | NME could not read the value change |
| `E0222` | NME could not read the line that leaves the loop |
| `E0223` | NME could not read the line that skips a round |
| `E0224` | NME could not read how long to wait |
| `E0225` | this list line could not be read |
| `E0226` | the timer has not been started yet |
| `E0227` | a chance can only go to one decimal place |
| `E0228` | a chance must be between 0% and 100% |
| `E0229` | items are counted from 1 |
| `E0230` | a name cannot have a space in it |
| `E0231` | this name was never made into a list |
| `E0232` | this story has nothing in it |
| `E0233` | this join does not say what to put between the items |
| `E0234` | a list line and a record line were mixed up |
| `E0235` | this job is given a different number of things than it takes |
| `E0236` | this job reads a name from outside and then changes it |
| `E0237` | this name is one the language itself needs |
| `E0238` | a file read with no name to read into |
| `E0239` | a file name that is not in quotation marks |
| `E0240` | putting something in requires a list |
| `E0301` | the condition is missing |
| `E0302` | NME could not read the condition |
| `E0303` | NME could not read what to repeat |
| `E0304` | NME could not read how many times to repeat |
| `E0305` | the repeat count is missing |
| `E0306` | NME could not read the line that goes through a list |
| `E0401` | NME bundles seven modules, and this is not one of them |
| `E0402` | latest and an exact version on one line |
| `E0403` | the module version is missing |
| `E0404` | this module version is not bundled |
| `E0405` | the module would take over a name you made |
| `E0406` | a module line NME could not read |
| `E0407` | a module path this line cannot use |
| `E0408` | a module import line NME could not read |
| `E0409` | a module loaded twice |
| `E0410` | a module tool with nothing to work on |
| `E0411` | the value to save is missing |
| `E0412` | NME could not read the value to save |
| `E0413` | the name to save into is missing |
| `E0414` | the name to save into is not a plain name |
| `E0501` | the lines to repeat are not indented |
| `E0502` | nothing to do when the condition is true |
| `E0503` | a one-line block with nothing in it |
| `E0504` | one thing to do per line |
| `E0505` | the line inside this block is not one NME knows |
| `E0506` | this line starts with a space |
| `E0601` | the sentence could mean more than one action |
| `E0602` | no NME action was found on this line |
| `E0603` | NME does not know this word |
| `E0604` | this line cannot do anything |
| `E0605` | this line uses a curly quote |
| `E0701` | a sentence-style line across several physical lines |
| `E0702` | the Python source is not valid |
| `E9001` | unknown command |
| `E9002` | `modules` takes no extra arguments |
| `E9003` | an option is missing its value |
| `E9004` | unknown option |
| `E9005` | unexpected extra file |
| `E9006` | `convert` needs a file |
| `E9007` | a file could not be read |
| `E9008` | a file could not be written |
| `E9009` | refusing to overwrite the output |
| `E9010` | the native compiler failed |
| `E9011` | the native compiler could not be started |
| `E9012` | CPython rejected the generated Python |
| `E9013` | Python could not be started |
| `E9014` | that is a folder, not a program |
| `E9015` | the program file does not exist |
| `E9016` | the current folder could not be read |
| `E9017` | no .nme program in this folder |
| `E9018` | the pick answer could not be read |
| `E9019` | no pick answer given |
| `E9020` | the pick answer is not a listed program |
| `E9021` | the pick answer matches several programs |
| `E9022` | several programs match this name |
| `E9023` | the error lookup takes one code |
| `E9024` | unknown error code |
| `E9025` | pip could not install the package |
| `E9026` | the native program could not be started |
| `E9027` | a temporary working folder could not be created |
| `E9028` | two imported modules have the same name |
| `E9029` | module imports are not supported by `nme compile` |
| `E9030` | the package name is missing |
| `E9031` | `-o` is only available with `nme native build` |
| `E9032` | more than one native action was given |

Run `nme en E0102` for the long explanation of a code (`nme ko E0102` in
Korean).
