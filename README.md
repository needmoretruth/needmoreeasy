# NeedMoreEasy (NME)

English | [한국어](README.ko.md)

**Programming, easier than Python.** NeedMoreEasy is a tiny language layer for
people who find even Python difficult. Every valid Python program is already a
valid NME program; NME adds just two beginner-friendly forms:

```text
5 times:
    say "Hello!"
```

NME turns that source into ordinary Python and runs it with CPython:

```python
for _ in range(5):
    print("Hello!")
```

Python and NME can be mixed freely in the same `.nme` file. There is no custom
runtime or separate standard library to learn.

> **Project status:** NME is an early v0.1 foundation. Its intentionally small
> language currently consists of `say` and `times:`. Expect development from
> source rather than a packaged release.

## Why NME?

- **Start smaller:** use `say` and `times:` before learning Python's longer
  spellings.
- **Keep all of Python:** imports, functions, classes, packages, and tutorials
  continue to work.
- **Safe compatibility:** valid Python always wins and is never reinterpreted
  as NME.
- **Friendly failures:** NME errors include a source span and a concrete hint.
- **Useful tracebacks:** transpilation preserves line numbers.

## Requirements

- A recent stable Rust toolchain with Cargo, to build NME
- Python 3.8 or later, to use `nme run`

`nme build` and `nme check` do not start Python. The `run` command uses
`python3` by default and accepts a different command with `--python`.

## Install from source

```sh
git clone https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
nme --version
```

Cargo installs the `nme` executable into its binary directory, commonly
`$HOME/.cargo/bin`. If your shell cannot find `nme`, add that directory to
`PATH` or run the project directly:

```sh
cargo run --quiet -p nme-cli -- run examples/hello.nme
```

## Quick start

Create `hello.nme`:

```text
name = "NME"

say f"Hello from {name}!"

3 times:
    say "This is easy."
```

Then run it:

```sh
nme run hello.nme
```

```text
Hello from NME!
This is easy.
This is easy.
This is easy.
```

## Language at a glance

| NME source | Python meaning |
| --- | --- |
| `say <expression>` | `print(<expression>)` |
| `<expression> times:` plus an indented body | `for _ in range(<expression>):` |
| `<expression> times: <statement>` | the same loop with one inline statement |
| Any valid Python | left as Python, byte-for-byte |

The golden rule is **Python wins**. For example, `say("hello")` remains a
normal Python function call, `times = 5` remains an assignment, and
`if times:` remains a Python `if` header. NME only recognizes its easier forms
when Python would not accept the same line.

Read the [language reference](docs/language.md) for the exact syntax,
semantics, compatibility rule, and current limits.

## Command-line usage

| Command | Purpose |
| --- | --- |
| `nme run program.nme` | Transpile and run with CPython |
| `nme run program.nme --python python` | Run with a different Python command |
| `nme build program.nme` | Print generated Python to standard output |
| `nme build program.nme -o program.py` | Write generated Python to a file |
| `nme check program.nme` | Check tokenization and NME transpilation without running |
| `nme --help` | Show command help |

Try the included programs:

```sh
nme run examples/hello.nme
nme run examples/mixed.nme
nme run examples/pure_python.nme
```

## Documentation

| Topic | English | 한국어 |
| --- | --- | --- |
| First program and CLI tutorial | [Getting started](docs/getting-started.md) | [시작하기](docs/getting-started.ko.md) |
| Complete NME syntax | [Language reference](docs/language.md) | [언어 레퍼런스](docs/language.ko.md) |
| Compiler design and invariants | [Architecture](docs/architecture.md) | — |

The architecture document is contributor-facing and remains in English so
there is one canonical description of the compiler's invariants.

## How it works

```text
.nme source
    → Python-aware tokenization
    → NME statement recognition (valid Python wins)
    → line-preserving Python source
    → CPython
```

The compiler core is a pure source-to-source function and performs no IO.
The CLI owns file access and Python execution. See
[docs/architecture.md](docs/architecture.md) for the design rationale.

## Repository layout

```text
crates/nme-core/   pure NME-to-Python compiler
crates/nme-cli/    nme command, file IO, and Python execution
docs/              user documentation and compiler architecture
examples/          runnable NME and mixed Python/NME programs
```

## Contributing

Read [AGENTS.md](AGENTS.md) and [docs/architecture.md](docs/architecture.md)
before changing code. Every change must keep the project small and pass:

```sh
cargo fmt --all
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Questions and bug reports are welcome in
[GitHub Issues](https://github.com/needmoretruth/needmoreeasy/issues).

## License

NeedMoreEasy is licensed only under the
[Apache License 2.0](LICENSE).
