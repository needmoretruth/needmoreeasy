# 48 — Shop — an inventory store

English | [한국어](48-shop.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★☆ (4/5)
- Prerequisites: [33 — Todo](33-todo.md), [31 — Records](31-address-book.md)
- Topic: a project
- Result: a JSON-persisted store with buy/sell/stock/list and a money balance

Guide [33](33-todo.md) saved a list and [31](31-address-book.md) saved
records. A store grows that idea one step: a dict of items (each with `price`
and `stock`) plus a `money` balance, all in `shop.json`.

## Steps

1. The shop is one JSON object with two parts: an `items` dict mapping a name
   to `{"price": N, "stock": N}`, plus a `money` balance:

   ```nme
   {
     "items": {
       "apple": {"price": 3, "stock": 10},
       "banana": {"price": 5, "stock": 5},
       "cherry": {"price": 2, "stock": 0}
     },
     "money": 20
   }
   ```

2. The whole store. Save `shop.nme` next to `shop.json`. `use file latest`
   loads `json_load`/`json_save`, and `os.path.exists` starts a fresh shop:

   ```nme
   # shop.nme — a small store kept in a JSON file.
   # Run: nme r shop
   # Type list, buy, sell, or quit.

   use file latest
   import os

   if os.path.exists("shop.json"):
       data = json_load("shop.json")
   else:
       data = {"items": {}, "money": 20}

   while True:
       show f"Money: {data['money']}"
       show "Commands: list, buy, sell, quit"
       ask command, "? "
       if command == "list":
           for name in data["items"]:
               item = data["items"][name]
               show f"{name}: {item['price']} each, {item['stock']} in stock"
       elif command == "buy" or command == "sell":
           ask name, "Item? "
           if name in data["items"]:
               item = data["items"][name]
               if command == "buy":
                   if item["stock"] > 0:
                       item["stock"] = item["stock"] - 1
                       data["money"] = data["money"] - item["price"]
                       show f"Bought {name}"
                       json_save("shop.json", data)
                   else:
                       show "Out of stock"
               else:
                   item["stock"] = item["stock"] + 1
                   data["money"] = data["money"] + item["price"]
                   show f"Sold {name}"
                   json_save("shop.json", data)
           else:
               show "No such item"
       elif command == "quit":
           show "Bye!"
           break
       else:
           show "Unknown command"
   ```

   `list` walks the dict with `for name in data["items"]:`; `buy` and `sell`
   share one branch via `or` (guide [09](09-and-or.md)). Buy pays the price
   and drops stock; sell refunds and adds; `json_save` writes it back, so the
   store survives between runs.

3. Run it and feed the commands through a pipe. `buy apple` pays 3 and drops
   stock to 9, `sell apple` refunds it:

   ```sh
   printf 'list\nbuy\napple\nsell\napple\nquit\n' | nme r shop
   ```

   ```text
   Money: 20
   Commands: list, buy, sell, quit
   ? apple: 3 each, 10 in stock
   banana: 5 each, 5 in stock
   cherry: 2 each, 0 in stock
   Money: 20
   Commands: list, buy, sell, quit
   ? Item? Bought apple
   Money: 17
   Commands: list, buy, sell, quit
   ? Item? Sold apple
   Money: 20
   Commands: list, buy, sell, quit
   ? Bye!
   ```

## Try it yourself

Add a `restock <name> <count>` command that adds to an item's stock, or an
`add <name> <price>` command that inserts a new item and saves it.

## What you learned

- A store is a dict of item dicts plus a money balance, all saved as JSON.
- `for name in data["items"]:` lists a dict's keys and reads each item.
- `buy`/`sell` change the balance and the stock, then `json_save` persists.
- A `quit` command with `break` ends the `while True:` menu.
