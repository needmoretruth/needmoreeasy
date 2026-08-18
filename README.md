# NeedMoreEasy (NME)

English | [한국어](README.ko.md)

> **Try it right now, with nothing installed — [needmoreeasy.com](https://needmoreeasy.com/).**
> The compiler and a Python engine both run inside the browser, so you write a
> sentence in the box and press Run. It works on a phone. All 90 guides and the
> full syntax list are on that site too.

**If Python still feels hard, start simpler and grow into Python one line at
a time.** NME is a learning bridge: begin with ordinary sentences, mix in
beginner syntax when you are ready, then replace pieces with Python inside the
same file. You never have to restart the project in another language.

Start with the easiest sentence form, add the compact beginner form when you
want more control, and turn one line at a time into ordinary Python. All three
levels can live together, in English, Korean, or a mixture of both.

```text
What is your name?
Hello name!
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

Questions can also be written as ordinary English or Korean. `이름이 뭐예요?`
and `What is your name` create the matching input variable automatically;
the final `?` is optional. Use `ask`/`물어봐` when a question is more complex or
needs a numeric input.

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

```text
set score to 6
score is greater than 5 then show high
색은 "빨강"
색이 빨강과 같으면 맞아요 말해줘
```

## Install the beta (pre-release)

This is the public beta, not a stable 1.0 release. It builds from source.
Install stable Rust, Python 3.8+, and Git, then follow the
[platform installation guide](docs/install.md) or run:

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
[installation guide](docs/install.md#windows-11).

Expected version: `nme 0.0.1-beta.160`.

Windows, macOS, and Linux instructions are in the
[installation guide](docs/install.md). The [five-minute guide](docs/getting-started.md)
starts from zero programming knowledge.

## Use it

```sh
nme run examples/hello-sentence
nme check examples/guessing-game.ko
nme build examples/three-levels -o three-levels.py
nme run examples/guessing-game
nme modules
```

For a larger learning project, compare the same time-loop mystery in
[`time-loop-sentence.nme`](examples/time-loop-sentence.nme),
[`time-loop-beginner.nme`](examples/time-loop-beginner.nme), and
[`time-loop-python.en.nme`](examples/time-loop-python.en.nme). The Korean
twins are [`time-loop-sentence.ko.nme`](examples/time-loop-sentence.ko.nme),
[`time-loop-beginner.ko.nme`](examples/time-loop-beginner.ko.nme), and
[`time-loop-python.nme`](examples/time-loop-python.nme).

For a larger Korean sentence-only project, try the
[`roulette.nme`](examples/roulette.nme) simulator (English twin:
[`roulette.en.nme`](examples/roulette.en.nme)). It combines questions,
conditions, loops, random numbers, and value updates in one beginner-friendly
program.
The matching English companion is [`roulette.en.nme`](examples/roulette.en.nme).

To write a story whose letters arrive one at a time, run
[`story-sentence.en.nme`](examples/story-sentence.en.nme). It clears the screen,
rules a line, centres a title, draws a box, tells its story slowly, times itself
and puts a door on a cooldown — all in sentence syntax. The Korean twin is
[`story-sentence.ko.nme`](examples/story-sentence.ko.nme).

To build a small cryptocurrency with real cryptographic calculations and proof
of work, compare the six `NeedMoreCoin` examples: Korean sentence
[`needmorecoin-sentence.ko.nme`](examples/needmorecoin-sentence.ko.nme), English
sentence [`needmorecoin-sentence.en.nme`](examples/needmorecoin-sentence.en.nme),
Korean beginner [`needmorecoin-beginner.ko.nme`](examples/needmorecoin-beginner.ko.nme),
English beginner [`needmorecoin-beginner.en.nme`](examples/needmorecoin-beginner.en.nme),
Korean advanced [`needmorecoin-advanced.ko.nme`](examples/needmorecoin-advanced.ko.nme),
and English advanced [`needmorecoin-advanced.en.nme`](examples/needmorecoin-advanced.en.nme).
The Korean sentence source is regression-tested to contain only Hangul,
decimal digits, and whitespace; the English sentence source is tested to
contain only ASCII letters, decimal digits, and whitespace. Both tests also
require every non-empty line to be lowered by NME. The
[NeedMoreCoin guide](docs/guides/cryptocurrency.md)
walks through wallets, context-bound transaction proofs, fees, replay-resistant
transaction nonces, SHA-256 proof of work, previous-hash linkage, tamper
detection, full state replay, and supply conservation. This remains a learning
single-process blockchain core, not a P2P network or production cryptocurrency.

For a separate real proof-of-knowledge example rather than a hash/signature
simulation, see [`zk-schnorr-relay.ko.nme`](examples/zk-schnorr-relay.ko.nme)
and its English twin [`zk-schnorr-relay.en.nme`](examples/zk-schnorr-relay.en.nme).

Programs can import named values from other `.nme` files in the same folder — see the [`examples/modules/`](examples/modules/) pair and `from "shapes.nme" import rect, circle` in a main program. The import list is the module interface, so there is no hidden global state.

NME can even write a compiler: [`bootstrap.nme`](examples/bootstrap.nme) transpiles a tiny language to Python and runs it — the seed of self-hosting.

Python packages are ordinary imports inside NME — see the [`birthday.nme`](examples/birthday.nme) countdown that uses the `datetime` package.

Networking and terminal programs are ordinary Python inside NME:
[`http-client.nme`](examples/http-client.nme) fetches a page from a local
server, and [`terminal-menu.nme`](examples/terminal-menu.nme) is a small
menu loop in the terminal.

The `.nme` ending is optional. `nme run program` and even `nme program` both
run `program.nme`. NME chooses the normal Python command for your operating
system; `--python` is only an advanced override for unusual setups.

Shorter commands keep the same meaning: `nme r program` runs, `nme c program`
checks, and `nme b program` builds. With no file name at all, `nme r` runs the
single `.nme` program in the current folder; when several programs are there,
NME lists them and asks which one to run. `nme c` and `nme b` behave the same
way for checking and building. `nme m`, `nme v`, and `nme h` are short forms of
`nme modules`, `nme --version`, and `nme help`. `nme comp program` compiles
with Nuitka, `nme conv app.py` converts Python into NME, and `nme install
requests` installs a Python package with pip.

A core subset of NME can also compile straight to native machine code with `nme native run hello` (boolean, integer, and finite-float values, strings, sentence `while`/`if`/`else`, one-line NME output bodies after `then`, logical `and`/`or`, beginner `times:`, `break`, functions with integer parameters and unconditional integer returns, and `say` — try [`native-factorial.nme`](examples/native-factorial.nme), [`native-boolean.nme`](examples/native-boolean.nme), and [`native-logical.nme`](examples/native-logical.nme) (Korean twins [`native-factorial.ko.nme`](examples/native-factorial.ko.nme), [`native-boolean.ko.nme`](examples/native-boolean.ko.nme), and [`native-logical.ko.nme`](examples/native-logical.ko.nme)); boolean arithmetic and other unsupported features still run on CPython). See the [native core reference](docs/native-reference.md) and [native-backend memo](docs/native-backend.md).

Program names may also be shortened while they stay unique: `nme r gue` runs
`guessing-game.nme`. When several programs match, NME lists them and asks you
to type more of the name instead of guessing.

Every error message carries a stable code such as `E0102` next to it. When a
message is hard to understand, `nme ko E0102` reads the long Korean
explanation (with an English translation) and `nme en E0102` the English one;
`nme ko` alone lists every code.

`run` is a development shortcut: NME compiles the file to Python and invokes
CPython. `build` emits the compiled Python source. For a standalone native
artifact, install Nuitka and use:

```sh
python3 -m pip install nuitka
nme compile examples/hello-sentence.nme -o hello
```

(The install guide adds the optional `[app]` extras.)

Native builds must be made on each target operating system. They can change
startup time, distribution size, and performance, so measure the actual
program; NME does not make a false blanket claim that every Python-compatible
program becomes faster or smaller.

## Versioned random, file, and zero-knowledge tools

```text
랜덤 사용 최신
show random_number(1, 6)
show 랜덤선택(["red", "blue"])
```

`random` / `랜덤` adapter version `0.0.1` is bundled, so `latest` / `최신`
resolves locally without a network download. Loading it exposes both Korean
and English helper names, allowing them to mix on the same line.

Reading and writing files works the same way with `file` / `파일`:

```text
파일 사용 최신
파일쓰기("note.txt", "안녕")
show 파일읽기("note.txt")
점수 = {"이름": "민수", "점수": 3}
json저장("save.json", 점수)
보관 = json_load("save.json")
show 보관["이름"]
```

`file` exposes `file_read`/`파일읽기`, `file_write`/`파일쓰기`,
`json_load`/`json읽기`, and `json_save`/`json저장` on version `0.0.1`.

The `zero_knowledge` / `영지식` adapter is also bundled at version `0.0.1`.
It implements the finite-field Schnorr proof-of-knowledge flow with secure
randomness from Python's `secrets` module and the 3072-bit MODP Group 15
subgroup. The Korean example is written entirely in Korean sentence syntax:

```text
영지식 사용 최신
비밀값은 영지식 비밀 만들기
공개값은 비밀값으로 영지식 공개값 만들기
일회값은 영지식 일회값 만들기
약속값은 일회값으로 영지식 약속 만들기
도전값은 영지식 도전 만들기
응답값은 일회값과 비밀값과 도전값으로 영지식 응답 만들기
검증값은 공개값과 약속값과 도전값과 응답값으로 영지식 검증
```

Run [`zk-schnorr-relay.ko.nme`](examples/zk-schnorr-relay.ko.nme) to see
sender A, receiver B, saved-transcript replay by malicious relay C,
zero-knowledge transcript simulation, and the separate live-relay case.
This is a mathematically faithful learning/reference implementation, not an
audited side-channel-hardened production cryptography library.

Run `nme modules` to see installed module versions.

The zero-knowledge adapter is version `0.0.2` in beta.18 and also provides a
context-bound Fiat-Shamir non-interactive proof. `zk_nizk_prove(secret, context)`
returns a JSON-friendly `[commitment, response]` proof, and
`zk_nizk_verify(public_key, proof, context)` recomputes the SHA-256 challenge
from the Group 15 generator, commitment, public key, and a length-prefixed UTF-8
context under an NME-specific domain tag. A proof therefore fails under a
different context. This does not by itself stop replay in the *same* context;
put a unique request ID or nonce in the context when freshness matters. See
[`zk-nizk-context.ko.nme`](examples/zk-nizk-context.ko.nme) and its English twin
[`zk-nizk-context.en.nme`](examples/zk-nizk-context.en.nme).

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

- [Syntax list](docs/syntax.md) — every accepted spelling and the Python it
  becomes, in one table. Generated from the compiler and verified by compiling it.
- [Prompts to hand to an AI](docs/prompts/README.md) — three documents you paste
  into a chat so the AI can write NME (sentence only / full syntax / with examples).
- [Language reference](docs/language.md) — all three levels, exact meanings,
  typo recovery, mixing, modules, and limitations
- [Learning guides](docs/guides/index.md) — small progressive guides with
  difficulty, prerequisites, topic, and result labels; learn in order or look
  up a topic
- [Build NeedMoreCoin](docs/guides/cryptocurrency.md) — compare one cryptocurrency
  across six syntax/language variants and follow wallets, transaction proofs,
  proof of work, and full-chain validation
- [How to write strong NME examples](docs/guides/example-authoring.md) — repository
  rules for learning goals, syntax levels, failure cases, regression tests, and review
- [Example template](docs/guides/example-template.md) — design card, six-file
  skeleton, guide outline, and regression-test starter
- [Learning path](docs/tutorial.md) — seven projects: Hello World, conversation,
  number guessing, mixed Python, the time-loop game, and a tiny compiler
- [VS Code, Cursor, and Zed](docs/editors.md) — ready tasks and file setup
- [AI coding assistants](docs/ai-assistants.md) — one link that Claude Code,
  Codex, Cursor Agent, or OpenCode can read before writing NME
- [Compiler architecture](docs/architecture.md) — contributor design rules
- [Native core reference](docs/native-reference.md) — the exact v0 values, statements, functions, and six surface examples
- [Native backend research](docs/native-backend.md) — the implemented v0 NME-native C backend and roadmap for extending its restricted subset
- [Version policy](docs/versioning.md) and [changelog](CHANGELOG.md)

## Compiler model

NME is a compiler, not a second Python interpreter. The Rust core performs a
pure source-to-source compilation into ordinary Python. Python tokenization
and parsing come from `rustpython-parser`; the Python-compatible path runs on
CPython, while `nme compile` can optionally invoke Nuitka. The separate
`nme native` command compiles its restricted NME subset to C and then to an
executable with the system C compiler. Compilation preserves physical line
counts so traceback line numbers continue to match the `.nme` file.

Licensed under Apache-2.0.
