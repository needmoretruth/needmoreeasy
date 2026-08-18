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

// ------------------------------------------- slow text, screen, and timing

/// The three long lines are written once here so every test below compares
/// against the exact same Python the compiler emits.
fn slowly(seconds: &str, text: &str) -> String {
    format!(
        "[print(_ch, end=\"\", flush=True) or __import__(\"time\").sleep({seconds}) for _ch in {text}]; print()\n"
    )
}

fn boxed(text: &str) -> String {
    format!(
        "print((lambda _t: (lambda _w: \"┌\" + \"─\" * (_w + 2) + \"┐\\n│ \" + _t + \" │\\n└\" + \"─\" * (_w + 2) + \"┘\")(sum(2 if __import__(\"unicodedata\").east_asian_width(_c) in \"WF\" else 1 for _c in _t)))({text}))\n"
    )
}

fn centred(text: &str) -> String {
    format!(
        "print((lambda _t: \" \" * max(0, (40 - sum(2 if __import__(\"unicodedata\").east_asian_width(_c) in \"WF\" else 1 for _c in _t)) // 2) + _t)({text}))\n"
    )
}

/// The exact Python every new statement produces, spelled out in full.
///
/// The helpers above build the same text from parts; this test is what pins
/// the parts down, so a change to any emitted line has to be made on purpose.
#[test]
fn the_new_statements_emit_exactly_this_python() {
    assert_eq!(
        ok("say slowly Hello\n"),
        "[print(_ch, end=\"\", flush=True) or __import__(\"time\").sleep(0.04) for _ch in \"Hello\"]; print()\n"
    );
    assert_eq!(
        ok("say very slowly Hello\n"),
        "[print(_ch, end=\"\", flush=True) or __import__(\"time\").sleep(0.12) for _ch in \"Hello\"]; print()\n"
    );
    assert_eq!(
        ok("say slowly every 3 seconds Hello\n"),
        "[print(_ch, end=\"\", flush=True) or __import__(\"time\").sleep(3) for _ch in \"Hello\"]; print()\n"
    );
    assert_eq!(
        ok("clear the screen\n"),
        "print(\"\\033[2J\\033[3J\\033[H\", end=\"\")\n"
    );
    assert_eq!(ok("draw a line\n"), "print(\"─\" * 40)\n");
    assert_eq!(
        ok("say in a box Hello\n"),
        "print((lambda _t: (lambda _w: \"┌\" + \"─\" * (_w + 2) + \"┐\\n│ \" + _t + \" │\\n└\" + \"─\" * (_w + 2) + \"┘\")(sum(2 if __import__(\"unicodedata\").east_asian_width(_c) in \"WF\" else 1 for _c in _t)))(\"Hello\"))\n"
    );
    assert_eq!(
        ok("say in the middle Hello\n"),
        "print((lambda _t: \" \" * max(0, (40 - sum(2 if __import__(\"unicodedata\").east_asian_width(_c) in \"WF\" else 1 for _c in _t)) // 2) + _t)(\"Hello\"))\n"
    );
    assert_eq!(
        ok("start the timer\n"),
        "_nme_clock = __import__(\"time\").time()\n"
    );
    assert_eq!(
        ok("start the timer\nshow elapsed\n"),
        "_nme_clock = __import__(\"time\").time()\nprint(round(__import__(\"time\").time() - _nme_clock, 2))\n"
    );
    assert_eq!(
        ok("put door on cooldown for 3 seconds\n"),
        "_nme_cool_door = __import__(\"time\").time() + 3\n"
    );
    assert_eq!(
        ok("when door is ready then show go\n"),
        "if (__import__(\"time\").time() >= _nme_cool_door): print(\"go\")\n"
    );
    assert_eq!(
        ok("when door is on cooldown then show wait\n"),
        "if (__import__(\"time\").time() < _nme_cool_door): print(\"wait\")\n"
    );
    assert_eq!(
        ok("wait for door\n"),
        "__import__(\"time\").sleep(max(0, _nme_cool_door - __import__(\"time\").time()))\n"
    );
}

#[test]
fn slow_text_works_in_both_languages() {
    assert_eq!(ok("say slowly Hello\n"), slowly("0.04", "\"Hello\""));
    assert_eq!(ok("show slowly Hello\n"), slowly("0.04", "\"Hello\""));
    assert_eq!(
        ok("천천히 말해줘 안녕하세요\n"),
        slowly("0.04", "\"안녕하세요\"")
    );
    assert_eq!(
        ok("천천히 말해 안녕하세요\n"),
        slowly("0.04", "\"안녕하세요\"")
    );
    assert_eq!(
        ok("천천히 보여줘 안녕하세요\n"),
        slowly("0.04", "\"안녕하세요\"")
    );
}

#[test]
fn slow_text_has_a_very_slow_spelling_in_both_languages() {
    assert_eq!(ok("say very slowly Hello\n"), slowly("0.12", "\"Hello\""));
    assert_eq!(
        ok("아주 천천히 말해줘 안녕하세요\n"),
        slowly("0.12", "\"안녕하세요\"")
    );
}

#[test]
fn slow_text_takes_an_explicit_interval_in_both_languages() {
    assert_eq!(
        ok("say slowly every 3 seconds Hello\n"),
        slowly("3", "\"Hello\"")
    );
    assert_eq!(
        ok("say slowly every 0.5 seconds Hello\n"),
        slowly("0.5", "\"Hello\"")
    );
    assert_eq!(
        ok("3초씩 천천히 말해줘 안녕하세요\n"),
        slowly("3", "\"안녕하세요\"")
    );
    assert_eq!(
        ok("0.5초씩 천천히 말해줘 안녕하세요\n"),
        slowly("0.5", "\"안녕하세요\"")
    );
}

#[test]
fn slow_text_reads_its_message_like_the_say_statement_does() {
    // A name written inside the sentence is substituted, and a value that is
    // not text is wrapped so it can be walked one character at a time.
    assert_eq!(
        ok("set name to Mina\nsay slowly Hello name\n"),
        format!(
            "name = \"Mina\"\n{}",
            slowly("0.04", "\"Hello \" + str(name)")
        )
    );
    assert_eq!(
        ok("점수는 7\n천천히 말해줘 점수\n"),
        format!("점수 = 7\n{}", slowly("0.04", "str(점수)"))
    );
}

#[test]
fn clearing_the_screen_works_in_both_languages() {
    let cleared = "print(\"\\033[2J\\033[3J\\033[H\", end=\"\")\n";
    assert_eq!(ok("clear the screen\n"), cleared);
    assert_eq!(ok("clear screen\n"), cleared);
    assert_eq!(ok("화면 지워\n"), cleared);
    assert_eq!(ok("화면 지워줘\n"), cleared);
    assert_eq!(ok("화면 비워\n"), cleared);
    assert_eq!(ok("화면 비워줘\n"), cleared);
}

#[test]
fn drawing_a_line_works_in_both_languages() {
    let rule = "print(\"─\" * 40)\n";
    assert_eq!(ok("draw a line\n"), rule);
    assert_eq!(ok("draw line\n"), rule);
    assert_eq!(ok("줄 그어\n"), rule);
    assert_eq!(ok("줄 그어줘\n"), rule);
    assert_eq!(ok("가로줄 그어\n"), rule);
    assert_eq!(ok("가로줄 그어줘\n"), rule);
}

#[test]
fn a_box_around_text_works_in_both_languages() {
    assert_eq!(ok("say in a box Hello\n"), boxed("\"Hello\""));
    assert_eq!(ok("상자로 말해줘 안녕\n"), boxed("\"안녕\""));
    assert_eq!(ok("상자로 말해 안녕\n"), boxed("\"안녕\""));
}

#[test]
fn a_box_counts_a_korean_character_as_two_columns() {
    // Without the width rule a Korean box comes out crooked, so the measure
    // has to be the terminal's own east-asian width, not `len`.
    let produced = ok("상자로 말해줘 안녕\n");
    assert!(
        produced.contains("east_asian_width(_c) in \"WF\""),
        "{produced}"
    );
    assert!(!produced.contains("len(_t)"), "{produced}");
}

#[test]
fn text_in_the_middle_works_in_both_languages() {
    assert_eq!(ok("say in the middle Hello\n"), centred("\"Hello\""));
    assert_eq!(ok("가운데 말해줘 안녕\n"), centred("\"안녕\""));
    assert_eq!(ok("가운데 말해 안녕\n"), centred("\"안녕\""));
}

#[test]
fn a_framed_message_is_worked_out_only_once() {
    // The width has to be measured from the same text that gets printed, so
    // a message that calls something may not be evaluated a second time.
    let produced = ok("set name to Mina\n가운데 말해줘 안녕 name\n");
    assert_eq!(produced.matches("str(name)").count(), 1, "{produced}");
    assert_eq!(
        produced,
        format!("name = \"Mina\"\n{}", centred("\"안녕 \" + str(name)"))
    );
}

#[test]
fn starting_the_timer_works_in_both_languages() {
    let started = "_nme_clock = __import__(\"time\").time()\n";
    assert_eq!(ok("start the timer\n"), started);
    assert_eq!(ok("start timer\n"), started);
    assert_eq!(ok("시간 재기 시작해\n"), started);
    assert_eq!(ok("시간재기 시작해\n"), started);
    assert_eq!(ok("시간 재기 시작\n"), started);
}

#[test]
fn the_stopwatch_reading_is_a_value_in_both_languages() {
    let reading = "round(__import__(\"time\").time() - _nme_clock, 2)";
    assert_eq!(
        ok("start the timer\nshow elapsed\n"),
        format!("_nme_clock = __import__(\"time\").time()\nprint({reading})\n")
    );
    assert_eq!(
        ok("start the timer\nset spent to elapsed\n"),
        format!("_nme_clock = __import__(\"time\").time()\nspent = {reading}\n")
    );
    assert_eq!(
        ok("시간 재기 시작해\n잰시간 말해줘\n"),
        format!("_nme_clock = __import__(\"time\").time()\nprint({reading})\n")
    );
    assert_eq!(
        ok("시간 재기 시작해\n걸린시간은 잰시간\n"),
        format!("_nme_clock = __import__(\"time\").time()\n걸린시간 = {reading}\n")
    );
}

#[test]
fn the_stopwatch_reading_is_also_a_value_in_a_condition() {
    let reading = "round(__import__(\"time\").time() - _nme_clock, 2)";
    assert_eq!(
        ok("시간 재기 시작해\n만약 잰시간이 3보다 크면 오래 말해줘\n"),
        format!(
            "_nme_clock = __import__(\"time\").time()\nif ({reading} > 3): print(\"오래\")\n"
        )
    );
    assert_eq!(
        ok("start the timer\nif elapsed is greater than 3 then show long\n"),
        format!(
            "_nme_clock = __import__(\"time\").time()\nif ({reading} > 3): print(\"long\")\n"
        )
    );
}

#[test]
fn reading_the_stopwatch_before_starting_it_is_reported() {
    assert_eq!(error_code("show elapsed\n"), "E0226");
    assert_eq!(error_code("잰시간 말해줘\n"), "E0226");
}

#[test]
fn a_name_of_your_own_beats_the_stopwatch_word() {
    assert_eq!(
        ok("set elapsed to 5\nshow elapsed\n"),
        "elapsed = 5\nprint(elapsed)\n"
    );
    assert_eq!(
        ok("잰시간은 5\n잰시간 말해줘\n"),
        "잰시간 = 5\nprint(잰시간)\n"
    );
}

#[test]
fn a_cooldown_is_set_in_both_languages() {
    assert_eq!(
        ok("put door on cooldown for 3 seconds\n"),
        "_nme_cool_door = __import__(\"time\").time() + 3\n"
    );
    assert_eq!(
        ok("문 쿨타임 3초 걸어\n"),
        "_nme_cool_문 = __import__(\"time\").time() + 3\n"
    );
    assert_eq!(
        ok("문 쿨타임 3초 걸어줘\n"),
        "_nme_cool_문 = __import__(\"time\").time() + 3\n"
    );
}

#[test]
fn a_finished_cooldown_is_a_condition_in_both_languages() {
    let ready = "if (__import__(\"time\").time() >= _nme_cool_door):";
    assert_eq!(
        ok("when door is ready\n    show go\nend\n"),
        format!("{ready}\n    print(\"go\")\n# end\n")
    );
    assert_eq!(
        ok("if door is ready\n    show go\nend\n"),
        format!("{ready}\n    print(\"go\")\n# end\n")
    );
    let ready_ko = "if (__import__(\"time\").time() >= _nme_cool_문):";
    assert_eq!(
        ok("만약 문 쿨타임이 끝났으면\n    발사 말해줘\n끝\n"),
        format!("{ready_ko}\n    print(\"발사\")\n# end\n")
    );
    assert_eq!(
        ok("문 쿨타임 끝났으면 발사 말해줘\n"),
        format!("{ready_ko} print(\"발사\")\n")
    );
}

#[test]
fn a_running_cooldown_is_a_condition_in_both_languages() {
    let busy = "if (__import__(\"time\").time() < _nme_cool_door):";
    assert_eq!(
        ok("when door is on cooldown\n    show wait\nend\n"),
        format!("{busy}\n    print(\"wait\")\n# end\n")
    );
    let busy_ko = "if (__import__(\"time\").time() < _nme_cool_문):";
    assert_eq!(
        ok("만약 문 쿨타임이 남았으면\n    대기 말해줘\n끝\n"),
        format!("{busy_ko}\n    print(\"대기\")\n# end\n")
    );
    assert_eq!(
        ok("문 쿨타임 남았으면 대기 말해줘\n"),
        format!("{busy_ko} print(\"대기\")\n")
    );
}

#[test]
fn cooldown_conditions_work_in_while_and_else_if() {
    assert_eq!(
        ok("while door is on cooldown\n    show waiting\nend\n"),
        "while (__import__(\"time\").time() < _nme_cool_door):\n    print(\"waiting\")\n# end\n"
    );
    assert_eq!(
        ok("when door is ready\n    show a\nelse if door is on cooldown\n    show b\nend\n"),
        concat!(
            "if (__import__(\"time\").time() >= _nme_cool_door):\n",
            "    print(\"a\")\n",
            "elif (__import__(\"time\").time() < _nme_cool_door):\n",
            "    print(\"b\")\n",
            "# end\n"
        )
    );
    assert_eq!(
        ok("문 쿨타임이 남았으면 동안\n    대기 말해줘\n끝\n"),
        "while (__import__(\"time\").time() < _nme_cool_문):\n    print(\"대기\")\n# end\n"
    );
}

#[test]
fn waiting_out_a_cooldown_works_in_both_languages() {
    let slept =
        "__import__(\"time\").sleep(max(0, _nme_cool_문 - __import__(\"time\").time()))\n";
    assert_eq!(
        ok("wait for door\n"),
        "__import__(\"time\").sleep(max(0, _nme_cool_door - __import__(\"time\").time()))\n"
    );
    assert_eq!(ok("문 쿨타임 끝날때까지 기다려\n"), slept);
    assert_eq!(ok("문 쿨타임 끝날 때까지 기다려\n"), slept);
}

#[test]
fn the_new_words_stay_words_inside_a_message() {
    assert_eq!(
        ok("show the screen is clear\n"),
        "print(\"the screen is clear\")\n"
    );
    assert_eq!(
        ok("화면 지워도 되는지 말해줘\n"),
        "print(\"화면 지워도 되는지\")\n"
    );
    assert_eq!(
        ok("줄 그어도 되는지 말해줘\n"),
        "print(\"줄 그어도 되는지\")\n"
    );
    assert_eq!(
        ok("show I will draw a line later\n"),
        "print(\"I will draw a line later\")\n"
    );
    assert_eq!(
        ok("가운데 자리 좋다고 말해줘\n"),
        "print(\"가운데 자리 좋다고\")\n"
    );
}

#[test]
fn a_wait_still_wins_over_the_new_openers() {
    // The old statements keep their lines: none of the new opening words is
    // allowed to change what these mean.
    assert_eq!(
        ok("wait for 5 seconds\n"),
        "__import__(\"time\").sleep(5)\n"
    );
    assert_eq!(ok("3초 기다려\n"), "__import__(\"time\").sleep(3)\n");
    assert_eq!(ok("잠깐 기다려\n"), "print(\"잠깐 기다려\")\n");
    assert_eq!(ok("화면에 1 더해\n"), "화면 = 화면 + 1\n");
}

#[test]
fn a_slow_message_may_contain_a_waiting_word() {
    assert_eq!(
        ok("천천히 말해줘 3초 기다려\n"),
        slowly("0.04", "\"3초 기다려\"")
    );
    assert_eq!(
        ok("say slowly wait 3 seconds\n"),
        slowly("0.04", "\"wait 3 seconds\"")
    );
}

#[test]
fn an_unreadable_cooldown_length_is_reported() {
    assert_eq!(
        error_code("put door on cooldown for 3 4 5 seconds\n"),
        "E0224"
    );
    assert_eq!(error_code("문 쿨타임 3 4 5초 걸어\n"), "E0224");
}

#[test]
fn every_new_statement_is_still_exactly_one_python_line() {
    let program = concat!(
        "start the timer\n",
        "clear the screen\n",
        "draw a line\n",
        "say in a box Hello\n",
        "say in the middle Hello\n",
        "say slowly Hello\n",
        "show elapsed\n",
        "put door on cooldown for 3 seconds\n",
        "wait for door\n",
        "화면 지워\n",
        "줄 그어\n",
        "상자로 말해줘 안녕\n",
        "가운데 말해줘 안녕\n",
        "천천히 말해줘 안녕\n",
        "잰시간 말해줘\n",
        "문 쿨타임 3초 걸어\n",
        "문 쿨타임 끝날때까지 기다려\n",
    );
    let produced = ok(program);
    assert_eq!(produced.lines().count(), program.lines().count());
}
