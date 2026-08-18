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
| Sentence | `repeat 3 times … end` | `for _ in range(3):` |
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
| Beginner | `for each friend in friends:` | `for friend in friends:` |
| Advanced | `for friend in friends:` | unchanged |

The name before `in` / `마다` holds each item in turn, and the block body can use
it immediately.

## 8. Repeat while a condition holds

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `while score is less than 3` | `while (score < 3):` |
| Sentence | `while ready and waiting` | `while (ready and waiting):` |
| Sentence | `while ready then show working` | `while (ready): print("working")` |
| Beginner | `while score < 3` | `while (score < 3):` |
| Advanced | `while score < 3:` | unchanged |

## 9. Conditions

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `if score is greater than 10 then show You won` | `if (score > 10): print("You won")` |
| Sentence | `if name exists` | `if (name):` |
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
| Sentence | `skip` | `continue` |
| Sentence | `end` | (closes the block) |

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
| Advanced | `friends = ["Mina"]` | unchanged |

Without the `list of` / `목록` marker a comma-separated line is ordinary text.
`append Mina to friends` (a list) and `add 1 to score` (a number) are different
commands and keep their own meanings.

## 13. Values and literals

| English | Korean | Python |
| --- | --- | --- |
| `True` · `true` | `참` | `True` |
| `False` · `false` | `거짓` | `False` |
| `None` · `none` · `null` | `없음` | `None` |

## 14. Randomness

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `set die to random number from 1 to 6` | `die = __import__("random").randint(1, 6)` |
| Sentence | `set color to pick from red or green` | `color = __import__("random").choice(("red", "green",))` |
| Beginner | `use random` | (binds the random helpers) |
| Beginner | `say random_number(1, 6)` | `print(random_number(1, 6))` |

A choice written as a number stays a number (`pick from 1 or 2` →
`choice((1, 2,))`).

## 15. Files

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `read "notes.txt" into memo` | `memo = __import__("pathlib").Path("notes.txt").read_text()` |
| Sentence | `write "hello" to "out.txt"` | `__import__("pathlib").Path("out.txt").write_text("hello")` |
| Beginner | `use file` | (binds the file helpers) |
| Beginner | `say file_read("notes.txt")` | `print(file_read("notes.txt"))` |

A file path is always quoted. This is the one place the sentence level asks for
a quote character.

## 16. Modules

| Level | NME | Python produced |
| --- | --- | --- |
| Beginner | `use random` | (binds random, random_number, random_pick, shuffle and their Korean twins) |
| Beginner | `use file` | (binds file_read, file_write, json_load, json_save and their Korean twins) |
| Beginner | `use zero_knowledge` | (binds zk_secret, zk_public, zk_nizk_prove, … and their Korean twins) |
| Beginner | `use random latest` | (the newest bundled adapter) |
| Beginner | `use random version "0.0.1"` | (that exact adapter) |
| Advanced | `from "helper.nme" import greet` | (from helper import greet — needs helper.nme next to the program) |

## 17. Slow text

| Level | NME | Python produced |
| --- | --- | --- |
| Sentence | `say slowly Hello` | `[print(_ch, end="", flush=True) or __import__("time").sleep(0.04) for _ch in "Hello"]; print()` |
| Sentence | `show slowly Hello` | `[print(_ch, end="", flush=True) or __import__("time").sleep(0.04) for _ch in "Hello"]; print()` |
| Sentence | `say very slowly Hello` | `[print(_ch, end="", flush=True) or __import__("time").sleep(0.12) for _ch in "Hello"]; print()` |
| Sentence | `say slowly every 3 seconds Hello` | `[print(_ch, end="", flush=True) or __import__("time").sleep(3) for _ch in "Hello"]; print()` |

Each character is printed on its own with a short pause after it. The pause is
0.04 seconds by default, 0.12 with `very`, and whatever you name with
`every 3 seconds` / `3초씩`.

## 18. Screen

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

## 19. The stopwatch

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

## 20. Cooldowns

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

## 21. Every action word

Every spelling accepted for each action, with nothing left out.

| Action | English spellings | Korean spellings |
| --- | --- | --- |
| 출력 / Output | `say` · `show` · `display` · `tell` · `print` | `말해` · `말해줘` · `말해주세요` · `보여줘` · `보여주세요` · `출력해` · `출력해줘` · `출력해주세요` · `해줘` · `해주세요` · `읽어줘` |
| 입력 / Input | `ask` · `prompt` · `question` | `물어봐` · `물어봐줘` · `물어보세요` · `질문해` · `질문해줘` · `입력받아` · `입력받아줘` · `입력받아주세요` · `물어봐요` · `물어봐주세요` · `질문해주세요` |
| 저장 / Save | `set` · `save` · `remember` | `저장` · `저장해` · `기억해` · `기억해줘` · `설정` · `설정해` |
| 더하기 / Add | `add` · `increase` · `increment` · `plus` | `더해` · `더해줘` · `올려` · `올려줘` · `늘려` · `늘려줘` |
| 빼기 / Subtract | `subtract` · `decrease` · `decrement` · `minus` · `remove` | `빼` · `빼줘` · `내려` · `내려줘` · `줄여` · `줄여줘` |
| 곱하기 / Multiply | `multiply` · `multiplied` | `곱해` · `곱해줘` · `곱하기해` |
| 나누기 / Divide | `divide` · `divided` | `나눠` · `나눠줘` · `나누어줘` |
| 기다리기 / Wait | `wait` · `pause` · `sleep` | `기다려` · `기다려줘` · `기다리세요` · `기다려주세요` · `쉬어` · `쉬어줘` · `쉬세요` |
| 반복 / Repeat | `repeat` · `again` · `do` | `반복` · `반복해` · `반복해줘` · `반복해주세요` · `반복하세요` · `반복해서` · `반복하고` · `반복한다음` · `다시해` · `다시해주세요` |
| 조건 / If | `when` · `if` | `만약` · `만약에` · `만일` · `혹시` |
| 조건 반복 / While | `while` | `동안` · `하는동안` · `할동안` |
| 다른 갈래 / Else | `else` · `otherwise` | `아니면` · `그렇지않으면` · `아니면만약` · `아니면만약에` · `그렇지않으면만약` · `그렇지않으면만약에` |
| 반복 중단 / Break | `break` · `breakhere` | `멈춰` · `멈춰라` · `중단` · `반복멈춰` · `여기서멈춰` |
| 건너뛰기 / Skip | `skip` · `skipthis` · `skipit` · `nextone` | `건너뛰어` · `건너뛰어줘` · `건너뛰기` · `건너뛰자` · `넘어가` · `넘어가줘` |
| 블록 닫기 / End | `end` | `끝` |
| 모듈 쓰기 / Use | `use` · `load` · `get` · `import` | `사용` · `사용해` · `사용해줘` · `사용해주세요` · `불러와` · `불러와줘` · `가져와` · `가져와줘` · `받아` · `받아줘` |
| 목록에 넣기 / Append | `append` · `push` | `넣어` · `넣어줘` · `추가해` · `추가해줘` · `붙여` · `붙여줘` |
| 목록 표시 / List | `list` | `목록` · `리스트` |
| 숫자로 / As a number | `number` · `numeric` | `숫자` · `숫자로` · `수로` |
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
| 군말 / Filler | `please` | `좀` · `혹시` · `제발` |

## 22. Korean particles

These endings are not treated as part of the name they follow.

`에게서는` · `한테서는` · `에게서` · `한테서` · `으로는` · `로는` · `에게` · `한테` · `에서` · `으로` · `까지` · `부터` · `처럼` · `보다` · `이라도` · `라도` · `에는` · `에서` · `은` · `는` · `이` · `가` · `을` · `를` · `와` · `과` · `도` · `의` · `에` · `로` · `아` · `야` · `랑` · `이랑` · `예요` · `이에요`

## 23. Typo recovery

For action words and connectors only, and only after Python has rejected the
line, NME retries once with a single edit repaired (one insertion, deletion,
substitution, or adjacent swap). If more than one repair is possible it repairs
nothing and points at the exact span instead. **Strings and comments are never
touched.**

## 24. Error codes

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
| `E0201` | `say` value could not be understood |
| `E0202` | the `say` expression is not valid |
| `E0203` | the sentence to show is not valid |
| `E0204` | `say` has nothing to show |
| `E0211` | the question after the comma is missing |
| `E0212` | the question could not be understood |
| `E0213` | the `ask` target is not a variable name |
| `E0221` | the value change could not be understood |
| `E0222` | the break command could not be understood |
| `E0223` | the skip command could not be understood |
| `E0224` | the wait length could not be understood |
| `E0225` | the list addition could not be understood |
| `E0226` | the timer has not been started yet |
| `E0301` | the condition is missing |
| `E0302` | the condition could not be understood |
| `E0303` | the repeated body could not be understood |
| `E0304` | the repeat count could not be understood |
| `E0305` | the repeat count is missing |
| `E0306` | the repeat-over-a-list line could not be understood |
| `E0401` | NME bundles `use random` and `use file` |
| `E0402` | latest and an exact version on one line |
| `E0403` | the module version is missing |
| `E0404` | this module version is not bundled |
| `E0405` | the module would overwrite your names |
| `E0406` | the use line shape is not understood |
| `E0411` | the value to save is missing |
| `E0412` | the value to save could not be understood |
| `E0413` | the name to save into is missing |
| `E0414` | the save target is not a simple name |
| `E0501` | the repeated block is not indented |
| `E0502` | the condition needs a colon |
| `E0503` | a block that starts without a statement |
| `E0504` | one statement per line |
| `E0505` | the block body is not a statement NME knows |
| `E0601` | the sentence could mean more than one action |
| `E0602` | no NME action was found on this line |
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
