# 81 — 테스트: 내가 쓴 함수 확인하기

[English](81-testing.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★★ (5/5)
- 선수 지식: [23 — Modules](23-modules.ko.md), [46 — Expressions](46-expressions.ko.md)
- 주제: 테스트
- 결과물: 자기 함수를 예상값과 비교해 통과·실패를 보고하는 작은 테스트 실행기

"잠깐만요, 실제로는 테스트하지 않은 상태로 '작동합니다'라고 말하지
않았나요?" — 대부분의 프로그래머가 스스로에게 묻는 질문입니다. 테스트가
이에 답합니다: 함수에 예상값을 넣어 실행하고 결과를 비교하는 프로그램.
한 번 작성해 두면, 바꿀 때마다 다시 실행해 망가진 것이 없는지 확인할
수 있습니다.

## 단계

1. 테스트는 한 줄의 데이터입니다: 이름, 함수, 인자들, 예상 결과.
   `add(2, 3)`은 `5`가 되어야 합니다:

   ```nme
   ["adds numbers", add, 2, 3, 5]
   ```

2. 실행기는 줄을 돌며 함수를 호출하고 비교합니다. 함수가 값으로 줄에
   들어갑니다 — [10](10-random.ko.md)에서 `random_number`가 값이었던 것과
   같은 방식입니다. `testcalc.nme`로 저장하세요:

   ```nme
   # testcalc.nme — 내가 쓴 함수를 작은 테스트 실행기로 확인하기.
   # 실행: nme 실행 testcalc

   def add(a, b):
       return a + b

   def mul(a, b):
       return a * b

   tests = [
       ["adds numbers", add, 2, 3, 5],
       ["adds negatives", add, -1, 1, 0],
       ["multiplies", mul, 3, 4, 12],
       ["multiplies by zero", mul, 9, 0, 0],
   ]

   passed = 0
   for test in tests:
       name = test[0]
       function = test[1]
       result = function(test[2], test[3])
       expected = test[4]
       if result == expected:
           passed = passed + 1
           show f"PASS {name}"
       else:
           show f"FAIL {name}: got {result}, expected {expected}"

   show f"{passed} of {len(tests)} tests passed"
   ```

3. 실행하세요:

   ```sh
   nme 실행 testcalc
   ```

   ```text
   PASS adds numbers
   PASS adds negatives
   PASS multiplies
   PASS multiplies by zero
   4 of 4 tests passed
   ```

   `PASS` 줄 하나마다 약속이 지켜졌다는 뜻입니다: 함수가 테스트가
   기대한 대로 동작했습니다. `FAIL`은 실제 결과를 예상값 옆에
   출력하므로, 망가진 함수가 조용히 실패하는 대신 스스로 이름을
   밝힙니다.

4. 이제 일부러 코드를 망가뜨려 실행기가 잡아내는지 보세요. `mul`을
   `return a - b`로 바꾸고 다시 실행하세요:

   ```text
   FAIL multiplies: got -1, expected 12
   FAIL multiplies by zero: got 9, expected 0
   PASS adds numbers
   PASS adds negatives
   2 of 4 tests passed
   ```

   실행기가 버그를 찾아냈고, 어떤 기대가 깨졌는지까지 가리킵니다 —
   손으로 확인도, 조용한 불시착도 없습니다. 계속하기 전에 원래 코드로
   되돌리세요.

5. 테스트 실행기는 커지는 프로젝트를 보호합니다. 함수는 모듈에
   ([23](23-modules.ko.md)), 테스트는 주 파일에 두고, 바꿀 때마다 테스트를
   실행하세요:

   ```nme
   # from "calc.nme" import add, mul   (모듈 버전)
   ```

   테스트 줄은 그대로입니다. import 줄만 바뀝니다. 이제 `calc.nme`를
   고칠 때마다 같은 실행기가 확인해 줍니다.

## 직접 해보기

빼기와 나누기 함수(`sub`, `div`)를 추가하고, 0으로 나누기 같은 경계
경우도 테스트 줄로 넣어 보세요 — 먼저 `div`가 0을 받으면 어떻게 할지
정하고, 그다음 그것을 말하는 테스트를 쓰세요. `passed == len(tests)`일
때만 `all tests passed`를 출력하는 줄도 추가해 보세요.

## 배운 것

- 테스트 줄은 데이터입니다: 이름, 함수, 인자, 예상 결과.
- 실행기가 함수마다 호출·비교하고 PASS 또는 FAIL을 보고합니다.
- 실패한 테스트는 실제 결과를 예상값 옆에 보여 줍니다.
- 테스트는 "되는 것 같아요"를 "실행기가 된다고 해요"로 바꿉니다.
- 매번 바꿀 때마다 같은 테스트를 다시 실행하면 회귀를 일찍 잡습니다.
