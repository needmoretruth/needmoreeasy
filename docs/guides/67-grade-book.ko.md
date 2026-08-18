# 67 — 프로젝트 — 성적부

[English](67-grade-book.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★★ (5/5)
- 선수 지식: [61 — 미니 은행](61-bank.ko.md), [48 — 상점](48-shop.ko.md)
- 주제: 프로젝트
- 결과물: 학생 추가·성적 추가·평균 보고를 저장 모듈과 함께 두는 JSON 성적부

[61](61-bank.ko.md)의 은행은 저장 모듈로 딕셔너리를 보관했고,
[48](48-shop.ko.md)의 상점은 명령 메뉴를 돌렸습니다. 성적부는 둘을 합친
것입니다: 학생 이름을 성적 목록에 연결하는 딕셔너리 하나, 그것을 불러오고
저장하는 `gradebook_ko.nme` 모듈, 그리고 `add`, `grade`, `report`, `quit`
메뉴. [54](54-stats.ko.md)의 `statistics.mean`이 성적 목록을 평균으로
바꿉니다.

## 단계

1. 성적부는 딕셔너리 하나입니다: 학생 이름마다 성적 목록이 연결됩니다.
   `statistics.mean`이 그 목록을 평균합니다:

   ```text
   from statistics import mean
   books = {"Mina": [88, 90], "Jun": [95]}
   show mean(books["Mina"])
   ```

   `89`가 출력됩니다 — 88과 90의 평균입니다.

2. 저장은 모듈에 둡니다. [61](61-bank.ko.md)의 은행 모듈처럼
   `gradebook_ko.nme`이 `load()`와 `save`를 내보냅니다. `load()`는 파일이
   없으면 빈 딕셔너리를 돌려줍니다:

   ```text
   # gradebook_ko.nme — 성적부의 파일 저장 모듈.

   import os
   파일 사용 최신

   def load():
       if os.path.exists("gradebook.json"):
           return json읽기("gradebook.json")
       return {}

   def save(성적부):
       json저장("gradebook.json", 성적부)
   ```

3. 프로그램 전체입니다. `gradebook_ko.nme` 옆에 `grade-book.ko.nme`으로
   저장합니다:

   ```text
   # grade-book.ko.nme — JSON에 보관하는 성적부.
   # 실행: nme 실행 grade-book.ko
   # add, grade, report, quit 중 하나를 입력하세요.

   from "gradebook_ko.nme" import load, save
   from statistics import mean

   성적부 = load()

   말해 "성적부 — gradebook.json에 보관"
   while True:
       말해 "명령: add, grade, report, quit"
       물어봐 명령, "? "
       if 명령 == "add":
           물어봐 이름, "이름? "
           if 이름 in 성적부:
               말해 "이미 추가된 학생"
           else:
               성적부[이름] = []
               save(성적부)
               말해 f"{이름} 추가"
       elif 명령 == "grade":
           물어봐 이름, "이름? "
           if 이름 in 성적부:
               물어봐 점수문자, "점수? "
               점수 = int(점수문자)
               성적부[이름].append(점수)
               save(성적부)
               말해 f"{이름}에게 {점수} 추가"
           else:
               말해 "그런 학생 없음"
       elif 명령 == "report":
           물어봐 이름, "이름? "
           if 이름 in 성적부:
               점수들 = 성적부[이름]
               if len(점수들) > 0:
                   말해 f"{이름}: 평균 {mean(점수들):.1f}, 성적 {len(점수들)}개"
               else:
                   말해 f"{이름}: 아직 성적 없음"
           else:
               말해 "그런 학생 없음"
       elif 명령 == "quit":
           말해 "안녕!"
           break
       else:
           말해 "알 수 없는 명령"
   ```

   `add`는 학생이 새 학생인지 확인한 뒤 빈 성적 목록을 만들고 저장합니다.
   `grade`는 학생이 있는지 확인하고 점수 하나를 더하고 저장합니다.
   `report`는 그 학생의 성적을 평균하고 개수를 보여 주며, 아직 없으면
   그렇게 말합니다. 변경마다 `save`가 실행되어 성적부가 실행 사이에도
   남습니다.

4. 파이프로 명령을 넣어 실행합니다:

   ```sh
   printf 'add\nMina\nreport\nMina\ngrade\nMina\n90\nreport\nMina\nquit\n' | nme 실행 grade-book.ko
   ```

   ```text
   성적부 — gradebook.json에 보관
   명령: add, grade, report, quit
   ? 이름? Mina 추가
   명령: add, grade, report, quit
   ? 이름? Mina: 아직 성적 없음
   명령: add, grade, report, quit
   ? 이름? 점수? Mina에게 90 추가
   명령: add, grade, report, quit
   ? 이름? Mina: 평균 90.0, 성적 1개
   명령: add, grade, report, quit
   ? 안녕!
   ```

   성적이 없을 때의 보고는 `아직 성적 없음`입니다. 성적 하나 뒤에는
   `:.1f` 형식으로 평균 90을 소수 한 자리까지 보여 줍니다.

5. `gradebook.json`을 열어 보면 성적부 전체가 들어 있습니다:

   ```text
   {"Mina": [90]}
   ```

   Mina의 성적을 하나 더 추가하면(`grade` 다음 `Mina` 다음 `100`) 파일이
   `{"Mina": [90, 100]}`이 되고 평균은 95.0이 됩니다.

## 직접 해보기

모든 학생의 평균을 보고하는 `list` 명령을 추가해 보세요. 또는 [66](66-top-ten.ko.md)의
`sorted(..., key=...)` 요령으로 학생들을 평균 순으로 정렬하는 `top` 명령을
추가해 보세요. `report`가 `max(점수들)`로 최고 성적도 출력하게 해도
좋습니다.

## 배운 것

- 성적부는 이름을 성적 목록에 연결하는 딕셔너리고, JSON으로 저장됩니다.
- `gradebook_ko.nme`의 `load()` / `save`가 파일 형식을 한 모듈에 둡니다.
- `statistics.mean(점수들)`이 성적 목록을 평균으로 바꿉니다.
- `add`, `grade`, `report`, `quit` 명령이 `while True:` 메뉴를 움직입니다.
