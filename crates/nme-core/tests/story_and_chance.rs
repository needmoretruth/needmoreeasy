//! Two grammar forms that carry their own safety argument.
//!
//! A **story block** (`이야기:` / `story:`) turns off every command word
//! inside it, so a page of prose is a page of prose. A **chance**
//! (`30% 확률로` / `30% chance`) lets a beginner say how often something
//! happens in the unit they already think in, without meeting
//! `random.random() < 0.3`.
//!
//! Both must exist in English and in Korean, both must lower to exactly one
//! Python line per NME line, and neither may take a sentence that merely
//! mentions a story or a percentage.

use nme_core::transpile;

fn ok(source: &str) -> String {
    transpile(source)
        .unwrap_or_else(|problems| panic!("expected successful transpile, got: {problems:?}"))
}

fn error_code(source: &str) -> String {
    let problems = transpile(source).expect_err("expected this line to be rejected");
    problems[0].code.code().to_string()
}

/// The exact Python a chance produces, so every expectation below reads as
/// the one thing that must not drift.
fn chance(permille: u32) -> String {
    format!("__import__(\"random\").randrange(1000) < {permille}")
}

// -------------------------------------------------------------- the story

#[test]
fn a_story_block_prints_every_line_in_both_languages() {
    assert_eq!(
        ok("이야기:\n    문이 천천히 열렸습니다.\n    아무도 없었습니다.\n끝\n"),
        "if True:\n    print(\"문이 천천히 열렸습니다.\")\n    print(\"아무도 없었습니다.\")\n# end\n"
    );
    assert_eq!(
        ok("story:\n    The door opened slowly.\n    Nobody was there.\nend\n"),
        "if True:\n    print(\"The door opened slowly.\")\n    print(\"Nobody was there.\")\n# end\n"
    );
}

#[test]
fn a_story_block_may_be_written_flat() {
    assert_eq!(
        ok("이야기:\n문이 열렸습니다.\n끝\n"),
        "if True:\n    print(\"문이 열렸습니다.\")\n# end\n"
    );
    assert_eq!(
        ok("story:\nThe door opened.\nend\n"),
        "if True:\n    print(\"The door opened.\")\n# end\n"
    );
}

#[test]
fn a_story_block_accepts_spaces_and_the_full_width_colon() {
    // A Korean IME writes `：`, which Python cannot read at all.
    assert_eq!(
        ok("이야기：\n    문이 열렸습니다.\n끝\n"),
        "if True:\n    print(\"문이 열렸습니다.\")\n# end\n"
    );
    assert_eq!(
        ok("story ：\n    The door opened.\nend\n"),
        "if True:\n    print(\"The door opened.\")\n# end\n"
    );
    assert_eq!(
        ok("이야기 :\n    문이 열렸습니다.\n끝\n"),
        "if True:\n    print(\"문이 열렸습니다.\")\n# end\n"
    );
}

#[test]
fn a_slow_story_tells_every_line_one_character_at_a_time() {
    let slowly = |seconds: &str, text: &str| {
        format!(
            "[print(_ch, end=\"\", flush=True) or __import__(\"time\").sleep({seconds}) for _ch in \"{text}\"]; print()"
        )
    };
    assert_eq!(
        ok("천천히 이야기:\n    문이 열렸습니다.\n끝\n"),
        format!(
            "if True:\n    {}\n# end\n",
            slowly("0.04", "문이 열렸습니다.")
        )
    );
    assert_eq!(
        ok("아주 천천히 이야기:\n    문이 열렸습니다.\n끝\n"),
        format!(
            "if True:\n    {}\n# end\n",
            slowly("0.12", "문이 열렸습니다.")
        )
    );
    assert_eq!(
        ok("0.2초씩 천천히 이야기:\n    문이 열렸습니다.\n끝\n"),
        format!(
            "if True:\n    {}\n# end\n",
            slowly("0.2", "문이 열렸습니다.")
        )
    );
    assert_eq!(
        ok("slow story:\n    The door opened.\nend\n"),
        format!(
            "if True:\n    {}\n# end\n",
            slowly("0.04", "The door opened.")
        )
    );
    assert_eq!(
        ok("very slow story:\n    The door opened.\nend\n"),
        format!(
            "if True:\n    {}\n# end\n",
            slowly("0.12", "The door opened.")
        )
    );
    assert_eq!(
        ok("slow story every 0.2 seconds:\n    The door opened.\nend\n"),
        format!(
            "if True:\n    {}\n# end\n",
            slowly("0.2", "The door opened.")
        )
    );
}

/// The rule the whole form exists for: inside a story block there are no
/// commands at all, so a line of prose can never quietly become a statement.
#[test]
fn nothing_inside_a_story_block_is_a_command() {
    assert_eq!(
        ok("이야기:\n    3초 기다려\n    만약에 준비가 있으면\n    빨강 또는 초록 중에서 골라\n끝\n"),
        concat!(
            "if True:\n",
            "    print(\"3초 기다려\")\n",
            "    print(\"만약에 준비가 있으면\")\n",
            "    print(\"빨강 또는 초록 중에서 골라\")\n",
            "# end\n"
        )
    );
    assert_eq!(
        ok("story:\n    wait 3 seconds\n    if ready then show hi\n    x = 1\nend\n"),
        concat!(
            "if True:\n",
            "    print(\"wait 3 seconds\")\n",
            "    print(\"if ready then show hi\")\n",
            "    print(\"x = 1\")\n",
            "# end\n"
        )
    );
}

#[test]
fn a_blank_line_inside_a_story_prints_an_empty_line() {
    assert_eq!(
        ok("이야기:\n    첫줄입니다.\n\n    둘째줄입니다.\n끝\n"),
        "if True:\n    print(\"첫줄입니다.\")\n    print()\n    print(\"둘째줄입니다.\")\n# end\n"
    );
    assert_eq!(
        ok("story:\nFirst.\n\nSecond.\nend\n"),
        "if True:\n    print(\"First.\")\n    print()\n    print(\"Second.\")\n# end\n"
    );
}

#[test]
fn a_comment_inside_a_story_stays_a_comment() {
    assert_eq!(
        ok("이야기:\n    첫줄입니다.\n    # 메모\n끝\n"),
        "if True:\n    print(\"첫줄입니다.\")\n    # 메모\n# end\n"
    );
}

#[test]
fn a_name_is_interpolated_inside_a_story() {
    assert_eq!(
        ok("이름은 민수\n이야기:\n    안녕 이름!\n끝\n"),
        "이름 = \"민수\"\nif True:\n    print(\"안녕 \" + str(이름) + \"!\")\n# end\n"
    );
    // Exactly what the same sentence produces with the output word on it.
    assert_eq!(
        ok("이름은 민수\n안녕 이름! 말해줘\n"),
        "이름 = \"민수\"\nprint(\"안녕 \" + str(이름) + \"!\")\n"
    );
    assert_eq!(
        ok("set name to Mina\nstory:\n    Hello name!\nend\n"),
        "name = \"Mina\"\nif True:\n    print(\"Hello \" + str(name) + \"!\")\n# end\n"
    );
}

#[test]
fn a_story_block_nests_inside_a_condition_block() {
    assert_eq!(
        ok("준비는 참\n만약에 준비가 있으면\n    이야기:\n        문이 열렸습니다.\n끝\n"),
        "준비 = True\nif (준비):\n    if True:\n        print(\"문이 열렸습니다.\")\n# end\n"
    );
    assert_eq!(
        ok("set ready to True\nif ready\n    story:\n        The door opened.\nend\n"),
        "ready = True\nif (ready):\n    if True:\n        print(\"The door opened.\")\n# end\n"
    );
}

#[test]
fn an_indented_story_block_ends_at_the_dedent() {
    assert_eq!(
        ok("이야기:\n    문이 열렸습니다.\n안녕 말해줘\n"),
        "if True:\n    print(\"문이 열렸습니다.\")\nprint(\"안녕\")\n"
    );
    // …and at the end of the file, which is the plainest dedent of all.
    assert_eq!(
        ok("story:\n    The door opened.\n"),
        "if True:\n    print(\"The door opened.\")\n"
    );
}

#[test]
fn a_flat_story_block_still_needs_its_end() {
    assert_eq!(error_code("이야기:\n문이 열렸습니다.\n"), "E0105");
}

/// Ordinary sentences that merely mention a story. The trailing colon is the
/// only thing that opens a story block, and none of these has one.
#[test]
fn a_sentence_about_a_story_stays_a_sentence() {
    assert_eq!(ok("이야기를 들려줘\n"), "print(\"이야기를 들려줘\")\n");
    assert_eq!(ok("옛날 이야기\n"), "print(\"옛날 이야기\")\n");
    assert_eq!(ok("story time\n"), "print(\"story time\")\n");
    assert_eq!(
        ok("재미있는 이야기입니다\n"),
        "print(\"재미있는 이야기입니다\")\n"
    );
}

// ------------------------------------------------------------- the chance

#[test]
fn a_chance_runs_one_statement_in_both_languages() {
    assert_eq!(
        ok("30% 확률로 말해줘 당첨\n"),
        format!("if {}: print(\"당첨\")\n", chance(300))
    );
    assert_eq!(
        ok("30% chance show You win\n"),
        format!("if {}: print(\"You win\")\n", chance(300))
    );
}

#[test]
fn a_chance_opens_a_block_in_both_languages() {
    assert_eq!(
        ok("30% 확률로\n    당첨 말해줘\n끝\n"),
        format!("if {}:\n    print(\"당첨\")\n# end\n", chance(300))
    );
    assert_eq!(
        ok("30% chance\n    show You win\nend\n"),
        format!("if {}:\n    print(\"You win\")\n# end\n", chance(300))
    );
    // The same block written flat.
    assert_eq!(
        ok("30% 확률로\n당첨 말해줘\n끝\n"),
        format!("if {}:\n    print(\"당첨\")\n# end\n", chance(300))
    );
}

#[test]
fn a_chance_may_name_one_decimal_place() {
    assert_eq!(
        ok("30.5% 확률로 말해줘 당첨\n"),
        format!("if {}: print(\"당첨\")\n", chance(305))
    );
    assert_eq!(
        ok("30.5% chance show You win\n"),
        format!("if {}: print(\"You win\")\n", chance(305))
    );
}

/// An integer percentage carries no decimal at all: 50% is 500 thousandths,
/// `100%` always happens and `0%` never does, and no float is compared.
#[test]
fn whole_percentages_stay_whole_numbers() {
    assert_eq!(
        ok("50% 확률로 말해줘 당첨\n"),
        format!("if {}: print(\"당첨\")\n", chance(500))
    );
    assert_eq!(
        ok("100% chance show You win\n"),
        format!("if {}: print(\"You win\")\n", chance(1000))
    );
    assert_eq!(
        ok("0% chance show You win\n"),
        format!("if {}: print(\"You win\")\n", chance(0))
    );
}

#[test]
fn a_chance_can_be_saved_in_a_name_and_asked_about_later() {
    assert_eq!(
        ok("운은 30% 확률\n만약에 운이 있으면 당첨 말해줘\n"),
        format!("운 = {}\nif (운): print(\"당첨\")\n", chance(300))
    );
    assert_eq!(
        ok("luck is a 30% chance\nif luck then show You win\n"),
        format!("luck = {}\nif (luck): print(\"You win\")\n", chance(300))
    );
    assert_eq!(
        ok("set luck to a 30% chance\n"),
        format!("luck = {}\n", chance(300))
    );
}

#[test]
fn the_tolerant_chance_spellings_all_work() {
    let korean = format!("if {}: print(\"당첨\")\n", chance(300));
    assert_eq!(ok("30%의 확률로 말해줘 당첨\n"), korean);
    assert_eq!(ok("확률 30%로 말해줘 당첨\n"), korean);
    assert_eq!(ok("30 % 확률로 말해줘 당첨\n"), korean);
    assert_eq!(ok("30퍼센트 확률로 말해줘 당첨\n"), korean);
    assert_eq!(ok("30 프로 확률로 말해줘 당첨\n"), korean);

    let english = format!("if {}: print(\"You win\")\n", chance(300));
    assert_eq!(ok("with a 30% chance show You win\n"), english);
    assert_eq!(ok("a 30% chance show You win\n"), english);
    assert_eq!(ok("30 percent chance show You win\n"), english);
    assert_eq!(ok("30% of the time show You win\n"), english);
    assert_eq!(ok("30% probability show You win\n"), english);
}

/// Rounding `30.25%` to `30.3%` would make the program mean something its
/// writer did not write, so it is refused instead.
#[test]
fn a_chance_finer_than_one_decimal_place_is_refused() {
    assert_eq!(error_code("30.25% 확률로 말해줘 당첨\n"), "E0227");
    assert_eq!(error_code("30.25% chance show You win\n"), "E0227");
    assert_eq!(error_code("운은 30.25% 확률\n"), "E0227");
    assert_eq!(error_code("luck is a 30.25% chance\n"), "E0227");

    let problems = transpile("30.25% chance show You win\n").expect_err("refused");
    assert!(problems[0]
        .message
        .contains("a chance can only go to one decimal place"));
    assert!(problems[0]
        .message_ko
        .as_deref()
        .is_some_and(|korean| korean.contains("소수점 첫째 자리")));
    assert!(problems[0]
        .hint
        .as_deref()
        .is_some_and(|hint| hint.contains("30.3% instead of 30.25%")));
}

#[test]
fn a_chance_outside_zero_to_a_hundred_is_refused() {
    assert_eq!(error_code("150% 확률로 말해줘 당첨\n"), "E0228");
    assert_eq!(error_code("101% chance show You win\n"), "E0228");
    assert_eq!(error_code("-5% chance show You win\n"), "E0228");
    assert_eq!(error_code("운은 150% 확률\n"), "E0228");

    let problems = transpile("101% chance show You win\n").expect_err("refused");
    assert!(problems[0]
        .message
        .contains("a chance must be between 0% and 100%"));
    assert!(problems[0]
        .message_ko
        .as_deref()
        .is_some_and(|korean| korean.contains("0%부터 100% 사이")));
}

/// A percentage on its own is never a chance. These three are the sentences
/// that made the rule: each of them mentions a percentage and means nothing
/// of the sort.
#[test]
fn a_sentence_with_a_percentage_in_it_is_not_a_chance() {
    for source in [
        "100% 확신합니다\n",
        "I am 100% sure\n",
        "전체의 30%가 왔습니다\n",
        "시험은 30% 남았습니다\n",
    ] {
        let produced = transpile(source).unwrap_or_default();
        assert!(
            !produced.contains("randrange"),
            "{source:?} became a chance: {produced}"
        );
    }
}

#[test]
fn a_name_the_program_saved_keeps_its_python_meaning() {
    // `chance = 7` makes `30 % chance` ordinary arithmetic again, and Python
    // wins as it always does.
    assert_eq!(ok("chance = 7\n30 % chance\n"), "chance = 7\n30 % chance\n");
}

// --------------------------------------------------- the two forms together

#[test]
fn a_chance_may_open_a_story() {
    assert_eq!(
        ok("30% 확률로\n이야기:\n문이 열렸습니다.\n끝\n끝\n"),
        format!(
            "if {}:\n    if True:\n        print(\"문이 열렸습니다.\")\n    # end\n# end\n",
            chance(300)
        )
    );
}

/// Every NME statement is exactly one Python line, so a line number in a
/// traceback is the line number in the `.nme` file.
#[test]
fn both_forms_keep_one_nme_line_to_one_python_line() {
    for source in [
        "이야기:\n    문이 열렸습니다.\n\n    끝났습니다.\n끝\n",
        "story:\n    The door opened.\n\n    The end.\nend\n",
        "30% 확률로\n    당첨 말해줘\n끝\n",
        "30% chance\n    show You win\nend\n",
    ] {
        assert_eq!(
            ok(source).lines().count(),
            source.lines().count(),
            "line count changed for {source:?}"
        );
    }
}

// ------------------------------------------- a percentage that is not one

/// `%` is Python's modulo operator and a Korean word is a valid Python name,
/// so `100% 확신합니다` really is a syntactically valid Python expression.
/// Handing it to Python means the writer meets a `NameError` at run time
/// instead of reading their own sentence.
#[test]
fn an_ordinary_sentence_with_a_percentage_prints_itself() {
    assert_eq!(ok("100% 확신합니다\n"), "print(\"100% 확신합니다\")\n");
    assert_eq!(
        ok("나는 100% 동의합니다\n"),
        "print(\"나는 100% 동의합니다\")\n"
    );
    assert_eq!(
        ok("전체의 30%가 왔습니다\n"),
        "print(\"전체의 30%가 왔습니다\")\n"
    );
    assert_eq!(
        ok("오늘 30% 할인합니다\n"),
        "print(\"오늘 30% 할인합니다\")\n"
    );
}

/// The words that follow a chance have to be a command. Without that rule
/// every sentence that mentions a percentage runs its own tail three times
/// in ten, and `a 20% chance remains` compiles to `if …: remains`.
#[test]
fn a_sentence_about_a_chance_is_not_a_chance() {
    assert_eq!(
        ok("확률 30%는 낮습니다\n"),
        "print(\"확률 30%는 낮습니다\")\n"
    );
    assert_eq!(
        ok("확률 30% 정도입니다\n"),
        "print(\"확률 30% 정도입니다\")\n"
    );
    for source in [
        "확률 100% 입니다\n",
        "확률 200% 입니다\n",
        "a 30% chance of rain today\n",
        "a 20% chance remains\n",
        "the 20% chance is small\n",
        "a 33.33% chance of winning\n",
        "30% of the time it rains\n",
    ] {
        let produced = transpile(source).unwrap_or_default();
        assert!(
            !produced.contains("randrange"),
            "{source:?} became a chance: {produced}"
        );
    }
}

/// The word-first spelling needs its particle, which is the difference
/// between a command and a remark about a percentage.
#[test]
fn the_word_first_korean_chance_needs_its_particle() {
    assert_eq!(
        ok("확률 30%로 말해줘 당첨\n"),
        format!("if {}: print(\"당첨\")\n", chance(300))
    );
    assert_eq!(
        ok("확률 30% 남았습니다\n"),
        "print(\"확률 30% 남았습니다\")\n"
    );
}

/// `재미있는` ends in the Korean topic particle, so this line was being saved
/// under the name `재미있` and printing nothing at all.
#[test]
fn a_label_and_its_text_prints_instead_of_becoming_a_name() {
    assert_eq!(
        ok("재미있는 이야기: 시작\n"),
        "print(\"재미있는 이야기: 시작\")\n"
    );
    assert_eq!(ok("제목: 오늘 할 일\n"), "print(\"제목: 오늘 할 일\")\n");
    assert_eq!(
        ok("note: remember this\n"),
        "print(\"note: remember this\")\n"
    );
    // A value with a colon that Python can read is still a value.
    assert_eq!(ok("이름은 {\"a\": 1}\n"), "이름 = {\"a\": 1}\n");
}

/// English has no written sentence ending to key on, so a sentence with a
/// digit in it is still reported rather than printed. What must hold is that
/// it never becomes a chance and never becomes a silent assignment.
#[test]
fn english_prose_with_a_percentage_is_never_a_chance() {
    for source in ["I am 100% sure\n", "the battery is at 50% now\n"] {
        let produced = transpile(source).unwrap_or_default();
        assert!(
            !produced.contains("randrange"),
            "{source:?} became a chance: {produced}"
        );
    }
}

/// Saving a value is unchanged: a number spoken as a sentence is a number.
#[test]
fn a_number_spoken_as_a_sentence_is_still_saved() {
    assert_eq!(ok("점수는 0입니다\n"), "점수 = 0\n");
    assert_eq!(ok("점수는 0\n"), "점수 = 0\n");
    assert_eq!(ok("이름은 민수\n"), "이름 = \"민수\"\n");
}

/// `할인율은 30%입니다` was saved as `할인율 = 30%입니다`, which Python reads as
/// `30 % 입니다`: a modulo against a name nothing ever bound. It compiled, and
/// then raised `NameError` the moment it ran.
#[test]
fn a_number_with_a_unit_after_it_is_a_sentence() {
    for (source, printed) in [
        ("할인율은 30%입니다\n", "할인율은 30%입니다"),
        ("할인율은 30% 입니다\n", "할인율은 30% 입니다"),
        ("비율은 30%이다\n", "비율은 30%이다"),
        ("점수는 30점입니다\n", "점수는 30점입니다"),
        ("가격은 1000원입니다\n", "가격은 1000원입니다"),
        ("나이는 12살입니다\n", "나이는 12살입니다"),
        ("거리는 3km입니다\n", "거리는 3km입니다"),
        ("온도는 25도입니다\n", "온도는 25도입니다"),
        ("이름은 민수입니다\n", "이름은 민수입니다"),
    ] {
        assert_eq!(ok(source), format!("print(\"{printed}\")\n"));
    }
}

/// The exception, and it is deliberate: the bare number is the whole value,
/// and the ending is only how the writer spoke it.
#[test]
fn a_bare_number_spoken_as_a_sentence_is_still_saved() {
    assert_eq!(ok("정답은 7입니다\n"), "정답 = 7\n");
    assert_eq!(ok("정답은 7.5입니다\n"), "정답 = 7.5\n");
    assert_eq!(ok("점수는 0입니다\n"), "점수 = 0\n");
    assert_eq!(ok("점수는 0이다\n"), "점수 = 0\n");
    assert_eq!(ok("합계는 10 입니다\n"), "합계 = 10\n");
}
