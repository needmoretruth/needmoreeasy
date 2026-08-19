# 72 — 날씨: 웹에서 JSON 읽기

[English](72-weather.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★★ (5/5)
- 선수 지식: [71 — HTTP](71-http.ko.md), [58 — 데이터](58-data.ko.md)
- 주제: 인터넷
- 결과물: 로컬 HTTP 서버의 JSON을 가져와 미니 날씨 보고서 출력하기

[71](71-http.ko.md)은 HTTP로 텍스트 파일을 가져왔고, [58](58-data.ko.md)은
디스크의 JSON을 불러왔습니다. 이 가이드는 HTTP로 JSON을 가져와
분석하고 미니 날씨 보고서를 출력합니다.

## 단계

1. `weather.json`을 만듭니다 — 다른 딕셔너리(`current`)와 딕셔너리 목록
   (`forecast`)을 담는 딕셔너리입니다:

   ```nme
   {
     "city": "Seoul",
     "current": {"temperature": 21, "condition": "sunny", "humidity": 55},
     "forecast": [
       {"day": "Mon", "temperature": 22},
       {"day": "Tue", "temperature": 19},
       {"day": "Wed", "temperature": 24}
     ]
   }
   ```

2. Python에 내장된 HTTP 서버로 폴더를 한 터미널에서 서비스합니다:

   ```sh
   python3 -m http.server 8000
   ```

   `weather.json`이 이제 `http://localhost:8000/weather.json`에 있습니다.

3. 프로그램 전체입니다. `weather.ko.nme`로 저장합니다:

   ```nme
   # weather.ko.nme — 로컬 웹 서버에서 읽는 미니 날씨 보고서
   # 실행: nme 실행 weather.ko — 먼저 이 폴더를 서비스하세요: python3 -m http.server 8000

   import urllib.request
   import json

   url = "http://localhost:8000/weather.json"
   with urllib.request.urlopen(url) as response:
       body = response.read().decode("utf-8")
   data = json.loads(body)

   도시 = data["city"]
   지금 = data["current"]
   전망 = data["forecast"]
   말해 f"{도시}의 날씨"
   말해 f"  지금: {지금['temperature']} C, {지금['condition']}"
   말해 f"  습도: {지금['humidity']}%"
   말해 "전망:"
   for day in 전망:
       말해 f"  {day['day']}: {day['temperature']} C"

   가장더운 = 전망[0]
   for day in 전망:
       if day["temperature"] > 가장더운["temperature"]:
           가장더운 = day
   합계 = 0
   for day in 전망:
       합계 = 합계 + day["temperature"]
   평균 = 합계 / len(전망)
   말해 f"가장 더운 날: {가장더운['day']} {가장더운['temperature']} C"
   말해 f"전망 평균 기온: {round(평균, 1)} C"
   ```

   `urlopen`이 연결을 열고 `.decode("utf-8")`이 바이트를 문자열로 바꿉니다.
   `json.loads`가 그 글을 딕셔너리로 분석하고, `for` 반복이 전망을 훑어
   가장 더운 날을 찾습니다.

4. 서버가 켜진 상태에서 실행합니다:

   ```sh
   nme 실행 weather.ko
   ```

   ```text
   Seoul의 날씨
     지금: 21 C, sunny
     습도: 55%
   전망:
     Mon: 22 C
     Tue: 19 C
     Wed: 24 C
   가장 더운 날: Wed 24 C
   전망 평균 기온: 21.7 C
   ```

5. 영어는 같은 보고서를 `show`로 씁니다; 전체 프로그램은 [영어 가이드](72-weather.md)에 있습니다.

## 직접 해보기

`weather.json`을 자기 도시와 기온으로 바꾸고 다시 실행해 보세요 — 서버는 요청마다 파일을 읽습니다. `current`에 `"wind"` 숫자를 넣어 보세요.

## 배운 것

- `python3 -m http.server 8000`은 JSON 파일도 텍스트 파일처럼 서비스합니다.
- `json.loads(body)`는 JSON 글을 딕셔너리나 목록으로 바꿉니다.
- JSON 딕셔너리는 딕셔너리와 목록을 담을 수 있고, NME는 똑같이 꺼냅니다.
- 보고서 반복과 `가장더운` 반복이 문서 전체를 한 단계씩 읽습니다.
