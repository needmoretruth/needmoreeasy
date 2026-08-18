# 06 — 변경: 값 바꾸기

[English](06-update.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★☆☆☆☆ (1/5)
- 선수 지식: [05 — 저장](05-set.ko.md)
- 주제: 값 변경
- 결과물: 점수에 더하고 빼는 프로그램

게임에는 바뀌는 점수가 필요합니다. `점수는 0`으로 점수를 시작하고,
`더해`로 `+`와 `=` 없이 바꿉니다.

## 단계

1. 쓰는 칸에 이렇게 적습니다:

   ```nme
   점수는 0
   점수에 1 더해
   점수 보여줘
   ```

   `점수는 0`이 숫자 0을 저장하고, `점수에 1 더해`가 1을 더합니다.
   `1`이 출력됩니다.

2. 값은 여러 자연스러운 순서로 바꿀 수 있습니다:

   ```nme
   점수에 2 더해
   점수에서 1 빼줘
   ```

   한 줄씩 바꿔 가며 `점수 보여줘`가 어떻게 바뀌는지 보세요.

3. 영어도 같습니다:

   ```nme
   set score to 0
   score add 1
   add 1 to score
   score increase by 1
   subtract 1 from score
   ```

   `set score to 0`이 `점수는 0`에 해당하는 영어 문장형입니다.

4. 곱하기와 나누기도 같은 방식입니다:

   ```nme
   점수에 2 곱해
   점수를 2로 나눠
   multiply score by 2
   divide score by 2
   ```

5. 고급 표기 `+=`도 같은 뜻이고 올바른 Python입니다:

   ```python
   score += 1
   ```

## 직접 해보기

5를 더해 올라갔다가 2를 빼 내려와 보세요:

```nme
점수는 0
점수에 5 더해
점수에서 2 빼줘
점수 보여줘
```

## 배운 것

- `점수는 0`은 숫자를 저장하고, `set score to 0`이 영어 문장형입니다.
- `점수에 1 더해`, `score add 1`, `add 1 to score`는 모두 더합니다.
- `점수에서 1 빼줘` / `subtract 1 from score`는 빼고, `점수에 2 곱해` / `multiply score by 2`는 곱합니다.
- `score += 1`은 일반 Python에서 같은 뜻입니다.
