# 72 — 모듈: 여러 파일 프로젝트

[English](72-project-files.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [67 — 성적부](67-grade-book.ko.md), [23 — 모듈](23-modules.ko.md)
- 주제 (Topic): 모듈/구조 / modules & structure
- 결과물 (Result): fetch·analyze·report 세 .nme 모듈로 나누고 주 프로그램이 가져오는 작은 날씨 보고서 프로젝트 / a small weather-report project split into three .nme modules (fetch, analyze, report) with clear interfaces, imported by a main program

[23](23-modules.ko.md)은 프로그램에서 도우미를 나누었고,
[67](67-grade-book.ko.md)은 저장 모듈 하나를 썼습니다. 진짜 프로젝트는
모듈 하나보다 큽니다. 이 가이드는 날씨 보고서를 모듈 세 개로 만듭니다 —
데이터를 불러오는 모듈, 분석하는 모듈, 출력하는 모듈. 각 모듈은 일 하나만
하고, 주 프로그램은 필요한 이름만 가져옵니다.

## 단계

1. 프로젝트에는 먼저 데이터가 필요합니다. `weather.json`을 만듭니다 —
   `day`와 `temp`(기온)가 든 하루짜리 기록들의 목록:

   ```text
   [
     {"day": "Mon", "temp": 21},
     {"day": "Tue", "temp": 25},
     {"day": "Wed", "temp": 18},
     {"day": "Thu", "temp": 24},
     {"day": "Fri", "temp": 27}
   ]
   ```

2. `fetch_ko.nme`은 입력 모듈입니다. 파일을 읽는 일만 합니다.
   [14](14-json.ko.md)의 `파일 사용` 모듈이 읽기를 담당합니다:

   ```text
   # fetch_ko.nme — 날씨 데이터를 불러옵니다.

   파일 사용 최신

   def 날씨불러오기():
       return json읽기("weather.json")
   ```

   모듈의 인터페이스는 이름 하나, `날씨불러오기`입니다.

3. `analyze_ko.nme`은 계산 담당입니다. `평균`은 기온을 더하고 나눕니다
   ([54](54-stats.ko.md)와 같습니다), `가장더운날`은 `if`로 날들을 훑으며
   가장 따뜻한 날을 기억합니다:

   ```text
   # analyze_ko.nme — 평균과 가장 더운 날을 계산합니다.

   def 평균(기온들):
       합계 = 0
       for t in 기온들:
           합계 = 합계 + t
       return 합계 / len(기온들)

   def 가장더운날(날들):
       제일 = 날들[0]
       for 날 in 날들:
           if 날["temp"] > 제일["temp"]:
               제일 = 날
       return 제일
   ```

4. `report_ko.nme`은 출력 모듈입니다. 보고서를 출력하는 방법만 알고, 다른
   것은 모릅니다 — 파일 읽기도, 계산도 없습니다:

   ```text
   # report_ko.nme — 날씨 보고서를 출력합니다.

   def 보고(날들, 평균기온, 제일):
       말해 f"날씨 보고서: {len(날들)}일"
       말해 f"평균: {평균기온:.1f}C"
       말해 f"가장 더운 날: {제일['day']} {제일['temp']}C"
   ```

5. `main.ko.nme`이 프로젝트를 묶습니다. 가져오기 줄이 모듈마다 인터페이스를
   나열합니다 — `from "fetch_ko.nme" import 날씨불러오기`, 그다음 분석 함수
   둘, 그다음 보고서 함수. 주 프로그램이 순서를 정합니다: 불러오고, 기온을
   모으고, 계산하고, 출력합니다. 세 모듈 옆에 저장합니다:

   ```text
   # main.ko.nme — 날씨 보고서 프로젝트.
   # 실행: nme r main.ko
   # weather.json이 같은 폴더에 있어야 합니다.

   from "fetch_ko.nme" import 날씨불러오기
   from "analyze_ko.nme" import 평균, 가장더운날
   from "report_ko.nme" import 보고

   날들 = 날씨불러오기()

   기온들 = []
   for 날 in 날들:
       기온들.append(날["temp"])

   평균기온 = 평균(기온들)
   제일 = 가장더운날(날들)
   보고(날들, 평균기온, 제일)
   ```

   `날들`은 불러온 목록이고, `기온들`은 기온의 열이고, `평균기온`은 평균,
   `제일`은 가장 따뜻한 기록입니다 — 각 값은 정확히 한 모듈에 속하며,
   main은 인터페이스 사이에서 값들을 옮길 뿐입니다.

6. 데이터 파일이 있는 상태에서 주 프로그램을 실행합니다:

   ```sh
   nme r main.ko
   ```

   ```text
   날씨 보고서: 5일
   평균: 23.0C
   가장 더운 날: Fri 27C
   ```

   21, 25, 18, 24, 27의 평균은 23.0이고, 27도의 금요일이 가장 더운 날입니다.

7. 모듈 경계를 넘는 것은 가져온 이름뿐입니다. `json읽기`는 `fetch_ko.nme`의
   `파일 사용 최신` 덕분에 그 모듈 안에 살지만, main은 그것을 전혀 볼 수
   없습니다 — main이 쓸 수 있는 것은 가져오기 목록의 이름뿐입니다. 모듈
   자신의 도우미도 같습니다. `analyze_ko.nme`에 `def _화씨(t)` 같은 비공개
   함수를 추가해도 그대로 비공개로 남습니다. 가져오기 목록이 곧
   인터페이스이므로, 모듈 내부를 바꿔도 가져오는 프로그램은 절대 깨지지
   않습니다.

8. 영어는 같은 프로젝트를 `use file latest`, `json_load`, `show`와
   `fetch.nme` 같은 이름으로 씁니다. 영어 파일 네 개는
   [영어 가이드](72-project-files.md)에 있습니다.

## 직접 해보기

`analyze_ko.nme`에 `가장추운날(날들)`을 추가하고 main에서 가져와 보고서에
가장 추운 날도 출력해 보세요. 힌트: `가장더운날`과 같은 반복이지만 비교를
`<`로 바꿉니다. 그런 다음 `report_ko.nme`에 도시 이름을 출력하는 머리 줄을
넣어 보세요.

## 배운 것

- 프로젝트는 일별로 모듈을 나눕니다 — 입력, 계산, 출력.
- 각 모듈은 작은 인터페이스, 즉 가져오기 목록의 이름들을 내보냅니다.
- `from "fetch_ko.nme" import 날씨불러오기`가 이름 하나만 넘겨 줍니다.
- 주 프로그램은 모듈 인터페이스 사이에서 값들을 옮길 뿐입니다.
