# NeedMoreEasy (NME)

English | [한국어](README.ko.md)

**If Python still feels hard, start simpler and grow into Python one line at
a time.** NME is a learning bridge: begin with ordinary sentences, mix in
beginner syntax when you are ready, then replace pieces with Python inside the
same file. You never have to restart the project in another language.

Start with the easiest sentence form, add the compact beginner form when you
want more control, and turn one line at a time into ordinary Python. All three
levels can live together, in English, Korean, or a mixture of both.

```text
Hello everyone!
name ask
Hello name show
3 times Welcome to NME
```

Blocks can be flat while you are learning. Close them with `end`/`끝`:

```text
score = 0
while score < 3
show score
add 1 to score
end
```

The same program may freely mix Korean, English, and Python:

```text
이름을 물어봐 이름이 뭐예요?
show Hello 이름!
3번 반복해서 Welcome to NME 말해줘
```

No language-mode declaration is needed.

## Three levels in one language

| Level | Purpose | Example |
| --- | --- | --- |
| Sentence | First-day coding with almost no code punctuation | `3번 반복해서 안녕 말해줘` |
| Beginner | Compact, precise, practical NME | `3 times: say "Hello"` |
| Advanced | Ordinary Python syntax, preserved unchanged | `for i in range(3): print(i)` |

The levels are not separate modes. Use any of them on any line. Valid Python
always wins and is kept byte-for-byte identical.

Sentence syntax understands common connecting words such as `만약에`,
`있으면`, `반복해서`, `그리고`, `또는`, and `then`. It also supports
`while`/`동안`, `break`/`멈춰`, `elif`/`아니면 만약에` (or
`아니면만약에`), and `else`/`아니면`
inside an explicit `end`/`끝` block. It also recovers a one-character typo in
an NME action word after Python has rejected the line, including a common
extra-character-plus-swap typo such as `shwoe` → `show`. A clearly spoken
multi-word line such as `Hello everyone!` is output even without an action;
one bare word remains ordinary Python because Python always wins. When the
meaning is not clear enough to recover safely, NME points at the uncertain text
and suggests a concrete repair. Conditions can start with the subject too:
`score is greater than 5 then show high` or `색이 빨강과 같으면 맞아요 말해줘`.

## Install the beta

NME currently builds from source. Install stable Rust, Python 3.8+, and Git,
then run:

```sh
git clone --branch beta https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"
nme --version
```

The `export` line is required in the current macOS/Linux terminal when Cargo
warns that its `bin` directory is not on `PATH`. It does not reinstall NME.
Windows PowerShell uses the PATH step in the
[installation guide](docs/install.md#windows-10-or-11).

Expected version: `nme 0.0.1-beta.8`.

Windows, macOS, and Linux instructions are in the
[installation guide](docs/install.md). The [five-minute guide](docs/getting-started.md)
starts from zero programming knowledge.

## Use it

```sh
nme run examples/hello-sentence
nme check examples/guessing-game.ko
nme build examples/three-levels -o three-levels.py
nme modules
```

The `.nme` ending is optional. `nme run program` and even `nme program` both
run `program.nme`. NME chooses the normal Python command for your operating
system; `--python` is only an advanced override for unusual setups.

`run` is a development shortcut: NME compiles the file to Python and invokes
CPython. `build` emits the compiled Python source. For a standalone native
artifact, install Nuitka and use:

```sh
python3 -m pip install nuitka
nme compile examples/hello-sentence.nme -o hello
```

Native builds must be made on each target operating system. They can change
startup time, distribution size, and performance, so measure the actual
program; NME does not make a false blanket claim that every Python-compatible
program becomes faster or smaller.

## Versioned random tools

```text
랜덤 사용 최신
show random_number(1, 6)
랜덤선택(["red", "blue"]) 말해줘
```

`random` / `랜덤` adapter version `0.0.1` is bundled, so `latest` / `최신`
resolves locally without a network download. Loading it exposes both Korean
and English helper names, allowing them to mix on the same line. Run
`nme modules` to see installed module versions.

Sentence syntax can use random without any punctuation or prior module line:

```text
set die to random number from 1 to 6
show die
set color to pick from red or green or blue
```

## Convert Python into easier NME

Choose a level and output language:

```sh
nme convert app.py --level advanced --language en
nme convert app.py --level beginner --language ko -o app.nme
nme convert app.py --level sentence --language ko -o app.nme
```

The converter rewrites constructs with a semantics-preserving NME equivalent.
Python constructs without one remain advanced syntax, which is valid NME.
See [Python conversion](docs/converting-python.md).

## Learn and use tools

- [Language reference](docs/language.md) — all three levels, exact meanings,
  typo recovery, mixing, modules, and limitations
- [Learning path](docs/tutorial.md) — Hello World, conversation, number
  guessing, mixed Python, and writing a tiny compiler in NME
- [VS Code, Cursor, and Zed](docs/editors.md) — ready tasks and file setup
- [AI coding assistants](docs/ai-assistants.md) — one link that Claude Code,
  Codex, Cursor Agent, or OpenCode can read before writing NME
- [Compiler architecture](docs/architecture.md) — contributor design rules
- [Version policy](docs/versioning.md) and [changelog](CHANGELOG.md)

## Compiler model

NME is a compiler, not a second Python interpreter. The Rust core performs a
pure source-to-source compilation into ordinary Python. Python tokenization
and parsing come from `rustpython-parser`; execution comes from CPython or the
optional Nuitka native backend. Compilation preserves physical line counts so
traceback line numbers continue to match the `.nme` file.

Licensed under Apache-2.0.
