# 09 — 그리고/또는: 조건 합치기

[English](09-and-or.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도 (Difficulty): ★★★☆☆ (3/5)
- 선수 지식 (Prerequisites): [08 — 멈춤](08-break.ko.md)
- 주제 (Topic): 조건 / conditions
- 결과물 (Result): 조건을 합쳐 판단하는 프로그램 / a program that judges combined conditions

실제 조건은 질문 두 개인 경우가 많습니다. `그리고`는 둘 다 참이어야
하고, `또는`은 하나만 참이면 됩니다.

## 단계

1. `combine.nme` 파일을 만듭니다:

   ```text
   준비 = True
   점수 = 5
   만약 준비 그리고 점수가 2보다 크면 성공 말해줘
   ```

   두 조건이 모두 참이므로 `성공`이 출력됩니다.

2. 영어는 `and`와 `or`로 이어 붙입니다:

   ```text
   ready = True
   score = 5
   if ready and score > 2 then show Go
   ```

   `or`는 한쪽만 참이면 됩니다:

   ```text
   준비 = True
   기다림 = False
   if 준비 or 기다림 then show Please wait
   ```

3. `그리고`가 `또는`보다 먼저 묶입니다. Python과 똑같습니다:

   ```text
   만약 준비 그리고 기다림 또는 점수 > 2 그러면 성공 말해줘
   ```

   이 조건은 `(준비 그리고 기다림) 또는 (점수 > 2)`를 뜻합니다.

4. 합친 조건은 반복에서도 쓸 수 있습니다:

   ```text
   동안 준비 또는 기다림
   show Still working
   멈춰
   끝
   ```

## 직접 해보기

이름이 있고 시간이 늦었을 때만 인사를 보여 주세요:

```text
name = "Mina"
hour = 21
만약에 이름이 있으면 그리고 hour가 18보다 크면 잘 자요 이름! 말해줘
```

## 배운 것

- `그리고` / `and`는 두 조건이 모두 참이어야 합니다.
- `또는` / `or`는 하나만 참이면 됩니다.
- `그리고`가 `또는`보다 먼저 묶입니다. 일반 Python과 같습니다.
- 합친 조건은 `만약`, `if`, `동안` 어디에서나 씁니다.
