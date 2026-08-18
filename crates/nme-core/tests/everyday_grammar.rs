//! The everyday statements a first program needs beyond say/ask/set/repeat:
//! waiting, repeating over a list, skipping a round, multiplying and dividing,
//! making a list, and adding to one.
//!
//! Each of them must exist at the sentence level in **both** languages, and
//! each must lower to exactly one line of Python.

use nme_core::transpile;

fn ok(source: &str) -> String {
    transpile(source)
        .unwrap_or_else(|problems| panic!("expected successful transpile, got: {problems:?}"))
}

fn error_code(source: &str) -> String {
    let problems = transpile(source).expect_err("expected this line to be rejected");
    problems[0].code.code().to_string()
}

// ------------------------------------------------------------------ waiting

#[test]
fn waiting_works_in_both_languages() {
    assert_eq!(ok("wait 3 seconds\n"), "__import__(\"time\").sleep(3)\n");
    assert_eq!(ok("pause 2\n"), "__import__(\"time\").sleep(2)\n");
    assert_eq!(ok("sleep 1\n"), "__import__(\"time\").sleep(1)\n");
    assert_eq!(
        ok("wait for 5 seconds\n"),
        "__import__(\"time\").sleep(5)\n"
    );
    assert_eq!(ok("3초 기다려\n"), "__import__(\"time\").sleep(3)\n");
    assert_eq!(ok("3초 쉬어\n"), "__import__(\"time\").sleep(3)\n");
    assert_eq!(ok("2 초 기다려주세요\n"), "__import__(\"time\").sleep(2)\n");
}

#[test]
fn waiting_accepts_a_saved_number() {
    assert_eq!(
        ok("쉬는시간은 3\n쉬는시간 기다려\n"),
        "쉬는시간 = 3\n__import__(\"time\").sleep(쉬는시간)\n"
    );
}

#[test]
fn a_wait_word_without_a_number_stays_ordinary_speech() {
    assert_eq!(ok("잠깐 기다려\n"), "print(\"잠깐 기다려\")\n");
}

#[test]
fn an_unreadable_wait_length_is_reported() {
    assert_eq!(error_code("wait 3 4 5 seconds\n"), "E0224");
}

// -------------------------------------------------------- repeat over a list

#[test]
fn repeating_over_a_list_works_in_both_languages() {
    assert_eq!(
        ok("for each friend in friends: show friend\n"),
        "for friend in friends: print(friend)\n"
    );
    assert_eq!(
        ok("친구들의 친구마다 반복해서 친구 말해줘\n"),
        "for 친구 in 친구들: print(친구)\n"
    );
    assert_eq!(
        ok("이름들에서 이름마다 반복해서 이름 말해줘\n"),
        "for 이름 in 이름들: print(이름)\n"
    );
}

#[test]
fn a_list_loop_opens_a_block_closed_by_end() {
    assert_eq!(
        ok("set friends to list of Mina, Ada\nfor each friend in friends\nshow Hello friend\nend\n"),
        "friends = [\"Mina\", \"Ada\"]\nfor friend in friends:\n    print(\"Hello \" + str(friend))\n# end\n"
    );
    assert_eq!(
        ok("친구들은 목록 민수, 지안\n친구들의 친구마다 반복해\n안녕 친구 말해줘\n끝\n"),
        "친구들 = [\"민수\", \"지안\"]\nfor 친구 in 친구들:\n    print(\"안녕 \" + str(친구))\n# end\n"
    );
}

#[test]
fn ordinary_speech_ending_in_mada_is_not_a_loop() {
    assert_eq!(ok("날마다 반가워요\n"), "print(\"날마다 반가워요\")\n");
}

// --------------------------------------------------------------- skip a round

#[test]
fn skipping_a_round_works_in_both_languages() {
    assert_eq!(
        ok("repeat 3 times\nskip\nend\n"),
        "for _ in range(3):\n    continue\n# end\n"
    );
    assert_eq!(
        ok("3번 반복해\n건너뛰어\n끝\n"),
        "for _ in range(3):\n    continue\n# end\n"
    );
}

#[test]
fn skip_outside_a_block_stays_ordinary_python() {
    // A bare name is valid Python, and Python always wins.
    assert_eq!(ok("skip\n"), "skip\n");
}

// -------------------------------------------------- multiplying and dividing

#[test]
fn multiplying_and_dividing_work_in_both_languages() {
    assert_eq!(ok("multiply score by 2\n"), "score = score * 2\n");
    assert_eq!(ok("divide score by 2\n"), "score = score / 2\n");
    assert_eq!(ok("score multiply 3\n"), "score = score * 3\n");
    assert_eq!(ok("점수에 2 곱해\n"), "점수 = 점수 * 2\n");
    assert_eq!(ok("점수를 2로 나눠\n"), "점수 = 점수 / 2\n");
}

#[test]
fn a_multi_token_amount_keeps_its_own_arithmetic() {
    // Without the parentheses Python would read `score - 1 + 2`.
    assert_eq!(ok("subtract 1 + 2 from score\n"), "score = score - (1 + 2)\n");
    assert_eq!(ok("점수에서 1 + 2 빼줘\n"), "점수 = 점수 - (1 + 2)\n");
    assert_eq!(ok("점수에 1 더해\n"), "점수 = 점수 + 1\n");
}

#[test]
fn times_still_means_repeat_not_multiply() {
    assert_eq!(
        ok("3 times: say \"hi\"\n"),
        "for _ in range(3): print(\"hi\")\n"
    );
}

// --------------------------------------------------------- lists and adding

#[test]
fn making_a_list_works_in_both_languages() {
    assert_eq!(
        ok("set friends to list of Mina, Ada and Grace\n"),
        "friends = [\"Mina\", \"Ada\", \"Grace\"]\n"
    );
    assert_eq!(
        ok("친구들은 목록 민수, 지안, 서준\n"),
        "친구들 = [\"민수\", \"지안\", \"서준\"]\n"
    );
    assert_eq!(
        ok("친구들은 목록 민수와 지안\n"),
        "친구들 = [\"민수\", \"지안\"]\n"
    );
    assert_eq!(ok("점수들은 목록 1, 2, 3\n"), "점수들 = [1, 2, 3]\n");
}

#[test]
fn adding_to_a_list_works_in_both_languages() {
    assert_eq!(ok("append Mina to friends\n"), "friends.append(\"Mina\")\n");
    assert_eq!(ok("친구들에 민수 넣어\n"), "친구들.append(\"민수\")\n");
    assert_eq!(ok("친구들에 민수 추가해\n"), "친구들.append(\"민수\")\n");
}

#[test]
fn ordinary_speech_is_not_a_list_addition() {
    assert_eq!(ok("설탕을 넣어\n"), "print(\"설탕을 넣어\")\n");
}

#[test]
fn add_still_means_a_value_change() {
    assert_eq!(ok("add 1 to score\n"), "score = score + 1\n");
}

// ----------------------------------------------------- comparison lowering

#[test]
fn a_negated_equality_lowers_to_not_equals() {
    assert_eq!(
        ok("만약에 점수가 5와 같지 않으면 달라요 말해줘\n"),
        "if (점수 != 5): print(\"달라요\")\n"
    );
    assert_eq!(
        ok("if score is not equal to 5 then show different\n"),
        "if (score != 5): print(\"different\")\n"
    );
}

#[test]
fn a_number_choice_stays_a_number() {
    assert_eq!(
        ok("주사위는 1 또는 2 중에서 랜덤선택\n"),
        "주사위 = __import__(\"random\").choice((1, 2,))\n"
    );
}

#[test]
fn an_age_question_reads_a_number() {
    assert_eq!(
        ok("몇 살이에요?\n"),
        "나이 = int(input(\"몇 살이에요?\" + \" \"))\n"
    );
    assert_eq!(
        ok("How old are you?\n"),
        "age = int(input(\"How old are you?\" + \" \"))\n"
    );
}

#[test]
fn a_comma_list_keeps_a_word_ending_in_the_joiner() {
    // `사과` ends in `과`, the Korean word for `and`. A comma has already
    // shown the separator, so the fruit must survive whole.
    assert_eq!(
        ok("과일들은 목록 사과, 바나나, 포도\n"),
        "과일들 = [\"사과\", \"바나나\", \"포도\"]\n"
    );
    assert_eq!(
        ok("친구들은 목록 민수와 지안\n"),
        "친구들 = [\"민수\", \"지안\"]\n"
    );
}

#[test]
fn arithmetic_words_inside_a_message_stay_words() {
    assert_eq!(
        ok("show I will multiply by 2\n"),
        "print(\"I will multiply by 2\")\n"
    );
    assert_eq!(
        ok("ask number factor What should I multiply by\n"),
        "factor = int(input(\"What should I multiply by\" + \" \"))\n"
    );
    assert_eq!(
        ok("2 곱해 말해줘\n"),
        "print(\"2 곱해\")\n"
    );
}

#[test]
fn a_one_line_body_may_skip_a_round() {
    assert_eq!(
        ok("repeat 3 times\nif 1 equals 1 then skip\nend\n"),
        "for _ in range(3):\n    if (1 == 1): continue\n# end\n"
    );
    assert_eq!(
        ok("3번 반복해\n만약에 1이 1과 같으면 건너뛰어\n끝\n"),
        "for _ in range(3):\n    if (1 == 1): continue\n# end\n"
    );
}

#[test]
fn a_one_line_body_may_change_a_value_in_korean() {
    // The English form always worked; the Korean line was swallowed by the
    // value-change matcher before the condition was ever read.
    assert_eq!(
        ok("점수는 3\n만약에 점수가 5보다 크면 점수에 1 더해\n"),
        "점수 = 3\nif (점수 > 5): 점수 = 점수 + 1\n"
    );
}

#[test]
fn a_list_loop_needs_no_colon() {
    assert_eq!(
        ok("친구들은 목록 민수\nfor each friend in 친구들 and show friend\n"),
        "친구들 = [\"민수\"]\nfor friend in 친구들: print(friend)\n"
    );
}

#[test]
fn a_message_may_contain_a_waiting_word() {
    assert_eq!(
        ok("set tired to False\nif tired then show Time to sleep\n"),
        "tired = False\nif (tired): print(\"Time to sleep\")\n"
    );
}
