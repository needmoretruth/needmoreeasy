# 11 — 확인과 빌드: Python 보기

[English](11-check-build.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★☆☆☆ (2/5)
- 선수 지식: [08 — 멈춤](08-break.ko.md)
- 주제: 도구 사용
- 결과물: 프로그램을 확인하고 생성된 Python을 읽는 습관

`nme check`와 `nme build`는 NME가 실제로 무엇인지 보여 줍니다. `check`는
Python에게 생성된 프로그램이 올바른지 물어보고, `build`는 생성된
Python 그 자체를 보여 줍니다.

## 단계

1. 실행하지 않고 프로그램을 확인합니다:

   ```sh
   nme 검사 hello
   nme 검사 hello
   ```

   프로그램이 정상이면 `check`는 아무것도 출력하지 않습니다. 조용함이
   성공입니다. `nme c`가 짧은 명령입니다.

2. 프로그램이 어떤 Python이 되는지 봅니다:

   ```sh
   nme 빌드 hello -o hello.py
   python3 hello.py
   ```

   `안녕하세요! 말해줘`라면 `hello.py`에는 다음이 들어 있습니다:

   ```python
   print("안녕하세요!")
   ```

   이렇게 읽는 것이 Python으로 한 줄씩 자라나는 방법입니다.

3. 모든 오류에는 안정적인 코드가 붙습니다. `break`를 반복 밖(블록 안이
   아닌 왼쪽 끝 줄)에 적어서 확인해 보세요:

   ```sh
   nme 검사 broken.nme
   ```

   컴파일러는 코드, 정확한 줄, 힌트를 함께 출력합니다:

   ```text
   오류[E0102]: `멈춰`는 반복문 안에서만 쓸 수 있어요
   error[E0102]: `break` can only be used inside a loop
     --> broken.nme:1:1
     |
   1 | break
     | ^^^^^
     = 도움말: `동안 ... 끝` 또는 `반복 ... 끝` 안에 넣어 주세요
     = hint: put it inside `while ... end` or `repeat ... end`
   ```

4. 코드의 자세한 설명을 한국어나 영어로 읽습니다:

   ```sh
   nme ko E0102
   nme en E0102
   ```

   `nme ko`만 실행하면 모든 코드를 나열합니다.

## 직접 해보기

지금까지 만든 모든 가이드 프로그램을 `nme 검사 <파일>`로 확인하고, 하나를
`nme 빌드`해 생성된 Python을 읽어 보세요.

## 배운 것

- `nme 검사` / `nme check` / `nme c`는 실행 없이 확인하고, 조용하면
  성공입니다.
- `nme 빌드` / `nme build` / `nme b`는 생성된 Python을 보여 주고, `-o`로
  저장합니다.
- 오류 메시지는 `E0102` 같은 코드와 정확한 캐럿, 힌트를 담습니다.
- `nme ko <코드>` / `nme en <코드>`로 각 코드를 설명받을 수 있습니다.
