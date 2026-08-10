# Convert Python to NME

English | [한국어](converting-python.ko.md)

[Home](../README.md) | [Getting started](getting-started.md) | [Tutorial](tutorial.md) | [Language reference](language.md)

The converter validates the complete input as Python, then rewrites only lines
whose meaning has a safe equivalent at the requested NME level.

```sh
nme convert app.py --level sentence --language ko -o app.nme
```

## Options

```text
--level advanced|beginner|sentence
--language en|ko
-o, --output <file.nme>
```

Korean option values are accepted too: `고급`, `초급`, `문장형`, `영어`,
`한국어`. Without options, the converter prints English sentence syntax to
standard output. Without `-o`, it never modifies the input file.

## What converts

| Python | Beginner | Sentence |
| --- | --- | --- |
| `print(value)` | `say value` | `show value` |
| `name = input(prompt)` | `ask name, prompt` | `ask name prompt` |
| `n = int(input(prompt))` | kept as Python | `ask number n prompt` |
| `for _ in range(n):` | `n times:` | `repeat n times` |
| `if condition:` | `when condition:` | `if condition` |
| `import random` | kept as Python | kept as Python |
| simple assignment | kept as Python | `set name to value` |

Korean output uses `말해`, `물어봐`, `번`, `만약`, `랜덤 사용`, and natural
Korean sentence forms.

Ordinary `import random` stays Python because rewriting it could overwrite a
user's `random`, `random_number`, or Korean helper name. The converter retains
string quotes and exact prompt contents so automatic
conversion cannot accidentally turn a literal word into interpolation or add
the friendly space used by hand-written natural prompts. You may remove the
quotes after inspection when you want a more conversational phrase. Comments,
indentation, blank lines, line endings, and unsupported Python remain
unchanged.

## Why some Python remains

NME advanced syntax is Python. A class, exception handler, complex call, or
multi-argument print has no smaller NME equivalent that is guaranteed to keep
the same behavior, so the converter leaves it as advanced syntax. This is a
complete, runnable NME result—not a failed partial translation.

Always inspect and check the result:

```sh
nme check app
nme build app -o app.generated.py
```
