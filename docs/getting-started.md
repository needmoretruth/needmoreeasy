# Grow from sentences to Python

English | [한국어](getting-started.ko.md)

[Home](../README.md) | [Install](install.md) | [Tutorial](tutorial.md) | [Language reference](language.md)

If Python still feels hard, this guide starts with something simpler and
changes one small idea at a time until you are writing Python. The five-minute
part starts after installation. If `nme --version` does not work yet, follow the
[Windows, macOS, or Linux installation guide](install.md) first.

## 1. Hello World

Create a UTF-8 file named `hello.nme` (in the same folder where you ran
`nme --version`; any text editor works — on Windows, Notepad saves UTF-8 by
default):

```text
show Hello world!
```

Run it:

```sh
nme run hello
```

Expected output: `Hello world!` You just wrote a complete program. There are
no quotes or parentheses.

Shortcuts work from the start: `nme r hello` runs, and if `hello.nme` is the
only program in the folder, plain `nme r` finds it for you. Later, `nme install
<package>` installs a Python package (guide [24](guides/24-python-packages.md)). `nme c hello`
checks a program without running it, and `nme b hello` shows the readable
Python it turns into — add `-o hello.py` to save it as a file.

## 2. Have a conversation

Replace the file with (then run `nme run hello` again):

```text
What is your name?
Hello name!
```

NME remembers that `name` holds the answer, so it inserts that value into the
second sentence automatically. More complex questions can use `ask name`, but
the ordinary question is enough for a first program.

The same ideas work in Korean and can be mixed immediately:

```text
이름이 뭐예요?
안녕하세요 이름!
```

## 3. Repeat without a colon

For one repeated sentence, replace the file with (put both lines in one
file — English and Korean mix freely):

```text
3 times Welcome to NME
3번 안녕하세요
```

For several sentences, replace the file with (indent the extra lines with
four spaces):

```text
repeat 3 times
    show First sentence
    둘째 문장 말해줘
```

If indentation is still uncomfortable, replace the file with this version
that uses an explicit closing word instead:

```text
score = 0
while score < 3
show score
add 1 to score
end
```

`score = 0` stores a number, `<` means 'less than', and `add 1 to score`
increases it — the same ideas as the sentence forms above.

The compact beginner repeat form can use the same trick — replace the file
with:

```text
3 times:
show one line
show another line
end
```

The same style supports `break`, `and`/`or`, `elif`, and `else`; Korean
spellings are `멈춰`, `그리고`/`또는`, `아니면 만약에`, and `아니면`.

## 4. Make a number game

Replace the file with:

```text
set answer to random number from 1 to 10
ask number guess Pick a number from 1 to 10

if guess equals answer
    show Correct!

if guess is less than answer
    show Go higher

if guess is greater than answer
    show Go lower
```

The game asks a question and waits for your typed answer. Run the complete
Korean version:

```sh
nme run examples/guessing-game.ko
```

The English version is [`examples/guessing-game.nme`](../examples/guessing-game.nme):

```sh
nme run examples/guessing-game
```

The random number, numeric input, comparisons, and output compile to ordinary
Python. No list literal, function call, equals sign, or colon was required.

## 5. Grow into precise and advanced syntax

Sentence syntax is the easiest start. Beginner syntax is shorter and precise.
Replace the file with:

```text
ask name, "What is your name? "
when name:
    say f"Hello, {name}!"
```

Three new pieces of syntax appear here:

- `ask name, "…"` asks a question and remembers the answer as `name`.
- `when name:` means "when a condition is true, run the indented lines below".
- `f"Hello, {name}!"` is a Python f-string: text where `{name}` is replaced
  by the value of `name`.

The [language reference](language.md) explains each form exactly.

Advanced syntax is just Python. Replace the file with:

```python
for number in range(1, 4):
    print(number**2)
```

Mix all three whenever useful — replace the file with:

```text
numbers = [1, 2, 3]

for number in numbers:
    show number

2 times: 말해 "done"
```

## 6. Check, build, and compile

```sh
nme check hello
nme build hello -o hello.py
python3 hello.py
```

(On Windows: `py hello.py`.)

For an optional standalone native executable:

```sh
python3 -m pip install nuitka
nme compile hello.nme -o hello
```

(On Windows: `py -m pip install nuitka`. The [install guide](install.md)
adds the optional `[app]` extras.)

NME automatically chooses `python3` on macOS/Linux and `py` on Windows. The
advanced `--python` option is only for unusual local setups.

## 7. Let NME simplify Python

First create `old_program.py` with a couple of lines such as `print("hi")` and
`for i in range(3): print(i)` — or reuse the `hello.py` from section 6. Then
run:

```sh
nme convert old_program.py --level beginner --language ko -o easier.ko.nme
nme convert old_program.py --level beginner --language en -o easier.nme
```

The converter keeps string quotes (`say "hi"`). In beginner syntax they are
usually unnecessary (`say hi`), so you can remove them after inspection if you
want a more conversational phrase.

## Where to continue

- [Learning path](tutorial.md): six projects from Hello World to a compiler
- [Language reference](language.md): exact rules for all three levels
- [Editors](editors.md): VS Code, Cursor, and Zed
- [AI assistants](ai-assistants.md): give an assistant one documentation link
