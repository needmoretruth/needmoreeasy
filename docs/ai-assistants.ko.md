# AI 코딩 도우미에게 NME 알려 주기

[English](ai-assistants.md) | 한국어

[README](../README.ko.md) | [5분 시작](getting-started.ko.md) | [학습 과정](tutorial.ko.md) | [문법 안내](language.ko.md)

AI 코딩 도우미에게 다음 문장 하나를 전달하세요.

```text
NME 코드를 작성하기 전에 이 문서를 읽고 따르세요.
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.ko.md

먼저 `nme --version`으로 지원되는 베타(현재 `0.0.1-beta.101`)를 확인하세요. 그다음 요청한 프로그램을 .nme 파일로 작성하세요. 초보자에게는 문장형을 우선하고,
더 분명할 때만 초급 문법이나 고급 Python을 섞으며, nme 검사로 확인하세요.
```

사용자의 프로젝트에 도우미 전용 파일을 만들 필요가 없습니다.

## 목적에 맞는 시작 문장 고르기

맨 위의 문장 하나는 새 프로그램을 작성하는 용도입니다. 다른 목적은 아래에서
맞는 짧은 시작 문장을 같은 링크와 함께 붙이세요. 각 시작 문장은 그 자체로
완결되며 항상 AI를 이 문서로 안내합니다.

### NME 코드 작성

원하는 프로그램이 이미 있고 계속 다듬을 .nme 파일이 필요할 때 사용하세요.

```text
이 NME 문서를 읽고 따르세요. 제가 요청하는 프로그램을 .nme 파일로 작성하고 nme 검사로 확인해 주세요.
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.ko.md
```

```text
Read and follow this NME handoff, then write the program I ask for as a .nme file and verify it with nme check:
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.md
```

### NME 배우기

문장형부터 시작해서 NME를 단계별로 배울 때 사용하세요.

```text
이 NME 문서를 읽고, 문장형부터 시작해서 NME를 단계별로 가르쳐 주세요. 키워드 질문에는 문법 레퍼런스를 사용하세요.
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.ko.md
```

```text
Read this NME handoff, then teach me NME step by step starting with sentence syntax, and use the language reference for keyword questions:
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.md
```

### NME 선생님 역할

내 코드를 줄마다 설명받고 작은 다음 단계 하나를 얻고 싶을 때 사용하세요.

```text
당신은 나의 NME 선생님입니다. 이 문서와 문법 레퍼런스를 읽고, 제 코드를 줄마다 설명하고 작은 다음 단계 하나를 제안해 주세요.
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.ko.md
```

```text
You are my NME teacher. Read this handoff and the language reference, then explain my code line by line and suggest one small next step:
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.md
```

### 기존 코드 설명/변환

Python이나 NME 코드를 붙여 넣고 설명이나 뜻이 같은 변환을 받고 싶을 때
사용하세요.

```text
이 NME 문서를 읽고, 제가 붙여 넣는 코드를 줄마다 설명하거나, 제가 요청하면 뜻이 같게 더 쉬운 NME로 변환해 주세요.
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.ko.md
```

```text
Read this NME handoff, then explain the code I paste line by line, or convert it to easier NME with the same meaning when I ask:
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.md
```

### 프로젝트 디버깅

프로그램이 실패하거나 이상하게 동작할 때 사용하세요.

```text
이 NME 문서를 읽고, 제 .nme 프로그램의 버그를 찾아 주세요: 원인을 설명하고 가장 작은 수정을 제안하며 nme 검사로 확인하라고 알려 주세요.
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.ko.md
```

```text
Read this NME handoff, then find the bug in my .nme program: explain the cause, suggest the smallest fix, and ask me to run nme check to verify:
https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/ai-assistants.md
```

## 이 페이지를 읽는 도우미가 따라야 할 규칙

NME에는 줄마다 섞을 수 있는 세 단계가 있습니다.

1. 문장형: 코드용 특수문자가 거의 없으며 처음 배우는 사람에게 기본으로 사용
2. 초급: 짧고 정확하며 임의의 Python 표현식이 필요할 때 사용
3. 고급: 올바른 Python과 완전히 같고 변경되지 않음

한국어와 영어는 별도 모드가 아니라 뜻이 같은 표현입니다.

```text
이름을 물어봐 이름이 뭐예요?
show Hello 이름!
3 times 반복해서 Welcome 말해줘
이름이 뭐예요?
안녕하세요 이름!
```

### 먼저 문장형으로 작성

```text
안녕하세요 말해줘
Hello world show
오늘도 반가워요!
Hello everyone!

이름을 물어봐
name ask
Hello name show

이름이 뭐예요?
What is your name?
안녕하세요 이름!

3번 다시 만나요
3 times See you again

만약에 점수가 10보다 크면 성공 말해줘
if score is greater than 10 then show You won
점수가 5보다 크면 높아요 말해줘
색이 빨강과 같으면 맞아요 말해줘

정답은 1부터 10까지 랜덤정수
추측을 숫자로 물어봐 숫자를 맞혀 보세요
```

가장 쉬운 출력은 `오늘도 반가워요!`처럼 분명한 여러 단어 문장을 동작 단어
없이 쓰는 것입니다. 한 단어만 있는 줄은 Python 이름일 수 있어 `말해줘`나
`보여줘`를 붙여야 출력됩니다. `repaet`, `shwoe` 같은 오타는 뜻이 하나로
정해질 때만 자동으로 고칩니다. `그리거`, `같먄` 같은 논리·조건 연결어 오타도
같은 제한으로 복구합니다.

문장 출력에서 앞서 입력받거나 저장한 이름은 실제 값으로 바뀝니다. `내 이름은
뭐예요?`처럼 평범한 질문을 쓰면 이름에 답이 자동으로 저장됩니다. 마지막 `?`는
생략할 수 있고, 따옴표 없는 질문에는 끝 공백이 자동으로 붙습니다.
`How old are you?`, `몇 살이에요?` 같은 나이 질문은 각각
`age`, `나이`에 자동으로 저장됩니다.

가장 쉬운 시작을 위해, 처음에는 들여쓰기 대신 `끝`/`end`로 블록을 닫으세요.

```text
동안 점수 < 3
점수 말해줘
점수에 1 더해
끝
```

같은 블록에서 `멈춰`/`break`, `그리고`/`and`, `또는`/`or`, `아니면 만약에`/`elif`,
`아니면`/`else`를 사용할 수 있습니다. 네 칸 들여쓰기와 일반 Python도 계속
유효합니다.

### 정확해야 하면 초급 문법 사용

```text
말해 <Python 표현식>
say <Python expression>
물어봐 이름, <Python 질문>
ask name, <Python prompt>
횟수번:
count times:
3번:
첫 줄 말해줘
끝
만약 <Python 조건>:
when <Python condition>:
동안 <조건> ... 끝
while <condition> ... end
멈춰 / break
아니면 만약에 <조건> / else if <condition>
아니면 / else
랜덤 사용 최신
use random
```

`횟수번:`은 `횟수`라는 변수에 적힌 숫자만큼 반복해요 (변수에 숫자가 있어야
해요). `count times:`는 변수 `count`로 같은 뜻이에요.

### 모든 Python을 고급 NME로 사용

쉬운 표현이 뜻을 바꿀 수 있다면 Python 문법을 억지로 번역하지 마세요. 함수,
클래스, import, 컴프리헨션, 비동기 코드, 예외 처리, 설치한 Python 패키지와
모든 올바른 Python 기능은 그대로 둘 수 있습니다.

올바른 Python이 항상 우선합니다. `say("x")`, `when`이라는 변수, 문자열
안의 글, 주석을 NME 문법으로 바꾸면 안 됩니다.

### 랜덤 모듈

내장 랜덤 어댑터 버전은 `0.0.1`입니다. `랜덤 사용 최신`과
`use random latest`가 로컬에서 이 버전을 고릅니다. 어느 쪽으로 불러도 다음
두 언어 이름이 모두 생깁니다.

- `랜덤정수` / `random_number`
- `랜덤선택` / `random_pick`
- `섞기` / `shuffle`
- `랜덤버전` / `random_version`

문장형 랜덤에는 import가 필요 없습니다.

```text
주사위는 1부터 6까지 랜덤정수
색은 빨강 또는 초록 또는 파랑 중에서 랜덤선택
```

### 만들어 내지 말고 검사

파일을 작성하거나 고친 뒤 실행하세요.

```sh
nme 버전
nme 검사 program
nme 빌드 program -o program.py
```

`검사`와 `빌드`는 생성된 Python을 선택한 CPython으로 컴파일해 확인하지만
실행하지는 않습니다. `nme 도움`처럼 한국어 명령을 쓰면 한국어 안내 뒤에 같은
내용의 영어 안내도 나옵니다. `nme 네이티브`와 `nme 설치`의 오류도 한국어 우선
한영 형식입니다. `nme help`와 영어 명령은 영어만 출력합니다.

실행해도 안전하고 사용자가 원하면 `nme 실행 program`을 사용합니다.
네이티브 결과물을 원하고 Nuitka가 설치됐을 때만 `nme 컴파일`을 사용합니다.

문서에 나온 NME 동작만 한국어와 영어 별칭을 지원합니다. 고급 문법은 Python이므로
`def`, `for`, `import`, `return` 같은 Python 키워드는 그대로 사용합니다. 없는
한국어 고급 키워드를 만들어 내지 마세요.

요청한 문장이 여기에 없다면 전체
[한국어 문법 레퍼런스](https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/language.ko.md)나
[영어 레퍼런스](https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/language.md)를
읽으세요. 지원하지 않는 NME 키워드를 만들어 내지 말고 항상 올바른 고급 NME인
Python으로 돌아가세요.

## 일반 대화형 AI와 NME 사용하기

ChatGPT, Claude(채팅), Gemini, Grok은 링크로 웹 페이지를 읽을 수 있으므로
같은 짧은 문장 + 링크 하나 설계가 그대로 작동하고 프로젝트에 파일이 필요
없습니다. 이 페이지 맨 위의 문장이나 위의 목적별 시작 문장을 새 채팅에
붙여 넣고, 같은 채팅에서 계속 질문하면 안내 문서가 대화 맥락에 남습니다.

- ChatGPT: 문장과 링크를 채팅에 붙이고 .nme 파일을 요청하세요.
- Claude(채팅, claude.ai): 같은 문장과 링크를 붙이세요. Claude는 답하기
  전에 링크에서 페이지를 읽습니다.
- Gemini: 문장과 링크를 붙이고 이어진 질문으로 결과를 다듬으세요.
- Grok: 문장과 링크를 한 메시지에 붙이고 그 대화를 이어 가세요.

이 문서에 없는 표현이 있으면 이 중 아무 AI나
[문법 레퍼런스](https://raw.githubusercontent.com/needmoretruth/needmoreeasy/beta/docs/language.ko.md)도
읽을 수 있습니다. 이런 대화형 AI는 보통 nme CLI를 직접 실행하지 못하므로,
프로그램을 받은 뒤에는 직접 `nme 검사`(또는 `nme c`)를 실행하고 오류를 다시
채팅에 붙여 주세요.

## 제품별 링크 전달 방법

- Cursor: URL을 `@Link`로 붙입니다. Cursor 공식
  [`@Link` 안내](https://docs.cursor.com/context/%40-symbols/%40-link)를
  참고하세요.
- Claude Code: 세션을 시작할 때 전달 문장을 붙입니다. Anthropic 공식
  [메모리 문서](https://code.claude.com/docs/en/memory)의 `CLAUDE.md`도 선택적으로
  쓸 수 있지만 NME에는 필요하지 않습니다.
- Codex: 전달 문장을 작업으로 붙입니다. Codex는 OpenAI 공식
  [안내](https://openai.com/index/introducing-codex/)에 나온 `AGENTS.md` 지침도
  지원하지만 NME 프로젝트에서 추적할 필요는 없습니다.
- OpenCode: 전달 문장을 붙입니다. 공식
  [규칙 문서](https://dev.opencode.ai/docs/rules/)는 사용자가 지속 설정을 원할 때
  원격 지침 URL도 지원합니다.

저장소 소유자가 명시적으로 추적하라고 하지 않았다면 도구별 AI 메타데이터는
NME 프로그램 저장소 밖에 두세요.
