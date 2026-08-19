#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Run every probe in probes.py through the NME CLI and record the outcome.

Writes results.tsv (one row per probe) and results.json (full detail incl.
the generated Python), both into this script's own directory.
"""
import json
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from probes import PROBES  # noqa: E402
from probes2 import PROBES2  # noqa: E402
from probes3 import PROBES3  # noqa: E402
from probes4 import PROBES4  # noqa: E402
PROBES = PROBES + PROBES2 + PROBES3 + PROBES4

ROOT = os.path.dirname(os.path.dirname(HERE))
NME = (sys.argv[1] if len(sys.argv) > 1
       else os.path.join(ROOT, "target", "release", "nme"))
WORK = os.path.join(HERE, "probes")
os.makedirs(WORK, exist_ok=True)

ERRLINE = re.compile(r"^error\[(E\d+)\]:\s*(.*)$", re.M)


def run(probe):
    src = probe["src"]
    if not src.endswith("\n"):
        src += "\n"
    base = probe["id"].replace("/", "_")
    nme_path = os.path.join(WORK, base + ".nme")
    py_path = os.path.join(WORK, base + ".py")
    with open(nme_path, "w", encoding="utf-8") as fh:
        fh.write(src)
    if os.path.exists(py_path):
        os.remove(py_path)

    chk = subprocess.run([NME, "check", nme_path], capture_output=True, text=True)
    out = (chk.stdout or "") + (chk.stderr or "")
    accepted = chk.returncode == 0
    code, msg, first = "", "", ""
    m = ERRLINE.search(out)
    if m:
        code, msg = m.group(1), m.group(2).strip()
        first = m.group(0).strip()
    hint = ""
    hm = re.search(r"^\s*=\s*hint:\s*(.*)$", out, re.M)
    if hm:
        hint = hm.group(1).strip()

    python = ""
    if accepted:
        bld = subprocess.run([NME, "build", nme_path, "-o", py_path],
                             capture_output=True, text=True)
        if bld.returncode == 0 and os.path.exists(py_path):
            with open(py_path, encoding="utf-8") as fh:
                python = fh.read()
        else:
            python = "<<build failed: %s>>" % ((bld.stdout or "") + (bld.stderr or "")).strip()

    semantic = ""
    if accepted and probe.get("expect"):
        semantic = "ok" if re.search(probe["expect"], python, re.M | re.I) else "MISCOMPILE"
    elif accepted:
        semantic = "n/a"

    return dict(probe, accepted=accepted, code=code, msg=msg,
                first_diag=first, hint=hint, python=python, semantic=semantic,
                raw=out.strip())


def esc(s):
    return s.replace("\\", "\\\\").replace("\n", "\\n").replace("\t", "\\t")


def main():
    rows = [run(p) for p in PROBES]

    with open(os.path.join(HERE, "results.tsv"), "w", encoding="utf-8") as fh:
        fh.write("id\tfamily\tlang\tverdict\tcode\tsemantic\tsource\tdiagnostic\thint\tpython\tintent\n")
        for r in rows:
            fh.write("\t".join([
                r["id"], r["family"], r["lang"],
                "accepted" if r["accepted"] else "rejected",
                r["code"] or "-", r["semantic"] or "-",
                esc(r["src"]), esc(r["first_diag"]), esc(r["hint"]),
                esc(r["python"].strip()), r["intent"],
            ]) + "\n")

    with open(os.path.join(HERE, "results.json"), "w", encoding="utf-8") as fh:
        json.dump(rows, fh, ensure_ascii=False, indent=1)

    n = len(rows)
    acc = sum(1 for r in rows if r["accepted"])
    mis = sum(1 for r in rows if r["semantic"] == "MISCOMPILE")
    print("probes=%d accepted=%d rejected=%d miscompile=%d" % (n, acc, n - acc, mis))


if __name__ == "__main__":
    main()
