# 61 — 프로젝트 — 미니 은행

[English](61-bank.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도: ★★★★★ (5/5)
- 선수 지식: [43 — 습관 체크](43-habit.ko.md), [48 — 상점](48-shop.ko.md)
- 주제: 프로젝트
- 결과물: 입금·출금·잔액·거래 내역을 모듈로 저장하는 JSON 은행 계좌

[48](48-shop.ko.md)은 상점의 돈을 딕셔너리에 담았고,
[43](43-habit.ko.md)은 저장 로직을 모듈에 넣었습니다. 은행 계좌는 같은
쌍을 한 단계 더 키운 것입니다: `balance`와 거래마다 자라는 `history`
목록이 든 딕셔너리, `bank_ko.nme` 모듈이 `account.json`에 저장합니다.

## 단계

1. 계좌는 두 부분으로 된 딕셔너리 하나입니다: `balance`는 0으로 시작하고,
   `history`는 입금·출금마다 자라는 목록입니다. `100`과 `['+100']`이
   출력됩니다:

   ```text
   계좌 = {"balance": 0, "history": []}
   계좌["balance"] = 계좌["balance"] + 100
   계좌["history"].append("+100")
   말해 계좌["balance"]
   말해 계좌["history"]
   ```

   거래 하나는 history 목록의 문자열 하나이고, 입금은 `+`, 출금은
   `-`로 시작합니다.

2. 저장은 모듈에 둡니다. `bank_ko.nme`이 `load()`와 `save`를 내보냅니다.
   `load()`는 파일이 없으면 새 계좌를 돌려줍니다 — [48](48-shop.ko.md)의
   상점 모듈과 같은 패턴입니다:

   ```text
   # bank_ko.nme — 미니 은행의 파일 저장 모듈.

   import os
   파일 사용 최신

   def load():
       if os.path.exists("account.json"):
           return json읽기("account.json")
       return {"balance": 0, "history": []}

   def save(계좌):
       json저장("account.json", 계좌)
   ```

   `json읽기`는 저장된 딕셔너리를 돌려주므로 `balance`와 `history`가 쓴
   그대로 돌아옵니다.

3. 은행 전체입니다. `bank_ko.nme` 옆에 `account.ko.nme`으로 저장합니다:

   ```text
   # account.ko.nme — JSON 파일에 보관하는 미니 은행.
   # 실행: nme 실행 account.ko
   # deposit, withdraw, balance, history, quit 중 하나를 입력하세요.

   from "bank_ko.nme" import load, save
   계좌 = load()

   말해 "미니 은행 — 잔액은 account.json에 보관"
   while True:
       말해 "명령: deposit, withdraw, balance, history, quit"
       물어봐 명령, "? "
       if 명령 == "deposit":
           물어봐 금액, "금액? "
           amount = int(금액)
           계좌["balance"] = 계좌["balance"] + amount
           계좌["history"].append(f"+{amount}")
           save(계좌)
           말해 f"{amount} 입금"
       elif 명령 == "withdraw":
           물어봐 금액, "금액? "
           amount = int(금액)
           if amount <= 계좌["balance"]:
               계좌["balance"] = 계좌["balance"] - amount
               계좌["history"].append(f"-{amount}")
               save(계좌)
               말해 f"{amount} 출금"
           else:
               말해 "잔액 부족"
       elif 명령 == "balance":
           말해 f"잔액: {계좌['balance']}"
       elif 명령 == "history":
           말해 f"거래 {len(계좌['history'])}건"
           for 기록 in 계좌["history"]:
               말해 기록
       elif 명령 == "quit":
           말해 "안녕!"
           break
       else:
           말해 "알 수 없는 명령"
   ```

   `deposit`은 잔액에 더하고 `+금액`을 기록하며, `withdraw`는 먼저 잔액을
   확인하고 `-금액`을 기록합니다. 둘 다 `save`를 불러 변경을 즉시
   `account.json`에 씁니다. `history`는 [31](31-address-book.ko.md)의
   `list`처럼 목록을 훑습니다.

4. 파이프로 명령을 넣어 실행합니다. `deposit 100` 다음 `withdraw 30`이
   70을 남기고, `history`가 두 거래를 보여 줍니다:

   ```sh
   printf 'deposit\n100\nwithdraw\n30\nbalance\nhistory\nquit\n' | nme 실행 account.ko
   ```

   ```text
   미니 은행 — 잔액은 account.json에 보관
   명령: deposit, withdraw, balance, history, quit
   ? 금액? 100 입금
   명령: deposit, withdraw, balance, history, quit
   ? 금액? 30 출금
   명령: deposit, withdraw, balance, history, quit
   ? 잔액: 70
   명령: deposit, withdraw, balance, history, quit
   ? 거래 2건
   +100
   -30
   명령: deposit, withdraw, balance, history, quit
   ? 안녕!
   ```

   `account.json`을 열어 보면 전체 상태가 들어 있습니다:

   ```text
   {"balance": 70, "history": ["+100", "-30"]}
   ```

   잔액보다 많이 출금하면 `잔액 부족`이 출력되고 아무것도 저장되지
   않으므로 계좌는 마이너스가 되지 않습니다:

   ```sh
   printf 'withdraw\n500\nbalance\nquit\n' | nme 실행 account.ko
   ```

   ```text
   미니 은행 — 잔액은 account.json에 보관
   명령: deposit, withdraw, balance, history, quit
   ? 금액? 잔액 부족
   명령: deposit, withdraw, balance, history, quit
   ? 잔액: 70
   명령: deposit, withdraw, balance, history, quit
   ? 안녕!
   ```

## 직접 해보기

계좌 하나에서 출금해 다른 계좌에 입금하는 `transfer` 명령을 추가해
보세요 — 둘을 모두 불러오고, 바꾸고, 저장합니다. 또는 `if amount <= 0:`
검사로 0이나 마이너스 입금을 거절해 보세요.

## 배운 것

- 계좌는 `{balance, history}` 딕셔너리고, `history`는 문자열 목록입니다.
- `bank_ko.nme`의 `load()` / `save`가 파일 형식을 한 모듈에 둡니다.
- `withdraw`는 쓰기 전에 `amount <= 계좌["balance"]`을 확인합니다.
- 변경마다 `save`가 실행되어 계좌가 실행 사이에도 남습니다.
