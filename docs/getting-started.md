# NME in five minutes

English | [한국어](getting-started.ko.md)

If `nme --version` does not work yet, follow the
[Windows, macOS, or Linux installation guide](install.md) first.

## 1. Hello World

Create a UTF-8 file named `hello.nme`:

```text
show Hello world!
```

Run it:

```sh
nme run hello.nme
```

You just wrote a complete program. There are no quotes or parentheses.

## 2. Have a conversation

Replace the file with:

```text
ask name What is your name?
show Hello name!
```

NME remembers that `name` holds the answer, so it inserts that value into the
second sentence automatically.

The same ideas work in Korean and can be mixed immediately:

```text
이름을 물어봐 이름이 뭐예요?
show Hello 이름!
```

## 3. Repeat without a colon

For one repeated sentence:

```text
repeat 3 times and show Welcome to NME
```

For several sentences, indent them with four spaces:

```text
repeat 3 times
    show First sentence
    둘째 문장 말해줘
```

## 4. Make a number game

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

Run the complete Korean version:

```sh
nme run examples/guessing-game.ko.nme
```

The random number, numeric input, comparisons, and output compile to ordinary
Python. No list literal, function call, equals sign, or colon was required.

## 5. Grow into precise and advanced syntax

Sentence syntax is the easiest start. Beginner syntax is shorter and precise:

```text
ask name, "What is your name? "
when name:
    say f"Hello, {name}!"
```

Advanced syntax is just Python:

```python
for number in range(1, 4):
    print(number**2)
```

Mix all three whenever useful:

```text
numbers = [1, 2, 3]

for number in numbers:
    show number

2 times: 말해 "done"
```

## 6. Check, build, and compile

```sh
nme check hello.nme
nme build hello.nme -o hello.py
python3 hello.py
```

For an optional standalone native executable:

```sh
python3 -m pip install nuitka
nme compile hello.nme -o hello
```

Use `python` or `py` instead of `python3` when that is your platform command,
and pass it to NME with `--python`.

## 7. Let NME simplify Python

```sh
nme convert old_program.py --level sentence --language en -o easier.nme
nme convert old_program.py --level beginner --language ko -o easier.ko.nme
```

## Where to continue

- [Learning path](tutorial.md): five projects from Hello World to a compiler
- [Language reference](language.md): exact rules for all three levels
- [Editors](editors.md): VS Code, Cursor, and Zed
- [AI assistants](ai-assistants.md): give an assistant one documentation link
