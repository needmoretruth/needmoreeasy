# Getting started with NME

English | [한국어](getting-started.ko.md)

This tutorial goes from a fresh checkout to writing, checking, transpiling,
and running a small NME program. For exact language rules, use the
[language reference](language.md).

## 1. Install the prerequisites

You need:

- a recent stable Rust toolchain with Cargo;
- Python 3.8 or later; and
- Git, if you clone the repository.

Confirm that the tools are available:

```sh
rustc --version
cargo --version
python3 --version
```

NME uses `python3` by default. On a system where the command is named
`python`, pass `--python python` when running a program.

## 2. Build and install NME

NME is currently installed from source:

```sh
git clone https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
```

Check the installation:

```sh
nme --version
nme --help
```

If `nme` is not on your `PATH`, Cargo commonly placed it in
`$HOME/.cargo/bin`. You can also use NME from the repository without
installing it:

```sh
cargo run --quiet -p nme-cli -- run examples/hello.nme
```

## 3. Write your first program

Create a UTF-8 text file named `hello.nme`:

```text
name = "friend"

say f"Hello, {name}!"

3 times:
    say "Welcome to NME."
```

This example deliberately mixes Python and NME:

- `name = "friend"` and the f-string expression are Python;
- `say` is NME's shorter spelling for showing one value; and
- `3 times:` repeats its indented body.

## 4. Run the program

```sh
nme run hello.nme
```

Expected output:

```text
Hello, friend!
Welcome to NME.
Welcome to NME.
Welcome to NME.
```

To choose a different interpreter command:

```sh
nme run hello.nme --python python
```

`run` inherits standard input, standard output, and standard error from your
terminal. It returns the Python process's exit status.

## 5. See the generated Python

Print the transpiled program:

```sh
nme build hello.nme
```

Write it to a file instead:

```sh
nme build hello.nme -o hello.py
python3 hello.py
```

The output is normal Python. Blank lines, comments, indentation, and line
numbers are preserved. Pure Python source is left byte-for-byte unchanged.

## 6. Check without running

```sh
nme check hello.nme
```

A successful check prints nothing and exits successfully. A failed check
shows every NME problem it can find, with a location and a hint. `check`
validates tokenization and NME transpilation; it does not execute the program
or replace CPython's runtime checks.

For example, this is missing a value after `+`:

```text
say 1 +
```

NME points at the invalid expression and suggests a valid `say` form.

## 7. Mix in any Python you need

NME is not a separate ecosystem. Add ordinary Python whenever you are ready:

```text
def greet(name):
    say f"Hello, {name}!"       # NME inside a Python function

for name in ["Ada", "Grace"]:  # an ordinary Python loop
    greet(name)

2 times:                        # an NME loop
    print("Python works here")  # Python inside NME
```

Valid Python always wins. `say("hello")` is a Python function call, not an
NME `say` statement. `times = 3` and `if times:` are ordinary Python too.

## CLI reference

```text
nme run <file.nme> [--python <command>]
nme build <file.nme> [-o <output.py>]
nme check <file.nme>
nme --help
nme --version
```

### `nme run`

Transpiles the input and starts CPython. The default interpreter command is
`python3`; `--python <command>` overrides it. Traceback line numbers match the
original `.nme` file, although v0.1 may display a temporary `.py` file name.

### `nme build`

Transpiles without executing. With no output option, Python source goes to
standard output. Use `-o <path>` or `--output <path>` to write a file.

### `nme check`

Runs the tokenization and transpilation stages and reports NME diagnostics.
It creates no output file and does not start Python.

## Where to go next

- Read the complete [NME language reference](language.md).
- Run the programs in [`examples/`](../examples/).
- Read the [compiler architecture](architecture.md) before contributing.
