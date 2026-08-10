# AI coding assistant handoff for NME

English | [한국어](ai-assistants.ko.md)

Give an AI coding assistant this one prompt:

```text
Read and follow this NME language handoff before writing code:
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.md

Then confirm that `nme --version` is the supported beta, then write the requested program as a .nme file, prefer sentence syntax for a
beginner, mix beginner or advanced Python only where it makes the result
clearer, and verify it with nme check.
```

No assistant-specific file is required in the user's project.

## Instructions for the assistant reading this page

NME has three levels that may be mixed line by line:

1. Sentence syntax: almost no code punctuation; use this by default for a new
   programmer.
2. Beginner syntax: concise and precise; use it for arbitrary Python
   expressions.
3. Advanced syntax: valid Python, unchanged and fully compatible.

Korean and English are equal aliases, not separate modes. Mixing is valid:

```text
이름을 물어봐 이름이 뭐예요?
show Hello 이름!
3 times 반복해서 Welcome 말해줘
What is your name?
Hello name!
```

### Generate sentence syntax first

```text
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

3 times Again
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
a trailing space automatically.
For the gentlest start, close a
flat block with `end`/`끝` instead of relying on indentation:

```text
while score < 3
show score
add 1 to score
end
```

The same block supports `break`/`멈춰`, `and`/`그리고`, `or`/`또는`,
`elif`/`아니면 만약에`, and `else`/`아니면`. Indented bodies and ordinary Python
remain valid when a learner is ready to use them.

### Use beginner syntax when precision matters

```text
say <Python expression>
말해 <Python 표현식>
ask name, <Python prompt>
물어봐 이름, <Python 질문>
count times:
횟수번:
when <Python condition>:
만약 <Python 조건>:
while <condition> ... end
동안 <조건> ... 끝
break / 멈춰
else if <condition> / 아니면 만약 <조건>
else / 아니면
use random
랜덤 사용 최신
```

### Use any Python as advanced NME

Do not translate a Python construct when the easier equivalent would change
semantics. Functions, classes, imports, comprehensions, async code, exception
handling, installed Python packages, and every other valid Python feature may
remain Python.

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

```text
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
Korean commands such as `nme 도움`, `nme 검사`, and `nme 실행` print Korean
guidance followed by the equivalent English guidance.

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
