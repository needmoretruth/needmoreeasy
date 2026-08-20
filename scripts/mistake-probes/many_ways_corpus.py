"""Every way a beginner might write the same thing.

Each group is one intention and the ways people actually reach for it — the
orderings, the connectives, the synonyms. The owner's words on 2026-08-19:
*순서를 바꾸거나 앞에 두거나 다른 문장연결어를 쓰든 뭘 하든 제대로 작동해야 해*.

`want` is a fragment that must appear in the Python the variant produced. It is
a fragment rather than the whole line because several groups have more than one
correct lowering (`input(...)` gains a trailing space, a block header is
followed by its body).

`setup` runs before the variant, `body` after it, so a block header has
something to open over.
"""

GROUPS = [
    dict(
        name="말하기", want='print("안녕하세요")', setup="", body="",
        ways=[
            "안녕하세요 말해줘", "안녕하세요 말해", "안녕하세요 말하기",
            "안녕하세요 출력해", "말해줘 안녕하세요", "말해 안녕하세요",
            "안녕하세요 라고 말해줘", "안녕하세요 라고 말해", "안녕하세요를 말해줘",
            "안녕하세요라고 출력해", "안녕하세요 보여줘", "안녕하세요 써줘",
            "안녕하세요 말", "안녕하세요 적어줘", "안녕하세요 알려줘",
        ],
    ),
    dict(
        name="say", want='print("hello")', setup="", body="",
        ways=[
            "show hello", "say hello", "print hello", "display hello",
            "hello show", "output hello", "write hello", "tell hello",
            "echo hello", "put hello on the screen",
        ],
    ),
    dict(
        name="저장", want="이름 = 5", setup="", body="",
        ways=[
            "이름은 5", "이름을 5로 정해", "이름은 5이다", "이름을 5로 해",
            "이름 = 5", "이름은 5로", "저장 이름 5", "이름을 5라고 하자",
            "5를 이름에 저장해", "이름을 5로 두어",
        ],
    ),
    dict(
        name="set", want="name = 5", setup="", body="",
        ways=[
            "set name to 5", "name is 5", "let name be 5", "name = 5",
            "save name 5", "make name 5", "put 5 in name", "name becomes 5",
            "call it name 5", "set name 5",
        ],
    ),
    dict(
        name="반복", want="range(3)", setup="", body="\n    안녕 말해줘\n끝",
        ways=[
            "3번 반복해", "3번 반복", "세 번 반복해", "3회 반복해",
            "반복해 3번", "3번 되풀이해", "3번 반복하기", "3번 돌려",
        ],
    ),
    dict(
        name="repeat", want="range(3)", setup="", body="\n    show hi\nend",
        ways=[
            "repeat 3 times", "3 times repeat", "do 3 times",
            "repeat three times", "loop 3 times", "3 times",
            "do it 3 times", "go round 3 times",
        ],
    ),
    dict(
        name="물어보기", want="이름 = input(", setup="", body="",
        ways=[
            "이름을 물어봐 이름이 뭐예요?", "이름 물어봐 이름이 뭐예요?",
            "이름을 입력받아 이름이 뭐예요?", "물어봐 이름 이름이 뭐예요?",
            "이름을 질문해 이름이 뭐예요?", "이름을 받아 이름이 뭐예요?",
        ],
    ),
    dict(
        name="ask", want="name = input(", setup="", body="",
        ways=[
            "ask name what is your name?", "ask for name what is your name?",
            "name ask what is your name?", "prompt name what is your name?",
            "get name what is your name?", "read name what is your name?",
        ],
    ),
    dict(
        name="넣기", want='친구들.append("민수")', setup="친구들은 빈 목록\n", body="",
        ways=[
            "친구들에 민수 넣어", "친구들에 민수를 넣어", "친구들에 민수 추가해",
            "민수를 친구들에 넣어", "친구들에다 민수 넣어", "친구들에 민수 넣기",
            "친구들 민수 넣어", "친구들에 민수 더해",
        ],
    ),
    dict(
        name="append", want='friends.append("Mina")',
        setup="set friends to an empty list\n", body="",
        ways=[
            "append Mina to friends", "add Mina to friends",
            "put Mina in friends", "friends append Mina",
            "to friends append Mina", "insert Mina into friends",
            "push Mina to friends",
        ],
    ),
    dict(
        name="조건", want="if (수 > 3):", setup="수는 5\n", body="\n    안녕 말해줘\n끝",
        ways=[
            "만약에 수가 3보다 크면", "만약 수가 3보다 크면", "수가 3보다 크면",
            "만약에 수 > 3 이면", "만일 수가 3보다 크면", "수가 3 초과면",
            "수가 3보다 클 때", "수가 3보다 많으면",
        ],
    ),
    dict(
        name="if", want="if (n > 3):", setup="set n to 5\n", body="\n    show hi\nend",
        ways=[
            "if n is greater than 3", "if n > 3", "when n is greater than 3",
            "if n is more than 3", "if n bigger than 3",
            "should n be greater than 3", "in case n is greater than 3",
        ],
    ),

    # ---- 2026-08-20: fourteen more intentions. The first twelve groups were
    # written the day the complaint arrived and only covered what a first
    # lesson reaches for. These are the rest of what guide 1–17 teaches, so
    # the number below measures the language rather than its opening.
    dict(
        name="값 바꾸기", want="점수 = 점수 + 1", setup="점수는 0\n", body="",
        ways=[
            "점수에 1 더해", "점수를 1 늘려", "점수에 1을 더해줘", "1을 점수에 더해",
            "점수를 1만큼 늘려줘", "점수에 1 추가해", "점수를 1 올려", "점수에 1 더하기",
            "점수 1 증가", "점수를 1 증가시켜",
        ],
    ),
    dict(
        name="update", want="score = score + 1", setup="set score to 0\n", body="",
        ways=[
            "score add 1", "add 1 to score", "increase score by 1",
            "score increase by 1", "raise score by 1", "bump score by 1",
            "grow score by 1", "increment score by 1", "up score by 1",
            "to score add 1",
        ],
    ),
    dict(
        name="기다리기", want="sleep(3)", setup="", body="",
        ways=[
            "3초 기다려", "3초 기다려줘", "3초 쉬어", "3초 대기해", "기다려 3초",
            "3초간 기다려", "3초 동안 기다려", "3초만 기다려",
        ],
    ),
    dict(
        name="wait", want="sleep(3)", setup="", body="",
        ways=[
            "wait 3 seconds", "wait for 3 seconds", "pause 3 seconds",
            "sleep 3 seconds", "hold 3 seconds", "delay 3 seconds",
            "wait 3 second", "rest for 3 seconds",
        ],
    ),
    dict(
        name="빼기", want='친구들.remove("민수")', setup="친구들은 목록 민수, 지안\n", body="",
        ways=[
            "친구들에서 민수 빼", "친구들에서 민수를 빼줘", "민수를 친구들에서 빼",
            "친구들에서 민수 삭제해", "친구들에서 민수 제거해", "친구들에서 민수 지워",
            "친구들 민수 빼", "친구들에서 민수 빼기",
        ],
    ),
    dict(
        name="remove", want='friends.remove("Mina")',
        setup="set friends to list of Mina, Ada\n", body="",
        ways=[
            "remove Mina from friends", "take Mina out of friends",
            "delete Mina from friends", "drop Mina from friends",
            "friends remove Mina", "erase Mina from friends",
            "subtract Mina from friends", "from friends remove Mina",
        ],
    ),
    dict(
        name="표에 넣기", want='나이표["민수"] = 90', setup="나이표는 빈 표\n", body="",
        ways=[
            "나이표에 민수를 90으로 넣어", "나이표에 민수를 90으로 저장해",
            "민수를 90으로 나이표에 넣어", "나이표에 민수를 90으로 적어",
            "나이표에 민수를 90으로 기억해", "나이표에 민수 90 넣어",
            "나이표에 민수를 90으로 넣기", "나이표에 민수를 90으로 두어",
        ],
    ),
    dict(
        name="record", want='ages["Mina"] = 90', setup="set ages to an empty record\n", body="",
        ways=[
            "put Mina at 90 in ages", "set Mina to 90 in ages",
            "in ages put Mina at 90", "store Mina as 90 in ages",
            "add Mina at 90 to ages", "record Mina as 90 in ages",
            "save Mina as 90 in ages", "put Mina 90 in ages",
        ],
    ),
    dict(
        name="목록마다", want="for 이름 in 이름들:", setup="이름들은 목록 민수, 지안\n",
        body="\n    이름 말해줘\n끝",
        ways=[
            "이름들의 이름마다 반복해", "이름들의 이름마다", "이름들 이름마다 반복해",
            "이름들의 각 이름마다 반복해", "이름들에서 이름마다 반복해",
            "이름들의 이름마다 반복하기", "이름들의 이름마다 돌아",
            "이름들의 이름마다 하나씩",
        ],
    ),
    dict(
        name="for each", want="for name in names:", setup="set names to list of Mina, Ada\n",
        body="\n    show name\nend",
        ways=[
            "for each name in names", "foreach name in names", "for name in names",
            "for every name in names", "repeat for each name in names",
            "go through names as name", "for each name of names",
            "loop for each name in names",
        ],
    ),
    dict(
        name="아니면", want="else:", setup="수는 5\n만약 수가 3보다 크면\n    안녕 말해줘\n",
        body="\n    잘가 말해줘\n끝",
        ways=[
            "아니면", "아니면은", "그렇지 않으면", "안 그러면", "그 외에는", "아니라면",
        ],
    ),
    dict(
        name="else", want="else:", setup="set n to 5\nif n > 3\n    show hi\n",
        body="\n    show bye\nend",
        ways=[
            "else", "otherwise", "or else", "else instead", "if not", "in every other case",
        ],
    ),
    dict(
        name="동안", want="while (수 < 3):", setup="수는 0\n", body="\n    수에 1 더해\n끝",
        ways=[
            "수가 3보다 작은 동안", "수가 3보다 작을 동안", "수가 3 미만인 동안",
            "수가 3보다 작은동안", "수가 3보다 작은 한", "수가 3보다 작으면 계속",
        ],
    ),
    dict(
        name="while", want="while (n < 3):", setup="set n to 0\n", body="\n    n add 1\nend",
        ways=[
            "while n < 3", "while n is less than 3", "as long as n < 3",
            "keep going while n < 3", "repeat while n < 3", "while n smaller than 3",
        ],
    ),
    # Verbs NME does not have, standing where the action word goes. Since
    # 2026-08-20 the line is read as the action word it stands for instead of
    # being handed back a message saying which word to write.
    dict(
        name="말하기-격식", want='print("안녕")', setup="", body="",
        ways=[
            "말합니다 안녕", "출력하기 안녕", "프린트해 안녕", "보여주기 안녕",
            "말해라 안녕",
        ],
    ),
    dict(
        name="say-verbs", want='print("hello")', setup="", body="",
        ways=[
            "log hello", "hello write", "puts hello", "speak hello",
        ],
    ),
    dict(
        name="기다리기-다른말", want="sleep(2)", setup="", body="",
        ways=[
            "2초 기다립니다", "2초 멈춰줘", "2초 잠시멈춰", "2초 슬립",
            "2초 대기해", "2초 대기",
        ],
    ),
    dict(
        name="wait-verbs", want="sleep(2)", setup="", body="",
        ways=[
            "hold 2 seconds", "delay 2 seconds", "rest 2 seconds",
        ],
    ),
    dict(
        name="반복-다른말", want="range(3)", setup="", body="\n    안녕 말해줘\n끝",
        ways=[
            "3번 돌려서", "3번 루프해서", "3번 반복합니다", "3번 되풀이해서",
        ],
    ),
    dict(
        name="물어보기-다른말", want="이름 = input(", setup="", body="",
        ways=[
            "이름을 입력해 이름이 뭐예요?", "이름을 무러봐 이름이 뭐예요?",
            "이름을 물어봐 이름이 뭐예요?",
        ],
    ),
    dict(
        name="넣기-다른말", want='친구들.append("민수")', setup="친구들은 목록 지안\n", body="",
        ways=[
            "친구들에 민수 집어넣어", "친구들에 민수 너허", "친구들에 민수 넣어",
        ],
    ),
]
