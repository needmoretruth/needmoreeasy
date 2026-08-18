# 54 — 통계: 데이터 이해

[English](54-stats.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★★ (5/5)
- 선수 지식: [30 — Data](30-data.ko.md), [42 — Compare](42-compare.ko.md)
- 주제: 통계/데이터
- 결과물: JSON 숫자 목록에서 count·평균·중앙값·최빈값·최소·최대·범위 보고하기

평균 하나는 데이터에 대해 알려 주지 못하는 것이 많습니다. 이 가이드는 숫자
목록을 불러와 여러 통계를 보고합니다. 각 통계는 데이터에 대해 다른 질문에
답합니다.

## 단계

1. 점수 목록을 담은 `numbers.json`을 만드세요:

   ```text
   [5, 2, 9, 1, 7, 3, 5, 8]
   ```

2. `statistics`가 라이브러리 버전을 제공합니다. 이 가이드는 평균을 손으로도
   계산해 그 의미를 보여 줍니다:

   ```text
   from statistics import mean, median, mode
   ```

   `sum(numbers) / len(numbers)`가 평균입니다: 합계 나누기 개수. 점수를
   정렬했을 때 가운데 값이 중앙값이고, 가장 자주 나오는 값이 최빈값입니다.

3. 전체 프로그램은 목록을 불러와 모든 통계를 출력합니다. `stats.nme`로
   저장하세요:

   ```text
   # stats.nme — 하나의 목록에 대한 여러 통계.
   # 실행: nme 실행 stats
   # 같은 폴더에 numbers.json이 있어야 합니다.

   use file latest

   from statistics import mean, median, mode

   numbers = json_load("numbers.json")

   말해 f"count: {len(numbers)}"
   말해 f"total: {sum(numbers)}"
   말해 f"mean by hand: {sum(numbers) / len(numbers)}"
   말해 f"mean from statistics: {mean(numbers)}"
   말해 f"median: {median(numbers)}"
   말해 f"mode: {mode(numbers)}"
   말해 f"min: {min(numbers)}"
   말해 f"max: {max(numbers)}"
   말해 f"range: {max(numbers) - min(numbers)}"
   ```

4. 실행하세요:

   ```sh
   nme 실행 stats
   ```

   ```text
   count: 8
   total: 40
   mean by hand: 5.0
   mean from statistics: 5.0
   median: 5.0
   mode: 5
   min: 1
   max: 9
   range: 8
   ```

5. 각 통계가 무엇을 말해 주는지:

   - **count** — 숫자가 몇 개인지.
   - **mean** — 균형점: 모두가 합계를 똑같이 나누면 각자 받을 값.
   - **median** — 정렬했을 때 가운데 값. 하나의 이상치가 평균을 움직여도
     중앙값은 안 움직입니다.
   - **mode** — 가장 흔한 값. 숫자가 아니라 범주에도 쓸 수 있는 유일한
     통계입니다.
   - **min/max** — 양끝값.
   - **range** — 데이터가 얼마나 퍼져 있는지(max − min).

## 직접 해보기

`numbers.json`을 `[10, 9, 9, 8]`로 바꾸고 다시 실행하세요: 중앙값은 9로
남지만 평균은 바뀝니다. 둘 다 중요한 이유가 보입니다.

## 배운 것

- `statistics.mean/median/mode`가 목록을 숫자 하나로 요약합니다.
- `sum(...) / len(...)`이 손으로 계산한 평균입니다.
- `min`/`max`가 양끝을, `range = max − min`이 범위를 구합니다.
- 평균·중앙값·최빈값은 같은 데이터에 다른 질문에 답합니다.
