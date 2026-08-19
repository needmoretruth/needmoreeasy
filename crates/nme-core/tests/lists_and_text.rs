//! Everything a beginner does to a list, and to a piece of text, in sentences.
//!
//! The list statements share one safety argument, and it is the reason they
//! can exist at all: **each of them is gated on a name the program already
//! made a list.** `sort out your things` names no list, so it stays the
//! sentence it is. That gate is what these tests are mostly about — the
//! Python each form produces is the easy half.
//!
//! Every capability appears in English and in Korean, at the sentence level
//! and at the beginner level, and each refusal is checked by its code.

use nme_core::transpile;

fn ok(source: &str) -> String {
    transpile(source)
        .unwrap_or_else(|problems| panic!("expected successful transpile, got: {problems:?}"))
}

fn error_code(source: &str) -> String {
    let problems = transpile(source).expect_err("expected this line to be rejected");
    problems[0].code.code().to_string()
}

/// The one line a program produces after the line that made the list.
fn after_list_en(line: &str) -> String {
    let source = format!("set friends to list of Mina, Ada\n{line}\n");
    ok(&source)
        .lines()
        .nth(1)
        .expect("a second line")
        .to_string()
}

fn after_list_ko(line: &str) -> String {
    let source = format!("친구들은 목록 민수, 지안\n{line}\n");
    ok(&source)
        .lines()
        .nth(1)
        .expect("a second line")
        .to_string()
}

/// A condition header, read out of the whole block it needs to be legal.
fn condition_en(header: &str) -> String {
    let source = format!("set friends to list of Mina, Ada\n{header}\n    show ok\n");
    ok(&source)
        .lines()
        .nth(1)
        .expect("a second line")
        .to_string()
}

fn condition_ko(header: &str) -> String {
    let source = format!("친구들은 목록 민수, 지안\n{header}\n    확인 말해줘\n");
    ok(&source)
        .lines()
        .nth(1)
        .expect("a second line")
        .to_string()
}

// ------------------------------------------------------------ empty lists

#[test]
fn an_empty_list_is_a_list_and_not_the_words() {
    assert_eq!(ok("set friends to an empty list\n"), "friends = []\n");
    assert_eq!(ok("set friends to empty list\n"), "friends = []\n");
    assert_eq!(ok("친구들은 빈 목록\n"), "친구들 = []\n");
    assert_eq!(ok("친구들은 새 목록\n"), "친구들 = []\n");
}

#[test]
fn an_empty_list_can_then_be_added_to() {
    assert_eq!(
        ok("친구들은 빈 목록\n친구들에 민수 넣어\n"),
        "친구들 = []\n친구들.append(\"민수\")\n"
    );
    assert_eq!(
        ok("set friends to an empty list\nappend Mina to friends\n"),
        "friends = []\nfriends.append(\"Mina\")\n"
    );
}

#[test]
fn the_empty_word_alone_is_still_a_sentence() {
    // The list word has to follow immediately, or this is ordinary speech.
    assert_eq!(ok("빈 방이었습니다\n"), "print(\"빈 방이었습니다\")\n");
    assert_eq!(ok("an empty room\n"), "print(\"an empty room\")\n");
    assert_eq!(ok("빈 목록입니다\n"), "print(\"빈 목록입니다\")\n");
}

// ----------------------------------------------------------- how many

#[test]
fn how_many_counts_a_list_in_both_languages() {
    assert_eq!(
        after_list_en("show how many friends"),
        "print(len(friends))"
    );
    assert_eq!(after_list_ko("친구들 개수 말해줘"), "print(len(친구들))");
    assert_eq!(
        after_list_en("show the number of friends"),
        "print(len(friends))"
    );
    assert_eq!(after_list_ko("친구들의 개수 말해줘"), "print(len(친구들))");
}

#[test]
fn how_many_is_also_a_value_and_a_condition() {
    assert_eq!(
        after_list_en("set total to how many friends"),
        "total = len(friends)"
    );
    assert_eq!(after_list_ko("총합은 친구들 개수"), "총합 = len(친구들)");
    assert_eq!(
        condition_en("if how many friends is greater than 1"),
        "if (len(friends) > 1):"
    );
    assert_eq!(
        condition_ko("만약에 친구들 개수가 1보다 크면"),
        "if (len(친구들) > 1):"
    );
}

#[test]
fn counting_needs_a_name_that_was_made_a_list() {
    // `many` was never saved, so there is nothing to count.
    assert_eq!(
        ok("how many people came\n"),
        "print(\"how many people came\")\n"
    );
    assert_eq!(
        ok("친구들 개수가 궁금합니다\n"),
        "print(\"친구들 개수가 궁금합니다\")\n"
    );
    // The name exists but is not a list.
    assert_eq!(
        ok("set friends to Mina\nshow how many friends\n"),
        "friends = \"Mina\"\nprint(\"how many \" + str(friends))\n"
    );
}

// ------------------------------------------------------------- contains

#[test]
fn a_list_can_be_asked_whether_it_holds_something() {
    assert_eq!(
        condition_en("if friends contains Mina"),
        "if (\"Mina\" in friends):"
    );
    assert_eq!(
        condition_ko("만약에 친구들에 민수가 있으면"),
        "if (\"민수\" in 친구들):"
    );
    assert_eq!(
        condition_en("if friends does not contain Grace"),
        "if (\"Grace\" not in friends):"
    );
    assert_eq!(
        condition_ko("만약에 친구들에 지수가 없으면"),
        "if (\"지수\" not in 친구들):"
    );
}

#[test]
fn a_one_word_existence_condition_keeps_its_old_meaning() {
    assert_eq!(
        ok("set ready to True\nif ready exists\n    show go\nend\n")
            .lines()
            .nth(1)
            .expect("a second line"),
        "if (ready):"
    );
    assert_eq!(
        ok("준비는 참\n만약에 준비가 없으면\n    안녕 말해줘\n끝\n")
            .lines()
            .nth(1)
            .expect("a second line"),
        "if (not (준비)):"
    );
}

// -------------------------------------------------------------- emptiness

#[test]
fn a_list_can_be_asked_whether_it_holds_nothing() {
    assert_eq!(condition_en("if friends is empty"), "if (not (friends)):");
    assert_eq!(
        condition_ko("만약에 친구들이 비었으면"),
        "if (not (친구들)):"
    );
    assert_eq!(
        condition_ko("만약에 친구들이 비어있으면"),
        "if (not (친구들)):"
    );
    assert_eq!(condition_en("if friends is not empty"), "if (friends):");
}

// ---------------------------------------------------------------- removal

#[test]
fn taking_an_item_out_is_a_removal_and_not_a_subtraction() {
    // This used to compile to `friends = friends - Mina` and die at run time.
    assert_eq!(
        after_list_en("remove Mina from friends"),
        "friends.remove(\"Mina\")"
    );
    assert_eq!(
        after_list_ko("친구들에서 민수 빼"),
        "친구들.remove(\"민수\")"
    );
    assert_eq!(after_list_en("remove 1 from friends"), "friends.remove(1)");
}

#[test]
fn subtracting_from_a_number_still_subtracts() {
    assert_eq!(
        ok("set score to 10\nsubtract 1 from score\n"),
        "score = 10\nscore = score - 1\n"
    );
    assert_eq!(
        ok("점수는 10\n점수에서 1 빼\n"),
        "점수 = 10\n점수 = 점수 - 1\n"
    );
}

#[test]
fn taking_away_a_word_nothing_saved_is_refused_with_a_reason() {
    assert_eq!(
        error_code("set score to 10\nremove Mina from score\n"),
        "E0221"
    );
}

// ----------------------------------------------------------- putting in order

#[test]
fn a_list_can_be_sorted_reversed_and_shuffled_in_both_languages() {
    assert_eq!(after_list_en("sort friends"), "friends.sort()");
    assert_eq!(after_list_ko("친구들 정렬해"), "친구들.sort()");
    assert_eq!(after_list_en("reverse friends"), "friends.reverse()");
    assert_eq!(after_list_ko("친구들 거꾸로 해"), "친구들.reverse()");
    assert_eq!(after_list_ko("친구들 뒤집어"), "친구들.reverse()");
    assert_eq!(
        after_list_en("shuffle friends"),
        "__import__(\"random\").shuffle(friends)"
    );
    assert_eq!(
        after_list_ko("친구들 섞어"),
        "__import__(\"random\").shuffle(친구들)"
    );
}

#[test]
fn putting_a_list_in_order_needs_a_list() {
    assert_eq!(error_code("sort friends\n"), "E0231");
    assert_eq!(error_code("친구들 정렬해\n"), "E0231");
    assert_eq!(
        error_code("set friends to Mina\nshuffle friends\n"),
        "E0231"
    );
}

#[test]
fn ordinary_sentences_that_contain_those_words_are_left_alone() {
    assert_eq!(
        ok("sort out your things\n"),
        "print(\"sort out your things\")\n"
    );
    assert_eq!(
        ok("put the car in reverse\n"),
        "print(\"put the car in reverse\")\n"
    );
    assert_eq!(
        ok("섞어 찌개를 먹었습니다\n"),
        "print(\"섞어 찌개를 먹었습니다\")\n"
    );
    // Still a sentence, and still printed. A saved name inside a sentence is
    // replaced by its value, which is what sentence output has always done.
    assert_eq!(
        ok("친구들은 목록 민수\n친구들 이야기를 들었습니다\n")
            .lines()
            .nth(1)
            .expect("a second line"),
        "print(str(친구들) + \" 이야기를 들었습니다\")"
    );
}

// -------------------------------------------------------------- positions

#[test]
fn an_item_can_be_asked_for_by_its_position() {
    assert_eq!(
        after_list_en("show the first of friends"),
        "print(friends[0])"
    );
    assert_eq!(after_list_ko("친구들 첫 번째 말해줘"), "print(친구들[0])");
    assert_eq!(after_list_ko("친구들 첫번째 말해줘"), "print(친구들[0])");
    assert_eq!(
        after_list_en("show the last of friends"),
        "print(friends[-1])"
    );
    assert_eq!(after_list_ko("친구들 마지막 말해줘"), "print(친구들[-1])");
    assert_eq!(after_list_en("show item 2 of friends"), "print(friends[1])");
    assert_eq!(after_list_ko("친구들 2번째 말해줘"), "print(친구들[1])");
}

#[test]
fn positions_are_counted_from_one_and_zero_is_refused() {
    assert_eq!(
        error_code("set friends to list of Mina\nshow item 0 of friends\n"),
        "E0229"
    );
    assert_eq!(
        error_code("친구들은 목록 민수\n친구들 0번째 말해줘\n"),
        "E0229"
    );
}

#[test]
fn the_first_of_something_that_is_not_a_list_is_a_sentence() {
    assert_eq!(ok("the first of many\n"), "print(\"the first of many\")\n");
    assert_eq!(
        ok("첫 번째 손님이었습니다\n"),
        "print(\"첫 번째 손님이었습니다\")\n"
    );
}

// --------------------------------------------------------------- arithmetic

#[test]
fn a_list_of_numbers_has_a_total_a_biggest_and_a_smallest() {
    let en = |line: &str| {
        let source = format!("set scores to list of 1, 2, 3\n{line}\n");
        ok(&source)
            .lines()
            .nth(1)
            .expect("a second line")
            .to_string()
    };
    let ko = |line: &str| {
        let source = format!("점수들은 목록 1, 2, 3\n{line}\n");
        ok(&source)
            .lines()
            .nth(1)
            .expect("a second line")
            .to_string()
    };
    assert_eq!(en("show the total of scores"), "print(sum(scores))");
    assert_eq!(ko("점수들 합 말해줘"), "print(sum(점수들))");
    assert_eq!(ko("점수들 합계 말해줘"), "print(sum(점수들))");
    assert_eq!(en("show the biggest of scores"), "print(max(scores))");
    assert_eq!(ko("점수들 중 가장 큰 것 말해줘"), "print(max(점수들))");
    assert_eq!(ko("점수들 최댓값 말해줘"), "print(max(점수들))");
    assert_eq!(en("show the smallest of scores"), "print(min(scores))");
    assert_eq!(ko("점수들 중 가장 작은 것 말해줘"), "print(min(점수들))");
}

#[test]
fn the_word_most_is_not_a_misspelled_save() {
    // `가장` is one keystroke from `저장`; this line used to become
    // `좋 = "하루였습니다"` and say nothing at all.
    assert_eq!(
        ok("가장 좋은 하루였습니다\n"),
        "print(\"가장 좋은 하루였습니다\")\n"
    );
    assert_eq!(
        ok("가장 큰 별이었습니다\n"),
        "print(\"가장 큰 별이었습니다\")\n"
    );
}

#[test]
fn the_word_how_is_not_a_misspelled_show() {
    // `how` is `show` without its `s`; this line used to print `are you`.
    assert_eq!(ok("how are you\n"), "print(\"how are you\")\n");
    assert_eq!(ok("how do you do\n"), "print(\"how do you do\")\n");
}

// ------------------------------------------------------------------ joining

#[test]
fn a_list_can_be_joined_into_one_piece_of_text() {
    assert_eq!(
        after_list_en("show friends joined by comma"),
        "print(\", \".join(map(str, friends)))"
    );
    assert_eq!(
        after_list_ko("친구들을 쉼표로 이어 말해줘"),
        "print(\", \".join(map(str, 친구들)))"
    );
    assert_eq!(
        after_list_en("show friends joined by space"),
        "print(\" \".join(map(str, friends)))"
    );
    assert_eq!(
        after_list_ko("친구들을 줄바꿈으로 이어 말해줘"),
        "print(\"\\n\".join(map(str, 친구들)))"
    );
}

// ------------------------------------------------------------------- text

#[test]
fn text_has_a_length_and_a_case_in_both_languages() {
    let en = |line: &str| {
        let source = format!("set greeting to Hello\n{line}\n");
        ok(&source)
            .lines()
            .nth(1)
            .expect("a second line")
            .to_string()
    };
    let ko = |line: &str| {
        let source = format!("인사는 안녕하세요\n{line}\n");
        ok(&source)
            .lines()
            .nth(1)
            .expect("a second line")
            .to_string()
    };
    assert_eq!(en("show the length of greeting"), "print(len(greeting))");
    assert_eq!(ko("인사 길이 말해줘"), "print(len(인사))");
    assert_eq!(
        en("show greeting in capitals"),
        "print(str(greeting).upper())"
    );
    assert_eq!(ko("인사 대문자로 말해줘"), "print(str(인사).upper())");
    assert_eq!(
        en("show greeting in small letters"),
        "print(str(greeting).lower())"
    );
    assert_eq!(ko("인사 소문자로 말해줘"), "print(str(인사).lower())");
}

#[test]
fn a_length_only_reads_a_name_the_program_made() {
    assert_eq!(
        ok("the length of the river\n"),
        "print(\"the length of the river\")\n"
    );
    assert_eq!(
        ok("이 강의 길이는 백 킬로미터입니다\n"),
        "print(\"이 강의 길이는 백 킬로미터입니다\")\n"
    );
}

// ------------------------------------------------------------ endless loop

#[test]
fn a_loop_with_no_counter_exists_in_both_languages() {
    assert_eq!(
        ok("repeat forever\nshow hi\nbreak\nend\n"),
        "while True:\n    print(\"hi\")\n    break\n# end\n"
    );
    assert_eq!(
        ok("계속 반복해\n안녕 말해줘\n멈춰\n끝\n"),
        "while True:\n    print(\"안녕\")\n    break\n# end\n"
    );
    assert_eq!(
        ok("repeat forever and show hi\n"),
        "while True: print(\"hi\")\n"
    );
    assert_eq!(
        ok("계속 반복해서 안녕 말해줘\n"),
        "while True: print(\"안녕\")\n"
    );
}

#[test]
fn skipping_and_breaking_work_inside_an_endless_loop() {
    assert_eq!(
        ok("계속 반복해\n건너뛰어\n끝\n"),
        "while True:\n    continue\n# end\n"
    );
    assert_eq!(
        ok("repeat forever\nskip\nend\n"),
        "while True:\n    continue\n# end\n"
    );
}

#[test]
fn forever_on_its_own_is_still_a_word() {
    assert_eq!(
        ok("best friends forever\n"),
        "print(\"best friends forever\")\n"
    );
    assert_eq!(ok("계속 걸었습니다\n"), "print(\"계속 걸었습니다\")\n");
}

// ------------------------------------------------------- a story wins over all

#[test]
fn nothing_inside_a_story_becomes_a_list_statement() {
    // A story still puts a saved name's value into its text, which is what
    // the story block has always done; what it must never do is read one of
    // these lines as a command.
    assert_eq!(
        ok("친구들은 목록 민수\n이야기:\n친구들 정렬해\n친구들 개수 말해줘\n끝\n"),
        "친구들 = [\"민수\"]\nif True:\n    print(str(친구들) + \" 정렬해\")\n    print(str(친구들) + \" 개수 말해줘\")\n# end\n"
    );
    assert_eq!(
        ok("set friends to list of Mina\nstory:\nsort friends\nend\n"),
        "friends = [\"Mina\"]\nif True:\n    print(\"sort \" + str(friends))\n# end\n"
    );
}

// ---------------------------------------------------------- Python wins

#[test]
fn ordinary_python_that_does_the_same_thing_is_untouched() {
    let source = "friends = []\nfriends.append(\"Mina\")\nfriends.sort()\nprint(len(friends))\n";
    assert_eq!(ok(source), source);
}

#[test]
fn a_python_list_is_also_a_list_for_the_sentence_statements() {
    assert_eq!(
        ok("friends = [\"Mina\"]\nsort friends\n"),
        "friends = [\"Mina\"]\nfriends.sort()\n"
    );
}

// -------------------------------------------------------------- remainders

#[test]
fn what_is_left_over_after_a_division_has_a_spelling() {
    let en = |line: &str| {
        let source = format!("set pile to 12\n{line}\n");
        ok(&source)
            .lines()
            .nth(1)
            .expect("a second line")
            .to_string()
    };
    let ko = |line: &str| {
        let source = format!("쌓인돌은 12\n{line}\n");
        ok(&source)
            .lines()
            .nth(1)
            .expect("a second line")
            .to_string()
    };
    assert_eq!(
        en("show the remainder of pile divided by 4"),
        "print(pile % 4)"
    );
    assert_eq!(ko("쌓인돌을 4로 나눈 나머지 말해줘"), "print(쌓인돌 % 4)");
    assert_eq!(
        en("set left to the remainder of pile divided by 4"),
        "left = pile % 4"
    );
    assert_eq!(
        ko("남은것은 쌓인돌을 4로 나눈 나머지"),
        "남은것 = 쌓인돌 % 4"
    );
}

#[test]
fn a_remainder_works_in_a_condition() {
    assert_eq!(
        ok("set pile to 12\nif the remainder of pile divided by 4 equals 0\n    show yes\n")
            .lines()
            .nth(1)
            .expect("a second line"),
        "if (pile % 4 == 0):"
    );
    assert_eq!(
        ok("쌓인돌은 12\n만약에 쌓인돌을 4로 나눈 나머지가 0과 같으면\n    네 말해줘\n")
            .lines()
            .nth(1)
            .expect("a second line"),
        "if (쌓인돌 % 4 == 0):"
    );
}

#[test]
fn a_saving_word_at_the_start_of_a_line_means_a_save() {
    // `set left to score divided by 4` used to become `set = set / 4`, which
    // bound a name nobody wrote and lost the value entirely.
    assert_eq!(
        ok("set score to 12\nset left to the remainder of score divided by 4\n"),
        "score = 12\nleft = score % 4\n"
    );
    let text = ok("set score to 12\nset left to score divided by 4\n");
    assert!(text.contains("left = "), "{text}");
    assert!(!text.contains("set = "), "{text}");
}

// -------------------------------------- the list word outside an assignment

#[test]
fn the_list_word_given_to_an_output_word_is_the_word() {
    // `목록을 보여 주세요` ("please show me the list") used to print `[]`: an
    // empty pair of brackets, and nothing to tell the writer that their
    // sentence had been read as a value instead of as words.
    assert_eq!(ok("목록 보여줘\n"), "print(\"목록\")\n");
    assert_eq!(ok("목록을 말해줘\n"), "print(\"목록\")\n");
    assert_eq!(ok("목록을 보여 주세요\n"), "print(\"목록\")\n");
    assert_eq!(ok("show list\n"), "print(\"list\")\n");
    assert_eq!(ok("빈 목록\n"), "print(\"빈 목록\")\n");
}

#[test]
fn a_sentence_that_merely_mentions_a_list_is_a_sentence() {
    assert_eq!(ok("목록이 깁니다\n"), "print(\"목록이 깁니다\")\n");
    assert_eq!(ok("이 목록은 길어요\n"), "print(\"이 목록은 길어요\")\n");
    assert_eq!(
        ok("장바구니 목록을 적었습니다\n"),
        "print(\"장바구니 목록을 적었습니다\")\n"
    );
    assert_eq!(ok("the list was long\n"), "print(\"the list was long\")\n");
}

#[test]
fn the_same_words_saved_into_a_name_are_still_an_empty_list() {
    assert_eq!(ok("친구들은 목록\n"), "친구들 = []\n");
    assert_eq!(ok("친구들은 빈 목록\n"), "친구들 = []\n");
    assert_eq!(ok("set scores to an empty list\n"), "scores = []\n");
    assert_eq!(ok("set friends to list of\n"), "friends = []\n");
    // A list with items in it still reads as one wherever a value may stand.
    assert_eq!(
        ok("show list of Mina, Ada\n"),
        "print([\"Mina\", \"Ada\"])\n"
    );
}
