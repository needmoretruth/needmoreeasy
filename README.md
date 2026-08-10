# NeedMoreEasy (NME)

English | [한국어](README.ko.md)

**Programming that starts easier and grows into Python.** NME is a small
bilingual language layer for beginners. Every valid Python program is already
a valid NME program, while a compact English/Korean vocabulary makes the first
steps shorter and more readable.

Current version: **`0.0.1-beta.1`**

```text
use random

name = "Mina"
pet = random_pick(["cat"])

when name:
    2 times: say f"{name} gets a {pet}!"
```

The same program can use Korean vocabulary and identifiers:

```text
랜덤 사용

이름 = "민아"
동물 = 랜덤선택(["고양이"])

만약 이름:
    2번: 말해 f"{이름}에게 {동물} 추천!"
```

NME transpiles either form to ordinary Python and runs it with CPython. You can
mix NME and Python on any line, use Python packages and tutorials, and move to
plain Python gradually instead of starting over.

> NME is an early beta. Version `1.0.0` is owner-controlled and cannot be
> released without the repository owner's explicit instruction. See the
> [version policy](docs/versioning.md).

## Why NME?

- **English or Korean:** use either vocabulary, even in the same file.
- **Five useful ideas:** show a value, ask for text, repeat, test a condition,
  and use random helpers.
- **All of Python remains:** assignments, lists, functions, classes, imports,
  packages, and Korean identifiers work normally.
- **Safe compatibility:** a line that is valid Python always stays Python,
  byte-for-byte.
- **Friendly errors:** NME mistakes include the exact location and a concrete
  hint; Korean forms receive Korean guidance.
- **Useful tracebacks:** transpilation preserves physical line counts.

## Install from source

Requirements:

- a recent stable Rust toolchain with Cargo;
- Python 3.8 or later; and
- Git, when cloning the repository.

```sh
git clone https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
nme --version
```

Expected version:

```text
nme 0.0.1-beta.1
```

You can also run NME directly from the repository:

```sh
cargo run --quiet -p nme-cli -- run examples/hello.nme
cargo run --quiet -p nme-cli -- run examples/ask.nme
cargo run --quiet -p nme-cli -- run examples/korean.nme
```

## Quick start

Create `hello.nme`:

```text
ask name, "What is your name? "

say f"Hello, {name}!"

3 times:
    say "NME is working."
```

Run it:

```sh
nme run hello.nme
```

Check it without executing Python:

```sh
nme check hello.nme
```

See the generated Python:

```sh
nme build hello.nme
nme build hello.nme -o hello.py
```

## Language at a glance

| English NME | Korean NME | Python meaning |
| --- | --- | --- |
| `say value` | `말해 값` | `print(value)` |
| `ask name` | `물어봐 이름` | `name = input()` |
| `ask name, prompt` | `물어봐 이름, 질문` | `name = input(prompt)` |
| `count times:` | `횟수번:` | `for _ in range(count):` |
| `when condition:` | `만약 조건:` | `if (condition):` |
| `use random` | `랜덤 사용` | import bundled Python random tools |

`times`/`번` and `when`/`만약` accept either an indented body or one
statement after the colon. Expressions are ordinary Python expressions and
are copied exactly as written.

After `use random`:

- `random_number(start, end)` returns an inclusive random integer;
- `random_pick(values)` chooses one item; and
- `shuffle(values)` changes a list's order in place.

`랜덤 사용` provides the equivalent names `랜덤정수`, `랜덤선택`, and
`섞기`, plus the module name `랜덤`. These tools use Python's included
`random` module, so no package needs to be installed.

The golden rule is **Python wins**. For example, `say("hello")`,
`말해("안녕")`, `ask = input`, and `times = 5` are valid Python and remain
unchanged. NME recognizes its easier forms only when Python rejects that line.

Read the [complete language reference](docs/language.md) for exact grammar,
semantics, generated Python, errors, and limits.

## Command line

| Command | Purpose |
| --- | --- |
| `nme run program.nme` | Transpile and run with CPython |
| `nme run program.nme --python python` | Choose another Python command |
| `nme build program.nme` | Print generated Python |
| `nme build program.nme -o program.py` | Write generated Python to a file |
| `nme check program.nme` | Check NME without running it |
| `nme --help` | Show command help |
| `nme --version` | Show the installed NME version |

## Documentation

English is the default documentation language. Korean mirrors are maintained
for every user-facing guide.

| Topic | English | 한국어 |
| --- | --- | --- |
| First program and CLI tutorial | [Getting started](docs/getting-started.md) | [시작하기](docs/getting-started.ko.md) |
| Exact syntax and behavior | [Language reference](docs/language.md) | [언어 레퍼런스](docs/language.ko.md) |
| Versions and release rules | [Versioning](docs/versioning.md) | [버전 정책](docs/versioning.ko.md) |
| Release changes | [Changelog](CHANGELOG.md) | [변경 기록](CHANGELOG.ko.md) |
| Compiler design | [Architecture](docs/architecture.md) | — |

## How it works

```text
.nme source
    → Python-aware tokenization
    → NME recognition (valid Python wins)
    → line-preserving Python source
    → CPython
```

The compiler core is a pure source-to-source function. File access and Python
execution stay in the CLI. See [the architecture](docs/architecture.md) before
changing compiler behavior.

## Contributing

Keep changes focused and verify Rust behavior with:

```sh
cargo fmt --all
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Questions and bug reports are welcome in
[GitHub Issues](https://github.com/needmoretruth/needmoreeasy/issues).

## License

NeedMoreEasy is licensed only under the [Apache License 2.0](LICENSE).
