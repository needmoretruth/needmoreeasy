use nme_core::transpile;

fn assert_transpiles(path: &str, source: &str) -> String {
    transpile(source).unwrap_or_else(|problems| panic!("{path} should transpile: {problems:?}"))
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
    let source_lines = source.lines().collect::<Vec<_>>();
    let python_lines = python.lines().collect::<Vec<_>>();
    assert_eq!(source_lines.len(), python_lines.len());

    for (line_number, (before, after)) in source_lines.iter().zip(&python_lines).enumerate() {
        if before.trim().is_empty() {
            continue;
        }
        assert_ne!(
            before.trim(),
            after.trim(),
            "line {} escaped the NME sentence parser instead of lowering",
            line_number + 1
        );
    }

    assert!(python.contains("zk_nizk_prove"));
    assert!(python.contains("zk_nizk_verify"));
    assert!(python.contains("zk_nizk_challenge"));
    assert!(python.contains("while ("));
}
