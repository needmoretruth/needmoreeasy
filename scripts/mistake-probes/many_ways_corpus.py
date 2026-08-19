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
]
