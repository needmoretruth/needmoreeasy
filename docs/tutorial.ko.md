# 프로그램 다섯 개로 NME 배우기

[English](tutorial.md) | 한국어

한 문장으로 시작해서 컴파일러로 끝나는 과정입니다. 다음 단계로 가기 전에 모든
예제를 실행하세요. 단어 하나를 바꾸고 결과를 보는 것도 학습입니다.

## 프로젝트 1: Hello World

`hello.nme`를 만듭니다.

```text
안녕하세요! 말해줘
```

```sh
nme 실행 hello
```

`말해줘`가 동작이고 그 앞의 내용이 보여 줄 문장입니다. 영어도 같습니다.

```text
show Hello world!
```

문장을 바꿔 보세요. 반복까지 보고 싶으면 `examples/hello-sentence.nme`를
실행합니다.

## 프로젝트 2: 인사 프로그램

```text
이름을 물어봐 이름이 뭐예요?
이름 만나서 반가워요! 말해줘
```

첫 줄이 `이름`이라는 이름을 만듭니다. 두 번째 줄은 이미 아는 그 이름을 찾아
실제 값으로 넣습니다. 영어도 서식 문법이 필요 없습니다.

```text
ask name What is your name?
show Nice to meet you name!
```

도전: 좋아하는 색을 물어보고 다음 문장에서 보여 주세요.

## 프로젝트 3: 숫자 맞히기

랜덤 정답과 숫자 입력부터 만듭니다.

```text
정답은 1부터 10까지 랜덤정수
추측을 숫자로 물어봐 1부터 10까지 숫자를 맞혀 보세요
```

값을 비교합니다.

```text
만약에 추측이 정답과 같으면
    정답입니다! 말해줘

만약에 추측이 정답보다 작으면
    더 큰 수예요 말해줘

만약에 추측이 정답보다 크면
    더 작은 수예요 말해줘
```

완성된 프로그램은
[`examples/guessing-game.ko.nme`](../examples/guessing-game.ko.nme)에 있습니다.

```sh
nme 실행 examples/guessing-game.ko
```

컴파일되는 내용입니다.

- `1부터 10까지 랜덤정수` → `random.randint(1, 10)`
- `숫자로 물어봐` → `int(input(...))`
- `같으면`, `작으면`, `크면` → `==`, `<`, `>`
- 들여쓰기 → 각 조건이 제어하는 문장 묶음

도전: 범위를 1부터 100으로 바꾸고, 입력과 조건을 반복해서 두 번째 기회를
추가하세요.

들여쓰기가 어렵다면 같은 제어 흐름을 평평하게 쓰고 `끝`으로 닫을 수 있습니다.

```text
동안 추측 < 정답
다시 시도해 말해줘
숫자로 추측을 물어봐 다른 숫자를 입력하세요
끝
```

`멈춰`, `그리고`/`또는`, `아니면 만약`, `아니면`으로 각각 `break`, `and`/`or`,
`elif`, `else`도 연습해 보세요.

## 프로젝트 4: 세 단계 모두 쓰기

문장형에 갇힐 필요가 없습니다. 초급 문법이나 Python이 더 분명할 때 바로
사용하세요.

```text
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
언어입니다. `examples/three-levels.nme`를 실행하세요.

Python 반복 옆에 들여쓰기 없는 NME 블록도 추가해 보세요.

```text
동안 사람 != "Grace"
사람 안녕 말해줘
멈춰
끝
```

도전: Python 함수를 만들고 그 안에서 문장형 `show`를 사용하세요.

## 프로젝트 5: NME로 컴파일러 만들기

컴파일러는 한 언어를 읽고 다른 언어를 만듭니다. 소스를 직접 실행할 필요가
없습니다. [`examples/tiny-compiler.nme`](../examples/tiny-compiler.nme)는 다음
두 문장을 가진 아주 작은 언어를 컴파일합니다.

```text
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

1. `tiny_source`를
   `Path(입력).read_text(encoding="utf-8").splitlines()`로 바꿉니다.
2. `Path(출력).write_text(generated, encoding="utf-8")`로 결과를 씁니다.
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
