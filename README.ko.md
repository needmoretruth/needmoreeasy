# NeedMoreEasy (NME)

[English](README.md) | 한국어

**더 쉽게 시작하고 자연스럽게 Python으로 이어지는 프로그래밍.** NME는 초보자를
위한 작고 이중언어를 지원하는 언어 계층입니다. 올바른 Python 프로그램은 모두
이미 올바른 NME 프로그램이며, 작게 엄선한 영어·한국어 문법으로 첫 코드를 더
짧고 읽기 쉽게 만들 수 있습니다.

현재 버전: **`0.0.1-beta.1`**

```text
랜덤 사용

이름 = "민아"
동물 = 랜덤선택(["고양이"])

만약 이름:
    2번: 말해 f"{이름}에게 {동물} 추천!"
```

같은 프로그램을 영어 문법으로 쓸 수도 있습니다.

```text
use random

name = "Mina"
pet = random_pick(["cat"])

when name:
    2 times: say f"{name} gets a {pet}!"
```

NME는 두 형식을 모두 평범한 Python으로 바꾸고 CPython으로 실행합니다. 한
파일에서 NME와 Python을 자유롭게 섞고, Python 패키지와 학습 자료도 그대로
쓰며, 처음부터 다시 시작하지 않고 조금씩 Python으로 넘어갈 수 있습니다.

> NME는 초기 베타입니다. 저장소 소유자의 명시적 지시 없이는 `1.0.0`을
> 출시할 수 없습니다. 자세한 내용은 [버전 정책](docs/versioning.ko.md)을
> 참고하세요.

## 왜 NME인가요?

- **영어 또는 한국어:** 한 파일에서도 원하는 문법을 골라 쓸 수 있습니다.
- **유용한 다섯 가지 개념:** 값 보여 주기, 글로 대답받기, 반복, 조건,
  랜덤 도구를 먼저 배웁니다.
- **Python을 모두 유지:** 대입, 목록, 함수, 클래스, import, 패키지와 한국어
  변수 이름이 평소처럼 동작합니다.
- **안전한 호환성:** 올바른 Python 줄은 항상 바이트 단위까지 그대로 남습니다.
- **친절한 오류:** NME 오류는 정확한 위치와 고치는 방법을 보여 주며, 한국어
  문법 오류에는 한국어 안내가 나옵니다.
- **정확한 줄 번호:** 변환 전후 실제 줄 수를 보존합니다.

## 소스에서 설치하기

필요한 도구:

- 최신 안정 Rust 도구 체인과 Cargo
- Python 3.8 이상
- 저장소를 복제할 때 사용할 Git

```sh
git clone https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
nme --version
```

예상 버전:

```text
nme 0.0.1-beta.1
```

설치하지 않고 저장소에서 바로 실행할 수도 있습니다.

```sh
cargo run --quiet -p nme-cli -- run examples/hello.nme
cargo run --quiet -p nme-cli -- run examples/ask.nme
cargo run --quiet -p nme-cli -- run examples/korean.nme
```

## 빠르게 시작하기

`hello.nme` 파일을 만듭니다.

```text
물어봐 이름, "이름이 뭐예요? "

말해 f"안녕하세요, {이름}!"

3번:
    말해 "NME가 잘 동작합니다."
```

실행합니다.

```sh
nme run hello.nme
```

Python을 실행하지 않고 검사합니다.

```sh
nme check hello.nme
```

생성되는 Python을 확인합니다.

```sh
nme build hello.nme
nme build hello.nme -o hello.py
```

## 문법 한눈에 보기

| 한국어 NME | 영어 NME | Python에서의 의미 |
| --- | --- | --- |
| `말해 값` | `say value` | `print(value)` |
| `물어봐 이름` | `ask name` | `name = input()` |
| `물어봐 이름, 질문` | `ask name, prompt` | `name = input(prompt)` |
| `횟수번:` | `count times:` | `for _ in range(count):` |
| `만약 조건:` | `when condition:` | `if (condition):` |
| `랜덤 사용` | `use random` | Python 기본 랜덤 도구 가져오기 |

`번`/`times`와 `만약`/`when` 뒤에는 들여쓴 여러 문장 또는 콜론 뒤의
한 문장을 쓸 수 있습니다. 표현식은 평범한 Python 표현식이며 적은 그대로
복사됩니다.

`랜덤 사용` 다음에는 아래 이름을 바로 쓸 수 있습니다.

- `랜덤정수(시작, 끝)`: 양 끝을 포함한 임의의 정수
- `랜덤선택(값들)`: 값 하나 고르기
- `섞기(목록)`: 목록의 순서를 그 자리에서 섞기

영어 `use random`은 같은 도구를 `random_number`, `random_pick`,
`shuffle`이라는 이름으로 제공합니다. 모두 Python에 포함된 `random` 모듈을
사용하므로 별도 패키지를 설치하지 않습니다.

가장 중요한 규칙은 **Python 우선**입니다. 예를 들어 `말해("안녕")`,
`say("hello")`, `물어봐 = input`, `times = 5`는 올바른 Python이므로
바뀌지 않습니다. NME는 Python이 그 줄을 거부할 때만 더 쉬운 문법으로
인식합니다.

정확한 문법, 동작, 생성되는 Python, 오류와 제한은
[전체 언어 레퍼런스](docs/language.ko.md)를 참고하세요.

## 명령줄

| 명령 | 용도 |
| --- | --- |
| `nme run program.nme` | CPython으로 변환하고 실행 |
| `nme run program.nme --python python` | 다른 Python 명령 선택 |
| `nme build program.nme` | 생성된 Python 출력 |
| `nme build program.nme -o program.py` | 생성된 Python을 파일에 저장 |
| `nme check program.nme` | 실행하지 않고 NME 검사 |
| `nme --help` | 명령 도움말 |
| `nme --version` | 설치된 NME 버전 |

## 문서

영어가 기본 문서 언어이며, 모든 사용자 가이드는 한국어 문서도 함께
관리합니다.

| 주제 | 한국어 | English |
| --- | --- | --- |
| 첫 프로그램과 CLI 튜토리얼 | [시작하기](docs/getting-started.ko.md) | [Getting started](docs/getting-started.md) |
| 정확한 문법과 동작 | [언어 레퍼런스](docs/language.ko.md) | [Language reference](docs/language.md) |
| 버전과 출시 규칙 | [버전 정책](docs/versioning.ko.md) | [Versioning](docs/versioning.md) |
| 출시 변경 사항 | [변경 기록](CHANGELOG.ko.md) | [Changelog](CHANGELOG.md) |
| 컴파일러 설계 | — | [Architecture](docs/architecture.md) |

## 동작 방식

```text
.nme 소스
    → Python을 이해하는 토큰화
    → NME 인식(올바른 Python 우선)
    → 줄 수를 보존한 Python 소스
    → CPython
```

컴파일러 코어는 IO가 없는 순수한 소스 변환 함수입니다. 파일 접근과 Python
실행은 CLI가 담당합니다. 컴파일러 동작을 변경하기 전에는
[아키텍처](docs/architecture.md)를 읽어 주세요.

## 기여하기

변경 범위를 작게 유지하고 Rust 동작을 아래 명령으로 검증합니다.

```sh
cargo fmt --all
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

질문과 버그 제보는
[GitHub Issues](https://github.com/needmoretruth/needmoreeasy/issues)에 남겨 주세요.

## 라이선스

NeedMoreEasy는 오직 [Apache License 2.0](LICENSE)으로 배포됩니다.
