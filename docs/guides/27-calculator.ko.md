# 27 — 계산기 — 명령줄 프로젝트

[English](27-calculator.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★☆ (4/5)
- 선수 지식: [24 — Python 패키지](24-python-packages.ko.md), [23 — 모듈](23-modules.ko.md)
- 주제: 프로젝트
- 결과물: 함수와 모듈 파일로 계속 물어보는 계산기

`3 + 4`를 읽고 답한 뒤 `quit`을 입력할 때까지 다시 물어보는 계산기는 완성된
작은 프로젝트입니다. 한 프로그램에서 세 단계를 모두 씁니다: 입력에 초급
`물어봐`, 반복과 계산에 순수 Python `while True:`와 `def`, 출력에 NME
`말해`.

## 단계

1. 계산기 전체를 `calculator.ko.nme` 한 파일로 저장하고 실행합니다:

   ```text
   # 명령줄 계산기: 3 + 4 를 입력하거나 quit.
   # 실행: nme 실행 calculator.ko

   def calculate(parts):
       if parts[1] == "+":
           return int(parts[0]) + int(parts[2])
       elif parts[1] == "-":
           return int(parts[0]) - int(parts[2])
       elif parts[1] == "*":
           return int(parts[0]) * int(parts[2])
       else:
           return int(parts[0]) / int(parts[2])


   말해 "계산기 — 3 + 4 같은 명령을 입력하거나 quit."

   while True:
       물어봐 command, "명령을 입력하세요? "
       만약 command == "quit":
           말해 "안녕!"
           break
       parts = command.split()
       if len(parts) == 3:
           answer = calculate(parts)
           말해 f"{command} = {answer}"
       else:
           말해 "형식: 숫자 연산자 숫자"
   ```

   ```sh
   printf '3 + 4\n10 - 3\n7 * 6\n10 / 4\nquit\n' | nme 실행 calculator.ko
   ```

   ```text
   계산기 — 3 + 4 같은 명령을 입력하거나 quit.
   명령을 입력하세요? 3 + 4 = 7
   명령을 입력하세요? 10 - 3 = 7
   명령을 입력하세요? 7 * 6 = 42
   명령을 입력하세요? 10 / 4 = 2.5
   명령을 입력하세요? 안녕!
   ```

2. 계산은 함수에 들어 있습니다. `def`가 이름을 붙이고, `parts`가 나뉜
   명령을 담으며, `return`이 답을 돌려줍니다. `parts[1]`은 연산자이고
   `int(parts[0])`은 글자 `"3"`을 숫자 `3`으로 바꿉니다:

   ```text
   def calculate(parts):
       if parts[1] == "+":
           return int(parts[0]) + int(parts[2])
       elif parts[1] == "-":
           return int(parts[0]) - int(parts[2])
       elif parts[1] == "*":
           return int(parts[0]) * int(parts[2])
       else:
           return int(parts[0]) / int(parts[2])
   ```

   `elif`가 연산자마다 갈래를 만들고, 마지막 `else`가 나눗셈입니다. 이
   블록은 NME 파일 안에 그대로 쓴 고급 Python입니다.

3. `split()`이 명령을 단어로 자릅니다. `"3 + 4".split()`은
   `['3', '+', '4']`가 되므로 `parts[0]`은 첫 숫자, `parts[1]`은 연산자,
   `parts[2]`는 둘째 숫자입니다. `len(parts) == 3`은 형식에 맞지 않는
   명령을 거릅니다:

   ```text
   parts = command.split()
   if len(parts) == 3:
       answer = calculate(parts)
       말해 f"{command} = {answer}"
   else:
       말해 "형식: 숫자 연산자 숫자"
   ```

   `말해 f"{command} = {answer}"`는 입력한 줄과 결과를 한 줄로 출력합니다.

4. 반복은 [22](22-terminal-menu.ko.md) 가이드의 메뉴와 같은 모양입니다:
   `while True:`는 스스로 끝나지 않으므로 `quit`이 `break`로 나가야 합니다.
   그 사이가 계산기의 한 턴입니다:

   ```text
   while True:
       물어봐 command, "명령을 입력하세요? "
       만약 command == "quit":
           말해 "안녕!"
           break
       parts = command.split()
       if len(parts) == 3:
           answer = calculate(parts)
           말해 f"{command} = {answer}"
       else:
           말해 "형식: 숫자 연산자 숫자"
   ```

5. 함수가 작동하면 [23](23-modules.ko.md) 가이드처럼 함수를 자기 모듈로
   옮깁니다. `calc.nme`에 함수만 저장합니다:

   ```text
   # calc.nme — calculate 함수만
   def calculate(parts):
       if parts[1] == "+":
           return int(parts[0]) + int(parts[2])
       elif parts[1] == "-":
           return int(parts[0]) - int(parts[2])
       elif parts[1] == "*":
           return int(parts[0]) * int(parts[2])
       else:
           return int(parts[0]) / int(parts[2])
   ```

   그다음 `calculator.ko.nme` 맨 위에서 불러옵니다. 이제 주 파일은 반복만
   설명하고 계산은 `calc.nme`에 남습니다:

   ```text
   from "calc.nme" import calculate

   말해 "계산기 — 3 + 4 같은 명령을 입력하거나 quit."

   while True:
       물어봐 command, "명령을 입력하세요? "
       만약 command == "quit":
           말해 "안녕!"
           break
       parts = command.split()
       if len(parts) == 3:
           answer = calculate(parts)
           말해 f"{command} = {answer}"
       else:
           말해 "형식: 숫자 연산자 숫자"
   ```

   `nme 검사 calculator.ko`가 두 파일을 모두 확인하고, `nme r
   calculator.ko`는 전처럼 import를 실행합니다.

6. 영어 쌍둥이 `calculator.nme`는 같은 `def`를 쓰고 반복을 `ask`,
   `if`, `show`로 씁니다:

   ```text
   def calculate(parts):
       if parts[1] == "+":
           return int(parts[0]) + int(parts[2])


   while True:
       ask command, "Your command? "
       if command == "quit":
           show "Bye!"
           break
       parts = command.split()
       answer = calculate(parts)
       show f"{command} = {answer}"
   ```

   같은 파이프 입력이 두 언어에서 같은 답을 냅니다.

## 직접 해보기

다섯 번째 연산자를 더해 보세요: `else` 갈래를 `//`(나머지 없는 나눗셈,
`int(parts[0]) // int(parts[2])`)로 바꾼 뒤 `17 // 5`를 시험하세요. 또는
모르는 연산자에 나눗셈 대신 친절한 메시지를 출력하게 하세요.

## 배운 것

- `def`/`return`이 계산을 재사용하는 함수 하나로 묶습니다.
- `command.split()`은 줄을 토막내고 `int(parts[0])`은 숫자를 읽습니다.
- `while True:`에 `quit`이 `break`로 나가면 계속 물어보는 반복이 됩니다.
- 함수를 `.nme` 모듈로 옮기면 프로젝트가 깔끔해집니다.
