# 47 — 검색 — JSON에서 찾기

[English](47-search.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★★ (5/5)
- 선수 지식: [41 — 기록](41-address-book.ko.md), [39 — JSON](39-json.ko.md)
- 주제: 검색/데이터
- 결과물: 로컬 서버나 파일에서 JSON 목록을 불러와 대소문자 구분 없이 검색하기

목록은 기록들의 모음입니다 — `name`과 `tags`가 든 딕셔너리. 검색 반복은
키워드를 물어보고 목록을 훑으며 이름이 맞는 기록을 골라냅니다. 이
가이드는 로컬 파일이나 HTTP 서버에서 목록을 불러오고 `.lower()`로
대소문자 구분 없이 글을 맞춥니다.

## 단계

1. 기록 하나는 `name`과 `tags` 목록을 담습니다. 목록 전체는 그런
   딕셔너리들의 모음이고 `catalog.json`으로 저장됩니다:

   ```nme
   [
     {"name": "Red Apple", "tags": ["fruit", "sweet"]},
     {"name": "Red Rose", "tags": ["flower", "garden"]},
     {"name": "Green Tea", "tags": ["drink", "warm"]},
     {"name": "Blueberry", "tags": ["fruit", "blue"]}
   ]
   ```

2. 프로그램 옆 파일에서 목록을 불러오거나, 파일이 없으면 로컬 서버에서
   불러옵니다. `os.path.exists`가 길을 고르고, 서버 부분은
   [76](76-net.ko.md)의 `urlopen` 줄입니다. `json읽기`와 `loads` 모두 같은
   모양을 돌려줍니다 — 딕셔너리들의 목록:

   ```nme
   import os
   파일 사용 최신
   from json import loads
   from urllib.request import urlopen

   if os.path.exists("catalog.json"):
       상품들 = json읽기("catalog.json")
   else:
       url = "http://localhost:8000/catalog.json"
       상품들 = loads(urlopen(url).read().decode("utf-8"))
   ```

3. `"Red" in "Red Apple"`은 대소문자를 구분해서 소문자 검색이 놓칩니다.
   `.lower()`이 문자열을 소문자로 복사하고, 양쪽에 붙이면 대소문자
   구분 없는 매칭이 됩니다. 태그는 목록이라 `word.lower() in 상품["tags"]`이
   태그 목록 전체를 확인합니다 — 문자열과 목록에 쓰는 같은 `in` 연산자.
   `Red Apple`이 두 번 출력됩니다:

   ```nme
   name = "Red Apple"
   word = "red"
   if word.lower() in name.lower():
       말해 name

   상품 = {"name": "Red Apple", "tags": ["fruit", "sweet"]}
   if "sweet" in 상품["tags"]:
       말해 상품["name"]
   ```

4. `found` 카운터가 "없음"을 침묵 대신 진짜 답으로 바꿉니다:

   ```nme
   found = 0
   for 상품 in 상품들:
       if "red" in 상품["name"].lower():
           말해 f"{상품['name']}: {', '.join(상품['tags'])}"
           found = found + 1
   if found == 0:
       말해 "없음"
   ```

5. 프로그램 전체입니다. `catalog.json` 옆에 `search.ko.nme`으로
   저장합니다:

   ```nme
   # search.ko.nme — JSON 목록에서 찾기.
   # 실행: nme 실행 search.ko
   # search, list, quit 중 하나를 입력하세요.

   import os
   파일 사용 최신
   from json import loads
   from urllib.request import urlopen

   # 로컬 파일이나 로컬 서버에서 목록을 불러옵니다.
   if os.path.exists("catalog.json"):
       상품들 = json읽기("catalog.json")
   else:
       url = "http://localhost:8000/catalog.json"
       상품들 = loads(urlopen(url).read().decode("utf-8"))

   말해 f"목록: {len(상품들)}개"
   while True:
       말해 "명령: search, list, quit"
       물어봐 명령, "? "
       if 명령 == "search":
           물어봐 단어, "검색어? "
           found = 0
           for 상품 in 상품들:
               # 양쪽을 소문자로 바꾸면 Red가 red를 찾습니다.
               name = 상품["name"].lower()
               if 단어.lower() in name or 단어.lower() in 상품["tags"]:
                   말해 f"{상품['name']}: {', '.join(상품['tags'])}"
                   found = found + 1
           if found == 0:
               말해 "없음"
       elif 명령 == "list":
           for 상품 in 상품들:
               말해 상품["name"]
       elif 명령 == "quit":
           말해 "안녕!"
           break
       else:
           말해 "알 수 없는 명령"
   ```

   검색은 이름과 태그를 모두 확인하므로 `red`는 이름으로 `Red Apple`과
   `Red Rose`를 찾고, `blue`는 태그를 통해 `Blueberry`를 찾습니다.

6. 파이프로 명령을 넣어 실행합니다. `search`가 `red`를 찾고, `list`가
   모든 이름을 출력하며, `quit`이 반복을 끝냅니다:

   ```sh
   printf 'search\nred\nlist\nquit\n' | nme 실행 search.ko
   ```

   ```text
   목록: 4개
   명령: search, list, quit
   ? 검색어? Red Apple: fruit, sweet
   Red Rose: flower, garden
   명령: search, list, quit
   ? Red Apple
   Red Rose
   Green Tea
   Blueberry
   명령: search, list, quit
   ? 안녕!
   ```

   맞는 게 없는 키워드는 `없음`을 출력합니다. 서버 경로를 쓰려면 폴더에서
   `python3 -m http.server 8000`을 시작하고 `catalog.json` 이름을 바꾼 뒤
   실행하세요 — 같은 프로그램이 HTTP로 똑같은 목록을 가져옵니다.

## 직접 해보기

이름 검사를 빼고 `단어.lower()`를 각 상품의 태그 목록에만 맞춰 보세요.
또는 반복 뒤에 `f"{found}개 찾음"`을 출력해 보세요.

## 배운 것

- 목록은 `name`과 `tags`가 든 딕셔너리들의 모음입니다.
- `os.path.exists`가 `json읽기`와 `loads(urlopen(...))` 사이를 고릅니다.
- 양쪽 `.lower()`이 `in` 매칭을 대소문자 구분 없이 만듭니다.
- `in`은 문자열(`name`)과 목록(`tags`) 모두에 쓸 수 있습니다.
- `found` 카운터가 빈 반복과 "없음"을 구분하게 해 줍니다.
