# 45 — CSV: 데이터 행

[English](45-csv.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★☆ (4/5)
- 선수 지식: [13 — 파일](13-files.ko.md), [30 — 데이터](30-data.ko.md)
- 주제: 데이터
- 결과물: 쉼표로 나뉜 텍스트 파일을 split(",")로 읽고 한 열의 평균을 구해 요약 CSV를 쓰기

CSV 파일은 그냥 텍스트입니다 — 줄마다 행 하나, 쉼표로 구분된 필드들.
작은 파일이라면 `split(",")`만으로 충분합니다.

## 단계

1. 줄마다 `이름,점수` 행 하나를 가진 `data.csv`를 만듭니다:

   ```text
   Mina,90
   Yuna,70
   Sora,85
   Jun,75
   ```

2. 파일을 읽고 행으로 자릅니다. `splitlines()`가 텍스트를 줄로 나누고,
   `line.split(",")`가 한 행을 필드로 나눕니다:

   ```text
   파일 사용 최신
   줄들 = 파일읽기("data.csv").splitlines()
   조각 = 줄들[0].split(",")
   말해 조각[0]
   말해 int(조각[1])
   ```

3. 점수에 합계를 더하고 행 수로 나눠 평균을 구합니다. `scores.ko.nme`으로
   저장합니다 — 모든 행을 모으고 요약 파일을 씁니다:

   ```text
   # scores.ko.nme — CSV를 읽고 점수 평균을 구해 요약 파일 쓰기.
   # 실행: nme 실행 scores.ko

   파일 사용 최신

   원문 = 파일읽기("data.csv")
   줄들 = 원문.splitlines()

   이름들 = []
   점수들 = []
   for 줄 in 줄들:
       조각 = 줄.split(",")
       이름들.append(조각[0])
       점수들.append(int(조각[1]))

   총합 = 0
   최고 = 점수들[0]
   for 점수 in 점수들:
       총합 = 총합 + 점수
       if 점수 > 최고:
           최고 = 점수
   평균 = 총합 / len(점수들)

   말해 f"data.csv에서 {len(줄들)}줄을 읽었습니다"
   for i in range(len(이름들)):
       말해 f"{이름들[i]}: {점수들[i]}"
   말해 f"총합: {총합}"
   말해 f"평균: {평균}"
   말해 f"최고: {최고}"

   요약 = f"rows,{len(줄들)}\ntotal,{총합}\naverage,{평균}\nhighest,{최고}\n"
   파일쓰기("summary.csv", 요약)
   말해 "summary.csv 작성 완료"
   ```

4. `data.csv` 옆에서 실행하고, 새 `summary.csv`도 확인합니다:

   ```sh
   nme 실행 scores.ko
   ```

   ```text
   data.csv에서 4줄을 읽었습니다
   Mina: 90
   Yuna: 70
   Sora: 85
   Jun: 75
   총합: 320
   평균: 80.0
   최고: 90
   summary.csv 작성 완료
   ```

   ```text
   rows,4
   total,320
   average,80.0
   highest,90
   ```

## 직접 해보기

가장 낮은 점수도 추적해 보세요 — `최저 = 점수들[0]`로 시작하고 `if 점수 < 최저:`로 갱신한 뒤 `요약`에 `lowest,<값>` 줄을 더합니다.

## 배운 것

- `파일읽기(...).splitlines()`이 파일을 행으로 자릅니다.
- `줄.split(",")`이 행을 필드로 나누고 `조각[1]`이 둘째 필드입니다.
- `int(조각[1])`이 글자를 숫자로 바꿔 더할 수 있게 합니다.
- `파일쓰기`가 요약을 CSV로 다시 써 줍니다.
