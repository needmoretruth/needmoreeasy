# -*- coding: utf-8 -*-
"""Batch 3: matched en/ko pairs that pin down the asymmetries precisely."""
from probes import (P_HELLO, P_ANNYEONG, SLEEP2, RANGE3, JUMSU0,
                    BREAKP, CONTP, IFSCORE, IFJUMSU, SET_EN, SET_KO)

PROBES3 = []


def P(pid, family, lang, src, intent, expect=None):
    PROBES3.append(dict(id=pid, family=family, lang=lang, src=src,
                        intent=intent, expect=expect))


# --- N. output-verb typo, verb-first vs verb-last, both languages --------
P("n2-en-01", "typopos", "en", "say hello", "print hello", P_HELLO)
P("n2-en-02", "typopos", "en", "sya hello", "print hello", P_HELLO)
P("n2-en-03", "typopos", "en", "hello say", "print hello", P_HELLO)
P("n2-en-04", "typopos", "en", "hello sya", "print hello", P_HELLO)
P("n2-en-05", "typopos", "en", "hello shwo", "print hello", P_HELLO)
P("n2-en-06", "typopos", "en", "hello sayy", "print hello", P_HELLO)
P("n2-en-07", "typopos", "en", "hello dispaly", "print hello", P_HELLO)
P("n2-ko-01", "typopos", "ko", "말해줘 안녕", "print 안녕", P_ANNYEONG)
P("n2-ko-02", "typopos", "ko", "말해조 안녕", "print 안녕", P_ANNYEONG)
P("n2-ko-03", "typopos", "ko", "안녕 말해줘", "print 안녕", P_ANNYEONG)
P("n2-ko-04", "typopos", "ko", "안녕 말해조", "print 안녕", P_ANNYEONG)
P("n2-ko-05", "typopos", "ko", "안녕 마해줘", "print 안녕", P_ANNYEONG)
P("n2-ko-06", "typopos", "ko", "안녕 말해쥐", "print 안녕", P_ANNYEONG)
P("n2-ko-07", "typopos", "ko", "안녕 보여저", "print 안녕", P_ANNYEONG)

# --- O. the -줘 politeness suffix across every action --------------------
P("o2-ko-01", "jwo", "ko", "안녕 말해줘", "print", P_ANNYEONG)
P("o2-ko-02", "jwo", "ko", "2초 기다려줘", "sleep", SLEEP2)
P("o2-ko-03", "jwo", "ko", "친구들은 목록 민수\n친구들에 지안 넣어줘", "append", r'친구들\.append\("지안"\)')
P("o2-ko-04", "jwo", "ko", "3번 반복해줘\n  안녕 말해줘\n끝", "loop", RANGE3)
P("o2-ko-05", "jwo", "ko", "3번 반복해\n  건너뛰어줘\n끝", "continue", CONTP)
P("o2-ko-06", "jwo", "ko", "3번 반복해\n  멈춰줘\n끝", "break", BREAKP)
P("o2-ko-07", "jwo", "ko", "3번 반복해\n  넘어가줘\n끝", "continue", CONTP)
P("o2-ko-08", "jwo", "ko", SET_KO + "점수에 1 더해줘", "add", r'점수 = 점수 \+ 1')
P("o2-ko-09", "jwo", "ko", "점수 저장해줘 0", "set", JUMSU0)

# --- P. Korean condition connectors after an operator --------------------
P("p2-ko-01", "cond2", "ko", SET_KO + "만약 점수 > 10: 성공 말해줘", "if", IFJUMSU)
P("p2-ko-02", "cond2", "ko", SET_KO + "만약 점수 > 10 이면 성공 말해줘", "if", IFJUMSU)
P("p2-ko-03", "cond2", "ko", SET_KO + "만약 점수 > 10 면 성공 말해줘", "if", IFJUMSU)
P("p2-ko-04", "cond2", "ko", SET_KO + "만약 점수 > 10 그러면 성공 말해줘", "if", IFJUMSU)
P("p2-ko-05", "cond2", "ko", SET_KO + "점수 > 10 이면 성공 말해줘", "if", IFJUMSU)
P("p2-ko-06", "cond2", "ko", SET_KO + "만약 점수 > 10\n  성공 말해줘\n끝", "if block", IFJUMSU)
P("p2-en-01", "cond2", "en", SET_EN + "if score > 10 then show won", "if", IFSCORE)
P("p2-en-02", "cond2", "en", SET_EN + "if score > 10\n  show won\nend", "if block", IFSCORE)

# --- Q. filler word position --------------------------------------------
P("q-ko-01", "filler2", "ko", "좀 안녕 말해줘", "print", P_ANNYEONG)
P("q-ko-02", "filler2", "ko", "안녕 좀 말해줘", "print", P_ANNYEONG)
P("q-ko-03", "filler2", "ko", "제발 2초 기다려", "sleep", SLEEP2)
P("q-ko-04", "filler2", "ko", "혹시 안녕 말해줘", "print", P_ANNYEONG)
P("q-en-01", "filler2", "en", "please say hello", "print", P_HELLO)
P("q-en-02", "filler2", "en", "please wait 2 seconds", "sleep", SLEEP2)

# --- R. write/read as output words, en vs ko -----------------------------
P("r-ko-01", "writeword", "ko", "써줘 안녕", "print", P_ANNYEONG)
P("r-ko-02", "writeword", "ko", "적어줘 안녕", "print", P_ANNYEONG)
P("r-ko-03", "writeword", "ko", "읽어줘 안녕", "print", P_ANNYEONG)
P("r-ko-04", "writeword", "ko", "안녕 써줘", "print", P_ANNYEONG)
P("r-en-01", "writeword", "en", "write hello", "print", P_HELLO)
P("r-en-02", "writeword", "en", "read hello", "print", P_HELLO)
P("r-en-03", "writeword", "en", "hello write", "print", P_HELLO)

# --- S. repeat-unit word, en vs ko ---------------------------------------
P("s2-en-01", "unit", "en", "repeat 3 times and show hello", "loop", RANGE3)
P("s2-en-02", "unit", "en", "repeat 3 time and show hello", "loop", RANGE3)
P("s2-en-03", "unit", "en", "repeat 3 loops and show hello", "loop", RANGE3)
P("s2-ko-01", "unit", "ko", "3번 반복해서 안녕 말해줘", "loop", RANGE3)
P("s2-ko-02", "unit", "ko", "3회 반복해서 안녕 말해줘", "loop", RANGE3)
P("s2-ko-03", "unit", "ko", "3차례 반복해서 안녕 말해줘", "loop", RANGE3)
P("s2-ko-04", "unit", "ko", "3판 반복해서 안녕 말해줘", "loop", RANGE3)

# --- T. joined words, ko ---------------------------------------------------
P("t2-ko-01", "joined", "ko", "2초기다려", "sleep", SLEEP2)
P("t2-ko-02", "joined", "ko", "3번반복해서 안녕 말해줘", "loop", RANGE3)
P("t2-ko-03", "joined", "ko", "3번 반복해\n  여기서멈춰\n끝", "break", BREAKP)
P("t2-ko-04", "joined", "ko", "3번 반복해\n  건너뛰어\n끝", "continue", CONTP)
P("t2-ko-05", "joined", "ko", "점수는0", "set", JUMSU0)
P("t2-en-01", "joined", "en", "wait2 seconds", "sleep", SLEEP2)
P("t2-en-02", "joined", "en", "wait 2seconds", "sleep", SLEEP2)
P("t2-en-03", "joined", "en", "repeat 3times and show hello", "loop", RANGE3)
P("t2-en-04", "joined", "en", "repeat3 times and show hello", "loop", RANGE3)
