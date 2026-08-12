# 24 — Python 패키지: 표준 라이브러리와 설치된 라이브러리

[English](24-python-packages.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [23 — Modules](23-modules.ko.md), [13 — Files](13-files.ko.md)
- 주제 (Topic): Python 패키지 / Python packages
- 결과물 (Result): 표준 라이브러리와 설치된 라이브러리 사용하기 / using the standard library and installed libraries

Python에는 준비된 패키지가 많이 들어 있습니다. 고급 NME는 일반 Python이므로
그중 아무 패키지나 `.nme` 파일 안에서 쓸 수 있습니다. 그것이 바로 세 번째
문법 계층입니다. 한국어 예제 `examples/birthday.ko.nme`는 `datetime` 패키지로
생일까지 남은 날을 셉니다. 영어판 `examples/birthday.nme`는 같은 계산을 영어
초급 표기로 보여 줍니다.

## 단계

1. 필요한 패키지 부분을 Python import 줄로 가져옵니다:

   ```text
   # examples/birthday.ko.nme의 일부
   from datetime import date

   today = date.today()
   말해 today.year
   ```

   `date`는 이제 다른 값처럼 쓸 수 있습니다: `date(2026, 12, 25)`는 날짜를
   만들고 `today.year`는 연도를 읽습니다.

2. 입력은 초급 문법으로, 계산은 패키지로:

   ```text
   # examples/birthday.ko.nme의 일부
   물어봐 월, "생일 월(1-12): "
   물어봐 일, "생일 일(1-31): "

   today = date.today()
   this_year = date(today.year, int(월), int(일))

   if this_year < today:
       this_year = date(today.year + 1, int(월), int(일))

   말해 "다음 생일까지 " + str((this_year - today).days) + "일 남았어요"
   ```

   실행하고 태어난 월과 일을 입력하세요:

   ```sh
   nme 실행 birthday.ko
   ```

   일수는 오늘 날짜와 입력에 따라 달라집니다:

   ```text
   다음 생일까지 <일수>일 남았어요
   ```

3. 다른 표준 패키지도 같은 방식입니다. `statistics`는 목록의 평균을,
   `collections`는 개수를 셀 수 있고, `json`(`use file` 모듈이 이미 사용)은
   데이터를 읽고 씁니다.

4. 써드파티 라이브러리는 NME의 패키지 명령으로 설치합니다. 이 명령은 Python의
   pip을 감싸며 운영체제에 맞는 일반 Python 명령을 고르고, 한 번에 패키지 하나를
   설치합니다:

   ```sh
   nme 설치 requests
   ```

   영어 명령도 같은 뜻입니다:

   ```sh
   nme install requests
   ```

   이 명령에는 인터넷 연결이 필요합니다. pip이 실패하면 NME가 E9025를
   보여 주므로 패키지 이름, 인터넷 연결, pip 설치 여부를 확인한 뒤 다시
   시도하세요. 설치가 끝나면 같은 방식으로 가져옵니다:

   ```text
   import requests
   ```

   설치된 패키지는 표준 패키지처럼 사용합니다. 패키지 설치 자체는 이 오프라인
   컴파일러의 역할이 아닙니다.

## 직접 해보기

`birthday.nme`를 바꿔 생일 대신 좋아하는 기념일까지 남은 날을 세거나,
`date(2026, 12, 25).strftime("%A")`로 그 날의 요일을 출력해 보세요.

## 배운 것

- `from datetime import date`가 프로그램에 패키지 이름을 가져옵니다.
- 표준 라이브러리는 NME 안에서 언제나 쓸 수 있습니다.
- 초급 `물어봐`와 Python 패키지 호출이 한 파일에서 자유롭게 섞입니다.
- `nme 설치` / `nme install`이 pip을 감싸며, 써드파티 패키지는 똑같이 가져옵니다.
