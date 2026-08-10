# Python을 NME로 변환하기

[English](converting-python.md) | 한국어

변환기는 입력 전체가 올바른 Python인지 먼저 확인한 뒤, 요청한 NME 단계에서
뜻을 안전하게 보존할 수 있는 줄만 바꿉니다.

```sh
nme 변환 app.py --level 문장형 --language 한국어 -o app.nme
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
| 간단한 대입 | Python 유지 | `name은 value` |

영어 출력은 `say`, `ask`, `times`, `when`, `use random`과 자연스러운 영어
문장형을 사용합니다.

평범한 `import random`을 바꾸면 사용자가 만든 `random`, `random_number`, 한국어
도구 이름을 덮어쓸 수 있으므로 Python으로 유지합니다. 자동 변환이 글자를 변수로
잘못 끼워 넣거나 자연어 질문용 공백을 새로 붙이지
않도록 문자열 따옴표와 질문 내용을 정확히 유지합니다. 결과를 확인한 뒤 더
문장처럼 쓰고 싶을 때 직접 따옴표를 없앨 수 있습니다. 주석, 들여쓰기, 빈 줄,
줄바꿈, 변환 대상이 아닌 Python은 그대로 유지합니다.

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
