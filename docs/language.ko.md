# NME 문법 레퍼런스

[English](language.md) | 한국어

NME는 하나의 컴파일러 안에 세 문법 단계가 있습니다. 모드를 바꾸는 선언은
없습니다. 한 파일과 한 블록 안에서 고급 Python, 초급 NME, 문장형 NME,
한국어, 영어를 모두 섞을 수 있습니다.

## 호환성 규칙

올바른 Python이 항상 우선합니다. NME는 쉬운 문법을 찾기 전에 실제 Python
파서로 그 줄이 올바른지 확인합니다. 따라서 올바른 Python 프로그램은 바이트
하나도 바뀌지 않습니다.

```python
say = print
say("Python 호출")
if 준비됨:
    print("Python 조건")
```

세 단계는 다음처럼 같은 Python으로 컴파일될 수 있습니다.

| 단계 | NME | 생성되는 Python |
| --- | --- | --- |
| 문장형 | `3번 반복해서 안녕 말해줘` | `for _ in range(3): print("안녕")` |
| 초급 | `3번: 말해 "안녕"` | `for _ in range(3): print("안녕")` |
| 고급 | `for _ in range(3): print("안녕")` | 그대로 유지 |

## 문장형 단계

문장형은 코딩 첫날을 위한 문법입니다. 아래의 자주 쓰는 작업에서는 따옴표,
쉼표, 괄호, 중괄호, 등호, 콜론이 필요하지 않습니다. 평범한 문장부호 `?`와
`!`는 따옴표 없이 쓸 수 있습니다.

### 글과 값 보여 주기

```text
안녕하세요! 말해줘
보여줘 반갑습니다
show Hello world!
Hello world show
```

입력이나 문장형 저장으로 앞에서 만든 이름은 자동으로 값이 들어갑니다.

```text
이름을 물어봐 이름이 뭐예요?
안녕하세요 이름! 말해줘

ask name What is your name?
show Hello name!
```

`이름`과 `name` 자리에는 실제 대답이 들어가고 나머지는 글로 남습니다. 알고
있는 한국어 이름 뒤의 조사는 출력에 그대로 붙습니다.

출력 동작으로 `보여줘`, `말해줘`, `말해주세요`, `출력해`, `출력해줘`,
`show`, `display`, `tell`, `say`를 쓸 수 있습니다. 정확한 초급 표기인
`말해 표현식`과 `say expression`은 올바른 Python 표현식을 코드로 처리합니다.

### 글이나 숫자 물어보기

```text
이름을 물어봐 이름이 뭐예요?
ask name What is your name?

나이를 숫자로 물어봐 몇 살인가요?
ask number age How old are you?
```

따옴표 없는 질문 끝에는 입력하기 편하도록 공백이 자동으로 하나 붙습니다. 글
입력은 `input(...)`, 숫자 입력은 `int(input(...))`로 컴파일됩니다.

`물어봐`, `물어봐줘`, `질문해`, `입력받아`, `ask`, `prompt`를 쓸 수
있습니다. 입력받을 한국어 이름의 `을`과 `를`은 변수 이름에서 빠집니다.

### 값 저장하기

```text
인사는 안녕하세요
정답은 7
set greeting to Hello
set answer to 7
```

평범한 대입문으로 바뀝니다. 숫자와 분명한 표현식은 코드, 평범한 단어는 글로
저장합니다. 저장한 이름은 뒤의 문장 출력과 조건에서 사용할 수 있습니다.

### 반복하기

한 문장을 한 줄에서 반복합니다.

```text
3번 반복해서 다시 말해줘
repeat 3 times and show Again
3 times 반복해서 mixed 말해줘
```

여러 문장은 콜론 없이 들여씁니다.

```text
3번 반복해
    첫째 말해줘
    show second

repeat 3 times
    show First
    둘째 말해줘
```

`반복`, `반복해`, `반복해서`, `repeat`를 `번` 또는 `times`와 섞어도
됩니다. 횟수에는 올바른 Python 표현식을 사용할 수 있습니다.

### 들여쓰기 없이 블록 닫기

Python으로 갈 준비가 되기 전에는 들여쓰기가 가장 어렵게 느껴질 수 있습니다.
이때는 블록 안 문장을 다음 줄에 그대로 쓰고 마지막에 `끝` 또는 `end`를 한
줄로 적으면 됩니다. 이 방식으로 Python으로 넘어갈 때 필요한 제어 흐름도
배웁니다.

```text
점수는 0
점수가 3보다 작을 동안
점수 말해줘
점수는 점수 + 1
끝

만약 준비 그리고 점수가 2보다 크면
성공 말해줘
아니면 만약 점수가 0과 같으면
다시 말해줘
아니면
아직 말해줘
끝

동안 준비 또는 기다리는중
계속 말해줘
멈춰
끝
```

`동안`, `만약`, `아니면`, `멈춰`, `끝`은 영어 `while`, `if`, `else`,
`break`, `end`와 같은 뜻입니다. `그리고`와 `또는`은 각각 `and`와 `or`로
섞어 쓸 수 있습니다. 익숙해지면 같은 블록을 네 칸 들여쓰기로 쓰거나 Python
문법으로 한 줄씩 바꿔도 됩니다.

### 조건 사용하기

콜론이 없는 블록입니다.

```text
만약에 이름이 있으면
    안녕하세요 이름 말해줘

if 준비됨
    show 시작
```

한 문장은 `then`이나 한국어 연결 어미 뒤에 바로 적습니다.

```text
만약에 점수가 10보다 크면 성공 말해줘
if score is greater than 10 then show You won
```

지원하는 문장형 비교입니다.

| 한국어 | 영어 | 뜻 |
| --- | --- | --- |
| `만약에 이름이 있으면` | `if name exists` | 값이 있음/참 |
| `만약에 이름이 없으면` | `if name missing` | 값이 없음/거짓 |
| `만약에 점수가 10과 같으면` | `if score equals 10` | `==` |
| `만약에 점수가 10보다 크면` | `if score is greater than 10` | `>` |
| `만약에 점수가 10보다 작으면` | `if score is less than 10` | `<` |

`만약 조건`, `만약에 조건`, `when condition`, 혼합형 `if 조건`도 전부
동작합니다. 모든 Python 표현식이 필요한 조건은 정확한 초급형을 쓰세요.

`and`는 `or`보다 먼저 계산됩니다. 영어와 한국어를 섞어도 됩니다.

```text
if 준비 그리고 점수 > 2 then 성공 말해줘
만약 준비 또는 기다리는중이면 기다려 말해줘
```

### 코드용 특수문자 없는 랜덤

```text
주사위는 1부터 6까지 랜덤정수
주사위 말해줘

색은 빨강 또는 초록 또는 파랑 중에서 랜덤선택
show 색
```

Python에 포함된 `random`을 바로 사용하므로 별도 모듈 줄도 필요 없습니다.

### 오타와 연결어 복구

문서에 있는 동작 단어의 여러 표현을 받아들이며, Python이 그 줄을 거부한 뒤에
한 글자 삽입·삭제·교체 또는 붙어 있는 두 글자 순서 바뀜을 복구합니다.
`물어바` → `물어봐`, `말헤` → `말해`, `repaet` → `repeat`가 예입니다.

복구는 동작 토큰에만 적용하며 Python 표현식, 문자열, 주석은 고치지 않습니다.
고칠 방법이 하나로 정해지지 않거나 동작이 분명하지 않으면 억지로 추측하지 않고
정확한 위치와 고치는 예시를 보여 줍니다. 모든 가능한 오타와 모든 사람의 문장을
안전하게 추측할 수 있는 컴파일러는 없으므로 이 경계는 의도적입니다.

## 초급 단계

초급 문법은 짧고 정확합니다. 모든 Python 표현식을 쓸 수 있으며 문장형 해석이
애매할 때 유용합니다. 문서에 있는 초급 동작은 모두 한국어 이름이 있으며 한
줄에서 영어와 한국어를 섞어도 됩니다.

```text
말해 <Python 표현식>
say <Python expression>

물어봐 <이름>
물어봐 <이름>, <Python 질문 표현식>
ask <name>
ask <name>, <Python prompt expression>

<횟수>번:
<count> times:

만약 <조건>:
when <condition>:

동안 <조건>
while <condition>
멈춰
break
아니면 만약 <조건>
else if <condition>
아니면
else
끝
end

랜덤 사용
use random
```

콜론 뒤에 한 문장을 바로 쓰거나 여러 줄을 들여쓸 수 있습니다.

```text
3번: 말해 "안녕"
3 times:
    say "Hi"
    print("고급 Python도 가능")
```

정확한 변환입니다.

| NME | Python |
| --- | --- |
| `말해 값` / `say value` | `print(value)` |
| `물어봐 이름` / `ask name` | `name = input()` |
| `물어봐 이름, 질문` | `name = input(prompt)` |
| `횟수번:` / `count times:` | `for _ in range(count):` |
| `만약 조건:` / `when condition:` | `if (condition):` |
| `동안 조건` ... `끝` / `while condition` ... `end` | `while (condition):` |
| `멈춰` / `break` | `break` |
| `아니면 만약 조건` / `else if condition` | `elif (condition):` |
| `아니면` / `else` | `else:` |

표현식은 NME가 다시 만들지 않는 Python 원본 범위입니다. 올바른지만 확인하고
적힌 그대로 복사합니다.

## 고급 단계

고급 NME는 Python 문법과 같습니다. 대입, 함수, 클래스, import, 예외,
비동기 코드, 패턴 매칭, 설치한 Python 패키지와 모든 올바른 Python 기능이
변경 없이 동작합니다.

```python
from pathlib import Path

def 단어들(경로):
    return Path(경로).read_text(encoding="utf-8").split()

for 단어 in 단어들("notes.txt"):
    show 단어
```

마지막 줄처럼 고급 Python 블록 안에 영어 문장형 NME를 쓸 수 있습니다.

## 버전이 있는 내장 모듈

쉬운 랜덤 어댑터 버전은 `0.0.1`입니다.

```text
랜덤 사용
랜덤 사용 최신
최신 랜덤 사용
랜덤 사용 버전 "0.0.1"

use random
use random latest
use latest random
use random version "0.0.1"
```

`최신` / `latest`는 설치한 NME 컴파일러에 들어 있는 가장 새 어댑터를
고릅니다. 제어되지 않은 네트워크 업데이트가 아니라 로컬에서 항상 같은 결과를
내는 선택입니다. 없는 정확한 버전을 요청하면 설치된 버전을 알려 주는 오류가
나옵니다.

어느 언어로 불러와도 두 언어의 이름이 모두 생깁니다.

| 한국어 | 영어 | Python 뜻 |
| --- | --- | --- |
| `랜덤정수(가, 나)` | `random_number(a, b)` | `random.randint(a, b)` |
| `랜덤선택(값들)` | `random_pick(values)` | `random.choice(values)` |
| `섞기(값들)` | `shuffle(values)` | `random.shuffle(values)` |
| `랜덤버전` | `random_version` | 어댑터 버전 문자열 |

`nme 모듈` 또는 `nme modules`로 버전을 확인합니다. 보안 번호나 비밀번호에는
랜덤 도구를 사용하지 마세요.

## Python 변환

`nme 변환`은 고른 단계와 언어로 Python을 안전하게 바꿉니다.

```sh
nme 변환 app.py --level 문장형 --language 한국어 -o app.nme
```

뜻을 보존할 수 있는 단일 값 `print`, `input` 대입, `int(input(...))`,
`for _ in range(...)`, `if`, 간단한 대입을 바꿉니다. 일반 `import random`은
이름이 덮어써질 위험이 있어 그대로 고급 Python으로 둡니다.
나머지는 올바른 NME인 고급 Python으로 남깁니다. [Python 변환 안내](converting-python.ko.md)를
참고하세요.

## 소스 보존과 오류

- Python 토큰화가 문자열과 주석을 보호합니다.
- 올바른 Python은 바이트 단위로 같습니다.
- 들여쓰기, 빈 줄, 주석, 줄바꿈 방식, 실제 줄 수를 보존합니다.
- NME 오류에는 쉬운 설명, 정확한 캐럿 위치, 고치는 힌트가 있습니다.
- 가능하면 서로 독립적인 문제를 한 번에 모읍니다.
- 한국어로 시작한 문법에는 한국어 안내가 나옵니다.

## 현재 제한

- 문장 출력은 간단한 대입, 함수 매개변수, 단순한 Python 반복 변수, NME 입력,
  문장형 저장으로 만든 이름을 찾습니다. 동적으로 만든 특이한 이름이나 애매한
  글은 초급 표현식을 쓰세요.
- 문장형 비교 단어는 의도적으로 작습니다. `그리고`/`또는` 논리와 복잡한
  표현식은 명시적 `끝` 블록이나 고급 Python을 쓰세요.
- 쉬운 모듈 문법은 이번 베타에서 내장 random 어댑터만 제공합니다.
- `검사`와 `빌드`는 생성된 Python을 선택한 CPython으로 컴파일까지 확인하지만
  실행하지는 않습니다. 실행 오류는 CPython이 담당합니다.
- `실행`, `빌드`, `검사`에는 CPython이 필요합니다. 선택적인 `컴파일`에는
  Python, Nuitka, 운영체제용 C 컴파일러가 필요합니다.
- 네이티브 컴파일이 모든 프로그램을 무조건 빠르고 작게 만들지는 않습니다.
  필요한 결과물을 직접 측정하세요.
