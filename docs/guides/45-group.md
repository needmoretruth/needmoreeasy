# 45 — Group: data by category

English | [한국어](45-group.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- Difficulty: ★★★★★ (5/5)
- Prerequisites: [42 — Word count](42-word-count.md), [41 — Records](41-address-book.md)
- Topic: working with data
- Result: grouping a list of dicts by a category key into a dict of lists, then reporting counts per category

Counting (guides [42](42-word-count.md) and [67](67-stats.md)) answers *how
many*; grouping answers *how many per category*: it splits one list of dicts
into lists, one per value of a key.

## Steps

1. Create `data.json`, a list of dicts — each has a `name`, `category`,
   `price`:

   ```nme
   [
     {"name": "Mina", "category": "fruit", "price": 3},
     {"name": "Jun", "category": "veggie", "price": 2},
     {"name": "Sora", "category": "fruit", "price": 5},
     {"name": "Tom", "category": "veggie", "price": 1},
     {"name": "Ari", "category": "fruit", "price": 4}
   ]
   ```

2. Load it with `json_load` from guide [39](39-json.md). The manual group uses
   the `if word in tally` idea from guide [42](42-word-count.md) — a new
   category starts an empty list:

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

3. `setdefault` does the "new key gets an empty list" step in one call, then
   `append` always works — the shorter spelling of the same loop:

   ```nme
   quick = {}
   for item in items:
       quick.setdefault(item["category"], []).append(item)
   ```

4. The full program groups, reports counts and totals, and proves `setdefault`
   agrees. Save it as `group.nme`:

   ```nme
   # group.nme — group a list of dicts by category, then report counts.
   # Run: nme r group

   use file latest
   items = json_load("data.json")
   show "loaded " + str(len(items)) + " items"

   groups = {}
   for item in items:
       cat = item["category"]
       if cat not in groups:
           groups[cat] = []
       groups[cat].append(item)

   show "counts per category:"
   for cat in groups:
       show cat + ": " + str(len(groups[cat])) + " items"

   show "total price per category:"
   for cat in groups:
       total = 0
       for item in groups[cat]:
           total = total + item["price"]
       show f"{cat}: ${total}"

   quick = {}
   for item in items:
       quick.setdefault(item["category"], []).append(item)

   show "setdefault agrees:"
   for cat in quick:
       show f"{cat}: {len(quick[cat])} items"
   ```

   The outer loop walks categories, the inner loop walks each category's items.

5. Run it with `data.json` in the folder:

   ```sh
   nme r group
   ```

   ```text
   loaded 5 items
   counts per category:
   fruit: 3 items
   veggie: 2 items
   total price per category:
   fruit: $12
   veggie: $3
   setdefault agrees:
   fruit: 3 items
   veggie: 2 items
   ```

   Each category is a list, so the report can count it, name it, or add prices.

## Try it yourself

Group by `price` instead of `category` — every price is its own group, showing
which items cost the same.

## What you learned

- Grouping splits one list of dicts into a dict of lists by a key's value.
- `if cat not in groups` starts a new list; `groups[cat].append(item)` fills it.
- `groups.setdefault(cat, []).append(item)` is the same in one call.
- The outer loop walks categories; the inner loop walks each category's items.
