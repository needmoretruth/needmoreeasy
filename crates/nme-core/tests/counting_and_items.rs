//! Counting loops, changing one item of a list, and a random item of a list.
//!
//! All three are gated on their whole shape and on names the program already
//! made. `set item 2 of values to 9` used to compile into a save of the words
//! `2 of … to 9`, which is the worst kind of failure this compiler has: a
//! program that runs and does something else. Most of these tests are about
//! the gates; the Python each form produces is the easy half.

use nme_core::{tidy, transpile, Language, SyntaxLevel};

fn ok(source: &str) -> String {
    transpile(source)
        .unwrap_or_else(|problems| panic!("expected successful transpile, got: {problems:?}"))
}

fn error_code(source: &str) -> String {
    let problems = transpile(source).expect_err("expected this line to be rejected");
    problems[0].code.code().to_string()
}

/// The one line a program produces after the lines that set it up.
fn last_line(source: &str) -> String {
    ok(source).lines().last().expect("a line").to_string()
}

// ------------------------------------------------------------ counting

#[test]
fn a_counting_loop_with_no_name_is_told_what_is_missing() {
    for source in [
        "count from 1 to 10\nshow hi\nend\n",
        "repeat from 1 to 10:\n    show hi\n",
        "1부터 10까지 세면서 반복해\n안녕 말해줘\n끝\n",
        "1부터 10까지 반복해\n안녕 말해줘\n끝\n",
    ] {
        let problems = transpile(source).expect_err("a counting loop with no name");
        // The `end` under a refused header is reported too, pointing at the
        // same line; the first message is the one that says what to write.
        assert_eq!(problems[0].code.code(), "E0307", "{source}: {problems:?}");
    }
    // A sentence that only begins the same way is still a sentence.
    assert_eq!(
        ok("Count from 1 to 10 and open your eyes.\n"),
        "print(\"Count from 1 to 10 and open your eyes.\")\n"
    );
    assert_eq!(
        ok("하나부터 열까지 세면서 기다렸습니다\n"),
        "print(\"하나부터 열까지 세면서 기다렸습니다\")\n"
    );
}
#[test]
fn a_counting_loop_counts_both_ends_in_both_languages() {
    let wanted = "for n in range(1, 11):\n    print(n)\n# end\n";
    assert_eq!(ok("count n from 1 to 10\nshow n\nend\n"), wanted);
    assert_eq!(ok("repeat with n from 1 to 10\nshow n\nend\n"), wanted);
    assert_eq!(ok("n을 1부터 10까지 세면서 반복해\nn 말해줘\n끝\n"), wanted);
    assert_eq!(
        ok("수를 1부터 10까지 세면서 반복해\n수 말해줘\n끝\n"),
        "for 수 in range(1, 11):\n    print(수)\n# end\n"
    );
}

#[test]
fn the_korean_counter_may_leave_its_particle_off_when_nothing_else_could_be_meant() {
    assert_eq!(
        ok("수 1부터 3까지 세면서 반복해\n수 말해줘\n끝\n"),
        "for 수 in range(1, 4):\n    print(수)\n# end\n"
    );
    // `아이는` could be the name `아이` with a topic particle on it, so it is
    // not a counter.
    assert!(!ok("아이는 1부터 10까지 세면서 반복해\n").contains("for "));
}

#[test]
fn two_written_numbers_the_wrong_way_round_count_down() {
    assert_eq!(
        ok("count n from 10 to 1\nshow n\nend\n"),
        "for n in range(10, 0, -1):\n    print(n)\n# end\n"
    );
    assert_eq!(
        ok("n을 10부터 1까지 세면서 반복해\nn 말해줘\n끝\n"),
        "for n in range(10, 0, -1):\n    print(n)\n# end\n"
    );
}

#[test]
fn a_name_at_either_end_lets_the_line_choose_its_direction() {
    let wanted = "for n in (range(1, top + 1) if 1 <= top else range(1, top - 1, -1)):";
    assert_eq!(
        ok("set top to 5\ncount n from 1 to top\nshow n\nend\n")
            .lines()
            .nth(1),
        Some(wanted)
    );
    assert_eq!(
        ok("top은 5\nn을 1부터 top까지 세면서 반복해\nn 말해줘\n끝\n")
            .lines()
            .nth(1),
        Some(wanted)
    );
    assert_eq!(
        ok("시작은 3\n끝값은 1\n수를 시작부터 끝값까지 세면서 반복해\n수 말해줘\n끝\n")
            .lines()
            .nth(2),
        Some("for 수 in (range(시작, 끝값 + 1) if 시작 <= 끝값 else range(시작, 끝값 - 1, -1)):")
    );
}

#[test]
fn counting_words_are_numbers_and_one_line_forms_carry_their_body() {
    assert_eq!(
        ok("count i from one to three\n    show i\n"),
        "for i in range(1, 4):\n    print(i)\n"
    );
    assert_eq!(
        ok("수를 하나부터 셋까지 세면서 반복해\n    수 말해줘\n"),
        "for 수 in range(1, 4):\n    print(수)\n"
    );
    assert_eq!(
        ok("count n from 1 to 3 and show n\n"),
        "for n in range(1, 4): print(n)\n"
    );
    assert_eq!(
        ok("n을 1부터 3까지 세면서 반복해서 n 말해줘\n"),
        "for n in range(1, 4): print(n)\n"
    );
}

#[test]
fn the_beginner_header_is_the_sentence_one_with_a_colon() {
    let wanted = "for n in range(1, 4):\n    print(n)\n";
    assert_eq!(ok("count n from 1 to 3:\n    print(n)\n"), wanted);
    assert_eq!(ok("n을 1부터 3까지 세면서:\n    print(n)\n"), wanted);
    assert_eq!(ok("n을 1부터 3까지 세면서 반복해:\n    print(n)\n"), wanted);
}

#[test]
fn skip_and_break_work_inside_a_counting_loop() {
    assert_eq!(
        ok("count n from 1 to 5\nif n equals 3 then skip\nif n equals 4 then break\nshow n\nend\n"),
        "for n in range(1, 6):\n    if (n == 3): continue\n    if (n == 4): break\n    print(n)\n# end\n"
    );
    assert_eq!(
        ok("n을 1부터 5까지 세면서 반복해\n멈춰\n끝\n"),
        "for n in range(1, 6):\n    break\n# end\n"
    );
}

#[test]
fn count_words_without_the_whole_shape_stay_sentences() {
    assert_eq!(
        ok("count sheep from dusk to dawn\n"),
        "print(\"count sheep from dusk to dawn\")\n"
    );
    assert_eq!(
        ok("하나부터 열까지 세면서 기다렸습니다\n"),
        "print(\"하나부터 열까지 세면서 기다렸습니다\")\n"
    );
}

#[test]
fn a_python_range_loop_is_left_exactly_as_written() {
    let python = "for n in range(1, 11):\n    print(n)\n";
    assert_eq!(ok(python), python);
    let mixed = "count n from 1 to 2\nprint(n * 2)\nend\n";
    assert_eq!(
        ok(mixed),
        "for n in range(1, 3):\n    print(n * 2)\n# end\n"
    );
}

// ------------------------------------------------------ changing an item

#[test]
fn one_item_of_a_list_gets_a_new_value_in_both_languages() {
    assert_eq!(
        last_line("set values to list 1, 2, 3\nset item 2 of values to 9\n"),
        "values[1] = 9"
    );
    assert_eq!(
        last_line("set values to list 1, 2, 3\nchange item 2 of values to 9\n"),
        "values[1] = 9"
    );
    for line in [
        "수들 2 번째를 9로 바꿔",
        "수들의 2번째를 9로 바꿔",
        "수들 2번째를 9로 바꿔",
    ] {
        assert_eq!(
            last_line(&format!("수들은 목록 1, 2, 3\n{line}\n")),
            "수들[1] = 9",
            "{line}"
        );
    }
}

#[test]
fn the_position_may_be_a_name_and_the_value_any_saved_value() {
    assert_eq!(
        last_line("수들은 목록 1, 2, 3\n순서는 2\n수들 순서 번째를 0으로 바꿔\n"),
        "수들[순서 - 1] = 0"
    );
    assert_eq!(
        last_line("set values to list 1, 2\nset place to 2\nset item place of values to Mina\n"),
        "values[place - 1] = \"Mina\""
    );
    assert_eq!(
        last_line("이름들은 목록 가, 나\n이름들 첫 번째를 민수로 바꿔\n"),
        "이름들[0] = \"민수\""
    );
    assert_eq!(
        last_line("set values to list 1, 2\nset score to 5\nset the last of values to score + 1\n"),
        "values[-1] = score + 1"
    );
}

#[test]
fn a_change_to_a_name_nobody_made_keeps_its_old_reading() {
    // Korean has always printed a line it could not read as a command.
    assert_eq!(
        ok("수들 2 번째를 9로 바꿔\n"),
        "print(\"수들 2 번째를 9로 바꿔\")\n"
    );
    // A made name never again becomes a save into a name called `item`.
    assert!(!ok("set values to 5\nset item 2 of values to 9\n").contains("item ="));
}

#[test]
fn a_record_has_no_numbered_item_to_change() {
    assert_eq!(
        error_code("set ages to an empty record\nset item 2 of ages to 9\n"),
        "E0234"
    );
    assert_eq!(
        error_code("나이표는 빈 표\n나이표 2번째를 9로 바꿔\n"),
        "E0234"
    );
}

#[test]
fn item_zero_and_a_missing_value_are_named() {
    assert_eq!(
        error_code("set values to list 1, 2\nset item 0 of values to 9\n"),
        "E0229"
    );
    assert_eq!(
        error_code("수들은 목록 1, 2\n수들 0번째를 9로 바꿔\n"),
        "E0229"
    );
    assert_eq!(
        error_code("set values to list 1, 2\nset item 2 of values\n"),
        "E0411"
    );
}

#[test]
fn a_python_item_assignment_is_left_exactly_as_written() {
    let python = "values = [1, 2]\nvalues[1] = 9\n";
    assert_eq!(ok(python), python);
}

// ------------------------------------------------------ a random item

#[test]
fn a_random_item_is_a_value_in_both_languages() {
    let pick = "print(__import__(\"random\").choice(songs))";
    assert_eq!(
        last_line("set songs to list a, b, c\nshow a random one of songs\n"),
        pick
    );
    assert_eq!(
        last_line("set songs to list a, b, c\nshow a random item of songs\n"),
        pick
    );
    assert_eq!(
        last_line("songs는 목록 a, b, c\nsongs 중 아무거나 말해줘\n"),
        pick
    );
    assert_eq!(
        last_line("songs는 목록 a, b, c\nsongs에서 아무거나 말해줘\n"),
        pick
    );
    assert_eq!(
        last_line("set songs to list a, b\nset song to a random item from songs\n"),
        "song = __import__(\"random\").choice(songs)"
    );
}

#[test]
fn a_random_item_stands_in_a_comparison() {
    assert_eq!(
        last_line("set songs to list a, b\nif a random one of songs equals a then show yes\n"),
        "if (__import__(\"random\").choice(songs) == \"a\"): print(\"yes\")"
    );
    assert_eq!(
        last_line("노래들은 목록 가, 나\n만약에 노래들 중 아무거나가 가와 같으면 맞아 말해줘\n"),
        "if (__import__(\"random\").choice(노래들) == \"가\"): print(\"맞아\")"
    );
}

#[test]
fn a_random_item_of_something_that_is_not_a_list_is_words() {
    assert_eq!(
        ok("show a random one of songs\n"),
        "print(\"a random one of songs\")\n"
    );
    assert_eq!(
        ok("노래들 중 아무거나 말해줘\n"),
        "print(\"노래들 중 아무거나\")\n"
    );
    let python = "import random\nsongs = [1]\nprint(random.choice(songs))\n";
    assert_eq!(ok(python), python);
}

// ------------------------------------------------------ all three, tidied

#[test]
fn a_program_using_all_three_moves_between_every_spelling_and_means_the_same() {
    let source = "set values to list 1, 2, 3\nset songs to list a, b\n\
                  count n from 1 to 3\nset item n of values to n\nend\n\
                  show a random one of songs\nset the first of values to 7\n";
    let python = ok(source);
    for language in [Language::English, Language::Korean] {
        for level in [SyntaxLevel::Sentence, SyntaxLevel::Beginner] {
            let written = tidy(source, level, language).expect("tidies").source;
            assert_eq!(ok(&written), python, "{level:?} {language:?}:\n{written}");
        }
    }
    let korean = tidy(source, SyntaxLevel::Sentence, Language::Korean)
        .expect("tidies")
        .source;
    assert!(
        korean.contains("n을 1부터 3까지 세면서 반복해\n"),
        "{korean}"
    );
    assert!(korean.contains("values n 번째를 n으로 바꿔\n"), "{korean}");
    assert!(korean.contains("songs 중 아무거나 말해줘\n"), "{korean}");
    assert!(korean.contains("values 첫 번째를 7로 바꿔\n"), "{korean}");
    // And from Python, which is where a program written elsewhere arrives.
    let back = tidy(&python, SyntaxLevel::Sentence, Language::English)
        .expect("tidies")
        .source;
    assert!(back.contains("count n from 1 to 3:\n"), "{back}");
    assert!(back.contains("set item n of values to n\n"), "{back}");
}
