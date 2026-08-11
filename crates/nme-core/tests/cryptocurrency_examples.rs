use nme_core::transpile;

fn assert_transpiles(path: &str, source: &str) -> String {
    transpile(source).unwrap_or_else(|problems| panic!("{path} should transpile: {problems:?}"))
}

fn assert_every_nonempty_line_lowers(path: &str, source: &str, python: &str) {
    let source_lines = source.lines().collect::<Vec<_>>();
    let python_lines = python.lines().collect::<Vec<_>>();
    assert_eq!(source_lines.len(), python_lines.len(), "{path}");

    for (line_number, (before, after)) in source_lines.iter().zip(&python_lines).enumerate() {
        if before.trim().is_empty() {
            continue;
        }
        assert_ne!(
            before.trim(),
            after.trim(),
            "{path} line {} escaped the NME sentence parser instead of lowering",
            line_number + 1
        );
    }
}

#[test]
fn all_six_cryptocurrency_examples_transpile() {
    let examples = [
        (
            "needmorecoin-sentence.ko.nme",
            include_str!("../../../examples/needmorecoin-sentence.ko.nme"),
        ),
        (
            "needmorecoin-sentence.en.nme",
            include_str!("../../../examples/needmorecoin-sentence.en.nme"),
        ),
        (
            "needmorecoin-beginner.ko.nme",
            include_str!("../../../examples/needmorecoin-beginner.ko.nme"),
        ),
        (
            "needmorecoin-beginner.en.nme",
            include_str!("../../../examples/needmorecoin-beginner.en.nme"),
        ),
        (
            "needmorecoin-advanced.ko.nme",
            include_str!("../../../examples/needmorecoin-advanced.ko.nme"),
        ),
        (
            "needmorecoin-advanced.en.nme",
            include_str!("../../../examples/needmorecoin-advanced.en.nme"),
        ),
    ];

    for (path, source) in examples {
        let python = assert_transpiles(path, source);
        assert_eq!(source.lines().count(), python.lines().count(), "{path}");
    }
}

#[test]
fn korean_sentence_cryptocurrency_is_pure_sentence_source() {
    let source = include_str!("../../../examples/needmorecoin-sentence.ko.nme");

    assert!(
        source.chars().all(|ch| {
            ch.is_whitespace() || ch.is_ascii_digit() || ('가'..='힣').contains(&ch)
        }),
        "the Korean sentence example may contain only Hangul, decimal digits, and whitespace"
    );

    let python = assert_transpiles("needmorecoin-sentence.ko.nme", source);
    assert_every_nonempty_line_lowers("needmorecoin-sentence.ko.nme", source, &python);
    assert!(python.contains("zk_nizk_prove"));
    assert!(python.contains("zk_nizk_verify"));
    assert!(python.contains("zk_nizk_challenge"));
    assert!(python.contains("while ("));
}

#[test]
fn english_sentence_cryptocurrency_is_pure_sentence_source() {
    let source = include_str!("../../../examples/needmorecoin-sentence.en.nme");

    assert!(
        source
            .chars()
            .all(|ch| ch.is_whitespace() || ch.is_ascii_digit() || ch.is_ascii_alphabetic()),
        "the English sentence example may contain only ASCII letters, decimal digits, and whitespace"
    );

    let python = assert_transpiles("needmorecoin-sentence.en.nme", source);
    assert_every_nonempty_line_lowers("needmorecoin-sentence.en.nme", source, &python);
    assert!(python.contains("zk_nizk_prove"));
    assert!(python.contains("zk_nizk_verify"));
    assert!(python.contains("zk_nizk_challenge"));
    assert!(python.contains("while ("));
}

#[test]
fn beginner_cryptocurrency_examples_stay_in_compact_nme() {
    let korean = include_str!("../../../examples/needmorecoin-beginner.ko.nme");
    let english = include_str!("../../../examples/needmorecoin-beginner.en.nme");

    assert!(korean.contains("저장 "));
    assert!(korean.contains("말해 "));
    assert!(english.contains("set "));
    assert!(english.contains("say "));
    for source in [korean, english] {
        assert!(!source.contains("def "));
        assert!(!source.contains("class "));
    }
}

#[test]
fn advanced_cryptocurrency_examples_are_byte_identical_python() {
    let korean = include_str!("../../../examples/needmorecoin-advanced.ko.nme");
    let english = include_str!("../../../examples/needmorecoin-advanced.en.nme");

    assert_eq!(
        assert_transpiles("needmorecoin-advanced.ko.nme", korean),
        korean
    );
    assert_eq!(
        assert_transpiles("needmorecoin-advanced.en.nme", english),
        english
    );

    for source in [korean, english] {
        assert!(source.contains("hashlib.sha256"));
        assert!(source.contains("secrets.randbelow"));
        assert!(source.contains("work_nonce") || source.contains("작업번호"));
    }
}

#[test]
fn learning_proof_of_work_difficulty_is_consistent_across_six_examples() {
    const FOUR_BIT_TARGET: &str =
        "7237005577332262213973186563042994240829374041602535252466099000494570602496";

    let sentence_ko = include_str!("../../../examples/needmorecoin-sentence.ko.nme");
    let sentence_en = include_str!("../../../examples/needmorecoin-sentence.en.nme");
    let beginner_ko = include_str!("../../../examples/needmorecoin-beginner.ko.nme");
    let beginner_en = include_str!("../../../examples/needmorecoin-beginner.en.nme");
    let advanced_ko = include_str!("../../../examples/needmorecoin-advanced.ko.nme");
    let advanced_en = include_str!("../../../examples/needmorecoin-advanced.en.nme");

    assert!(sentence_ko.contains("작업난이도는 4"));
    for source in [sentence_ko, sentence_en, beginner_ko, beginner_en] {
        assert!(source.contains(FOUR_BIT_TARGET));
    }
    assert!(advanced_ko.contains("채굴접두사 = \"0\""));
    assert!(advanced_en.contains("MINING_PREFIX = \"0\""));
}
