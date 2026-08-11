# 61 — Project — a mini bank

English | [한국어](61-bank.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [43 — Habit](43-habit.md), [48 — Shop](48-shop.md)
- 주제 (Topic): 프로젝트 / a project
- 결과물 (Result): 입금·출금·잔액·거래 내역을 모듈로 저장하는 JSON 은행 계좌 / a JSON-persisted bank account with deposit, withdraw, balance, history, and a storage module

Guide [48](48-shop.md) kept a shop's money in a dict; guide
[43](43-habit.md) put the storage logic in a module. A bank account is the
same pair one step further: a dict with a `balance` and a `history` list of
every transaction, saved to `account.json` by a `bank.nme` module.

## Steps

1. An account is one dict with two parts: `balance` starts at 0, and
   `history` is a list that grows with every deposit or withdrawal. It prints
   `100` and `['+100']`:

   ```text
   account = {"balance": 0, "history": []}
   account["balance"] = account["balance"] + 100
   account["history"].append("+100")
   show account["balance"]
   show account["history"]
   ```

   Each transaction is one string in the history list, with a `+` for deposits
   and a `-` for withdrawals.

2. Storage lives in a module, `bank.nme`, exporting `load()` and `save`.
   `load()` returns a fresh account when no file exists yet — the same pattern
   as the store module in guide [48](48-shop.md):

   ```text
   # bank.nme — file storage for the mini bank.

   import os
   use file latest

   def load():
       if os.path.exists("account.json"):
           return json_load("account.json")
       return {"balance": 0, "history": []}

   def save(account):
       json_save("account.json", account)
   ```

   `json_load` returns the saved dict, so `balance` and `history` come back
   exactly as they were written.

3. The whole bank. Save `account.nme` next to `bank.nme`:

   ```text
   # account.nme — a mini bank kept in a JSON file.
   # Run: nme r account
   # Type deposit, withdraw, balance, history, or quit.

   from "bank.nme" import load, save
   account = load()

   show "Mini bank — balance is kept in account.json"
   while True:
       show "Commands: deposit, withdraw, balance, history, quit"
       ask command, "? "
       if command == "deposit":
           ask amount_text, "Amount? "
           amount = int(amount_text)
           account["balance"] = account["balance"] + amount
           account["history"].append(f"+{amount}")
           save(account)
           show f"Deposited {amount}"
       elif command == "withdraw":
           ask amount_text, "Amount? "
           amount = int(amount_text)
           if amount <= account["balance"]:
               account["balance"] = account["balance"] - amount
               account["history"].append(f"-{amount}")
               save(account)
               show f"Withdrew {amount}"
           else:
               show "Not enough money"
       elif command == "balance":
           show f"Balance: {account['balance']}"
       elif command == "history":
           show f"{len(account['history'])} transactions"
           for entry in account["history"]:
               show entry
       elif command == "quit":
           show "Bye!"
           break
       else:
           show "Unknown command"
   ```

   `deposit` adds to the balance and records `+amount`; `withdraw` checks the
   balance first and records `-amount`. Both call `save`, so every change is
   written to `account.json` immediately. `history` walks the list the way
   `list` did in guide [31](31-address-book.md).

4. Run it and feed the commands through a pipe. `deposit 100` then
   `withdraw 30` leaves 70, and `history` shows both transactions:

   ```sh
   printf 'deposit\n100\nwithdraw\n30\nbalance\nhistory\nquit\n' | nme r account
   ```

   ```text
   Mini bank — balance is kept in account.json
   Commands: deposit, withdraw, balance, history, quit
   ? Amount? Deposited 100
   Commands: deposit, withdraw, balance, history, quit
   ? Amount? Withdrew 30
   Commands: deposit, withdraw, balance, history, quit
   ? Balance: 70
   Commands: deposit, withdraw, balance, history, quit
   ? 2 transactions
   +100
   -30
   Commands: deposit, withdraw, balance, history, quit
   ? Bye!
   ```

   Look at `account.json` — it now holds the whole state:

   ```text
   {"balance": 70, "history": ["+100", "-30"]}
   ```

   Withdrawing more than the balance prints `Not enough money` and saves
   nothing, so the account can never go negative:

   ```sh
   printf 'withdraw\n500\nbalance\nquit\n' | nme r account
   ```

   ```text
   Mini bank — balance is kept in account.json
   Commands: deposit, withdraw, balance, history, quit
   ? Amount? Not enough money
   Commands: deposit, withdraw, balance, history, quit
   ? Balance: 70
   Commands: deposit, withdraw, balance, history, quit
   ? Bye!
   ```

## Try it yourself

Add a `transfer` command that withdraws from this account and deposits into a
second one — load both, change both, save both. Or refuse deposits of zero or
negative amounts with an `if amount <= 0:` check.

## What you learned

- An account is a dict of `{balance, history}`; `history` is a list of strings.
- `load()` / `save()` in `bank.nme` keep the file format in one module.
- `withdraw` checks `amount <= account["balance"]` before spending.
- Every change calls `save`, so the account survives between runs.
