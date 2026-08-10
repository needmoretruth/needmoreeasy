# NME 언어 레퍼런스(v0.1)

[English](language.md) | 한국어

NME는 Python에 두 가지 쉬운 문장 형식을 더한 언어입니다. 이 문서는 그 문법과
동작을 정의합니다. 첫 NME 프로그램을 만드는 중이라면 먼저
[시작하기 튜토리얼](getting-started.ko.md)을 읽어 보세요.

## 핵심 호환 규칙: Python 우선

올바른 모든 Python 프로그램은 올바른 NME 프로그램입니다. 소스 한 줄이 올바른
Python이면 `say`나 `times`라는 이름이 포함되어 있어도 항상 Python으로
유지됩니다. Python이 그 줄을 거부하고 NME 형식과 일치할 때만 NME가 해석합니다.

| 소스 | 해석 |
| --- | --- |
| `say("안녕")` | Python 함수 호출, 변경 없음 |
| `say = print` | Python 대입문, 변경 없음 |
| `times = 5` | Python 대입문, 변경 없음 |
| `if times:` | Python `if` 헤더, 변경 없음 |
| `say "안녕"` | NME `say` 문장 |
| `5 times:` | NME `times` 블록 |

따라서 `say`와 `times`는 전역 예약어가 아닙니다. 문자열, f-string, 삼중 따옴표
문자열, 주석은 Python 방식으로 토큰화되며 NME 코드로 잘못 인식되지 않습니다.

## 문법 요약

```text
say <Python 표현식>

<Python 표현식> times:
    <문장들>

<Python 표현식> times: <한 문장>
```

꺾쇠괄호 안에는 평범한 Python 소스를 씁니다. NME는 Python 파서에 표현식이
올바른지 물어보고 그 표기를 다시 작성하지 않은 채 그대로 복사합니다.

## `say` — 값 하나 보여 주기

### 문법

```text
say <Python 표현식>
```

### 의미

```text
say expression
```

위 코드는 다음과 같이 변환됩니다.

```python
print(expression)
```

예제:

```text
say "안녕하세요!"
say 1 + 1
say f"2 + 2는 {2 + 2}"
say [name.upper() for name in names]
```

`say` 다음 부분은 하나의 올바른 Python 표현식이어야 합니다. NME는 괄호와 문자열
서식을 포함한 표현식을 그대로 보존합니다.

`say`는 표현식 하나의 값을 출력합니다. 여러 인수나 `sep=`, `end=`, `file=`,
`flush=` 같은 옵션이 필요하면 Python의 `print(...)`를 직접 사용하세요.

### Python이 우선하는 경우

```python
say("안녕")  # say라는 Python 함수 호출
say.attr      # Python 속성 접근
say[0]        # Python 인덱싱
say           # Python 이름 표현식
```

NME는 런타임 함수 `say`를 정의하지 않습니다. 따라서 위 예제는 해당 Python
이름이 알맞게 정의되어 있어야 실행됩니다. `say`만 쓰면 런타임에 Python
`NameError`가 발생할 수 있습니다.

## `times:` — 반복하기

`times:`는 횟수 표현식을 Python의 `range`에 전달하여 반복문을 실행합니다.

### 블록 형식

```text
<Python 표현식> times:
    <문장들>
```

예제:

```text
5 times:
    say "안녕"
    say "다시 한 번"
```

생성되는 Python:

```python
for _ in range(5):
    print("안녕")
    print("다시 한 번")
```

본문에는 뒤따르는 더 깊게 들여쓴 코드 줄이 있어야 합니다. 들여쓰기는 Python과
같은 규칙을 따르며 Python과 같은 방식으로 중첩할 수 있습니다.

### 한 줄 형식

```text
<Python 표현식> times: <한 문장>
```

예제:

```text
5 times: say "안녕"
3 times: print("Python도 동작합니다")
2 times: 3 times: say "중첩"
```

콜론 다음에는 정확히 한 문장만 쓸 수 있습니다. 최상위 세미콜론은 허용하지
않습니다.

```text
5 times: say "A"; say "B"  # 오류
```

여러 문장은 들여쓴 블록으로 작성하세요.

```text
5 times:
    say "A"
    say "B"
```

한 줄 `times:` 안에서 본문 없는 새 블록을 열 수 없습니다.

```text
2 times: 3 times:  # 오류: 안쪽 반복문에 한 줄 본문이 없음
```

중첩 블록은 들여쓰거나, 한 줄 문법의 마지막에 실제 문장을 작성하세요.

### 횟수 표현식과 런타임 동작

횟수에는 올바른 모든 Python 표현식을 사용할 수 있습니다.

```text
(2 + 3) times:
    say "다섯 번"

len(items) times: say "항목마다 한 번"
```

생성되는 코드는 `range(표현식)`이므로 그 값은 런타임에 Python `range`가
받아들일 수 있어야 합니다. 0과 음의 정수는 본문을 한 번도 실행하지 않습니다.
횟수 표현식은 반복문에 진입할 때 한 번 평가됩니다.

현재 NME는 생성된 Python 반복 변수로 `_`를 사용합니다. `times:` 반복문 안이나
뒤에서 `_`의 값에 의존하지 마세요.

## Python과 NME 섞기

모든 Python 문장과 표현식을 그대로 사용할 수 있습니다.

```text
import random

def greet(name):
    say f"안녕하세요, {name}!"    # Python 함수 안의 NME

for name in ["Ada", "Grace"]:   # Python 반복문
    greet(name)

2 times:                         # NME 반복문
    print(random.random())       # NME 안의 Python
```

NME에는 별도의 타입 시스템, 모듈 시스템, 런타임, 패키지 관리자, 표준 라이브러리가
없습니다. 이 동작들은 모두 Python의 동작을 따릅니다.

## 공백, 주석, 소스 보존

- 블록 들여쓰기는 Python과 같은 의미를 가집니다.
- 빈 줄과 주석을 보존합니다.
- NME 문장 뒤의 주석을 보존합니다.
- CRLF와 LF 줄바꿈을 보존합니다.
- Python 문자열이나 주석 속 NME처럼 보이는 텍스트는 건드리지 않습니다.
- 변환 전후 실제 줄 수가 같아서 트레이스백 줄 번호가 `.nme` 소스와 계속
  일치합니다.

예제:

```text
text = "5 times: say something"  # 문자열 내용, 변경 없음
# 5 times: say "안녕"             # 주석, 변경 없음
say text                           # NME, 뒤쪽 주석도 보존
```

## 진단 메시지

잘못된 NME 형식과 어휘 분석 문제에는 다음 내용을 담은 초보자 친화적인 진단이
표시됩니다.

1. 쉬운 말로 쓴 오류 메시지
2. 관련 소스 위치 아래의 캐럿 표시
3. 고쳐 볼 방법을 알려 주는 힌트

가능하면 컴파일러는 여러 NME 문제를 한 번에 수집합니다. Python 런타임 오류는
선택한 CPython 인터프리터가 표시하며 원본 줄 번호가 유지됩니다.

## 현재 제약

- v0.1의 NME 문법은 `say`와 `times:` 두 개뿐입니다.
- 한 줄 `times:`는 정확히 한 문장만 받으며 최상위 세미콜론을 거부합니다.
- 한 줄 문장은 블록을 여는 `times:` 형식으로 끝날 수 없습니다.
- 실행하려면 CPython 인터프리터가 설치되어 있어야 합니다. 변환과 검사는
  Python을 실행하지 않습니다.
- 런타임 트레이스백의 줄 번호는 유지되지만 v0.1에서는 원본 `.nme` 경로 대신
  임시 `.py` 파일 이름이 표시될 수 있습니다.

이 제약들은 의도적입니다. NME는 많은 축약 문법보다 작고 예측 가능한 언어를
우선합니다.

## 정확한 변환 표

| NME | 생성되는 Python |
| --- | --- |
| `say value` | `print(value)` |
| `count times:` | `for _ in range(count):` |
| `count times: say value` | `for _ in range(count): print(value)` |
| `count times: python_statement` | `for _ in range(count): python_statement` |

CLI 명령과 첫 실행 전체 과정은 [NME 시작하기](getting-started.ko.md)를
참고하세요.
