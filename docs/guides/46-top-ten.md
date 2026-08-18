# 46 — Top ten: ranking records

English | [한국어](46-top-ten.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [41 — Records](41-address-book.md), [45 — Group](45-group.md)
- Topic: data & ranking
- Result: loading JSON records, sorting by a numeric score with sorted(..., key=...), and showing the top ten

Guide [52](52-sorting.md) sorts plain numbers and guide [67](67-stats.md)
summarizes them. Real data is a list of records, each a dict with several
fields; ranking them means sorting by one field — a score. This guide loads
records from JSON, sorts them by score with `sorted(..., key=...)`, and shows
the top ten with a slice.

## Steps

1. Create `records.json` with at least twelve records. Each record is a dict
   with a `name` and a numeric `score`:

   ```nme
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

2. Load the list with `json_load` from guide [39](39-json.md):

   ```nme
   use file latest
   records = json_load("records.json")
   show f"loaded {len(records)} records"
   ```

   It prints `loaded 12 records`.

3. `sorted` needs to know which field to compare, because a whole dict has no
   natural order. The `key` argument supplies that field: `lambda r:
   r["score"]` is a tiny function that takes a record `r` and returns its
   `score`. `reverse=True` puts the highest score first:

   ```nme
   records = [{"name": "Mina", "score": 88}, {"name": "Jun", "score": 95}]
   top = sorted(records, key=lambda r: r["score"], reverse=True)
   show top
   ```

   ```text
   [{'name': 'Jun', 'score': 95}, {'name': 'Mina', 'score': 88}]
   ```

   Jun ranks first because 95 beats 88 — the dicts are compared by score, not
   by name.

4. `[:10]` keeps the first ten entries of a list — the top ten. Slicing works
   on numbers and records alike:

   ```nme
   scores = [5, 2, 9, 1, 7, 3]
   top = sorted(scores, reverse=True)[:3]
   show top
   ```

   ```text
   [9, 7, 5]
   ```

5. The full program loads the records, sorts them by score, prints the top ten
   with a rank number, and shows who barely missed the cut. Save
   `top-ten.nme`:

   ```nme
   # top-ten.nme — the ten highest scores from records.json.
   # Run: nme r top-ten
   # The file records.json must exist in the same folder.

   use file latest

   records = json_load("records.json")
   show f"loaded {len(records)} records"

   top = sorted(records, key=lambda r: r["score"], reverse=True)[:10]

   show "top ten:"
   rank = 1
   for r in top:
       show f"  {rank}. {r['name']}: {r['score']}"
       rank = rank + 1

   all_ranked = sorted(records, key=lambda r: r["score"], reverse=True)
   just_missed = all_ranked[10]
   show f"just missed: {just_missed['name']}: {just_missed['score']}"
   ```

   The `rank` counter starts at 1 and grows inside the loop, so line 1 is the
   best score and line 10 the lowest of the ten. `all_ranked[10]` reads the
   eleventh entry — index 10 counts from 0, so it is the first record outside
   the top ten.

6. Run it with the data file present:

   ```sh
   nme r top-ten
   ```

   ```text
   loaded 12 records
   top ten:
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
   just missed: Tom: 67
   ```

   Tom (67) is the 11th-highest score and Kim (58) the 12th. `sorted` ordered
   all twelve by score; `[:10]` dropped the last two.

7. Korean writes the same steps with `파일 사용 최신`, `json읽기`, and `말해`;
   the full Korean program is in the [Korean guide](46-top-ten.ko.md).

## Try it yourself

Add a thirteenth record with a high score and rerun — the weakest of the ten
drops off and the new one appears. Or use `[:5]` for a top five, or change
`reverse=True` to `False` to show the bottom ten.

## What you learned

- Records are dicts; `sorted` needs a `key` to compare one field.
- `lambda r: r["score"]` is a tiny function that returns a record's score.
- `sorted(records, key=..., reverse=True)` ranks the highest score first.
- `[:10]` slices the sorted list down to the top ten.
