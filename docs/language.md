# NME language reference

English | [한국어](language.ko.md)

[Home](../README.md) | [Install](install.md) | [Getting started](getting-started.md) | [Tutorial](tutorial.md)

NME has three syntax levels in one compiler. There is no mode switch: advanced
Python, beginner NME, sentence NME, Korean, and English may appear together in
one file or one block.

## The compatibility rule

Valid Python always wins. NME asks a real Python parser whether a line is valid
before matching easier syntax. A valid Python program therefore compiles
byte-identically.

```python
say = print
say("Python call")
if ready:
    print("Python condition")
```

The three levels compile as follows:

| Level | NME | Python result |
| --- | --- | --- |
| Sentence | `repeat 3 times and show Hi` | `for _ in range(3): print("Hi")` |
| Beginner | `3 times: say "Hi"` | `for _ in range(3): print("Hi")` |
| Advanced | `for _ in range(3): print("Hi")` | unchanged |

## Sentence level

Sentence syntax is for a first program. Quotes, commas, parentheses, braces,
equals signs, and colons are optional for the common tasks below. Normal
sentence punctuation `?` and `!` is accepted without quotes.

### Show text and values

```nme
show Hello world!
Hello world show
보여줘 안녕하세요!
안녕하세요 말해줘
Please show me hello
```

These print literal text. A name created earlier by an input or sentence
assignment is inserted automatically:

```nme
ask name What is your name?
show Hello name!

이름을 물어봐 이름이 뭐예요?
안녕하세요 이름! 말해줘
```

The result uses the value of `name` / `이름`; other words stay literal.
Korean particles following a known name remain in the output.

If a line is clearly ordinary multi-word speech, NME can print it without an
action word:

```nme
Hello everyone!
오늘도 반가워요!
```

A line holding a single word prints too. It is valid Python — a name read and
thrown away — but a program that says nothing and then dies with a `NameError`
is nobody's intention, so NME prints the word. Two lines keep their Python
meaning: a name the program set earlier, and a word NME spells out itself
(`say`, `end`, `skip`, `목록`).

The shortest conversation does not need a prompt or punctuation:

```nme
name ask
Hello name show
```

For the gentlest possible first input, ask as a normal question:

```nme
What is your name?
What's your city?
Hello name!
```

The matching target (`name` or `city`) is inferred from the question. Korean
questions such as `이름이 뭐예요`, `내 이름은 뭐예요?`, and
`나이는 몇 살이에요?` work the same way; `몇 살이에요?` is also inferred as
`나이`. English `How old are you?` / `How old am I` infer `age`. The final `?`
is optional. Use `ask number` when the answer must be converted to a number;
complex or ambiguous questions should use the explicit `ask` form.

Accepted output actions include `show`, `display`, `tell`, `say`, `보여줘`,
`말해줘`, `말해주세요`, `출력해`, and `출력해줘`. The precise beginner
spellings `say expression` and `말해 표현식` continue to treat a valid Python
expression as code.

### Ask for text or a number

```nme
ask name What is your name?
ask name, What is your name?
이름을 물어봐 이름이 뭐예요?

ask number age How old are you?
나이를 숫자로 물어봐 몇 살인가요?
```

Natural prompts receive a separating space automatically. A comma is optional
for a plain-language prompt; quoted or expression prompts may use the precise
beginner comma form. Text input compiles to `input(...)`; number input compiles
to `int(input(...))`.

Accepted actions include `ask`, `prompt`, `물어봐`, `물어봐줘`, `질문해`, and
`입력받아`. Korean target particles `을` and `를` are removed from the variable
name.

### Save a value

```nme
인사는 안녕하세요
정답은 7
set greeting to Hello
set answer to 7
greeting save Hello
이름 저장 민수
score add 1
subtract 1 from score
```

These become normal assignments. Target-first speech such as `name save Mina`
or `이름 저장 민수` is also supported. Numbers and clear expressions remain code;
plain words become text. A saved name is available for later sentence
interpolation and conditions.

Small value changes can also be written without `+`, `-`, or `=`. Use
`score add 1`, `add 1 to score`, or `score increase by 1`; subtraction uses
`subtract 1 from score`.

### Lists

A list is made with `list of` / `목록`, and an empty one with `an empty list` /
`빈 목록`:

```nme
set friends to list of Mina, Ada
친구들은 목록 민수, 지안
set pals to an empty list
할일은 빈 목록
```

Once a name holds a list, the sentences that read it and rearrange it work on
it:

<!-- nme-check: skip — a side-by-side vocabulary listing, not a program. -->
```nme
append Grace to friends
친구들에 지수 넣어
remove Mina from friends
친구들에서 민수 빼
show how many friends
친구들 개수 말해줘
sort friends
친구들 정렬해
reverse friends
친구들 거꾸로 해
shuffle friends
친구들 섞어
show the first of friends
친구들 첫 번째 말해줘
show the last of friends
친구들 마지막 말해줘
show item 2 of friends
친구들 2번째 말해줘
show the total of scores
점수들 합 말해줘
show the biggest of scores
점수들 중 가장 큰 것 말해줘
show friends joined by comma
친구들을 쉼표로 이어 말해줘
show friends joined together
친구들을 붙여 말해줘
```

The separators are `comma`, `space` and `newline` (`쉼표`, `빈칸`, `줄바꿈`), and
`joined together` / `붙여` puts nothing at all between the items, which is how
a row of stars is drawn. A join that names no separator is refused with
`E0233`: `show friends joined` used to print `friends joined` back at you,
which reads like success and is not.

**Items are counted from one.** `the first of friends` is `item 1 of friends`,
and both become `friends[0]`. There is no item 0: writing one is refused with
`E0229` rather than quietly handing back the last item, which is what Python's
`friends[-1]` would do.

Two of these read a list inside a condition:

<!-- nme-check: skip — a side-by-side vocabulary listing, not a program. -->
```nme
if friends contains Mina
만약에 친구들에 민수가 있으면
if friends is empty
만약에 친구들이 비었으면
```

Every one of them needs a name the program **already made a list**. That is
the whole reason `sort out your things`, `the first of many` and
`친구들 이야기를 들었습니다` stay ordinary sentences and print themselves. Using
one on a name that is not a list is refused with `E0231`, so a mistake is a
message and not a program that means something else.

`how many` is also a value and a condition, not only something to show:

<!-- nme-check: skip — a side-by-side vocabulary listing, not a program. -->
```nme
set total to how many friends
총합은 친구들 개수
if how many friends is greater than 3
만약에 친구들 개수가 3보다 크면
```

### Records

A record holds many named values at once, each one under a name of its own.
Python calls it a dictionary.

```nme
set ages to an empty record
put Mina at 90 in ages
show Mina in ages
show how many ages
remove Mina from ages
```

```nme
나이표는 빈 표
나이표에 민수를 90으로 넣어
나이표의 민수 말해줘
나이표 개수 말해줘
나이표에서 민수 빼
```

The word `record` (`표`, and `table` in English) is only the kind of thing
being made **where a value is being saved**. Everywhere else it is a word
somebody wrote: `I keep a record of everything I read` and
`표는 두 장 남았습니다` print themselves.

Most of a record's grammar is spelled **exactly like a list's** — `how many` /
`개수`, `remove` / `빼`, `contains` / `…에 …가 있으면`, `for each` / `…마다
반복해` — and the compiler decides which is meant from the kind the name holds,
never from the wording. That is the point: a reader should not have to remember
which spelling belongs to which container. A record line written on a list is
refused with `E0234`, because appending `Mina를 90` to the list as one piece of
text would be a program nobody wrote.

A record has no order and nothing to add up, so `the total of`, `the biggest
of`, `the first of` and `sort` are list-only and stay refused for a record.

Looping over a record hands back its **names**, exactly as Python does, so the
value is read back inside the loop:

```nme
set ages to an empty record
put Mina at 90 in ages
for each name in ages
    show name in ages
end
```

A name ordinary Python wrote as `ages = {}` is a record to all of these too.

### What is left over

<!-- nme-check: skip — a side-by-side vocabulary listing, not a program. -->
```nme
show the remainder of pile divided by 4
쌓인돌을 4로 나눈 나머지 말해줘
set left to the remainder of pile divided by 4
if the remainder of pile divided by 4 equals 0
만약에 쌓인돌을 4로 나눈 나머지가 0과 같으면
```

The remainder is `%` in Python, and it decides most counting games. It is a
value, so it works in output, in a saved name, and in a condition. The number
being divided by has to be a written number or a name the program already made.

### Working with text

```nme
show the length of name
이름 길이 말해줘
show name in capitals
이름 대문자로 말해줘
show name in small letters
이름 소문자로 말해줘
```

These read any saved name, not only a list. `the length of` gives how many
characters there are; the other two give the same text with its letters
changed. All three are values, so they may be saved or compared as well as
shown.

Text can also be cut into a list, which is the step after reading a file:

```nme
set names to memo split by line
이름들은 메모를 줄마다 나눈 것
set fields to line split by comma
칸들은 줄을 쉼표로 나눈 것
set words to sentence split by space
말들은 문장을 빈칸으로 나눈 것
```

`split by line` / `줄마다` is Python's `splitlines()`, which copes with a file
that ends in a newline and with the Windows line ending. The others cut on the
separator itself: **a comma here is `","` and not `", "`**, because a line read
back out of a file says `Mina,Ada`. What a split saves is a list, so
`how many names` / `이름들 개수` works on it straight away.

And one piece of text can be written over and over:

```nme
set bar to star repeated 5 times
막대는 별표를 5개 붙인 것
show star repeated 20 times
별표를 20번 붙인 것 말해줘
```

`5번` may be written here even though it means *five times* in a counted loop,
because this is a noun phrase closing with `붙인 것` and no loop ever says that.
The text is wrapped in `str(...)` first, so a name holding `3` gives `"33333"`
and not `15`: the sentence asked for five copies, not for arithmetic.

### Repeat

One sentence on one line:

```nme
repeat 3 times and show Again
3번 반복해서 다시 말해줘
3 times 반복해서 mixed 말해줘
3 times Welcome to NME
3번 안녕하세요
```

When the count comes first, the plain words after it are repeated output. This
is the easiest form; add `show`/`말해줘` when you want the meaning to be
visibly explicit.

Several lines use indentation but no colon:

```nme
repeat 3 times
    show First
    둘째 말해줘

3번 반복해
    show mixed
```

`repeat`, `반복`, `반복해`, and `반복해서` may be mixed with `times` or
`번`. The count is any valid Python expression.

A block written with a colon and ordinary indentation takes its branches the
same way, in both languages:

```nme
만약 score > 10:
    print(1)
아니면 만약에 score == 0:
    print(2)
아니면:
    print(3)
```

`skip` / `건너뛰어` and `break` / `멈춰` work inside an indented `3 times:`
block too. Until this release they were left as bare Python names there, so
the program compiled and then raised `NameError`.

A loop with no count at all is written `repeat forever` / `계속 반복해`, and
`break` / `멈춰` is the way out of it:

```nme
repeat forever
    show still going
    break

계속 반복해
    아직 진행 중 말해줘
    멈춰
```

The compact colon form can also use an explicit closing word, so indentation is
optional while you are learning:

```nme
3 times:
show First sentence
둘째 문장 말해줘
end
```

The same flat form works with Korean `3번:` and `끝`. A normal Python `for` or
`if` line with a colon remains Python and keeps Python's usual indentation
rules.

### A block without indentation

Indentation is useful when you are ready for Python, but it is not required
for the first programs. Put `end` (or `끝`) on its own line to close an easy
block. This form also introduces the control flow needed to grow into Python:

```nme
score = 0
while score < 3
show score
score = score + 1
end

if ready and score > 2
show Go
else if score == 0
show Try again
else
show Not yet
end

while ready or waiting
show Still working
break
end
```

`동안`, `만약`, `아니면`, `아니면만약에`, `멈춰`, and `끝` are Korean
spellings of the same ideas. `and`/`그리고` and `or`/`또는` may be mixed in
one condition. A block
may still use ordinary four-space indentation; the explicit `end` form is the
beginner-friendly bridge when indentation is the part that feels hardest.
Spoken Korean can put the loop ending after its subject too, as in
`준비하는동안`, `준비 하는 동안`, or `준비 동안`. The English `while` keyword
may head a Korean sentence condition with the same ending, as in
`while 점수가 3보다 작을 동안`.

### Conditions

Colon-free blocks:

```nme
if ready
    show Go

만약에 이름이 있으면
    안녕하세요 이름 말해줘
```

Inline sentences use `then` or a Korean connecting ending:

```nme
if score is greater than 10 then show You won
만약에 점수가 10보다 크면 성공 말해줘
```

You may also start with the subject when that reads more naturally:

```nme
score is greater than 5 then show high
name exists then show Welcome name
색이 빨강과 같으면 맞아요 말해줘
```

The subject-first form is limited to a clear comparison, existence check, or
unmistakable action body. Ordinary speech such as `Hello then world` remains
prose.

Korean can shorten the comparison ending without changing the meaning:
`이름이 철수면`, `이름이 철수라면`, and `준비가 거짓이면` are accepted. Spoken
particles may be separated too (`이름 이 철수 면`), and a bare subject can use
`준비면` for a truthy condition. A bounded spoken typo such as `있으먄`,
`철수먄`, or `만악에` is recovered when there is only one clear condition.
This form only works as a one-line condition.

Supported sentence comparisons:

| English | Korean | Meaning |
| --- | --- | --- |
| `if name exists` | `만약에 이름이 있으면` | truthy value |
| `if name missing` | `만약에 이름이 없으면` | falsey value |
| `if score equals 10` | `만약에 점수가 10과 같으면` | `==` |
| `if score is not equal to 10` | `만약에 점수가 10과 같지 않으면` | `!=` |
| `if score is greater than 10` | `만약에 점수가 10보다 크면` | `>` |
| `if score is less than 10` | `만약에 점수가 10보다 작으면` | `<` |
| `if score is less than or equal to 10` | `만약에 점수가 10보다 작거나 같으면` | `<=` |
| `if score is greater than or equal to 10` | `만약에 점수가 10보다 크거나 같으면` | `>=` |

`when condition`, `만약 condition`, `만약에 condition`, and the mixed
`if 조건` are all valid. Use the beginner form when a condition needs the full
precision of an arbitrary Python expression.

Logical conditions use normal Python precedence (`and` before `or`):

```nme
if ready and score > 2 then show Go
만약 준비 그리고 점수가 2보다 크면 성공 말해줘
if ready or waiting then show Please wait
```

Parentheses may surround a whole NME condition in an `if` or `while` header,
for example `if (ready and score > 2)`. Keep the header colon-free so NME owns
the line; a valid Python call such as `when(ready and score > 2)` remains
Python byte-for-byte. Korean sentence endings can stay inside the same wrapper,
as in `만약 (점수가 2보다 작으면)`. A comparison ending may also precede a
logical connector inside the wrapper, as in `만약 (점수가 2보다 크면 그리고
준비)`. The same placement works for a Korean `while` ending, as in
`동안 (횟수가 2보다 작을 동안 그리고 준비)`. The connector spellings can be
mixed too, as in `만약 (점수가 2보다 크면 and 준비)`.

Korean NME words can also be valid Python identifiers. For example,
`만약 (준비)` is a valid Python call shape when `만약` is bound, so it stays
byte-identical Python. To make the line an NME block instead, use a spoken
condition ending such as `만약 준비라면`, or include an NME connector such as
`만약 ((준비 그리고 참))`.

### Random without code punctuation

```nme
set die to random number from 1 to 6
show die

set color to pick from red or green or blue
show color
```

These forms use Python's bundled `random` module directly, so a separate
module line is unnecessary.

### A chance in percent

```nme
30% chance show You win

30% chance
    show You win
    score add 1
end

luck is a 30% chance
if luck then show Lucky
```

`30%` means 300 times in a thousand, and the Python says exactly that:
`if __import__("random").randrange(1000) < 300:`. Counting in thousandths
keeps every chance a whole number, so nothing is ever decided by comparing two
floating-point numbers that are nearly equal.

A percentage may name one decimal place (`30.5%`) and nothing finer. `30.25%`
is refused with `E0227` rather than rounded, because a program must never
quietly mean something its writer did not write. Anything outside `0%` to
`100%` is `E0228`; `100%` always happens and `0%` never does.

The same phrase can be written `with a 30% chance`, `a 30% chance`,
`30 percent chance`, or `30% of the time`; Korean also takes `30%의 확률로`
and `확률 30%로`. A percentage on its own is never a chance, so `I am 100%
sure` and `전체의 30%가 왔습니다` stay the ordinary lines they are.

### A story block

```nme
story:
    The door opened slowly.
    Nobody was there.

    A light came on.
end
```

Inside a story block **every line is text**, so a page of prose needs no output
word on every line. Nothing in there is a command: `wait 3 seconds` prints
those words, and so does `if ready`. The rule has no exceptions, because a line
of a story that quietly became a statement would be the worst mistake this
compiler could make.

A blank line prints an empty line. Names saved earlier are still put into the
text, exactly as they are in `show Hello name!`. `slow story:` tells every line
one character at a time, `very slow story:` more slowly still, and `slow story
every 0.2 seconds:` at whatever pace you name. The block closes at `end` / `끝`
or, when you opened it by indenting, where the indentation ends.

`story:` is the first NME form written with a colon. That is deliberate: a bare
`story:` is a syntax error in Python, so claiming it disturbs the
Python-wins rule not at all, and the colon is the shape Python itself uses to
open a block. It also keeps the form well away from ordinary sentences —
`story time` and `tell me a story` carry no colon and stay sentences.

### A named job

A named job gives a piece of program a name, so it can be run later by that
name. Python calls it a function.

```nme
to greet:
    show Hello
    show Nice to meet you
end
do greet
```

```nme
인사하기라는 일:
    안녕하세요 말해줘
    반가워요 말해줘
끝
인사하기 해줘
```

`to`, `do`, `일`, `하기` and `해줘` are among the most ordinary words either
language has, so a job is recognized by **structure and never by a word**. The
header needs the opening `to` — or, in Korean, `라는` on the name and `일` or
`작업` after it — *and* a closing `:` *and* a block underneath. `to be honest`
and `할 일이 많습니다` have none of it.

**Without a block there is no job.** A heading such as `To do:` or
`오늘의 할 일:` prints as the line it is, and there is no one-line form, so a
colon in the middle of a sentence can never open one either.

The line that *runs* a job is gated on something stronger still: the name has
to be one this program already made a job. A Python `def` that takes no
arguments counts, so the three levels mix freely.

A name saved inside a job stays inside it, and an ordinary Python `return`
written in there is accepted, because what the job becomes is a real `def`.

A job may be given **one thing**. The header names it in front of the job
name, and the line that runs the job hands it over the same way:

```nme
to greet someone:
    show Hello someone!
end
do greet with Mina
```

```nme
이름에게 인사하기라는 일:
    안녕하세요 이름! 말해줘
끝
민수에게 인사하기 해줘
```

How many things a job takes is remembered with its name, so running it the
other way round is refused with `E0235` rather than left to become a Python
`TypeError` at run time on a line that looks right. In English the thing it is
given only has to be a plain name — `someone` and `something` are exactly what
a beginner calls it, and the job name in front already carries the check that
keeps a heading from becoming a function.

Sentence grammar has **no job that takes two things** and **no job that hands
something back** yet. Write a Python `def` when you need either — advanced NME
is ordinary Python and passes through untouched.

### Typo and connector recovery

NME action words, logical connectors, and condition connectors accept their documented variants and recover one insertion,
deletion, substitution, or adjacent transposition after Python rejects the
line. A common two-keystroke pattern—one extra/missing character combined
with an adjacent swap—is also accepted when it has one clear action. Examples
include `물어바` → `물어봐`, `말헤` → `말해`, `repaet` → `repeat`, and
`shwoe` → `show`, `thne` → `then` in `if score is greater than 5 thne ...`,
and `그리거` → `그리고`, and `만악에` → `만약에`.

Recovery applies only to these action/connector tokens, never to Python
expressions, strings, or comments. If a repair is not unique or the sentence
has no clear action, NME reports the exact span and a concrete hint instead of
silently guessing.
This bounded rule is intentional: no compiler can safely infer every possible
typo or every human sentence.

## Beginner level

Beginner syntax is compact and exact. It accepts every Python expression and
is useful when sentence interpretation would be ambiguous. Every documented
beginner action has a Korean spelling, and both languages may be mixed.

The `<...>` parts below are placeholders that stand for real values — copy a
line and replace them, rather than running the template itself:

```text
say <Python expression>
말해 <Python 표현식>

ask <name>
ask <name>, <Python prompt expression>
물어봐 <이름>
물어봐 <이름>, <Python 질문 표현식>

save <name> to <value>
저장 <이름> <값>
설정 <이름> <값>

<count> times:
<횟수>번:

when <condition>:
만약 <조건>:

while <condition>
동안 <조건>
break
멈춰
else if <condition>
아니면 만약에 <조건>
아니면만약에 <조건>
else
아니면
end
끝

use random
랜덤 사용
use zero_knowledge
영지식 사용
```

Blocks may contain one inline statement after `:` or several indented lines:

```nme
3 times: say "Hi"
3번:
    말해 "안녕"
    print("advanced Python is fine")
```

Exact lowering:

| NME | Python |
| --- | --- |
| `say value` / `말해 값` | `print(value)` |
| `ask name` / `물어봐 이름` | `name = input()` |
| `ask name, prompt` | `name = input(prompt)` |
| `save name to value` / `저장 이름 값` | `name = value` |
| `count times:` / `횟수번:` | `for _ in range(count):` |
| `when condition:` / `만약 조건:` | `if (condition):` |
| `while condition` / `동안 조건` ... `end` / `끝` | `while (condition):` |
| `break` / `멈춰` | `break` |
| `else if condition` / `아니면 만약에 조건` / `아니면만약에 조건` | `elif (condition):` |
| `else` / `아니면` | `else:` |

Expressions are opaque Python spans. NME validates and copies them; it never
reformats or reimplements Python expressions.

## Advanced level

Advanced NME is Python syntax. Assignments, functions, classes, imports,
exceptions, async code, pattern matching, installed Python packages, and all
other valid Python features work unchanged.

```python
from pathlib import Path

def words(path):
    return Path(path).read_text(encoding="utf-8").split()

for word in words("notes.txt"):
    show word
```

The last line demonstrates that an advanced Python block may contain sentence
NME.

## Versioned bundled modules

Seven beginner modules ship with NME: `random` (dice and picks), `file`
(reading, writing, and JSON), `zero_knowledge` / `영지식` (a Schnorr
proof-of-knowledge reference implementation), `list` / `목록`, `text` / `글자`,
`math` / `수학`, and `date` / `날짜`. Everything is bundled at `0.0.1` except
zero knowledge, which is at `0.0.2`. One `use` line per module is enough;
importing the same module twice is a collision error:

```nme
use random
use file
use zero_knowledge
use list
use text
use math
use date
```

`list`, `text`, `math`, `date` and their Korean names are words people write in
ordinary sentences, so NME only reads one as a module when it stands directly
beside the `use` / `사용` word and no other word is left over on the line.
`get the list of names`, `What is the date today?` and
`장 볼 목록을 사용해 보세요` are sentences, and they print.

`use random latest`, `use latest random`, and `use random version "0.0.1"` are
equivalents, and so are the Korean spellings `랜덤 사용`, `랜덤 사용 최신`,
`최신 랜덤 사용`, and `랜덤 사용 버전 "0.0.1"`. The `file` module accepts the
same forms with `file` / `파일`: `파일 사용`, `파일 사용 최신`, `파일 사용
버전 "0.0.1"`. The zero-knowledge adapter uses `zero_knowledge` / `영지식`
with the same forms, including `영지식 사용 최신`. Strict punctuation-free
English sentence source may use the alias `use zeroknowledge latest`.

`latest` / `최신` selects the newest adapter bundled with the installed NME
compiler. It is local and deterministic, not an uncontrolled network update.
Clear one-edit misspellings such as `use random lates` are recovered. An
unavailable exact version produces an error showing the installed version.

Every spelling exposes both vocabularies:

| English | Korean | Python meaning |
| --- | --- | --- |
| `random_number(a, b)` | `랜덤정수(a, b)` | `random.randint(a, b)` |
| `random_pick(values)` | `랜덤선택(values)` | `random.choice(values)` |
| `shuffle(values)` | `섞기(values)` | `random.shuffle(values)` |
| `random_version` | `랜덤버전` | adapter version string |

| English | Korean | Python meaning |
| --- | --- | --- |
| `file_read(path)` | `파일읽기(path)` | `pathlib.Path(path).read_text()` |
| `file_write(path, text)` | `파일쓰기(path, text)` | `pathlib.Path(path).write_text(text)` |
| `json_load(path)` | `json읽기(path)` | `json.loads(pathlib.Path(path).read_text())` |
| `json_save(path, value)` | `json저장(path, value)` | `pathlib.Path(path).write_text(json.dumps(value))` |
| `file_version` | `파일버전` | adapter version string |

| English | Korean | Python meaning |
| --- | --- | --- |
| `count(values)` | `개수(값들)` | `len(values)` |
| `sort(values)` | `정렬(값들)` | `sorted(values)`, a new list |
| `reverse(values)` | `뒤집기(값들)` | `list(reversed(values))` |
| `remove(values, x)` | `빼기(값들, x)` | a new list without `x` in it |
| `first(values)` | `첫번째(값들)` | `values[0]` |
| `last(values)` | `마지막(값들)` | `values[-1]` |
| `sum(values)` | `합계(값들)` | `sum(values)` |
| `largest(values)` | `최대(값들)` | `max(values)` |
| `smallest(values)` | `최소(값들)` | `min(values)` |
| `list_version` | `목록버전` | adapter version string |

`sort`, `reverse` and `remove` hand back a new list and leave the original
alone. The sentence statements `sort friends`, `reverse friends` and
`remove Mina from friends` change the list itself; both exist because both are
things people mean.

| English | Korean | Python meaning |
| --- | --- | --- |
| `upper(text)` | `대문자(글)` | `str(text).upper()` |
| `lower(text)` | `소문자(글)` | `str(text).lower()` |
| `trim(text)` | `공백없애기(글)` | `str(text).strip()` |
| `split(text, sep)` | `나누기(글, 구분자)` | `str(text).split(sep)` |
| `join(sep, values)` | `합치기(구분자, 값들)` | `str(sep).join(map(str, values))` |
| `replace(text, a, b)` | `바꾸기(글, a, b)` | `str(text).replace(a, b)` |
| `starts_with(text, a)` | `로시작(글, a)` | `str(text).startswith(a)` |
| `length(text)` | `길이(글)` | `len(text)` |
| `text_version` | `글자버전` | adapter version string |

| English | Korean | Python meaning |
| --- | --- | --- |
| `root(x)` | `제곱근(x)` | `math.sqrt(x)` |
| `round_to(x, places)` | `반올림(x, 자리)` | `round(x, places)`; `places` may be left out |
| `pi` | `원주율` | `math.pi` |
| `power(x, y)` | `거듭제곱(x, y)` | `pow(x, y)`; whole numbers stay whole |
| `absolute(x)` | `절댓값(x)` | `abs(x)` |
| `floor(x)` | `내림(x)` | `math.floor(x)` |
| `ceil(x)` | `올림(x)` | `math.ceil(x)` |
| `math_version` | `수학버전` | adapter version string |

| English | Korean | Python meaning |
| --- | --- | --- |
| `today()` | `오늘()` | `date.today().isoformat()`, text such as `"2026-08-19"` |
| `now()` | `지금()` | `datetime.now().strftime("%H:%M")` |
| `year()` | `올해()` | `date.today().year` |
| `month()` | `이번달()` | `date.today().month` |
| `day_of_month()` | `오늘일자()` | `date.today().day` |
| `weekday()` | `요일()` | the weekday's name; see below |
| `days_after(n)` | `며칠뒤(n)` | `(date.today() + timedelta(days=n)).isoformat()` |
| `date_version` | `날짜버전` | adapter version string |

`days_after` counts forwards, so a negative number counts backwards:
`days_after(-1)` is yesterday.

`weekday` and `요일` are the one place in any bundled module where the two
languages hold **different** values. Every other helper hands back a number, a
list, or the writer's own text, and one value serves both names. A weekday name
is a word, and a word has to be in some language: on a Wednesday `weekday()`
answers `Wednesday` and `요일()` answers `수요일`. The name you write chooses
the language of the answer.

The clock is the machine's own clock, and the clock a browser hands Python is
**UTC**. In the browser playground `today()` is therefore today in UTC and
`now()` is the time in UTC, which is not the wall clock of a reader outside
that zone. Dates come back as ISO text (`2026-08-19`), which sorts correctly
and reads the same everywhere.

Every one of these is a plain Python builtin or one call into `math` or
`datetime`, so a program using them runs unchanged in the browser as well as on
a desktop Python.

All bundled adapters reserve their helper names. If one already exists, NME stops
and asks you to rename it instead of silently overwriting your value.

### Schnorr zero-knowledge adapter

The zero-knowledge adapter uses a fixed finite-field group: RFC 3526 MODP
Group 15 (3072-bit safe prime), generator 2, its prime-order subgroup
`q = (p - 1) / 2`, and 256-bit verifier challenges. Secure random values come
from Python's `secrets` module.

| English helper | Korean helper | Meaning |
| --- | --- | --- |
| `zk_secret()` | `영지식비밀만들기()` | create a nonzero secret scalar |
| `zk_public(secret)` | `영지식공개값(비밀값)` | create the public value |
| `zk_nonce()` | `영지식일회값만들기()` | create a one-time prover nonce |
| `zk_commitment(nonce)` | `영지식약속(일회값)` | first Schnorr message |
| `zk_challenge()` | `영지식도전만들기()` | fresh 256-bit verifier challenge |
| `zk_challenge_except(c)` | `영지식다른도전(도전값)` | fresh challenge different from `c` |
| `zk_response(v,a,c)` | `영지식응답(일회값,비밀값,도전값)` | Schnorr response |
| `zk_verify(A,V,c,r)` | `영지식검증(공개값,약속값,도전값,응답값)` | verify the proof transcript |
| `zk_simulated_response()` | `영지식모의응답만들기()` | choose a simulator response |
| `zk_simulated_commitment(A,c,r)` | `영지식모의약속(공개값,도전값,응답값)` | simulate a transcript for a preselected challenge |

Both Korean and English sentence surfaces can remove function punctuation
for the complete proof flow, and each of the thirteen values has a spelling in
both languages:

| English sentence form | Korean sentence form |
| --- | --- |
| `zero knowledge secret make` | `영지식 비밀 만들기` |
| `secret zero knowledge public make` | `비밀로 영지식 공개값 만들기` |
| `zero knowledge nonce make` | `영지식 일회값 만들기` |
| `nonce zero knowledge commitment make` | `일회값으로 영지식 약속 만들기` |
| `zero knowledge challenge make` | `영지식 도전 만들기` |
| `challenge different zero knowledge challenge make` | `도전과 다른 영지식 도전 만들기` |
| `nonce secret challenge zero knowledge response make` | `일회값과 비밀과 도전으로 영지식 응답 만들기` |
| `public commitment challenge response zero knowledge verify` | `공개값과 약속과 도전과 응답으로 영지식 검증` |
| `zero knowledge simulated response make` | `영지식 모의 응답 만들기` |
| `public challenge response zero knowledge simulated commitment make` | `공개값과 도전과 응답으로 영지식 모의 약속 만들기` |
| `public commitment context zero knowledge challenge make` | `공개값과 약속과 문맥으로 영지식 비대화 도전 만들기` |
| `secret context zero knowledge proof make` | `비밀과 문맥으로 영지식 비대화 증명 만들기` |
| `public proof context zero knowledge verify` | `공개값과 증명과 문맥으로 영지식 비대화 검증` |

Until this release the last five English forms did not exist, and an attempt
at one was saved as a **sentence**: `set ok to p c e z zero knowledge verify`
stored a string, so the program ran, checked nothing, and said nothing. See `examples/needmorecoin-sentence.en.nme` for
a strict ASCII-letters/digits/whitespace example and
`examples/zk-schnorr-relay.ko.nme` for the Korean proof flow. The verifier
validates subgroup membership and scalar/challenge ranges before checking the
Schnorr equation.

A stored transcript cannot answer a different fresh challenge. A transcript
for a challenge chosen in advance can be simulated without the secret, which
demonstrates the zero-knowledge property. A *live relay* is different: an
attacker that forwards the verifier's challenge to the real prover can forward
the real response back. Bind authentication to the intended channel/session
when relay resistance matters.

This adapter is a mathematically faithful learning/reference implementation.
CPython big integers are not promised to be constant-time or side-channel
hardened, so use an audited production cryptography implementation for real
credentials, money, or other sensitive systems.

Run `nme modules` or `nme 모듈` to list versions and names. Files are written
next to the program's working folder, so save them in your project folder.
`random` is not suitable for passwords or other security decisions.

## Modules: importing another `.nme` program

A program can import named values from another `.nme` file in the same folder.
The explicit name list is the module's interface — only those names cross the
file boundary, so there is no hidden global state:

```nme
from "helper.nme" import greet, score
show greet
```

The module file defines the values with ordinary NME or Python:

```nme
# helper.nme
greet = "hello"
score = 0
```

`nme run` (and `nme check` / `nme build`) finds `helper.nme` next to the main
program, transpiles it, and makes it importable; module errors surface with
the module's file name. Imports may chain (`helper.nme` can import another
module), the file name must be a Python identifier (`helper.nme`, not
`my-helper.nme` or `shapes.ko.nme`), and two imported modules must not share a
name; that collision is reported as E9028 with a repair suggestion. `nme compile`
does not support module imports yet and reports E9029; use `nme run`, `nme check`,
or `nme build` for a program that imports another `.nme` file. If an imported
file cannot be opened, the CLI reports E9007 and names the module path.

The same import has a sentence spelling in each language, so a program can be
split across files without meeting `from … import …`:

```nme
use greet from "helper.nme"
use greet, score from "helper.nme"
"helper.nme"에서 greet 가져와
"helper.nme"에서 greet, score 불러와
```

The quoted path ending in `.nme` is what makes these lines an import and
nothing else, so `use random` still loads the bundled random module.

Sentence syntax can read and write files without the module line or Python
punctuation. The path is always a quoted string:

```nme
read "notes.txt" into memo
memo read "notes.txt"
memo에 "notes.txt" 읽어서
memo에 "notes.txt" 읽어서 저장해
```

```text
write "hello" to "out.txt"
"out.txt" 파일에 "hello"를 저장해
```

These lower to `pathlib.Path(...).read_text()` / `.write_text(...)` lines, so
the generated Python is the same stdlib the `file` module teaches. Weak
matches such as `read the book` or `write hello` stay plain sentence output
instead of becoming file operations.

## Running a program with arguments

Words typed after the program name go to the program, exactly like
`python program.py ...`:

```sh
nme run greet Mina
nme r dice 6
nme 실행 todo add "buy milk"
```

The program reads them from `sys.argv`: `sys.argv[0]` is the program path,
`sys.argv[1]` the first argument. Options such as `--python` must come
before the file name; everything after it belongs to the program.

## Native backend

A restricted, statically typed core subset can compile straight to native
machine code, independent of CPython. `nme native run hello` compiles to C
with the system C compiler and runs the executable; `nme native build hello
-o hello` keeps the C source and the executable.

The native core covers: boolean, integer, and finite-float values with `+ - * %` arithmetic
(integer modulo; float modulo is rejected); string literals and string
variables with one binary `+` concatenation, `len`, and `==`/`!=` string
comparisons; `while`/`if`/`else`/`else if` over integer, float, and string
comparisons (including `<=`/`>=` and the natural-language "or equal"
  connectors), over integer and finite-float truthiness (`if ready`, `while turns`;
  zero is false), and over boolean literals and bindings (`if ready`, `while ready`;
  `False` is false); boolean equality/inequality; the beginner `times:` loop; `break`;
  logical `and`/`or` conditions use Python precedence and short-circuiting;
  names assigned on every possible fall-through path of an `if`/`else` block
  are available after it, and a branch that returns early or breaks its
  enclosing loop does not need to assign them, including a terminating path
  that contains a nested conditional; functions over integer scalar
  parameters with a required top-level
  integer `return` (recursion works); `say`/`show`/`말해` of
  integers, floats, booleans, and strings. Boolean arithmetic, value changes,
  boolean function arguments/returns, ordinary Python `for` loops, Python
  inline bodies, and inline value changes remain outside the native core.
  Sentence repeats and beginner `times:`/`번:` loops may use one-line NME
  output bodies, and one-line NME `say`/`show`/`말해` or `break` bodies after
  `then`/`그러면` are supported for these control statements and branch chains.
  A one-line `break` must be inside a native loop; otherwise it is rejected with
  `E0102`.
  Sentence repeats also accept sentence one-line `break here` bodies, such as
  `repeat 3 times and break here`.
  Float arithmetic that would produce a
non-finite result stops with a bilingual runtime error. Everything else — input, modules, files,
classes, packages — is rejected with a clear diagnostic and still runs on
CPython with `nme run`. Identifiers that collide with C keywords are rejected,
never renamed. See the [native core reference](native-reference.md) for the
accepted surface and the [native-backend memo](native-backend.md) for the
design and honest measured benchmark.

## Python conversion

`nme convert` safely converts Python into a selected level and language:

```sh
nme convert app.py --level sentence --language ko -o app.nme
```

It rewrites single-value `print`, `input` assignments, `int(input(...))`,
`for _ in range(...)`, `if`, and simple assignments when a
semantics-preserving equivalent exists. Ordinary `import random` remains
advanced Python so an existing variable named `random` can never be silently
overwritten. Other lines remain advanced Python.
See [the conversion guide](converting-python.md).

## Source preservation and diagnostics

- Strings and comments are protected by Python tokenization.
- Valid Python is byte-identical.
- Indentation, blank lines, comments, line endings, and physical line counts
  are preserved.
- NME diagnostics include a plain message, exact caret span, and repair hint.
- Every diagnostic carries a stable error code such as `E0102`, printed next
  to the message as `error[E0102]:`. Read the long Korean explanation with
  `nme ko <CODE>` (English: `nme en <CODE>`); `nme ko` alone lists every code.
  Compiler codes run from `E0001`; command-line errors (missing file, unknown
  command, Python startup) use `E9xxx` and are explained the same way.
- A top-level or inline `return` outside a Python `def` function gets `E0106`
  with a bilingual hint; one-line class suites do not inherit an outer
  function, and valid returns inside functions remain Python.
- A top-level or inline Python `continue` outside a loop gets `E0107` with a
  bilingual hint; one-line function/class suites do not inherit an outer loop,
  while valid `continue` statements inside loops remain Python.
- A Python `break` outside a loop gets `E0102`; one-line function/class suites
  do not inherit an outer loop, while valid `break` statements inside loops
  remain Python. The check also covers controls after an earlier semicolon-
  separated simple statement in the same one-line suite.
- A top-level or inline Python `yield` outside a function gets `E0108`, and
  `await` outside an `async def` function gets `E0109`; valid generator and
  asynchronous function bodies remain Python.
- `yield from` inside an `async def` function gets `E0110`; use `async for`
  there, while ordinary generator functions may keep `yield from` unchanged.
- Python `async for` and `async with` outside an `async def` function get
  `E0111` and `E0112`; valid asynchronous function bodies remain unchanged.
- Python `nonlocal` without an enclosing function gets `E0113`, including in a
  one-line function or class suite. A nested function or class under an outer
  function remains unchanged; CPython separately checks whether the requested
  name is bound in that outer function.
- Python `from ... import *` inside a function or class gets `E0114`, including
  one-line suites and a star import after an earlier semicolon-separated
  statement; import the names explicitly there. Module-level star imports,
  including ones under a module-level conditional, remain unchanged.
- Python does not allow `break`, `continue`, or `return` inside an `except*`
  block; NME reports `E0115`, including when the control follows an earlier
  semicolon-separated statement in the handler. Nested function bodies and
  control flow after the handler suite remain unchanged.
- Python does not allow `yield` inside a list, set, dictionary, or generator
  comprehension; NME reports `E0116`. A plain `yield` expression and a
  generator lambda remain unchanged when Python permits them.
- An `async for` inside a list, set, dictionary, or generator comprehension
  outside an `async def` function gets `E0117`. Move the comprehension into an
  async function; valid async comprehensions remain unchanged.
- An async generator cannot return a value; NME reports `E0118` even when the
  return appears before the first `yield`. One-line Python suites such as
  `async def stream(): yield 1; return 2` use the same function context as a
  normally indented body, while a bare `return` and returns in nested
  functions remain valid.
- A `global` or `nonlocal` declaration after an earlier use or assignment in
  the same scope gets `E0119` or `E0120`, including in one-line suites;
  parameters and annotated targets cannot use either declaration. Put the
  declaration first. Valid module, nested-function, and comprehension scopes
  remain unchanged. Names used in annotations count as uses; f-string
  validation remains CPython's responsibility.
- Generator lambdas remain valid advanced Python: a `yield` inside
  `lambda: ...` is checked against the lambda's own function context.
- An inline body must contain one statement; an inline `else`/`elif` without an
  open condition gets `E0103`. Put branch lines in the same explicit condition
  block before its `end`.
- Independent problems are collected when possible.
- Korean-led CLI commands receive Korean-first explanations and recovery examples
  such as `nme 실행`, `nme 컴파일`, and `nme 설치`; English command invocations
  remain English-only.

## Current limits

- Sentence interpolation recognizes names introduced by simple assignments,
  function parameters, simple Python loop targets, NME input, and sentence
  assignments. Use beginner expressions for unusual dynamic names or
  ambiguous literal words.
- Sentence comparison vocabulary is intentionally small; arbitrary expressions
  and `and`/`or` logic can use the explicit block form or advanced Python.
- The bundled `random`, `file`, and `zero_knowledge` modules have easy module
  syntax in this beta; other Python libraries are used with ordinary `import`.
- The zero-knowledge adapter is a learning/reference implementation, not a
  side-channel-hardened production cryptography library.
- `check` and `build` ask the selected CPython to compile the lowered output;
  they do not run it. Runtime errors still belong to Python.
- `run`, `build`, and `check` require CPython. Optional `compile` requires
  Python, Nuitka, and a platform C compiler.
- Native compilation does not guarantee that every program is faster or
  smaller; benchmark the artifact that matters.
