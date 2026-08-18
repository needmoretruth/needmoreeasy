# 58 — 데이터: 목록 통계

[English](58-data.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★★ (5/5)
- 선수 지식: [39 — JSON](39-json.ko.md), [52 — 정렬](52-sorting.ko.md)
- 주제: 데이터
- 결과물: JSON 파일에서 숫자를 불러와 표준 라이브러리 statistics로 평균·중앙값·최댓값 구하기

저장된 JSON 목록은 프로그램이 바로 쓸 수 있는 데이터입니다. 이 가이드는
숫자 목록을 파일에서 불러와 손으로 계산한 요약과 Python 표준 라이브러리로
구한 요약을 한 보고서로 출력합니다. `파일 사용`이 데이터를 읽고,
`statistics`가 계산을 합니다.

## 단계

1. 데이터 파일 `numbers.json`을 JSON 목록 하나로 만듭니다:

   ```nme
   [5, 2, 9, 1, 7, 3]
   ```

   숫자 여섯 개를 JSON 목록으로 저장한 것입니다. [39](39-json.ko.md)에서는
   `json저장`으로 딕셔너리를 저장했지요. JSON 목록도 같은 방식으로, 값이
   하나씩 들어 있습니다.

2. `파일 사용 최신`과 `json읽기`로 목록을 불러옵니다. 불러온 값은 Python
   리스트이므로 `len(숫자들)`로 개수를 셀 수 있습니다. `load.ko.nme`로
   저장하세요:

   ```nme
   파일 사용 최신
   숫자들 = json읽기("numbers.json")
   말해 f"숫자 {len(숫자들)}개를 불러왔습니다"
   ```

   `nme 실행 load.ko`를 실행하면 `숫자 6개를 불러왔습니다`가 출력됩니다. 파일에
   목록이 들어 있으므로 `json읽기`는 [39](39-json.ko.md)의 딕셔너리가 아니라
   리스트를 돌려줍니다.

3. 표준 라이브러리에서 `mean`과 `median`을 불러옵니다.
   [64](64-python-packages.ko.md) 가이드도 `from datetime import date`를
   씁니다. 이름 두 개를 한 줄에 가져오는 방법도 같습니다:

   ```nme
   파일 사용 최신
   from statistics import mean, median

   숫자들 = json읽기("numbers.json")
   말해 f"평균: {mean(숫자들)}"
   말해 f"중앙값: {median(숫자들)}"
   ```

   실행하면 `평균: 4.5`와 `중앙값: 4.0`이 보입니다. `mean`은 값을 모두 더해
   개수로 나누고, `median`은 목록을 정렬했을 때 가운데 값입니다.

4. `max(...)`는 Python 내장 함수라 import가 필요 없습니다:

   ```nme
   숫자들 = [5, 2, 9, 1, 7, 3]
   말해 f"최댓값: {max(숫자들)}"
   ```

   `최댓값: 9`가 출력됩니다. `min(숫자들)`은 `1`을 출력할 것입니다.

5. 이제 보고서 전체를 한 파일에 담습니다. `numbers.ko.nme`로 저장합니다:

   ```nme
   # numbers.ko.nme — 저장된 숫자 목록의 통계
   # 실행: nme 실행 numbers.ko
   # numbers.json 파일이 같은 폴더에 있어야 합니다.

   파일 사용 최신
   from statistics import mean, median

   숫자들 = json읽기("numbers.json")

   말해 f"numbers.json에서 숫자 {len(숫자들)}개를 불러왔습니다:"
   for n in 숫자들:
       말해 f"  {n}"

   개수 = 0
   합계 = 0
   for n in 숫자들:
       개수 = 개수 + 1
       합계 = 합계 + n

   최댓값 = 숫자들[0]
   for n in 숫자들:
       if n > 최댓값:
           최댓값 = n

   평균 = 합계 / 개수
   말해 ""
   말해 f"개수: {개수}"
   말해 f"합계: {합계}"
   말해 f"직접 계산한 평균: {평균}"
   말해 f"statistics의 mean: {mean(숫자들)}"
   말해 f"statistics의 median: {median(숫자들)}"
   말해 f"직접 찾은 최댓값: {최댓값}"
   말해 f"max()의 최댓값: {max(숫자들)}"
   ```

   데이터 파일이 있는 상태에서 실행합니다:

   ```sh
   nme 실행 numbers.ko
   ```

   ```text
   numbers.json에서 숫자 6개를 불러왔습니다:
     5
     2
     9
     1
     7
     3

   개수: 6
   합계: 27
   직접 계산한 평균: 4.5
   statistics의 mean: 4.5
   statistics의 median: 4.0
   직접 찾은 최댓값: 9
   max()의 최댓값: 9
   ```

   손으로 짠 반복이 `mean`과 `max`가 내부에서 하는 일을 보여 줍니다:
   계속 더해 가는 합계와 계속 갱신되는 최댓값입니다. `statistics` 줄은
   같은 답을 각각 한 번의 호출로 냅니다.

6. 영어는 같은 단계를 `use file latest`와 `json_load`로 씁니다. 전체 영어
   프로그램은 [영어 가이드](58-data.md)에 있고, 이 조각은 목록을
   불러옵니다:

   ```nme
   use file latest
   numbers = json_load("numbers.json")
   show f"Loaded {len(numbers)} numbers"
   ```

## 직접 해보기

`numbers.json`을 `[10, 20, 30]`으로 바꾸고 `numbers.ko.nme`를 다시
실행해 보세요. 평균, 중앙값, 최댓값이 모두 함께 바뀝니다. 그리고 보고서에
`말해 f"최솟값: {min(숫자들)}"` 줄을 추가해 보세요.

## 배운 것

- 파일에 JSON 목록이 있으면 `json읽기`는 리스트를 돌려줍니다.
- `from statistics import mean, median`은 표준 라이브러리 이름 두 개를
  가져옵니다.
- `mean(숫자들)`과 `median(숫자들)`은 목록 전체를 한 번의 호출로 요약합니다.
- `max(숫자들)`와 `min(숫자들)`은 Python 내장 함수라 import가 필요
  없습니다.
- 손으로 짠 반복으로도 합계와 최댓값을 한 단계씩 찾을 수 있습니다.
