# NME 정리하기, Python을 NME로 바꾸기

[English](converting-python.md) | 한국어

[README](../README.ko.md) | [5분 시작](getting-started.ko.md) | [학습 과정](tutorial.ko.md) | [문법 안내](language.ko.md)

## 설치 없이, 사이트에서

[needmoreeasy.com](https://needmoreeasy.com/ko/)의 쓰는 칸 아래에 **정리하기**
단추가 있습니다. 옆에서 **어느 말로**(한국어·English) 쓸지와 **어느 표기로**
(문장·초급·파이썬) 쓸지를 고른 뒤 누르면, 쓰신 프로그램이 그 한 가지 표기로
다시 쓰입니다. 마음에 들지 않으면 **되돌리기**를 누르면 됩니다.

빠르게 쓸 때는 줄임말을 쓰든, 순서를 바꾸든, 두 나라 말을 섞든 그대로 두세요.
정리는 나중에 한 번 누르면 됩니다.

**프로그램이 실행되는 상태여야 합니다.** 정리는 각 줄이 무슨 뜻인지 읽어서 다시
쓰는 것이라, 컴파일러가 읽지 못하는 줄은 다시 쓸 수도 없습니다. 어디가
막혔는지는 쓰는 칸 아래에 줄 번호와 함께 나옵니다.

**프로그램이 하는 일은 바뀌지 않습니다.** 다시 쓴 줄은 전부 컴파일러에 한 번 더
걸어서, 나오는 Python이 글자 하나까지 같은지 확인합니다. 달라지는 줄은 버리고
쓰신 그대로 둡니다.

## 내 컴퓨터에서

변환기는 입력 전체가 올바른 Python인지 먼저 확인한 뒤, 요청한 NME 단계에서
뜻을 안전하게 보존할 수 있는 줄만 바꿉니다.

```sh
nme 변환 app.py --level 문장형 --language 한국어 -o app.nme
```

같은 명령으로 **NME 파일을 정리**할 수도 있습니다. `.nme` 프로그램을 주면 이미
쓴 NME를 한 단계·한 언어 표기로 다시 씁니다. 세 단계와 두 언어가 섞인 파일이
한 가지 표기로 나옵니다.

```sh
nme 변환 app.nme --level 문장형 --language 한국어 -o app.tidy.nme
```

## 옵션

```text
--level advanced|beginner|sentence
--language en|ko
-o, --output <file.nme>
```

`고급`, `초급`, `문장형`, `영어`, `한국어`도 옵션 값으로 쓸 수 있습니다.
옵션이 없으면 영어 문장형 결과를 화면에 표시합니다. `-o`가 없으면 입력 파일을
절대 수정하지 않습니다.

## 바뀌는 문법

| Python | 초급 | 문장형 |
| --- | --- | --- |
| `print(value)` | `말해 value` | `보여줘 value` |
| `name = input(prompt)` | `물어봐 name, prompt` | `name을 물어봐 prompt` |
| `n = int(input(prompt))` | Python 유지 | `n을 숫자로 물어봐 prompt` |
| `for _ in range(n):` | `n번:` | `n번 반복해` |
| `if condition:` | `만약 condition:` | `만약에 condition` |
| `import random` | Python 유지 | Python 유지 |
| `x = open("f").read()` | Python 유지 | `x에 "f" 읽어서` |
| `open("f", "w").write(v)` | Python 유지 | `"f" 파일에 "v"를 저장해` |
| `x = Path("f").read_text()` | Python 유지 | `x에 "f" 읽어서` |
| 간단한 대입 | Python 유지 | `name은 value` |

영어 출력은 `say`, `ask`, `times`, `when`, `use random`과 자연스러운 영어
문장형을 사용합니다. 파일 읽기·쓰기는 문장형에서 `x에 "f" 읽어서` /
`"f" 파일에 "v"를 저장해`로 변환되며, 초급 변환은 `use file` 모듈이
초급 파일 도구이므로 Python을 유지합니다.

평범한 `import random`을 바꾸면 사용자가 만든 `random`, `random_number`, 한국어
도구 이름을 덮어쓸 수 있으므로 Python으로 유지합니다. 자동 변환이 글자를 변수로
잘못 끼워 넣거나 자연어 질문용 공백을 새로 붙이지
않도록 문자열 따옴표와 질문 내용을 정확히 유지합니다. 결과를 확인한 뒤 더
문장처럼 쓰고 싶을 때 직접 따옴표를 없앨 수 있습니다. 주석, 들여쓰기, 빈 줄,
줄바꿈, 변환 대상이 아닌 Python은 그대로 유지합니다.

## NME 파일을 정리하면 일어나는 일

`.nme` 입력은 먼저 컴파일이 되어야 합니다. 실행되지 않는 프로그램을 정리하는
것은 무슨 뜻이었는지 추측하는 일이므로, `nme 검사`가 알려 주는 문제를 그대로
알려 주고 아무것도 쓰지 않습니다.

컴파일러가 알아본 문장은 요청한 단계와 언어의 대표 표기로 다시 쓰고, 남은
Python 줄은 위의 Python 변환을 그대로 거칩니다. `--level advanced`는 만들어진
Python을 돌려줍니다. 평범한 Python이 곧 NME의 고급 단계이기 때문입니다.

줄 수와 들여쓰기는 변하지 않고, 프로그램이 하는 일도 변하지 않습니다. 정리한
파일에서 만들어지는 Python을 원래 파일에서 만들어지는 Python과 한 바이트씩
비교해서, 뜻이 달라지는 수정은 쓰지 않고 버립니다. 요청한 단계에 표기가 없는
문장은 — 초급은 문장형보다 좁은 범위를 일부러 유지합니다 — 쓴 그대로 남습니다.

## 일부 Python이 남는 이유

NME 고급 문법은 Python입니다. 클래스, 예외 처리, 복잡한 호출, 여러 값을
출력하는 `print`에는 같은 동작을 반드시 보장하는 더 짧은 NME 표현이 없으므로
고급 문법으로 남깁니다. 이것은 실패한 일부 변환이 아니라 완전하게 실행할 수
있는 NME 결과입니다.

결과를 확인하고 검사하세요.

```sh
nme 검사 app
nme 빌드 app -o app.generated.py
```
