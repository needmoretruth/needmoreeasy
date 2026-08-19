//! Every beginner mistake whose answer changed in the "never silently
//! mis-compile" round, one test case per probe in the 566-probe corpus.
//!
//! Each refusal asserts three things: the stable error code, the physical line
//! the caret lands on, and that the message names the word the writer has to
//! change. Both languages are checked, because a Korean beginner must get the
//! same information as an English one.

use nme_core::diagnostics::DiagnosticCode;
use nme_core::transpile;

/// One refused line: the probe it came from, the source, the code it must
/// carry, the 1-based line the caret must land on, and a word the message has
/// to name.
struct Refusal {
    probe: &'static str,
    source: &'static str,
    code: DiagnosticCode,
    line: usize,
    names: &'static str,
}

fn check(case: &Refusal) {
    let problems = match transpile(case.source) {
        Ok(python) => panic!("{}: expected a refusal, got {python:?}", case.probe),
        Err(problems) => problems,
    };
    let problem = problems
        .iter()
        .find(|problem| problem.code == case.code)
        .unwrap_or_else(|| {
            panic!(
                "{}: expected {:?}, got {:?}",
                case.probe,
                case.code,
                problems.iter().map(|p| p.code).collect::<Vec<_>>()
            )
        });
    assert_eq!(
        problem.line_col(case.source).0,
        case.line,
        "{}: the caret is on the wrong line",
        case.probe
    );
    let english = format!(
        "{} {}",
        problem.message,
        problem.hint.clone().unwrap_or_default()
    );
    let korean = format!(
        "{} {}",
        problem
            .message_ko
            .clone()
            .unwrap_or_else(|| panic!("{}: no Korean message", case.probe)),
        problem
            .hint_ko
            .clone()
            .unwrap_or_else(|| panic!("{}: no Korean hint", case.probe))
    );
    // Both languages carry the same information. The example spellings differ
    // on purpose, so only the writer's own word is compared across the two.
    assert_ne!(
        english, korean,
        "{}: the Korean text is the English text",
        case.probe
    );
    if case.names.is_empty() {
        return;
    }
    assert!(
        english.contains(case.names),
        "{}: the English message does not name `{}`: {english}",
        case.probe,
        case.names
    );
    assert!(
        korean.contains(case.names),
        "{}: the Korean message does not name `{}`: {korean}",
        case.probe,
        case.names
    );
}

fn accepts(probe: &str, source: &str, expected: &str) {
    let python = transpile(source)
        .unwrap_or_else(|problems| panic!("{probe}: expected this to compile, got {problems:?}"));
    assert!(
        python.contains(expected),
        "{probe}: expected `{expected}`, got {python:?}"
    );
}

#[test]
fn raw_cpython_errors_become_nme_diagnostics() {
    for case in [
        Refusal {
            probe: "f-en-01",
            source: "  say hello\n",
            code: DiagnosticCode::UnexpectedIndent,
            line: 1,
            names: "",
        },
        Refusal {
            probe: "f-en-02",
            source: "	say hello\n",
            code: DiagnosticCode::UnexpectedIndent,
            line: 1,
            names: "",
        },
        Refusal {
            probe: "f-ko-01",
            source: "  안녕 말해줘\n",
            code: DiagnosticCode::UnexpectedIndent,
            line: 1,
            names: "",
        },
        Refusal {
            probe: "f-ko-02",
            source: "	안녕 말해줘\n",
            code: DiagnosticCode::UnexpectedIndent,
            line: 1,
            names: "",
        },
        Refusal {
            probe: "h-en-03",
            source: "set friends to list of Mina\nput Mina in friends\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 2,
            names: "put",
        },
        Refusal {
            probe: "k-ko-19",
            source: "2초 대기해\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "대기해",
        },
        Refusal {
            probe: "k-ko-20",
            source: "2초 대기\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "대기",
        },
        Refusal {
            probe: "k-ko-21",
            source: "2초 기다립니다\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "기다립니다",
        },
        Refusal {
            probe: "o-en-10",
            source: "set score to 0\n1 add to score\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 2,
            names: "add",
        },
        Refusal {
            probe: "o-ko-08",
            source: "0 점수에 저장해\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "저장해",
        },
        Refusal {
            probe: "s-en-03",
            source: "  say hello\n",
            code: DiagnosticCode::UnexpectedIndent,
            line: 1,
            names: "",
        },
        Refusal {
            probe: "s-en-12",
            source: "setscore to 0\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "setscore",
        },
        Refusal {
            probe: "y-en-18",
            source: "assign 0 to score\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "assign",
        },
        Refusal {
            probe: "y-en-20",
            source: "delay 2 seconds\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "delay",
        },
        Refusal {
            probe: "y-en-21",
            source: "hold 2 seconds\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "hold",
        },
        Refusal {
            probe: "y-en-22",
            source: "rest 2 seconds\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "rest",
        },
        Refusal {
            probe: "y-en-29",
            source: "set score to 0\nbump score by 1\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 2,
            names: "bump",
        },
        Refusal {
            probe: "y-ko-16",
            source: "2초 멈춰줘\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "멈춰줘",
        },
        Refusal {
            probe: "y-ko-17",
            source: "2초 잠시멈춰\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "잠시멈춰",
        },
        Refusal {
            probe: "y-ko-18",
            source: "2초 슬립\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "슬립",
        },
        Refusal {
            probe: "y-ko-28",
            source: "점수는 0\n점수를 1 증가해\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 2,
            names: "증가해",
        },
        Refusal {
            probe: "y-ko-29",
            source: "점수는 0\n점수를 1 감소해\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 2,
            names: "감소해",
        },
    ] {
        check(&case);
    }
}

#[test]
fn a_line_that_starts_with_a_space_is_named() {
    for case in [
        Refusal {
            probe: "f-en-01",
            source: "  say hello\n",
            code: DiagnosticCode::UnexpectedIndent,
            line: 1,
            names: "",
        },
        Refusal {
            probe: "f-en-02",
            source: "	say hello\n",
            code: DiagnosticCode::UnexpectedIndent,
            line: 1,
            names: "",
        },
        Refusal {
            probe: "f-ko-01",
            source: "  안녕 말해줘\n",
            code: DiagnosticCode::UnexpectedIndent,
            line: 1,
            names: "",
        },
        Refusal {
            probe: "f-ko-02",
            source: "	안녕 말해줘\n",
            code: DiagnosticCode::UnexpectedIndent,
            line: 1,
            names: "",
        },
        Refusal {
            probe: "s-en-03",
            source: "  say hello\n",
            code: DiagnosticCode::UnexpectedIndent,
            line: 1,
            names: "",
        },
    ] {
        check(&case);
    }
}

#[test]
fn a_word_that_is_not_an_action_is_named() {
    for case in [
        Refusal {
            probe: "h-en-03",
            source: "set friends to list of Mina\nput Mina in friends\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 2,
            names: "put",
        },
        // `j-ko-04` (`3번 되풀이해서 안녕 말해줘`) used to be refused because
        // `되풀이해서` was not a repeat word. It is one now, and both
        // `probes2.py` and `probes.py` have always recorded the line as
        // meaning `range(3): print("안녕")`, so the two cases for it moved to
        // `many_ways.rs` as an accepted spelling.
        Refusal {
            probe: "k-ko-13",
            source: "말합니다 안녕\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "말합니다",
        },
        Refusal {
            probe: "k-ko-14",
            source: "출력하기 안녕\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "출력하기",
        },
        Refusal {
            probe: "k-ko-19",
            source: "2초 대기해\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "대기해",
        },
        Refusal {
            probe: "k-ko-20",
            source: "2초 대기\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "대기",
        },
        Refusal {
            probe: "k-ko-21",
            source: "2초 기다립니다\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "기다립니다",
        },
        Refusal {
            probe: "k-ko-27",
            source: "이름을 입력해 이름이 뭐예요?\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "입력해",
        },
        Refusal {
            probe: "k-ko-33",
            source: "3번 반복합니다\n  안녕 말해줘\n끝\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "반복합니다",
        },
        Refusal {
            probe: "m-ko-02",
            source: "친구들은 목록 민수\n친구들에 민수 너허\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 2,
            names: "너허",
        },
        Refusal {
            probe: "o-en-10",
            source: "set score to 0\n1 add to score\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 2,
            names: "add",
        },
        Refusal {
            probe: "o-ko-08",
            source: "0 점수에 저장해\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "저장해",
        },
        Refusal {
            probe: "s-en-06",
            source: "wait3 seconds\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "wait3",
        },
        Refusal {
            probe: "s-en-12",
            source: "setscore to 0\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "setscore",
        },
        Refusal {
            probe: "s-ko-09",
            source: "3번반복해서 안녕 말해줘\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "번반복해서",
        },
        Refusal {
            probe: "t-ko-26",
            source: "이름을 무러봐 이름이 뭐예요?\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "무러봐",
        },
        Refusal {
            probe: "t2-en-01",
            source: "wait2 seconds\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "wait2",
        },
        Refusal {
            probe: "t2-ko-02",
            source: "3번반복해서 안녕 말해줘\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "번반복해서",
        },
        Refusal {
            probe: "y-en-18",
            source: "assign 0 to score\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "assign",
        },
        Refusal {
            probe: "y-en-20",
            source: "delay 2 seconds\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "delay",
        },
        Refusal {
            probe: "y-en-21",
            source: "hold 2 seconds\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "hold",
        },
        Refusal {
            probe: "y-en-22",
            source: "rest 2 seconds\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "rest",
        },
        Refusal {
            probe: "y-en-27",
            source: "set friends to list of Mina\ninsert Mina into friends\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 2,
            names: "insert",
        },
        Refusal {
            probe: "y-en-29",
            source: "set score to 0\nbump score by 1\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 2,
            names: "bump",
        },
        Refusal {
            probe: "y-ko-08",
            source: "출력하기 안녕\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "출력하기",
        },
        Refusal {
            probe: "y-ko-09",
            source: "프린트해 안녕\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "프린트해",
        },
        Refusal {
            probe: "y-ko-12",
            source: "보여주기 안녕\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "보여주기",
        },
        // `y-ko-13` is the same line as `j-ko-04` above, and left for the
        // same reason: `probes.py` records it as a working loop.
        Refusal {
            probe: "y-ko-14",
            source: "3번 돌려서 안녕 말해줘\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "말해줘",
        },
        Refusal {
            probe: "y-ko-15",
            source: "3번 루프해서 안녕 말해줘\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "말해줘",
        },
        Refusal {
            probe: "y-ko-16",
            source: "2초 멈춰줘\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "멈춰줘",
        },
        Refusal {
            probe: "y-ko-17",
            source: "2초 잠시멈춰\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "잠시멈춰",
        },
        Refusal {
            probe: "y-ko-18",
            source: "2초 슬립\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 1,
            names: "슬립",
        },
        // `y-ko-22` (`이름을 여쭤봐 이름이 뭐예요?`) is accepted now: `여쭤봐` is
        // the polite word for asking, the line ends in a question mark, and
        // `probes.py` has always recorded it as `이름 = input(...)`. It moved
        // to `many_ways.rs`.
        Refusal {
            probe: "y-ko-25",
            source: "친구들은 목록 민수\n친구들에 민수 집어넣어\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 2,
            names: "집어넣어",
        },
        Refusal {
            probe: "y-ko-28",
            source: "점수는 0\n점수를 1 증가해\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 2,
            names: "증가해",
        },
        Refusal {
            probe: "y-ko-29",
            source: "점수는 0\n점수를 1 감소해\n",
            code: DiagnosticCode::UnknownActionWord,
            line: 2,
            names: "감소해",
        },
    ] {
        check(&case);
    }
}

#[test]
fn a_line_that_cannot_do_anything_is_named() {
    for case in [
        Refusal {
            probe: "b-en-05",
            source: "score is 0\n",
            code: DiagnosticCode::StatementDoesNothing,
            line: 1,
            names: "",
        },
        Refusal {
            probe: "p-en-08",
            source: "say: hello\n",
            code: DiagnosticCode::StatementDoesNothing,
            line: 1,
            names: ":",
        },
        Refusal {
            probe: "p-en-09",
            source: "show: hello\n",
            code: DiagnosticCode::StatementDoesNothing,
            line: 1,
            names: ":",
        },
        Refusal {
            probe: "p-ko-11",
            source: "말해줘: 안녕\n",
            code: DiagnosticCode::StatementDoesNothing,
            line: 1,
            names: ":",
        },
        Refusal {
            probe: "s-en-04",
            source: "sayhello\n",
            code: DiagnosticCode::StatementDoesNothing,
            line: 1,
            names: "sayhello",
        },
        Refusal {
            probe: "s-en-05",
            source: "showhello\n",
            code: DiagnosticCode::StatementDoesNothing,
            line: 1,
            names: "showhello",
        },
        Refusal {
            probe: "s-ko-07",
            source: "안녕말해줘\n",
            code: DiagnosticCode::StatementDoesNothing,
            line: 1,
            names: "안녕말해줘",
        },
        Refusal {
            probe: "s-ko-15",
            source: "점수는 0\n점수에1더해\n",
            code: DiagnosticCode::StatementDoesNothing,
            line: 2,
            names: "점수에1더해",
        },
        Refusal {
            probe: "s-ko-20",
            source: "점수는0\n",
            code: DiagnosticCode::StatementDoesNothing,
            line: 1,
            names: "점수는0",
        },
        Refusal {
            probe: "t-en-47",
            source: "repeat 3 times\n  show hello\nedn\n",
            code: DiagnosticCode::StatementDoesNothing,
            line: 3,
            names: "edn",
        },
        Refusal {
            probe: "t-ko-34",
            source: "3번 반복해\n  안녕 말해줘\n끋\n",
            code: DiagnosticCode::StatementDoesNothing,
            line: 3,
            names: "끋",
        },
        Refusal {
            probe: "t-ko-35",
            source: "3번 반복해\n  안녕 말해줘\n끗\n",
            code: DiagnosticCode::StatementDoesNothing,
            line: 3,
            names: "끗",
        },
        Refusal {
            probe: "t2-ko-05",
            source: "점수는0\n",
            code: DiagnosticCode::StatementDoesNothing,
            line: 1,
            names: "점수는0",
        },
    ] {
        check(&case);
    }
}

#[test]
fn a_curly_quote_says_what_to_write_instead() {
    for case in [
        Refusal {
            probe: "p-en-10",
            source: "say “hello”\n",
            code: DiagnosticCode::CurlyQuote,
            line: 1,
            names: "“",
        },
        Refusal {
            probe: "p-en-11",
            source: "say ‘hello’\n",
            code: DiagnosticCode::CurlyQuote,
            line: 1,
            names: "‘",
        },
        Refusal {
            probe: "p-ko-12",
            source: "“안녕” 말해\n",
            code: DiagnosticCode::CurlyQuote,
            line: 1,
            names: "“",
        },
    ] {
        check(&case);
    }
}

#[test]
fn a_value_that_would_become_text_is_refused() {
    for case in [
        Refusal {
            probe: "b-en-03",
            source: "set score = 0\n",
            code: DiagnosticCode::SaveValueUnparseable,
            line: 1,
            names: "",
        },
        Refusal {
            probe: "t-en-31",
            source: "set score t 0\n",
            code: DiagnosticCode::SaveValueUnparseable,
            line: 1,
            names: "",
        },
        Refusal {
            probe: "t-en-32",
            source: "set score too 0\n",
            code: DiagnosticCode::SaveValueUnparseable,
            line: 1,
            names: "",
        },
    ] {
        check(&case);
    }
}

#[test]
fn punctuation_a_keyboard_produces_is_read_as_written() {
    accepts(
        "j-en-03",
        "repeat 3 times, show hello\n",
        "for _ in range(3): print(\"hello\")",
    );
    accepts(
        "l-en-06",
        "ask age as a number How old are you?\n",
        "age = int(input(\"How old are you?\" + \" \"))",
    );
    accepts("p-ko-04", "2초 기다려？\n", "__import__(\"time\").sleep(2)");
    accepts("p-ko-05", "안녕 말해줘！\n", "print(\"안녕\")");
    accepts("p-ko-06", "안녕 말해줘、\n", "print(\"안녕\")");
    accepts(
        "p-ko-14",
        "친구들은 목록 민수、지안\n",
        "친구들 = [\"민수\", \"지안\"]",
    );
}
/// A line of ordinary writing prints itself, unchanged.
fn prints(source: &str) {
    let python = transpile(&format!("{source}\n"))
        .unwrap_or_else(|problems| panic!("{source:?} must print, got {problems:?}"));
    assert!(
        python.starts_with("print("),
        "{source:?} must print, got {python:?}"
    );
}

#[test]
fn a_korean_block_word_after_the_output_word_is_part_of_the_message() {
    // `동안` means "during" and is an ordinary word in the middle of ordinary
    // Korean. Once an output word has been written, everything else on the
    // line is the message, and no block opens.
    for source in [
        "안녕 말해줘 동안",
        "안녕 말해줘 만약",
        "안녕 말해줘 반복",
        "안녕 말해줘 아니면",
    ] {
        prints(source);
    }
    // The rule is positional, not lexical: with the output word last, the
    // block word in front of it is read as the message it always was.
    assert_eq!(
        transpile("커서가 깜빡이는 동안 말해줘\n").unwrap(),
        "print(\"커서가 깜빡이는 동안\")\n"
    );
    // And the verb-first slow spelling keeps its whole message.
    assert!(transpile("천천히 말해줘 커서가 깜빡이는 동안\n")
        .unwrap()
        .contains("커서가 깜빡이는 동안"));
}

#[test]
fn an_english_block_word_inside_a_message_already_prints_and_must_keep_doing_so() {
    assert_eq!(
        transpile("show wait here while I think\n").unwrap(),
        "print(\"wait here while I think\")\n"
    );
    assert_eq!(
        transpile("say hello if you can\n").unwrap(),
        "print(\"hello if you can\")\n"
    );
    assert_eq!(
        transpile("show repeat after me\n").unwrap(),
        "print(\"repeat after me\")\n"
    );
    assert_eq!(transpile("show the end\n").unwrap(), "print(\"the end\")\n");
}

#[test]
fn a_korean_sentence_ending_is_a_sentence_not_an_assignment() {
    // `-은`/`-는` on an adjective is one of the commonest shapes in Korean.
    // Every one of these used to become a variable and print nothing.
    for source in [
        "좋은 아침입니다",
        "작은 방이었습니다",
        "밝은 빛이 보였습니다",
        "붉은 문이 열렸습니다",
        "나는 학생입니다",
        "이것은 시작입니다",
        "끝입니다",
    ] {
        prints(source);
    }
    // The documented assignment still assigns: a bare word, a number, and a
    // number spoken as a sentence.
    assert_eq!(transpile("이름은 민수\n").unwrap(), "이름 = \"민수\"\n");
    assert_eq!(transpile("점수는 0\n").unwrap(), "점수 = 0\n");
    assert_eq!(transpile("점수는 0이다\n").unwrap(), "점수 = 0\n");
    assert_eq!(
        transpile("인사는 안녕하세요\n").unwrap(),
        "인사 = \"안녕하세요\"\n"
    );
}

#[test]
fn thirty_ordinary_sentences_print_themselves() {
    for source in [
        "Hello everyone!",
        "It was a dark and stormy night.",
        "The door was locked.",
        "Nobody answered the phone.",
        "She looked at the map again.",
        "Good morning",
        "Welcome to the game",
        "You have three lives left",
        "Try again tomorrow",
        "The end",
        "log in first",
        "let me think",
        "store it away",
        "echo of the mountain",
        "insert coin to continue",
        "please log in first",
        "안녕하세요 여러분!",
        "어두운 밤이었습니다.",
        "문이 잠겨 있었습니다.",
        "아무도 대답하지 않았습니다.",
        "지도를 다시 보았습니다.",
        "좋은 아침입니다",
        "게임에 오신 것을 환영합니다",
        "목숨이 세 개 남았습니다",
        "내일 다시 해 보세요",
        "끝입니다",
        "말하기 연습",
        "작은 방이었습니다",
        "나는 학생입니다",
        "이것은 시작입니다",
        "밝은 빛이 보였습니다",
        "다음 사람",
        "입력해 주세요",
    ] {
        prints(source);
    }
}

#[test]
fn a_separator_or_a_dash_inside_a_message_is_ordinary_text() {
    assert_eq!(
        transpile("커피 · 차 · 물 말해줘\n").unwrap(),
        "print(\"커피 · 차 · 물\")\n"
    );
    assert_eq!(
        transpile("show NOW — press enter\n").unwrap(),
        "print(\"NOW — press enter\")\n"
    );
    assert_eq!(
        transpile("축하해 🎉 말해줘\n").unwrap(),
        "print(\"축하해 🎉\")\n"
    );
}

#[test]
fn a_one_word_line_that_nme_uses_as_a_word_stays_python() {
    // A word NME spells out itself keeps its Python meaning: `say`, `end`,
    // `skip` and `목록` are names a Python program is free to use, and a line
    // holding one of them opens or closes nothing.
    assert_eq!(transpile("say\n").unwrap(), "say\n");
    assert_eq!(transpile("end\n").unwrap(), "end\n");
    // A name the program set earlier is Python doing nothing, and that is not
    // NME's to change.
    assert_eq!(transpile("Mina = 1\nMina\n").unwrap(), "Mina = 1\nMina\n");
    // Any other one-word line prints. Until 2026-08-19 it stayed a bare
    // Python name and the program died with a `NameError` pointing at a line
    // that is not the mistake; `Mina` and `Sana` on their own lines are a
    // list of names somebody wrote, and now they say themselves.
    assert_eq!(
        transpile("Mina\nSana\n").unwrap(),
        "print(\"Mina\")\nprint(\"Sana\")\n"
    );
    assert_eq!(transpile("Hello\n").unwrap(), "print(\"Hello\")\n");
    assert_eq!(transpile("안녕\n").unwrap(), "print(\"안녕\")\n");
}
