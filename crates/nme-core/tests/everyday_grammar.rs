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
fn skip_outside_a_block_says_it_needs_a_loop() {
    // `skip` used to be left alone as an ordinary Python name, on the grounds
    // that Python always wins a bare name. It ran, and died with `NameError:
    // name 'skip' is not defined` on a line the writer had read as a command.
    // Now it is read as the skip word it is, and skipping outside a loop is
    // the thing that gets explained.
    assert_eq!(error_code("skip\n"), "E0107");
    assert_eq!(error_code("건너뛰어\n"), "E0107");
    // A name the program made is still Python doing nothing.
    assert_eq!(ok("skip = 1\nskip\n"), "skip = 1\nskip\n");
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
    assert_eq!(
        ok("subtract 1 + 2 from score\n"),
        "score = score - (1 + 2)\n"
    );
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
    assert_eq!(ok("2 곱해 말해줘\n"), "print(\"2 곱해\")\n");
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
        format!("_nme_clock = __import__(\"time\").time()\nif ({reading} > 3): print(\"오래\")\n")
    );
    assert_eq!(
        ok("start the timer\nif elapsed is greater than 3 then show long\n"),
        format!("_nme_clock = __import__(\"time\").time()\nif ({reading} > 3): print(\"long\")\n")
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
    let slept = "__import__(\"time\").sleep(max(0, _nme_cool_문 - __import__(\"time\").time()))\n";
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

// -------- 1. `wait`, `append`, `break`, `skip` and `for each` recover from one typo

#[test]
fn wait_append_break_and_skip_recover_from_one_typo() {
    // t-en-13
    assert_eq!(ok("waite 2 seconds\n"), "__import__(\"time\").sleep(2)\n");
    // t-en-14
    assert_eq!(ok("wat 2 seconds\n"), "__import__(\"time\").sleep(2)\n");
    // t-en-15
    assert_eq!(ok("wiat 2 seconds\n"), "__import__(\"time\").sleep(2)\n");
    // t-en-16
    assert_eq!(ok("waitt 2 seconds\n"), "__import__(\"time\").sleep(2)\n");
    // t-en-19
    assert_eq!(ok("pasue 2\n"), "__import__(\"time\").sleep(2)\n");
    // t-en-20
    assert_eq!(ok("slep 2\n"), "__import__(\"time\").sleep(2)\n");
    // t-en-21
    assert_eq!(ok("sleeep 2\n"), "__import__(\"time\").sleep(2)\n");
    // m-en-05
    assert_eq!(ok("waitt 3\n"), "__import__(\"time\").sleep(3)\n");
    // t-ko-11
    assert_eq!(ok("2초 기다러\n"), "__import__(\"time\").sleep(2)\n");
    // t-ko-12
    assert_eq!(ok("2초 기달려\n"), "__import__(\"time\").sleep(2)\n");
    // t-ko-13
    assert_eq!(ok("2초 기디려\n"), "__import__(\"time\").sleep(2)\n");
    // t-ko-14
    assert_eq!(ok("2초 기다려어\n"), "__import__(\"time\").sleep(2)\n");
    // t-ko-15
    assert_eq!(ok("2초 다려\n"), "__import__(\"time\").sleep(2)\n");
    // t-ko-16
    assert_eq!(ok("2초 쉬여\n"), "__import__(\"time\").sleep(2)\n");
    // m-ko-05
    assert_eq!(ok("3초 기다랴\n"), "__import__(\"time\").sleep(3)\n");
    // s-ko-11
    assert_eq!(ok("2초기다려\n"), "__import__(\"time\").sleep(2)\n");
    // t2-ko-01
    assert_eq!(ok("2초기다려\n"), "__import__(\"time\").sleep(2)\n");
    // t-en-48
    assert_eq!(
        ok("repeat 3 times\n  brek\nend\n"),
        "for _ in range(3):\n  break\n# end\n"
    );
    // t-en-49
    assert_eq!(
        ok("repeat 3 times\n  braek\nend\n"),
        "for _ in range(3):\n  break\n# end\n"
    );
    // m-en-03
    assert_eq!(
        ok("repeat 3 times\n  breakk\nend\n"),
        "for _ in range(3):\n  break\n# end\n"
    );
    // t-ko-36
    assert_eq!(
        ok("3번 반복해\n  멈처\n끝\n"),
        "for _ in range(3):\n  break\n# end\n"
    );
    // t-ko-37
    assert_eq!(
        ok("3번 반복해\n  머춰\n끝\n"),
        "for _ in range(3):\n  break\n# end\n"
    );
    // t-en-50
    assert_eq!(
        ok("repeat 3 times\n  skipp\nend\n"),
        "for _ in range(3):\n  continue\n# end\n"
    );
    // t-en-51
    assert_eq!(
        ok("repeat 3 times\n  skp\nend\n"),
        "for _ in range(3):\n  continue\n# end\n"
    );
    // m-en-04
    assert_eq!(
        ok("repeat 3 times\n  sikp\nend\n"),
        "for _ in range(3):\n  continue\n# end\n"
    );
    // t-ko-38
    assert_eq!(
        ok("3번 반복해\n  건너뛰여\n끝\n"),
        "for _ in range(3):\n  continue\n# end\n"
    );
    // t-ko-39
    assert_eq!(
        ok("3번 반복해\n  건너띄어\n끝\n"),
        "for _ in range(3):\n  continue\n# end\n"
    );
    // t-en-52
    assert_eq!(
        ok("set friends to list of Mina\nappned Mina to friends\n"),
        "friends = [\"Mina\"]\nfriends.append(\"Mina\")\n"
    );
    // t-en-53
    assert_eq!(
        ok("set friends to list of Mina\napend Mina to friends\n"),
        "friends = [\"Mina\"]\nfriends.append(\"Mina\")\n"
    );
    // m-en-01
    assert_eq!(
        ok("set friends to list of Mina\nappendd Mina to friends\n"),
        "friends = [\"Mina\"]\nfriends.append(\"Mina\")\n"
    );
    // m-en-02
    assert_eq!(
        ok("set friends to list of Mina\npsuh Mina to friends\n"),
        "friends = [\"Mina\"]\nfriends.append(\"Mina\")\n"
    );
    // t-ko-40
    assert_eq!(
        ok("친구들은 목록 민수\n친구들에 민수 너어\n"),
        "친구들 = [\"민수\"]\n친구들.append(\"민수\")\n"
    );
    // t-ko-41
    assert_eq!(
        ok("친구들은 목록 민수\n친구들에 민수 추가헤\n"),
        "친구들 = [\"민수\"]\n친구들.append(\"민수\")\n"
    );
    // t-en-54
    assert_eq!(
        ok("for eahc friend in friends\n  show friend\nend\n"),
        "for friend in friends:\n  print(friend)\n# end\n"
    );
    // t-en-26
    assert_eq!(
        ok("repeat 3 tiems and show hello\n"),
        "for _ in range(3): print(\"hello\")\n"
    );
    // t-ko-22
    assert_eq!(
        ok("3번 반복헤\n  안녕 말해줘\n끝\n"),
        "for _ in range(3):\n  print(\"안녕\")\n# end\n"
    );
}

// -------- 2. Number words and unit words

#[test]
fn counting_words_and_counter_words_are_numbers() {
    // n-en-05
    assert_eq!(ok("wait two seconds\n"), "__import__(\"time\").sleep(2)\n");
    // n-en-06
    assert_eq!(ok("wait one second\n"), "__import__(\"time\").sleep(1)\n");
    // k2-en-02
    assert_eq!(
        ok("wait three seconds\n"),
        "__import__(\"time\").sleep(3)\n"
    );
    // n-en-10
    assert_eq!(
        ok("repeat three times and show hello\n"),
        "for _ in range(3): print(\"hello\")\n"
    );
    // k2-en-03
    assert_eq!(
        ok("repeat five times and show hello\n"),
        "for _ in range(5): print(\"hello\")\n"
    );
    // n-en-11
    assert_eq!(
        ok("repeat once and show hello\n"),
        "for _ in range(1): print(\"hello\")\n"
    );
    // n-en-12
    assert_eq!(
        ok("repeat twice and show hello\n"),
        "for _ in range(2): print(\"hello\")\n"
    );
    // n-en-08
    assert_eq!(
        ok("repeat 1 time and show hello\n"),
        "for _ in range(1): print(\"hello\")\n"
    );
    // n-en-09
    assert_eq!(
        ok("repeat 3 time and show hello\n"),
        "for _ in range(3): print(\"hello\")\n"
    );
    // s2-en-02
    assert_eq!(
        ok("repeat 3 time and show hello\n"),
        "for _ in range(3): print(\"hello\")\n"
    );
    // s2-en-03
    assert_eq!(
        ok("repeat 3 loops and show hello\n"),
        "for _ in range(3): print(\"hello\")\n"
    );
    // n-ko-02
    assert_eq!(
        ok("한 번 반복해서 안녕 말해줘\n"),
        "for _ in range(1): print(\"안녕\")\n"
    );
    // n-ko-03
    assert_eq!(
        ok("한번 반복해서 안녕 말해줘\n"),
        "for _ in range(1): print(\"안녕\")\n"
    );
    // n-ko-04
    assert_eq!(
        ok("세 번 반복해서 안녕 말해줘\n"),
        "for _ in range(3): print(\"안녕\")\n"
    );
    // n-ko-05
    assert_eq!(
        ok("세번 반복해서 안녕 말해줘\n"),
        "for _ in range(3): print(\"안녕\")\n"
    );
    // n-ko-06
    assert_eq!(
        ok("두 번 반복해서 안녕 말해줘\n"),
        "for _ in range(2): print(\"안녕\")\n"
    );
    // k2-ko-03
    assert_eq!(
        ok("세 번 반복해\n  안녕 말해줘\n끝\n"),
        "for _ in range(3):\n  print(\"안녕\")\n# end\n"
    );
    // k2-ko-04
    assert_eq!(
        ok("다섯 번 반복해서 안녕 말해줘\n"),
        "for _ in range(5): print(\"안녕\")\n"
    );
    // n-ko-07
    assert_eq!(
        ok("삼회 반복해서 안녕 말해줘\n"),
        "for _ in range(3): print(\"안녕\")\n"
    );
    // n-ko-08
    assert_eq!(
        ok("3회 반복해서 안녕 말해줘\n"),
        "for _ in range(3): print(\"안녕\")\n"
    );
    // s2-ko-02
    assert_eq!(
        ok("3회 반복해서 안녕 말해줘\n"),
        "for _ in range(3): print(\"안녕\")\n"
    );
    // s2-ko-03
    assert_eq!(
        ok("3차례 반복해서 안녕 말해줘\n"),
        "for _ in range(3): print(\"안녕\")\n"
    );
    // s2-ko-04
    assert_eq!(
        ok("3판 반복해서 안녕 말해줘\n"),
        "for _ in range(3): print(\"안녕\")\n"
    );
    // n-ko-09
    assert_eq!(ok("일초 기다려\n"), "__import__(\"time\").sleep(1)\n");
    // n-ko-10
    assert_eq!(ok("이초 기다려\n"), "__import__(\"time\").sleep(2)\n");
    // k2-ko-02
    assert_eq!(ok("삼초 기다려\n"), "__import__(\"time\").sleep(3)\n");
}

// -------- 3. Korean value-setting is as wide as English

#[test]
fn korean_value_setting_matches_english() {
    // k-ko-02
    assert_eq!(ok("점수를 0으로\n"), "점수 = 0\n");
    // k-ko-04
    assert_eq!(ok("점수가 0\n"), "점수 = 0\n");
    // b-ko-09
    assert_eq!(ok("점수를 0\n"), "점수 = 0\n");
    // k-ko-07
    assert_eq!(ok("점수에 0 저장해\n"), "점수 = 0\n");
    // k-ko-08
    assert_eq!(ok("점수를 0으로 설정해\n"), "점수 = 0\n");
    // k-ko-09
    assert_eq!(ok("점수를 0으로 저장해줘\n"), "점수 = 0\n");
    // y-ko-19
    assert_eq!(ok("점수를 0으로 지정해\n"), "점수 = 0\n");
    // y-ko-20
    assert_eq!(ok("점수를 0으로 정해\n"), "점수 = 0\n");
    // y-ko-21
    assert_eq!(ok("점수를 0으로 만들어\n"), "점수 = 0\n");
    // k-ko-05
    assert_eq!(ok("점수는 0이다\n"), "점수 = 0\n");
    // k-ko-06
    assert_eq!(ok("점수는 0입니다\n"), "점수 = 0\n");
    // b-ko-08
    assert_eq!(ok("점수는 0으로\n"), "점수 = 0\n");
}

// -------- 4. Korean typo recovery is no longer switched off by its own word list

#[test]
fn korean_output_typos_recover_like_english() {
    // t-ko-02
    assert_eq!(ok("말해조 안녕\n"), "print(\"안녕\")\n");
    // t-ko-03
    assert_eq!(ok("마해줘 안녕\n"), "print(\"안녕\")\n");
    // t-ko-04
    assert_eq!(ok("말해쥐 안녕\n"), "print(\"안녕\")\n");
    // t-ko-05
    assert_eq!(ok("말해주 안녕\n"), "print(\"안녕\")\n");
    // n2-ko-02
    assert_eq!(ok("말해조 안녕\n"), "print(\"안녕\")\n");
    // n2-ko-04
    assert_eq!(ok("안녕 말해조\n"), "print(\"안녕\")\n");
    // n2-ko-05
    assert_eq!(ok("안녕 마해줘\n"), "print(\"안녕\")\n");
    // n2-ko-06
    assert_eq!(ok("안녕 말해쥐\n"), "print(\"안녕\")\n");
    // k-ko-12
    assert_eq!(ok("말해라 안녕\n"), "print(\"안녕\")\n");
    // b-ko-03
    assert_eq!(ok("점수 저장헤 0\n"), "점수 = 0\n");
    // t-ko-24
    assert_eq!(ok("점수 저장헤 0\n"), "점수 = 0\n");
    // t-ko-28
    assert_eq!(
        ok("이름을 물어봐아 이름이 뭐예요?\n"),
        "이름 = input(\"이름이 뭐예요?\" + \" \")\n"
    );
    // l-ko-05
    assert_eq!(
        ok("이름을 물어봐줘요 이름이 뭐예요?\n"),
        "이름 = input(\"이름이 뭐예요?\" + \" \")\n"
    );
    // t-ko-32
    assert_eq!(
        ok("점수는 0\n점수에 1 더해에\n"),
        "점수 = 0\n점수 = 점수 + 1\n"
    );
    // t-ko-42
    assert_eq!(
        ok("점수는 0\n만약게 점수가 10보다 크면 성공 말해줘\n"),
        "점수 = 0\nif (점수 > 10): print(\"성공\")\n"
    );
    // t-ko-31
    assert_eq!(error_code("점수는 0\n점수에 1 대해\n"), "E0601");
}

// -------- 5. Trailing punctuation

#[test]
fn trailing_punctuation_is_punctuation() {
    // p-en-06
    assert_eq!(ok("set score to 0.\n"), "score = 0\n");
    // p-en-07
    assert_eq!(ok("set score to 0;\n"), "score = 0\n");
    // p-en-05
    assert_eq!(error_code("repeat 3 times.\n"), "E0501");
    // p-ko-09
    assert_eq!(ok("점수는 0.\n"), "점수 = 0\n");
    // p-ko-10
    assert_eq!(ok("점수는 0;\n"), "점수 = 0\n");
    // e-en-06
    assert_eq!(
        ok("repeat 3 times.\n  show hello\nend\n"),
        "for _ in range(3):\n  print(\"hello\")\n# end\n"
    );
    // e-ko-05
    assert_eq!(
        ok("3번 반복해.\n  안녕 말해줘\n끝\n"),
        "for _ in range(3):\n  print(\"안녕\")\n# end\n"
    );
    // e-en-05
    assert_eq!(error_code("repeat 3 times.\n"), "E0501");
    // e-ko-04
    assert_eq!(error_code("3번 반복해.\n"), "E0501");
}

// -------- 6. Loop control and block closing synonyms

#[test]
fn loop_control_and_block_closing_synonyms() {
    // i-en-05
    assert_eq!(
        ok("repeat 3 times\n  stop\nend\n"),
        "for _ in range(3):\n  break\n# end\n"
    );
    // o2-ko-06
    assert_eq!(
        ok("3번 반복해\n  멈춰줘\n끝\n"),
        "for _ in range(3):\n  break\n# end\n"
    );
    // y-en-12
    assert_eq!(
        ok("repeat 3 times\n  stop here\nend\n"),
        "for _ in range(3):\n  break\n# end\n"
    );
    // y-en-13
    assert_eq!(
        ok("repeat 3 times\n  exit loop\nend\n"),
        "for _ in range(3):\n  break\n# end\n"
    );
    // y-en-15
    assert_eq!(
        ok("repeat 3 times\n  quit\nend\n"),
        "for _ in range(3):\n  break\n# end\n"
    );
    // y-ko-01
    assert_eq!(
        ok("3번 반복해\n  그만해\n끝\n"),
        "for _ in range(3):\n  break\n# end\n"
    );
    // y-ko-02
    assert_eq!(
        ok("3번 반복해\n  멈추기\n끝\n"),
        "for _ in range(3):\n  break\n# end\n"
    );
    // y-ko-03
    assert_eq!(
        ok("3번 반복해\n  정지해\n끝\n"),
        "for _ in range(3):\n  break\n# end\n"
    );
    // y-ko-04
    assert_eq!(
        ok("3번 반복해\n  종료해\n끝\n"),
        "for _ in range(3):\n  break\n# end\n"
    );
    // i-ko-05
    assert_eq!(
        ok("3번 반복해\n  멈춰줘\n끝\n"),
        "for _ in range(3):\n  break\n# end\n"
    );
    // y-en-11
    assert_eq!(
        ok("repeat 3 times\n  keep going\nend\n"),
        "for _ in range(3):\n  continue\n# end\n"
    );
    // y-ko-05
    assert_eq!(
        ok("3번 반복해\n  계속해\n끝\n"),
        "for _ in range(3):\n  continue\n# end\n"
    );
    // y-ko-06
    assert_eq!(
        ok("3번 반복해\n  넘겨\n끝\n"),
        "for _ in range(3):\n  continue\n# end\n"
    );
    // y-ko-07
    assert_eq!(
        ok("3번 반복해\n  다음\n끝\n"),
        "for _ in range(3):\n  continue\n# end\n"
    );
    // y-en-30
    assert_eq!(
        ok("repeat 3 times\n  show hello\nfinish\n"),
        "for _ in range(3):\n  print(\"hello\")\n# end\n"
    );
    // y-en-31
    assert_eq!(
        ok("repeat 3 times\n  show hello\ndone\n"),
        "for _ in range(3):\n  print(\"hello\")\n# end\n"
    );
    // y-ko-26
    assert_eq!(
        ok("3번 반복해\n  안녕 말해줘\n종료\n"),
        "for _ in range(3):\n  print(\"안녕\")\n# end\n"
    );
    // y-ko-27
    assert_eq!(
        ok("3번 반복해\n  안녕 말해줘\n마침\n"),
        "for _ in range(3):\n  print(\"안녕\")\n# end\n"
    );
}

// -------- 7. Word order

#[test]
fn word_order_that_can_only_mean_one_thing() {
    // o-en-09
    assert_eq!(
        ok("set score to 0\nto score add 1\n"),
        "score = 0\nscore = score + 1\n"
    );
    // o-en-12
    assert_eq!(
        ok("set score to 0\nby 1 increase score\n"),
        "score = 0\nscore = score + 1\n"
    );
    // o-ko-05
    assert_eq!(
        ok("점수는 0\n1을 점수에 더해\n"),
        "점수 = 0\n점수 = 점수 + 1\n"
    );
    // k-ko-29
    assert_eq!(
        ok("점수는 0\n점수에다 1 더해\n"),
        "점수 = 0\n점수 = 점수 + 1\n"
    );
    // o-en-20
    assert_eq!(
        ok("set friends to list of Mina\nto friends append Mina\n"),
        "friends = [\"Mina\"]\nfriends.append(\"Mina\")\n"
    );
    // o-ko-12
    assert_eq!(
        ok("친구들은 목록 민수\n민수를 친구들에 넣어\n"),
        "친구들 = [\"민수\"]\n친구들.append(\"민수\")\n"
    );
    // o-ko-13
    assert_eq!(
        ok("친구들은 목록 민수\n넣어 친구들에 민수\n"),
        "친구들 = [\"민수\"]\n친구들.append(\"민수\")\n"
    );
    // h-ko-04
    assert_eq!(
        ok("친구들은 목록 민수\n민수 친구들에 넣어\n"),
        "친구들 = [\"민수\"]\n친구들.append(\"민수\")\n"
    );
    // k-ko-34
    assert_eq!(
        ok("친구들은 목록 민수\n친구들에 민수를 넣어줘\n"),
        "친구들 = [\"민수\"]\n친구들.append(\"민수\")\n"
    );
    // o-en-18
    assert_eq!(
        ok("ask What is your name? name\n"),
        "name = input(\"What is your name?\" + \" \")\n"
    );
    // l-en-04
    assert_eq!(
        ok("ask the name What is your name?\n"),
        "name = input(\"What is your name?\" + \" \")\n"
    );
}

// -------- 8. Filler words, anywhere in the line

#[test]
fn filler_words_anywhere_in_the_line() {
    // q-en-01
    assert_eq!(ok("please say hello\n"), "print(\"hello\")\n");
    // q-en-02
    assert_eq!(
        ok("please wait 2 seconds\n"),
        "__import__(\"time\").sleep(2)\n"
    );
    // g-en-02
    assert_eq!(ok("say hello please\n"), "print(\"hello\")\n");
    // g-en-03
    assert_eq!(
        ok("wait 2 seconds please\n"),
        "__import__(\"time\").sleep(2)\n"
    );
    // q-ko-01
    assert_eq!(ok("좀 안녕 말해줘\n"), "print(\"안녕\")\n");
    // q-ko-02
    assert_eq!(ok("안녕 좀 말해줘\n"), "print(\"안녕\")\n");
    // q-ko-03
    assert_eq!(ok("제발 2초 기다려\n"), "__import__(\"time\").sleep(2)\n");
    // g-ko-01
    assert_eq!(ok("안녕 좀 말해줘\n"), "print(\"안녕\")\n");
    // g-ko-04
    assert_eq!(ok("2초 좀 기다려\n"), "__import__(\"time\").sleep(2)\n");
    // q-ko-04
    assert_eq!(ok("혹시 안녕 말해줘\n"), "print(\"안녕\")\n");
}

// -------- 9. `for each` folds case and recovers

#[test]
fn list_loop_headers_fold_case_and_recover() {
    // d-en-02
    assert_eq!(
        ok("For each friend in friends\n  show friend\nend\n"),
        "for friend in friends:\n  print(friend)\n# end\n"
    );
    // d-en-03
    assert_eq!(
        ok("FOR EACH friend in friends\n  show friend\nend\n"),
        "for friend in friends:\n  print(friend)\n# end\n"
    );
    // d-en-04
    assert_eq!(
        ok("for eahc friend in friends\n  show friend\nend\n"),
        "for friend in friends:\n  print(friend)\n# end\n"
    );
    // d-en-05
    assert_eq!(
        ok("foreach friend in friends\n  show friend\nend\n"),
        "for friend in friends:\n  print(friend)\n# end\n"
    );
    // C-en-20
    assert_eq!(
        ok("For each friend in friends\n  show friend\nend\n"),
        "for friend in friends:\n  print(friend)\n# end\n"
    );
    // d-ko-02
    assert_eq!(
        ok("친구들의 친구마다 반복헤\n  친구 말해줘\n끝\n"),
        "for 친구 in 친구들:\n  print(친구)\n# end\n"
    );
    // d-ko-03
    assert_eq!(
        ok("친구들의 친구 마다 반복해\n  친구 말해줘\n끝\n"),
        "for 친구 in 친구들:\n  print(친구)\n# end\n"
    );
}

// -------- 10. `add X to <a list>`

#[test]
fn adding_a_word_to_a_list_is_an_append() {
    // y-en-26
    assert_eq!(
        ok("set friends to list of Mina\nadd Mina to friends\n"),
        "friends = [\"Mina\"]\nfriends.append(\"Mina\")\n"
    );
    // y-ko-24
    assert_eq!(
        ok("친구들은 목록 민수\n친구들에 민수 더해\n"),
        "친구들 = [\"민수\"]\n친구들.append(\"민수\")\n"
    );
}

// -------- 11. Korean comparison endings after an operator

#[test]
fn korean_comparison_endings_after_a_symbol() {
    // p2-ko-02
    assert_eq!(
        ok("점수는 0\n만약 점수 > 10 이면 성공 말해줘\n"),
        "점수 = 0\nif (점수 > 10): print(\"성공\")\n"
    );
    // p2-ko-03
    assert_eq!(
        ok("점수는 0\n만약 점수 > 10 면 성공 말해줘\n"),
        "점수 = 0\nif (점수 > 10): print(\"성공\")\n"
    );
    // p2-ko-05
    assert_eq!(
        ok("점수는 0\n점수 > 10 이면 성공 말해줘\n"),
        "점수 = 0\nif (점수 > 10): print(\"성공\")\n"
    );
    // c2-ko-05
    assert_eq!(
        ok("점수는 0\n만약에 점수 > 10 이면 성공 말해줘\n"),
        "점수 = 0\nif (점수 > 10): print(\"성공\")\n"
    );
    // p-ko-17
    assert_eq!(
        ok("점수는 0\n만약에 점수 > 10 이면 성공 말해줘\n"),
        "점수 = 0\nif (점수 > 10): print(\"성공\")\n"
    );
    // c2-ko-06
    assert_eq!(
        ok("점수는 0\n만약에 점수가 10 초과면 성공 말해줘\n"),
        "점수 = 0\nif (점수 > 10): print(\"성공\")\n"
    );
}

// -------- 12. `따라` no longer turns ordinary prose into a random pick

#[test]
fn ordinary_korean_prose_is_never_a_random_pick() {
    // Found live on the site. `따라` ("along") is one edit from `골라`
    // ("pick one"), so this story sentence compiled to a random choice
    // between four fragments — in a program that never asked for randomness.
    assert_eq!(
        ok("강이 나옵니다. 강을 따라 집으로 갑니다 말해줘\n"),
        "print(\"강이 나옵니다. 강을 따라 집으로 갑니다\")\n"
    );
}

#[test]
fn a_random_pick_still_works_when_the_word_is_unmistakable() {
    assert_eq!(
        ok("색은 빨강 또는 초록 중에서 골라\n"),
        "색 = __import__(\"random\").choice((\"빨강\", \"초록\",))\n"
    );
    assert_eq!(
        ok("색은 빨강 또는 초록 중에서 뽑아\n"),
        "색 = __import__(\"random\").choice((\"빨강\", \"초록\",))\n"
    );
    assert_eq!(
        ok("set color to pick from red or green\n"),
        "color = __import__(\"random\").choice((\"red\", \"green\",))\n"
    );
    assert_eq!(
        ok("set color to choose from red or green\n"),
        "color = __import__(\"random\").choice((\"red\", \"green\",))\n"
    );
    // Everything between two separators is one choice, however many words it
    // is. Taking a token at a time made this four things to pick from, and
    // said nothing: the program fought a `golem` some rounds and a `stone`
    // others.
    assert_eq!(
        ok("set foe to pick from stone golem or black knight\n"),
        "foe = __import__(\"random\").choice((\"stone golem\", \"black knight\",))\n"
    );
    assert_eq!(
        ok("set animal to pick from cat, dog, small bird\n"),
        "animal = __import__(\"random\").choice((\"cat\", \"dog\", \"small bird\",))\n"
    );
    // `하나 골라` is one phrase with its space written in, so the `하나`
    // belongs to the picking rather than to what is being picked from.
    assert_eq!(
        ok("적은 돌 골렘 또는 검은 기사 중에서 하나 골라\n"),
        "적 = __import__(\"random\").choice((\"돌 골렘\", \"검은 기사\",))\n"
    );
}

// -------- guards: none of the new acceptances may claim ordinary speech

#[test]
fn a_count_that_is_not_a_number_never_reaches_range() {
    assert_eq!(error_code("repeat lots times and show hello\n"), "E0304");
    assert_eq!(error_code("여러 번 반복해서 안녕 말해줘\n"), "E0304");
}

#[test]
fn a_saved_count_is_still_a_count() {
    assert_eq!(
        ok("점수는 3\n점수 번 반복해서 안녕 말해줘\n"),
        "점수 = 3\nfor _ in range(점수): print(\"안녕\")\n"
    );
}

#[test]
fn the_new_counter_and_comparison_words_leave_prose_alone() {
    // `10회` and `큰` are ordinary Korean; neither opens a loop or a
    // comparison without the repeat word or the `보다` marker beside it.
    assert_eq!(
        ok("같은 하루를 최대 10회 되풀이할 수 있어요 말해줘\n"),
        "print(\"같은 하루를 최대 10회 되풀이할 수 있어요\")\n"
    );
    assert_eq!(ok("아주 큰 소리로 말해줘\n"), "print(\"아주 큰 소리로\")\n");
    // A list line needs a name the program made; `그릇` is not one.
    assert_eq!(
        ok("설탕을 그릇에 넣어\n"),
        "print(\"설탕을 그릇에 넣어\")\n"
    );
    // `안녕하세요` ends in `예요`, which is a sentence ending only when what
    // is left is a number.
    assert_eq!(ok("인사는 안녕하세요\n"), "인사 = \"안녕하세요\"\n");
}

#[test]
fn adding_a_number_is_still_arithmetic() {
    assert_eq!(
        ok("set score to 0\nadd 1 to score\n"),
        "score = 0\nscore = score + 1\n"
    );
    // Adding a bare word to something that is not a list is reported instead
    // of compiling to `score = score + Mina`.
    assert_eq!(error_code("set score to 0\nadd Mina to score\n"), "E0221");
}

#[test]
fn loop_control_words_outside_a_block_stay_python() {
    assert_eq!(ok("stop = 1\nstop\n"), "stop = 1\nstop\n");
    assert_eq!(ok("done = 1\ndone\n"), "done = 1\ndone\n");
}

// ---------------------------------------------------------------------------
// 2026-08-19 — English prose. Measured against 302 ordinary English sentences
// (`scripts/mistake-probes/english_prose.py`): 184 of them printed themselves
// word for word, and 44 compiled into a different program. Each test below
// closes one of the causes, and each keeps the command it was hiding behind.

#[test]
fn ordinary_prose_may_carry_a_number() {
    // A digit anywhere used to switch the sentence path off for the whole
    // line: `The soup needs cream.` printed and `The soup needs 250 ml of
    // cream.` was refused. Prices, ages, times, dates and room numbers are
    // what people put in sentences.
    assert_eq!(
        ok("The soup needs 250 ml of cream.\n"),
        "print(\"The soup needs 250 ml of cream.\")\n"
    );
    assert_eq!(
        ok("Room 214 is at the end of the corridor.\n"),
        "print(\"Room 214 is at the end of the corridor.\")\n"
    );
    assert_eq!(ok("I have 3 apples\n"), "print(\"I have 3 apples\")\n");
    assert_eq!(
        ok("가격은 5000원이었습니다\n"),
        "print(\"가격은 5000원이었습니다\")\n"
    );
    // And a number written beside a command word is still a command.
    assert_eq!(ok("wait 3 seconds\n"), "__import__(\"time\").sleep(3)\n");
    assert_eq!(ok("3초 기다려\n"), "__import__(\"time\").sleep(3)\n");
    assert_eq!(ok("set score to 0\n"), "score = 0\n");
}

#[test]
fn a_pick_needs_its_choices_marked_off_from_each_other() {
    // Found while writing a guide. Nothing in this line names alternatives,
    // and the parser split it on its spaces and printed one word of it at
    // random — a different one every run.
    assert_eq!(
        ok("마음에 드는 것을 골라 보세요\n"),
        "print(\"마음에 드는 것을 골라 보세요\")\n"
    );
    assert_eq!(
        ok("천천히 골라도 됩니다\n"),
        "print(\"천천히 골라도 됩니다\")\n"
    );
    // `중에서` on its own is a scope, not a pair of choices: `여러 개` is one
    // phrase meaning "several", and it too was being picked apart.
    assert_eq!(
        ok("여러 개 중에서 뽑아\n"),
        "print(\"여러 개 중에서 뽑아\")\n"
    );
    assert_eq!(
        ok("pick a flower from the garden\n"),
        "print(\"pick a flower from the garden\")\n"
    );
    assert_eq!(
        ok("Which one did you choose in the end?\n"),
        "print(\"Which one did you choose in the end?\")\n"
    );
    // The documented spellings mark their choices, and they still pick.
    assert_eq!(
        ok("색은 빨강 또는 초록 중에서 골라\n"),
        "색 = __import__(\"random\").choice((\"빨강\", \"초록\",))\n"
    );
    assert_eq!(
        ok("set color to pick from red or green\n"),
        "color = __import__(\"random\").choice((\"red\", \"green\",))\n"
    );
}

#[test]
fn a_module_line_names_a_module() {
    // `road` is one letter from `load` and `us` is one from `use`, and the
    // search ran over every word of the line, so both of these were answered
    // with the list of modules NME bundles.
    assert_eq!(ok("end of the road\n"), "print(\"end of the road\")\n");
    assert_eq!(
        ok("Are you coming with us?\n"),
        "print(\"Are you coming with us?\")\n"
    );
    assert_eq!(
        ok("We use 2 spoons of salt.\n"),
        "print(\"We use 2 spoons of salt.\")\n"
    );
    // A real module line still works, misspelled or not, and a module line
    // written back to front is still refused.
    assert!(ok("use random latest\n").contains("import random"));
    assert!(ok("use randmo latest\n").contains("import random"));
    assert!(ok("랜덤 사용 최신\n").contains("import random"));
    assert_eq!(error_code("never use random\n"), "E0406");
}

#[test]
fn a_repaired_english_output_word_takes_one_word_of_message() {
    // `day` is one letter from `say` and `snow` one from `show`, so the last
    // word of each of these sentences disappeared without a word being said.
    assert_eq!(
        ok("Today is a good day\n"),
        "print(\"Today is a good day\")\n"
    );
    assert_eq!(
        ok("Clear a path through the snow.\n"),
        "print(\"Clear a path through the snow.\")\n"
    );
    assert_eq!(
        ok("Snow began to fall on the empty market square.\n"),
        "print(\"Snow began to fall on the empty market square.\")\n"
    );
    assert_eq!(
        ok("How much further is it?\n"),
        "print(\"How much further is it?\")\n"
    );
    // What a beginner really mistypes is the word in front of one word of
    // message, and that is still repaired — in both orders, in both languages.
    assert_eq!(ok("shwo hello\n"), "print(\"hello\")\n");
    assert_eq!(ok("hello sya\n"), "print(\"hello\")\n");
    assert_eq!(ok("말해조 안녕\n"), "print(\"안녕\")\n");
    assert_eq!(ok("안녕 말해조\n"), "print(\"안녕\")\n");
    // An output word spelled exactly is not a guess, so it keeps its whole
    // message, at either end of the line.
    assert_eq!(ok("show Hello world\n"), "print(\"Hello world\")\n");
    assert_eq!(ok("Hello world show\n"), "print(\"Hello world\")\n");
    assert_eq!(
        ok("안녕하세요 여러분 말해줘\n"),
        "print(\"안녕하세요 여러분\")\n"
    );
}

#[test]
fn a_command_word_does_not_bind_a_word_that_is_never_a_name() {
    // Exit 0, no output, no error: the worst way to be told that a sentence
    // was read as a command.
    assert_eq!(
        ok("set the table for four people\n"),
        "print(\"set the table for four people\")\n"
    );
    assert_eq!(
        ok("Set your alarm for the early train.\n"),
        "print(\"Set your alarm for the early train.\")\n"
    );
    assert_eq!(
        ok("remember to water the plants\n"),
        "print(\"remember to water the plants\")\n"
    );
    assert_eq!(
        ok("ask me anything you like\n"),
        "print(\"ask me anything you like\")\n"
    );
    assert_eq!(
        ok("Ask nicely and she might say yes.\n"),
        "print(\"Ask nicely and she might say yes.\")\n"
    );
    // A `to` after the name, or quotes around the question, say that a name
    // really was meant — whatever the word is. Converting Python back into
    // sentences writes exactly these.
    assert_eq!(ok("set then to 1\n"), "then = 1\n");
    assert_eq!(ok("set score to 0\n"), "score = 0\n");
    assert_eq!(ok("ask your \"hi\"\n"), "your = input(\"hi\")\n");
    // And the prompt of a question is text, whatever words are in it.
    assert_eq!(
        ok("ask answer yes or no\n"),
        "answer = input(\"yes or no\" + \" \")\n"
    );
    assert_eq!(
        ok("ask number taken How many? 1, 2 or 3\n"),
        "taken = int(input(\"How many? 1, 2 or 3\" + \" \"))\n"
    );
    assert_eq!(
        ok("대답을 물어봐 예 또는 아니오\n"),
        "대답 = input(\"예 또는 아니오\" + \" \")\n"
    );
}

#[test]
fn a_list_line_needs_its_items_marked_off() {
    assert_eq!(
        ok("List the ingredients on the back.\n"),
        "print(\"List the ingredients on the back.\")\n"
    );
    assert_eq!(
        ok("list everyone who came to the party\n"),
        "print(\"list everyone who came to the party\")\n"
    );
    assert_eq!(
        ok("set friends to list of Mina, Ada\n"),
        "friends = [\"Mina\", \"Ada\"]\n"
    );
    assert_eq!(
        ok("친구들은 목록 민수, 지안\n"),
        "친구들 = [\"민수\", \"지안\"]\n"
    );
}

#[test]
fn a_glued_word_only_comes_apart_into_words() {
    // `doctor` was read as `do ctor` and `finished` as `finish ed`, so these
    // sentences were refused with a suggestion nobody could act on.
    assert_eq!(
        ok("story of a small town doctor\n"),
        "print(\"story of a small town doctor\")\n"
    );
    assert_eq!(ok("Nearly finished\n"), "print(\"Nearly finished\")\n");
    assert_eq!(
        ok("a story worth telling\n"),
        "print(\"a story worth telling\")\n"
    );
    // A space really left out used to be named and left for the writer to
    // retype. NME had already worked out where the space goes in order to say
    // so, so since 2026-08-20 it does the line instead of describing it — the
    // same choice as reading the verb a beginner wrote.
    assert_eq!(ok("sayhello\n"), "print(\"hello\")\n");
    assert_eq!(ok("안녕말해줘\n"), "print(\"안녕\")\n");
    assert_eq!(ok("wait3 seconds\n"), "__import__(\"time\").sleep(3)\n");
    assert_eq!(ok("점수에1더해\n"), "점수 = 점수 + 1\n");
    assert_eq!(
        ok("3번반복해서 안녕 말해줘\n"),
        "for _ in range(3): print(\"안녕\")\n"
    );
}

#[test]
fn a_word_that_is_never_an_action_is_not_repaired_into_one() {
    // `let` used to be refused and explained, on the grounds that repairing
    // it into `set` is a translation, not a spelling. The owner asked on
    // 2026-08-19 for the near-synonyms to be accepted instead, so `let` saves
    // a value — but only where it says where the value starts. That is also
    // what keeps the apostrophe line below a sentence.
    assert_eq!(ok("let score be 0\n"), "score = 0\n");
    assert_eq!(ok("let me know\n"), "print(\"let me know\")\n");
    assert_eq!(
        ok("Let's not talk about it tonight.\n"),
        "print(\"Let's not talk about it tonight.\")\n"
    );
}

#[test]
fn a_one_word_line_says_itself() {
    // `Hello` on its own was a bare Python name, and the program died with a
    // `NameError` on a line that is not the mistake.
    assert_eq!(ok("Hello\n"), "print(\"Hello\")\n");
    assert_eq!(ok("Prologue\n"), "print(\"Prologue\")\n");
    assert_eq!(ok("안녕\n"), "print(\"안녕\")\n");
    // A name the program set is Python doing nothing, and stays Python.
    assert_eq!(ok("score = 1\nscore\n"), "score = 1\nscore\n");
    // A word NME spells out itself used to stay Python, which meant `say` on
    // its own compiled to the bare name `say` and the program died with a
    // `NameError`. An action word alone is now handed to its own matcher,
    // which says what is missing; a word that names no action still prints.
    assert_eq!(error_code("say\n"), "E0204");
    assert_eq!(error_code("말해줘\n"), "E0204");
    assert_eq!(ok("목록\n"), "print(\"목록\")\n");
    // And a name the program made stays Python doing nothing.
    assert_eq!(ok("say = 1\nsay\n"), "say = 1\nsay\n");
}

// ------------------------------------- a question whose prompt has a verb in it

#[test]
fn a_helper_verb_inside_a_question_does_not_stop_it_asking() {
    // `주문을 물어봐 마법의 주문을 말해 보세요` is the `ko/password` example on
    // the site, and it sits inside a loop. When the compound verb `말해 보세요`
    // in its prompt made the whole line prose, the line printed itself, the
    // loop never got an answer, and the program ran forever.
    //
    // The line names what it is asking for before it asks, so everything
    // after the asking word is the text shown while it waits — including any
    // ordinary Korean verb.
    assert_eq!(
        ok("주문을 물어봐 마법의 주문을 말해 보세요\n"),
        "주문 = input(\"마법의 주문을 말해 보세요\" + \" \")\n"
    );
    assert_eq!(
        ok("색을 물어봐 좋아하는 색을 말해 보세요\n"),
        "색 = input(\"좋아하는 색을 말해 보세요\" + \" \")\n"
    );
    // The four neighbours that were right throughout, so the boundary is
    // pinned from both sides.
    assert_eq!(
        ok("이름을 물어봐 이름을 적어 보세요\n"),
        "이름 = input(\"이름을 적어 보세요\" + \" \")\n"
    );
    assert_eq!(
        ok("이름을 물어봐 이름이 뭐예요?\n"),
        "이름 = input(\"이름이 뭐예요?\" + \" \")\n"
    );
    assert_eq!(
        ok("나이를 숫자로 물어봐 몇 살인지 알려 주세요\n"),
        "나이 = int(input(\"몇 살인지 알려 주세요\" + \" \"))\n"
    );
    assert_eq!(
        ok("색을 물어봐 좋아하는 색을 말해 주세요\n"),
        "색 = input(\"좋아하는 색을 말해 주세요\" + \" \")\n"
    );
}

#[test]
fn a_helper_verb_on_the_asking_word_itself_is_still_a_sentence() {
    // No name stands in front of the asking word, so `주셔서` hangs off
    // `물어봐` and the line is a thank-you, not a question.
    assert_eq!(
        ok("물어봐 주셔서 감사합니다\n"),
        "print(\"물어봐 주셔서 감사합니다\")\n"
    );
    // A name in front is not enough on its own: the compound verb is still
    // the asking word's, and nothing follows it that could be a prompt.
    assert_eq!(
        ok("말씀을 물어봐 주셔서 감사합니다\n"),
        "print(\"말씀을 물어봐 주셔서 감사합니다\")\n"
    );
    assert_eq!(
        ok("말해 봐야 소용없는 일이었습니다\n"),
        "print(\"말해 봐야 소용없는 일이었습니다\")\n"
    );
}

// ------------------------------------------------------- a date on either side

/// `yesterday`, `2 days ago` and `3 days from now` — and their Korean twins —
/// are the sentence way to step off today. They mean a date only after the
/// date toolbox is open, because that is what binds what they lower to.
#[test]
fn stepping_off_today_works_in_both_languages() {
    for (source, expected) in [
        ("use date\nshow yesterday\n", "print(days_after(-1))"),
        ("use date\nshow tomorrow\n", "print(days_after(1))"),
        (
            "use date\nset before to 1 day ago\n",
            "before = days_after(-1)",
        ),
        (
            "use date\nset later to 3 days from now\n",
            "later = days_after(3)",
        ),
        ("날짜 사용\n어제 말해줘\n", "print(days_after(-1))"),
        ("날짜 사용\n내일 말해줘\n", "print(days_after(1))"),
        (
            "날짜 사용\n어제날짜는 1일 전\n",
            "어제날짜 = days_after(-1)",
        ),
        ("날짜 사용\n모레날짜는 2일 뒤\n", "모레날짜 = days_after(2)"),
    ] {
        let python = ok(source);
        assert!(python.contains(expected), "{source} -> {python}");
    }
}

/// Without the toolbox the same words are ordinary speech, so a sentence that
/// happens to hold them keeps every word it has.
#[test]
fn stepping_off_today_needs_the_toolbox() {
    assert_eq!(
        ok("3 days ago I saw her\n"),
        "print(\"3 days ago I saw her\")\n"
    );
    assert_eq!(
        ok("약속은 3일 전이었습니다\n"),
        "print(\"약속은 3일 전이었습니다\")\n"
    );
    assert_eq!(
        ok("어제 비가 왔습니다\n"),
        "print(\"어제 비가 왔습니다\")\n"
    );
}
