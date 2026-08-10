# NeedMoreEasy (NME)

[English](README.md) | 한국어

**Python보다 더 쉬운 프로그래밍.** NeedMoreEasy는 Python도 어렵게 느껴지는
사람을 위한 아주 얇은 언어 계층입니다. 올바른 Python 프로그램은 모두 이미
올바른 NME 프로그램이며, NME는 초보자를 위한 두 가지 표현만 더합니다.

```text
5 times:
    say "안녕!"
```

NME는 이 소스를 평범한 Python으로 바꾸고 CPython으로 실행합니다.

```python
for _ in range(5):
    print("안녕!")
```

하나의 `.nme` 파일에서 Python과 NME를 자유롭게 섞어 쓸 수 있습니다. 별도의
런타임이나 표준 라이브러리를 새로 배울 필요가 없습니다.

> **프로젝트 상태:** NME는 초기 v0.1 기반을 만드는 단계입니다. 현재 언어는
> 의도적으로 작은 `say`, `times:` 두 문법만 제공합니다. 아직 배포 패키지보다
> 소스에서 빌드하여 사용하는 단계입니다.

## 왜 NME인가요?

- **더 작게 시작합니다:** 긴 Python 표현을 배우기 전에 `say`와 `times:`부터
  사용할 수 있습니다.
- **Python을 그대로 씁니다:** import, 함수, 클래스, 패키지, 튜토리얼이 모두
  계속 동작합니다.
- **안전하게 호환됩니다:** 올바른 Python이 항상 우선하며 NME로 잘못 해석되지
  않습니다.
- **오류가 친절합니다:** NME 오류에는 정확한 위치와 고치는 방법이 함께
  표시됩니다.
- **트레이스백이 유용합니다:** 변환 전후의 줄 번호가 유지됩니다.

## 준비 사항

- NME 빌드에 필요한 최신 안정 Rust 도구 체인과 Cargo
- `nme run` 실행에 필요한 Python 3.8 이상

`nme build`와 `nme check`는 Python을 실행하지 않습니다. `run`은 기본적으로
`python3`를 사용하며 `--python`으로 다른 명령을 지정할 수 있습니다.

## 소스에서 설치하기

```sh
git clone https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
nme --version
```

Cargo는 보통 `$HOME/.cargo/bin`인 Cargo 바이너리 디렉터리에 `nme` 실행 파일을
설치합니다. 셸에서 `nme`를 찾지 못하면 이 디렉터리를 `PATH`에 추가하거나
프로젝트를 직접 실행하세요.

```sh
cargo run --quiet -p nme-cli -- run examples/hello.nme
```

## 빠르게 시작하기

`hello.nme` 파일을 만듭니다.

```text
name = "NME"

say f"{name}에서 보낸 인사!"

3 times:
    say "정말 쉽습니다."
```

다음 명령으로 실행합니다.

```sh
nme run hello.nme
```

```text
NME에서 보낸 인사!
정말 쉽습니다.
정말 쉽습니다.
정말 쉽습니다.
```

## 문법 한눈에 보기

| NME 소스 | Python에서의 의미 |
| --- | --- |
| `say <표현식>` | `print(<표현식>)` |
| `<표현식> times:`와 들여쓴 본문 | `for _ in range(<표현식>):` |
| `<표현식> times: <문장>` | 한 문장을 같은 줄에서 실행하는 반복문 |
| 올바른 모든 Python | Python 그대로 바이트 단위까지 유지 |

가장 중요한 규칙은 **Python 우선**입니다. 예를 들어 `say("안녕")`은 평범한
Python 함수 호출로, `times = 5`는 대입문으로, `if times:`는 Python `if` 문으로
남습니다. NME는 Python이 같은 줄을 받아들이지 못할 때만 더 쉬운 NME 문법으로
인식합니다.

정확한 문법, 의미, 호환 규칙, 현재 제약은
[언어 레퍼런스](docs/language.ko.md)에서 확인할 수 있습니다.

## 명령줄 사용법

| 명령 | 용도 |
| --- | --- |
| `nme run program.nme` | CPython으로 변환하고 실행 |
| `nme run program.nme --python python` | 다른 Python 명령으로 실행 |
| `nme build program.nme` | 생성된 Python을 표준 출력에 표시 |
| `nme build program.nme -o program.py` | 생성된 Python을 파일에 저장 |
| `nme check program.nme` | 실행하지 않고 토큰화와 NME 변환을 검사 |
| `nme --help` | 명령 도움말 표시 |

저장소의 예제도 바로 실행할 수 있습니다.

```sh
nme run examples/hello.nme
nme run examples/mixed.nme
nme run examples/pure_python.nme
```

## 문서

| 주제 | English | 한국어 |
| --- | --- | --- |
| 첫 프로그램과 CLI 튜토리얼 | [Getting started](docs/getting-started.md) | [시작하기](docs/getting-started.ko.md) |
| 전체 NME 문법 | [Language reference](docs/language.md) | [언어 레퍼런스](docs/language.ko.md) |
| 컴파일러 설계와 불변 조건 | [Architecture](docs/architecture.md) | — |

아키텍처 문서는 기여자를 위한 컴파일러 불변 조건의 단일 기준이므로 영어로
유지합니다.

## 동작 방식

```text
.nme 소스
    → Python을 이해하는 토큰화
    → NME 문장 인식(올바른 Python 우선)
    → 줄 번호를 보존한 Python 소스
    → CPython
```

컴파일러 코어는 IO가 없는 순수한 소스 변환 함수입니다. 파일 접근과 Python
실행은 CLI가 담당합니다. 설계 근거는
[docs/architecture.md](docs/architecture.md)를 참고하세요.

## 저장소 구조

```text
crates/nme-core/   순수 NME → Python 컴파일러
crates/nme-cli/    nme 명령, 파일 IO, Python 실행
docs/              사용자 문서와 컴파일러 아키텍처
examples/          실행 가능한 NME 및 Python/NME 혼합 프로그램
```

## 기여하기

코드를 변경하기 전에 [AGENTS.md](AGENTS.md)와
[docs/architecture.md](docs/architecture.md)를 읽어 주세요. 모든 변경은
프로젝트를 작게 유지하고 아래 검증을 통과해야 합니다.

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
