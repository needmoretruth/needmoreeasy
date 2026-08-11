# 07 — 동안: 조건이 참인 동안 반복

[English](07-while.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도 (Difficulty): ★★★☆☆ (3/5)
- 선수 지식 (Prerequisites): [06 — 조건](06-if.ko.md)
- 주제 (Topic): 반복과 조건 / loops and conditions
- 결과물 (Result): 조건이 참인 동안 반복하는 블록 / a block that loops while a condition is true

`반복`은 정해진 횟수만큼 실행합니다. `동안`은 조건이 참인 동안 계속
실행하고, 거짓이 되면 멈춥니다.

## 단계

1. `count.nme` 파일을 만듭니다:

   ```text
   점수는 0
   동안 점수 < 3
   show 점수
   add 1 to 점수
   끝
   ```

   블록은 `끝`으로 닫으므로 들여쓰기는 선택입니다. `0`, `1`, `2`가
   출력되고 점수가 3이 되면 멈춥니다.

2. 영어는 `while`과 `end`를 씁니다:

   ```text
   score = 0
   while score < 3
   show score
   add 1 to score
   end
   ```

3. 한 조건 안에 영어와 한국어를 섞을 수 있습니다. 한국어 끝맺음 `동안`은
   주어 뒤에 옵니다:

   ```text
   점수는 0
   while 점수가 3보다 작을 동안
   show 점수
   add 1 to 점수
   end
   ```

4. 말하는 것처럼 한 줄 반복도 가능합니다. 이 줄은 멈추지 않으므로
   터미널에서 Ctrl+C로 멈추세요:

   ```text
   준비 = True
   준비하는동안 성공 말해줘
   ```

## 직접 해보기

이름이 비어 있는 동안 계속 물어보는 반복을 만들어 보세요:

```text
name = ""
while name == ""
    ask name What is your name?
end
```

## 배운 것

- `동안 조건` ... `끝`은 조건이 참인 동안 반복합니다.
- `while condition` ... `end`가 영어 표기입니다.
- 반복은 조건을 바꿔야 멈춥니다. 바꾸지 않으면 끝나지 않습니다.
- 말하는 한국어는 주어 뒤에 끝맺음을 붙일 수 있습니다: `준비하는동안`.
