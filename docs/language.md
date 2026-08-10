# NME language reference

English | [한국어](language.ko.md)

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

```text
show Hello world!
Hello world show
보여줘 안녕하세요!
안녕하세요 말해줘
```

These print literal text. A name created earlier by an input or sentence
assignment is inserted automatically:

```text
ask name What is your name?
show Hello name!

이름을 물어봐 이름이 뭐예요?
안녕하세요 이름! 말해줘
```

The result uses the value of `name` / `이름`; other words stay literal.
Korean particles following a known name remain in the output.

Accepted output actions include `show`, `display`, `tell`, `say`, `보여줘`,
`말해줘`, `말해주세요`, `출력해`, and `출력해줘`. The precise beginner
spellings `say expression` and `말해 표현식` continue to treat a valid Python
expression as code.

### Ask for text or a number

```text
ask name What is your name?
이름을 물어봐 이름이 뭐예요?

ask number age How old are you?
나이를 숫자로 물어봐 몇 살인가요?
```

Natural prompts receive a separating space automatically. Text input compiles
to `input(...)`; number input compiles to `int(input(...))`.

Accepted actions include `ask`, `prompt`, `물어봐`, `물어봐줘`, `질문해`, and
`입력받아`. Korean target particles `을` and `를` are removed from the variable
name.

### Save a value

```text
인사는 안녕하세요
정답은 7
set greeting to Hello
set answer to 7
```

These become normal assignments. Numbers and clear expressions remain code;
plain words become text. A saved name is available for later sentence
interpolation and conditions.

### Repeat

One sentence on one line:

```text
repeat 3 times and show Again
3번 반복해서 다시 말해줘
3 times 반복해서 mixed 말해줘
```

Several lines use indentation but no colon:

```text
repeat 3 times
    show First
    둘째 말해줘

3번 반복해
    show mixed
```

`repeat`, `반복`, `반복해`, and `반복해서` may be mixed with `times` or
`번`. The count is any valid Python expression.

### Conditions

Colon-free blocks:

```text
if ready
    show Go

만약에 이름이 있으면
    안녕하세요 이름 말해줘
```

Inline sentences use `then` or a Korean connecting ending:

```text
if score is greater than 10 then show You won
만약에 점수가 10보다 크면 성공 말해줘
```

Supported sentence comparisons:

| English | Korean | Meaning |
| --- | --- | --- |
| `if name exists` | `만약에 이름이 있으면` | truthy value |
| `if name missing` | `만약에 이름이 없으면` | falsey value |
| `if score equals 10` | `만약에 점수가 10과 같으면` | `==` |
| `if score is greater than 10` | `만약에 점수가 10보다 크면` | `>` |
| `if score is less than 10` | `만약에 점수가 10보다 작으면` | `<` |

`when condition`, `만약 condition`, `만약에 condition`, and the mixed
`if 조건` are all valid. Use the beginner form when a condition needs the full
precision of an arbitrary Python expression.

### Random without code punctuation

```text
set die to random number from 1 to 6
show die

set color to pick from red or green or blue
show color
```

These forms use Python's bundled `random` module directly, so a separate
module line is unnecessary.

### Typo and connector recovery

NME action words accept their documented variants and recover one insertion,
deletion, substitution, or adjacent transposition after Python rejects the
line. Examples include `물어바` → `물어봐`, `말헤` → `말해`, and `repaet` →
`repeat`.

Recovery applies only to action tokens, never to Python expressions, strings,
or comments. If a repair is not unique or the sentence has no clear action,
NME reports the exact span and a concrete hint instead of silently guessing.
This bounded rule is intentional: no compiler can safely infer every possible
typo or every human sentence.

## Beginner level

Beginner syntax is compact and exact. It accepts every Python expression and
is useful when sentence interpretation would be ambiguous.

```text
say <Python expression>
말해 <Python 표현식>

ask <name>
ask <name>, <Python prompt expression>
물어봐 <이름>
물어봐 <이름>, <Python 질문 표현식>

<count> times:
<횟수>번:

when <condition>:
만약 <조건>:

use random
랜덤 사용
```

Blocks may contain one inline statement after `:` or several indented lines:

```text
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
| `count times:` / `횟수번:` | `for _ in range(count):` |
| `when condition:` / `만약 조건:` | `if (condition):` |

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

The easy random adapter has version `0.0.1`.

```text
use random
use random latest
use latest random
use random version "0.0.1"

랜덤 사용
랜덤 사용 최신
최신 랜덤 사용
랜덤 사용 버전 "0.0.1"
```

`latest` / `최신` selects the newest adapter bundled with the installed NME
compiler. It is local and deterministic, not an uncontrolled network update.
An unavailable exact version produces an error showing the installed version.

Every spelling exposes both vocabularies:

| English | Korean | Python meaning |
| --- | --- | --- |
| `random_number(a, b)` | `랜덤정수(a, b)` | `random.randint(a, b)` |
| `random_pick(values)` | `랜덤선택(values)` | `random.choice(values)` |
| `shuffle(values)` | `섞기(values)` | `random.shuffle(values)` |
| `random_version` | `랜덤버전` | adapter version string |

Run `nme modules` or `nme 모듈` to list versions. Random is not suitable for
passwords or other security decisions.

## Python conversion

`nme convert` safely converts Python into a selected level and language:

```sh
nme convert app.py --level sentence --language ko -o app.nme
```

It rewrites single-value `print`, `input` assignments, `int(input(...))`,
`for _ in range(...)`, `if`, simple assignments, and `import random` when a
semantics-preserving equivalent exists. Other lines remain advanced Python.
See [the conversion guide](converting-python.md).

## Source preservation and diagnostics

- Strings and comments are protected by Python tokenization.
- Valid Python is byte-identical.
- Indentation, blank lines, comments, line endings, and physical line counts
  are preserved.
- NME diagnostics include a plain message, exact caret span, and repair hint.
- Independent problems are collected when possible.
- Korean-led forms receive Korean guidance.

## Current limits

- Sentence interpolation recognizes names introduced by simple assignments,
  function parameters, simple Python loop targets, NME input, and sentence
  assignments. Use beginner expressions for unusual dynamic names or
  ambiguous literal words.
- Sentence comparison vocabulary is intentionally small; arbitrary logic uses
  `when expression:` / `만약 표현식:` or advanced Python.
- Only the bundled random adapter has easy module syntax in this beta.
- `check` validates NME tokenization and forms; CPython still owns full Python
  syntax and runtime errors.
- `run` requires CPython. `build` and `check` only need NME. Optional
  `compile` requires Python, Nuitka, and a platform C compiler.
- Native compilation does not guarantee that every program is faster or
  smaller; benchmark the artifact that matters.
