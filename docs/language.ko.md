# NME 언어 레퍼런스 — 0.0.1-beta.1

[English](language.md) | 한국어

이 문서는 NME의 정확한 문법과 생성되는 Python을 정의합니다. 첫 프로그램을
천천히 만들어 보고 싶다면 [NME 시작하기](getting-started.ko.md)를 먼저
읽어 보세요.

## 호환 규칙: Python 우선

올바른 모든 Python 프로그램은 올바른 NME 프로그램입니다. NME는 소스의 논리적
한 줄마다 실제 Python 파서에 먼저 물어봅니다. 그 줄이 이미 올바른 Python이면
바이트 단위까지 그대로 둡니다. Python이 거부한 줄만 NME 문법과 비교합니다.

| 소스 | 의미 |
| --- | --- |
| `say("hello")` | Python 함수 호출, 변경 없음 |
| `말해("안녕")` | 한국어 이름을 사용한 Python 함수 호출, 변경 없음 |
| `물어봐 = input` | Python 대입문, 변경 없음 |
| `times = 5` | Python 대입문, 변경 없음 |
| `if times:` | Python `if` 헤더, 변경 없음 |
| `say "hello"` | NME 출력 문장 |
| `말해 "안녕"` | 같은 뜻의 한국어 NME 출력 문장 |
| `5 times:` | NME 반복 블록 |
| `5번:` | 같은 뜻의 한국어 NME 반복 블록 |

따라서 NME 단어는 전역 예약어가 아닙니다. 문자열, f-string, 삼중 따옴표 문자열,
주석은 Python 방식으로 토큰화되므로 NME 코드로 잘못 인식되지 않습니다.

Python은 유니코드 식별자를 지원하므로 평범한 대입문과 표현식에서도 한국어
이름을 바로 쓸 수 있습니다.

```python
이름 = "민아"
좋아하는_수 = 7
친구들 = ["하나", "두리"]
```

## 작고 이중언어를 지원하는 문법

```text
말해 <Python 표현식>
say <Python expression>

물어봐 <간단한 이름>
물어봐 <간단한 이름>, <Python 질문 표현식>
ask <simple name>
ask <simple name>, <Python prompt expression>

<Python 표현식>번:
    <문장들>
<Python expression> times:
    <statements>

만약 <Python 표현식>:
    <문장들>
when <Python expression>:
    <statements>

랜덤 사용
use random
```

반복과 조건 문법은 콜론 뒤에 한 문장을 바로 쓰는 형식도 지원합니다. 영어와
한국어 표기를 섞어도 되며 의미는 항상 같습니다.

NME는 표현식을 다시 작성하지 않습니다. Python에 올바른 표현식인지 물어보고,
원본의 바이트 위치를 기록한 뒤, 생성되는 Python에 적은 그대로 복사합니다.

## `말해` / `say` — 값 하나 보여 주기

```text
말해 <표현식>
say <expression>
```

두 형식 모두 `print(표현식)`으로 바뀝니다.

```text
말해 "안녕하세요!"
say "Hello!"
말해 f"2 + 2 = {2 + 2}"
```

```python
print("안녕하세요!")
print("Hello!")
print(f"2 + 2 = {2 + 2}")
```

키워드 뒤에는 올바른 Python 표현식 하나가 와야 합니다. 여러 값을 출력하거나
`sep=`, `end=` 같은 옵션이 필요하면 Python `print(...)`를 직접 쓰세요.

`말해("안녕")`이나 `say("hello")`처럼 괄호로 호출하면 이미 올바른
Python이므로 Python 이름을 호출하는 코드로 남습니다. NME는 런타임 함수
`말해`나 `say`를 따로 만들지 않습니다.

## `물어봐` / `ask` — 글로 대답받기

질문 문구 없이 입력받기:

```text
물어봐 이름
ask name
```

```python
이름 = input()
name = input()
```

질문을 보여 주고 입력받기:

```text
물어봐 이름, "이름이 뭐예요? "
ask name, "What is your name? "
```

```python
이름 = input("이름이 뭐예요? ")
name = input("What is your name? ")
```

대답을 담을 곳에는 한국어 이름을 포함한 간단한 Python 식별자 하나를 씁니다.
쉼표 뒤의 선택적 질문은 모든 Python 표현식을 사용할 수 있습니다. Python
`input`과 마찬가지로 대답은 항상 글자입니다. 숫자가 필요하면 명시적으로
바꿉니다.

```text
물어봐 대답, "숫자는? "
숫자 = int(대답)
말해 숫자 + 1
```

`물어봐(이름)`, `ask(name)`, 이 이름들에 값을 넣는 대입문은 올바른
Python이므로 변경되지 않습니다.

## `번` / `times` — 반복하기

### 들여쓴 블록

```text
5번:
    말해 "안녕"

5 times:
    say "Hello"
```

두 형식 모두 Python `range` 반복문으로 바뀝니다.

```python
for _ in range(5):
    print("안녕")
```

한국어 `번`은 `5번:`처럼 횟수에 붙이거나 `5 번:`처럼 띄어 쓸 수
있습니다. 횟수에는 모든 올바른 Python 표현식을 사용할 수 있습니다.

```text
(2 + 3)번:
    말해 "다섯 번"

len(항목들) times:
    say "once per item"
```

### 한 줄 본문

```text
3번: 말해 "안녕"
3 times: say "Hi"
2번: print("평범한 Python")
```

콜론 뒤에는 정확히 한 문장만 쓸 수 있습니다. 최상위 세미콜론은 거부되므로
여러 문장은 들여쓴 블록으로 쓰세요. 한 줄 본문의 마지막에서 본문이 없는 새
NME 블록을 열 수는 없습니다.

생성되는 반복 변수는 `_`입니다. NME 반복 안이나 뒤에서 이 값에 의존하지
마세요. 횟수는 반복문에 들어갈 때 한 번 계산되며, 실행 시 Python `range`가
받아들일 수 있는 값이어야 합니다.

## `만약` / `when` — 조건에 따라 실행하기

### 들여쓴 블록

```text
만약 점수 >= 10:
    말해 "성공!"

when score >= 10:
    say "You won!"
```

평범한 Python `if`로 바뀝니다. 모든 올바른 Python 표현식을 헤더에서 안전하게
쓸 수 있도록 NME가 괄호를 붙입니다.

```python
if (점수 >= 10):
    print("성공!")
```

### 한 줄 본문

```text
만약 준비됨: 말해 "시작!"
when ready: say "Go!"
```

조건에는 모든 올바른 Python 표현식을 쓸 수 있습니다. 반복문과 같은 한 문장
및 들여쓰기 규칙을 적용합니다.

NME는 모든 Python 제어문에 별도 별칭을 만들지 않습니다. 다른 조건이 필요할
때는 Python `elif`와 `else`를 그대로 씁니다.

```text
만약 점수 >= 10:
    말해 "성공"
else:
    말해 "다시 도전"
```

이렇게 초보 문법은 작게 유지하면서도 자연스럽게 Python으로 확장할 수 있습니다.

## `랜덤 사용` / `use random` — 기본 랜덤 도구

Python에는 `random` 모듈이 이미 들어 있습니다. NME는 패키지 설치 없이
기억하기 쉬운 이름 몇 개를 한 줄로 제공합니다.

### 한국어 이름

```text
랜덤 사용

말해 랜덤정수(1, 6)
말해 랜덤선택(["빨강", "초록", "파랑"])

카드 = [1, 2, 3]
섞기(카드)
말해 카드
```

| 이름 | Python 함수 | 동작 |
| --- | --- | --- |
| `랜덤` | `random` 모듈 | 전체 Python 모듈 |
| `랜덤정수(가, 나)` | `random.randint(a, b)` | 양 끝을 포함한 임의의 정수 |
| `랜덤선택(값들)` | `random.choice(values)` | 값 하나 선택 |
| `섞기(값들)` | `random.shuffle(values)` | 변경 가능한 목록을 그 자리에서 섞기 |

### 영어 이름

```text
use random

say random_number(1, 6)
say random_pick(["red", "green", "blue"])

cards = [1, 2, 3]
shuffle(cards)
say cards
```

| Name | Python function | Behavior |
| --- | --- | --- |
| `random` | module | 전체 Python 모듈 |
| `random_number(a, b)` | `random.randint(a, b)` | 양 끝을 포함한 정수 |
| `random_pick(values)` | `random.choice(values)` | 값 하나 선택 |
| `shuffle(values)` | `random.shuffle(values)` | 목록을 그 자리에서 섞기 |

선언문은 같은 줄의 import와 평범한 이름 별칭으로 바뀝니다. 순수 Python 파일을
그대로 보존하기 위해 사용하겠다고 명시해야 합니다. 이 이름들은 현재 모듈의
평범한 변수가 되므로 선언은 파일 위쪽에 두고 다른 값의 이름으로 다시 쓰지
마세요.

이번 베타에서 쉬운 `사용` 문법을 제공하는 모듈은 `random` 하나뿐입니다.
나머지는 평범한 Python import를 씁니다.

```python
import math
from pathlib import Path
```

랜덤 도구는 게임, 예제, 시뮬레이션용이며 비밀번호나 보안 결정에는 사용하지
마세요.

## 영어, 한국어, Python 섞기

```text
랜덤 사용

def greet(이름):                 # 평범한 Python 함수
    말해 f"안녕하세요, {이름}!"   # 한국어 NME

for name in ["Ada", "Grace"]:   # 평범한 Python 반복문
    greet(name)

2 times:                        # 영어 NME
    만약 random_pick([True]):   # 한국어 NME + 영어 도우미
        print("모두 섞어도 됩니다")
```

영어와 한국어 키워드는 별도 언어 모드가 아니라 같은 문법의 별칭입니다. 옵션이나
파일 선언이 필요하지 않습니다.

## 소스 보존과 오류 안내

- Python 토큰화가 코드, 문자열, 주석을 구분합니다.
- 빈 줄, 들여쓰기, 주석, 줄바꿈 방식, 뒤쪽 주석을 보존합니다.
- NME 논리적 한 줄은 Python 논리적 한 줄로 변환됩니다.
- 생성 결과의 실제 줄 수가 같아서 트레이스백 줄 번호를 보존합니다.
- 잘못된 NME 문법에는 쉬운 메시지, 정확한 캐럿 위치, 고치는 힌트를 표시합니다.
- 가능하면 서로 독립적인 NME 문제를 한 번에 모읍니다.
- 한국어 NME 문법 오류에는 한국어 메시지와 힌트를 표시합니다.

예제:

```text
문장 = "3번: 말해 무엇"     # 문자열 내용, 변경 없음
# when ready: say "go"     # 주석, 변경 없음
말해 문장                   # NME, 뒤쪽 주석 보존
```

## 현재 베타의 제한

- NME는 의도적으로 이 문서의 다섯 개념만 제공합니다.
- 한 줄 반복 및 조건은 문장 하나만 받습니다.
- `물어봐`는 글자를 받으며 숫자 변환은 평범한 Python으로 합니다.
- 쉬운 별칭을 제공하는 모듈은 Python 기본 `random` 하나입니다.
- 일반 Python 문법 및 런타임 오류는 계속 CPython이 표시합니다.
- `nme check`는 토큰화와 NME 문법을 검사하며 CPython의 모든 문법·런타임
  검사를 대신하지 않습니다.
- 실행에는 CPython 인터프리터가 필요하지만 빌드와 검사에는 필요하지 않습니다.
- 트레이스백 줄 번호는 보존되지만 원본 `.nme` 경로 대신 임시 `.py` 파일
  이름이 표시될 수 있습니다.

## 정확한 변환 표

| NME | 생성되는 Python |
| --- | --- |
| `말해 값` / `say value` | `print(value)` |
| `물어봐 이름` / `ask name` | `name = input()` |
| `물어봐 이름, 질문` / `ask name, prompt` | `name = input(prompt)` |
| `횟수번:` / `count times:` | `for _ in range(count):` |
| `만약 조건:` / `when condition:` | `if (condition):` |
| `랜덤 사용` | `random` import와 한국어 별칭 |
| `use random` | `random` import와 영어 별칭 |

설치, CLI 명령, 첫 실행 전체 과정은
[NME 시작하기](getting-started.ko.md)를 참고하세요.
