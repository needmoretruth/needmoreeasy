# NME native core reference

English | [한국어](native-reference.ko.md)

[Home](../README.md) | [Install](install.md) | [Getting started](getting-started.md) | [Tutorial](tutorial.md) | [Language reference](language.md) | [Guides](guides/index.md)

This page describes the currently implemented NME-native core. It is a small,
statically typed AOT path: `nme native` lowers the accepted subset to C and
uses the system C compiler to make an executable. It does not compile all NME
or all Python, and it never silently switches to CPython. For the design
trade-offs, see the [native-backend memo](native-backend.md).

## Six equivalent entry points

These six programs all calculate and show `10`. They use the same frontend and
the same `nme-native` lowering path; only the surface spelling changes.

### Sentence English

```text
score = 5
while score is less than 10
    score add 1
end
show score
```

### Sentence Korean

```text
점수 = 5
동안 점수가 10보다 작을 동안
    점수에 1 더해
끝
점수 말해줘
```

### Beginner English

```text
score = 5
5 times:
    score add 1
show score
```

### Beginner Korean

```text
점수 = 5
5번:
    점수에 1 더해
점수 말해줘
```

### Advanced English

```text
def twice(value):
    return value * 2

show twice(5)
```

### Advanced Korean usage

```text
def 두배(값):
    return 값 * 2

말해 두배(5)
```

Advanced Korean keeps Python keywords such as `def` and `return` unchanged.
Korean identifiers and NME's `말해` output spelling can still be used in the
same native program. All six examples print:

```text
10
```

## Values and expressions

The native core currently supports these values:

- Signed 32-bit integers from `-2147483648` through `2147483647`. Integer
  overflow and integer modulo by zero stop with a bilingual native-runtime
  error instead of relying on C overflow behavior.
- Finite floating-point values represented by C `double`. Float literals must
  be finite. `+`, `-`, and `*` work for floats; `%` is integer-only. Whole
  floats use C-style `%g` output, so `5.0` may print as `5` and `-0.0` keeps
  its sign. Arithmetic that would produce a non-finite result stops with a
  bilingual native-runtime error.
- String literals and string variables. A stored or concatenated value may be
  at most 8191 UTF-8 bytes. Escaped newlines and tabs work, embedded NUL
  characters and ordering comparisons do not. `len` counts Unicode characters.
- Boolean values. `True`/`False` and the sentence spellings `true`/`false`/
  `참`/`거짓` can be assigned to names as a type distinct from integers.
  Boolean names work in truthy conditions and `show` prints them as `True` or `False`. Boolean
  equality and inequality comparisons are supported; boolean arithmetic,
  `add`/`subtract` updates, and boolean function arguments or returns are not.

Arithmetic uses `+`, `-`, `*`, and integer `%`. Comparisons support integers,
finite floats, string equality/inequality, and boolean equality/inequality.
Integer, finite-float, and boolean values may also be used directly as
conditions: zero, `False`, and `false` are false; nonzero values, `True`, and
`true` are true.

NME block conditions can combine supported conditions with `and` and `or`.
They keep Python's precedence (`and` binds more tightly than `or`) and
short-circuit evaluation. The Korean spellings `그리고` and `또는` have the
same behavior. Parentheses may surround a whole colon-free NME condition, but
Python-colon conditions remain outside this native subset.

Use `say`, `show`, or `말해` to output an integer, float, boolean, string, or
supported expression. A native expression may use a literal, a name assigned
earlier, a supported comparison, or a call to a declared native function.

These equivalent bindings cover the three NME levels and both user languages:

| Level | English | Korean |
| --- | --- | --- |
| Sentence | `ready save true` | `준비는 참` |
| Beginner | `set ready to True` | `저장 준비 True` |
| Advanced | `ready = True` | `준비 = True` |

After any of these lines, `show ready`/`말해 준비` prints `True`, and the name
can be used directly in `if` or `while`. Reassigning `False` keeps the same
boolean type.

Run the paired examples with `nme native run examples/native-boolean` and
`nme native run examples/native-boolean.ko`. The logical-condition pair is
available as `examples/native-logical` and `examples/native-logical.ko`.

## Statements and blocks

- `name = expression` creates a typed binding. A later assignment must keep the
  same type; sentence updates such as `score add 1` and `점수에 1 더해` require
  an existing integer or float binding and cannot change a boolean.
- Sentence `while`, `if`, `else`, and `else if` blocks use `end` or `끝`.
  Comparisons may use symbolic operators or the documented natural-language
  connectors, and supported conditions may combine with logical `and`/`or`.
  Beginner `times:` and `번:` loops are supported.
- `break` works inside a native loop, including an `if` nested in that loop.
  A break outside a loop is rejected before C is emitted.
- Bindings created only in a possibly skipped branch are not available before
  they are definitely assigned. A name assigned on every possible fall-through
  path of an `if`/`else` chain is available after the block; a branch that
  returns early or breaks out of its enclosing loop does not need to assign it,
  including a terminating path that contains a nested conditional. A name
  assigned in only one continuing branch, or inside a loop that may not run,
  remains conditional. Function-local bindings stay inside their function.

Use the NME sentence block forms for native control flow. Python-colon control
headers such as `while score < 10:` are outside this core; use the CPython path
for unrestricted Python control flow.

## Functions

Native functions use ordinary Python-style `def` headers, but their native
signature is intentionally small:

```text
def fact(n):
    if n is less than 2
        return 1
    end
    return n * fact(n - 1)

show fact(5)
```

Functions may have zero or more simple positional integer parameters and must
have a top-level integer `return`. An early `return` may terminate one branch;
every path that continues after a control block must still reach the top-level
return. Calls may use a function defined later in the file, including recursive
calls, but must use the declared number of positional arguments. Defaults,
varargs, keyword arguments, nested function definitions, float or string
function values, functions with only branch returns, and top-level `return` are
outside the native core.

For example, the `else` path below returns before the block ends, so `result` is
required only on the path that reaches the final return:

```text
def choose(value):
    if value
        result = 2
    else
        return 3
    end
    return result
```

## What stays on CPython

Input, imports, modules, packages, classes, lists, dictionaries, dynamic
Python values, and the `use random`/`use file` adapters are not part of this
native core. `nme native` reports a bilingual diagnostic for such a program;
it does not produce a possibly-wrong executable. Run the same `.nme` file with
`nme run` when it needs the full Python-compatible language.

## Commands and portability

Run a core program without keeping artifacts:

```sh
nme native run examples/native-factorial
```

Keep the generated C and executable with `build`:

```sh
nme native build examples/native-factorial -o factorial
```

On macOS and Linux NME invokes `cc`; on Windows use a Developer PowerShell for
Visual Studio or another shell where `cl.exe` is on `PATH`. The generated C
receives the platform's UTF-8 option so Korean and English strings retain their
meaning. `run` uses a temporary executable, while `build` preserves the C
source for inspection.

For the complete artifact naming rules, diagnostics, and backend rationale,
see the [native-backend memo](native-backend.md). The [native guide](guides/25-native.md)
walks through the same workflow as a first native project.
