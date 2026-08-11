# 24 — Python 패키지: 표준 라이브러리와 설치된 라이브러리

[English](24-python-packages.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [23 — Modules](23-modules.ko.md), [13 — Files](13-files.ko.md)
- 주제 (Topic): Python 패키지 / Python packages
- 결과물 (Result): 표준 라이브러리와 설치된 라이브러리 사용하기 / using the standard library and installed libraries

Python에는 준비된 패키지가 많이 들어 있습니다. 고급 NME는 일반 Python이므로
그중 아무 패키지나 `.nme` 파일 안에서 쓸 수 있습니다. 그것이 바로 세 번째
문법 계층입니다. `examples/birthday.nme`는 `datetime` 패키지로 생일까지
남은 날을 셉니다.

## 단계

1. 필요한 패키지 부분을 Python import 줄로 가져옵니다:

   ```text
   # part of examples/birthday.nme
   from datetime import date

   today = date.today()
   show today.year
   ```

   `date`는 이제 다른 값처럼 쓸 수 있습니다: `date(2026, 12, 25)`는 날짜를
   만들고 `today.year`는 연도를 읽습니다.

2. 입력은 초급 문법으로, 계산은 패키지로:

   ```text
   # part of examples/birthday.nme
   ask month, "your birth month (1-12): "
   ask day, "your birth day (1-31): "

   today = date.today()
   this_year = date(today.year, int(month), int(day))

   if this_year < today:
       this_year = date(today.year + 1, int(month), int(day))

   show "your next birthday is in " + str((this_year - today).days) + " days"
   ```

   실행하고 태어난 월과 일을 입력하세요:

   ```sh
   nme run birthday
   ```

   ```text
   your next birthday is in 136 days
   ```

3. 다른 표준 패키지도 같은 방식입니다. `statistics`는 목록의 평균을,
   `collections`는 개수를 셀 수 있고, `json`(`use file` 모듈이 이미 사용)은
   데이터를 읽고 씁니다.

4. 써드파티 라이브러리는 먼저 pip로 설치한 뒤 똑같이 가져옵니다. 명령은
   운영체제마다 다르지만([설치 안내](../install.ko.md) 참고) NME 코드는 언제나
   같습니다:

   ```sh
   python3 -m pip install requests
   ```

   ```text
   import requests
   ```

   설치된 패키지는 표준 패키지처럼 사용합니다. 패키지 설치는 인터넷이
   필요하며 이 오프라인 컴파일러의 역할이 아닙니다.

## 직접 해보기

`birthday.nme`를 바꿔 생일 대신 좋아하는 기념일까지 남은 날을 세거나,
`date(2026, 12, 25).strftime("%A")`로 그 날의 요일을 출력해 보세요.

## 배운 것

- `from datetime import date`가 프로그램에 패키지 이름을 가져옵니다.
- 표준 라이브러리는 NME 안에서 언제나 쓸 수 있습니다.
- 초급 `ask`와 Python 패키지 호출이 한 파일에서 자유롭게 섞입니다.
- 써드파티 패키지는 pip로 설치하고 똑같이 가져옵니다.
