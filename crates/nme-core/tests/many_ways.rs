//! The other ways a beginner writes the same thing.
//!
//! The owner watched one use NME on 2026-08-19 and reported it in a sentence:
//! *무언가 문법을 사용할 때 순서를 바꾸거나 앞에 두거나 다른 문장연결어를 쓰든
//! 뭘 하든 제대로 작동해야 해. 지금 코드는 잘 받아주는 척하면서 정작 「왜 이게
//! 작동 안 하지?」 싶은 게 너무 많아.*
//!
//! Every spelling added in answer to that is here, and beside each one the
//! ordinary sentence it must not claim. The two halves are the whole point:
//! a spelling that costs a sentence is not an improvement.

use nme_core::transpile;

fn ok(source: &str) -> String {
    transpile(source)
        .unwrap_or_else(|problems| panic!("expected successful transpile, got: {problems:?}"))
}

fn error_code(source: &str) -> String {
    let problems = transpile(source).expect_err("expected this line to be rejected");
    problems[0].code.code().to_string()
}

fn refusal_names(source: &str) -> String {
    let problems = transpile(source).expect_err("expected this line to be rejected");
    format!(
        "{} {}",
        problems[0].message,
        problems[0].hint.clone().unwrap_or_default()
    )
}

// ----------------------------------------------- Korean output, written last

#[test]
fn korean_output_words_that_only_close_a_line() {
    assert_eq!(ok("안녕하세요 말하기\n"), "print(\"안녕하세요\")\n");
    assert_eq!(ok("안녕하세요 알려줘\n"), "print(\"안녕하세요\")\n");
    assert_eq!(ok("안녕하세요 알려주세요\n"), "print(\"안녕하세요\")\n");
    assert_eq!(ok("안녕하세요 얘기해\n"), "print(\"안녕하세요\")\n");
    assert_eq!(ok("안녕하세요 표시해\n"), "print(\"안녕하세요\")\n");
    assert_eq!(ok("안녕하세요 출력하기\n"), "print(\"안녕하세요\")\n");
    // The commonest command shape there is: an object particle and the verb.
    assert_eq!(ok("결과를 알려줘\n"), "print(\"결과\")\n");
    assert_eq!(ok("가격을 표시해\n"), "print(\"가격\")\n");
}

#[test]
fn two_nouns_joined_by_a_particle_are_a_noun_phrase() {
    // `와`, `보다`, `의` and their relatives tie the word they are on to the
    // noun after it, so the noun is not the verb of the line.
    assert_eq!(ok("듣기와 말하기\n"), "print(\"듣기와 말하기\")\n");
    assert_eq!(ok("글쓰기보다 말하기\n"), "print(\"글쓰기보다 말하기\")\n");
}

#[test]
fn a_korean_output_word_at_the_front_is_still_a_sentence() {
    // Korean states its verb last. At the front `말하기` is the noun
    // *speaking*, and the line is prose. (`보여주기 싫어요` is refused instead,
    // and was before this round: `보여주기` opens the line, which is where
    // `COMMAND_WORDS_LEADING` reads it.)
    assert_eq!(ok("말하기 연습\n"), "print(\"말하기 연습\")\n");
    assert_eq!(ok("말하기 대회에 나갔습니다\n"), "print(\"말하기 대회에 나갔습니다\")\n");
}

#[test]
fn the_one_syllable_output_word() {
    // One syllable is not much to go on, so the message has to be something a
    // program would say: it ends the way spoken Korean ends, or it is a name
    // the program already made.
    assert_eq!(ok("안녕하세요 말\n"), "print(\"안녕하세요\")\n");
    assert_eq!(ok("고맙습니다 말\n"), "print(\"고맙습니다\")\n");
    assert_eq!(ok("점수는 5\n점수 말\n"), "점수 = 5\nprint(점수)\n");
    assert_eq!(
        ok("이름을 물어봐 이름이 뭐예요?\n안녕하세요 이름! 말\n"),
        "이름 = input(\"이름이 뭐예요?\" + \" \")\nprint(\"안녕하세요 \" + str(이름) + \"!\")\n"
    );
    // A bare noun is not: `엄마 말`, `친구 말` and `농담 말` are noun phrases.
    assert_eq!(ok("엄마 말\n"), "print(\"엄마 말\")\n");
    assert_eq!(ok("친구 말\n"), "print(\"친구 말\")\n");
    assert_eq!(ok("농담 말\n"), "print(\"농담 말\")\n");
}

#[test]
fn a_noun_phrase_ending_in_the_word_word_is_still_a_sentence() {
    // An adnominal ending (`ㄴ`/`ㄹ`) or a determiner in front of `말` makes
    // it the noun *word*, which is the only thing standing there in Korean.
    for sentence in [
        "그건 좋은 말",
        "무슨 말",
        "그 말",
        "내 말",
        "따뜻한 말",
        "요즘 말",
        "당신 말",
        "우리 말",
        "예쁜 말",
        "한마디 말",
    ] {
        let source = format!("{sentence}\n");
        assert_eq!(
            ok(&source),
            format!("print(\"{sentence}\")\n"),
            "{sentence} stopped being a sentence"
        );
    }
}

#[test]
fn the_short_output_word_is_never_repaired() {
    // `말` is one character from `물`, `발`, `날` and `살`, so only the word
    // itself counts.
    assert_eq!(ok("안녕하세요 물\n"), "print(\"안녕하세요 물\")\n");
    assert_eq!(ok("안녕하세요 날\n"), "print(\"안녕하세요 날\")\n");
}

// ----------------------------------------------------- where a message goes

#[test]
fn the_screen_is_a_place_a_message_goes() {
    assert_eq!(ok("put hello on the screen\n"), "print(\"hello\")\n");
    assert_eq!(ok("write hello on the screen\n"), "print(\"hello\")\n");
    assert_eq!(ok("show hello to the screen\n"), "print(\"hello\")\n");
    assert_eq!(ok("안녕하세요 화면에 띄워\n"), "print(\"안녕하세요\")\n");
    assert_eq!(ok("안녕하세요 화면에다 보여줘\n"), "print(\"안녕하세요\")\n");
}

#[test]
fn a_line_without_a_screen_is_still_a_sentence() {
    assert_eq!(ok("put the kettle on\n"), "print(\"put the kettle on\")\n");
    assert_eq!(
        ok("the screen was too bright\n"),
        "print(\"the screen was too bright\")\n"
    );
    // The verb is what makes it a command; without one these are writing.
    assert_eq!(
        ok("words appeared on the screen\n"),
        "print(\"words appeared on the screen\")\n"
    );
    assert_eq!(ok("휴대폰 화면에\n"), "print(\"휴대폰 화면에\")\n");
}

// ------------------------------------------- words that are not output words

#[test]
fn one_word_after_a_verb_that_is_not_an_action_is_named() {
    for (source, word) in [
        ("log hello\n", "log"),
        ("read hello\n", "read"),
        ("hello write\n", "write"),
    ] {
        assert_eq!(error_code(source), "E0603", "{source} was not refused");
        assert!(
            refusal_names(source).contains(word),
            "{source} did not name `{word}`"
        );
    }
}

#[test]
fn the_same_verbs_keep_a_whole_sentence() {
    for sentence in [
        "write your name on the envelope",
        "write it down before you forget",
        "read the instructions twice",
        "echo of the mountain",
        "log the miles you walked this week",
        // A phrasal verb is two words and still not a command: `out`, `on`
        // and `back` are words a sentence may never make into a name.
        "log out",
        "read on",
        "echo back",
    ] {
        let source = format!("{sentence}\n");
        assert_eq!(
            ok(&source),
            format!("print(\"{sentence}\")\n"),
            "{sentence} stopped being a sentence"
        );
    }
}

// ------------------------------------------------------------ saving a value

#[test]
fn korean_saves_with_its_everyday_verb() {
    assert_eq!(ok("이름을 5로 해\n"), "이름 = 5\n");
    assert_eq!(ok("이름을 5라고 하자\n"), "이름 = 5\n");
    assert_eq!(ok("점수를 0으로 하자\n"), "점수 = 0\n");
    assert_eq!(ok("인사를 안녕하세요라고 하자\n"), "인사 = \"안녕하세요\"\n");
}

#[test]
fn the_everyday_verb_needs_the_mark_that_says_what_it_becomes() {
    // `해` attaches to any noun in the language. Without `로`/`라고` there is
    // no assignment here, only somebody asking for the rice to be nice.
    assert_eq!(ok("밥을 맛있게 해\n"), "print(\"밥을 맛있게 해\")\n");
    assert_eq!(ok("숙제를 해\n"), "print(\"숙제를 해\")\n");
    assert_eq!(ok("조용히 해\n"), "print(\"조용히 해\")\n");
}

#[test]
fn english_saves_with_its_everyday_verb() {
    assert_eq!(ok("name becomes 5\n"), "name = 5\n");
    assert_eq!(ok("score become 0\n"), "score = 0\n");
    assert_eq!(ok("set best to 9\nscore becomes best\n"), "best = 9\nscore = best\n");
    assert_eq!(ok("call it name 5\n"), "name = 5\n");
    assert_eq!(ok("call it greeting Hello world\n"), "greeting = \"Hello world\"\n");
}

#[test]
fn a_word_a_sentence_may_not_name_keeps_the_sentence() {
    assert_eq!(ok("Call it a day.\n"), "print(\"Call it a day.\")\n");
    assert_eq!(
        ok("Call it what you like.\n"),
        "print(\"Call it what you like.\")\n"
    );
    assert_eq!(
        ok("call your grandmother on Sunday\n"),
        "print(\"call your grandmother on Sunday\")\n"
    );
    // `becomes` takes a number, a literal or a name the program made, so the
    // sentences that use it about the world keep their words.
    assert_eq!(
        ok("Water becomes ice at zero degrees.\n"),
        "print(\"Water becomes ice at zero degrees.\")\n"
    );
    assert_eq!(ok("Winter becomes spring.\n"), "print(\"Winter becomes spring.\")\n");
}

// ------------------------------------------------------------------- asking

#[test]
fn a_verb_that_only_asks_on_a_line_that_asks() {
    assert_eq!(
        ok("read name what is your name?\n"),
        "name = input(\"what is your name?\" + \" \")\n"
    );
    assert_eq!(
        ok("get name what is your name?\n"),
        "name = input(\"what is your name?\" + \" \")\n"
    );
    assert_eq!(
        ok("이름을 받아 이름이 뭐예요?\n"),
        "이름 = input(\"이름이 뭐예요?\" + \" \")\n"
    );
    assert_eq!(
        ok("이름을 여쭤봐 이름이 뭐예요?\n"),
        "이름 = input(\"이름이 뭐예요?\" + \" \")\n"
    );
}

#[test]
fn the_same_verbs_without_a_question_stay_sentences() {
    assert_eq!(
        ok("read the label on the bottle\n"),
        "print(\"read the label on the bottle\")\n"
    );
    assert_eq!(
        ok("Read to your children every night.\n"),
        "print(\"Read to your children every night.\")\n"
    );
}

#[test]
fn the_short_asking_word() {
    assert_eq!(
        ok("이름 물어 이름이 뭐예요?\n"),
        "이름 = input(\"이름이 뭐예요?\" + \" \")\n"
    );
    // Two syllables is one edit from half the language, so it is never
    // repaired: both of these are sentences.
    assert_eq!(
        ok("물을 가져와 마셨습니다\n"),
        "print(\"물을 가져와 마셨습니다\")\n"
    );
    assert_eq!(ok("됐어 그만해\n"), "print(\"됐어 그만해\")\n");
    // A helper verb straight after it makes a compound verb, not a question.
    assert_eq!(
        ok("길을 물어 보았습니다\n"),
        "print(\"길을 물어 보았습니다\")\n"
    );
}

// --------------------------------------------------------- adding to a list

#[test]
fn the_list_may_be_written_first() {
    assert_eq!(
        ok("set friends to an empty list\nfriends append Mina\n"),
        "friends = []\nfriends.append(\"Mina\")\n"
    );
    assert_eq!(
        ok("친구들은 빈 목록\n친구들 민수 넣어\n"),
        "친구들 = []\n친구들.append(\"민수\")\n"
    );
}

#[test]
fn a_name_the_program_never_made_a_list_is_not_one() {
    // Without the particle the list has to be a name the program already
    // made one, so this is somebody cooking.
    assert_eq!(ok("그릇 설탕 넣어\n"), "print(\"그릇 설탕 넣어\")\n");
}

// ------------------------------------------------------------------ repeating

#[test]
fn a_repeat_word_with_a_count_beside_it() {
    assert_eq!(
        ok("loop 3 times\n    show hi\nend\n"),
        "for _ in range(3):\n    print(\"hi\")\n# end\n"
    );
    assert_eq!(
        ok("3번 돌려\n    안녕 말해줘\n끝\n"),
        "for _ in range(3):\n    print(\"안녕\")\n# end\n"
    );
    assert_eq!(
        ok("3번 되풀이해\n    안녕 말해줘\n끝\n"),
        "for _ in range(3):\n    print(\"안녕\")\n# end\n"
    );
    assert_eq!(
        ok("3번 되풀이해서 안녕 말해줘\n"),
        "for _ in range(3): print(\"안녕\")\n"
    );
}

#[test]
fn a_repeat_word_without_a_count_is_an_ordinary_verb() {
    assert_eq!(
        ok("loop the ribbon around twice\n"),
        "print(\"loop the ribbon around twice\")\n"
    );
    assert_eq!(
        ok("같은 하루를 최대 10회 되풀이할 수 있어요 말해줘\n"),
        "print(\"같은 하루를 최대 10회 되풀이할 수 있어요\")\n"
    );
}

// ----------------------------------------------------------------- comparing

#[test]
fn more_and_fewer_compare_the_way_greater_and_less_do() {
    assert_eq!(
        ok("set n to 5\nif n is more than 3\n    show hi\nend\n"),
        "n = 5\nif (n > 3):\n    print(\"hi\")\n# end\n"
    );
    assert_eq!(
        ok("set n to 5\nif n is fewer than 3\n    show hi\nend\n"),
        "n = 5\nif (n < 3):\n    print(\"hi\")\n# end\n"
    );
    assert_eq!(
        ok("수는 5\n수가 3보다 많으면\n    안녕 말해줘\n끝\n"),
        "수 = 5\nif (수 > 3):\n    print(\"안녕\")\n# end\n"
    );
    assert_eq!(
        ok("수는 5\n수가 3보다 적으면\n    안녕 말해줘\n끝\n"),
        "수 = 5\nif (수 < 3):\n    print(\"안녕\")\n# end\n"
    );
}

// ------------------------------------ four defects found writing a real game
//
// A 234-line sentence-syntax program (a turn-based role-playing game) was
// written with this compiler on 2026-08-19, and each of these is something it
// hit. Every one of them compiled into a program that ran and did something
// else, which is the failure this file exists to end.

#[test]
fn an_input_line_is_not_read_as_a_condition() {
    // `사려면` inside the question ends in `-면`, which is also how a Korean
    // condition connects. The line names what it is asking for and then asks,
    // so the question wins — and the name loses its object particle, because
    // `상점선택을` is not a name any program ever made.
    assert_eq!(
        ok("상점선택을 물어봐 살까요? 사려면 사다 라고 적어 주세요.\n"),
        "상점선택 = input(\"살까요? 사려면 사다 라고 적어 주세요.\" + \" \")\n"
    );
}

#[test]
fn a_random_range_may_end_at_a_name() {
    assert_eq!(
        ok("적공격은 4\n받은피해는 1부터 적공격까지 랜덤정수\n"),
        "적공격 = 4\n받은피해 = __import__(\"random\").randint(1, 적공격)\n"
    );
    assert_eq!(
        ok("적공격은 4\n받은피해는 1 부터 적공격 까지 랜덤정수\n"),
        "적공격 = 4\n받은피해 = __import__(\"random\").randint(1, 적공격)\n"
    );
    assert_eq!(
        ok("set attack to 4\nset damage to random number from 1 to attack\n"),
        "attack = 4\ndamage = __import__(\"random\").randint(1, attack)\n"
    );
    // The literal bounds that always worked still do.
    assert_eq!(
        ok("받은피해는 1부터 6까지 랜덤정수\n"),
        "받은피해 = __import__(\"random\").randint(1, 6)\n"
    );
    assert_eq!(
        ok("set damage to random number from 1 to 6\n"),
        "damage = __import__(\"random\").randint(1, 6)\n"
    );
}

#[test]
fn else_may_follow_a_one_line_if() {
    assert_eq!(
        ok("체력은 5\n만약에 체력이 0보다 크면 살아있음 말해줘\n아니면 쓰러졌습니다 말해줘\n"),
        "체력 = 5\nif (체력 > 0): print(\"살아있음\")\nelse: print(\"쓰러졌습니다\")\n"
    );
    assert_eq!(
        ok("set hp to 5\nif hp is greater than 0 then show alive\notherwise show down\n"),
        "hp = 5\nif (hp > 0): print(\"alive\")\nelse: print(\"down\")\n"
    );
    assert_eq!(
        ok("체력은 5\n만약에 체력이 10보다 크면 좋음 말해줘\n아니면 만약에 체력이 0보다 크면 보통 말해줘\n아니면 쓰러졌습니다 말해줘\n"),
        "체력 = 5\nif (체력 > 10): print(\"좋음\")\nelif (체력 > 0): print(\"보통\")\nelse: print(\"쓰러졌습니다\")\n"
    );
}

#[test]
fn a_second_else_after_a_one_line_if_is_refused() {
    assert_eq!(
        error_code("체력은 5\n만약에 체력이 0보다 크면 A 말해줘\n아니면 B 말해줘\n아니면 C 말해줘\n"),
        "E0104"
    );
}

#[test]
fn an_else_still_needs_an_if_in_front_of_it() {
    assert_eq!(error_code("아니면 쓰러졌습니다 말해줘\n"), "E0103");
    assert_eq!(
        error_code("안녕 말해줘\n아니면 쓰러졌습니다 말해줘\n"),
        "E0103"
    );
}

#[test]
fn the_manqeum_particle_interpolates_a_name() {
    assert_eq!(
        ok("준피해는 3\n준피해만큼 베었습니다 말해줘\n"),
        "준피해 = 3\nprint(str(준피해) + \"만큼 베었습니다\")\n"
    );
    assert_eq!(
        ok("준피해는 3\n준피해밖에 없습니다 말해줘\n"),
        "준피해 = 3\nprint(str(준피해) + \"밖에 없습니다\")\n"
    );
}

/// `동안` used a shorter comparison vocabulary than `만약`, and the gap was not
/// a refusal: `점수가 10보다 크거나 같을 동안` reached the one-letter repair,
/// which read `같을` as `작을` and built the loop **backwards**. A loop that
/// runs when it should stop is the worst thing a compiler can do quietly.
#[test]
fn a_while_loop_compares_the_way_the_words_say() {
    for (source, python) in [
        ("점수가 10보다 크거나 같을 동안\n", "while (점수 >= 10):\n"),
        ("점수가 10보다 크거나 같은 동안\n", "while (점수 >= 10):\n"),
        ("점수가 10보다 작거나 같을 동안\n", "while (점수 <= 10):\n"),
        ("점수가 10보다 작거나 같은 동안\n", "while (점수 <= 10):\n"),
        ("점수가 10 이상인 동안\n", "while (점수 >= 10):\n"),
        ("점수가 10 이상일 동안\n", "while (점수 >= 10):\n"),
        ("점수가 10 이하인 동안\n", "while (점수 <= 10):\n"),
        ("점수가 10 이하일 동안\n", "while (점수 <= 10):\n"),
        ("점수가 10보다 클 동안\n", "while (점수 > 10):\n"),
        ("점수가 10보다 큰 동안\n", "while (점수 > 10):\n"),
        ("점수가 10보다 작은 동안\n", "while (점수 < 10):\n"),
        ("점수가 10과 같을 동안\n", "while (점수 == 10):\n"),
    ] {
        let program = format!("점수는 50\n{source}  가 말해줘\n끝\n");
        let built = ok(&program);
        let second = built.lines().nth(1).unwrap_or_default();
        assert_eq!(
            format!("{second}\n"),
            python,
            "for {source:?}"
        );
    }
}

/// And the words that made those endings possible must stay ordinary words.
/// `큰`, `작은`, `많은`, `다른` are among the commonest Korean has; listing them
/// as comparison endings turned three shipped examples into comparisons.
#[test]
fn bare_korean_adnominals_are_still_ordinary_words() {
    assert_eq!(ok("더 큰 수예요 말해줘\n"), "print(\"더 큰 수예요\")\n");
    assert!(ok("상자로 말해줘 작은 차림표\n").contains("작은 차림표"));
    assert_eq!(ok("다른 길로 갑시다\n"), "print(\"다른 길로 갑시다\")\n");
    assert_eq!(ok("같은 반 친구입니다\n"), "print(\"같은 반 친구입니다\")\n");
}

/// A question is addressed to a person. Substituting the program's own names
/// into it made the password program ask *용가 무엇입니까?* — with the password
/// in the question.
#[test]
fn a_question_is_read_as_the_words_that_were_typed() {
    assert_eq!(
        ok("비밀번호는 용\n입력을 물어봐 비밀번호가 무엇입니까?\n"),
        "비밀번호 = \"용\"\n입력 = input(\"비밀번호가 무엇입니까?\" + \" \")\n"
    );
    assert_eq!(
        ok("set password to dragon\nask word What is the password?\n"),
        "password = \"dragon\"\nword = input(\"What is the password?\" + \" \")\n"
    );
}

/// The same question read two ways depending on how it was spelled: written on
/// its own it gave a number, written after `ask <name>` it gave text, and the
/// comparison the learner wrote next was silently false for ever.
#[test]
fn a_question_that_asks_a_number_gives_a_number_either_way() {
    for source in [
        "How old are you?\n",
        "ask age How old are you?\n",
        "몇 살이에요?\n",
        "나이를 물어봐 몇 살이에요?\n",
        "ask legs How many legs?\n",
    ] {
        assert!(
            ok(source).contains("int(input("),
            "expected a number for {source:?}, got {}",
            ok(source)
        );
    }
    // And a question that asks for words still gives words.
    assert!(!ok("ask name What is your name?\n").contains("int(input("));
}

/// `값들 무작위로 섞어` put a correctly spelled `섞어` one edit from `넣어`, so
/// the shuffle became `값들.append("무작위로")`: the adverb went into the data
/// and the list was never shuffled.
#[test]
fn an_adverb_between_the_name_and_the_verb_still_arranges_the_list() {
    let head = "값들은 목록 3, 1, 2\n";
    for (line, python) in [
        ("값들 무작위로 섞어\n", "__import__(\"random\").shuffle(값들)\n"),
        ("값들 잘 섞어\n", "__import__(\"random\").shuffle(값들)\n"),
        ("값들 한번 섞어\n", "__import__(\"random\").shuffle(값들)\n"),
        ("값들 다시 정렬해\n", "값들.sort()\n"),
        ("값들 그냥 거꾸로해\n", "값들.reverse()\n"),
    ] {
        let built = ok(&format!("{head}{line}"));
        assert_eq!(built.lines().nth(1).map(|l| format!("{l}\n")), Some(python.to_string()), "for {line:?}");
    }
    // English says it with politeness on the end instead.
    assert_eq!(
        ok("set xs to list of 3, 1, 2\nsort xs please\n"),
        "xs = [3, 1, 2]\nxs.sort()\n"
    );
    assert_eq!(
        ok("set xs to list of 3, 1, 2\nplease shuffle xs\n"),
        "xs = [3, 1, 2]\n__import__(\"random\").shuffle(xs)\n"
    );
}

/// And a sentence that merely contains one of those verbs is still a sentence:
/// the first word has to be a list the program already made.
#[test]
fn an_ordinary_sentence_with_an_arranging_verb_still_prints() {
    assert_eq!(ok("모두 잘 섞어\n"), "print(\"모두 잘 섞어\")\n");
    assert_eq!(
        ok("카드를 잘 섞어 나눠 주세요\n"),
        "print(\"카드를 잘 섞어 나눠 주세요\")\n"
    );
}

/// A character sheet printed `strength 7` as `7 7`: the word used as the label
/// was replaced by the very value it was labelling. The same name twice in one
/// sentence is a label and then its value, so only the last one is replaced.
#[test]
fn a_name_written_twice_labels_first_and_shows_the_value_last() {
    assert_eq!(
        ok("set strength to 7\nshow strength strength\n"),
        "strength = 7\nprint(\"strength \" + str(strength))\n"
    );
    assert_eq!(
        ok("힘은 7\n힘 힘 말해줘\n"),
        "힘 = 7\nprint(\"힘 \" + str(힘))\n"
    );
    // Korean writes the label with a particle on it and the value bare.
    assert_eq!(
        ok("점수는 10\n점수는 점수 말해줘\n"),
        "점수 = 10\nprint(\"점수는 \" + str(점수))\n"
    );
    // One occurrence still behaves exactly as before.
    assert_eq!(
        ok("set score to 10\nshow you scored score points\n"),
        "score = 10\nprint(\"you scored \" + str(score) + \" points\")\n"
    );
}

/// `show You put the key in your bag.` printed the whole inventory list in the
/// middle of the sentence. A word right after an article, a possessive or a
/// determiner is an ordinary word, not a request for what that name holds.
#[test]
fn a_word_after_an_article_is_an_ordinary_word() {
    assert_eq!(
        ok("set bag to list of \"key\"\nshow You put the key in your bag.\n"),
        "bag = [\"key\"]\nprint(\"You put the key in your bag.\")\n"
    );
    assert_eq!(
        ok("set answer to 7\nshow The answer is answer\n"),
        "answer = 7\nprint(\"The answer is \" + str(answer))\n"
    );
    assert_eq!(
        ok("점수는 10\n모든 점수를 보여줍니다 말해줘\n"),
        "점수 = 10\nprint(\"모든 점수를 보여줍니다\")\n"
    );
    // Korean `그` points at the value just spoken about, so it still stands in.
    assert_eq!(
        ok("점수는 10\n그 점수 말해줘\n"),
        "점수 = 10\nprint(\"그 \" + str(점수))\n"
    );
}

/// Whatever the words happen to be, a sentence inside quotation marks is
/// printed exactly as it was written — the escape both languages share.
#[test]
fn a_quoted_sentence_is_printed_exactly_as_written() {
    assert_eq!(
        ok("가방은 목록 \"열쇠\"\n\"가방에 열쇠를 넣었습니다\" 말해줘\n"),
        "가방 = [\"열쇠\"]\nprint(\"가방에 열쇠를 넣었습니다\")\n"
    );
    assert_eq!(
        ok("set bag to list of \"key\"\nshow \"You put the key in your bag.\"\n"),
        "bag = [\"key\"]\nprint(\"You put the key in your bag.\")\n"
    );
}

/// The everyday verbs a beginner reaches for when the message is one word.
/// They may not swallow a sentence, so they claim the line only when a single
/// word is left after them — and that word has to be one a sentence could
/// make into a name, or `give up` and `echo back` would lose half of
/// themselves.
#[test]
fn an_everyday_verb_shows_the_one_word_after_it() {
    for line in [
        "output hello", "write hello", "echo hello", "reveal hello", "report hello",
        "give hello", "list hello", "present hello", "announce hello",
    ] {
        assert_eq!(
            ok(&format!("{line}\n")),
            "print(\"hello\")\n",
            "{line} did not show its word"
        );
    }
    assert_eq!(ok("set score to 7\ngive score\n"), "score = 7\nprint(score)\n");
    // Two words, or one that never names anything, and it is a sentence again.
    for sentence in ["give up", "echo back", "write in", "report on", "give it a try"] {
        assert_eq!(
            ok(&format!("{sentence}\n")),
            format!("print(\"{sentence}\")\n"),
            "{sentence} stopped being a sentence"
        );
    }
}

/// `안녕 띄워` and `점수 나타내` are output; `배를 띄워` and `감정을 나타내` are
/// sentences. What separates them is the object mark on the word before.
#[test]
fn korean_transitive_output_words_leave_their_own_sentences_alone() {
    assert_eq!(ok("안녕 띄워\n"), "print(\"안녕\")\n");
    assert_eq!(ok("안녕하세요 띄워줘\n"), "print(\"안녕하세요\")\n");
    assert_eq!(ok("점수는 7\n점수 나타내\n"), "점수 = 7\nprint(점수)\n");
    assert_eq!(ok("배를 띄워\n"), "print(\"배를 띄워\")\n");
    assert_eq!(ok("감정을 나타내\n"), "print(\"감정을 나타내\")\n");
    assert_eq!(ok("연을 하늘에 띄워줘\n"), "print(\"연을 하늘에 띄워줘\")\n");
}

/// `put`, `insert` and `place` are the words a beginner reaches for when the
/// list is a box and the value is a thing. They are ordinary English verbs
/// too, so they make a list line only when the name after the connector is
/// already a list — and `put 5 in score` goes on to mean saving, which is
/// what it says.
#[test]
fn everyday_verbs_put_things_in_lists_and_names() {
    let list = "set friends to list of \"Mina\"\n";
    for line in ["insert Sana into friends", "put Sana in friends", "place Sana onto friends"] {
        assert_eq!(
            ok(&format!("{list}{line}\n")),
            "friends = [\"Mina\"]\nfriends.append(\"Sana\")\n",
            "for {line}"
        );
    }
    assert_eq!(ok("put 5 in score\n"), "score = 5\n");
    assert_eq!(ok("put \"Mina\" in name\n"), "name = \"Mina\"\n");
    // The same words in a sentence keep every word they have.
    for sentence in [
        "put the kettle on",
        "put money in the bank",
        "insert the key into the lock",
        "place your order here",
        "put your name in the box",
    ] {
        assert_eq!(
            ok(&format!("{sentence}\n")),
            format!("print(\"{sentence}\")\n"),
            "{sentence} stopped being a sentence"
        );
    }
}

/// The record line whose key was written as two words used to be told that
/// `put` is not an action word, and to write `add` — which would put the key
/// in a list instead. The space is what is wrong, and that is now what it
/// says.
#[test]
fn a_record_key_written_as_two_words_is_named() {
    let source = "set marks to an empty record\nput wash up at 90 in marks\n";
    assert_eq!(error_code(source), "E0230");
    assert!(refusal_names(source).contains("washup"), "{}", refusal_names(source));
}

/// The shortest ways anybody says a value went up or down. `up` and `down`
/// are ordinary words, so the line has to open with a name the program made —
/// `give up` and `write it down before you forget` keep every word.
#[test]
fn a_value_goes_up_and_down_in_the_words_people_use() {
    let head = "set score to 0\n";
    for (line, python) in [
        ("score up 1", "score = score + 1"),
        ("score goes up by 2", "score = score + 2"),
        ("score down 1", "score = score - 1"),
        ("score goes down by 2", "score = score - 2"),
    ] {
        assert_eq!(
            ok(&format!("{head}{line}\n")),
            format!("score = 0\n{python}\n"),
            "for {line}"
        );
    }
    let korean = "점수는 0\n";
    for (line, python) in [
        ("점수 1 증가", "점수 = 점수 + 1"),
        ("점수에 1 더하기", "점수 = 점수 + 1"),
        ("점수 2 감소", "점수 = 점수 - 2"),
        ("점수에서 2 빼기", "점수 = 점수 - 2"),
    ] {
        assert_eq!(
            ok(&format!("{korean}{line}\n")),
            format!("점수 = 0\n{python}\n"),
            "for {line}"
        );
    }
    for sentence in [
        "give up",
        "log out",
        "write it down before you forget",
        "put it in the fire",
    ] {
        assert_eq!(
            ok(&format!("{sentence}\n")),
            format!("print(\"{sentence}\")\n"),
            "{sentence} stopped being a sentence"
        );
    }
}

/// One more sweep of near-synonyms and orders a writer may reach for. Each
/// line here failed before the widening it belongs to.
#[test]
fn more_ways_to_write_the_same_command() {
    for (line, python) in [
        ("rep 3 times and show hi", "for _ in range(3): print(\"hi\")"),
        ("go round 2 times and show ok", "for _ in range(2): print(\"ok\")"),
        (
            "run through 2 times and show ok",
            "for _ in range(2): print(\"ok\")",
        ),
    ] {
        assert_eq!(ok(&format!("{line}\n")), format!("{python}\n"), "for {line}");
    }

    let start = "set n to 5\n";
    for line in [
        "when n is greater than 3 then show big",
        "should n be greater than 3 then show big",
        "whenever n is greater than 3 then show big",
        "incase n is greater than 3 then show big",
    ] {
        assert_eq!(
            ok(&format!("{start}{line}\n")),
            "n = 5\nif (n > 3): print(\"big\")\n",
            "for {line}"
        );
    }

    assert_eq!(
        ok("친구들은 목록 하나 둘\n친구들마다 반복해\n  친구 말해줘\n끝\n"),
        "친구들 = [\"하나 둘\"]\nfor 친구 in 친구들:\n  print(친구)\n# end\n",
    );
}

/// A sentence that ends in an output word keeps its verb.
///
/// English tolerates the message-first order (`Hello world show`), and read
/// without care that order eats the last word of ordinary writing. A subject,
/// a modal, `to` or a conjunction in front of the output word settles which
/// one it is.
#[test]
fn a_sentence_ending_in_an_output_word_still_prints_whole() {
    for sentence in [
        "time will tell",
        "what did she say",
        "I have nothing to say",
        "that is what they say",
        "so they say",
        "or so they say",
        "go on and tell",
        "in the beginning there was light",
        "as far as I know she left",
        "and then it rained",
    ] {
        assert_eq!(
            ok(&format!("{sentence}\n")),
            format!("print(\"{sentence}\")\n"),
            "{sentence} stopped being a sentence"
        );
    }
    // The documented message-first order still works.
    assert_eq!(ok("Hello world show\n"), "print(\"Hello world\")\n");
}

/// A word English already has is never a misspelling of an NME word.
///
/// One typo is all that separates most short words, so without that rule
/// `shop milk` printed `milk`, `well done` printed `done` and `bell rings`
/// printed `rings`. Repair still catches what it is for.
#[test]
fn ordinary_words_are_not_read_as_typos() {
    for sentence in [
        "shop milk",
        "snow falls",
        "bell rings",
        "well done",
        "fell over",
        "sell it",
        "sad news",
        "sick day",
        "tall tree",
        "shot down",
        "saw it",
    ] {
        assert_eq!(
            ok(&format!("{sentence}\n")),
            format!("print(\"{sentence}\")\n"),
            "{sentence} stopped being a sentence"
        );
    }
    for typo in ["shwo hello", "sohw hello", "sya hello", "pirnt hello", "tel hello"] {
        assert_eq!(ok(&format!("{typo}\n")), "print(\"hello\")\n", "for {typo}");
    }
    // The rule may not cost the comparison words their own reading.
    assert_eq!(
        ok("set n to 1\nif n is less than 5\n  show low\nend\n"),
        "n = 1\nif (n < 5):\n  print(\"low\")\n# end\n",
    );
}
