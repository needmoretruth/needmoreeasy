//! Broken input must produce friendly, beginner-oriented diagnostics —
//! never silently broken Python output.

use nme_core::diagnostics::render_all;
use nme_core::transpile;

/// Transpiles and expects exactly one diagnostic; returns message + hint.
fn err(source: &str) -> String {
    match transpile(source) {
        Ok(output) => panic!("expected an error, got output: {output:?}"),
        Err(problems) => {
            assert_eq!(
                problems.len(),
                1,
                "expected exactly one error: {problems:?}"
            );
            let problem = &problems[0];
            match &problem.hint {
                Some(hint) => format!("{} [hint: {hint}]", problem.message),
                None => problem.message.clone(),
            }
        }
    }
}

#[test]
fn times_without_indented_block() {
    let message = err("5 times:\nsay \"hi\"\n");
    assert!(message.contains("indented"), "{message}");
    assert!(message.contains("hint"), "{message}");
}

#[test]
fn times_at_end_of_file() {
    let message = err("5 times:\n");
    assert!(message.contains("indented"), "{message}");
}

#[test]
fn times_with_ununderstandable_count() {
    let message = err("x = 5 times:\n    say \"hi\"\n");
    assert!(message.contains("how many times"), "{message}");
}

#[test]
fn say_with_ununderstandable_value() {
    let message = err("say 1 +\n");
    assert!(message.contains("what you want to `say`"), "{message}");
}

#[test]
fn inline_block_cannot_open_a_block() {
    let message = err("2 times: 3 times:\n    say \"x\"\n");
    assert!(message.contains("can't start"), "{message}");
}

#[test]
fn inline_body_allows_only_one_statement() {
    let message = err("5 times: say \"a\"; say \"b\"\n");
    assert!(message.contains("only one statement"), "{message}");
}

#[test]
fn unterminated_string_is_reported_gently() {
    let message = err("say \"oops\n");
    assert!(
        message.contains("not something Python or NME can read"),
        "{message}"
    );
}

#[test]
fn all_problems_are_reported_at_once() {
    let problems = transpile("5 times:\nsay 1 +\n").unwrap_err();
    assert_eq!(problems.len(), 2, "{problems:?}");
}

#[test]
fn diagnostics_render_with_location_and_hint() {
    let source = "say \"ok\"\nsay 1 +\n";
    let problems = transpile(source).unwrap_err();
    let rendered = render_all(&problems, source, "hello.nme");
    assert!(rendered.contains("error:"), "{rendered}");
    // The caret points at the offending expression on line 2.
    assert!(rendered.contains("hello.nme:2:5"), "{rendered}");
    assert!(rendered.contains("say 1 +"), "{rendered}");
    assert!(rendered.contains("hint:"), "{rendered}");
}

#[test]
fn ask_requires_a_simple_target_and_comma() {
    let target = err("ask 123, \"Number? \"\n");
    assert!(target.contains("name that should hold"), "{target}");

    let comma = err("ask name \"Name? \"\n");
    assert!(comma.contains("comma"), "{comma}");
}

#[test]
fn ask_requires_a_valid_prompt() {
    let missing = err("ask name,\n");
    assert!(missing.contains("missing"), "{missing}");

    let invalid = err("ask name, 1 +\n");
    assert!(invalid.contains("question"), "{invalid}");
}

#[test]
fn when_requires_a_condition_colon_and_body() {
    let colon = err("when ready\n");
    assert!(colon.contains("needs `:`"), "{colon}");

    let condition = err("when:\n    say \"no\"\n");
    assert!(condition.contains("condition is missing"), "{condition}");

    let body = err("when ready:\nsay \"not indented\"\n");
    assert!(body.contains("indented"), "{body}");
}

#[test]
fn korean_forms_return_korean_guidance() {
    let say = err("말해 1 +\n");
    assert!(say.contains("이해하지 못했어요"), "{say}");

    let repeat = err("3번:\n말해 \"들여쓰기 없음\"\n");
    assert!(repeat.contains("들여써야"), "{repeat}");

    let when = err("만약 준비됨\n");
    assert!(when.contains("필요해요"), "{when}");
}

#[test]
fn only_random_is_bundled() {
    let message = err("use math\n");
    assert!(message.contains("only bundles `use random`"), "{message}");
}
