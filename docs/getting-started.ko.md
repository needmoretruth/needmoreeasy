# NME 시작하기

[English](getting-started.md) | 한국어

이 튜토리얼은 새로 내려받은 저장소에서 NME 프로그램을 작성하고, 검사하고,
Python으로 변환하고, 실행하는 과정까지 안내합니다. 정확한 언어 규칙은
[언어 레퍼런스](language.ko.md)를 참고하세요.

## 1. 필요한 도구 설치하기

다음 도구가 필요합니다.

- 최신 안정 Rust 도구 체인과 Cargo
- Python 3.8 이상
- 저장소를 복제할 때 사용할 Git

도구가 준비되었는지 확인합니다.

```sh
rustc --version
cargo --version
python3 --version
```

NME는 기본적으로 `python3` 명령을 사용합니다. Python 명령이 `python`인
환경에서는 프로그램을 실행할 때 `--python python`을 지정하세요.

## 2. NME 빌드하고 설치하기

현재 NME는 소스에서 설치합니다.

```sh
git clone https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
```

설치 결과를 확인합니다.

```sh
nme --version
nme --help
```

`nme`가 `PATH`에 없다면 Cargo가 보통 `$HOME/.cargo/bin`에 설치했는지
확인하세요. 설치하지 않고 저장소에서 직접 실행할 수도 있습니다.

```sh
cargo run --quiet -p nme-cli -- run examples/hello.nme
```

## 3. 첫 프로그램 작성하기

UTF-8 텍스트 파일 `hello.nme`를 만듭니다.

```text
name = "친구"

say f"안녕하세요, {name}!"

3 times:
    say "NME에 오신 것을 환영합니다."
```

이 예제는 일부러 Python과 NME를 함께 사용합니다.

- `name = "친구"`와 f-string 표현식은 Python입니다.
- `say`는 하나의 값을 보여 주는 NME의 짧은 문법입니다.
- `3 times:`는 들여쓴 본문을 반복합니다.

## 4. 프로그램 실행하기

```sh
nme run hello.nme
```

예상 출력은 다음과 같습니다.

```text
안녕하세요, 친구!
NME에 오신 것을 환영합니다.
NME에 오신 것을 환영합니다.
NME에 오신 것을 환영합니다.
```

다른 인터프리터 명령을 선택하려면 다음과 같이 실행합니다.

```sh
nme run hello.nme --python python
```

`run`은 터미널의 표준 입력, 표준 출력, 표준 오류를 그대로 사용하며 Python
프로세스의 종료 상태를 반환합니다.

## 5. 생성된 Python 확인하기

변환된 프로그램을 화면에 출력합니다.

```sh
nme build hello.nme
```

파일에 저장하려면 다음과 같이 실행합니다.

```sh
nme build hello.nme -o hello.py
python3 hello.py
```

결과는 평범한 Python입니다. 빈 줄, 주석, 들여쓰기, 줄 번호가 유지됩니다.
Python으로만 작성한 소스는 바이트 단위까지 바뀌지 않습니다.

## 6. 실행하지 않고 검사하기

```sh
nme check hello.nme
```

검사에 성공하면 아무것도 출력하지 않고 성공 상태로 종료합니다. 실패하면 찾을
수 있는 모든 NME 문제를 위치 및 힌트와 함께 보여 줍니다. `check`는 토큰화와
NME 변환을 검사하지만 프로그램을 실행하거나 CPython의 런타임 검사를 대신하지
않습니다.

예를 들어 다음 코드는 `+` 다음 값이 빠졌습니다.

```text
say 1 +
```

NME는 잘못된 표현식의 위치를 표시하고 올바른 `say` 사용법을 제안합니다.

## 7. 필요한 Python 자유롭게 섞기

NME는 별도의 생태계가 아닙니다. 준비되는 만큼 평범한 Python을 추가하세요.

```text
def greet(name):
    say f"안녕하세요, {name}!"    # Python 함수 안의 NME

for name in ["Ada", "Grace"]:   # 평범한 Python 반복문
    greet(name)

2 times:                         # NME 반복문
    print("Python도 동작합니다")  # NME 안의 Python
```

올바른 Python은 항상 우선합니다. `say("안녕")`은 NME `say` 문장이 아니라
Python 함수 호출입니다. `times = 3`과 `if times:`도 평범한 Python입니다.

## CLI 레퍼런스

```text
nme run <file.nme> [--python <command>]
nme build <file.nme> [-o <output.py>]
nme check <file.nme>
nme --help
nme --version
```

### `nme run`

입력을 변환하고 CPython을 시작합니다. 기본 인터프리터 명령은 `python3`이며
`--python <명령>`으로 바꿀 수 있습니다. 트레이스백의 줄 번호는 원본 `.nme`
파일과 일치하지만, v0.1에서는 임시 `.py` 파일 이름이 표시될 수 있습니다.

### `nme build`

실행하지 않고 변환합니다. 출력 옵션이 없으면 Python 소스를 표준 출력으로
보냅니다. `-o <경로>` 또는 `--output <경로>`를 사용하면 파일에 저장합니다.

### `nme check`

토큰화 및 변환 단계를 실행하고 NME 진단을 표시합니다. 출력 파일을 만들지
않고 Python도 시작하지 않습니다.

## 다음에 볼 문서

- 전체 [NME 언어 레퍼런스](language.ko.md)
- [`examples/`](../examples/)의 실행 가능한 프로그램
- 기여하기 전에 읽어야 할 [컴파일러 아키텍처](architecture.md)
