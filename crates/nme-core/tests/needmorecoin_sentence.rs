use nme_core::transpile;

#[test]
fn needmorecoin_uses_only_korean_sentence_source() {
    let source = include_str!("../../../examples/needmorecoin.ko.nme");

    assert!(
        source.chars().all(|ch| {
            ch.is_whitespace() || ch.is_ascii_digit() || ('가'..='힣').contains(&ch)
        }),
        "needmorecoin.ko.nme must contain only Hangul, decimal digits, and whitespace"
    );

    let python = transpile(source)
        .unwrap_or_else(|problems| panic!("needmorecoin should transpile: {problems:?}"));

    assert_eq!(source.lines().count(), python.lines().count());
    assert!(python.contains("sha256"));
    assert!(python.contains("secrets"));
    assert!(python.contains("영지식비대화증명"));
    assert!(python.contains("영지식비대화도전"));
}
