# NME architecture

This is the canonical design document for contributors. Read it before
changing compiler behavior.

## Product model

NME is one language with three syntax levels that may be mixed line by line:

1. **Advanced** — ordinary Python. Every valid Python program is valid NME.
2. **Beginner** — compact NME forms such as `say`, `ask`, and `3 times:`.
3. **Sentence** — conversational English or Korean without required quotes,
   commas, braces, or block colons for the supported beginner tasks. Explicit
   `end`/`끝` blocks provide `while`, `break`, `elif`, `else`, and `and`/`or`
   without making a first learner manage indentation.

English and Korean are two spellings of the same AST and may also be mixed.
NME transpiles all claimed syntax to ordinary Python; CPython remains the
runtime and standard library. NME does not implement a second evaluator,
garbage collector, or general-purpose standard library.

## Compiler pipeline

```text
source text (.nme)
  │
  ▼  lexer.rs     rustpython-parser tokens → logical lines
  ▼  parser.rs    Python-wins check + token patterns → NmeStmt list
                    + virtual indentation for explicit end blocks
  ▼  lower.rs     NmeStmt → same-line Python edits
  ▼  transpile.rs apply edits → Python source
```

`nme-core` stays pure: its main compiler is `&str -> Result<String,
Vec<Diagnostic>>`. `nme-cli` owns file IO, process execution, and optional
native compilation.

Python conversion is a second pure path:

```text
valid Python
  │
  ▼  convert.rs   safe token patterns → requested NME surface level
  ▼               unsupported lines remain advanced Python
```

The converter is deliberately partial and lossless. Since advanced Python is
already NME, retaining a line is always safer than guessing at a conversational
rewrite.

## Module responsibilities

| Module | Responsibility | Must not do |
| --- | --- | --- |
| `diagnostics` | problem data and beginner-friendly rendering | know grammar |
| `lexer` | Python tokens and logical lines | decide NME meaning |
| `syntax` | AST, shared language data, bundled module versions | perform IO |
| `parser` | Python-wins classification and NME token matching | emit Python text |
| `lower` | same-line Python replacements | decide which syntax matched |
| `transpile` | compiler pipeline glue | contain language policy |
| `convert` | safe Python-to-NME surface conversion | invent semantics |
| `cli::exec` | Python execution and optional native backend | parse NME |

## Invariants

### 1. Python always wins

A valid Python statement or compound header is copied byte for byte, even when
its names resemble NME words. The bundled parser decides known Python grammar
with `rustpython_parser::parse`, including the synthetic `pass` body needed to
test headers; conservative newer-grammar shapes are preserved by the fallback
described below. Every new matcher must stay behind these checks.

The one intentional-looking case is colon-free `if condition`: it is invalid
Python and can therefore be claimed by sentence NME. Normal `if condition:` is
advanced Python and remains untouched.

### 2. Tokens define structure

All structural understanding comes from `rustpython-parser` tokens and byte
spans. Never use regexes, global replacement, or a hand-written Python scanner.
Strings, comments, f-strings, and NME-looking text inside them remain sacred.

Sentence punctuation has one narrow lexer exception. Python reports `?` and
`!` as unrecognized tokens, so `lexer.rs` preserves those exact reported spans
as synthetic punctuation tokens. Only an already-recognized sentence matcher
may consume them. An unrelated invalid line still produces a diagnostic.

### 3. Three levels share semantics

Beginner expressions are validated as opaque Python expressions and copied by
span. Sentence forms instead build the same small AST variants (`Say`, `Ask`,
`Set`, `Update`, `Times`, `When`, `While`, `ElseIf`, `Else`, `Break`, `End`,
`UseRandom`) from explicit token templates. A sentence repeat may use plain
words after its count (`3번 안녕하세요`), and a small value change may use
`score add 1` or `점수에 1 더해`; both lower through the same AST path. Known
variable names may be interpolated into sentence output; unknown words remain
literal text. Subject-first conditions such as `color equals red then show yes`
use the same `When` node as `if`/`만약` forms. Both languages lower through the
same code path.

Do not add a parallel Korean runtime, duplicate AST variants per language, or
different behavior for equivalent English and Korean forms.

### 4. Friendly tolerance is bounded

The sentence parser accepts documented connecting words, common Korean
particles, and a single insertion, deletion, substitution, or adjacent
transposition in action words and condition connectors. It also accepts the
bounded common pattern of one extra/missing character plus an adjacent swap
when the action candidate is unique. Recovery only runs after Python rejects
the line and only when the surrounding token pattern identifies one
construct. Clearly multi-word prose can be output directly; a single bare
word remains Python because of the Python-wins rule.

Unlimited typo correction would silently change programs. When more than one
meaning is plausible, emit an exact caret diagnostic and an actionable hint.
Never guess across expressions, identifiers, numbers, or arbitrary prose.

When the bundled Rust parser does not yet know a newer CPython construct (for
example, a Python 3.14 t-string), conservative future-Python token shapes are
left byte-identical. The CLI then asks the selected CPython to validate them;
the core never claims that its own parser covers every future Python grammar.

### 5. Lowering preserves lines

Every claimed logical line becomes exactly one Python line. Edits exclude
indentation, line endings, and trailing comments. This keeps CPython traceback
line numbers aligned with the `.nme` source. Multi-line runtime helpers are not
allowed in lowering; use one-line expressions or explicit imports. For an
explicit `end`/`끝` block, the parser records a virtual indentation level and
the transpiler inserts only that leading prefix on affected lines. A plain
Python line inside the block receives the same prefix, so the source can stay
flat while generated Python remains syntactically nested.

### 6. Errors are part of the language

Every user-facing compiler error needs a plain message, an exact caret span,
and a useful `hint`. Collect independent errors in one pass. Input that is
neither valid Python nor an unambiguous NME form must never be emitted as
silently broken Python.
Python context errors use the shared parser too: top-level, inline, and one-line
function/class `return`, `yield`, `await`, `break`, and `continue` receive
stable diagnostics when their enclosing function or loop context is invalid,
and `yield from` in `async def` receives `E0110`, while
`async for`/`async with` outside `async def` receive `E0111`/`E0112`, and
`nonlocal` without an enclosing function receives `E0113`; valid nested
function/class bodies and generator lambdas remain byte-identical. CPython
still owns validation of whether a requested `nonlocal` name is bound in an
outer function. A valid module-level `from ... import *` remains unchanged,
while the same star import inside a function or class receives `E0114`.
`break`/`continue`/`return` inside an `except*` suite receive `E0115`; the
tracker resets across nested Python function/class scopes and after the suite.
`yield` inside a comprehension receives `E0116`; token-depth matching keeps
ordinary `yield` expressions and lambdas nested inside comprehensions intact.
An `async for` inside a comprehension outside an `async def` receives `E0117`;
the same token-depth path distinguishes it from an ordinary `async for` header.
An async generator's value-bearing `return` receives `E0118`; the parser
tracks direct yields and defers the decision so a return before the first yield
is diagnosed without inheriting nested function or class scopes. One-line
Python function suites use their body tokens as the same function context, so
valid inline `yield`, `await`, and bare `return` statements are preserved.
Conflicting `global`/`nonlocal` declarations receive `E0119`/`E0120`; the
scope tracker covers one-line function/class suites, excludes nested function
parameters and comprehension-local names, and rejects annotated targets where
Python does so. Valid Python remains byte-identical. Annotation expressions
count as name uses for this check, while f-string contents remain opaque to the
NME token layer and are validated by CPython.

### 7. Bundled modules are local and versioned

`use random latest` / `랜덤 사용 최신` resolves to the newest random helper
shipped in this NME binary. An exact supported version may be requested. The
version constant lives in `syntax.rs`; both English and Korean aliases are
always exposed so languages can be mixed after one import.

This is a deterministic bundled-module registry, not a network package
manager. Adding a module requires a present beginner use case, a fixed version,
both language aliases, diagnostics, tests, and bilingual documentation.

### 8. `.nme` modules import by explicit interface

`from "helper.nme" import name1, name2` imports only the listed names from a
sibling `.nme` file. The CLI transpiles imported modules (transitively) into a
temporary folder and adds that folder to `sys.path` through an environment
variable, so the transpiled `from helper import name1` is ordinary Python
importing an ordinary Python module. Each invocation owns its staging folder;
the folder is removed after execution and also when a module write fails. The
explicit name list is the module boundary: nothing else leaks between files,
and there is no shared global state. Module file names must be valid Python
identifiers because the generated import uses the file stem.

### 8. Small and safe Rust

`unsafe` is forbidden workspace-wide. `rustpython-parser` is the only Rust
dependency because it prevents an unsafe reimplementation of Python syntax.
Add no dependency or abstraction without a current, demonstrated need.

## Adding syntax

1. Define one-line Python semantics first.
2. Add or reuse a language-neutral `NmeStmt` variant in `syntax.rs`.
3. Add a token-only matcher in `parser.rs`, after Python-wins checks.
4. Lower it in `lower.rs` without changing line count.
5. Add message, caret, and hint diagnostics for every failure form.
6. Test the construct, English/Korean equivalence, all-level mixing,
   look-alike valid Python, preservation, and errors.
7. Update both language references, READMEs, and a runnable example.

## Execution and build model

- `nme run` sends compiled source to the selected CPython without a temporary
  user file, while preserving the original `.nme` path, imports, resources,
  arguments, standard input, and traceback names.
- `nme build` asks CPython to validate the generated source before printing or
  writing readable Python. It never creates the output file after a failed
  syntax check.
- `nme compile` is an optional CLI adapter for installed Nuitka. It can create
  a standalone executable, but it does not change NME semantics and does not
  make universal speed or size guarantees. Results depend on the program,
  platform, compiler, and packaging mode.
- `nme native run`/`nme native build` is the NME-native AOT backend
  (`nme-native` crate): it compiles a restricted, statically typed core
  subset to C and then to a native executable with the system C compiler.
  Its current numeric policy is checked signed 32-bit integer arithmetic,
  finite C `double` literals, and file-scope integer functions with
  unconditional returns only; overflow and
  modulo-by-zero are explicit native runtime errors rather than undefined C
  behavior.
  Everything outside the documented core is rejected with a clear bilingual
  diagnostic and remains runnable on CPython. See
  [the native-backend memo](native-backend.md).

Traceback line numbers and displayed file names both point to the original
`.nme` source.

## Out of scope

- replacing CPython or reimplementing the full Python grammar;
- silently rewriting ambiguous prose or every possible typo;
- a network package registry or floating remote dependency resolution;
- claims that every native build is faster or smaller;
- a JIT, a new garbage collector, or an NME-specific general runtime. The
  native backend targets only its documented core subset; it is not a
  Python reimplementation.

NME can still be used to *write* compilers, including a small NME-to-Python
translator, because advanced Python and all installed Python libraries remain
available inside an NME file.
