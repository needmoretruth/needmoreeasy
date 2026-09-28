//! A comparison whose two sides can never be equal is refused, not compiled.
//!
//! Every one of these programs used to compile into Python that ran and did
//! the wrong thing for ever: a branch that no input could reach. What is
//! tested here is mostly the other direction — the shapes that look like the
//! same mistake and are not, which must keep compiling exactly as before.

use nme_core::diagnostics::DiagnosticCode;
use nme_core::transpile;

fn refused(source: &str) -> String {
    let problems = transpile(source).expect_err("expected a refusal");
    assert_eq!(problems.len(), 1, "expected exactly one: {problems:?}");
    assert_eq!(problems[0].code, DiagnosticCode::ComparisonNeverTrue);
    let problem = &problems[0];
    format!(
        "{} [{}] {} [{}]",
        problem.message,
        problem.hint.clone().unwrap_or_default(),
        problem.message_ko.clone().unwrap_or_default(),
        problem.hint_ko.clone().unwrap_or_default()
    )
}

fn compiles(source: &str) -> String {
    transpile(source).expect("expected this to compile")
}

/// The bug this check exists for. `수호룬` is a menu word to the writer and a
/// name to the compiler, so the branch was dead and nothing said so.
#[test]
fn a_menu_word_that_is_also_a_name_is_refused() {
    let message = refused(concat!(
        "수호룬은 거짓\n",
        "선택을 물어봐 수호룬, 폭약\n",
        "만약에 선택이 수호룬과 같으면\n",
        "    수호룬을 골랐습니다 말해줘\n",
        "끝\n",
    ));
    assert!(message.contains("수호룬"), "{message}");
    assert!(message.contains("참/거짓"), "{message}");
    assert!(message.contains("글"), "{message}");
}

/// The commoner mistake the same check catches: an answer read as text and
/// weighed against a number.
#[test]
fn an_answer_asked_as_text_never_equals_a_number() {
    let message = refused(concat!(
        "답을 물어봐 숫자를 대세요\n",
        "만약에 답이 7과 같으면\n",
        "    맞았습니다 말해줘\n",
        "끝\n",
    ));
    assert!(message.contains("숫자로 물어봐"), "{message}");
}

#[test]
fn the_same_refusal_reads_in_english() {
    let message = refused(concat!(
        "ask reply Type a number\n",
        "if reply equals 7\n",
        "show Correct\n",
        "end\n",
    ));
    assert!(message.contains("can never be true"), "{message}");
    assert!(message.contains("ask number"), "{message}");
}

/// `!=` on the same two kinds is the mirror mistake: always true. It is worth
/// stopping for only where something below it never runs.
#[test]
fn the_opposite_comparison_says_it_is_always_true() {
    let message = refused(concat!(
        "정답은 7\n",
        "답을 물어봐 숫자를 대세요\n",
        "만약에 답이 정답과 같지 않으면\n",
        "    틀렸습니다 말해줘\n",
        "아니면\n",
        "    맞았습니다 말해줘\n",
        "끝\n",
    ));
    assert!(message.contains("무엇을 넣어도 참"), "{message}");
}

/* --- the shapes that must keep compiling --------------------------------- */

/// Python says `True == 1`, so this pair really can meet and is not ours.
#[test]
fn a_number_against_true_or_false_is_left_alone() {
    let python = compiles(concat!(
        "깃발은 참\n",
        "만약에 깃발이 1과 같으면\n",
        "    켜져 있습니다 말해줘\n",
        "끝\n",
    ));
    assert!(python.contains("if (깃발 == 1):"), "{python}");
}

/// The number-guessing game every guide opens with: both sides are numbers.
#[test]
fn asking_for_a_number_and_comparing_it_still_compiles() {
    let python = compiles(concat!(
        "정답은 1부터 10까지 무작위 숫자\n",
        "추측을 숫자로 물어봐 숫자를 맞혀 보세요\n",
        "만약에 추측이 정답과 같으면\n",
        "    맞았습니다 말해줘\n",
        "끝\n",
    ));
    assert!(python.contains("if (추측 == 정답):"), "{python}");
}

/// A name a plain Python line touches is beyond the check, whatever NME did
/// with it further up. Without this, one line of Python would turn a correct
/// program into a refused one.
#[test]
fn a_name_a_python_line_touches_is_never_reported() {
    let python = compiles(concat!(
        "답은 안녕\n",
        "답 = 7\n",
        "만약에 답이 7과 같으면\n",
        "    맞았습니다 말해줘\n",
        "끝\n",
    ));
    assert!(python.contains("if (답 == 7):"), "{python}");
}

/// Given two kinds by NME itself, a name is dropped rather than guessed at.
#[test]
fn a_name_given_two_kinds_is_never_reported() {
    let python = compiles(concat!(
        "답은 안녕\n",
        "답은 7\n",
        "만약에 답이 7과 같으면\n",
        "    맞았습니다 말해줘\n",
        "끝\n",
    ));
    assert!(python.contains("if (답 == 7):"), "{python}");
}

/// Two pieces of text are two pieces of text, however they were made.
#[test]
fn text_against_text_is_left_alone() {
    let python = compiles(concat!(
        "비밀은 용\n",
        "답을 물어봐 비밀번호가 무엇입니까\n",
        "만약에 답이 비밀과 같으면\n",
        "    열렸습니다 말해줘\n",
        "끝\n",
    ));
    assert!(python.contains("if (답 == 비밀):"), "{python}");
}

/// A name a loop hands out is not one this check knows.
#[test]
fn a_loop_name_is_never_reported() {
    let python = compiles(concat!(
        "점수들은 목록 1, 2, 3\n",
        "점수들의 점수마다 반복해\n",
        "    만약에 점수가 안녕과 같으면\n",
        "        같습니다 말해줘\n",
        "    끝\n",
        "끝\n",
    ));
    assert!(python.contains("for 점수 in 점수들:"), "{python}");
}

/// A menu word that is also the name of a job. Nothing a reader types is ever
/// equal to a function, so this branch was dead too.
#[test]
fn a_menu_word_that_is_also_a_job_is_refused() {
    let message = refused(concat!(
        "공격이라는 일:\n",
        "    칩니다 말해줘\n",
        "끝\n",
        "행동을 물어봐 공격, 도망\n",
        "만약에 행동이 공격과 같으면\n",
        "    쳤습니다 말해줘\n",
        "끝\n",
    ));
    assert!(message.contains("일"), "{message}");
}

/// Looking for true-or-false in a list of words: the same dead branch, worn
/// as `contains` rather than `equals`.
#[test]
fn looking_for_a_flag_in_a_list_of_words_is_refused() {
    let message = refused(concat!(
        "검은 거짓\n",
        "가방은 목록 밧줄, 램프\n",
        "만약에 가방에 검이 있으면\n",
        "    있습니다 말해줘\n",
        "끝\n",
    ));
    assert!(message.contains("목록"), "{message}");
}

/// A list of words really can hold a word, so this one is left alone.
#[test]
fn looking_for_a_word_in_a_list_of_words_still_compiles() {
    let python = compiles(concat!(
        "가방은 목록 밧줄, 램프\n",
        "찾는것을 물어봐 무엇을 찾습니까\n",
        "만약에 가방에 찾는것이 있으면\n",
        "    있습니다 말해줘\n",
        "끝\n",
    ));
    assert!(python.contains("if (찾는것 in 가방):"), "{python}");
}

/// A list built a line at a time says nothing about what it holds, so the
/// check keeps quiet about it.
#[test]
fn a_list_built_up_a_line_at_a_time_is_never_reported() {
    let python = compiles(concat!(
        "검은 거짓\n",
        "가방은 빈 목록\n",
        "가방에 밧줄 넣어\n",
        "만약에 가방에 검이 있으면\n",
        "    있습니다 말해줘\n",
        "끝\n",
    ));
    assert!(python.contains("if (검 in 가방):"), "{python}");
}

/// The middle level writes the condition as Python with the sentences around
/// it. The same dead branch has to be caught there.
#[test]
fn the_same_dead_comparison_is_caught_when_the_condition_is_python() {
    let message = refused(concat!(
        "수호룬은 거짓\n",
        "선택을 물어봐 수호룬, 폭약\n",
        "만약 선택 == 수호룬:\n",
        "    골랐습니다 말해줘\n",
    ));
    assert!(message.contains("수호룬"), "{message}");
}

/// The Python reading is deliberately narrow: one name or one written value
/// on each side, and nothing else. A call or a piece of arithmetic in there is
/// left to Python, even where a person could see the branch is dead.
#[test]
fn a_python_condition_with_more_in_it_is_left_alone() {
    let python = compiles(concat!(
        "수호룬은 거짓\n",
        "선택을 물어봐 수호룬, 폭약\n",
        "만약 선택.lower() == 수호룬:\n",
        "    골랐습니다 말해줘\n",
    ));
    assert!(python.contains("선택.lower() == 수호룬"), "{python}");
}

/// One dead half of an `or` is still a dead half, and it is reported wherever
/// it stands — the sentence spelling of `또는` behaves the same way.
#[test]
fn a_dead_half_of_an_or_is_still_reported() {
    let message = refused(concat!(
        "수호룬은 거짓\n",
        "선택을 물어봐 수호룬, 폭약\n",
        "만약 선택 == 수호룬 or 선택 == \"폭약\":\n",
        "    골랐습니다 말해줘\n",
    ));
    assert!(message.contains("수호룬"), "{message}");
}

/// The program that a refusal built on the question's own words would have
/// turned away: a password weighed against the word the program is holding.
#[test]
fn a_password_program_keeps_compiling() {
    let python = compiles(concat!(
        "비밀번호는 열려라\n",
        "입력을 물어봐 비밀번호:\n",
        "만약에 입력이 비밀번호와 같으면\n",
        "    들어오세요 말해줘\n",
        "끝\n",
    ));
    assert!(python.contains("if (입력 == 비밀번호):"), "{python}");
}

/// A name given a second kind inside a block is out of the check's reach, and
/// the program keeps compiling.
#[test]
fn a_name_reassigned_inside_a_block_is_never_reported() {
    let python = compiles(concat!(
        "수호룬은 참\n",
        "3번 반복해\n",
        "    수호룬은 폭약\n",
        "끝\n",
        "선택을 물어봐 무엇을 하시겠습니까\n",
        "만약에 선택이 수호룬과 같으면\n",
        "    맞습니다 말해줘\n",
        "끝\n",
    ));
    assert!(python.contains("if (선택 == 수호룬):"), "{python}");
}

/// Trimming the answer before weighing it is the commonest shape of all, and
/// it must still be caught.
#[test]
fn a_lowercased_answer_against_a_flag_is_refused() {
    let message = refused(concat!(
        "수호룬은 거짓\n",
        "선택을 물어봐 수호룬, 폭약\n",
        "다듬은것은 선택 소문자로\n",
        "만약에 다듬은것이 수호룬과 같으면\n",
        "    골랐습니다 말해줘\n",
        "끝\n",
    ));
    assert!(message.contains("참/거짓"), "{message}");
}

/// Always true, with nothing hanging off it: the body runs, which is what the
/// line says. Nothing of the reader's is lost, so nothing is said.
#[test]
fn always_true_with_no_other_branch_is_left_alone() {
    let python = compiles(concat!(
        "칸은 0\n",
        "만약에 칸이 안녕과 같지 않으면\n",
        "    숫자입니다 말해줘\n",
        "끝\n",
    ));
    assert!(python.contains("if (칸 != \"안녕\"):"), "{python}");
}

/// The same line with an `아니면` under it does lose something — that branch
/// runs for no input at all — so it is refused.
#[test]
fn always_true_with_an_other_branch_is_refused() {
    let message = refused(concat!(
        "칸은 0\n",
        "만약에 칸이 안녕과 같지 않으면\n",
        "    숫자입니다 말해줘\n",
        "아니면\n",
        "    글입니다 말해줘\n",
        "끝\n",
    ));
    assert!(message.contains("무엇을 넣어도 참"), "{message}");
}

/// `ask number` is the answer only where a question made the name. A name the
/// program set itself has no question to ask differently.
#[test]
fn the_ask_number_advice_only_goes_where_a_question_was_asked() {
    let asked = refused(concat!(
        "답을 물어봐 숫자를 대세요\n",
        "만약에 답이 7과 같으면\n",
        "    맞았습니다 말해줘\n",
        "끝\n",
    ));
    assert!(asked.contains("숫자로 물어봐"), "{asked}");

    let set = refused(concat!(
        "칸은 안녕\n",
        "만약에 칸이 7과 같으면\n",
        "    맞았습니다 말해줘\n",
        "끝\n",
    ));
    assert!(!set.contains("숫자로 물어봐"), "{set}");
}
