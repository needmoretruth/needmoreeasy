//! A record, and a named job — the two things Part 2 of the guides was waiting
//! for.
//!
//! Both share their whole vocabulary with ordinary language, so both are built
//! on something a word cannot give them:
//!
//! * a **record** is told from a list by the *kind the name holds*, never by
//!   the wording. `개수`, `빼`, `넣어`, `…에 …가 있으면` and `…마다 반복해` are
//!   the same words for both, and the compiler branches on the name.
//! * a **job** is told from prose by *structure*: a closing colon and a block
//!   underneath. `일`, `하기`, `to` and `do` carry no meaning on their own.
//!
//! Most of what is below is therefore about what must **not** happen.

use nme_core::transpile;

fn ok(source: &str) -> String {
    transpile(source)
        .unwrap_or_else(|problems| panic!("expected successful transpile, got: {problems:?}"))
}

fn error_code(source: &str) -> String {
    let problems = transpile(source).expect_err("expected this line to be rejected");
    problems[0].code.code().to_string()
}

/// The line a program produces after the line that made the record.
fn after_record_en(line: &str) -> String {
    let source = format!("set ages to an empty record\n{line}\n");
    ok(&source)
        .lines()
        .nth(1)
        .expect("a second line")
        .to_string()
}

fn after_record_ko(line: &str) -> String {
    let source = format!("나이표는 빈 표\n{line}\n");
    ok(&source)
        .lines()
        .nth(1)
        .expect("a second line")
        .to_string()
}

// ------------------------------------------------------------------ records

#[test]
fn a_record_is_made_in_both_languages_and_at_both_levels() {
    assert_eq!(ok("set ages to an empty record\n"), "ages = {}\n");
    assert_eq!(ok("set ages to record\n"), "ages = {}\n");
    assert_eq!(ok("set ages to an empty table\n"), "ages = {}\n");
    assert_eq!(ok("나이표는 빈 표\n"), "나이표 = {}\n");
    assert_eq!(ok("나이표는 표\n"), "나이표 = {}\n");
    // Beginner and advanced write the braces themselves.
    assert_eq!(ok("save ages to {}\n"), "ages = {}\n");
    assert_eq!(ok("저장 나이표 {}\n"), "나이표 = {}\n");
}

#[test]
fn a_value_is_written_under_a_name() {
    assert_eq!(
        after_record_en("put Mina at 90 in ages"),
        "ages[\"Mina\"] = 90"
    );
    assert_eq!(
        after_record_en("put \"Mina\" at 90 in ages"),
        "ages[\"Mina\"] = 90"
    );
    assert_eq!(
        after_record_ko("나이표에 민수를 90으로 넣어"),
        "나이표[\"민수\"] = 90"
    );
    assert_eq!(
        after_record_ko("나이표에 민수를 90으로 두어"),
        "나이표[\"민수\"] = 90"
    );
    assert_eq!(
        after_record_ko("나이표에 \"민수\"를 90으로 넣어"),
        "나이표[\"민수\"] = 90"
    );
    // The value may be a name, and the name it is written under may be one too.
    assert_eq!(
        ok("set ages to an empty record\nset who to Mina\nput who at 90 in ages\n")
            .lines()
            .nth(2)
            .expect("a third line"),
        "ages[who] = 90"
    );
}

#[test]
fn one_value_is_read_back_out() {
    assert_eq!(
        after_record_en("show Mina in ages"),
        "print(ages[\"Mina\"])"
    );
    assert_eq!(
        after_record_ko("나이표의 민수 말해줘"),
        "print(나이표[\"민수\"])"
    );
    assert_eq!(
        after_record_ko("나이표에서 민수 말해줘"),
        "print(나이표[\"민수\"])"
    );
    // Saved into another name, and compared in a condition.
    assert_eq!(
        after_record_en("set best to Mina in ages"),
        "best = ages[\"Mina\"]"
    );
    // The same six words with a number where the name was. A beginner writing
    // `set Mina to 90 in ages` means to write 90 down under Mina, and used to
    // get `Mina = ages[90]` and a `KeyError` at run time. What follows the
    // saving word tells the two apart: a number or a quoted string is a value,
    // a word is a name. A record kept under numbers is still readable through
    // a name.
    assert_eq!(after_record_en("set Mina to 90 in ages"), "ages[\"Mina\"] = 90");
    assert_eq!(
        after_record_en("set Mina to \"ninety\" in ages"),
        "ages[\"Mina\"] = \"ninety\""
    );
    assert!(
        ok("set ages to an empty record\nset key to 90\nset who to key in ages\n")
            .contains("who = ages[key]")
    );
    assert!(
        ok("나이표는 빈 표\n만약에 나이표의 민수가 90보다 크면\nhi 말해줘\n끝\n")
            .contains("if (나이표[\"민수\"] > 90):")
    );
    assert!(
        ok("set ages to an empty record\nif Mina in ages is greater than 90\nshow hi\nend\n")
            .contains("if (ages[\"Mina\"] > 90):")
    );
}

#[test]
fn the_loop_variable_reads_the_record_it_is_looping_over() {
    // This is the shape every guide that uses a record needs.
    assert_eq!(
        ok("set ages to an empty record\nfor each name in ages\nshow name in ages\nend\n"),
        "ages = {}\nfor name in ages:\n    print(ages[name])\n# end\n"
    );
    assert_eq!(
        ok("나이표는 빈 표\n나이표의 이름마다 반복해\n나이표의 이름 말해줘\n끝\n"),
        "나이표 = {}\nfor 이름 in 나이표:\n    print(나이표[이름])\n# end\n"
    );
}

#[test]
fn the_list_spellings_mean_the_record_thing_when_the_name_is_a_record() {
    // Counting.
    assert_eq!(after_record_en("show how many ages"), "print(len(ages))");
    assert_eq!(after_record_ko("나이표 개수 말해줘"), "print(len(나이표))");
    // Holding a name.
    assert!(
        ok("set ages to an empty record\nif ages contains Mina\nshow hi\nend\n")
            .contains("if (\"Mina\" in ages):")
    );
    assert!(
        ok("나이표는 빈 표\n만약에 나이표에 민수가 있으면\nhi 말해줘\n끝\n")
            .contains("if (\"민수\" in 나이표):")
    );
    // Taking one out. A record has no `.remove`, so this must be `del`.
    assert_eq!(
        after_record_en("remove Mina from ages"),
        "del ages[\"Mina\"]"
    );
    assert_eq!(
        after_record_ko("나이표에서 민수 빼"),
        "del 나이표[\"민수\"]"
    );
}

#[test]
fn the_readings_that_mean_nothing_for_a_record_are_refused() {
    // A record has no order and nothing to add up. Refusing says so; handing
    // back `sum({})` would be a program nobody wrote.
    for line in [
        "show the total of ages",
        "show the biggest of ages",
        "show the first of ages",
        "sort ages",
    ] {
        let source = format!("set ages to an empty record\n{line}\n");
        let python = transpile(&source);
        assert!(
            python.is_err() || !python.as_ref().expect("checked").contains("sum(ages)"),
            "expected `{line}` not to be read as a list reading"
        );
    }
}

#[test]
fn a_record_line_naming_a_list_is_refused_rather_than_appended() {
    // Left alone, the list statement would take the whole of `민수를 90` and
    // append it as one piece of text — a program nobody wrote, that looks
    // like it worked.
    assert_eq!(
        error_code("친구들은 빈 목록\n친구들에 민수를 90으로 넣어\n"),
        "E0234"
    );
    assert_eq!(
        error_code("set pals to an empty list\nput Mina at 90 in pals\n"),
        "E0234"
    );
}

#[test]
fn a_korean_amount_keeps_its_particle_off_the_name() {
    // `결과를 둘째수로 나눠` produced `결과 = 결과 / 둘째수로`: the particle
    // stayed glued to the name, and `둘째수로` is a perfectly valid Python
    // expression — a name nothing ever set. Which of the two the program
    // actually made is what tells them apart, so that is asked first.
    assert_eq!(
        ok("첫수는 12\n둘째수는 4\n결과는 첫수\n결과를 둘째수로 나눠\n"),
        "첫수 = 12\n둘째수 = 4\n결과 = 첫수\n결과 = 결과 / 둘째수\n"
    );
    // A written number keeps working, and so do the other three operations.
    assert_eq!(
        ok("결과는 12\n결과를 2로 나눠\n"),
        "결과 = 12\n결과 = 결과 / 2\n"
    );
    assert_eq!(
        ok("둘째수는 4\n결과는 12\n결과에 둘째수 곱해\n"),
        "둘째수 = 4\n결과 = 12\n결과 = 결과 * 둘째수\n"
    );
}

#[test]
fn a_list_line_naming_a_record_is_refused_rather_than_appended() {
    // The mirror of `a_record_line_naming_a_list_is_refused`. Without it,
    // `표에 사과 넣어` and `append apple to ages` compiled to
    // `표.append("사과")` — `AttributeError: 'dict' object has no attribute
    // 'append'` at run time, on a line that reads perfectly.
    assert_eq!(error_code("나이표는 빈 표\n나이표에 사과 넣어\n"), "E0234");
    assert_eq!(
        error_code("set ages to an empty record\nappend apple to ages\n"),
        "E0234"
    );
    assert_eq!(
        error_code("set ages to an empty record\nto ages append apple\n"),
        "E0234"
    );
    // A list still takes the same line without complaint.
    assert_eq!(
        ok("set friends to an empty list\nappend Mina to friends\n"),
        "friends = []\nfriends.append(\"Mina\")\n"
    );
    assert_eq!(
        ok("친구들은 빈 목록\n친구들에 민수 넣어\n"),
        "친구들 = []\n친구들.append(\"민수\")\n"
    );
}

#[test]
fn a_python_dictionary_is_a_record_to_the_sentence_statements() {
    assert_eq!(
        ok("ages = {}\nput Mina at 90 in ages\n"),
        "ages = {}\nages[\"Mina\"] = 90\n"
    );
    assert_eq!(
        ok("ages = {\"Mina\": 90}\nshow how many ages\n"),
        "ages = {\"Mina\": 90}\nprint(len(ages))\n"
    );
    // A set literal is not a record: `{1, 2}` has no names in it.
    assert!(!ok("ages = {1, 2}\n나이표는 빈 표\n").contains("del"));
}

#[test]
fn record_words_in_ordinary_sentences_stay_sentences() {
    for source in [
        "표는 두 장 남았습니다\n",
        "빈 표가 하나도 없었습니다\n",
        "성적표를 아직 받지 못했습니다\n",
        "그릇에 설탕을 한 스푼으로 넣어\n",
        "봉투에 편지를 등기로 넣어\n",
        "창고에 상자를 세로로 두어\n",
        "우리의 시간은 늘 모자랍니다\n",
        "가게에서 우유를 샀습니다\n",
        "I keep a record of everything I read.\n",
        "She broke the record by two seconds.\n",
        "The table was covered in papers.\n",
        "Put the kettle on.\n",
        "Put your trust in me.\n",
        "Put Mina at ease in the garden.\n",
    ] {
        let python = ok(source);
        assert!(
            python.starts_with("print("),
            "expected a printed sentence for {source:?}, got {python:?}"
        );
    }
}

#[test]
fn a_repaired_list_word_may_not_invent_the_container_too() {
    // Both of these came out of this round's prose sweep. `있어` and `두어`
    // are each one character from `넣어`, and the line became
    // `너.append("하고 싶은 말이")` — a program that dies with `NameError` on a
    // line that reads as a sentence. A guess at the verb may not also invent
    // the name it puts something in.
    for source in [
        "너에게 하고 싶은 말이 있어\n",
        "창고에 상자를 세로로 두어\n",
    ] {
        let python = ok(source);
        assert!(
            python.starts_with("print("),
            "expected a printed sentence for {source:?}, got {python:?}"
        );
    }
    // The exact spelling on a list the program made is untouched.
    assert_eq!(
        ok("친구들은 빈 목록\n친구들에 민수 넣어\n"),
        "친구들 = []\n친구들.append(\"민수\")\n"
    );
}

// --------------------------------------------------------------- named jobs

#[test]
fn a_job_is_a_def_in_both_languages() {
    assert_eq!(
        ok("to greet:\nshow Hello\nend\n"),
        "def greet():\n    print(\"Hello\")\n# end\n"
    );
    assert_eq!(
        ok("인사하기라는 일:\n안녕하세요 말해줘\n끝\n"),
        "def 인사하기():\n    print(\"안녕하세요\")\n# end\n"
    );
    assert_eq!(
        ok("계산이라는 작업:\n하나 말해줘\n끝\n"),
        "def 계산():\n    print(\"하나\")\n# end\n"
    );
    // Written the way Python writes it: an indented body, no `end`.
    assert_eq!(
        ok("to greet:\n    show Hello\n"),
        "def greet():\n    print(\"Hello\")\n"
    );
}

#[test]
fn a_job_is_run_by_name() {
    assert_eq!(
        ok("to greet:\nshow Hello\nend\ndo greet\n"),
        "def greet():\n    print(\"Hello\")\n# end\ngreet()\n"
    );
    assert_eq!(
        ok("to greet:\nshow Hello\nend\nrun greet\n"),
        "def greet():\n    print(\"Hello\")\n# end\ngreet()\n"
    );
    assert_eq!(
        ok("인사하기라는 일:\n안녕 말해줘\n끝\n인사하기 해줘\n"),
        "def 인사하기():\n    print(\"안녕\")\n# end\n인사하기()\n"
    );
    assert_eq!(
        ok("인사하기라는 일:\n안녕 말해줘\n끝\n인사하기 실행해\n"),
        "def 인사하기():\n    print(\"안녕\")\n# end\n인사하기()\n"
    );
    // A zero-argument Python function is a job the sentence tier can run.
    assert_eq!(
        ok("def greet():\n    print(1)\ndo greet\n"),
        "def greet():\n    print(1)\ngreet()\n"
    );
}

#[test]
fn a_job_body_may_be_indented_and_still_close_with_end() {
    // Every other block in the language takes an indented body with an `end`
    // under it, and every guide in this repository writes them that way. Jobs
    // refused exactly that shape until 2026-08-19 — with `E0101` pointing at a
    // header that was already correct — because a job opened its explicit
    // block only when the body was flat.
    assert_eq!(
        ok("to greet:\n    show Hello\nend\ndo greet\n"),
        "def greet():\n    print(\"Hello\")\n# end\ngreet()\n"
    );
    assert_eq!(
        ok("인사하기라는 일:\n    안녕하세요 말해줘\n끝\n인사하기 해줘\n"),
        "def 인사하기():\n    print(\"안녕하세요\")\n# end\n인사하기()\n"
    );
    // The same, for a job that takes something.
    assert_eq!(
        ok("to praise someone:\n    show someone\nend\ndo praise with Mina\n"),
        "def praise(someone):\n    print(someone)\n# end\npraise(\"Mina\")\n"
    );
    assert_eq!(
        ok("이름에게 축하하기라는 일:\n    이름 말해줘\n끝\n민수에게 축하하기 해줘\n"),
        "def 축하하기(이름):\n    print(이름)\n# end\n축하하기(\"민수\")\n"
    );
    // A flat body still needs its `end`, and an indented body still may skip
    // it: the two ways a block is closed anywhere else in the language.
    assert_eq!(
        ok("to greet:\nshow Hello\nend\n"),
        "def greet():\n    print(\"Hello\")\n# end\n"
    );
    assert_eq!(
        ok("to greet:\n    show Hello\n"),
        "def greet():\n    print(\"Hello\")\n"
    );
}

#[test]
fn a_job_body_is_a_real_function_scope() {
    // A name set inside the job stays inside it, so the line after it prints
    // the word rather than reading a value that does not exist out there.
    assert_eq!(
        ok("to greet:\n    set inner to 5\nshow inner\n"),
        "def greet():\n    inner = 5\nprint(\"inner\")\n"
    );
    // And `return` inside a job is Python that belongs there.
    assert_eq!(
        ok("to twice:\n    return 2\n"),
        "def twice():\n    return 2\n"
    );
}

#[test]
fn a_job_header_without_a_block_is_a_sentence() {
    // The colon alone proves nothing: `To do:` and `To summarise:` are
    // headings people write, and a heading with no block under it prints.
    for source in [
        "to summarise:\n",
        "to summarise:\nit was a good day\n",
        "인사하기라는 일:\n",
    ] {
        let python = ok(source);
        assert!(
            !python.contains("def "),
            "expected no function for {source:?}, got {python:?}"
        );
    }
    // There is no one-line form either, so a colon in the middle of a
    // sentence can never open one.
    assert!(!ok("to greet: it was fine\n").contains("def "));
}

#[test]
fn job_words_in_ordinary_sentences_stay_sentences() {
    for source in [
        "할 일이 많습니다\n",
        "오늘 할 일을 적어 두었습니다\n",
        "그 일은 제가 맡겠습니다\n",
        "「어서 오세요」라는 간판이 걸려 있었습니다\n",
        "민수라는 사람을 만났습니다\n",
        "작업이 아직 끝나지 않았습니다\n",
        "to be honest I was tired\n",
        "Nice to meet you.\n",
        "To my surprise it worked.\n",
    ] {
        let python = ok(source);
        assert!(
            python.starts_with("print("),
            "expected a printed sentence for {source:?}, got {python:?}"
        );
    }
    // A job name is the only gate on the line that runs one: with no job
    // called `everything`, this is not a call.
    assert!(!ok("나는 오늘 청소 해\n").contains("청소()"));
}

#[test]
fn a_job_holds_a_loop_and_closes_with_one_end_each() {
    assert_eq!(
        ok("인사하기라는 일:\n3번 반복해\n안녕 말해줘\n끝\n끝\n인사하기 해줘\n"),
        "def 인사하기():\n    for _ in range(3):\n        print(\"안녕\")\n    # end\n# end\n인사하기()\n"
    );
}

#[test]
fn a_job_can_be_given_one_thing() {
    assert_eq!(
        ok("to greet someone:\nshow Hello someone!\nend\ndo greet with Mina\n"),
        "def greet(someone):\n    print(\"Hello \" + str(someone) + \"!\")\n# end\ngreet(\"Mina\")\n"
    );
    assert_eq!(
        ok("이름에게 인사하기라는 일:\n안녕 이름! 말해줘\n끝\n민수에게 인사하기 해줘\n"),
        "def 인사하기(이름):\n    print(\"안녕 \" + str(이름) + \"!\")\n# end\n인사하기(\"민수\")\n"
    );
    // A number, and a name the program already saved, are both values here.
    assert_eq!(
        ok("수를 두배라는 일:\n수 말해줘\n끝\n3에게 두배 해줘\n"),
        "def 두배(수):\n    print(수)\n# end\n두배(3)\n"
    );
    assert_eq!(
        ok("수를 두배라는 일:\n수 말해줘\n끝\n누구는 지안\n누구에게 두배 해줘\n")
            .lines()
            .last()
            .expect("a last line"),
        "두배(누구)"
    );
}

#[test]
fn the_number_of_things_a_job_takes_is_remembered() {
    // Handing one thing to a job that takes none, or none to a job that takes
    // one, would be a `TypeError` at run time on a line that looks right, so
    // it is named at compile time instead.
    assert_eq!(
        error_code("to greet:\nshow hi\nend\ndo greet with Mina\n"),
        "E0235"
    );
    assert_eq!(
        error_code("이름에게 인사하기라는 일:\nhi 말해줘\n끝\n인사하기 해줘\n"),
        "E0235"
    );
}

#[test]
fn a_job_left_open_is_named_as_a_job() {
    // The `end` here closes the loop, so the job is still open at the bottom
    // of the file. The message has to say a *job* was left open, not guess at
    // a loop.
    let problems =
        transpile("to greet:\nrepeat 2 times\nshow hi\nend\n").expect_err("the job is left open");
    assert_eq!(problems[0].code.code(), "E0105");
    assert!(
        problems[0].message.contains("job"),
        "expected the message to name a job, got {:?}",
        problems[0].message
    );
}

/// A job may count something that was made outside it.
///
/// Python decides for a whole function at once whether a name is local, so
/// `to tally: add 1 to total end` built a function that read `total` before
/// it had one and died with `UnboundLocalError` — a word the reader has never
/// met, on a line that looks right. The declaration rides on the same
/// physical line, because one NME statement is one line of Python.
#[test]
fn a_job_can_change_a_name_made_outside_it() {
    assert_eq!(
        ok("set total to 0\nto tally:\n  add 1 to total\nend\ndo tally\n"),
        "total = 0\ndef tally():\n  global total; total = total + 1\n# end\ntally()\n"
    );
    assert_eq!(
        ok("총합은 0\n세기라는 일:\n  총합에 1 더해\n끝\n세기 해줘\n"),
        "총합 = 0\ndef 세기():\n  global 총합; 총합 = 총합 + 1\n# end\n세기()\n"
    );
    // A name the job makes itself is the job's own, and says nothing.
    assert_eq!(
        ok("to greet:\n  set hello to hi\n  show hello\nend\n"),
        "def greet():\n  hello = \"hi\"\n  print(hello)\n# end\n"
    );
}

/// Reading first and changing afterwards cannot be written at all, and saying
/// so beats a `SyntaxError` from CPython about a declaration nobody wrote.
#[test]
fn a_job_that_reads_before_it_changes_is_named() {
    let problems =
        nme_core::transpile("set total to 0\nto tally:\n  show total\n  add 1 to total\nend\n")
            .expect_err("expected this to be refused");
    assert_eq!(problems[0].code.code(), "E0236");
    assert!(problems[0].message.contains("total"), "{problems:?}");
}
