//! The bundled date module: `use date` / `날짜 사용`.
//!
//! Three things are worth a test of their own here.
//!
//! * `date` and `날짜` are ordinary words. A sentence that merely contains one
//!   must still be the sentence it is, before and after the module is opened.
//! * The module binds `today`, `now`, `year`, `month`, `오늘`, `지금`, `올해`
//!   and friends — all of them ordinary words too, and all of them names the
//!   compiler can see on later lines once they are bound.
//! * `weekday` and `요일` are the one pair in any bundled module bound to
//!   different values, so the difference is pinned rather than left to be
//!   tidied away by someone who reads it as a mistake.

use nme_core::transpile;

/// Transpiles and expects success.
fn ok(source: &str) -> String {
    transpile(source)
        .unwrap_or_else(|problems| panic!("expected successful transpile, got: {problems:?}"))
}

/// The one line `use date` lowers to, in either language.
const DATE_TOOLS: &str = concat!(
    "import datetime as 날짜모듈; ",
    "today = 오늘 = lambda: 날짜모듈.date.today().isoformat(); ",
    "now = 지금 = lambda: 날짜모듈.datetime.now().strftime(\"%H:%M\"); ",
    "year = 올해 = lambda: 날짜모듈.date.today().year; ",
    "month = 이번달 = lambda: 날짜모듈.date.today().month; ",
    "day_of_month = 오늘일자 = lambda: 날짜모듈.date.today().day; ",
    "weekday = lambda: 날짜모듈.date.today().strftime(\"%A\"); ",
    "요일 = lambda: [\"월요일\", \"화요일\", \"수요일\", \"목요일\", \"금요일\", \"토요일\", \"일요일\"]",
    "[날짜모듈.date.today().weekday()]; ",
    "days_after = 며칠뒤 = lambda 일수: (날짜모듈.date.today() + 날짜모듈.timedelta(days=일수)).isoformat(); ",
    "date_version = 날짜버전 = \"0.0.1\"\n",
);

#[test]
fn date_tools_are_ready_after_one_easy_line() {
    assert_eq!(ok("use date\n"), DATE_TOOLS);
    assert_eq!(ok("날짜 사용\n"), DATE_TOOLS);
    assert_eq!(ok("use date latest\n"), DATE_TOOLS);
    assert_eq!(ok("use latest date\n"), DATE_TOOLS);
    assert_eq!(ok("날짜 사용 최신\n"), DATE_TOOLS);
    assert_eq!(ok("최신 날짜 사용\n"), DATE_TOOLS);
    assert_eq!(ok("use date version \"0.0.1\"\n"), DATE_TOOLS);
    assert_eq!(ok("날짜 사용 버전 \"0.0.1\"\n"), DATE_TOOLS);
}

#[test]
fn one_module_line_lowers_to_exactly_one_python_line() {
    // Every NME statement lowers to one physical line, which is what keeps a
    // Python traceback pointing at the line the writer can see.
    assert_eq!(ok("use date\n").lines().count(), 1);
    assert_eq!(ok("날짜 사용\nsay today()\n").lines().count(), 2);
}

#[test]
fn every_date_reading_lowers_to_what_the_table_says() {
    // English and Korean, at the beginner level and at the sentence level.
    for (source, expected) in [
        ("use date\nsay today()\n", "print(today())"),
        ("use date\nshow now()\n", "print(now())"),
        ("use date\nsay year()\n", "print(year())"),
        ("use date\nsay month()\n", "print(month())"),
        ("use date\nsay day_of_month()\n", "print(day_of_month())"),
        ("use date\nsay weekday()\n", "print(weekday())"),
        ("use date\nsay days_after(7)\n", "print(days_after(7))"),
        ("use date\nsay days_after(-1)\n", "print(days_after(-1))"),
        ("use date\nsay date_version\n", "print(date_version)"),
        ("날짜 사용\n말해 오늘()\n", "print(오늘())"),
        ("날짜 사용\n말해줘 지금()\n", "print(지금())"),
        ("날짜 사용\n말해 올해()\n", "print(올해())"),
        ("날짜 사용\n말해 이번달()\n", "print(이번달())"),
        ("날짜 사용\n말해 오늘일자()\n", "print(오늘일자())"),
        ("날짜 사용\n말해 요일()\n", "print(요일())"),
        ("날짜 사용\n말해 며칠뒤(7)\n", "print(며칠뒤(7))"),
        ("날짜 사용\n말해 날짜버전\n", "print(날짜버전)"),
    ] {
        let python = ok(source);
        let last = python.lines().last().unwrap_or_default();
        assert_eq!(last, expected, "for {source:?}");
    }
}

#[test]
fn date_weekday_names_differ_by_language() {
    // The one place in any bundled module where a Korean helper and its
    // English twin are bound to different values. A weekday name is a word,
    // and a word has to be in some language: `Wednesday` is the wrong answer
    // to `요일()` and `수요일` is the wrong answer to `weekday()`. The name
    // the writer uses chooses the language of the answer.
    //
    // If this test ever fails because the two were made equal, the fix is not
    // to delete the test: it is to decide, in writing, which language a
    // program in the other language should be answered in.
    let tools = ok("use date\n");
    assert!(
        tools.contains("weekday = lambda: 날짜모듈.date.today().strftime(\"%A\")"),
        "{tools}"
    );
    assert!(
        tools.contains("요일 = lambda: [\"월요일\", \"화요일\", \"수요일\""),
        "{tools}"
    );
    // Neither is an alias of the other, and neither is bound twice.
    assert_eq!(tools.matches("weekday = ").count(), 1, "{tools}");
    assert_eq!(tools.matches("요일 = ").count(), 1, "{tools}");
}

#[test]
fn the_date_module_does_not_take_the_names_date_and_날짜() {
    // `date` is what people call their own variable. Binding it would turn
    // `use date` into a refusal in exactly the programs that want it most.
    let tools = ok("use date\n");
    assert!(!tools.contains("date = "), "{tools}");
    assert!(!tools.contains("날짜 = "), "{tools}");

    // So a program that already keeps its own date may still open the module.
    let python = ok("set date to 5\nuse date\nsay today()\n");
    assert!(python.starts_with("date = 5\n"), "{python}");
    assert!(python.contains("print(today())"), "{python}");
    let korean = ok("날짜는 5\n날짜 사용\n말해 오늘()\n");
    assert!(korean.starts_with("날짜 = 5\n"), "{korean}");
}

#[test]
fn the_date_module_refuses_to_overwrite_an_existing_name() {
    for (source, name) in [
        ("today = 3\nuse date\n", "today"),
        ("weekday = 3\nuse date\n", "weekday"),
        ("올해 = 3\n날짜 사용\n", "올해"),
        ("요일 = 3\n날짜 사용\n", "요일"),
    ] {
        let problems = transpile(source).expect_err("expected a refusal");
        let rendered = format!("{problems:?}");
        assert!(rendered.contains(name), "for {source:?}: {rendered}");
    }
}

#[test]
fn a_sentence_containing_a_date_word_is_still_a_sentence() {
    // Before the module is opened.
    for source in [
        "I forgot to write the date on the form.\n",
        "It is very cold today.\n",
        "I will do it now.\n",
        "It has been a difficult year.\n",
        "The rent is due at the end of the month.\n",
        "The museum is free on a weekday.\n",
        "No date has been set for the repairs.\n",
        "날짜를 잘못 적었습니다\n",
        "오늘 날씨가 좋네요\n",
        "지금 가고 있습니다\n",
        "올해도 잘 부탁드립니다\n",
        "이번 달은 정말 빨리 갔습니다\n",
        "요일을 자꾸 헷갈립니다\n",
        "며칠 뒤에 다시 오세요\n",
    ] {
        let python = ok(source);
        let expected = format!("print(\"{}\")\n", source.trim_end());
        assert_eq!(python, expected, "before the module, for {source:?}");

        // And after it, with every one of those words now a bound name.
        let opened = if source.is_ascii() {
            format!("use date\n{source}")
        } else {
            format!("날짜 사용\n{source}")
        };
        let python = ok(&opened);
        assert_eq!(
            python.lines().last().unwrap_or_default(),
            expected.trim_end(),
            "after the module, for {source:?}"
        );
    }
}

#[test]
fn opening_the_module_does_not_change_a_line_it_does_not_own() {
    // The sharp form of the rule above, for the lines NME reads as something
    // other than a printed sentence. Whatever reading a line gets, it gets the
    // same one before and after `use date` — a question ending in `?` is still
    // a question, and `What is the date today?` still asks, exactly as
    // `지금 몇 시예요?` does. `scripts/mistake-probes/date_words.py` holds the
    // same check over 310 sentences.
    for (alone, opened) in [
        (
            "What is the date today?\n",
            "use date\nWhat is the date today?\n",
        ),
        (
            "지금 통화 괜찮으신가요\n",
            "날짜 사용\n지금 통화 괜찮으신가요\n",
        ),
        (
            "며칠 동안 잠을 못 잤습니다\n",
            "날짜 사용\n며칠 동안 잠을 못 잤습니다\n",
        ),
    ] {
        let before = ok(alone);
        let after = ok(opened);
        assert_eq!(
            after.lines().skip(1).collect::<Vec<_>>(),
            before.lines().collect::<Vec<_>>(),
            "for {alone:?}"
        );
    }
}

#[test]
fn use_with_a_date_word_further_along_the_line_is_a_sentence() {
    // `date` only names the module when it stands beside the `use` word.
    for source in [
        "Use the date on the label, not the one on the box.\n",
        "I use a paper diary for dates and a phone for everything else.\n",
        "이 날짜 사용법을 알려 주세요\n",
        "그 날짜를 사용했습니다\n",
    ] {
        let python = ok(source);
        assert!(
            python.starts_with("print("),
            "expected a printed sentence for {source:?}, got {python:?}"
        );
    }
}

/// A name may not take a module's word away, whichever line comes first.
///
/// `set today to Monday` before `use date` has always been refused. The other
/// order was accepted and died at run time with `'str' object is not
/// callable`, on the later line that called `today()`.
#[test]
fn a_name_cannot_take_a_loaded_module_word() {
    let problems = transpile("use date\nset today to Monday\nshow today()\n")
        .expect_err("expected this to be refused");
    assert_eq!(problems[0].code.code(), "E0405");
    assert!(problems[0].message.contains("today"), "{problems:?}");
    let korean = transpile("날짜 사용\n오늘은 월요일\n").expect_err("expected this to be refused");
    assert_eq!(korean[0].code.code(), "E0405");
    // A name of its own is untouched, and so is a question that happens to
    // end on one of the module's words — see the test above.
    assert!(transpile("use date\nset weekday_name to Monday\n").is_ok());
    assert!(transpile("use date\nWhat is the date today?\n").is_ok());
}

/// `show Today is today()` is a sentence, not Python's identity test.
///
/// It is valid Python, so it came out as one and printed the words back
/// unevaluated after dying on `Today` — a name nothing ever made.
#[test]
fn a_sentence_with_is_in_it_is_not_an_identity_test() {
    assert_eq!(
        ok("use date\nshow Today is today()\n")
            .lines()
            .last()
            .unwrap(),
        "print(\"Today is today()\")"
    );
    assert_eq!(
        ok("use date\nshow It is weekday()\n")
            .lines()
            .last()
            .unwrap(),
        "print(\"It is weekday()\")"
    );
    // Names the program made are still put into the sentence, which is what
    // the sentence level has always done with `show <words with names in>`.
    assert_eq!(
        ok("a = 1\nb = 1\nshow a is b\n").lines().last().unwrap(),
        "print(str(a) + \" is \" + str(b))"
    );
}
