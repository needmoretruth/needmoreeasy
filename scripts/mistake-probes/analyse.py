#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Aggregate results.json: counts, silent mis-compiles, error-code census."""
import json
import re
from collections import Counter, defaultdict

import os
HERE = os.path.dirname(os.path.abspath(__file__))
rows = json.load(open(os.path.join(HERE, 'results.json'), encoding='utf-8'))

BARE = re.compile(r'^\s*([^\W\d]\w*)\s*$', re.U)
ANNOT = re.compile(r'^\s*([^\W\d]\w*)\s*:\s*([^\W\d]\w*)\s*$', re.U)
OK_BARE = {'pass', 'break', 'continue', 'quit', 'exit'}

print("=" * 78)
print("TOTALS")
n = len(rows)
acc = sum(1 for r in rows if r['accepted'])
print(f"probes={n}  accepted={acc} ({acc*100//n}%)  rejected={n-acc}")

print()
print("PER FAMILY x LANG   (acc/total, miscompiles among accepted)")
tab = defaultdict(lambda: [0, 0, 0])
for r in rows:
    k = (r['family'], r['lang'])
    tab[k][1] += 1
    if r['accepted']:
        tab[k][0] += 1
        if r['semantic'] == 'MISCOMPILE':
            tab[k][2] += 1
for k in sorted(tab):
    a, t, m = tab[k]
    print(f"  {k[0]:11} {k[1]:3}  accepted {a:3}/{t:<3}  rejected {t-a:3}  miscompiled {m:3}")

print()
print("PER LANG")
for lang in ('en', 'ko'):
    sub = [r for r in rows if r['lang'] == lang]
    a = sum(1 for r in sub if r['accepted'])
    m = sum(1 for r in sub if r['semantic'] == 'MISCOMPILE')
    print(f"  {lang}: probes {len(sub)}  accepted {a}  rejected {len(sub)-a}  miscompiled {m}")

print()
print("ERROR CODE CENSUS (rejections)")
for code, c in Counter(r['code'] for r in rows if not r['accepted']).most_common():
    ex = next(r for r in rows if r['code'] == code and not r['accepted'])
    print(f"  {code}  x{c:<3}  {ex['msg'][:70]}")

print()
print("=" * 78)
print("SILENT MIS-COMPILES  (accepted but wrong)")
buckets = defaultdict(list)
for r in rows:
    if not r['accepted']:
        continue
    py = r['python']
    lines = [l for l in py.splitlines() if l.strip()]
    kind = None
    for l in lines:
        m = ANNOT.match(l)
        if m and m.group(1) not in ('for', 'if', 'while', 'else'):
            kind = 'no-op annotation'
            break
        m = BARE.match(l)
        if m and m.group(1) not in OK_BARE:
            kind = 'bare name -> NameError'
            break
    if kind is None and r['semantic'] == 'MISCOMPILE':
        if re.search(r'range\((?![\d)])', py):
            kind = 'range(<undefined word>) -> NameError'
        elif re.search(r'print\(', py):
            kind = 'silently printed the command'
        elif re.search(r'= "', py):
            kind = 'value became a string'
        else:
            kind = 'other wrong Python'
    if kind:
        buckets[kind].append(r)

for kind in sorted(buckets, key=lambda k: -len(buckets[k])):
    rs = buckets[kind]
    print(f"\n--- {kind}  ({len(rs)} probes) ---")
    for r in rs:
        src = r['src'].replace('\n', ' / ').strip()
        py = r['python'].strip().replace('\n', ' / ')
        print(f"  [{r['id']:9} {r['lang']}] {src}\n      -> {py}")

tot = sum(len(v) for v in buckets.values())
print(f"\nTOTAL silent mis-compiles: {tot}")
