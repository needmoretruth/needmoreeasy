# 25 — 네이티브: 기계어로 컴파일하기

[English](25-native.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [23 — Modules](23-modules.ko.md), [07 — While](07-while.ko.md)
- 주제 (Topic): 네이티브 컴파일 / native compilation
- 결과물 (Result): CPython 없이 기계어로 실행하기 / running a program as machine code without CPython

지금까지 모든 프로그램은 CPython 위에서 실행됐습니다. NME가 Python으로
컴파일하고 Python이 그것을 실행했습니다. 언어의 작은 부분 — 네이티브 코어
— 은 더 나아가 바로 기계어가 될 수 있습니다. `nme 네이티브 실행`은 그 코어를 C로
바꾸고 시스템 C 컴파일러로 네이티브 실행 파일을 만듭니다.

## 단계

1. 네이티브 코어 안에 머무는 프로그램을 쓰세요. 코어는 정수·유한 실수와
   문자열 리터럴, `while`/`if`/`else`, `break`, `return` 함수, `say`를 다룹니다:

   ```text
   score = 0
   while score is less than 10
       score add 1
   end
   show score
   ```

   네이티브 문자열 변수는 검사하는 8192바이트 버퍼를 사용합니다. 저장하거나
   이어붙인 값이 UTF-8 8191바이트보다 크면 네이티브 프로그램이 영어·한국어
   런타임 오류와 함께 중단됩니다. 이스케이프한 줄바꿈과 탭은 지원하지만 내부
   NUL 문자는 거부합니다. 제한 없는 텍스트에는 CPython 경로를 쓰세요.
   저장 한도는 UTF-8 바이트 기준이지만 `len`은 유니코드 문자 수를 셉니다.
   네이티브 정수는 `-2147483648`부터 `2147483647`까지의 부호 있는
   32비트 값입니다. 오버플로와 0 제수 나머지 연산은 영어·한국어 런타임
   오류로 중단됩니다. 네이티브 함수는 현재 정수만 매개변수로 받고 반환하며,
   각 함수에 조건부 블록 뒤의 최상위 정수 `return`이 필요합니다. 같은 파일의
   뒤쪽에 정의된 함수도 호출할 수 있으며 선언된 위치 인자 개수를 맞춰야 합니다.
   헤더에는
   단순한 정수 매개변수만 쓰며 기본값·가변 인자·키워드 인자는 네이티브 코어
   밖에 있고 중첩 함수 정의도 지원하지 않습니다.
   실수 리터럴은 유한해야 합니다. 네이티브 실수 산술은 C `double`을 쓰며,
   `%g` 출력 때문에 `5.0`이 `5`로 보일 수 있고 `-0.0`의 부호는 `-0`으로
   유지됩니다.
   네이티브 식에서는 먼저 대입한 이름을 사용하거나 선언된 함수를 호출해야
   합니다. 함수 값을 사용하거나 매개변수를 중복해서 적거나 함수 이름을
   변수·매개변수로 다시 쓰는 경우에는 거부되므로 동적인 Python 이름 동작에는
   CPython 경로를 사용하세요.
   `if true` 뒤에서 실행되지 않는 `else`나 `else if`에만 대입한 이름은 블록
   밖에서 사용할 수 없으며, 한 분기에서 처음 대입한 이름을 형제 분기에서
   읽을 수도 없습니다.

2. 네이티브로 컴파일해 실행하세요:

   ```sh
   nme 네이티브 실행 count
   ```

   ```text
   10
   ```

   영어 명령 `nme native count`도 같은 방식입니다.

3. C 소스와 실행 파일을 남기려면:

   ```sh
   nme 네이티브 빌드 count -o count
   ```

   `nme 네이티브 빌드`는 실행 파일 옆에 `count.c`를 씁니다. `-o` 없이 `.ko`
   파일을 빌드하면 C 이름에도 접미사를 유지한 `count.ko.c`를 만들어 영어판과
   한국어판을 한 폴더에서 빌드할 수 있습니다. Windows에서는 원본 줄기가 `.ko`로
   끝나도 기본 출력에 `.exe`가 붙습니다. C를 읽으면 프로그램이 실제로 무엇이
   되는지 볼 수 있습니다. `count.c.nme` 같은 원본은 Unix에서 기본 실행 파일
   `count.c`, Windows에서 `count.c.exe`를 사용하고, 생성 C 소스는 `count.c.c`입니다.
   명시적인 `-o count.c`만 C 소스 충돌로 거부됩니다.
   `-o` 옵션은 `빌드`에만 해당합니다. `nme 네이티브 실행 count -o count`는
   `실행`이 결과 파일을 저장하지 않으므로 E9031으로 거부됩니다.
   동작 단어는 하나만 선택하세요. `실행`과 `빌드`를 함께 적으면 마지막 단어를
   조용히 적용하지 않고 E9032로 거부합니다.

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
   nme 네이티브 실행 examples/native-factorial.ko
   ```

   둘 다 `120`을 출력합니다.

5. 코어 밖의 것은 명확한 오류로 거부되며 CPython으로 그대로 실행됩니다.
   네이티브 백엔드는 절대 조용히 잘못 컴파일하지 않습니다:

   ```sh
   nme 네이티브 실행 ask-demo    # "지원하지 않습니다" 진단 출력
   nme r ask-demo         # CPython으로는 그대로 동작
   ```

## 어떻게 동작하나요

`nme-native`(Rust 크레이트)는 Python 경로와 같은 프론트엔드 AST를 받아
모든 문장을 문서화된 코어와 대조한 뒤 C를 만듭니다. macOS와 Linux에서는
`cc`가 그것을 `-O2`로 기계어로 만들고, Windows에서는 Visual Studio
Developer PowerShell의 Microsoft `cl`이 `/O2`와 `/utf-8`로 같은 일을 합니다.
[구조 메모](../native-backend.ko.md)는 이 C 백엔드를 LLVM·Cranelift와 비교하고
왜 C가 첫 백엔드인지 설명합니다.

성능은 정직하게 측정됩니다: 이 머신에서 정수 5,000만 회 반복문이
네이티브에서 CPython보다 약 60배 빠릅니다. 빡빡한 반복 하나의
마이크로벤치마크이지, 모든 프로그램에 대한 주장이 아닙니다.

## 직접 해보기

카운트다운을 100까지 세도록 바꾸거나, `square(n)` 함수를 만들고
`square(7)`을 출력한 뒤 `nme 네이티브 실행`으로 실행해 보세요.

## 배운 것

- 네이티브 코어는 C로, macOS·Linux에서는 `cc`로, Windows에서는 MSVC `cl`로
  네이티브 실행 파일로 컴파일됩니다.
- `nme 네이티브 실행`이 실행하고 `nme 네이티브 빌드`가 C와 실행 파일을
  남깁니다. 영어 명령은 `nme native run`과 `nme native build`입니다.
- 함수, 반복, 조건, `say`가 모두 코어 안에서 동작합니다.
- 코어 밖에서는 잘못 컴파일하는 대신 프로그램을 거부합니다.
