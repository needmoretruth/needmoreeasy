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

```text
show Hello world!
Hello world show
보여줘 안녕하세요!
안녕하세요 말해줘
Please show me hello
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

If a line is clearly ordinary multi-word speech, NME can print it without an
action word:

```text
Hello everyone!
오늘도 반가워요!
```

A single bare word is still valid Python, so Python wins and it remains an
ordinary name expression. Add `show` or `말해줘` when a one-word line should
print.

The shortest conversation does not need a prompt or punctuation:

```text
name ask
Hello name show
```

For the gentlest possible first input, ask as a normal question:

```text
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

```text
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

```text
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

### Repeat

One sentence on one line:

```text
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

```text
repeat 3 times
    show First
    둘째 말해줘

3번 반복해
    show mixed
```

`repeat`, `반복`, `반복해`, and `반복해서` may be mixed with `times` or
`번`. The count is any valid Python expression.

The compact colon form can also use an explicit closing word, so indentation is
optional while you are learning:

```text
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

```text
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

You may also start with the subject when that reads more naturally:

```text
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

```text
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

```text
set die to random number from 1 to 6
show die

set color to pick from red or green or blue
show color
```

These forms use Python's bundled `random` module directly, so a separate
module line is unnecessary.

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

Three beginner modules ship with NME: `random` (dice and picks), `file`
(reading, writing, and JSON), and `zero_knowledge` / `영지식` (a Schnorr
proof-of-knowledge reference implementation). Random and file are bundled at
`0.0.1`; zero knowledge is bundled at `0.0.2`. One `use` line per module is
enough; importing the same module twice is a collision error:

```text
use random
use file
use zero_knowledge
```

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
for the complete proof flow. English forms include `zero knowledge secret
make`, `secret zero knowledge public make`, `zero knowledge nonce make`, `nonce
zero knowledge commitment make`, `secret context zero knowledge proof make`,
`public proof context zero knowledge verify`, and `public commitment context
zero knowledge challenge make`. See `examples/needmorecoin-sentence.en.nme` for
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

```text
from "helper.nme" import greet, score
show greet
```

The module file defines the values with ordinary NME or Python:

```text
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

Sentence syntax can read and write files without the module line or Python
punctuation. The path is always a quoted string:

```text
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
  with a bilingual hint; valid returns inside functions remain Python.
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
