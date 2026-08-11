# 78 — 데이터: 한 달 온도 분석하기

[English](78-analysis.md) | 한국어

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [54 — Stats](54-stats.md), [45 — CSV](45-csv.md)
- 주제 (Topic): 데이터 분석 / data analysis
- 결과물 (Result): 한 달치 온도를 읽어 통계와 히스토그램을 계산하고 보고서 파일로 저장하기 / loading a month of temperatures, computing statistics and a histogram, and saving a report file

평균은 시작일 뿐, 이야기 전체는 아닙니다. 이번 가이드는 실제에 가까운
데이터 — 30일치 일일 최고 기온을 분석합니다. 숫자가 데이터를 설명하고,
히스토그램이 모양을 그리고, 보고서 파일이 결과를 보관합니다. 어떤 크기의
데이터 분석 프로젝트에서도 같은 세 단계입니다.

## 단계

1. 30일치 일일 최고 기온(섭씨)으로 `temps.json`을 만드세요:

   ```text
   [22, 24, 21, 25, 27, 26, 23, 20, 19, 24,
    26, 28, 29, 30, 27, 25, 24, 23, 22, 26,
    28, 31, 32, 29, 26, 24, 23, 21, 20, 25]
   ```

2. `json_load`로 불러오고([14](14-json.md)) [54](54-stats.md)의 통계를
   계산합니다:

   ```text
   use file latest

   from statistics import mean, median

   temps = json_load("temps.json")
   average = round(mean(temps), 1)
   middle = median(temps)
   hottest = max(temps)
   coolest = min(temps)
   ```

   `round(mean(temps), 1)`이 소수 한 자리로 만들어 보고서에
   25.333333333333332처럼 길게 나오지 않게 합니다.

3. 히스토그램은 데이터의 모양을 보여 줍니다. 날짜마다 어느 구간에
   들어가는지 세고, 구간마다 막대 하나를 문자열 곱셈으로 그립니다
   ([71](71-chart.md)):

   ```text
   ranges = ["under 20", "20-24", "25-29", "30+"]
   counts = [0, 0, 0, 0]
   for t in temps:
       if t < 20:
           counts[0] = counts[0] + 1
       elif t < 25:
           counts[1] = counts[1] + 1
       elif t < 30:
           counts[2] = counts[2] + 1
       else:
           counts[3] = counts[3] + 1

   for i in range(4):
       show f"{ranges[i]:8s} {'#' * counts[i]}"
   ```

   `elif` 연결은 각 날짜를 정확히 한 구간에 넣으므로 네 개의 개수 합이
   항상 30이 됩니다 — 확인해 볼 가치가 있는 검산입니다.

4. 전체 프로그램은 결과를 `report.txt`에도 씁니다. `analysis.nme`로
   저장하세요:

   ```text
   # analysis.nme — 한 달 온도를 분석해 보고서를 쓰기.
   # 실행: nme r analysis
   # 같은 폴더에 temps.json이 있어야 합니다.

   use file latest

   from statistics import mean, median

   temps = json_load("temps.json")
   average = round(mean(temps), 1)
   middle = median(temps)
   hottest = max(temps)
   coolest = min(temps)

   lines = []
   lines.append(f"temperature report ({len(temps)} days)")
   lines.append(f"  average: {average}")
   lines.append(f"  median:  {middle}")
   lines.append(f"  hottest: {hottest}")
   lines.append(f"  coolest: {coolest}")

   ranges = ["under 20", "20-24", "25-29", "30+"]
   counts = [0, 0, 0, 0]
   for t in temps:
       if t < 20:
           counts[0] = counts[0] + 1
       elif t < 25:
           counts[1] = counts[1] + 1
       elif t < 30:
           counts[2] = counts[2] + 1
       else:
           counts[3] = counts[3] + 1

   lines.append("distribution:")
   for i in range(4):
       lines.append(f"  {ranges[i]:8s} {'#' * counts[i]}")

   report = "\n".join(lines)
   show report
   file_write("report.txt", report)
   show "saved report.txt"
   ```

   모든 결과를 먼저 `lines`에 모은 뒤, 같은 텍스트를 화면과 파일에
   씁니다 — 보고서가 화면과 파일 사이에서 어긋날 수 없습니다.

5. 실행하세요:

   ```sh
   nme r analysis
   ```

   ```text
   temperature report (30 days)
     average: 25
     median:  25.0
     hottest: 32
     coolest: 19
   distribution:
     under 20 #
     20-24    #############
     25-29    ##############
     30+      ##
   ```

   숫자와 히스토그램이 일치합니다. 대부분의 날은 20~29도에 있고, 선선한
   날이 한 번, 더운 날이 두 번 있습니다.

## 직접 해보기

작년 온도로 파일을 하나 더 만들고 평균을 비교해 보세요. 또는 "더운 날"
(`>= 30`) 수를 세어 보고서에 넣어 보세요. `ranges` 목록은 데이터입니다 —
`["under 18", "18-27", "28+"]`로 바꿔도 `elif` 연결이 모든 날을 빠짐없이
구간에 넣습니다.

## 배운 것

- `elif` 연결 하나로 모든 값을 정확히 한 구간에 넣을 수 있습니다.
- 히스토그램은 구간별 개수를 `'#' * count`로 그린 것입니다.
- `round(값, 1)`은 보고서 숫자를 짧게 유지합니다.
- 보고서를 `lines` 목록으로 모으면 화면과 파일이 항상 같습니다.
- 통계와 히스토그램을 함께 쓰면 어느 하나만 쓸 때보다 데이터를 더 잘
  설명합니다.
