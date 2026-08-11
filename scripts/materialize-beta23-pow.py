#!/usr/bin/env python3
from pathlib import Path

example = Path("examples/needmorecoin-sentence.ko.nme")
text = example.read_text(encoding="utf-8")
old = "작업난이도는 8"
new = "작업난이도는 4"
if text.count(old) != 1:
    raise SystemExit(f"expected exactly one stale difficulty label, found {text.count(old)}")
example.write_text(text.replace(old, new, 1), encoding="utf-8")

test = Path("crates/nme-core/tests/cryptocurrency_examples.rs")
text = test.read_text(encoding="utf-8")
marker = "fn learning_proof_of_work_difficulty_is_consistent_across_six_examples()"
if marker in text:
    raise SystemExit("consistency test already materialized")
text += r'''

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
'''
test.write_text(text, encoding="utf-8")
print("materialized beta23 proof-of-work consistency fix")
