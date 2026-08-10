//! Broken input must produce friendly, beginner-oriented diagnostics —
//! never silently broken Python output.

use nme_core::diagnostics::{render_all, render_all_bilingual};
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

fn bilingual_err(source: &str) -> String {
    let problems = transpile(source).expect_err("expected an error");
    render_all_bilingual(&problems, source, "test.nme")
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
fn ask_requires_a_simple_target() {
    let target = err("ask 123, \"Number? \"\n");
    assert!(target.contains("name that should hold"), "{target}");
}

#[test]
fn ask_recovers_a_missing_comma_as_sentence_syntax() {
    assert_eq!(
        transpile("ask name \"Name? \"\n").unwrap(),
        "name = input(\"Name? \")\n"
    );
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
    let say = bilingual_err("말해 1 +\n");
    assert!(say.contains("이해하지 못했어요"), "{say}");
    assert!(say.contains("couldn't understand"), "{say}");

    let repeat = bilingual_err("3번:\n말해 \"들여쓰기 없음\"\n");
    assert!(repeat.contains("들여써야"), "{repeat}");
    assert!(repeat.contains("must be indented"), "{repeat}");

    let when = bilingual_err("만약 준비됨\n");
    assert!(when.contains("필요해요"), "{when}");
    assert!(when.contains("needs `:`"), "{when}");
}

#[test]
fn only_random_is_bundled() {
    let message = err("use math\n");
    assert!(message.contains("only bundles `use random`"), "{message}");
}

#[test]
fn unavailable_random_version_reports_the_bundled_version() {
    let message = err("use random version \"9.9.9\"\n");
    assert!(message.contains("9.9.9"), "{message}");
    assert!(message.contains("0.0.1"), "{message}");
}

#[test]
fn sentence_punctuation_without_an_action_is_ambiguous() {
    let message = err("Hello there!\n");
    assert!(message.contains("ambiguous"), "{message}");
    assert!(message.contains("show"), "{message}");
}

#[test]
fn action_typos_must_have_one_unambiguous_meaning() {
    let say_or_ask = err("asy name Hello\n");
    assert!(say_or_ask.contains("more than one action"), "{say_or_ask}");

    let ask_or_use = err("usk random latest\n");
    assert!(ask_or_use.contains("more than one action"), "{ask_or_use}");
}

#[test]
fn unknown_prose_and_broken_sentence_actions_never_pass_silently() {
    let prose = err("hello world\n");
    assert!(prose.contains("clear action"), "{prose}");

    let typo = err("shwoe Hello\n");
    assert!(typo.contains("clear action"), "{typo}");
}

#[test]
fn condition_templates_reject_unexplained_middle_words() {
    let message = err("ready = True\nif ready banana exists then show no\n");
    assert!(message.contains("condition"), "{message}");
}

#[test]
fn module_sentences_reject_negation_reordering_and_extra_words() {
    for source in [
        "never use random\n",
        "version 0.0.1 random use\n",
        "use random latest version 9.9.9\n",
        "use os and random\n",
    ] {
        let message = err(source);
        assert!(
            message.contains("module") || message.contains("choose either"),
            "{source:?}: {message}"
        );
    }
}

#[test]
fn a_one_edit_condition_connector_is_recovered() {
    assert_eq!(
        transpile("name = \"Ada\"\n만약에 name이 있으먄 안녕 말해줘\n").unwrap(),
        "name = \"Ada\"\nif (name): print(\"안녕\")\n"
    );
}

#[test]
fn sentence_lowering_never_changes_physical_line_numbers() {
    let message = err("show Hello \\\nworld\n");
    assert!(message.contains("one physical line"), "{message}");
}

#[test]
fn explicit_blocks_report_structural_mistakes() {
    let missing = err("while ready\nshow waiting\n");
    assert!(missing.contains("missing its closing `end`"), "{missing}");
    let unmatched = err("else\n");
    assert!(unmatched.contains("open condition block"), "{unmatched}");
    let outside = err("break\n");
    assert!(outside.contains("inside a loop"), "{outside}");
}

#[test]
fn incomplete_value_changes_get_a_friendly_diagnostic() {
    let message = err("score add\n");
    assert!(message.contains("value change"), "{message}");
    assert!(message.contains("score add 1"), "{message}");
}
