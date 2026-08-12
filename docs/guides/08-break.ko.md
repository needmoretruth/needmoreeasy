# 08 — 멈춤: 반복 멈추기

[English](08-break.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도 (Difficulty): ★★★☆☆ (3/5)
- 선수 지식 (Prerequisites): [07 — 동안](07-while.ko.md)
- 주제 (Topic): 반복 / loops
- 결과물 (Result): 반복을 일찍 끝내는 프로그램 / a program that leaves a loop early

반복이 도중에 멈춰야 할 때가 있습니다. `멈춰`는 가장 가까운 반복을
즉시 끝내고 그다음 줄로 넘어갑니다.

## 단계

1. `stop.nme` 파일을 만듭니다:

   ```text
   준비 = True
   기다림 = False
   동안 준비 또는 기다림
       아직 작동 중이에요 말해줘
       멈춰
   끝
   ```

   첫 바퀴가 돌며 `아직 작동 중이에요`를 출력하고 `멈춰`가 반복을 끝내므로
   정확히 한 번 실행됩니다.

2. 영어는 `while`, `break`, `end`를 씁니다:

   ```text
   ready = True
   while ready
       show working
       break
   end
   ```

3. 문장 표기 `여기서 멈춰`와 `break here`도 블록 안에서는 같은 뜻입니다:

   ```text
   점수는 0
   동안 점수 < 10
   점수에 1 더해
   break here
   끝
   ```

4. 반복 밖의 `멈춰`는 코드 `E0102` 오류입니다. 읽는 방법은 가이드 11에서
   배웁니다. `멈춰`는 `동안`이나 `반복` 블록 안에서만 쓰세요.

## 직접 해보기

이름이 생기는 즉시 반복을 멈추세요:

```text
이름은 ""
동안 이름 == ""
    이름을 물어봐 이름이 뭐예요?
    만약에 이름이 있으면
        멈춰
    끝
끝
```

## 배운 것

- `멈춰` / `break`는 가장 가까운 반복을 즉시 끝냅니다.
- `여기서 멈춰` / `break here`가 문장 표기입니다.
- 바로 멈추는 반복도 한 번은 실행합니다.
- 반복 밖의 `멈춰`는 `E0102` 오류입니다.
