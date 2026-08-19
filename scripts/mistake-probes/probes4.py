# -*- coding: utf-8 -*-
"""Batch 4: the list and text grammar, and the prose it must not swallow.

Every statement here is gated on a name the program already made a list, so
each probe carries its own `set friends to list of …` / `친구들은 목록 …`
line. The prose probes deliberately do **not**: their whole point is that a
sentence containing one of these words stays a sentence either way.
"""
from probes import LIST_EN, LIST_KO

PROBES4 = []

NUMS_EN = "set scores to list of 3, 1, 2\n"
NUMS_KO = "점수들은 목록 3, 1, 2\n"
NAME_EN = "set greeting to Hello\n"
NAME_KO = "인사는 Hello\n"


def P(pid, family, lang, src, intent, expect=None):
    PROBES4.append(dict(id=pid, family=family, lang=lang, src=src,
                        intent=intent, expect=expect))


# --- AA. an empty list --------------------------------------------------
P("aa-en-01", "emptylist", "en", "set friends to an empty list", "friends = []",
  r'^friends = \[\]$')
P("aa-en-02", "emptylist", "en", "set friends to empty list", "friends = []",
  r'^friends = \[\]$')
P("aa-en-03", "emptylist", "en", "set friends to a blank list", "friends = []",
  r'^friends = \[\]$')
P("aa-ko-01", "emptylist", "ko", "친구들은 빈 목록", "친구들 = []", r'^친구들 = \[\]$')
P("aa-ko-02", "emptylist", "ko", "친구들은 새 목록", "친구들 = []", r'^친구들 = \[\]$')
P("aa-ko-03", "emptylist", "ko", "친구들은 빈 리스트", "친구들 = []", r'^친구들 = \[\]$')
# The words on their own are speech, not a list.
P("aa-en-04", "emptylist", "en", "an empty room", "print the sentence",
  r'print\("an empty room"\)')
P("aa-ko-04", "emptylist", "ko", "빈 방이었습니다", "print the sentence",
  r'print\("빈 방이었습니다"\)')
P("aa-ko-05", "emptylist", "ko", "빈 목록입니다", "print the sentence",
  r'print\("빈 목록입니다"\)')

# --- BB. how many -------------------------------------------------------
P("bb-en-01", "count", "en", LIST_EN + "show how many friends", "len", r'print\(len\(friends\)\)')
P("bb-en-02", "count", "en", LIST_EN + "show the number of friends", "len", r'print\(len\(friends\)\)')
P("bb-en-03", "count", "en", LIST_EN + "show the count of friends", "len", r'print\(len\(friends\)\)')
P("bb-en-04", "count", "en", LIST_EN + "set total to how many friends", "len",
  r'^total = len\(friends\)$')
P("bb-ko-01", "count", "ko", LIST_KO + "친구들 개수 말해줘", "len", r'print\(len\(친구들\)\)')
P("bb-ko-02", "count", "ko", LIST_KO + "친구들의 개수 말해줘", "len", r'print\(len\(친구들\)\)')
P("bb-ko-03", "count", "ko", LIST_KO + "친구들 갯수 말해줘", "len", r'print\(len\(친구들\)\)')
P("bb-ko-04", "count", "ko", LIST_KO + "총합은 친구들 개수", "len", r'^총합 = len\(친구들\)$')
# Prose with the same words.
P("bb-en-05", "count", "en", "how many roads must a man walk down", "print",
  r'print\("how many roads must a man walk down"\)')
P("bb-en-06", "count", "en", "count me in", "print", r'print\("count me in"\)')
P("bb-en-07", "count", "en", "how are you", "print", r'print\("how are you"\)')
P("bb-ko-05", "count", "ko", "친구들 개수가 궁금합니다", "print",
  r'print\("친구들 개수가 궁금합니다"\)')
P("bb-ko-06", "count", "ko", "개수가 부족합니다", "print", r'print\("개수가 부족합니다"\)')

# --- CC. is it in the list, is it empty ---------------------------------
P("cc-en-01", "contains", "en", LIST_EN + "if friends contains Mina\n  show yes\nend",
  "in", r'if \("Mina" in friends\)')
P("cc-en-02", "contains", "en", LIST_EN + "if friends includes Mina\n  show yes\nend",
  "in", r'if \("Mina" in friends\)')
P("cc-en-03", "contains", "en", LIST_EN + "if friends does not contain Ada\n  show yes\nend",
  "not in", r'if \("Ada" not in friends\)')
P("cc-ko-01", "contains", "ko", LIST_KO + "만약에 친구들에 민수가 있으면\n  네 말해줘\n끝",
  "in", r'if \("민수" in 친구들\)')
P("cc-ko-02", "contains", "ko", LIST_KO + "만약에 친구들에 지안이 없으면\n  네 말해줘\n끝",
  "not in", r'if \("지안" not in 친구들\)')
P("cc-en-04", "contains", "en", LIST_EN + "if friends is empty\n  show yes\nend",
  "not", r'if \(not \(friends\)\)')
P("cc-ko-03", "contains", "ko", LIST_KO + "만약에 친구들이 비었으면\n  네 말해줘\n끝",
  "not", r'if \(not \(친구들\)\)')
P("cc-ko-04", "contains", "ko", LIST_KO + "만약에 친구들이 비어있으면\n  네 말해줘\n끝",
  "not", r'if \(not \(친구들\)\)')
P("cc-ko-05", "contains", "ko", "상자가 비었습니다", "print", r'print\("상자가 비었습니다"\)')

# --- DD. taking an item out ---------------------------------------------
P("dd-en-01", "removeitem", "en", LIST_EN + "remove Mina from friends", "remove",
  r'friends\.remove\("Mina"\)')
P("dd-ko-01", "removeitem", "ko", LIST_KO + "친구들에서 민수 빼", "remove",
  r'친구들\.remove\("민수"\)')
P("dd-ko-02", "removeitem", "ko", LIST_KO + "친구들에서 민수 빼줘", "remove",
  r'친구들\.remove\("민수"\)')
# Numbers still subtract.
P("dd-en-02", "removeitem", "en", "set score to 10\nsubtract 1 from score", "minus",
  r'score = score - 1')
P("dd-ko-03", "removeitem", "ko", "점수는 10\n점수에서 1 빼", "minus", r'점수 = 점수 - 1')
# Prose.
P("dd-en-03", "removeitem", "en", "remove your shoes at the door", "refused or printed", None)
P("dd-ko-04", "removeitem", "ko", "어깨에 힘을 빼세요", "print", r'print\("어깨에 힘을 빼세요"\)')

# --- EE. putting a list in order ----------------------------------------
P("ee-en-01", "arrange", "en", LIST_EN + "sort friends", "sort", r'friends\.sort\(\)')
P("ee-en-02", "arrange", "en", LIST_EN + "reverse friends", "reverse", r'friends\.reverse\(\)')
P("ee-en-03", "arrange", "en", LIST_EN + "shuffle friends", "shuffle",
  r'__import__\("random"\)\.shuffle\(friends\)')
P("ee-ko-01", "arrange", "ko", LIST_KO + "친구들 정렬해", "sort", r'친구들\.sort\(\)')
P("ee-ko-02", "arrange", "ko", LIST_KO + "친구들 거꾸로 해", "reverse", r'친구들\.reverse\(\)')
P("ee-ko-03", "arrange", "ko", LIST_KO + "친구들 뒤집어", "reverse", r'친구들\.reverse\(\)')
P("ee-ko-04", "arrange", "ko", LIST_KO + "친구들 섞어", "shuffle",
  r'__import__\("random"\)\.shuffle\(친구들\)')
# Prose that merely contains the words.
P("ee-en-04", "arrange", "en", "sort out your things", "print",
  r'print\("sort out your things"\)')
P("ee-en-05", "arrange", "en", "shuffle along now", "print", r'print\("shuffle along now"\)')
P("ee-ko-05", "arrange", "ko", "거꾸로 읽어도 같습니다", "print",
  r'print\("거꾸로 읽어도 같습니다"\)')
P("ee-ko-06", "arrange", "ko", "설탕을 물에 섞어 주세요", "refused or printed", None)
# A name that was never made a list is refused, not guessed at.
P("ee-en-06", "arrange", "en", "sort friends", "refused", None)
P("ee-ko-07", "arrange", "ko", "친구들 정렬해", "refused", None)

# --- FF. an item by its position ----------------------------------------
P("ff-en-01", "position", "en", LIST_EN + "show the first of friends", "[0]",
  r'print\(friends\[0\]\)')
P("ff-en-02", "position", "en", LIST_EN + "show the last of friends", "[-1]",
  r'print\(friends\[-1\]\)')
P("ff-en-03", "position", "en", LIST_EN + "show item 3 of friends", "[2]",
  r'print\(friends\[2\]\)')
P("ff-ko-01", "position", "ko", LIST_KO + "친구들 첫 번째 말해줘", "[0]", r'print\(친구들\[0\]\)')
P("ff-ko-02", "position", "ko", LIST_KO + "친구들 첫번째 말해줘", "[0]", r'print\(친구들\[0\]\)')
P("ff-ko-03", "position", "ko", LIST_KO + "친구들 마지막 말해줘", "[-1]", r'print\(친구들\[-1\]\)')
P("ff-ko-04", "position", "ko", LIST_KO + "친구들 3번째 말해줘", "[2]", r'print\(친구들\[2\]\)')
# Zero is refused rather than read as the last item.
P("ff-en-04", "position", "en", LIST_EN + "show item 0 of friends", "refused", None)
P("ff-ko-05", "position", "ko", LIST_KO + "친구들 0번째 말해줘", "refused", None)
# Prose.
P("ff-en-05", "position", "en", "the first of many", "print", r'print\("the first of many"\)')
P("ff-en-06", "position", "en", "last but not least", "print", r'print\("last but not least"\)')
P("ff-ko-06", "position", "ko", "첫 번째 손님이었습니다", "print",
  r'print\("첫 번째 손님이었습니다"\)')
P("ff-ko-07", "position", "ko", "마지막 기차를 놓쳤습니다", "print",
  r'print\("마지막 기차를 놓쳤습니다"\)')

# --- GG. total, biggest, smallest ---------------------------------------
P("gg-en-01", "arith", "en", NUMS_EN + "show the total of scores", "sum", r'print\(sum\(scores\)\)')
P("gg-en-02", "arith", "en", NUMS_EN + "show the sum of scores", "sum", r'print\(sum\(scores\)\)')
P("gg-en-03", "arith", "en", NUMS_EN + "show the biggest of scores", "max", r'print\(max\(scores\)\)')
P("gg-en-04", "arith", "en", NUMS_EN + "show the smallest of scores", "min", r'print\(min\(scores\)\)')
P("gg-ko-01", "arith", "ko", NUMS_KO + "점수들 합 말해줘", "sum", r'print\(sum\(점수들\)\)')
P("gg-ko-02", "arith", "ko", NUMS_KO + "점수들 합계 말해줘", "sum", r'print\(sum\(점수들\)\)')
P("gg-ko-03", "arith", "ko", NUMS_KO + "점수들 중 가장 큰 것 말해줘", "max", r'print\(max\(점수들\)\)')
P("gg-ko-04", "arith", "ko", NUMS_KO + "점수들 중 가장 작은 것 말해줘", "min",
  r'print\(min\(점수들\)\)')
P("gg-ko-05", "arith", "ko", NUMS_KO + "점수들 최댓값 말해줘", "max", r'print\(max\(점수들\)\)')
# Prose.
P("gg-en-05", "arith", "en", "the sum of all fears", "print", r'print\("the sum of all fears"\)')
P("gg-ko-06", "arith", "ko", "합이 맞습니다", "print", r'print\("합이 맞습니다"\)')
P("gg-ko-07", "arith", "ko", "가장 좋은 하루였습니다", "print",
  r'print\("가장 좋은 하루였습니다"\)')
P("gg-ko-08", "arith", "ko", "가장 큰 별이었습니다", "print", r'print\("가장 큰 별이었습니다"\)')

# --- HH. joining a list into text ---------------------------------------
P("hh-en-01", "joinlist", "en", LIST_EN + "show friends joined by comma", "join",
  r'print\(", "\.join\(map\(str, friends\)\)\)')
P("hh-en-02", "joinlist", "en", LIST_EN + "show friends joined by space", "join",
  r'print\(" "\.join\(map\(str, friends\)\)\)')
P("hh-ko-01", "joinlist", "ko", LIST_KO + "친구들을 쉼표로 이어 말해줘", "join",
  r'print\(", "\.join\(map\(str, 친구들\)\)\)')
P("hh-ko-02", "joinlist", "ko", LIST_KO + "친구들을 빈칸으로 이어 말해줘", "join",
  r'print\(" "\.join\(map\(str, 친구들\)\)\)')
P("hh-en-03", "joinlist", "en", "we joined hands", "print", r'print\("we joined hands"\)')
P("hh-ko-03", "joinlist", "ko", "쉼표를 찍어 주세요", "refused or printed", None)

# --- II. text: length and case ------------------------------------------
P("ii-en-01", "textread", "en", NAME_EN + "show the length of greeting", "len",
  r'print\(len\(greeting\)\)')
P("ii-en-02", "textread", "en", NAME_EN + "show greeting in capitals", "upper",
  r'print\(str\(greeting\)\.upper\(\)\)')
P("ii-en-03", "textread", "en", NAME_EN + "show greeting in small letters", "lower",
  r'print\(str\(greeting\)\.lower\(\)\)')
P("ii-ko-01", "textread", "ko", NAME_KO + "인사 길이 말해줘", "len", r'print\(len\(인사\)\)')
P("ii-ko-02", "textread", "ko", NAME_KO + "인사 대문자로 말해줘", "upper",
  r'print\(str\(인사\)\.upper\(\)\)')
P("ii-ko-03", "textread", "ko", NAME_KO + "인사 소문자로 말해줘", "lower",
  r'print\(str\(인사\)\.lower\(\)\)')
P("ii-en-04", "textread", "en", "the length of the river", "print",
  r'print\("the length of the river"\)')
P("ii-ko-04", "textread", "ko", "이 강의 길이는 백 킬로미터입니다", "print",
  r'print\("이 강의 길이는 백 킬로미터입니다"\)')
P("ii-ko-05", "textread", "ko", "대문자로 시작합니다", "print",
  r'print\("대문자로 시작합니다"\)')

# --- JJ. a loop with no counter -----------------------------------------
P("jj-en-01", "forever", "en", "repeat forever\n  break\nend", "while True", r'while True:')
P("jj-en-02", "forever", "en", "repeat always\n  break\nend", "while True", r'while True:')
P("jj-en-03", "forever", "en", "repeat forever and show hi", "while True",
  r'while True: print\("hi"\)')
P("jj-ko-01", "forever", "ko", "계속 반복해\n  멈춰\n끝", "while True", r'while True:')
P("jj-ko-02", "forever", "ko", "무한 반복해\n  멈춰\n끝", "while True", r'while True:')
P("jj-ko-03", "forever", "ko", "계속 반복해서 안녕 말해줘", "while True",
  r'while True: print\("안녕"\)')
P("jj-en-04", "forever", "en", "best friends forever", "print",
  r'print\("best friends forever"\)')
P("jj-ko-04", "forever", "ko", "계속 걸었습니다", "print", r'print\("계속 걸었습니다"\)')
P("jj-ko-05", "forever", "ko", "끝없이 이어졌습니다", "print", r'print\("끝없이 이어졌습니다"\)')

# --- KK. a name cannot have a space in it --------------------------------
# Guide 05 taught `할 일은 목록`, which printed the sentence instead of making
# a list. The working form is `할일은 목록`.
P("kk-ko-01", "spacedname", "ko", "할일은 목록", "empty list", r'^할일 = \[\]$')
P("kk-ko-02", "spacedname", "ko", "할일은 빈 목록", "empty list", r'^할일 = \[\]$')
P("kk-ko-03", "spacedname", "ko", "할일은 목록\n할일에 청소 넣어", "append",
  r'할일\.append\("청소"\)')

# --- LL. the parity holes the audit found ---------------------------------
ZK_EN = "use zero_knowledge\n"
ZK_KO = "영지식 사용\n"
# The English zero-knowledge forms that used to be saved as a sentence.
P("ll-en-01", "zksentence", "en",
  ZK_EN + "set p to 1\nset c to 2\nset e to 3\nset z to 4\n"
  "set ok to p c e z zero knowledge verify", "verify", r'^ok = \(1 < \(p\) <')
P("ll-en-02", "zksentence", "en",
  ZK_EN + "set r to 1\nset s to 2\nset e to 3\nset z to r s e zero knowledge response make",
  "response", r'^z = \(\(r\) - \(s\) \* \(e\)\) %')
P("ll-en-03", "zksentence", "en",
  ZK_EN + "set z to zero knowledge simulated response make", "simulated response",
  r'^z = __import__\("secrets"\)\.randbelow\(')
P("ll-ko-01", "zksentence", "ko",
  ZK_KO + "p는 1\nc는 2\ne는 3\nz는 4\nok는 p와 c와 e와 z로 영지식 검증", "verify",
  r'^ok = \(1 < \(p\) <')

# --- MM. loop control inside an indented beginner block --------------------
P("mm-en-01", "indentloop", "en", "2 times:\n    skip", "continue", r'^\s+continue$')
P("mm-en-02", "indentloop", "en", "2 times:\n    break", "break", r'^\s+break$')
P("mm-en-03", "indentloop", "en", "2 times:\n    continue", "continue", r'^\s+continue$')
P("mm-ko-01", "indentloop", "ko", "2번:\n    건너뛰어", "continue", r'^\s+continue$')
P("mm-ko-02", "indentloop", "ko", "2번:\n    멈춰", "break", r'^\s+break$')

# --- NN. branches inside an indented block ---------------------------------
P("nn-ko-01", "indentbranch", "ko", "score = 0\n만약 score > 10:\n    print(1)\n아니면:\n    print(2)",
  "else", r'^else:$')
P("nn-ko-02", "indentbranch", "ko",
  "score = 0\n만약 score > 10:\n    print(1)\n아니면 만약에 score == 0:\n    print(2)",
  "elif", r'^elif \(score == 0\):$')
P("nn-en-01", "indentbranch", "en",
  "score = 0\nwhen score > 10:\n    print(1)\nelse if score == 0:\n    print(2)",
  "elif", r'^elif \(score == 0\):$')
P("nn-en-02", "indentbranch", "en",
  "score = 0\nif score > 10:\n    print(1)\nelse:\n    print(2)", "unchanged Python",
  r'^else:$')

# --- OO. importing another .nme program in a sentence ----------------------
P("oo-en-01", "nmeimport", "en", 'use greet from "helper.nme"', "import",
  r'^from helper import greet$')
P("oo-en-02", "nmeimport", "en", 'use greet, score from "helper.nme"', "import",
  r'^from helper import greet, score$')
P("oo-ko-01", "nmeimport", "ko", '"helper.nme"에서 greet 가져와', "import",
  r'^from helper import greet$')
P("oo-ko-02", "nmeimport", "ko", '"helper.nme"에서 greet 가져오기', "import",
  r'^from helper import greet$')
P("oo-en-03", "nmeimport", "en", "use random", "the bundled module", r'import random as ')
P("oo-ko-03", "nmeimport", "ko", "랜덤 사용", "the bundled module", r'import random as ')
P("oo-en-04", "nmeimport", "en", "take a break", "print", r'print\("take a break"\)')
P("oo-ko-04", "nmeimport", "ko", "자료를 불러오기", "print", r'print\("자료를 불러오기"\)')

# --- PP. asking with the question and no name ------------------------------
P("pp-en-01", "askquestion", "en", "ask what is your name", "name = input",
  r'^name = input\("what is your name" \+ " "\)$')
P("pp-en-02", "askquestion", "en", "ask how old are you", "age = int(input",
  r'^age = int\(input\("how old are you" \+ " "\)\)$')
P("pp-ko-01", "askquestion", "ko", "물어봐 이름이 뭐예요?", "이름 = input",
  r'^이름 = input\("이름이 뭐예요\?" \+ " "\)$')
P("pp-ko-02", "askquestion", "ko", "물어봐 몇 살이에요?", "나이 = int(input",
  r'^나이 = int\(input\("몇 살이에요\?" \+ " "\)\)$')
P("pp-en-03", "askquestion", "en", "ask who is there", "refused", None)
P("pp-en-04", "askquestion", "en", "who is there", "print", r'print\("who is there"\)')

# --- QQ. a name written with a space ---------------------------------------
P("qq-ko-01", "spacedname", "ko", "할 일은 목록", "refused", None)
P("qq-ko-02", "spacedname", "ko", "할 일에 청소 넣어", "refused", None)
P("qq-ko-03", "spacedname", "ko", "오늘 날씨는 맑음", "print", r'print\("오늘 날씨는 맑음"\)')
P("qq-ko-04", "spacedname", "ko", "설탕을 그릇에 넣어", "print", r'print\("설탕을 그릇에 넣어"\)')
P("qq-ko-05", "spacedname", "ko", "할 일이 많습니다", "print", r'print\("할 일이 많습니다"\)')

# --- RR. what is left over after a division --------------------------------
PILE_EN = "set pile to 12\n"
PILE_KO = "쌓인돌은 12\n"
P("rr-en-01", "remainder", "en", PILE_EN + "show the remainder of pile divided by 4",
  "modulo", r'^print\(pile % 4\)$')
P("rr-en-02", "remainder", "en", PILE_EN + "set left to the remainder of pile divided by 4",
  "modulo", r'^left = pile % 4$')
P("rr-ko-01", "remainder", "ko", PILE_KO + "쌓인돌을 4로 나눈 나머지 말해줘", "modulo",
  r'^print\(쌓인돌 % 4\)$')
P("rr-ko-02", "remainder", "ko", PILE_KO + "남은것은 쌓인돌을 4로 나눈 나머지", "modulo",
  r'^남은것 = 쌓인돌 % 4$')
P("rr-en-03", "remainder", "en",
  PILE_EN + "if the remainder of pile divided by 4 equals 0\n  show yes\nend", "modulo",
  r'if \(pile % 4 == 0\)')
P("rr-ko-03", "remainder", "ko",
  PILE_KO + "만약에 쌓인돌을 4로 나눈 나머지가 0과 같으면\n  네 말해줘\n끝", "modulo",
  r'if \(쌓인돌 % 4 == 0\)')
# A saving word opens a save, not arithmetic on a name called `set`.
P("rr-en-04", "remainder", "en", "set score to 12\nset left to score divided by 4",
  "not `set = set / 4`", r'^left = ')
# Prose.
P("rr-en-05", "remainder", "en", "the rest of the story", "print",
  r'print\("the rest of the story"\)')
P("rr-ko-04", "remainder", "ko", "나머지 이야기는 내일", "print",
  r'print\("나머지 이야기는 내일"\)')

# --- SS. story near-misses --------------------------------------------------
P("ss-en-01", "storynear", "en", "story:", "refused, not a raw CPython error", None)
P("ss-ko-01", "storynear", "ko", "이야기:", "refused, not a raw CPython error", None)
P("ss-ko-02", "storynear", "ko", "이야기:\n끝", "refused", None)
P("ss-ko-03", "storynear", "ko", "얘기: 그만하자", "refused", None)
P("ss-en-02", "storynear", "en", "story: chapter", "refused", None)
P("ss-en-03", "storynear", "en", "story: the end", "print", r'print\("story: the end"\)')
P("ss-ko-04", "storynear", "ko", "이야기:\n옛날 옛적에\n끝", "a story", r'print\("옛날 옛적에"\)')

# --- TT. `목록` / `list` is a value only where a value is being saved --------
# `목록을 보여 주세요` ("please show me the list") used to compile to
# `print([])`: an empty pair of brackets, and nothing on screen to tell the
# writer that their sentence had been read as a value.
P("tt-ko-01", "listword", "ko", "목록 보여줘", "print the word", r'^print\("목록"\)$')
P("tt-ko-02", "listword", "ko", "목록을 말해줘", "print the word", r'^print\("목록"\)$')
P("tt-ko-03", "listword", "ko", "목록을 보여 주세요", "print the word", r'^print\("목록"\)$')
P("tt-en-01", "listword", "en", "show list", "print the word", r'^print\("list"\)$')
P("tt-ko-04", "listword", "ko", "목록이 깁니다", "print the sentence",
  r'^print\("목록이 깁니다"\)$')
P("tt-ko-05", "listword", "ko", "이 목록은 길어요", "print the sentence",
  r'^print\("이 목록은 길어요"\)$')
P("tt-ko-06", "listword", "ko", "장바구니 목록을 적었습니다", "print the sentence",
  r'^print\("장바구니 목록을 적었습니다"\)$')
P("tt-en-02", "listword", "en", "the list was long", "print the sentence",
  r'^print\("the list was long"\)$')
# The same words in a saved value are still an empty list.
P("tt-ko-07", "listword", "ko", "친구들은 목록", "친구들 = []", r'^친구들 = \[\]$')
P("tt-ko-08", "listword", "ko", "친구들은 빈 목록", "친구들 = []", r'^친구들 = \[\]$')
P("tt-en-03", "listword", "en", "set scores to an empty list", "scores = []",
  r'^scores = \[\]$')
