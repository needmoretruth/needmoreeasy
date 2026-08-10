# Learn NME by making five programs

English | [한국어](tutorial.ko.md)

[Home](../README.md) | [Install](install.md) | [Getting started](getting-started.md) | [Language reference](language.md)

This path starts with one sentence and ends with a compiler. Run every example
before moving on; changing a word and observing the result is part of learning.

## Project 1: Hello World

Create `hello.nme`:

```text
show Hello world!
```

```sh
nme run hello
```

`show` is an action. Everything after it is the sentence to display. Korean is
equally valid:

```text
안녕하세요! 말해줘
```

Try changing the message. Run `examples/hello-sentence.nme` when you want to
see a repeat too.

## Project 2: A greeting program

```text
What is your name?
Nice to meet you name!
```

The first line creates the name `name` from an ordinary question. The second
line recognizes that known name and inserts its value. The Korean version needs
no formatting syntax:

```text
이름이 뭐예요?
이름 만나서 반가워요!
```

Challenge: ask for a favorite color and show it in another sentence.

## Project 3: Number guessing

Start with a random answer and numeric input:

```text
set answer to random number from 1 to 10
ask number guess Pick a number from 1 to 10
```

Then compare them:

```text
if guess equals answer
    show Correct!

if guess is less than answer
    show Go higher

if guess is greater than answer
    show Go lower
```

The complete Korean program is
[`examples/guessing-game.ko.nme`](../examples/guessing-game.ko.nme):

```sh
nme run examples/guessing-game.ko
```

What was compiled:

- `1부터 10까지 랜덤정수` becomes `random.randint(1, 10)`;
- `숫자로 물어봐` becomes `int(input(...))`;
- `같으면`, `작으면`, and `크면` become `==`, `<`, and `>`;
- indentation groups the statements controlled by each condition.

Challenge: change the range to 1–100, then add a second guess by repeating the
input and conditions.

If indentation is getting in the way, write the same control flow as one flat
block and close it with `end`:

```text
while guess != answer
show Try again
ask number guess Pick another number
end
```

Use `break`, `and`/`or`, `elif`, and `else` in the same style. Their Korean
spellings are `멈춰`, `그리고`/`또는`, `아니면 만약`, and `아니면`.

## Project 4: Use all three levels

Sentence syntax is not a cage. Use compact beginner syntax or Python whenever
it says the idea more clearly:

```text
people = ["Ada", "Grace"]

2 times:
    say "beginner syntax"

repeat 2 times
    한국어 문장형 말해줘

for person in people:
    show Hello person!
```

The list and `for person` loop are advanced Python. `2 times:` and `say` are
beginner NME. `repeat` and `show` are sentence NME. This is one language, not
three files or modes. Run `examples/three-levels.nme`.

Try adding a flat NME block beside the Python loop:

```text
while person != "Grace"
show Hello person
break
end
```

Challenge: write a Python function and use sentence `show` inside it.

## Project 5: Move one game toward Python

The time-loop mystery is the same project written at three levels. Start with
the conversational Korean version, then compare the compact beginner version,
and finally read the ordinary Python version:

- [`time-loop-sentence.ko.nme`](../examples/time-loop-sentence.ko.nme) — easiest
  sentences, `끝`, and natural Korean conditions;
- [`time-loop-beginner.ko.nme`](../examples/time-loop-beginner.ko.nme) — compact
  `저장`, `물어봐`, `만약`, and `N번:` forms;
- [`time-loop-python.nme`](../examples/time-loop-python.nme) — lists,
  dictionaries, f-strings, `while`, `break`, and ordinary Python.

All three compile with the same command:

```sh
nme check examples/time-loop-sentence.ko
nme check examples/time-loop-beginner.ko
nme check examples/time-loop-python
```

For a larger standalone example before the Python version feels comfortable,
try [`roulette.nme`](../examples/roulette.nme). Its English companion is
[`roulette.en.nme`](../examples/roulette.en.nme).

Run one when you are ready to answer its prompts. The point is not to rewrite
the whole project at once: replace one block or line with the next level and
keep the rest of the program unchanged.

## Project 6: Build a compiler in NME

This is an optional advanced capstone, not a new syntax level. Before opening
it, make sure the earlier projects feel comfortable. If indexing, slices, or
Python method calls are still new, leave this project for later and keep
building small programs with sentence and beginner NME first:

```text
count = 0
while count < 2
add 1 to count
end

2 times: say "one small rule"
```

A compiler reads one language and writes another. It does not need to execute
the source directly. The example
[`examples/tiny-compiler.nme`](../examples/tiny-compiler.nme) compiles this
tiny two-sentence language:

```text
말하기 안녕하세요
3번 말하기 NME로 컴파일러를 만들었어요
```

The compiler produces Python equivalent to:

```python
print('안녕하세요')
for _ in range(3): print('NME로 컴파일러를 만들었어요')
```

Run the compiler, then inspect the compiler's own generated Python:

```sh
nme run examples/tiny-compiler
nme build examples/tiny-compiler -o tiny-compiler.py
```

The example deliberately mixes advanced Python for list processing with
sentence NME for its output. That is how a beginner can grow a compiler one
small rule at a time while retaining the full Python ecosystem.

To turn it into a file compiler:

1. Replace `tiny_source` with
   `Path(input_name).read_text(encoding="utf-8").splitlines()`.
2. Write `generated` with
   `Path(output_name).write_text(generated, encoding="utf-8")`.
3. Add a friendly error for any line that matches neither tiny sentence.
4. Add tests that compile a source file and run the generated Python.

This mirrors NME's real architecture at a smaller scale: tokenize or classify
input, create a precise intermediate meaning, lower it to target code, and
test the result. NME's production compiler is written in Rust for reliability
and distribution, but compilers made *with* NME may use all of NME and Python.

## Your next steps

- Read the [language reference](language.md) only when you need an exact rule.
- Use `nme check` after every small change.
- Convert an existing Python exercise with [nme convert](converting-python.md).
- Build a native artifact only after the program works with `nme run`.
