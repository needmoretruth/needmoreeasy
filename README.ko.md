# NeedMoreEasy (NME)

[English](README.md) | 한국어

**Python도 어렵다면 더 쉬운 문장부터 시작해서, 한 줄씩 Python으로
성장하세요.** NME는 학습을 위한 다리입니다. 평범한 문장으로 시작하고, 익숙해지면
초급 문법을 섞고, 같은 파일의 일부를 조금씩 Python으로 바꿉니다. 다른 언어로
프로젝트를 처음부터 다시 만들 필요가 없습니다.

가장 쉬운 문장형으로 시작하고, 더 정확한 초급 문법을 섞고, 한 줄씩 일반
Python으로 바꾸면 됩니다. 세 단계는 영어·한국어 어느 쪽으로도, 두 언어를
섞어서도 함께 쓸 수 있습니다.

```text
이름이 뭐예요?
안녕하세요 이름!
3번 환영합니다
```

배우는 동안에는 들여쓰기를 하지 않고 `끝`으로 블록을 닫을 수 있습니다.

```text
점수는 0
점수가 3보다 작을 동안
점수 말해줘
점수에 1 더해
끝
```

한 프로그램에서 한국어, 영어, Python을 마음대로 섞어도 됩니다.

```text
ask 이름 What is your name?
show Hello 이름!
3 times 반복해서 Welcome to NME 말해줘
```

언어 모드를 선언할 필요가 없습니다.

질문도 말하듯이 쓸 수 있습니다. `이름이 뭐예요?`는 `이름`에 답을 저장하고,
영어 `What is your name`도 같은 방식으로 동작합니다. 마지막 `?`는 생략해도
됩니다. 더 복잡한 질문이나 숫자 입력에는 `물어봐`/`ask`를 사용하세요.

## 한 언어 안의 세 단계

| 단계 | 용도 | 예시 |
| --- | --- | --- |
| 문장형 | 코딩 첫날, 코드용 특수문자를 거의 쓰지 않음 | `3번 반복해서 안녕 말해줘` |
| 초급 | 짧고 정확하며 실용적인 NME | `3번: 말해 "안녕"` |
| 고급 | Python과 문법이 완전히 같음 | `for i in range(3): print(i)` |

세 단계는 서로 다른 모드가 아닙니다. 줄마다 원하는 방식을 쓰면 됩니다. 올바른
Python이 항상 우선하며 한 글자도 바뀌지 않습니다.

문장형은 `만약에`, `있으면`, `반복해서`, `그리고`, `또는`, `then` 같은 연결어를
이해합니다. `동안`/`while`, `멈춰`/`break`, `아니면 만약에` 또는
`아니면만약에`/`elif`,
`아니면`/`else`도 `끝` 블록에서 사용할 수 있습니다.
Python이 아닌 NME 동작 단어에 한 글자 오타가 있으면 고쳐서 이해하고,
`shwoe` → `show`처럼 흔한 여분 글자+자리 바꿈도 복구합니다. `Hello everyone!`처럼
분명한 여러 단어 문장은 동작 단어 없이도 출력합니다. 한 단어만 있는 줄은 Python이
항상 우선하므로 일반 Python으로 남습니다. 뜻을 하나로 확정하기 어렵다면 억지로
추측하지 않고 정확한 위치와 고치는 예시를 보여 줍니다. 조건은 `만약`을 빼고
주어부터 말해도 됩니다. `색이 빨강과 같으면 맞아요 말해줘`처럼 쓰고, 여러 줄
블록은 `끝`으로 닫습니다.

## 베타 설치

현재 NME는 소스에서 빌드해 설치합니다. 안정 Rust, Python 3.8 이상, Git을
설치한 다음 실행하세요.

```sh
git clone --branch beta https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"
nme --version
```

Cargo가 설치한 `bin` 폴더가 `PATH`에 없다고 경고하면 macOS/Linux의 현재
터미널에서는 `export` 줄을 반드시 먼저 실행해야 합니다. NME를 다시 설치하는
명령이 아닙니다. Windows PowerShell은
[설치 안내](docs/install.ko.md#windows-10-또는-11)의 PATH 단계를 사용하세요.

표시될 버전은 `nme 0.0.1-beta.9`입니다.

Windows, macOS, Linux별 과정은 [설치 안내](docs/install.ko.md)에 있습니다.
프로그래밍을 전혀 모른다면 [5분 시작 안내](docs/getting-started.ko.md)부터
따라 하세요.

## 사용하기

```sh
nme 실행 examples/hello-sentence
nme 검사 examples/guessing-game.ko
nme 빌드 examples/three-levels -o three-levels.py
nme 모듈
```

`.nme`는 생략할 수 있습니다. `program.nme`는 `nme 실행 program`만으로
실행됩니다. NME가 운영체제에 맞는 Python 명령을 자동으로 고르므로 보통
`--python`을 쓸 필요가 없습니다.

`실행`은 개발할 때 쓰는 지름길입니다. NME를 Python으로 컴파일한 뒤 CPython을
시작합니다. `빌드`는 컴파일된 Python 파일을 만듭니다. 독립 실행 파일이
필요하면 Nuitka를 설치하고 네이티브 컴파일을 사용하세요.

```sh
python3 -m pip install nuitka
nme 컴파일 examples/hello-sentence.nme -o hello
```

네이티브 파일은 실행할 운영체제에서 각각 빌드해야 합니다. 시작 속도, 파일
크기, 실행 속도는 프로그램에 따라 달라지므로 직접 측정해야 합니다. Python과
완전히 호환되는 모든 프로그램이 무조건 더 빠르고 작아진다는 거짓 보장은 하지
않습니다.

## 버전이 있는 랜덤 도구

```text
랜덤 사용 최신
random_number(1, 6) 말해줘
랜덤선택(["빨강", "파랑"]) 말해줘
```

`random` / `랜덤` 어댑터 `0.0.1`이 NME 안에 들어 있습니다. 따라서 `최신`은
인터넷에서 받을 필요 없이 들어 있는 최신 버전을 고릅니다. 한 번 불러오면
한국어와 영어 도구 이름이 모두 생기므로 같은 줄에서도 섞어 쓸 수 있습니다.
`nme 모듈`로 설치된 버전을 확인합니다.

문장형에서는 모듈 선언이나 특수문자 없이도 랜덤을 바로 쓸 수 있습니다.

```text
주사위는 1부터 6까지 랜덤정수
주사위 말해줘
색은 빨강 또는 초록 또는 파랑 중에서 랜덤선택
```

## Python 코드를 쉬운 NME로 바꾸기

단계와 출력 언어를 고릅니다.

```sh
nme 변환 app.py --level 고급 --language 한국어
nme 변환 app.py --level 초급 --language 한국어 -o app.nme
nme 변환 app.py --level 문장형 --language 한국어 -o app.nme
```

뜻을 그대로 보존할 수 있는 문장은 고른 단계로 바꾸고, 같은 뜻의 쉬운 문법이
없는 Python 문장은 고급 문법으로 남깁니다. 고급 문법도 올바른 NME입니다.
자세한 내용은 [Python 변환](docs/converting-python.ko.md)을 참고하세요.

## 배우고 도구 연결하기

- [문법 레퍼런스](docs/language.ko.md) — 세 단계, 정확한 뜻, 오타 복구,
  혼용, 모듈, 제한
- [학습 과정](docs/tutorial.ko.md) — Hello World, 대화, 숫자 맞히기,
  Python 혼용, NME로 작은 컴파일러 만들기
- [VS Code, Cursor, Zed](docs/editors.ko.md) — 준비된 작업과 파일 설정
- [AI 코딩 도우미](docs/ai-assistants.ko.md) — Claude Code, Codex,
  Cursor Agent, OpenCode에 링크 하나만 전달하는 방법
- [컴파일러 구조](docs/architecture.md) — 기여자용 설계 규칙
- [버전 정책](docs/versioning.ko.md), [변경 기록](CHANGELOG.ko.md)

## 컴파일 방식

NME는 별도의 Python 인터프리터가 아니라 컴파일러입니다. Rust로 만든 순수
코어가 NME 소스를 평범한 Python 소스로 컴파일합니다. Python 토큰화와 파싱은
`rustpython-parser`, 실행은 CPython 또는 선택적인 Nuitka 네이티브 백엔드가
담당합니다. 컴파일 전후 실제 줄 수를 같게 유지해서 오류의 줄 번호가 원본
`.nme` 파일과 맞습니다.

Apache-2.0으로 배포합니다.
