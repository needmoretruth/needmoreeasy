# 14 — JSON: save and load data

English | [한국어](14-json.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [13 — Files](13-files.md)
- Topic: json
- Result: a program that saves a name and a score and loads them back

One file can hold a whole record. JSON stores several values together as text,
so a name and a score come back later — even after the program closes.

## Steps

1. Create `record.nme` and save a record:

   ```nme
   use file latest
   record = {"name": "Mina", "score": 7}
   json_save("record.json", record)
   ```

   `json_save` writes the dict into `record.json` as JSON text.

2. Run it and look at the new file:

   ```sh
   nme r record
   cat record.json
   ```

   The file stores the data as `{"name": "Mina", "score": 7}`.

3. Load it back in a fresh program, `load.nme`:

   ```nme
   use file latest
   record = json_load("record.json")
   say f"{record['name']} scored {record['score']}"
   ```

   Run `nme r load`; it prints `Mina scored 7` — the values survived the end
   of the first program.

4. Korean uses `json저장` and `json읽기`:

   ```nme
   파일 사용 최신
   기록 = {"이름": "민수", "점수": 7}
   json저장("기록.json", 기록)
   기록 = json읽기("기록.json")
   말해 f"{기록['이름']}: {기록['점수']}점"
   ```

## Try it yourself

Save a dict with your city and age, then load it back in one program:

```nme
use file latest
me = {"city": "Seoul", "age": 12}
json_save("me.json", me)
back = json_load("me.json")
say f"I live in {back['city']} and I am {back['age']}"
```

## What you learned

- `json_save(path, value)` / `json저장(경로, 값)` writes a dict to a file.
- `json_load(path)` / `json읽기(경로)` reads it back into a dict.
- A JSON file survives after the program ends; another run can load it.
- Values inside the dict use subscripts such as `record["name"]`.
