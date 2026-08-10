# NME language reference — 0.0.1-beta.1

English | [한국어](language.ko.md)

This document defines NME's exact syntax and generated Python. For a slower
first-program walkthrough, start with [Getting started](getting-started.md).

## The compatibility rule: Python wins

Every valid Python program is a valid NME program. For each logical source
line, NME first asks a real Python parser whether that line is already Python.
If it is, NME leaves it byte-for-byte unchanged. Only a line Python rejects can
match an NME form.

| Source | Meaning |
| --- | --- |
| `say("hello")` | Python function call, unchanged |
| `말해("안녕")` | Python function call with a Korean name, unchanged |
| `ask = input` | Python assignment, unchanged |
| `times = 5` | Python assignment, unchanged |
| `if times:` | Python `if` header, unchanged |
| `say "hello"` | NME output statement |
| `말해 "안녕"` | the same NME output statement in Korean |
| `5 times:` | NME repetition block |
| `5번:` | the same NME repetition block in Korean |

NME words are therefore not global reserved words. Strings, f-strings,
triple-quoted strings, and comments are tokenized as Python and can never be
mistaken for NME code.

Python supports Unicode identifiers, so ordinary assignments and expressions
can already use Korean names:

```python
이름 = "민아"
좋아하는_수 = 7
친구들 = ["하나", "두리"]
```

## Small bilingual grammar

```text
say <Python expression>
말해 <Python 표현식>

ask <simple name>
ask <simple name>, <Python prompt expression>
물어봐 <간단한 이름>
물어봐 <간단한 이름>, <Python 질문 표현식>

<Python expression> times:
    <statements>
<Python 표현식>번:
    <문장들>

when <Python expression>:
    <statements>
만약 <Python 표현식>:
    <문장들>

use random
랜덤 사용
```

The repetition and condition forms also accept one statement after the colon.
English and Korean spellings can be mixed; they always have identical
semantics.

NME never rewrites expressions. It asks Python whether each expression is
valid, records its byte span, and copies the original text into generated
Python.

## `say` / `말해` — show one value

```text
say <expression>
말해 <표현식>
```

Both forms generate `print(expression)`:

```text
say "Hello!"
말해 "안녕하세요!"
말해 f"2 + 2 = {2 + 2}"
```

```python
print("Hello!")
print("안녕하세요!")
print(f"2 + 2 = {2 + 2}")
```

The part after the keyword must be one valid Python expression. For multiple
print arguments or options such as `sep=` and `end=`, use Python's
`print(...)` directly.

A call such as `say("hello")` or `말해("안녕")` is already valid Python, so
it stays a call to a Python name. NME does not define runtime `say` or `말해`
functions.

## `ask` / `물어봐` — read text

Without a prompt:

```text
ask name
물어봐 이름
```

```python
name = input()
이름 = input()
```

With a prompt:

```text
ask name, "What is your name? "
물어봐 이름, "이름이 뭐예요? "
```

```python
name = input("What is your name? ")
이름 = input("이름이 뭐예요? ")
```

The target must be one simple Python identifier, including a Korean
identifier. The optional prompt after the comma can be any Python expression.
Like Python `input`, the answer is always text. Convert it explicitly when a
number is needed:

```text
ask answer, "Number? "
number = int(answer)
say number + 1
```

`ask(name)`, `물어봐(이름)`, and assignments to those names are valid
Python and remain unchanged.

## `times` / `번` — repeat

### Indented block

```text
5 times:
    say "Hello"

5번:
    말해 "안녕"
```

Both lower to a Python `range` loop:

```python
for _ in range(5):
    print("Hello")
```

Korean `번` may touch the count (`5번:`) or be separated by whitespace
(`5 번:`). The count may be any valid Python expression:

```text
(2 + 3) times:
    say "five"

len(항목들)번:
    말해 "항목 수만큼"
```

### One-line body

```text
3 times: say "Hi"
3번: 말해 "안녕"
2 times: print("ordinary Python")
```

Exactly one statement may follow the colon. A top-level semicolon is rejected;
use an indented block for multiple statements. A one-line body cannot end by
opening another body-less NME block.

The generated loop variable is `_`. Do not rely on its value inside or after
an NME repetition. The count is evaluated once when entering the loop and must
be accepted by Python's `range` at runtime.

## `when` / `만약` — run conditionally

### Indented block

```text
when score >= 10:
    say "You won!"

만약 점수 >= 10:
    말해 "성공!"
```

These generate an ordinary Python `if` (NME adds parentheses so every valid
Python expression remains safe in the header):

```python
if (score >= 10):
    print("You won!")
```

### One-line body

```text
when ready: say "Go!"
만약 준비됨: 말해 "시작!"
```

The condition can be any valid Python expression. The same one-statement and
indentation rules as repetition apply.

NME deliberately has no second spelling for every Python control-flow form.
Use Python `elif` and `else` when needed:

```text
만약 점수 >= 10:
    말해 "성공"
else:
    말해 "다시 도전"
```

This keeps the starter vocabulary small while preserving a clear path into
Python.

## `use random` / `랜덤 사용` — bundled random tools

Python already includes the `random` module. NME exposes a small, memorable
set of aliases with one line and no package installation.

### English names

```text
use random

say random_number(1, 6)
say random_pick(["red", "green", "blue"])

cards = [1, 2, 3]
shuffle(cards)
say cards
```

| Name | Python function | Behavior |
| --- | --- | --- |
| `random` | module | the full Python module |
| `random_number(a, b)` | `random.randint(a, b)` | integer including both ends |
| `random_pick(values)` | `random.choice(values)` | choose one item |
| `shuffle(values)` | `random.shuffle(values)` | reorder a mutable list in place |

### Korean names

```text
랜덤 사용

말해 랜덤정수(1, 6)
말해 랜덤선택(["빨강", "초록", "파랑"])

카드 = [1, 2, 3]
섞기(카드)
말해 카드
```

| 이름 | Python 함수 | 동작 |
| --- | --- | --- |
| `랜덤` | `random` 모듈 | 전체 Python 모듈 |
| `랜덤정수(가, 나)` | `random.randint(a, b)` | 양 끝을 포함한 정수 |
| `랜덤선택(값들)` | `random.choice(values)` | 값 하나 선택 |
| `섞기(값들)` | `random.shuffle(values)` | 변경 가능한 목록을 그 자리에서 섞기 |

The declaration lowers on the same line to an import plus ordinary name
aliases. It is explicit so pure Python stays unchanged. These aliases become
normal variables in the current module and can replace existing names, so put
the declaration near the top and do not reuse those names for other values.

Only `random` has NME's easy `use` form in this beta. Use normal Python
imports for everything else:

```python
import math
from pathlib import Path
```

The random helpers are for games, examples, and simulations, not secrets or
security decisions.

## Mixing English, Korean, and Python

```text
랜덤 사용

def greet(이름):                 # ordinary Python function
    말해 f"안녕하세요, {이름}!"   # Korean NME

for name in ["Ada", "Grace"]:   # ordinary Python loop
    greet(name)

2 times:                        # English NME
    만약 random_pick([True]):   # Korean NME + English helper
        print("Everything mixes")
```

English and Korean keywords are aliases, not separate language modes. No flag
or file declaration is required.

## Source preservation and diagnostics

- Python tokenization decides what is code, a string, or a comment.
- Blank lines, indentation, comments, line endings, and trailing comments are
  preserved.
- Each NME logical line lowers to one Python logical line.
- Generated output has the same physical line count, preserving traceback
  line numbers.
- Malformed NME forms produce a plain message, an exact caret span, and a hint.
- Independent NME problems are collected when possible.
- Korean NME forms use Korean messages and hints.

Example:

```text
text = "3번: 말해 무엇"   # string content, untouched
# when ready: say "go"   # comment, untouched
말해 text                 # NME; trailing comment preserved
```

## Current beta limits

- NME intentionally has only the five concepts described above.
- Inline repetition and conditions accept exactly one statement.
- `ask` reads text; numeric conversion uses ordinary Python.
- Only Python's bundled `random` module has easy aliases.
- General Python syntax and runtime errors still come from CPython.
- `nme check` validates tokenization and NME forms; it is not a replacement
  for all CPython syntax or runtime checks.
- Running requires a CPython interpreter. Building and checking do not.
- Traceback line numbers are preserved, but the temporary `.py` filename may
  be shown instead of the original `.nme` path.

## Exact lowering table

| NME | Generated Python |
| --- | --- |
| `say value` / `말해 값` | `print(value)` |
| `ask name` / `물어봐 이름` | `name = input()` |
| `ask name, prompt` / `물어봐 이름, 질문` | `name = input(prompt)` |
| `count times:` / `횟수번:` | `for _ in range(count):` |
| `when condition:` / `만약 조건:` | `if (condition):` |
| `use random` | import `random` plus English aliases |
| `랜덤 사용` | import `random` plus Korean aliases |

For installation, CLI commands, and a complete first run, see
[Getting started with NME](getting-started.md).
