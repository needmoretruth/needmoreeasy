# AI coding assistant handoff for NME

English | [한국어](ai-assistants.ko.md)

[Home](../README.md) | [Getting started](getting-started.md) | [Tutorial](tutorial.md) | [Language reference](language.md)

## The simplest way

**Three prompts sit at the top of [needmoreeasy.com](https://needmoreeasy.com/).**
Copy one whole and paste it at the start of a chat, and that is the whole job.
They are generated from the compiler, so they are always current, and they work
in a chat window that cannot open a link. If you have not written a program
before, use the first one — sentences only.

What follows is for a tool that *can* read a link, when you would rather hand
over an address than a wall of text.

## Handing over an address

Give an AI coding assistant this one prompt:

```text
Read and follow this NME language handoff before writing code:
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.md

Then confirm that `nme --version` is the supported beta (currently `0.6.0`), then write the requested program as a .nme file, prefer sentence syntax for a
beginner, mix beginner or advanced Python only where it makes the result
clearer, and verify it with nme check.
```

No assistant-specific file is required in the user's project.

## Pick a purpose for your starter prompt

The one prompt at the top writes a new program. For other goals, paste the
matching short prompt with the same link. Each prompt is complete on its own
and always starts by sending the AI to the handoff page.

### Write NME code

Use this when you know what the program should do and want a .nme file you can
keep improving.

```text
Read and follow this NME handoff, then write the program I ask for as a .nme file and verify it with nme check:
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.md
```

```text
이 NME 문서를 읽고 따르세요. 제가 요청하는 프로그램을 .nme 파일로 작성하고 nme 검사로 확인해 주세요.
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.ko.md
```

### Learn NME

Use this to study NME step by step, starting from sentence syntax.

```text
Read this NME handoff, then teach me NME step by step starting with sentence syntax, and use the language reference for keyword questions:
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.md
```

```text
이 NME 문서를 읽고, 문장형부터 시작해서 NME를 단계별로 가르쳐 주세요. 키워드 질문에는 문법 레퍼런스를 사용하세요.
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.ko.md
```

### NME teacher role

Use this to get line-by-line explanations of your code and one small next step.

```text
You are my NME teacher. Read this handoff and the language reference, then explain my code line by line and suggest one small next step:
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.md
```

```text
당신은 나의 NME 선생님입니다. 이 문서와 문법 레퍼런스를 읽고, 제 코드를 줄마다 설명하고 작은 다음 단계 하나를 제안해 주세요.
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.ko.md
```

### Explain or convert existing code

Use this when you paste Python or NME code and want it explained, or converted
with the same meaning.

```text
Read this NME handoff, then explain the code I paste line by line, or convert it to easier NME with the same meaning when I ask:
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.md
```

```text
이 NME 문서를 읽고, 제가 붙여 넣는 코드를 줄마다 설명하거나, 제가 요청하면 뜻이 같게 더 쉬운 NME로 변환해 주세요.
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.ko.md
```

### Debug a project

Use this when a program fails or behaves wrongly.

```text
Read this NME handoff, then find the bug in my .nme program: explain the cause, suggest the smallest fix, and ask me to run nme check to verify:
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.md
```

```text
이 NME 문서를 읽고, 제 .nme 프로그램의 버그를 찾아 주세요: 원인을 설명하고 가장 작은 수정을 제안하며 nme 검사로 확인하라고 알려 주세요.
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.ko.md
```

## Instructions for the assistant reading this page

NME has three levels that may be mixed line by line:

1. Sentence syntax: almost no code punctuation; use this by default for a new
   programmer.
2. Beginner syntax: concise and precise; use it for arbitrary Python
   expressions.
3. Advanced syntax: valid Python, unchanged and fully compatible.

Korean and English are equal aliases, not separate modes. Mixing is valid:

```nme
이름을 물어봐 이름이 뭐예요?
show Hello 이름!
3 times 반복해서 Welcome 말해줘
What is your name?
Hello name!
```

### Generate sentence syntax first

```nme
Hello world show
안녕하세요 말해줘
Hello everyone!
오늘도 반가워요!

name ask
이름을 물어봐
Hello name show

What is your name?
What's your city?
Hello name!

3 times See you again
3번 다시 만나요

if score is greater than 10 then show You won
만약에 점수가 10보다 크면 성공 말해줘
score is greater than 5 then show high
색이 빨강과 같으면 맞아요 말해줘

set answer to random number from 1 to 10
ask number guess Pick a number
```

For the gentlest possible output, a clear multi-word sentence can omit the
action word (`Hello everyone!`). A single bare word is valid Python and stays
Python, so use `show`/`말해줘` when that one word should be printed. Common
action, logical, and condition typos such as `repaet`, `shwoe`, `그리거`, and
`같먄` are repaired only when the intended meaning is unique.

Known input or assignment names are interpolated in sentence output. Natural
questions such as `What is your name` infer the target; Korean forms such as
`내 이름은 뭐예요?` do too. The final `?` is optional, and natural prompts get
a trailing space automatically. Age questions such as `How old are you?` and
`몇 살이에요?` infer `age` and `나이`.
For the gentlest start, close a
flat block with `end`/`끝` instead of relying on indentation:

```nme
while score < 3
show score
add 1 to score
end
```

The same block supports `break`/`멈춰`, `and`/`그리고`, `or`/`또는`,
`elif`/`아니면 만약에`, and `else`/`아니면`. Indented bodies and ordinary Python
remain valid when a learner is ready to use them. A whole NME condition may be
parenthesized in a colon-free `if` or `while` header, such as `if (ready and
score > 2)`; valid Python calls such as `when(ready and score > 2)` remain
Python. Korean comparison endings can remain inside that wrapper and precede a
connector, such as `만약 (점수가 2보다 크면 그리고 준비)`.
The same shared rule applies to a Korean `while` ending, such as
`동안 (횟수가 2보다 작을 동안 그리고 준비)`.
The connector spellings can be mixed in the same wrapper, for example
`만약 (점수가 2보다 크면 and 준비)`.
Korean NME words can also be valid Python identifiers: if `만약` is bound,
`만약 (준비)` is a Python call shape and must stay byte-identical. Use a spoken
ending such as `만약 준비라면`, or an NME connector such as
`만약 ((준비 그리고 참))`, when an NME block is intended.

### Use beginner syntax when precision matters

```text
say <Python expression>
말해 <Python 표현식>
ask name, <Python prompt>
물어봐 이름, <Python 질문>
count times:
횟수번:
3 times:
show one line
end
when <Python condition>:
만약 <Python 조건>:
while <condition> ... end
동안 <조건> ... 끝
break / 멈춰
else if <condition> / 아니면 만약에 <조건>
else / 아니면
use random
랜덤 사용 최신
```

`count times:` repeats `count` times — the variable must hold a number.
`횟수번:` does the same with `횟수`.

When the user asks for the restricted native backend, boolean names are
supported as a type distinct from integers: assign `True`/`False`, use the name
in a truthy `if`/`while`, combine supported conditions with short-circuiting
`and`/`or`, and `show` prints `True` or `False`. Do not use
boolean arithmetic or `add`/`subtract` updates in native code; direct those
programs to CPython unless the user explicitly wants a native-core diagnostic.
Native `if`/`while`/branch bodies may use one-line NME `say`/`show` or `break`
after `then`/`그러면`; sentence repeats may use a one-line `break here` body.
Python inline bodies and inline value updates remain outside the native subset.
The [native core reference](https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/native-reference.md)
defines the complete boundary.

### Use any Python as advanced NME

Do not translate a Python construct when the easier equivalent would change
semantics. Functions, classes, imports, comprehensions, async code, exception
handling, installed Python packages, and every other valid Python feature may
remain Python.

Python context rules still apply: `return` and `yield` belong inside a function,
while `await` belongs inside `async def`, and `break`/`continue` belong inside
loops. `nme check` reports invalid top-level, inline, and one-line
function/class cases with stable bilingual codes `E0102`, `E0106`–`E0110`; a
one-line function or class does not inherit an outer loop. In an `async def`, use
`async for` instead of `yield from`; the same control-flow check covers a
semicolon-separated statement later in a one-line suite.
`async for` and `async with` also belong inside `async def`; invalid placements
receive `E0111` and `E0112`.
`nonlocal` needs an enclosing function; invalid top-level, top-level-class,
non-nested-function, and one-line-suite placements receive `E0113`. Nested
functions and classes under an outer function remain valid, while CPython
checks whether the named outer binding exists.
Python star imports (`from ... import *`) are module-level only; using one
inside a function or class, including a one-line suite or a star import after
an earlier semicolon-separated statement, receives `E0114`. Import names
explicitly there.
Python also rejects `break`, `continue`, and `return` inside `except*`; NME
reports `E0115`, including when the control follows an earlier semicolon-
separated statement in the handler. Keep those controls outside the handler or
use a normal `except` block when its semantics fit.
Python rejects `yield` inside list, set, dictionary, and generator
comprehensions; NME reports `E0116`. Replace the comprehension with an explicit
loop, while keeping ordinary generator lambdas unchanged.
Python also rejects an `async for` inside a comprehension outside `async def`;
NME reports `E0117`. Move that comprehension into an async function, while
preserving valid async comprehensions.
An async generator cannot return a value; NME reports `E0118`, including when
the return appears before its first `yield`. One-line Python suites are tracked
as function bodies too, so valid inline `yield`, `await`, and bare `return`
remain unchanged. Use a bare `return`; nested function returns remain valid.
`global` and `nonlocal` must precede uses or assignments in their scope, and
cannot name parameters or annotated targets; NME reports `E0119`/`E0120` for
those conflicts, including in one-line suites. Put the declaration first, and
annotation expressions count as uses. F-string validation remains with CPython;
comprehension-local names stay separate.
Generator lambdas such as `lambda: (yield value)` are valid advanced Python
and must remain unchanged.

Valid Python always wins. Never rewrite `say("x")`, a variable named `when`,
text inside a string, or a comment as NME syntax.

### Random module

The bundled random adapter is version `0.0.1`. `use random latest` and
`랜덤 사용 최신` select it locally. Loading either spelling exposes both:

- `random_number` / `랜덤정수`
- `random_pick` / `랜덤선택`
- `shuffle` / `섞기`
- `random_version` / `랜덤버전`

Sentence random forms need no import:

```nme
set die to random number from 1 to 6
set color to pick from red or green or blue

주사위는 1부터 6까지 랜덤정수
색은 빨강 또는 초록 또는 파랑 중에서 랜덤선택
```

### Verify rather than invent

After writing or editing a file, run:

```sh
nme --version
nme check program
nme build program -o program.py
```

`check` and `build` ask the selected CPython to compile the generated source;
they do not execute it. `nme help` and English commands print English only.
Korean commands such as `nme 도움`, `nme 검사`, `nme 실행`, `nme 네이티브`, and
`nme 설치` use Korean-first bilingual diagnostics or guidance.

Use `nme run program` when execution is safe and desired. Use
`nme compile program -o program` only when the user wants a native artifact
and Nuitka is installed.

Only the documented NME actions have Korean and English aliases. Advanced
syntax is Python, so Python keywords such as `def`, `for`, `import`, and
`return` remain Python keywords. Never invent Korean versions of them.

If a requested phrase is not described here, read the full
[English language reference](https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/language.md)
or [Korean reference](https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/language.ko.md).
Do not invent unsupported NME keywords. Fall back to valid Python, which is
always valid advanced NME.

## Use a general chat AI with NME

ChatGPT, Claude (chat), Gemini, and Grok read web pages from a link, so the
same short-prompt + one-link design works: no file is needed in your project.
Paste the prompt at the top of this page, or one of the purpose prompts above,
into a new chat, then keep asking follow-up questions in the same chat so the
handoff stays in context.

- ChatGPT: paste the prompt with the link into a chat and ask for a .nme file.
- Claude (chat, claude.ai): paste the same prompt with the link; Claude reads
  the page before answering.
- Gemini: paste the prompt with the link and refine the result with follow-up
  questions.
- Grok: paste the prompt with the link in one message and continue the
  conversation from there.

When a phrase is not in the handoff, any of them can also read the
[language reference](https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/language.md).
These chat AIs usually cannot run the nme CLI themselves, so after they write a
program, run `nme check` (or `nme c`) yourself and paste the error back into
the chat.

## Product-specific ways to provide the link

- Cursor: paste the URL with `@Link`. Cursor's official
  [@Link guide](https://docs.cursor.com/context/%40-symbols/%40-link) describes
  the link context flow.
- Claude Code: paste the handoff prompt at session start. Claude Code also
  supports persistent `CLAUDE.md` memory, documented in the official
  [Claude Code memory guide](https://code.claude.com/docs/en/memory), but it is
  not required for NME.
- Codex: paste the handoff prompt as the task. Codex also supports layered
  `AGENTS.md` instructions, as described by [OpenAI](https://openai.com/index/introducing-codex/),
  but NME projects do not need to track one.
- OpenCode: paste the handoff prompt. Its official
  [rules documentation](https://dev.opencode.ai/docs/rules/) also supports
  remote instruction URLs when a user wants persistent configuration.

Keep tool-specific assistant metadata outside the NME program repository
unless the repository owner explicitly asks to track it.
