# NME language reference (v0.1)

English | [한국어](language.ko.md)

NME is Python plus two easier statement forms. This document defines their
syntax and behavior. It is a reference; if this is your first NME program,
start with the [getting-started tutorial](getting-started.md).

## Core compatibility rule: Python wins

Every valid Python program is a valid NME program. A source line that is valid
Python is always kept as Python, even when it contains the names `say` or
`times`. NME only claims a line when Python rejects that line and it matches an
NME form.

| Source | Interpretation |
| --- | --- |
| `say("hello")` | Python function call, unchanged |
| `say = print` | Python assignment, unchanged |
| `times = 5` | Python assignment, unchanged |
| `if times:` | Python `if` header, unchanged |
| `say "hello"` | NME `say` statement |
| `5 times:` | NME `times` block |

`say` and `times` are therefore not globally reserved words. Strings,
f-strings, triple-quoted strings, and comments are tokenized as Python and
never mistaken for NME code.

## Syntax summary

```text
say <python-expression>

<python-expression> times:
    <statements>

<python-expression> times: <one-statement>
```

Text in angle brackets is ordinary Python source. NME asks a Python parser
whether expressions are valid and copies their spelling without rewriting it.

## `say` — show one value

### Syntax

```text
say <python-expression>
```

### Meaning

```text
say expression
```

is transpiled to:

```python
print(expression)
```

Examples:

```text
say "Hello, world!"
say 1 + 1
say f"2 + 2 is {2 + 2}"
say [name.upper() for name in names]
```

The part after `say` must be one valid Python expression. NME preserves that
expression exactly, including parentheses and string formatting.

`say` prints the value of one expression. Use Python's `print(...)` directly
when you need multiple arguments or options such as `sep=`, `end=`, `file=`,
or `flush=`.

### Python-wins cases

```python
say("hello")  # a call to a Python function named say
say.attr       # Python attribute access
say[0]         # Python subscription
say            # a Python name expression
```

NME does not define a runtime function named `say`, so these examples only run
if the Python name has been defined appropriately. A bare `say` may produce a
Python `NameError` at runtime.

## `times:` — repeat

`times:` passes a count expression to Python's `range` and runs a loop.

### Block form

```text
<python-expression> times:
    <statements>
```

Example:

```text
5 times:
    say "Hello"
    say "Again"
```

Generated Python:

```python
for _ in range(5):
    print("Hello")
    print("Again")
```

The body must contain a following, more-indented code line. Indentation follows
Python's rules and may use the same nesting patterns as Python.

### Inline form

```text
<python-expression> times: <one-statement>
```

Examples:

```text
5 times: say "Hello"
3 times: print("Python works here too")
2 times: 3 times: say "nested"
```

Exactly one statement may follow the colon. A top-level semicolon is rejected:

```text
5 times: say "A"; say "B"  # error
```

Use an indented block for multiple statements:

```text
5 times:
    say "A"
    say "B"
```

An inline `times:` cannot end by opening another block:

```text
2 times: 3 times:  # error: the inner loop has no inline body
```

Indent nested blocks instead, or finish the inline chain with one statement.

### Count expression and runtime behavior

The count may be any valid Python expression:

```text
(2 + 3) times:
    say "five times"

len(items) times: say "one per item"
```

The generated code is `range(expression)`, so the value must be accepted by
Python's `range` at runtime. Zero and negative integers run the body zero times.
The count expression is evaluated once when entering the loop.

NME currently uses `_` as the generated Python loop variable. Avoid relying on
the value of `_` inside or after a `times:` loop.

## Mixing Python and NME

All Python statements and expressions remain available:

```text
import random

def greet(name):
    say f"Hello, {name}!"       # NME inside a Python function

for name in ["Ada", "Grace"]:  # Python loop
    greet(name)

2 times:                        # NME loop
    print(random.random())      # Python inside NME
```

NME has no separate type system, module system, runtime, package manager, or
standard library. Those behaviors are Python's behaviors.

## Whitespace, comments, and source preservation

- Block indentation has the same significance it has in Python.
- Blank lines and comments are preserved.
- Trailing comments on NME statements are preserved.
- CRLF and LF line endings are preserved.
- Text that looks like NME inside any Python string or comment is untouched.
- Transpilation keeps the same number of physical lines, so traceback line
  numbers continue to match the `.nme` source.

For example:

```text
text = "5 times: say something"  # string content, untouched
# 5 times: say "hello"            # comment, untouched
say text                           # NME, trailing comment preserved
```

## Diagnostics

Malformed NME forms and lexical problems produce beginner-oriented
diagnostics with:

1. a plain-language message;
2. a caret under the relevant source span; and
3. a hint showing what to try.

The compiler collects multiple NME problems when possible. Python runtime
errors still come from the selected CPython interpreter, with the original
line numbers preserved.

## Current limits

- v0.1 has exactly two NME constructs: `say` and `times:`.
- Inline `times:` accepts exactly one statement and rejects a top-level
  semicolon.
- An inline statement cannot finish with a block-opening `times:` form.
- Running requires an installed CPython interpreter. Building and checking do
  not.
- Runtime tracebacks preserve source line numbers, but v0.1 may show a
  temporary `.py` file name instead of the original `.nme` path.

These limits are deliberate. NME favors a small, predictable language over a
large set of shortcuts.

## Exact lowering table

| NME | Generated Python |
| --- | --- |
| `say value` | `print(value)` |
| `count times:` | `for _ in range(count):` |
| `count times: say value` | `for _ in range(count): print(value)` |
| `count times: python_statement` | `for _ in range(count): python_statement` |

For CLI commands and a complete first-run walkthrough, see
[Getting started with NME](getting-started.md).
