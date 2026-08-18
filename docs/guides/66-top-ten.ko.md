# 66 — 상위 열: 기록 순위

[English](66-top-ten.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★★ (5/5)
- 선수 지식: [39 — 정렬](39-sorting.ko.md), [54 — 통계](54-stats.ko.md)
- 주제: 데이터/순위
- 결과물: JSON 기록을 불러와 숫자 점수 기준으로 정렬하고 상위 열 개를 보여 주는 프로그램

[39](39-sorting.ko.md)는 숫자를 정렬하고, [54](54-stats.ko.md)는 그 숫자를
요약했습니다. 실제 데이터는 기록들의 목록이고, 기록 하나는 여러 필드가 든
딕셔너리입니다. 순위를 매긴다는 것은 한 필드 — 점수 — 로 정렬한다는 뜻입니다.
이 가이드는 기록을 JSON에서 불러와 `sorted(..., key=...)`로 점수 순으로
정렬하고, 슬라이스로 상위 열 개를 보여 줍니다.

## 단계

1. 기록이 열두 개 이상인 `records.json`을 만듭니다. 기록 하나는 `name`과
   숫자 `score`가 든 딕셔너리입니다:

   ```text
   [
     {"name": "Mina", "score": 88},
     {"name": "Jun", "score": 95},
     {"name": "Sora", "score": 72},
     {"name": "Ravi", "score": 91},
     {"name": "Lena", "score": 84},
     {"name": "Tom", "score": 67},
     {"name": "Aya", "score": 79},
     {"name": "Ben", "score": 93},
     {"name": "Kim", "score": 58},
     {"name": "Nia", "score": 86},
     {"name": "Leo", "score": 74},
     {"name": "Sam", "score": 90}
   ]
   ```

2. [14](14-json.ko.md)의 `json읽기`로 목록을 불러옵니다:

   ```text
   파일 사용 최신
   기록들 = json읽기("records.json")
   말해 f"기록 {len(기록들)}개를 불러왔습니다"
   ```

   `기록 12개를 불러왔습니다`가 출력됩니다.

3. `sorted`는 어떤 필드로 비교할지 알아야 합니다. 딕셔너리 전체에는
   자연스러운 순서가 없기 때문입니다. `key` 인자가 그 필드를 정해 줍니다:
   `lambda r: r["score"]`는 기록 `r`을 받아 그 `score`를 돌려주는 아주
   작은 함수입니다. `reverse=True`가 높은 점수를 먼저 둡니다:

   ```text
   records = [{"name": "Mina", "score": 88}, {"name": "Jun", "score": 95}]
   top = sorted(records, key=lambda r: r["score"], reverse=True)
   show top
   ```

   ```text
   [{'name': 'Jun', 'score': 95}, {'name': 'Mina', 'score': 88}]
   ```

   95가 88보다 크므로 Jun이 1위입니다 — 이름이 아니라 점수로 비교했습니다.

4. `[:10]`은 목록의 첫 열 개를 남깁니다 — 상위 열입니다. 슬라이스는 숫자든
   기록이든 똑같이 됩니다:

   ```text
   scores = [5, 2, 9, 1, 7, 3]
   top = sorted(scores, reverse=True)[:3]
   show top
   ```

   ```text
   [9, 7, 5]
   ```

5. 전체 프로그램은 기록을 불러와 점수 순으로 정렬하고 순위 번호와 함께 상위
   열 개를 출력하며, 간발의 차로 들지 못한 기록도 보여 줍니다.
   `top-ten.ko.nme`로 저장합니다:

   ```text
   # top-ten.ko.nme — records.json에서 가장 높은 열 개 점수.
   # 실행: nme 실행 top-ten.ko
   # records.json 파일이 같은 폴더에 있어야 합니다.

   파일 사용 최신

   기록들 = json읽기("records.json")
   말해 f"기록 {len(기록들)}개를 불러왔습니다"

   상위 = sorted(기록들, key=lambda r: r["score"], reverse=True)[:10]

   말해 "상위 열:"
   순위 = 1
   for r in 상위:
       말해 f"  {순위}. {r['name']}: {r['score']}"
       순위 = 순위 + 1

   전체순위 = sorted(기록들, key=lambda r: r["score"], reverse=True)
   아깝게 = 전체순위[10]
   말해 f"아깝게 탈락: {아깝게['name']}: {아깝게['score']}"
   ```

   `순위` 카운터는 1에서 시작해 반복 안에서 커지므로 1번 줄이 최고 점수,
   10번 줄이 열 개 중 가장 낮은 점수입니다. `전체순위[10]`은 열한 번째
   항목을 읽습니다 — 인덱스 10은 0부터 세므로 상위 열 개 밖의 첫
   기록입니다.

6. 데이터 파일이 있는 상태에서 실행합니다:

   ```sh
   nme 실행 top-ten.ko
   ```

   ```text
   기록 12개를 불러왔습니다
   상위 열:
     1. Jun: 95
     2. Ben: 93
     3. Ravi: 91
     4. Sam: 90
     5. Mina: 88
     6. Nia: 86
     7. Lena: 84
     8. Aya: 79
     9. Leo: 74
     10. Sora: 72
   아깝게 탈락: Tom: 67
   ```

   Tom(67)은 열한 번째로 높은 점수이고 Kim(58)은 열두 번째입니다.
   `sorted`가 열두 개를 점수 순으로 정렬하고 `[:10]`이 마지막 둘을
   버렸습니다.

7. 영어는 같은 단계를 `use file latest`, `json_load`, `show`로 씁니다.
   전체 영어 프로그램은 [영어 가이드](66-top-ten.md)에 있습니다.

## 직접 해보기

높은 점수의 열세 번째 기록을 추가하고 다시 실행해 보세요 — 열 개 중 가장
약한 기록이 빠지고 새 기록이 나타납니다. 또는 `[:5]`로 상위 다섯을 보거나
`reverse=True`를 `False`로 바꿔 하위 열 개를 보세요.

## 배운 것

- 기록은 딕셔너리고, `sorted`는 한 필드를 비교할 `key`가 필요합니다.
- `lambda r: r["score"]`는 기록의 점수를 돌려주는 아주 작은 함수입니다.
- `sorted(기록들, key=..., reverse=True)`는 높은 점수를 먼저 두는 순위를
  만듭니다.
- `[:10]`이 정렬된 목록을 상위 열 개로 잘라냅니다.
