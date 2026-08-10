# NME 시작하기

[English](getting-started.md) | 한국어

NME를 처음 접한다고 가정하고 설명합니다. 첫 베타를 설치하고, 대화형 프로그램을
만들고, 조건과 반복을 사용하고, 기본 랜덤 도구를 써 본 뒤, 평범한 Python을
어떻게 섞는지 알아봅니다.

## 1. 필요한 도구 준비하기

다음 도구가 필요합니다.

- 최신 안정 Rust 도구 체인과 Cargo
- Python 3.8 이상
- 저장소를 복제할 때 사용할 Git

설치되었는지 확인합니다.

```sh
rustc --version
cargo --version
python3 --version
```

NME는 기본적으로 `python3`를 사용합니다. Python 명령이 `python`이라면
`nme run program.nme --python python`처럼 실행하세요.

## 2. NME 설치하기

```sh
git clone https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
```

첫 베타 버전을 확인합니다.

```sh
nme --version
```

```text
nme 0.0.1-beta.1
```

셸에서 `nme`를 찾지 못하면 Cargo가 보통 `$HOME/.cargo/bin`에 설치했는지
확인하세요. 설치하지 않고 내려받은 저장소에서 바로 실행할 수도 있습니다.

```sh
cargo run --quiet -p nme-cli -- run examples/korean.nme
```

## 3. 첫 프로그램 만들기

UTF-8 텍스트 파일 `hello.nme`를 만듭니다.

```text
물어봐 이름, "이름이 뭐예요? "
말해 f"안녕하세요, {이름}!"

3번:
    말해 "NME에 오신 것을 환영합니다."
```

실행합니다.

```sh
nme run hello.nme
```

이 프로그램은 이름을 물어보고 인사한 다음 마지막 문장을 세 번 보여 줍니다.

별도 언어 모드를 고르지 않고 같은 개념을 영어로 쓸 수도 있습니다.

```text
ask name, "What is your name? "
say f"Hello, {name}!"

3 times:
    say "Welcome to NME."
```

영어와 한국어 단어는 뜻이 같은 별칭이며 서로 섞어도 됩니다.

## 4. 다섯 가지 개념 배우기

### 값 보여 주기

```text
말해 "안녕"
말해 1 + 2
say "Hello"
say f"answer: {1 + 2}"
```

`말해`와 `say`는 Python `print(...)`로 바뀝니다.

### 글로 대답받기

```text
물어봐 도시, "어디에 사나요? "
ask city, "Where do you live? "
```

대답은 쉼표 앞의 이름에 저장됩니다. Python `input`처럼 대답은 글자입니다.
숫자가 필요하면 명시적으로 바꿉니다.

```text
물어봐 대답, "몇 살인가요? "
나이 = int(대답)
```

### 반복하기

```text
3번:
    말해 "다시"

3 times: say "Again"
```

여러 문장은 들여쓰고, 한 문장은 `:` 뒤에 바로 쓸 수 있습니다. `3번:`과
`3 번:`은 모두 동작합니다.

### 조건 확인하기

```text
점수 = 12

만약 점수 >= 10:
    말해 "성공!"

when 점수 < 10: say "Try again"
```

`만약`과 `when`은 Python `if`로 바뀝니다. 두 번째 갈래가 필요하면
평범한 Python `else`를 씁니다.

### 랜덤 도구 사용하기

```text
랜덤 사용

주사위 = 랜덤정수(1, 6)
색 = 랜덤선택(["빨강", "초록", "파랑"])
말해 f"주사위는 {주사위}, 색은 {색}"
```

영어 도구도 같은 방식입니다.

```text
use random

die = random_number(1, 6)
color = random_pick(["red", "green", "blue"])
say f"You rolled {die} and got {color}."
```

`섞기(목록)` 또는 `shuffle(list)`으로 목록을 그 자리에서 섞을 수 있습니다.
모두 Python에 기본 포함된 `random` 모듈을 사용하므로 다른 패키지를 설치하지
않습니다.

## 5. 필요할 때 평범한 Python 쓰기

NME는 별도 생태계가 아닙니다. 대입, 목록, 함수, import, 패키지는 Python을
그대로 씁니다.

```text
import math

def 원_넓이(반지름):
    return math.pi * 반지름**2

반지름 = 3
말해 원_넓이(반지름)

for 이름 in ["Ada", "Grace"]:
    말해 f"안녕하세요, {이름}"
```

올바른 Python은 항상 우선하며 바뀌지 않습니다. 따라서 `말해("안녕")`은
Python 함수 호출이고, `말해 "안녕"`은 NME 문장입니다.

## 6. 검사하고 빌드하기

프로그램을 실행하지 않고 NME 문법을 검사합니다.

```sh
nme check hello.nme
```

성공하면 아무것도 출력하지 않습니다. NME 문법이 잘못되면 위치와 고치는 방법을
표시하며, 한국어 문법에는 한국어 안내가 나옵니다. 이 명령이 CPython의 모든
문법 및 런타임 검사를 대신하지는 않습니다.

생성되는 Python을 화면에서 봅니다.

```sh
nme build hello.nme
```

파일에 저장합니다.

```sh
nme build hello.nme -o hello.py
python3 hello.py
```

출력은 평범한 Python입니다. NME는 빈 줄, 주석, 들여쓰기, 줄바꿈 방식, 실제
줄 수를 보존합니다.

## 7. CLI 레퍼런스

```text
nme run <file.nme> [--python <command>]
nme build <file.nme> [-o <output.py>]
nme check <file.nme>
nme --help
nme --version
```

- `run`: 변환 후 터미널 입출력을 그대로 사용하여 CPython 실행
- `build`: 실행 없이 변환하여 Python을 화면이나 파일에 출력
- `check`: 출력 파일 없이 토큰화와 NME 문법 검사

## 다음 단계

- `examples/korean.nme`, `examples/ask.nme`, `examples/hello.nme`,
  `examples/mixed.nme`를 실행해 보세요.
- 정확한 규칙은 [언어 레퍼런스](language.ko.md)를 곁에 두고 확인하세요.
- 베타 번호와 잠긴 `1.0.0` 규칙은 [버전 정책](versioning.ko.md)을
  참고하세요.
- NME 컴파일러 동작을 바꾸기 전에는 [아키텍처](architecture.md)를 읽어 주세요.
