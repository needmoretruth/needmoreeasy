//! The holes the 2026-08-18 parity audit found, and the shapes that close them.
//!
//! Every one of these was a place where one language, or one level, could do
//! something the other could not — and in the worst of them the attempt was
//! not refused but quietly read as text. Each test names the old behaviour so
//! that a regression reads as the bug it is.

use nme_core::transpile;

fn ok(source: &str) -> String {
    transpile(source)
        .unwrap_or_else(|problems| panic!("expected successful transpile, got: {problems:?}"))
}

fn error_code(source: &str) -> String {
    let problems = transpile(source).expect_err("expected this line to be rejected");
    problems[0].code.code().to_string()
}

/// The last line of a program, which is the one each of these is about.
fn last(source: &str) -> String {
    ok(source)
        .lines()
        .last()
        .expect("at least one line")
        .to_string()
}

// ------------------------------------------- English zero knowledge, at last

#[test]
fn the_five_missing_english_zero_knowledge_forms_are_not_text() {
    // `set ok to p c e z zero knowledge verify` used to save a *sentence*.
    let line = last(concat!(
        "use zero_knowledge\n",
        "set p to 1\nset c to 2\nset e to 3\nset z to 4\n",
        "set ok to p c e z zero knowledge verify\n"
    ));
    assert!(line.starts_with("ok = (1 < (p) <"), "{line}");

    let line = last(concat!(
        "use zero_knowledge\nset e to zero knowledge challenge make\n",
        "set f to e different zero knowledge challenge make\n"
    ));
    assert!(line.contains("randbelow((1 << 256) - 1)"), "{line}");

    let line = last(concat!(
        "use zero_knowledge\nset r to 1\nset s to 2\nset e to 3\n",
        "set z to r s e zero knowledge response make\n"
    ));
    assert!(line.starts_with("z = ((r) - (s) * (e)) %"), "{line}");

    let line = last("use zero_knowledge\nset z to zero knowledge simulated response make\n");
    assert!(
        line.starts_with("z = __import__(\"secrets\").randbelow("),
        "{line}"
    );

    let line = last(concat!(
        "use zero_knowledge\nset p to 1\nset e to 2\nset z to 3\n",
        "set c to p e z zero knowledge simulated commitment make\n"
    ));
    assert!(line.starts_with("c = (pow(2, (z),"), "{line}");
}

#[test]
fn the_english_and_korean_zero_knowledge_forms_produce_the_same_python() {
    let english = last(concat!(
        "use zero_knowledge\nset p to 1\nset c to 2\nset e to 3\nset z to 4\n",
        "set ok to p c e z zero knowledge verify\n"
    ));
    let korean = last(concat!(
        "영지식 사용\np는 1\nc는 2\ne는 3\nz는 4\n",
        "ok는 p와 c와 e와 z로 영지식 검증\n"
    ));
    assert_eq!(english, korean);
}

// -------------------------------------- loop control inside an indented block

#[test]
fn skip_and_break_work_inside_an_indented_beginner_loop() {
    // These used to be left as bare Python names: the program compiled and
    // then raised `NameError` on a line that looked right.
    assert_eq!(
        ok("2 times:\n    skip\n"),
        "for _ in range(2):\n    continue\n"
    );
    assert_eq!(
        ok("2번:\n    건너뛰어\n"),
        "for _ in range(2):\n    continue\n"
    );
    assert_eq!(ok("2번:\n    멈춰\n"), "for _ in range(2):\n    break\n");
    // Python's own `continue` was refused here even though `break` was not.
    assert_eq!(
        ok("2 times:\n    continue\n"),
        "for _ in range(2):\n    continue\n"
    );
}

#[test]
fn a_skip_outside_any_loop_is_still_left_alone() {
    // `skip` is an ordinary Python name at the top level, and the Python-wins
    // rule keeps it one.
    assert_eq!(ok("skip = 1\nprint(skip)\n"), "skip = 1\nprint(skip)\n");
}

// ------------------------------------------- branches inside an indented block

#[test]
fn korean_branches_work_inside_an_indented_block() {
    assert_eq!(
        ok("score = 0\n만약 score > 10:\n    print(1)\n아니면:\n    print(2)\n"),
        "score = 0\nif (score > 10):\n    print(1)\nelse:\n    print(2)\n"
    );
    assert_eq!(
        ok("score = 0\n만약 score > 10:\n    print(1)\n아니면 만약에 score == 0:\n    print(2)\n"),
        "score = 0\nif (score > 10):\n    print(1)\nelif (score == 0):\n    print(2)\n"
    );
}

#[test]
fn english_branches_work_there_too_without_borrowing_python_spelling() {
    assert_eq!(
        ok("score = 0\nwhen score > 10:\n    print(1)\nelse if score == 0:\n    print(2)\n"),
        "score = 0\nif (score > 10):\n    print(1)\nelif (score == 0):\n    print(2)\n"
    );
}

#[test]
fn an_ordinary_python_block_keeps_its_own_branches() {
    let source = "score = 0\nif score > 10:\n    print(1)\nelse:\n    print(2)\n";
    assert_eq!(ok(source), source);
}

// ------------------------------------------------- importing another program

#[test]
fn another_nme_program_can_be_imported_in_a_sentence() {
    assert_eq!(
        ok("use greet from \"helper.nme\"\n"),
        "from helper import greet\n"
    );
    assert_eq!(
        ok("\"helper.nme\"에서 greet 가져와\n"),
        "from helper import greet\n"
    );
    assert_eq!(
        ok("\"helper.nme\"에서 greet 가져오기\n"),
        "from helper import greet\n"
    );
    assert_eq!(
        ok("use greet, score from \"helper.nme\"\n"),
        "from helper import greet, score\n"
    );
    assert_eq!(
        ok("\"helper.nme\"에서 greet, score 불러와\n"),
        "from helper import greet, score\n"
    );
}

#[test]
fn the_bundled_module_statement_is_untouched_by_the_import_spelling() {
    assert!(ok("use random\n").starts_with("import random as "));
    assert!(ok("랜덤 사용\n").starts_with("import random as "));
}

// ------------------------------------------------------- ask with a question

#[test]
fn ask_may_be_followed_by_the_question_itself() {
    assert_eq!(
        ok("ask what is your name\n"),
        "name = input(\"what is your name\" + \" \")\n"
    );
    assert_eq!(
        ok("ask how old are you\n"),
        "age = int(input(\"how old are you\" + \" \"))\n"
    );
    // Korean used to keep the particle and save into `이름이`, or into `몇`.
    assert_eq!(
        ok("물어봐 이름이 뭐예요?\n"),
        "이름 = input(\"이름이 뭐예요?\" + \" \")\n"
    );
    assert_eq!(
        ok("물어봐 몇 살이에요?\n"),
        "나이 = int(input(\"몇 살이에요?\" + \" \"))\n"
    );
}

#[test]
fn a_question_with_no_name_to_save_into_is_still_refused() {
    assert_eq!(error_code("ask who is there\n"), "E0213");
    assert_eq!(error_code("ask number who is there\n"), "E0213");
}

#[test]
fn naming_the_answer_yourself_still_wins() {
    assert_eq!(
        ok("ask friend What is your name?\n"),
        "friend = input(\"What is your name?\" + \" \")\n"
    );
    assert_eq!(
        ok("이름을 물어봐 이름이 뭐예요?\n"),
        "이름 = input(\"이름이 뭐예요?\" + \" \")\n"
    );
}

// ----------------------------------------------------- a name with a space

#[test]
fn a_list_name_written_as_two_words_is_refused_not_printed() {
    // Guide 05 taught `할 일은 목록`, and the learner saw their own sentence.
    assert_eq!(error_code("할 일은 목록\n"), "E0230");
    assert_eq!(error_code("할 일은 빈 목록\n"), "E0230");
    assert_eq!(error_code("할 일에 청소 넣어\n"), "E0230");
}

#[test]
fn the_joined_spelling_is_the_one_that_works() {
    assert_eq!(ok("할일은 목록\n"), "할일 = []\n");
    assert_eq!(
        ok("할일은 목록\n할일에 청소 넣어\n"),
        "할일 = []\n할일.append(\"청소\")\n"
    );
}

#[test]
fn an_ordinary_two_word_sentence_still_prints() {
    // Only a *list* line is claimed, so a plain assignment-shaped sentence
    // keeps printing.
    assert_eq!(ok("오늘 날씨는 맑음\n"), "print(\"오늘 날씨는 맑음\")\n");
    assert_eq!(ok("좋은 아침입니다\n"), "print(\"좋은 아침입니다\")\n");
    assert_eq!(
        ok("설탕을 그릇에 넣어\n"),
        "print(\"설탕을 그릇에 넣어\")\n"
    );
}

// ------------------------------------------------------ story near-misses

#[test]
fn a_story_with_nothing_in_it_is_named_not_handed_to_cpython() {
    // `story:` alone became `if True:` with no body. CPython refused it, and
    // the reader met a `SyntaxError` with a caret in generated code.
    assert_eq!(error_code("story:\n"), "E0232");
    assert_eq!(error_code("이야기:\n"), "E0232");
    assert_eq!(error_code("이야기:\n끝\n"), "E0232");
    assert_eq!(error_code("story:\nend\n"), "E0232");
}

#[test]
fn a_story_word_annotated_with_another_word_is_named() {
    // `얘기: 그만하자` is a valid Python annotation: it compiled, did nothing,
    // and said nothing.
    assert_eq!(error_code("얘기: 그만하자\n"), "E0604");
    assert_eq!(error_code("이야기: 그만하자\n"), "E0604");
    assert_eq!(error_code("story: chapter\n"), "E0604");
}

#[test]
fn a_real_story_still_tells_itself() {
    assert_eq!(
        ok("이야기:\n옛날 옛적에\n끝\n"),
        "if True:\n    print(\"옛날 옛적에\")\n# end\n"
    );
    assert_eq!(
        ok("story:\nOnce upon a time\nend\n"),
        "if True:\n    print(\"Once upon a time\")\n# end\n"
    );
    // A label with a colon in the middle of a sentence is still writing.
    assert_eq!(ok("story: the end\n"), "print(\"story: the end\")\n");
}
