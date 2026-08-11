# 60 — Search — finding items in JSON

English | [한국어](60-search.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [31 — Records](31-address-book.md), [55 — Network](55-net.md)
- 주제 (Topic): 검색/데이터 / search & data
- 결과물 (Result): 로컬 서버나 파일에서 JSON 목록을 불러와 대소문자 구분 없이 검색하기 / loading a JSON catalog from a local server or file and searching it by keyword, case-insensitively

A catalog is a list of records — dicts with a `name` and `tags`. A search
loop asks for a keyword, walks the list, and keeps every record whose name
matches. This guide loads the catalog from a local file or an HTTP server and
matches text case-insensitively with `.lower()`.

## Steps

1. One record has a `name` and a `tags` list. The whole catalog is a list of
   those dicts, saved as `catalog.json`:

   ```text
   [
     {"name": "Red Apple", "tags": ["fruit", "sweet"]},
     {"name": "Red Rose", "tags": ["flower", "garden"]},
     {"name": "Green Tea", "tags": ["drink", "warm"]},
     {"name": "Blueberry", "tags": ["fruit", "blue"]}
   ]
   ```

2. Load the catalog from the file next to the program, or from a local server
   when the file is missing. `os.path.exists` picks the path; the server
   branch is the `urlopen` line from guide [55](55-net.md). Both `json_load`
   and `loads` return the same shape — a list of dicts:

   ```text
   import os
   use file latest
   from json import loads
   from urllib.request import urlopen

   if os.path.exists("catalog.json"):
       items = json_load("catalog.json")
   else:
       url = "http://localhost:8000/catalog.json"
       items = loads(urlopen(url).read().decode("utf-8"))
   ```

3. `"Red" in "Red Apple"` is case-sensitive, so a lowercase search would miss
   it. `.lower()` copies a string in lowercase; on both sides it makes the
   match case-insensitive. Tags are a list, so `word.lower() in item["tags"]`
   checks the whole tag list — the same `in` operator on a string and a list.
   It prints `Red Apple` twice:

   ```text
   name = "Red Apple"
   word = "red"
   if word.lower() in name.lower():
       show name

   item = {"name": "Red Apple", "tags": ["fruit", "sweet"]}
   if "sweet" in item["tags"]:
       show item["name"]
   ```

4. A `found` counter turns "no matches" into a real answer instead of silence:

   ```text
   found = 0
   for item in items:
       if "red" in item["name"].lower():
           show f"{item['name']}: {', '.join(item['tags'])}"
           found = found + 1
   if found == 0:
       show "no matches"
   ```

5. The whole program. Save `search.nme` next to `catalog.json`:

   ```text
   # search.nme — find items in a JSON catalog.
   # Run: nme r search
   # Type search, list, or quit.

   import os
   use file latest
   from json import loads
   from urllib.request import urlopen

   # Load the catalog from the local file, or from a local server.
   if os.path.exists("catalog.json"):
       items = json_load("catalog.json")
   else:
       url = "http://localhost:8000/catalog.json"
       items = loads(urlopen(url).read().decode("utf-8"))

   show f"catalog: {len(items)} items"
   while True:
       show "Commands: search, list, quit"
       ask command, "? "
       if command == "search":
           ask word, "Keyword? "
           found = 0
           for item in items:
               # Lowercase both sides, so Red finds red.
               name = item["name"].lower()
               if word.lower() in name or word.lower() in item["tags"]:
                   show f"{item['name']}: {', '.join(item['tags'])}"
                   found = found + 1
           if found == 0:
               show "no matches"
       elif command == "list":
           for item in items:
               show item["name"]
       elif command == "quit":
           show "Bye!"
           break
       else:
           show "Unknown command"
   ```

   The search checks the name and the tags, so `red` finds `Red Apple` and
   `Red Rose` by name, and `blue` finds `Blueberry` through its tag.

6. Run it and feed the commands through a pipe. `search` looks up `red`,
   `list` prints every name, and `quit` leaves the loop:

   ```sh
   printf 'search\nred\nlist\nquit\n' | nme r search
   ```

   ```text
   catalog: 4 items
   Commands: search, list, quit
   ? Keyword? Red Apple: fruit, sweet
   Red Rose: flower, garden
   Commands: search, list, quit
   ? Red Apple
   Red Rose
   Green Tea
   Blueberry
   Commands: search, list, quit
   ? Bye!
   ```

   A keyword with no matches prints `no matches`. To use the server branch,
   start `python3 -m http.server 8000` in the folder, rename `catalog.json`
   away, and the same program fetches the identical list over HTTP.

## Try it yourself

Search the tags only — drop the name check and match `word.lower()` against
each item's tag list. Or print `f"{found} matches"` after the loop.

## What you learned

- A catalog is a list of dicts, each with a `name` and `tags`.
- `os.path.exists` chooses between `json_load` and `loads(urlopen(...))`.
- `.lower()` on both sides makes an `in` match case-insensitive.
- `in` works on both strings (`name`) and lists (`tags`).
- A `found` counter distinguishes no matches from an empty loop.
