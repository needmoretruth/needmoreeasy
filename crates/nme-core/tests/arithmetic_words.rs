//! Arithmetic said in words, a reading on the right of a comparison, and a
//! one-syllable Korean name standing alone in a condition.
//!
//! Before these, every line below that reads correctly compiled into a
//! *different* program: `set left to total minus done` saved the text
//! `10 minus 3`, the second length in `말 길이가 가장긴말 길이보다 크면` became
//! the string `"가장긴말 길이"`, and `만약에 비면` tested a name nobody had
//! made. Each test names the old behaviour so that a regression reads as the
//! bug it is.

use nme_core::{tidy, transpile, Language, SyntaxLevel};

fn ok(source: &str) -> String {
    transpile(source)
        .unwrap_or_else(|problems| panic!("expected successful transpile, got: {problems:?}"))
}

/// The last line of a program, which is the one each of these is about.
fn last(source: &str) -> String {
    ok(source)
        .lines()
        .last()
        .expect("at least one line")
        .to_string()
}

fn tidied(source: &str, level: SyntaxLevel, language: Language) -> String {
    tidy(source, level, language)
        .unwrap_or_else(|problems| panic!("expected the program to tidy, got: {problems:?}"))
        .source
}

// ------------------------------------------------------ arithmetic as a value

#[test]
fn the_arithmetic_words_lower_to_the_python_operators() {
    let english = "set total to 10\nset done to 3\n";
    // Used to be `left = str(total) + " minus " + str(done)`.
    assert_eq!(
        last(&format!("{english}set left to total minus done\n")),
        "left = total - done"
    );
    assert_eq!(
        last(&format!("{english}set both to total plus done\n")),
        "both = total + done"
    );
    assert_eq!(
        last(&format!("{english}set big to total times done\n")),
        "big = total * done"
    );
    assert_eq!(
        last(&format!("{english}set big to total multiplied by done\n")),
        "big = total * done"
    );
    assert_eq!(
        last(&format!("{english}set part to total divided by done\n")),
        "part = total / done"
    );

    let korean = "전체는 10\n순서는 3\n";
    assert_eq!(
        last(&format!("{korean}남은수는 전체 빼기 순서\n")),
        "남은수 = 전체 - 순서"
    );
    assert_eq!(
        last(&format!("{korean}합은 전체 더하기 순서\n")),
        "합 = 전체 + 순서"
    );
    assert_eq!(
        last(&format!("{korean}곱은 전체 곱하기 순서\n")),
        "곱 = 전체 * 순서"
    );
    assert_eq!(
        last(&format!("{korean}비율은 전체 나누기 순서\n")),
        "비율 = 전체 / 순서"
    );
}

#[test]
fn a_chain_keeps_the_precedence_a_maths_book_gives() {
    assert_eq!(
        last("set a to 3\nset b to 4\nset c to 2\nset x to a plus b times c minus 1\n"),
        "x = a + b * c - 1"
    );
    assert_eq!(
        last("가는 3\n나는 4\n다는 2\n라는 가 더하기 나 곱하기 다 빼기 1\n"),
        "라 = 가 + 나 * 다 - 1"
    );
    // `2.` at the end of a sentence is the number two and a full stop, not
    // the float `2.0`.
    assert_eq!(last("set a to 3\nset x to a times 2.\n"), "x = a * 2");
}

#[test]
fn english_and_korean_arithmetic_is_one_program() {
    let english = ok("set total to 10\nset done to 3\nset left to total minus done times 2\n");
    let korean = ok("total은 10\ndone은 3\nleft는 total 빼기 done 곱하기 2\n");
    assert_eq!(english, korean);
}

#[test]
fn show_and_say_print_the_answer_rather_than_the_words() {
    // Used to be `print(str(total) + " minus " + str(done))`.
    assert_eq!(
        last("set total to 10\nset done to 3\nshow total minus done\n"),
        "print(total - done)"
    );
    assert_eq!(
        last("전체는 10\n순서는 3\n전체 빼기 순서 말해줘\n"),
        "print(전체 - 순서)"
    );
    // Korean puts its particle on the last word.
    assert_eq!(
        last("전체는 10\n순서는 3\n전체 빼기 순서를 말해줘\n"),
        "print(전체 - 순서)"
    );
}

// ------------------------------------------------------- prose stays prose

#[test]
fn every_side_has_to_be_a_number_or_a_name_the_program_made() {
    assert_eq!(last("설탕 빼기 말해줘\n"), "print(\"설탕 빼기\")");
    assert_eq!(
        last("show one plus one equals two\n"),
        "print(\"one plus one equals two\")"
    );
    assert_eq!(
        last("show the price minus tax\n"),
        "print(\"the price minus tax\")"
    );
    // One side made, the other not: still the sentence it was.
    assert_eq!(
        last("set total to 10\nshow total minus tax\n"),
        "print(str(total) + \" minus tax\")"
    );
    // A job and a record are names the program made, but nothing adds one.
    assert!(!last("to greet:\n    show hi\nshow greet plus 1\n").contains("greet + 1"));
    assert!(!last("set ages to an empty record\nshow ages plus 1\n").contains("ages + 1"));
}

#[test]
fn arithmetic_is_only_read_when_it_is_the_whole_value() {
    assert_eq!(
        last("set total to 10\nset done to 3\nshow You have total minus done left\n"),
        "print(\"You have \" + str(total) + \" minus \" + str(done) + \" left\")"
    );
    assert_eq!(
        last("set total to 10\nshow total minus\n"),
        "print(str(total) + \" minus\")"
    );
}

#[test]
fn the_repeat_forms_that_say_times_are_unchanged() {
    let python = ok(concat!(
        "set a to 3\nset name to hi\n",
        "repeat 3 times\nshow a\nend\n",
        "3 times Welcome\n",
        "show name repeated 5 times\n",
        "a times show hi\n",
    ));
    assert!(
        python.contains("for _ in range(3):\n    print(a)"),
        "{python}"
    );
    assert!(
        python.contains("for _ in range(3): print(\"Welcome\")"),
        "{python}"
    );
    assert!(python.contains("print(str(name) * 5)"), "{python}");
    assert!(
        python.contains("for _ in range(a): print(\"hi\")"),
        "{python}"
    );
}

#[test]
fn valid_python_with_the_same_words_is_kept_byte_for_byte() {
    let source = concat!(
        "plus = 1\n",
        "minus = 2\n",
        "times = 3\n",
        "x = plus + minus * times\n",
        "y = x-plus  # plus minus times\n",
        "print(\"a plus b\")\n",
    );
    assert_eq!(ok(source), source);
}

#[test]
fn arithmetic_words_mix_with_beginner_and_python_lines() {
    let python = ok(concat!(
        "save total to 10\n",
        "done = 3\n",
        "set left to total minus done\n",
        "say left - 1\n",
        "남은것은 total 빼기 done\n",
    ));
    assert_eq!(
        python,
        "total = 10\ndone = 3\nleft = total - done\nprint(left - 1)\n남은것 = total - done\n"
    );
}

// ------------------------------------------------------ either side of a comparison

#[test]
fn arithmetic_stands_on_either_side_of_a_comparison() {
    let english = ok(concat!(
        "set total to 10\nset done to 3\n",
        "if total minus done is greater than 5 then show big\n",
        "if total equals done plus 7 then show same\n",
    ));
    assert!(
        english.contains("if (total - done > 5): print(\"big\")"),
        "{english}"
    );
    assert!(
        english.contains("if (total == done + 7): print(\"same\")"),
        "{english}"
    );

    let korean = ok(concat!(
        "전체는 10\n순서는 3\n",
        "만약에 전체 빼기 순서가 5보다 크면 크다 말해줘\n",
        "만약에 전체가 순서 더하기 7과 같으면 같다 말해줘\n",
    ));
    assert!(
        korean.contains("if (전체 - 순서 > 5): print(\"크다\")"),
        "{korean}"
    );
    assert!(
        korean.contains("if (전체 == 순서 + 7): print(\"같다\")"),
        "{korean}"
    );
}

#[test]
fn a_name_that_is_also_a_particle_starts_the_right_side() {
    // `가` is a subject particle as well as the name made on the first line.
    assert_eq!(
        last("가는 3\n나는 4\n다는 12\n만약에 다가 가 곱하기 나와 같으면 맞다 말해줘\n"),
        "if (다 == 가 * 나): print(\"맞다\")"
    );
}

// -------------------------------------------- a reading on the right (PLAN K)

#[test]
fn a_length_on_the_right_of_a_comparison_is_a_length() {
    // Used to be `if (len(말) > "가장긴말 길이"):`, a `TypeError` at run time.
    assert_eq!(
        ok("말은 안녕\n가장긴말은 하이\n만약에 말 길이가 가장긴말 길이보다 크면\n말 말해줘\n끝\n")
            .lines()
            .nth(2),
        Some("if (len(말) > len(가장긴말)):")
    );
    assert_eq!(
        ok(concat!(
            "set word to hi\nset longest to hello\n",
            "if the length of word is greater than the length of longest\n",
            "show word\nend\n",
        ))
        .lines()
        .nth(2),
        Some("if (len(word) > len(longest)):")
    );
}

#[test]
fn the_right_side_takes_every_reading_the_left_side_takes() {
    let english = ok(concat!(
        "set friends to list of Mina, Ada\nset pals to list of Bo\nset score to 9\n",
        "if how many pals is less than how many friends then show a\n",
        "if score equals the remainder of score divided by 4 then show b\n",
        "if score equals the whole number of score divided by 2 then show c\n",
    ));
    assert!(
        english.contains("if (len(pals) < len(friends))"),
        "{english}"
    );
    assert!(english.contains("if (score == score % 4)"), "{english}");
    assert!(english.contains("if (score == score // 2)"), "{english}");

    let korean = ok(concat!(
        "친구들은 목록 민수, 지안\n짝들은 목록 보라\n점수는 9\n",
        "만약에 짝들 개수가 친구들 개수보다 작으면 가 말해줘\n",
        "만약에 점수가 점수를 4로 나눈 나머지와 같으면 나 말해줘\n",
        "만약에 점수가 점수를 2로 나눈 몫과 같으면 다 말해줘\n",
    ));
    assert!(korean.contains("if (len(짝들) < len(친구들))"), "{korean}");
    assert!(korean.contains("if (점수 == 점수 % 4)"), "{korean}");
    assert!(korean.contains("if (점수 == 점수 // 2)"), "{korean}");
}

#[test]
fn a_right_side_naming_nothing_the_program_made_is_still_text() {
    assert_eq!(
        last("set word to hi\nif word equals the length of cake then show odd\n"),
        "if (word == \"the length of cake\"): print(\"odd\")"
    );
}

// ------------------------------------------ a bare Korean name (PLAN E)

#[test]
fn a_one_syllable_korean_name_is_a_condition_on_its_own() {
    // Used to be `if (비면):`, a `NameError` at run time; English `if rain`
    // already worked.
    let python = ok(concat!(
        "비는 30% 확률\n",
        "만약에 비면\n우산 말해줘\n",
        "아니면 만약에 비이면\n장화 말해줘\n끝\n",
        "만약에 비라면\n우산 말해줘\n끝\n",
        "만약에 비면 우산 말해줘\n",
    ));
    assert_eq!(
        python.lines().skip(1).collect::<Vec<_>>(),
        [
            "if (비):",
            "    print(\"우산\")",
            "elif (비):",
            "    print(\"장화\")",
            "# end",
            "if (비):",
            "    print(\"우산\")",
            "# end",
            "if (비): print(\"우산\")",
        ]
    );
    assert_eq!(
        ok("rain is a 30% chance\nif rain\nshow umbrella\nend\n")
            .lines()
            .nth(1),
        Some("if (rain):")
    );
}

#[test]
fn the_longest_name_the_program_made_wins() {
    assert_eq!(
        ok("놀은 1\n놀이는 2\n만약에 놀이면\n재미 말해줘\n끝\n")
            .lines()
            .nth(2),
        Some("if (놀이):")
    );
}

#[test]
fn an_ending_on_a_word_the_program_never_made_keeps_its_old_reading() {
    assert_eq!(
        ok("만약에 해면\n모자 말해줘\n끝\n").lines().next(),
        Some("if (해면):")
    );
    let problems = transpile("만약에 눈이면\n장갑 말해줘\n끝\n").expect_err("still refused");
    assert_eq!(problems[0].code.code(), "E0301");
}

// ---------------------------------------------------------------- the tidier

#[test]
fn the_tidier_writes_arithmetic_in_every_spelling() {
    let program = concat!(
        "set total to 10\nset done to 3\n",
        "set left to total minus done times 2\n",
        "if total minus done is greater than done\nshow big\nend\n",
    );
    let korean = tidied(program, SyntaxLevel::Sentence, Language::Korean);
    assert!(
        korean.contains("left는 total 빼기 done 곱하기 2\n"),
        "{korean}"
    );
    assert!(
        korean.contains("만약에 total 빼기 done이 done보다 크면\n"),
        "{korean}"
    );
    let beginner = tidied(program, SyntaxLevel::Beginner, Language::English);
    assert!(
        beginner.contains("save left to total - done * 2\n"),
        "{beginner}"
    );
    assert!(
        beginner.contains("when total - done > done\n"),
        "{beginner}"
    );

    // And back: ordinary Python arrives at the same sentence.
    let python = "total = 10\ndone = 3\nleft = total - done * 2\nprint(total / done)\n";
    let english = tidied(python, SyntaxLevel::Sentence, Language::English);
    assert!(
        english.contains("set left to total minus done times 2\n"),
        "{english}"
    );
    assert!(
        english.contains("show total divided by done\n"),
        "{english}"
    );
}
