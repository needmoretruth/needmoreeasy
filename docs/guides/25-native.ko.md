# 25 — 네이티브: 기계어로 컴파일하기

[English](25-native.md) | 한국어

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [23 — Modules](23-modules.md), [07 — While](07-while.md)
- 주제 (Topic): 네이티브 컴파일 / native compilation
- 결과물 (Result): CPython 없이 기계어로 실행하기 / running a program as machine code without CPython

지금까지 모든 프로그램은 CPython 위에서 실행됐습니다. NME가 Python으로
컴파일하고 Python이 그것을 실행했습니다. 언어의 작은 부분 — 네이티브 코어
— 은 더 나아가 바로 기계어가 될 수 있습니다. `nme native`는 그 코어를 C로
바꾸고 시스템 C 컴파일러로 네이티브 실행 파일을 만듭니다.

## 단계

1. 네이티브 코어 안에 머무는 프로그램을 쓰세요. 코어는 정수와 문자열
   리터럴, `while`/`if`/`else`, `break`, `return` 함수, `say`를 다룹니다:

   ```text
   score = 0
   while score is less than 10
       score add 1
   end
   show score
   ```

2. 네이티브로 컴파일해 실행하세요:

   ```sh
   nme 네이티브 실행 count
   ```

   ```text
   10
   ```

   짧은 형태 `nme native count`도 같은 방식입니다.

3. C 소스와 실행 파일을 남기려면:

   ```sh
   nme native build count -o count
   ```

   `nme native build`는 실행 파일 옆에 `count.c`를 씁니다. C를 읽으면
   프로그램이 실제로 무엇이 되는지 볼 수 있습니다.

4. 함수와 재귀는 코어 안에서 동작합니다. 한국어판
   `examples/native-factorial.ko.nme`(영어판 `examples/native-factorial.nme`)는
   두 백엔드에서 모두 팩토리얼을 계산합니다:

   ```text
   # part of examples/native-factorial.ko.nme
   def fact(n):
       만약에 n이 2보다 작으면
           return 1
       끝
       return n * fact(n - 1)


   show fact(5)
   ```

   ```sh
   nme r examples/native-factorial.ko
   nme native examples/native-factorial.ko
   ```

   둘 다 `120`을 출력합니다.

5. 코어 밖의 것은 명확한 오류로 거부되며 CPython으로 그대로 실행됩니다.
   네이티브 백엔드는 절대 조용히 잘못 컴파일하지 않습니다:

   ```sh
   nme native ask-demo    # "지원하지 않습니다" 진단 출력
   nme r ask-demo         # CPython으로는 그대로 동작
   ```

## 어떻게 동작하나요

`nme-native`(Rust 크레이트)는 Python 경로와 같은 프론트엔드 AST를 받아
모든 문장을 문서화된 코어와 대조한 뒤 C를 만듭니다. 시스템 C 컴파일러
(`cc`)가 그것을 `-O2`로 기계어로 만듭니다.
[구조 메모](../native-backend.md)는 이 C 백엔드를 LLVM·Cranelift와 비교하고
왜 C가 첫 백엔드인지 설명합니다.

성능은 정직하게 측정됩니다: 이 머신에서 정수 5,000만 회 반복문이
네이티브에서 CPython보다 약 60배 빠릅니다. 빡빡한 반복 하나의
마이크로벤치마크이지, 모든 프로그램에 대한 주장이 아닙니다.

## 직접 해보기

카운트다운을 100까지 세도록 바꾸거나, `square(n)` 함수를 만들고
`square(7)`을 출력한 뒤 `nme native`로 실행해 보세요.

## 배운 것

- 네이티브 코어는 C로, 그리고 `cc`로 네이티브 실행 파일로 컴파일됩니다.
- `nme native 실행`이 실행하고 `nme native 빌드`가 C와 실행 파일을 남깁니다.
- 함수, 반복, 조건, `say`가 모두 코어 안에서 동작합니다.
- 코어 밖에서는 잘못 컴파일하는 대신 프로그램을 거부합니다.
