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
주어부터 말해도 됩니다:

```text
set score to 6
score is greater than 5 then show high
색은 "빨강"
색이 빨강과 같으면 맞아요 말해줘
```

여러 줄 블록은 `끝`으로 닫습니다.

## 베타 설치(정식 1.0 이전)

현재 공개 버전은 정식 1.0이 아닌 베타입니다. NME는 소스에서 빌드합니다.
먼저 [운영체제별 설치 안내](docs/install.ko.md)를 읽거나 다음 명령을
실행하세요.

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
[설치 안내](docs/install.ko.md#windows-11)의 PATH 단계를 사용하세요.

표시될 버전은 `nme 0.0.1-beta.15`입니다.

Windows, macOS, Linux별 과정은 [설치 안내](docs/install.ko.md)에 있습니다.
프로그래밍을 전혀 모른다면 [5분 시작 안내](docs/getting-started.ko.md)부터
따라 하세요.

## 사용하기

```sh
nme 실행 examples/hello-sentence
nme 검사 examples/guessing-game.ko
nme 빌드 examples/three-levels -o three-levels.py
nme 실행 examples/guessing-game.ko
nme 모듈
```

조금 더 큰 학습 프로젝트는 같은 타임루프 추리 게임을 문장형
[`time-loop-sentence.ko.nme`](examples/time-loop-sentence.ko.nme), 초급형
[`time-loop-beginner.ko.nme`](examples/time-loop-beginner.ko.nme), 고급 Python
[`time-loop-python.nme`](examples/time-loop-python.nme)으로 비교해 보세요.

한국어 문장형만으로 만든 더 큰 예제를 보고 싶다면
[`roulette.nme`](examples/roulette.nme)를 실행해 보세요. 질문, 조건, 반복,
랜덤 숫자, 값 변경을 하나의 초보자용 프로그램에서 함께 연습할 수 있습니다.
같은 내용을 영어로 쓴 [`roulette.en.nme`](examples/roulette.en.nme)도 있습니다.

블록체인이 데이터를 저장하고, 보호하고, 합의에 이르는 방법을 배우려면 교육용
프로젝트 네 개를 따라가 보세요(학습용이며 투자 조언이 아닙니다):
[`blockchain-ledger.nme`](examples/blockchain-ledger.nme)(초급),
[`proof-of-work.nme`](examples/proof-of-work.nme)(중급),
[`signatures.nme`](examples/signatures.nme)(고급),
[`consensus.nme`](examples/consensus.nme)(초고급). 각각 한국어 버전도
있습니다.

NME는 컴파일러도 쓸 수 있습니다: [`bootstrap.nme`](examples/bootstrap.nme)가 아주 작은 언어를 Python으로 변환해 실행합니다 — 셀프호스팅의 씨앗.

Python 패키지는 NME 안에서 일반 import로 씁니다 — `datetime` 패키지로 만든 [`birthday.nme`](examples/birthday.nme) 카운트다운을 보세요.

네트워크와 터미널 프로그램은 NME 안에서 일반 Python으로 씁니다:
[`http-client.nme`](examples/http-client.nme)는 로컬 서버에서 페이지를
받아 오고, [`terminal-menu.nme`](examples/terminal-menu.nme)는 터미널에서
작동하는 작은 메뉴 반복문입니다.

`.nme`는 생략할 수 있습니다. `program.nme`는 `nme 실행 program`과 `nme
program` 모두 실행됩니다. NME가 운영체제에 맞는 Python 명령을 자동으로 고르므로
보통 `--python`을 쓸 필요가 없습니다.

짧은 명령도 같은 뜻입니다. `nme r program`은 실행, `nme c program`은 검사,
`nme b program`은 빌드입니다. 파일 이름 없이 `nme r`만 실행하면 현재 폴더에
`.nme` 프로그램이 하나일 때 그 프로그램을 바로 실행하고, 여러 개일 때는
목록을 보여 주고 어느 것을 실행할지 물어봅니다. `nme c`와 `nme b`도 검사와
빌드에서 같은 방식으로 동작합니다. `nme m`, `nme v`, `nme h`는 각각
`nme 모듈`, `nme --version`, `nme 도움`의 짧은 형태입니다. `nme comp
program`은 Nuitka로 컴파일, `nme conv app.py`는 Python을 NME로 변환하며,
`nme 설치 requests`는 Python 패키지를 pip으로 설치합니다.

NME의 코어 부분집합은 `nme 네이티브 실행 hello`로 바로 네이티브 기계어로 컴파일할 수 있습니다(정수 값, 문장형 `while`/`if`/`else`, `break`, 재귀 함수, `say` — [`native-factorial.ko.nme`](examples/native-factorial.ko.nme)(영어판 [`native-factorial.nme`](examples/native-factorial.nme)) 시도; 그 외에는 CPython으로 실행). [네이티브 백엔드 조사](docs/native-backend.ko.md)를 보세요.

프로그램 이름도 겹치지 않는 범위에서 줄여 쓸 수 있습니다: `nme r gue`는
`guessing-game.nme`를 실행합니다. 여러 프로그램이 일치하면 추측하지 않고
목록을 보여 준 뒤 이름을 더 입력하라고 안내합니다.

모든 오류 메시지에는 `E0102` 같은 안정적인 코드가 함께 표시됩니다. 메시지가
이해되지 않으면 `nme ko E0102`로 자세한 한국어 설명(영어 설명 포함)을,
`nme en E0102`로 영어 설명을 볼 수 있습니다. `nme ko`만 실행하면 모든
코드를 나열해 줍니다.

`실행`은 개발할 때 쓰는 지름길입니다. NME를 Python으로 컴파일한 뒤 CPython을
시작합니다. `빌드`는 컴파일된 Python 파일을 만듭니다. 독립 실행 파일이
필요하면 Nuitka를 설치하고 네이티브 컴파일을 사용하세요.

```sh
python3 -m pip install nuitka
nme 컴파일 examples/hello-sentence.nme -o hello
```

(설치 안내에는 선택적인 `[app]` 구성요소가 포함되어 있습니다.)

네이티브 파일은 실행할 운영체제에서 각각 빌드해야 합니다. 시작 속도, 파일
크기, 실행 속도는 프로그램에 따라 달라지므로 직접 측정해야 합니다. Python과
완전히 호환되는 모든 프로그램이 무조건 더 빠르고 작아진다는 거짓 보장은 하지
않습니다.

## 버전이 붙은 랜덤·파일 도구

```text
랜덤 사용 최신
말해 random_number(1, 6)
말해 랜덤선택(["빨강", "파랑"])
```

`random` / `랜덤` 어댑터 `0.0.1`이 NME 안에 들어 있습니다. 따라서 `최신`은
인터넷에서 받을 필요 없이 들어 있는 최신 버전을 고릅니다. 한 번 불러오면
한국어와 영어 도구 이름이 모두 생기므로 같은 줄에서도 섞어 쓸 수 있습니다.

파일 읽기와 쓰기도 `file` / `파일`로 같은 방식으로 쓸 수 있습니다.

```text
파일 사용 최신
파일쓰기("note.txt", "안녕")
말해 파일읽기("note.txt")
점수 = {"이름": "민수", "점수": 3}
json저장("save.json", 점수)
보관 = json_load("save.json")
말해 보관["이름"]
```

`file`은 버전 `0.0.1`로 `파일읽기`/`file_read`, `파일쓰기`/`file_write`,
`json읽기`/`json_load`, `json저장`/`json_save`를 제공합니다.
`nme 모듈`로 설치된 버전과 이름을 확인합니다.

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
- [학습 가이드](docs/guides/index.ko.md) — 난이도, 선수 지식, 주제, 결과물을
  표시한 작은 점진적 가이드. 순서대로 배우거나 주제로 찾아볼 수 있습니다
- [학습 과정](docs/tutorial.ko.md) — 여섯 프로젝트: Hello World, 대화,
  숫자 맞히기, Python 혼용, 타임루프 게임, NME로 작은 컴파일러 만들기
- [VS Code, Cursor, Zed](docs/editors.ko.md) — 준비된 작업과 파일 설정
- [AI 코딩 도우미](docs/ai-assistants.ko.md) — Claude Code, Codex,
  Cursor Agent, OpenCode에 링크 하나만 전달하는 방법
- [컴파일러 구조](docs/architecture.md) — 기여자용 설계 규칙 (영어 문서)
- [네이티브 백엔드 조사](docs/native-backend.ko.md) — Python 호환과 분리된 진짜 NME 네이티브 AOT 컴파일러를 위한 정직한 계획
- [버전 정책](docs/versioning.ko.md), [변경 기록](CHANGELOG.ko.md)

## 컴파일 방식

NME는 별도의 Python 인터프리터가 아니라 컴파일러입니다. Rust로 만든 순수
코어가 NME 소스를 평범한 Python 소스로 컴파일합니다. Python 토큰화와 파싱은
`rustpython-parser`, 실행은 CPython 또는 선택적인 Nuitka 네이티브 백엔드가
담당합니다. 컴파일 전후 실제 줄 수를 같게 유지해서 오류의 줄 번호가 원본
`.nme` 파일과 맞습니다.

Apache-2.0으로 배포합니다.
