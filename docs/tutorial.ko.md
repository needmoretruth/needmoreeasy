# 프로그램 일곱 개로 NME 배우기

[English](tutorial.md) | 한국어

[README](../README.ko.md) | [설치](install.ko.md) | [5분 시작](getting-started.ko.md) | [문법 안내](language.ko.md)

한 문장으로 시작해서 컴파일러로 끝나는 과정입니다. 다음 단계로 가기 전에 모든
예제를 실행하세요. 단어 하나를 바꾸고 결과를 보는 것도 학습입니다.

## 프로젝트 1: Hello World

`hello.nme`를 만듭니다(`nme --version`을 실행한 같은 폴더에서; 아무 텍스트
편집기나 괜찮습니다 — Windows에서는 메모장이 기본으로 UTF-8로 저장합니다).

```nme
안녕하세요! 말해줘
```

```sh
nme 실행 hello
```

`말해줘`가 동작이고 그 앞의 내용이 보여 줄 문장입니다. 영어도 같습니다.

```nme
show Hello world!
```

문장을 바꿔 보세요. 반복까지 보고 싶으면
[`examples/hello-sentence.ko.nme`](../examples/hello-sentence.ko.nme)를
실행합니다. 영어판 쌍둥이는
[`examples/hello-sentence.nme`](../examples/hello-sentence.nme)예요.

## 프로젝트 2: 인사 프로그램

```nme
이름이 뭐예요?
이름 만나서 반가워요!
```

첫 줄이 평범한 질문에서 `이름`이라는 이름을 자동으로 만듭니다. 두 번째 줄은
이미 아는 그 이름을 찾아 실제 값으로 넣습니다. 영어도 서식 문법이 필요
없습니다.

```nme
What is your name?
Nice to meet you name!
```

도전: 좋아하는 색을 물어보고 다음 문장에서 보여 주세요.

## 프로젝트 3: 숫자 맞히기

랜덤 정답과 숫자 입력부터 만듭니다.

```nme
정답은 1부터 10까지 랜덤정수
추측을 숫자로 물어봐 1부터 10까지 숫자를 맞혀 보세요
```

값을 비교합니다.

```nme
만약에 추측이 정답과 같으면
    정답입니다! 말해줘

만약에 추측이 정답보다 작으면
    더 큰 수예요 말해줘

만약에 추측이 정답보다 크면
    더 작은 수예요 말해줘
```

완성된 프로그램은
[`examples/guessing-game.ko.nme`](../examples/guessing-game.ko.nme)에 있고,
영어판 쌍둥이는
[`examples/guessing-game.nme`](../examples/guessing-game.nme)입니다.

```sh
nme 실행 examples/guessing-game.ko
```

컴파일되는 내용입니다.

- `1부터 10까지 랜덤정수` → `__import__("random").randint(1, 10)`
- `숫자로 물어봐` → `int(input(...))`
- `같으면`, `작으면`, `크면` → `==`, `<`, `>`
- 들여쓰기 → 각 조건이 제어하는 문장 묶음

도전: 범위를 1부터 100으로 바꾸고, 입력과 조건을 반복해서 두 번째 기회를
추가하세요.

들여쓰기가 어렵다면 같은 제어 흐름을 평평하게 쓰고 `끝`으로 닫을 수 있습니다.

```nme
추측이 정답과 같지 않을 동안
다시 시도해 말해줘
추측을 숫자로 물어봐 다른 숫자를 입력하세요
끝
```

`!=`는 '같지 않다'를 뜻하며, 문장형 표기는 `is not equal to`입니다.

`멈춰`, `그리고`/`또는`, `아니면 만약에`, `아니면`으로 각각 `break`, `and`/`or`,
`elif`, `else`도 연습해 보세요.

## 프로젝트 4: 이름 목록으로 인사하기

지금까지는 값 하나씩만 다뤘습니다. 여러 개를 한 번에 담는 것이 **목록**이고,
목록도 문장으로 씁니다. `friends.nme`를 만듭니다.

```nme
친구들은 목록 민수, 지안, 하늘
친구들의 친구마다 반복해
    친구 안녕하세요! 말해줘
    1초 기다려
끝
```

```sh
nme 실행 friends
```

세 사람에게 한 명씩, 1초 간격으로 인사합니다.

- `친구들은 목록 민수, 지안, 하늘` — 이름 셋을 담은 목록을 만듭니다.
- `친구들의 친구마다 반복해` — 목록을 처음부터 끝까지 하나씩 지나갑니다.
  지나가는 동안 그 하나를 `친구`라고 부릅니다.
- `1초 기다려` — 다음 줄로 넘어가기 전에 잠깐 멈춥니다.

비어 있는 목록으로 시작해 하나씩 넣을 수도 있습니다.

```nme
친구들은 빈 목록
친구들에 민수 넣어
친구들에 지안 넣어
친구들 말해줘
```

도전: 이름을 물어봐서 목록에 넣는 것을 세 번 반복한 뒤, 전부 인사하세요.

여기까지 프로그램 넷을 만드는 동안 따옴표도, 대괄호도, 콜론도, 등호도
필요하지 않았습니다. **문장형만으로 프로그램을 끝까지 만들 수 있습니다.**
다음 두 프로젝트는 Python을 이미 아는 사람, 또는 언젠가 알고 싶어진 사람을
위한 것입니다. 지금 넘어가도 되고, 한참 뒤에 돌아와도 됩니다.

## 프로젝트 5: 세 단계 모두 쓰기

문장형에 갇힐 필요가 없습니다. 초급 문법이나 Python이 더 분명할 때 바로
사용하세요.

```nme
사람들 = ["Ada", "Grace"]

2번:
    말해 "초급 문법"

repeat 2 times
    한국어 문장형 말해줘

for 사람 in 사람들:
    show Hello 사람!
```

목록과 `for 사람` 반복은 고급 Python, `2번:`과 `말해`는 초급 NME,
`repeat`와 `show`는 문장형 NME입니다. 서로 다른 모드나 파일이 아니라 한
언어입니다. [`examples/three-levels.nme`](../examples/three-levels.nme)를
실행하세요. 완성된 들여쓰기 없는 제어 흐름 쌍은
[`examples/control-flow-sentence.nme`](../examples/control-flow-sentence.nme)와
[`examples/control-flow-korean.nme`](../examples/control-flow-korean.nme)입니다
— 둘 다 실행하고 같은 출력을 비교해 보세요.

Python 반복 옆에 들여쓰기 없는 NME 블록도 추가해 보세요.

```nme
동안 사람 != "Grace"
사람 안녕 말해줘
멈춰
끝
```

도전: Python 함수를 만들고 그 안에서 문장형 `show`를 사용하세요.

## 프로젝트 6: 한 게임을 Python으로 옮기기

타임루프 추리 게임을 같은 내용으로 세 단계로 작성했습니다. 가장 쉬운 한국어
문장형부터 시작해 초급형을 비교하고, 마지막에 일반 Python 버전을 읽으세요.

- [`time-loop-sentence.ko.nme`](../examples/time-loop-sentence.ko.nme) — 쉬운
  문장, `끝`, 자연스러운 한국어 조건;
- [`time-loop-beginner.ko.nme`](../examples/time-loop-beginner.ko.nme) —
  `저장`, `물어봐`, `만약`, `N번:` 초급형. `저장`은 변수에 값을 넣는 초급
  문법이에요 (예: `저장 점수를 3`);
- [`time-loop-python.nme`](../examples/time-loop-python.nme) — 목록, 딕셔너리,
  f-string, `while`, `break`, 일반 Python.

세 파일 모두 같은 방식으로 검사합니다.

```sh
nme 검사 examples/time-loop-sentence.ko
nme 검사 examples/time-loop-beginner.ko
nme 검사 examples/time-loop-python
```

Python으로 넘어가기 전에 더 큰 예제를 보고 싶다면
[`roulette.nme`](../examples/roulette.nme)를 실행해 보세요. 영어판은
[`roulette.en.nme`](../examples/roulette.en.nme)입니다.

준비가 되면 하나를 실행해 질문에 답해 보세요. 한 번에 전체를 다시 쓰지 말고,
한 블록이나 한 줄만 다음 단계 문법으로 바꾸면서 나머지 프로그램은 그대로
두는 것이 목표입니다.

## 프로젝트 7: NME로 컴파일러 만들기

이 단계는 선택적인 고급 도전 과제입니다. 새로운 문법 단계가 아니므로 앞의
프로젝트가 편해질 때까지 미뤄도 됩니다. 인덱싱, 슬라이스, Python 메서드 호출이
아직 낯설다면 먼저 문장형과 초급 문법으로 작은 규칙을 더 연습하세요.

```nme
횟수는 0
동안 횟수 < 2
횟수에 1 더해
끝

2번: 말해 "작은 규칙 하나"
```

컴파일러는 한 언어를 읽고 다른 언어를 만듭니다. 소스를 직접 실행할 필요가
없습니다. [`examples/tiny-compiler.nme`](../examples/tiny-compiler.nme)는 다음
두 문장을 가진 아주 작은 언어를 컴파일합니다.

```nme
말하기 안녕하세요
3번 말하기 NME로 컴파일러를 만들었어요
```

컴파일러는 다음과 같은 Python을 만듭니다.

```python
print('안녕하세요')
for _ in range(3): print('NME로 컴파일러를 만들었어요')
```

컴파일러를 실행하고 컴파일러 자체가 생성한 Python도 확인합니다.

```sh
nme 실행 examples/tiny-compiler
nme 빌드 examples/tiny-compiler -o tiny-compiler.py
```

예제는 목록 처리에 고급 Python, 결과 출력에 문장형 NME를 일부러 섞었습니다.
초보자도 Python 생태계를 전부 유지하면서 작은 규칙 하나씩 컴파일러를 키울 수
있습니다.

파일 컴파일러로 키우는 순서입니다.

1. 위쪽에 `from pathlib import Path`를 추가하고, `tiny_source`를
   `input_name = "tiny.txt"`와
   `Path(input_name).read_text(encoding="utf-8").splitlines()`로 바꿉니다.
2. `output_name = "tiny.py"`를 정의하고
   `Path(output_name).write_text(generated, encoding="utf-8")`로 결과를 씁니다.
3. 두 문장 어디에도 맞지 않는 줄에 쉬운 오류를 추가합니다.
4. 소스 파일을 컴파일하고 생성된 Python을 실행하는 테스트를 만듭니다.

이 흐름은 작은 NME 실제 구조와 같습니다. 입력을 토큰화하거나 분류하고, 정확한
중간 뜻을 만들고, 대상 코드로 낮추고, 결과를 테스트합니다. 배포와 신뢰성을 위해
NME의 실제 컴파일러는 Rust로 만들었지만, NME로 만드는 컴파일러는 NME와
Python의 모든 기능을 사용할 수 있습니다.

## 다음 단계

- 정확한 규칙이 필요할 때만 [문법 레퍼런스](language.ko.md)를 봅니다.
- 작은 변경을 할 때마다 `nme 검사`를 실행합니다.
- [nme 변환](converting-python.ko.md)으로 기존 Python 연습 문제를 바꿉니다.
- `nme 실행`으로 먼저 확인한 뒤 네이티브 결과물을 만듭니다.
