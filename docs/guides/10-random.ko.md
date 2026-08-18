# 10 — 랜덤: 주사위와 선택

[English](10-random.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★☆☆☆ (2/5)
- 선수 지식: [06 — 조건](06-if.ko.md)
- 주제: 랜덤
- 결과물: 주사위를 굴리고 색을 고르는 프로그램

게임에는 깜짝 놀랄 일이 필요합니다. NME에는 Python이 제공하는 랜덤
도우미가 들어 있고, `랜덤 사용` 한 줄로 불러옵니다.

## 단계

1. `dice.nme` 파일을 만듭니다:

   ```nme
   랜덤 사용 최신
   주사위는 1부터 6까지 랜덤정수
   주사위 보여줘
   ```

   여러 번 실행해 보세요. `주사위`는 1부터 6까지 무작위 정수입니다.

2. 여러 선택 중 하나 고르는 것도 같습니다:

   ```nme
   랜덤 사용 최신
   색은 빨강 또는 초록 또는 파랑 중에서 랜덤선택
   색 보여줘
   ```

3. 영어는 `use random latest`로 불러옵니다:

   ```nme
   use random latest
   set die to random number from 1 to 6
   show die
   set color to pick from red or green or blue
   show color
   ```

4. 도우미 이름은 식 안에서도 쓸 수 있습니다. `랜덤정수(가, 나)`는
   무작위 정수, `랜덤선택(값들)`은 목록에서 하나를 고릅니다.

   ```nme
   랜덤 사용 최신
   주사위는 1부터 6까지 랜덤정수
   주사위 보여줘
   ```

## 직접 해보기

주사위를 두 번 굴려 둘 다 보여 주세요:

```nme
랜덤 사용 최신
첫번째는 1부터 6까지 랜덤정수
두번째는 1부터 6까지 랜덤정수
첫번째 보여줘
두번째 보여줘
```

## 배운 것

- `랜덤 사용 최신` / `use random latest`가 내장 랜덤 도우미를 불러옵니다.
- `1부터 6까지 랜덤정수` / `random number from 1 to 6`로 주사위를 굴립니다.
- `... 중에서 랜덤선택` / `pick from ...`으로 하나를 고릅니다.
- `use file`은 같은 프로그램에서 `use random`과 함께 쓸 수 있습니다 —
  [15](15-high-score.ko.md)가 그렇게 최고 점수를 저장합니다.
