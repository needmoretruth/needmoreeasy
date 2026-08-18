# 54 — 병합: 두 목록 합치기

[English](54-merge.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★★ (5/5)
- 선수 지식: [46 — 상위 열](46-top-ten.ko.md), [41 — 기록](41-address-book.ko.md)
- 주제: 데이터/병합
- 결과물: 두 JSON 목록을 이름 키로 합쳐 결합 보고서 만들기

실제 데이터는 한 파일에만 있지 않습니다. 학교는 학생을 한 목록에, 점수를
다른 목록에 둡니다. 둘을 합치려면 두 번째 목록에서 이름을 찾아야 하는데,
바로 그것이 dict의 역할입니다.

## 단계

1. 데이터 파일 두 개를 만드세요. `students.json`은 학생마다 dict 하나:

   ```nme
   [
     {"name": "Mina", "class": "A"},
     {"name": "Jun", "class": "A"},
     {"name": "Sora", "class": "B"},
     {"name": "Tom", "class": "B"}
   ]
   ```

2. `scores.json`은 같은 이름과 점수:

   ```nme
   [
     {"name": "Mina", "score": 92},
     {"name": "Jun", "score": 88},
     {"name": "Tom", "score": 75}
   ]
   ```

3. 점수로 검색 dict를 만드세요: 이름이 점수로 이어지므로 학생 점수를 찾는
   것이 목록 전체를 훑는 대신 빠른 `[]` 조회 하나가 됩니다:

   ```nme
   scores_by_name = {}
   for record in scores:
       scores_by_name[record["name"]] = record["score"]
   ```

4. 전체 프로그램은 두 목록을 불러와 합치고 결합 보고서를 출력합니다.
   `merge.nme`로 저장하세요:

   ```nme
   # merge.nme — 학생과 점수를 이름으로 합치기.
   # 실행: nme 실행 merge
   # students.json과 scores.json이 같은 폴더에 있어야 합니다.

   use file latest

   students = json_load("students.json")
   scores = json_load("scores.json")

   scores_by_name = {}
   for record in scores:
       scores_by_name[record["name"]] = record["score"]

   말해 f"class report ({len(students)} students):"
   for student in students:
       name = student["name"]
       score = scores_by_name.get(name, 0)
       말해 f"  {name} in class {student['class']}: {score} points"
   ```

   `scores_by_name.get(name, 0)`은 점수를 주고, 아직 점수가 없는 학생은
   `0`을 줍니다 — Sora도 보고서에 나타납니다.

5. 실행하세요:

   ```sh
   nme 실행 merge
   ```

   ```text
   class report (4 students):
     Mina in class A: 92 points
     Jun in class A: 88 points
     Sora in class B: 0 points
     Tom in class B: 75 points
   ```

## 직접 해보기

`scores.json`에 Mina의 점수를 하나 더 추가하고, 조회가 최고 점수를
유지하도록 바꾸거나, 점수에서 `grade` 필드를 만들어 루프에서 추가하세요.

## 배운 것

- dict가 이름을 값으로 바꿔 빠른 조회를 만듭니다.
- 두 목록을 합치는 것은 하나로 검색 dict를 만들고 다른 하나를 도는 것.
- `dict.get(key, 기본값)`은 없는 키에도 멈추지 않습니다.
- 병합은 서로 다른 파일에 있는 데이터를 프로그램이 합치는 방법입니다.
