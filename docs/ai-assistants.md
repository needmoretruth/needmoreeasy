# AI coding assistant handoff for NME

English | [한국어](ai-assistants.ko.md)

Give an AI coding assistant this one prompt:

```text
Read and follow this NME language handoff before writing code:
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.md

Then write the requested program as a .nme file, prefer sentence syntax for a
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
```

### Generate sentence syntax first

```text
show Hello world!
안녕하세요 말해줘

ask name What is your name?
이름을 물어봐 이름이 뭐예요?

repeat 3 times and show Again
3번 반복해서 다시 말해줘

if score is greater than 10 then show You won
만약에 점수가 10보다 크면 성공 말해줘

set answer to random number from 1 to 10
ask number guess Pick a number
```

Known input or assignment names are interpolated in sentence output. Natural
prompts get a trailing space automatically. Indent multi-line bodies with four
spaces; sentence headers do not need a colon.

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
nme check program.nme
nme build program.nme -o program.py
```

Use `nme run program.nme` when execution is safe and desired. Use
`nme compile program.nme -o program` only when the user wants a native artifact
and Nuitka is installed.

If a requested phrase is not described here, read the full
[English language reference](https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/language.md)
or [Korean reference](https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/language.ko.md).
Do not invent unsupported NME keywords. Fall back to valid Python, which is
always valid advanced NME.

## Product-specific ways to provide the link

- Cursor: paste the URL with `@Link`. Cursor's official
  [context documentation](https://docs.cursor.com/context/%40-symbols/overview)
  describes link context.
- Claude Code: paste the handoff prompt at session start. Claude Code also
  supports persistent `CLAUDE.md` memory, documented by
  [Anthropic](https://docs.anthropic.com/en/docs/claude-code/memory), but it is
  not required for NME.
- Codex: paste the handoff prompt as the task. Codex also supports layered
  `AGENTS.md` instructions according to the
  [official OpenAI documentation](https://learn.chatgpt.com/docs/agent-configuration/agents-md),
  but NME projects do not need to track one.
- OpenCode: paste the handoff prompt. Its official
  [rules documentation](https://opencode.ai/docs/rules/) also supports remote
  instruction URLs when a user wants persistent configuration.

Keep tool-specific assistant metadata outside the NME program repository
unless the repository owner explicitly asks to track it.
