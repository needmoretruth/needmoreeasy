# 48 — 상점 — 재고 관리

[English](48-shop.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★☆ (4/5)
- 선수 지식: [33 — 할 일](33-todo.ko.md), [31 — 기록](31-address-book.ko.md)
- 주제: 프로젝트
- 결과물: 사고팔고 재고를 확인하며 잔액이 남는 JSON 저장 상점

[33](33-todo.ko.md)은 목록을 저장했고, [31](31-address-book.ko.md)은
기록을 저장했습니다. 상점은 그 생각을 한 단계 키운 것입니다: 각 상품이
`price`와 `stock`을 가진 상품들의 딕셔너리, 그리고 `money` 잔액. 전부
`shop.json`에 담깁니다.

## 단계

1. 상점은 두 부분으로 된 JSON 객체 하나입니다: 이름을
   `{"price": N, "stock": N}`에 연결하는 `items` 딕셔너리, 그리고
   `money` 잔액:

   ```text
   {
     "items": {
       "apple": {"price": 3, "stock": 10},
       "banana": {"price": 5, "stock": 5},
       "cherry": {"price": 2, "stock": 0}
     },
     "money": 20
   }
   ```

2. 상점 전체입니다. `shop.json` 옆에 `shop.ko.nme`을 저장합니다.
   `파일 사용 최신`이 `json읽기`/`json저장`을 불러오고, `os.path.exists`가
   첫 실행에 새 상점을 만듭니다:

   ```text
   # shop.ko.nme — JSON 파일에 보관하는 작은 상점.
   # 실행: nme 실행 shop.ko
   # list, buy, sell, quit 중 하나를 입력하세요.

   파일 사용 최신
   import os

   if os.path.exists("shop.json"):
       데이터 = json읽기("shop.json")
   else:
       데이터 = {"items": {}, "money": 20}

   while True:
       말해 f"돈: {데이터['money']}"
       말해 "명령: list, buy, sell, quit"
       물어봐 명령, "? "
       if 명령 == "list":
           for 이름 in 데이터["items"]:
               상품 = 데이터["items"][이름]
               말해 f"{이름}: {상품['price']}원, 재고 {상품['stock']}개"
       elif 명령 == "buy" or 명령 == "sell":
           물어봐 이름, "상품? "
           if 이름 in 데이터["items"]:
               상품 = 데이터["items"][이름]
               if 명령 == "buy":
                   if 상품["stock"] > 0:
                       상품["stock"] = 상품["stock"] - 1
                       데이터["money"] = 데이터["money"] - 상품["price"]
                       말해 f"{이름} 구매"
                       json저장("shop.json", 데이터)
                   else:
                       말해 "재고 없음"
               else:
                   상품["stock"] = 상품["stock"] + 1
                   데이터["money"] = 데이터["money"] + 상품["price"]
                   말해 f"{이름} 판매"
                   json저장("shop.json", 데이터)
           else:
               말해 "그런 상품 없음"
       elif 명령 == "quit":
           말해 "안녕!"
           break
       else:
           말해 "알 수 없는 명령"
   ```

   `list`는 `for 이름 in 데이터["items"]:`으로 딕셔너리를 지나갑니다.
   `buy`와 `sell`은 `or`([09](09-and-or.ko.md))로 한 갈래를 공유합니다.
   buy는 가격을 내고 재고를 줄이며, sell은 돌려주고 더하며, `json저장`이
   다시 씁니다 — 상점이 실행 사이에도 남습니다.

3. 파이프로 명령을 넣어 실행합니다. `buy apple`은 3원을 내고 재고를 9로
   줄이며, `sell apple`은 돌려 줍니다:

   ```sh
   printf 'list\nbuy\napple\nsell\napple\nquit\n' | nme 실행 shop.ko
   ```

   ```text
   돈: 20
   명령: list, buy, sell, quit
   ? apple: 3원, 재고 10개
   banana: 5원, 재고 5개
   cherry: 2원, 재고 0개
   돈: 20
   명령: list, buy, sell, quit
   ? 상품? apple 구매
   돈: 17
   명령: list, buy, sell, quit
   ? 상품? apple 판매
   돈: 20
   명령: list, buy, sell, quit
   ? 안녕!
   ```

## 직접 해보기

상품 재고에 수를 더하는 `restock <name> <count>` 명령을 추가해 보세요.
또는 새 상품을 넣고 저장하는 `add <name> <price>` 명령을 추가해 보세요.

## 배운 것

- 상점은 상품 딕셔너리들의 딕셔너리에 돈 잔액을 더한 것이고, 전부 JSON으로
  저장됩니다.
- `for 이름 in 데이터["items"]:`이 키를 나열하고 그 상품을 읽습니다.
- `buy`/`sell`이 잔액과 재고를 바꾸고 `json저장`이 보관합니다.
- `quit` 명령과 `break`가 `while True:` 메뉴를 끝냅니다.
