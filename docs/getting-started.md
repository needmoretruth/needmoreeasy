# Grow from sentences to Python

English | [한국어](getting-started.ko.md)

[Home](../README.md) | [Install](install.md) | [Tutorial](tutorial.md) | [Language reference](language.md)

If Python still feels hard, this guide starts with something simpler and
changes one small idea at a time until you are writing Python. The five-minute
part starts after installation. If `nme --version` does not work yet, follow the
[Windows, macOS, or Linux installation guide](install.md) first.

## 0. Try it with nothing installed

**No installation, and it works on a phone.** Open **needmoreeasy.com** in a
browser (**nmelang.com** goes to the same place). Type a program on the left and
the Python it becomes appears on the right; press **Run** and the result appears
underneath. The compiler and a Python engine both run inside the browser, so the
program never leaves that tab.

The in-browser engine cannot use files, the network, or installed packages. When
a program needs those, follow the install steps below.

The five minutes below start after installing. If `nme --version` does not work
yet, follow the [Windows, macOS and Linux install guide](install.md) first.

## 1. Hello World

Create a UTF-8 file named `hello.nme` (in the same folder where you ran
`nme --version`; any text editor works — on Windows, Notepad saves UTF-8 by
default):

```nme
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

```nme
What is your name?
Hello name!
```

NME remembers that `name` holds the answer, so it inserts that value into the
second sentence automatically. More complex questions can use `ask name`, but
the ordinary question is enough for a first program.

The same ideas work in Korean and can be mixed immediately:

```nme
이름이 뭐예요?
안녕하세요 이름!
```

## 3. Repeat without a colon

For one repeated sentence, replace the file with (put both lines in one
file — English and Korean mix freely):

```nme
3 times Welcome to NME
3번 안녕하세요
```

For several sentences, replace the file with (indent the extra lines with
four spaces):

```nme
repeat 3 times
    show First sentence
    둘째 문장 말해줘
```

If indentation is still uncomfortable, replace the file with this version
that uses an explicit closing word instead:

```nme
set score to 0
while score is less than 3
show score
add 1 to score
end
```

`set score to 0` stores a number, `while score is less than 3` keeps going
for as long as that stays true, and `add 1 to score` increases it.

The same style supports `break`, `and`/`or`, `else if`, and `else`; Korean
spellings are `멈춰`, `그리고`/`또는`, `아니면 만약에`, and `아니면`.

## 4. Make a number game

Replace the file with:

```nme
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

## 5. Lists and waiting — still only sentences

A list is written the way you would say it out loud. Replace the file with:

```nme
set friends to list of Mina, Ada
for each friend in friends
    show Hello friend
    wait 1 second
end
```

Add to a list the same way:

```nme
set friends to empty list
append Mina to friends
append Ada to friends
show friends
```

Nothing above needed quotes, brackets, colons or `=`. **Sentence syntax is
not a beginner's toy that you leave behind — it is the language.** Guides 01–12
and 86–88 are written this way from start to finish, and they cover output,
questions, lists, loops, conditions, files of your own making, stories, the
screen, a stopwatch and cooldowns.

Two more levels exist for the day you want them: a shorter beginner syntax,
and ordinary Python, which NME keeps exactly as you typed it. Neither is
needed to finish a program, and both are described in the
[language reference](language.md). If you are here to learn to program, stay
on sentences — the next section shows you the Python your sentences already
became.

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

- [Learning guides](guides/index.md): 73 small progressive guides, each with
  difficulty, prerequisites, topic, and result
- [Learning path](tutorial.md): seven projects from Hello World to a compiler
- [Language reference](language.md): exact rules for all three levels
- [Editors](editors.md): VS Code, Cursor, and Zed
- [AI assistants](ai-assistants.md): give an assistant one documentation link
