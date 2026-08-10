# Getting started with NME

English | [한국어](getting-started.ko.md)

This guide assumes no NME experience. You will install the first beta, write a
small interactive program, use conditions and repetition, try bundled random
tools, and see how ordinary Python fits in.

## 1. Prepare the tools

You need:

- a recent stable Rust toolchain with Cargo;
- Python 3.8 or later; and
- Git, when cloning the repository.

Check them:

```sh
rustc --version
cargo --version
python3 --version
```

NME uses `python3` by default. If your command is `python`, use
`nme run program.nme --python python`.

## 2. Install NME

```sh
git clone https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
```

Confirm the first beta:

```sh
nme --version
```

```text
nme 0.0.1-beta.1
```

If the shell cannot find `nme`, Cargo commonly installed it in
`$HOME/.cargo/bin`. You can also run from the checkout without installing:

```sh
cargo run --quiet -p nme-cli -- run examples/hello.nme
```

## 3. Make your first program

Create a UTF-8 text file named `hello.nme`:

```text
ask name, "What is your name? "
say f"Hello, {name}!"

3 times:
    say "Welcome to NME."
```

Run it:

```sh
nme run hello.nme
```

The program asks for a name, greets that person, and prints the final sentence
three times.

You may write the same ideas in Korean without selecting a language mode:

```text
물어봐 이름, "이름이 뭐예요? "
말해 f"안녕하세요, {이름}!"

3번:
    말해 "NME에 오신 것을 환영합니다."
```

English and Korean words are equal aliases. Mixing them is allowed too.

## 4. Learn the five ideas

### Show a value

```text
say "Hello"
say 1 + 2
말해 "안녕"
말해 f"답은 {1 + 2}"
```

`say` and `말해` become Python `print(...)`.

### Ask for text

```text
ask city, "Where do you live? "
물어봐 도시, "어디에 사나요? "
```

The answer is stored in the name before the comma. Answers are text, just like
Python `input`. Convert explicitly when you need a number:

```text
ask answer, "How old are you? "
age = int(answer)
```

### Repeat

```text
3 times:
    say "Again"

3번: 말해 "다시"
```

Use indentation for several statements or put one statement after `:`.
`3번:` and `3 번:` both work.

### Test a condition

```text
score = 12

when score >= 10:
    say "You won!"

만약 score < 10: 말해 "Try again"
```

`when` and `만약` become Python `if`. Use normal Python `else` when a
second branch is needed.

### Use random tools

```text
use random

die = random_number(1, 6)
color = random_pick(["red", "green", "blue"])
say f"You rolled {die} and got {color}."
```

The Korean toolkit has matching names:

```text
랜덤 사용

주사위 = 랜덤정수(1, 6)
색 = 랜덤선택(["빨강", "초록", "파랑"])
말해 f"주사위는 {주사위}, 색은 {색}"
```

Use `shuffle(list)` or `섞기(목록)` to reorder a list in place. These
helpers come from Python's bundled `random` module; nothing else is installed.

## 5. Use ordinary Python whenever needed

NME is not a separate ecosystem. Assignments, lists, functions, imports, and
packages are Python:

```text
import math

def circle_area(radius):
    return math.pi * radius**2

반지름 = 3
말해 circle_area(반지름)

for name in ["Ada", "Grace"]:
    say f"Hello, {name}"
```

Valid Python always wins and stays unchanged. `say("hello")` is therefore a
Python function call, while `say "hello"` is NME.

## 6. Check and build

Check NME forms without running the program:

```sh
nme check hello.nme
```

Success prints nothing. An NME mistake shows its location and a suggestion.
Korean forms receive Korean guidance. This command does not replace every
CPython syntax or runtime check.

Print generated Python:

```sh
nme build hello.nme
```

Write it to a file:

```sh
nme build hello.nme -o hello.py
python3 hello.py
```

The output is ordinary Python. NME preserves blank lines, comments,
indentation, line endings, and physical line counts.

## 7. CLI reference

```text
nme run <file.nme> [--python <command>]
nme build <file.nme> [-o <output.py>]
nme check <file.nme>
nme --help
nme --version
```

- `run` transpiles and starts CPython with inherited terminal input/output.
- `build` transpiles without executing and prints or writes Python.
- `check` validates tokenization and NME forms without creating output.

## Next steps

- Run `examples/hello.nme`, `examples/ask.nme`, `examples/korean.nme`, and
  `examples/mixed.nme`.
- Keep the [language reference](language.md) nearby for exact rules.
- Read the [version policy](versioning.md) for beta numbering and the locked
  `1.0.0` release rule.
- Read [compiler architecture](architecture.md) before changing NME behavior.
