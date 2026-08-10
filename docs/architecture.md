# NME architecture

This document explains how the NME compiler is built and *why*. It is the
first thing to read before changing anything. The audience is future
contributors.

## What NME is (and is not)

NME is a **thin, easy syntax layer on top of Python**, not a new language
implementation. The compiler turns `.nme` source into ordinary `.py` source;
the real CPython interpreter executes it. NME does not have its own runtime,
evaluator, garbage collector, or standard library — on purpose. Everything
Python can do, NME can do, because NME programs *are* Python programs after
a small, safe rewrite.

## The pipeline

```text
source text (.nme)
  │
  ▼  lexer.rs     rustpython-parser's lexer → logical lines
  ▼  parser.rs    token-pattern matching, "Python wins" rule → NmeStmt list
  ▼  lower.rs     NmeStmt → Python text edits (line-preserving)
  ▼  transpile.rs apply edits → Python source
```

`nme-core` is a **pure function** `&str -> Result<String, Vec<Diagnostic>>`.
No IO, no processes, no global state. All IO (reading files, printing,
running Python) lives in `nme-cli`. This keeps the compiler trivially
testable and reusable (future REPL, LSP, playground, ...).

## Module responsibilities (one job each — keep it that way)

| Module         | Responsibility                                              | May NOT do                            |
| -------------- | ----------------------------------------------------------- | ------------------------------------- |
| `diagnostics`  | describe problems + render them for beginners               | know about tokens or grammar          |
| `lexer`        | group Python tokens into logical lines                      | know about NME syntax                 |
| `syntax`       | define NME's AST and keywords (the language definition)     | any logic beyond data                 |
| `parser`       | decide which logical lines are NME, and which construct     | generate Python text                  |
| `lower`        | turn one NME statement into Python text                     | decide what is NME                    |
| `transpile`    | wire the pipeline together                                   | any logic of its own                  |
| `cli::exec`    | run the transpiled program with the system Python           | transpilation details                 |

## Key design decisions (do not regress these)

### 1. Python wins

A line that is valid Python is **always** treated as Python, even if it
looks like NME. NME only claims lines Python itself rejects. Validity is
decided by `rustpython_parser::parse` — a real Python parser, not a guess.
Two forms are tried: the line alone (`say(x)` is valid) and the line with a
`s pass` body (`if times:` is a valid header). This is what makes "mix
Python and NME in one file" safe by construction.

### 2. Never scan text ourselves

All understanding of Python source comes from `rustpython-parser` tokens.
No regex, no `str::replace`, no hand-rolled string scanning anywhere in the
pipeline. That is why NME-looking text inside strings, triple-quoted
strings, f-strings and comments can never be rewritten by accident — the
tokenizer hands us whole tokens with byte spans, and comments/strings never
appear as statement starts.

### 3. NME expressions are opaque Python

NME never parses, rewrites or re-prints expressions. The parser checks output
values, input prompts, repetition counts, and conditions with
`Mode::Expression`, records their **byte spans**, and lowering copies the
original text verbatim. Every Python expression feature — present and future —
works inside NME statements for free.

### 4. Line-preserving lowering

Every NME statement lowers to Python **on the same single line**, and the
edit span never includes leading indentation or trailing comments. Result:
output has exactly as many lines as input, so CPython tracebacks point at
the line numbers the user wrote, and comments survive untouched. (Verified
by tests, including a real `ZeroDivisionError` traceback line check.)

### 5. Errors are a feature

NME's users are beginners. Diagnostics say what is wrong, where (caret
under the exact span), and what to try instead (`hint`). The compiler
collects **all** problems in one pass instead of stopping at the first.
Anything that is neither valid Python nor valid NME produces a diagnostic —
never silently broken Python output.

### 6. A small bilingual starter set

The first beta has five concepts: output, text input, repetition, conditions,
and an explicit `random` toolkit. English and Korean spellings share the same
AST variants and Python semantics. This is enough for a beginner to make an
interactive program without creating a second standard library or duplicating
Python's general-purpose syntax. Consistency still beats feature count: add a
new concept only when the existing set cannot express a common beginner task
clearly.

The bundled `random` toolkit is a one-line import expansion backed entirely by
Python's standard-library `random` module. It is not a runtime or dependency,
and it remains explicit so pure Python files stay byte-identical.

### 7. No `unsafe`, minimal dependencies

`unsafe_code` is forbidden workspace-wide (`Cargo.toml` `[workspace.lints]`).
The only dependency is `rustpython-parser` — a proven, maintained crate that
saves us from reimplementing Python's lexical rules and grammar. Do not add
dependencies without a clear, present need. The CLI parses its two flags by
hand on purpose; if subcommands grow, adopting `clap` is a reasonable
*future* decision.

## Adding a new NME construct (recipe)

1. **Design the Python meaning first.** The lowering must be one line in,
   one line out, and valid Python 3.8+.
2. Add a variant to `syntax::NmeStmt` (spans, never owned text).
3. Add a `match_*` function in `parser.rs`; call it from `classify`.
   Match on **tokens only**. Reuse the Python-wins checks
   (`is_valid_python_statement` / `is_valid_python_header`) so your
   construct can never hijack valid Python.
4. Add lowering in `lower.rs` (`lower_stmt`).
5. Add beginner-friendly diagnostics (message + hint) for the ways a
   beginner can get the construct wrong.
6. Add tests: both language spellings when applicable, mixed Python/NME,
   look-alike valid Python that must stay untouched, and every error case.
7. Update both language references (`docs/language.md` and
   `docs/language.ko.md`), both READMEs, and an example.

## Execution model

`nme run` writes the transpiled program to a temporary file named after the
source (`<stem>.py`) and runs it with the system Python (`--python` to
override). stdio is inherited; the exit code is the interpreter's. Known
limitation (future work): Python tracebacks show the temporary file *name*
(correct line numbers, though); mapping file names back to `.nme` paths is
a deliberately postponed nicety, e.g. via a linecache hook.

## What is explicitly out of scope (for now)

LLVM/JIT/machine code, a garbage collector, a package manager, a
Python-grammar reimplementation, an NME runtime. If a task seems to require
one of these, the task is off-course.
