# -*- coding: utf-8 -*-
"""Batch 2: targeted follow-up probes that isolate the asymmetries and
mechanisms suggested by batch 1."""
from probes import (P_HELLO, P_ANNYEONG, SLEEP2, RANGE3, SCORE0, JUMSU0,
                    SCOREADD, JUMSUADD, ASKNAME, ASKIREUM, BREAKP, CONTP,
                    IFSCORE, IFJUMSU, SET_EN, SET_KO, LIST_EN, LIST_KO)

PROBES2 = []


def P(pid, family, lang, src, intent, expect=None):
    PROBES2.append(dict(id=pid, family=family, lang=lang, src=src,
                        intent=intent, expect=expect))


# --- A. when does a line print itself? (sentence fallback, en vs ko) --------
P("a-en-01", "fallback", "en", "Hello everyone!", "print sentence", r'print\("Hello everyone!"\)')
P("a-en-02", "fallback", "en", "hello there", "print sentence", r'print\("hello there"\)')
P("a-en-03", "fallback", "en", "wibble hello", "print sentence", None)
P("a-en-04", "fallback", "en", "banana hello", "print sentence", None)
P("a-en-05", "fallback", "en", "hello", "one bare word", None)
P("a-ko-01", "fallback", "ko", "오늘도 반가워요!", "print sentence", r'print\("오늘도 반가워요!"\)')
P("a-ko-02", "fallback", "ko", "아무개 안녕", "print sentence", None)
P("a-ko-03", "fallback", "ko", "바나나 안녕", "print sentence", None)
P("a-ko-04", "fallback", "ko", "안녕", "one bare word", None)
P("a-ko-05", "fallback", "ko", "안녕하세요 여러분", "print sentence", r'print\("안녕하세요 여러분"\)')

# --- B. the 저장해 / 저자해 puzzle and set-word coverage --------------------
P("b-ko-01", "setword", "ko", "점수 저장 0", "점수=0", JUMSU0)
P("b-ko-02", "setword", "ko", "점수 저장해 0", "점수=0", JUMSU0)
P("b-ko-03", "setword", "ko", "점수 저장헤 0", "점수=0", JUMSU0)
P("b-ko-04", "setword", "ko", "점수 저자해 0", "점수=0", JUMSU0)
P("b-ko-05", "setword", "ko", "점수 설정해 0", "점수=0", JUMSU0)
P("b-ko-06", "setword", "ko", "점수 기억해 0", "점수=0", JUMSU0)
P("b-ko-07", "setword", "ko", "점수 기어해 0", "점수=0", JUMSU0)
P("b-ko-08", "setword", "ko", "점수는 0으로", "점수=0", JUMSU0)
P("b-ko-09", "setword", "ko", "점수를 0", "점수=0", JUMSU0)
P("b-en-01", "setword", "en", "set score to 0", "score=0", SCORE0)
P("b-en-02", "setword", "en", "set score as 0", "score=0", SCORE0)
P("b-en-03", "setword", "en", "set score = 0", "score=0", SCORE0)
P("b-en-04", "setword", "en", "remember score to 0", "score=0", SCORE0)
P("b-en-05", "setword", "en", "score is 0", "score=0", SCORE0)

# --- C. Korean condition shapes vs English -------------------------------
P("c2-ko-01", "cond", "ko", SET_KO + "만약 점수 > 10: 성공 말해줘", "if", IFJUMSU)
P("c2-ko-02", "cond", "ko", SET_KO + "만약 점수 > 10 성공 말해줘", "if", IFJUMSU)
P("c2-ko-03", "cond", "ko", SET_KO + "만약에 점수가 10보다 크면 성공 말해줘", "if", IFJUMSU)
P("c2-ko-04", "cond", "ko", SET_KO + "점수가 10보다 크면 성공 말해줘", "if", IFJUMSU)
P("c2-ko-05", "cond", "ko", SET_KO + "만약에 점수 > 10 이면 성공 말해줘", "if", IFJUMSU)
P("c2-ko-06", "cond", "ko", SET_KO + "만약에 점수가 10 초과면 성공 말해줘", "if", IFJUMSU)
P("c2-en-01", "cond", "en", SET_EN + "if score > 10: show won", "if", IFSCORE)
P("c2-en-02", "cond", "en", SET_EN + "if score > 10 show won", "if", IFSCORE)
P("c2-en-03", "cond", "en", SET_EN + "if score > 10 then show won", "if", IFSCORE)
P("c2-en-04", "cond", "en", SET_EN + "score > 10 then show won", "if", IFSCORE)
P("c2-en-05", "cond", "en", SET_EN + "if score bigger than 10 then show won", "if", IFSCORE)

# --- D. `for each` case / typo / spacing ---------------------------------
P("d-en-01", "foreach", "en", "for each friend in friends\n  show friend\nend", "for each", r'for friend in friends')
P("d-en-02", "foreach", "en", "For each friend in friends\n  show friend\nend", "for each", r'for friend in friends')
P("d-en-03", "foreach", "en", "FOR EACH friend in friends\n  show friend\nend", "for each", r'for friend in friends')
P("d-en-04", "foreach", "en", "for eahc friend in friends\n  show friend\nend", "for each", r'for friend in friends')
P("d-en-05", "foreach", "en", "foreach friend in friends\n  show friend\nend", "for each", r'for friend in friends')
P("d-en-06", "foreach", "en", "for every friend in friends\n  show friend\nend", "for each", r'for friend in friends')
P("d-ko-01", "foreach", "ko", "친구들의 친구마다 반복해\n  친구 말해줘\n끝", "for each", r'for 친구 in 친구들')
P("d-ko-02", "foreach", "ko", "친구들의 친구마다 반복헤\n  친구 말해줘\n끝", "for each", r'for 친구 in 친구들')
P("d-ko-03", "foreach", "ko", "친구들의 친구 마다 반복해\n  친구 말해줘\n끝", "for each", r'for 친구 in 친구들')
P("d-ko-04", "foreach", "ko", "친구들에서 친구마다 반복해\n  친구 말해줘\n끝", "for each", r'for 친구 in 친구들')

# --- E. trailing punctuation: does en keep it, ko strip it? --------------
P("e-en-01", "trailpunct", "en", "say hello.", "print hello", P_HELLO)
P("e-en-02", "trailpunct", "en", "show hello.", "print hello", P_HELLO)
P("e-en-03", "trailpunct", "en", "hello say.", "print hello", P_HELLO)
P("e-en-04", "trailpunct", "en", "say hello!", "print hello!", r'print\("hello!"\)')
P("e-en-05", "trailpunct", "en", "repeat 3 times.", "loop 3 block", RANGE3)
P("e-ko-01", "trailpunct", "ko", "말해줘 안녕.", "print 안녕", P_ANNYEONG)
P("e-ko-02", "trailpunct", "ko", "안녕 말해줘.", "print 안녕", P_ANNYEONG)
P("e-ko-03", "trailpunct", "ko", "안녕! 말해줘", "print 안녕!", r'print\("안녕!"\)')
P("e-ko-04", "trailpunct", "ko", "3번 반복해.", "loop 3 block", RANGE3)
P("e-ko-05", "trailpunct", "ko", "3번 반복해.\n  안녕 말해줘\n끝", "loop 3 block", RANGE3)
P("e-en-06", "trailpunct", "en", "repeat 3 times.\n  show hello\nend", "loop 3 block", RANGE3)

# --- F. leading / trailing whitespace, en vs ko --------------------------
P("f-en-01", "ws", "en", "  say hello", "print hello", P_HELLO)
P("f-en-02", "ws", "en", "\tsay hello", "print hello", P_HELLO)
P("f-en-03", "ws", "en", "say hello  ", "print hello", r'print\("hello"\)')
P("f-ko-01", "ws", "ko", "  안녕 말해줘", "print 안녕", P_ANNYEONG)
P("f-ko-02", "ws", "ko", "\t안녕 말해줘", "print 안녕", P_ANNYEONG)
P("f-ko-03", "ws", "ko", "안녕 말해줘  ", "print 안녕", r'print\("안녕"\)')

# --- G. politeness filler words -----------------------------------------
P("g-en-01", "filler", "en", "please say hello", "print hello", P_HELLO)
P("g-en-02", "filler", "en", "say hello please", "print hello", P_HELLO)
P("g-en-03", "filler", "en", "wait 2 seconds please", "sleep 2", SLEEP2)
P("g-ko-01", "filler", "ko", "안녕 좀 말해줘", "print 안녕", P_ANNYEONG)
P("g-ko-02", "filler", "ko", "제발 안녕 말해줘", "print 안녕", P_ANNYEONG)
P("g-ko-03", "filler", "ko", "2초만 기다려", "sleep 2", SLEEP2)
P("g-ko-04", "filler", "ko", "2초 좀 기다려", "sleep 2", SLEEP2)

# --- H. append vs add, en vs ko -----------------------------------------
P("h-en-01", "append", "en", LIST_EN + "append Mina to friends", "append", r'friends\.append\("Mina"\)')
P("h-en-02", "append", "en", LIST_EN + "push Mina to friends", "append", r'friends\.append\("Mina"\)')
P("h-en-03", "append", "en", LIST_EN + "put Mina in friends", "append", r'friends\.append\("Mina"\)')
P("h-en-04", "append", "en", LIST_EN + "append Mina into friends", "append", r'friends\.append\("Mina"\)')
P("h-ko-01", "append", "ko", LIST_KO + "친구들에 민수 넣어", "append", r'친구들\.append\("민수"\)')
P("h-ko-02", "append", "ko", LIST_KO + "친구들에 민수 추가해", "append", r'친구들\.append\("민수"\)')
P("h-ko-03", "append", "ko", LIST_KO + "친구들에 민수 붙여", "append", r'친구들\.append\("민수"\)')
P("h-ko-04", "append", "ko", LIST_KO + "민수 친구들에 넣어", "append", r'친구들\.append\("민수"\)')

# --- I. break/skip word coverage inside a loop, en vs ko -----------------
P("i-en-01", "loopctl", "en", "repeat 3 times\n  break\nend", "break", BREAKP)
P("i-en-02", "loopctl", "en", "repeat 3 times\n  break here\nend", "break", BREAKP)
P("i-en-03", "loopctl", "en", "repeat 3 times\n  skip\nend", "continue", CONTP)
P("i-en-04", "loopctl", "en", "repeat 3 times\n  skip this\nend", "continue", CONTP)
P("i-en-05", "loopctl", "en", "repeat 3 times\n  stop\nend", "break", BREAKP)
P("i-ko-01", "loopctl", "ko", "3번 반복해\n  멈춰\n끝", "break", BREAKP)
P("i-ko-02", "loopctl", "ko", "3번 반복해\n  중단\n끝", "break", BREAKP)
P("i-ko-03", "loopctl", "ko", "3번 반복해\n  건너뛰어\n끝", "continue", CONTP)
P("i-ko-04", "loopctl", "ko", "3번 반복해\n  넘어가\n끝", "continue", CONTP)
P("i-ko-05", "loopctl", "ko", "3번 반복해\n  멈춰줘\n끝", "break", BREAKP)

# --- J. does the repeat body keep the verb? ------------------------------
P("j-ko-01", "body", "ko", "3번 반복해서 안녕 말해줘", "loop print 안녕", r'range\(3\): print\("안녕"\)')
P("j-ko-02", "body", "ko", "3번 반복하고 안녕 말해줘", "loop print 안녕", r'range\(3\): print\("안녕"\)')
P("j-ko-03", "body", "ko", "3번 반복한다음 안녕 말해줘", "loop print 안녕", r'range\(3\): print\("안녕"\)')
P("j-ko-04", "body", "ko", "3번 되풀이해서 안녕 말해줘", "loop print 안녕", r'range\(3\): print\("안녕"\)')
P("j-en-01", "body", "en", "repeat 3 times and show hello", "loop print hello", r'range\(3\): print\("hello"\)')
P("j-en-02", "body", "en", "repeat 3 times then show hello", "loop print hello", r'range\(3\): print\("hello"\)')
P("j-en-03", "body", "en", "repeat 3 times, show hello", "loop print hello", r'range\(3\): print\("hello"\)')
P("j-en-04", "body", "en", "loop 3 times and show hello", "loop print hello", r'range\(3\): print\("hello"\)')

# --- K. numeric-word coverage in both languages -------------------------
P("k2-en-01", "numword", "en", "wait 3 seconds", "sleep 3", r'sleep\(3\)')
P("k2-en-02", "numword", "en", "wait three seconds", "sleep 3", r'sleep\(3\)')
P("k2-ko-01", "numword", "ko", "3초 기다려", "sleep 3", r'sleep\(3\)')
P("k2-ko-02", "numword", "ko", "삼초 기다려", "sleep 3", r'sleep\(3\)')
P("k2-ko-03", "numword", "ko", "세 번 반복해\n  안녕 말해줘\n끝", "loop 3", RANGE3)
P("k2-ko-04", "numword", "ko", "다섯 번 반복해서 안녕 말해줘", "loop 5", r'range\(5\)')
P("k2-en-03", "numword", "en", "repeat five times and show hello", "loop 5", r'range\(5\)')

# --- L. ask shapes, en vs ko --------------------------------------------
P("l-en-01", "ask", "en", "ask name What is your name?", "name=input", ASKNAME)
P("l-en-02", "ask", "en", "What is your name?", "name=input", ASKNAME)
P("l-en-03", "ask", "en", "ask for name What is your name?", "name=input", ASKNAME)
P("l-en-04", "ask", "en", "ask the name What is your name?", "name=input", ASKNAME)
P("l-en-05", "ask", "en", "ask number age How old are you?", "age=int(input)", r'age = int\(input\(')
P("l-en-06", "ask", "en", "ask age as a number How old are you?", "age=int(input)", r'age = int\(input\(')
P("l-ko-01", "ask", "ko", "이름을 물어봐 이름이 뭐예요?", "이름=input", ASKIREUM)
P("l-ko-02", "ask", "ko", "이름이 뭐예요?", "이름=input", ASKIREUM)
P("l-ko-03", "ask", "ko", "나이를 숫자로 물어봐 몇 살이에요?", "나이=int(input)", r'나이 = int\(input\(')
P("l-ko-04", "ask", "ko", "나이를 숫자 물어봐 몇 살이에요?", "나이=int(input)", r'나이 = int\(input\(')
P("l-ko-05", "ask", "ko", "이름을 물어봐줘요 이름이 뭐예요?", "이름=input", ASKIREUM)

# --- M. one more typo sweep on the matchers that lack Recover -----------
P("m-en-01", "typo2", "en", LIST_EN + "appendd Mina to friends", "append", r'friends\.append\("Mina"\)')
P("m-en-02", "typo2", "en", LIST_EN + "psuh Mina to friends", "append", r'friends\.append\("Mina"\)')
P("m-en-03", "typo2", "en", "repeat 3 times\n  breakk\nend", "break", BREAKP)
P("m-en-04", "typo2", "en", "repeat 3 times\n  sikp\nend", "continue", CONTP)
P("m-en-05", "typo2", "en", "waitt 3", "sleep 3", r'sleep\(3\)')
P("m-ko-01", "typo2", "ko", LIST_KO + "친구들에 민수 넣어줘", "append", r'친구들\.append\("민수"\)')
P("m-ko-02", "typo2", "ko", LIST_KO + "친구들에 민수 너허", "append", r'친구들\.append\("민수"\)')
P("m-ko-03", "typo2", "ko", "3번 반복해\n  멈춰라\n끝", "break", BREAKP)
P("m-ko-04", "typo2", "ko", "3번 반복해\n  건너뛰기\n끝", "continue", CONTP)
P("m-ko-05", "typo2", "ko", "3초 기다랴", "sleep 3", r'sleep\(3\)')
