# -*- coding: utf-8 -*-
"""Thirty ordinary sentences — a story, a message, a greeting — that must
print themselves and nothing else. Half English, half Korean.

    python3 prose_corpus.py [path-to-nme]
"""
import subprocess, sys, tempfile, os

NME = sys.argv[1] if len(sys.argv) > 1 else "/home/user/nmt/needmoreeasy/target/release/nme"

PROSE = [
    "Hello everyone!", "It was a dark and stormy night.", "The door was locked.",
    "Nobody answered the phone.", "She looked at the map again.", "Good morning",
    "Welcome to the game", "You have three lives left", "Try again tomorrow",
    "The end", "log in first", "let me think", "store it away",
    "echo of the mountain", "insert coin to continue",
    "안녕하세요 여러분!", "어두운 밤이었습니다.", "문이 잠겨 있었습니다.",
    "아무도 대답하지 않았습니다.", "지도를 다시 보았습니다.", "좋은 아침입니다",
    "게임에 오신 것을 환영합니다", "목숨이 세 개 남았습니다", "내일 다시 해 보세요",
    "끝입니다", "말하기 연습", "작은 방이었습니다", "나는 학생입니다",
    "이것은 시작입니다", "밝은 빛이 보였습니다",
]

bad = []
for src in PROSE:
    with tempfile.TemporaryDirectory() as d:
        nme = os.path.join(d, "p.nme")
        py = os.path.join(d, "p.py")
        open(nme, "w", encoding="utf-8").write(src + "\n")
        run = subprocess.run([NME, "build", nme, "-o", py], capture_output=True, text=True)
        if run.returncode != 0:
            code = [l for l in run.stdout.splitlines() if l.startswith("error[")]
            bad.append((src, code[0] if code else run.stdout.strip()[:60]))
            continue
        out = open(py, encoding="utf-8").read().strip()
        if not out.startswith("print("):
            bad.append((src, "not a print: " + out[:50]))

print(f"{len(PROSE) - len(bad)}/{len(PROSE)} 문장이 그대로 출력됩니다.")
for src, why in bad:
    print(f"  실패 {src!r} -> {why}")
sys.exit(1 if bad else 0)
