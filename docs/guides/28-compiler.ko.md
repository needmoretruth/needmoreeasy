# 28 — 첫 컴파일러 — 아주 작은 언어

[English](28-compiler.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★☆ (4/5)
- 선수 지식: [26 — 모험](26-adventure.ko.md), [23 — 모듈](23-modules.ko.md)
- 주제: 컴파일러
- 결과물: `add 2 3` 같은 줄을 읽고 답을 출력하는 아주 작은 언어

컴파일러는 글을 읽고 그 뜻을 판단합니다. 여러분은 이미 그중 가장 어려운
부분을 [27](27-calculator.ko.md)에서 썼습니다: 줄을 나누고, 첫 단어를 읽고,
그 단어로 갈라지는 것입니다. 컴파일러는 그 일을 프로그램의 모든 줄에 되풀이할
뿐입니다. 이 가이드는 진짜 컴파일러의 씨앗 — `add`와 `mul` 명령이 있는 아주
작은 언어 — 을 만들고 `examples/tiny-compiler.nme` 예제와 비교합니다.

## 단계

1. 파이프라인을 보세요. 아무리 큰 컴파일러라도 반복 속에서 세 단계만
   합니다: 글 줄을 읽고, 단어로 나누고, 단어를 해석합니다. 여러분 언어의
   유일한 일은 각 줄에 답하는 것입니다:

   ```text
   # 파이프라인: 읽기, 나누기, 해석
   line = "add 2 3"
   parts = line.split()
   말해 int(parts[1]) + int(parts[2])
   ```

   실행하면 `5`가 출력됩니다 — 언어가 `add 2 3` 줄에 줄 답과 같습니다.

2. 해석기 전체를 `mini.ko.nme` 한 파일로 저장합니다:

   ```text
   # 아주 작은 계산기 언어: add 2 3, mul 4 5, 또는 quit.
   # 실행: nme 실행 mini.ko

   def run_command(parts):
       verb = parts[0]
       if verb == "add":
           return int(parts[1]) + int(parts[2])
       elif verb == "mul":
           return int(parts[1]) * int(parts[2])
       else:
           return "모르는 명령입니다"


   말해 "미니 언어 — add 2 3, mul 4 5, 또는 quit."

   while True:
       물어봐 line, "다음 줄? "
       만약 line == "quit":
           말해 "안녕히 가세요!"
           break
       parts = line.split()
       if len(parts) == 3:
           result = run_command(parts)
           말해 result
       else:
           말해 "형식: add 2 3 또는 mul 4 5"
   ```

3. 실행하고 작은 프로그램을 넣어 보세요:

   ```sh
   printf 'add 2 3\nmul 4 5\nsub 9 2\nquit\n' | nme 실행 mini.ko
   ```

   ```text
   미니 언어 — add 2 3, mul 4 5, 또는 quit.
   다음 줄? 5
   다음 줄? 20
   다음 줄? 모르는 명령입니다
   다음 줄? 안녕히 가세요!
   ```

   `add 2 3`은 `5`로, `mul 4 5`는 `20`으로 답하고, `sub 9 2`는 아직
   명령이 아니라 모르는 명령 메시지를 받습니다.

4. 해석은 함수입니다. `verb = parts[0]`이 첫 단어에 이름을 붙이고, 동사에
   대한 `if`가 명령마다 갈래 하나로 나눕니다. 둘째와 셋째 단어는
   `int(parts[1])`과 `int(parts[2])`로 숫자가 됩니다:

   ```text
   def run_command(parts):
       verb = parts[0]
       if verb == "add":
           return int(parts[1]) + int(parts[2])
       elif verb == "mul":
           return int(parts[1]) * int(parts[2])
       else:
           return "모르는 명령입니다"
   ```

   마지막 `else`가 오류 처리입니다: 모르는 동사는 크래시 대신 메시지로
   돌아옵니다.

5. 반복이 파이프라인입니다. `물어봐`로 줄을 읽고, `quit`에서 멈추고,
   `.split()`으로 나누고, `len(parts) == 3`으로 모양을 지키고, 해석해
   출력합니다:

   ```text
   while True:
       물어봐 line, "다음 줄? "
       만약 line == "quit":
           말해 "안녕히 가세요!"
           break
       parts = line.split()
       if len(parts) == 3:
           result = run_command(parts)
           말해 result
       else:
           말해 "형식: add 2 3 또는 mul 4 5"
   ```

6. `examples/tiny-compiler.nme` 예제는 진짜 (아주 작은) 컴파일러입니다:
   터미널 대신 목록에서 소스 줄을 읽고, 출력으로 Python 코드를 만듭니다:

   ```text
   # examples/tiny-compiler.nme의 일부
   python_lines = []
   for line in tiny_source:
       words = line.split()
       if words[0] == "말하기":
           text = " ".join(words[1:])
           python_lines.append(f"print({text!r})")
   ```

   각 줄을 나누고 첫 단어를 확인해 `print(...)` 줄을 만듭니다 — 숫자 대신
   만들어진 Python이 답일 뿐 같은 읽기 → 나누기 → 해석 파이프라인입니다.
   숫자 대신 `answer = 5`를 만드는 생성 갈래를 더하면 해석기가
   컴파일러로 바뀝니다.

7. 영어 쌍둥이 `mini.nme`는 같은 `def`를 쓰고 반복을 `ask`, `if`, `show`로
   씁니다:

   ```text
   def run_command(parts):
       verb = parts[0]
       if verb == "add":
           return int(parts[1]) + int(parts[2])


   while True:
       ask line, "Next line? "
       if line == "quit":
           show "Goodbye!"
           break
       parts = line.split()
       result = run_command(parts)
       show result
   ```

   같은 파이프 입력이 두 언어에서 답합니다.

## 직접 해보기

`run_command`에 `sub`와 `div` 명령을 더해 보세요: `sub`는 뺄셈
(`int(parts[1]) - int(parts[2])`)이고 `div`는 나눗셈입니다. 그다음 프로그램에
`sub 9 2`와 `div 12 3`을 넣어 모르는 명령 메시지가 사라지는 것을
보세요.

## 배운 것

- 컴파일러는 글을 읽고 단어로 나누고 각 줄을 해석합니다.
- 동사에 대한 `if`가 달린 `run_command(parts)` 함수가 분기입니다.
- `split()`은 줄을 자르고 `int()`는 글을 숫자로 바꿉니다.
- 읽기 → 나누기 → 해석 파이프라인이 진짜 컴파일러의 씨앗입니다.
