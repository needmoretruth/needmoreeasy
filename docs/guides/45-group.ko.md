# 45 — 묶기: 범주별 데이터

[English](45-group.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★★ (5/5)
- 선수 지식: [42 — 단어 세기](42-word-count.ko.md), [41 — 기록](41-address-book.ko.md)
- 주제: 데이터
- 결과물: 범주 키로 dict 목록을 목록들의 dict로 묶고, 범주별 개수를 보고하기

세기(가이드 [42](42-word-count.ko.md), [67](67-stats.ko.md))는 *몇 개인지* 답합니다.
묶기는 *범주별로 몇 개인지* 답합니다: dict 목록 하나를 키 값 하나당 하나의
목록들로 나눕니다.

## 단계

1. dict 목록인 `data.json`을 만드세요 — 각 항목은 `name`, `category`,
   `price`를 가집니다:

   ```nme
   [
     {"name": "Mina", "category": "fruit", "price": 3},
     {"name": "Jun", "category": "veggie", "price": 2},
     {"name": "Sora", "category": "fruit", "price": 5},
     {"name": "Tom", "category": "veggie", "price": 1},
     {"name": "Ari", "category": "fruit", "price": 4}
   ]
   ```

2. 가이드 [39](39-json.ko.md)의 `json_load`로 불러옵니다. 손으로 묶는 법은
   가이드 [42](42-word-count.ko.md)의 `if word in tally` 생각입니다 — 새 범주는
   빈 목록부터 시작합니다:

   ```nme
   use file latest
   items = json_load("data.json")
   groups = {}
   for item in items:
       cat = item["category"]
       if cat not in groups:
           groups[cat] = []
       groups[cat].append(item)
   ```

3. `setdefault`는 "새 키는 빈 목록부터" 단계를 한 번에 합니다. 그다음
   `append`는 언제나 동작합니다 — 같은 반복의 짧은 표기입니다:

   ```nme
   quick = {}
   for item in items:
       quick.setdefault(item["category"], []).append(item)
   ```

4. 전체 프로그램은 묶고, 개수와 총 가격을 보고하며, `setdefault`가 같다고
   보여 줍니다. `group.ko.nme`로 저장하세요:

   ```nme
   # group.ko.nme — 범주별로 묶고 개수를 보고하는 프로그램.
   # 실행: nme 실행 group.ko

   use file latest
   items = json_load("data.json")
   말해 "항목 " + str(len(items)) + "개를 불러왔습니다"

   groups = {}
   for item in items:
       cat = item["category"]
       if cat not in groups:
           groups[cat] = []
       groups[cat].append(item)

   말해 "범주별 개수:"
   for cat in groups:
       말해 cat + ": " + str(len(groups[cat])) + "개"

   말해 "범주별 총 가격:"
   for cat in groups:
       total = 0
       for item in groups[cat]:
           total = total + item["price"]
       말해 f"{cat}: ${total}"

   quick = {}
   for item in items:
       quick.setdefault(item["category"], []).append(item)

   말해 "setdefault로 만든 묶음:"
   for cat in quick:
       말해 f"{cat}: {len(quick[cat])}개"
   ```

   바깥 반복이 범주를, 안쪽 반복이 각 범주 목록의 항목들을 돌아봅니다.

5. 같은 폴더에 `data.json`을 두고 실행하세요:

   ```sh
   nme 실행 group.ko
   ```

   ```text
   항목 5개를 불러왔습니다
   범주별 개수:
   fruit: 3개
   veggie: 2개
   범주별 총 가격:
   fruit: $12
   veggie: $3
   setdefault로 만든 묶음:
   fruit: 3개
   veggie: 2개
   ```

   이제 각 범주가 목록이므로, 보고서가 그 개수를 세거나 이름을 나열하거나
   가격을 더할 수 있습니다.

## 직접 해보기

`category` 대신 `price`로 묶어 보세요 — 가격마다 자기 묶음이 생겨 어떤
항목이 같은 가격인지 보여 줍니다.

## 배운 것

- 묶기는 하나의 dict 목록을 키 값별 dict의 목록으로 나눕니다.
- `if cat not in groups`가 새 목록을 시작하고 `groups[cat].append(item)`이
  채웁니다.
- `groups.setdefault(cat, []).append(item)`은 같은 일을 한 번에 합니다.
- 바깥 반복이 범주를, 안쪽 반복이 각 범주의 항목들을 돌아봅니다.
