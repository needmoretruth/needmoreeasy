//! End-to-end tests for the NME → Python transpiler.
//!
//! These tests encode the language's compatibility contract:
//!
//! * **pure Python** comes out byte-identical,
//! * **pure NME** lowers to the expected Python,
//! * **mixed** files combine both freely,
//! * text inside **strings and comments** is never touched.

use nme_core::transpile;

/// Transpiles and expects success.
fn ok(source: &str) -> String {
    transpile(source)
        .unwrap_or_else(|problems| panic!("expected successful transpile, got: {problems:?}"))
}

// --------------------------------------------------------- explicit end blocks

#[test]
fn an_indented_sentence_block_closes_before_the_next_flat_block() {
    let python = ok("만약 True라면\n    say \"a\"\n점수가 5와 같으면\n점수 말해줘\n끝\n");
    assert_eq!(
        python,
        "if (True):\n    print(\"a\")\nif (점수 == 5):\n    print(\"점수\")\n# end\n"
    );
}

#[test]
fn an_indented_body_line_keeps_flat_continuations_in_the_same_flat_block() {
    let python = ok("if score is equal to 5\n    say \"x\"\nshow keep going\nend\n");
    assert_eq!(
        python,
        "if (score == 5):\n    print(\"x\")\n    print(\"keep going\")\n# end\n"
    );
}

#[test]
fn a_flat_block_after_an_indented_block_closes_with_one_end() {
    let python = ok("if ready\n    say \"a\"\nif score is equal to 5\nshow keep going\nend\n");
    assert_eq!(
        python,
        "if (ready):\n    print(\"a\")\nif (score == 5):\n    print(\"keep going\")\n# end\n"
    );
}

#[test]
fn an_indented_suite_loop_followed_by_a_flat_block() {
    let python =
        ok("while score is less than 3\n    score add 1\nif score is equal to 5\nshow done\nend\n");
    assert_eq!(
        python,
        "while (score < 3):\n    score = score + 1\nif (score == 5):\n    print(\"done\")\n# end\n"
    );
}

#[test]
fn a_flat_header_after_an_indented_body_stays_nested_with_enough_ends() {
    let python =
        ok("while score is less than 3\n    score add 1\n만약 score == 2\nshow two\n끝\n끝\n");
    assert_eq!(
        python,
        "while (score < 3):\n    score = score + 1\n    if (score == 2):\n        print(\"two\")\n    # end\n# end\n"
    );
}

// ---------------------------------------------------------------- pure Python

#[test]
fn pure_python_is_byte_identical() {
    let source = r#"#!/usr/bin/env python3
"""Module docstring with NME-looking text: 5 times: say "hi"."""

import math
from dataclasses import dataclass

# 5 times: say "this is a comment, not code"


@dataclass
class Point:
    x: float
    y: float


def distance(a, b):
    """Say '5 times:' inside a docstring stays untouched."""
    return math.hypot(a.x - b.x, a.y - b.y)


async def main():
    points = [Point(x=i, y=i * 2) for i in range(10) if i % 2 == 0]
    match len(points):
        case 0:
            print("empty")
        case times:
            print(f"{times} points")
    text = f"result: {(lambda v: v * 2)(21)=}"
    print(text, sep="\n", end="\n")
    print(f"{distance(Point(0, 0), Point(3, 4))=}")


if __name__ == "__main__":
    import asyncio

    asyncio.run(main())
"#;
    assert_eq!(ok(source), source);
}

#[test]
fn empty_source_stays_empty() {
    assert_eq!(ok(""), "");
    assert_eq!(ok("\n\n# only comments\n"), "\n\n# only comments\n");
}

// ----------------------------------------------------------------- pure NME

#[test]
fn say_a_string() {
    assert_eq!(ok("say \"Hello\"\n"), "print(\"Hello\")\n");
}

#[test]
fn say_an_expression() {
    assert_eq!(ok("say 1 + 1\n"), "print(1 + 1)\n");
    assert_eq!(ok("say f\"hi {name}\"\n"), "print(f\"hi {name}\")\n");
}

#[test]
fn korean_say_uses_the_same_semantics() {
    assert_eq!(ok("말해 \"안녕하세요\"\n"), "print(\"안녕하세요\")\n");
    assert_eq!(ok("말해 1 + 1\n"), "print(1 + 1)\n");
}

#[test]
fn ask_reads_text_into_an_english_or_korean_name() {
    assert_eq!(ok("ask name\n"), "name = input()\n");
    assert_eq!(
        ok("ask name, \"Your name? \"\n"),
        "name = input(\"Your name? \")\n"
    );
    assert_eq!(
        ok("물어봐 이름, \"이름이 뭐예요? \"\n"),
        "이름 = input(\"이름이 뭐예요? \")\n"
    );
    assert_eq!(
        ok("물어봐 이름, 이름이 뭐예요?\n"),
        "이름 = input(\"이름이 뭐예요?\" + \" \")\n"
    );
}

#[test]
fn ordinary_multiword_sentences_need_no_output_action() {
    assert_eq!(
        ok("Hello everyone!\n오늘도 반가워요!\n"),
        "print(\"Hello everyone!\")\nprint(\"오늘도 반가워요!\")\n"
    );
}

#[test]
fn ordinary_contractions_and_logical_words_need_no_output_action() {
    let source = "Don't stop!\nIt's easy.\nI'm happy and you're ready!\n";
    assert_eq!(
        ok(source),
        "print(\"Don't stop!\")\nprint(\"It's easy.\")\nprint(\"I'm happy and you're ready!\")\n"
    );
}

#[test]
fn sentence_syntax_needs_no_quotes_commas_braces_or_colons() {
    let source = concat!(
        "이름을 물어봐 이름이 뭐예요?\n",
        "안녕하세요 이름! 말해줘\n",
        "3번 반복해서 NME에 오신 것을 환영합니다 말해줘\n",
    );
    let expected = concat!(
        "이름 = input(\"이름이 뭐예요?\" + \" \")\n",
        "print(\"안녕하세요 \" + str(이름) + \"!\")\n",
        "for _ in range(3): print(\"NME에 오신 것을 환영합니다\")\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn sentence_blocks_are_colon_free_and_accept_connecting_words() {
    let source = concat!(
        "이름을 물어봐 이름이 뭐예요\n",
        "만약에 이름이 있으면\n",
        "    안녕하세요 이름 말해줘\n",
        "repeat 2 times\n",
        "    show Welcome 이름\n",
    );
    let expected = concat!(
        "이름 = input(\"이름이 뭐예요\" + \" \")\n",
        "if (이름):\n",
        "    print(\"안녕하세요 \" + str(이름))\n",
        "for _ in range(2):\n",
        "    print(\"Welcome \" + str(이름))\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn korean_condition_can_end_with_a_subject_particle() {
    let source = "이름 = \"NME\"\n만약에 이름이\n    이름 말해줘\n";
    let expected = "이름 = \"NME\"\nif (이름):\n    print(이름)\n";
    assert_eq!(ok(source), expected);
}

#[test]
fn python_beginner_sentence_korean_and_english_mix_line_by_line() {
    let source = concat!(
        "조건 = True\n",
        "if 조건\n",
        "    성공이라고 말해\n",
        "2 times 반복해\n",
        "    말해 \"mixed\"\n",
    );
    let expected = concat!(
        "조건 = True\n",
        "if (조건):\n",
        "    print(\"성공\")\n",
        "for _ in range(2):\n",
        "    print(\"mixed\")\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn sentence_output_uses_a_variable_bound_by_an_advanced_python_loop() {
    let source = "for person in [\"Ada\", \"Grace\"]:\n    show Hello person!\n";
    let expected =
        "for person in [\"Ada\", \"Grace\"]:\n    print(\"Hello \" + str(person) + \"!\")\n";
    assert_eq!(ok(source), expected);
}

#[test]
fn common_one_edit_typos_are_recovered_only_after_python_rejects_the_line() {
    let source = concat!(
        "물어바 이름 이름이 뭐예요\n",
        "안녕하세요 이름 말헤\n",
        "2번 반목해서 다시 말해줘\n",
        "repaet 2 times and show typo fixed\n",
    );
    let expected = concat!(
        "이름 = input(\"이름이 뭐예요\" + \" \")\n",
        "print(\"안녕하세요 \" + str(이름))\n",
        "for _ in range(2): print(\"다시\")\n",
        "for _ in range(2): print(\"typo fixed\")\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn sentence_assignments_and_random_numbers_are_symbol_free() {
    let source = "주사위는 1부터 6까지 랜덤정수\n주사위 말해줘\n";
    let expected = concat!(
        "주사위 = __import__(\"random\").randint(1, 6)\n",
        "print(주사위)\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn sentence_set_is_not_mistaken_for_a_typo_of_get() {
    assert_eq!(
        ok("set name to \"Ada\"\nshow name\n"),
        "name = \"Ada\"\nprint(name)\n"
    );
}

#[test]
fn a_korean_number_guessing_game_can_use_only_sentence_syntax() {
    let source = concat!(
        "정답은 1부터 10까지 랜덤정수\n",
        "추측을 숫자로 물어봐 1부터 10까지 숫자를 맞혀 보세요\n",
        "만약에 추측이 정답과 같으면\n",
        "    정답입니다! 말해줘\n",
        "만약에 추측이 정답보다 작으면\n",
        "    더 큰 수예요 말해줘\n",
        "만약에 추측이 정답보다 크면\n",
        "    더 작은 수예요 말해줘\n",
    );
    let expected = concat!(
        "정답 = __import__(\"random\").randint(1, 10)\n",
        "추측 = int(input(\"1부터 10까지 숫자를 맞혀 보세요\" + \" \"))\n",
        "if (추측 == 정답):\n",
        "    print(\"정답입니다!\")\n",
        "if (추측 < 정답):\n",
        "    print(\"더 큰 수예요\")\n",
        "if (추측 > 정답):\n",
        "    print(\"더 작은 수예요\")\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn an_english_number_game_can_use_only_sentence_syntax() {
    let source = concat!(
        "set answer to random number from 1 to 10\n",
        "ask number guess Pick a number\n",
        "if guess equals answer then show Correct!\n",
        "set color to pick from red or green or blue\n",
        "show color\n",
    );
    let expected = concat!(
        "answer = __import__(\"random\").randint(1, 10)\n",
        "guess = int(input(\"Pick a number\" + \" \"))\n",
        "if (guess == answer): print(\"Correct!\")\n",
        "color = __import__(\"random\").choice((\"red\", \"green\", \"blue\",))\n",
        "print(color)\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn english_sentence_conditions_support_comparisons_and_inline_then() {
    let source = concat!(
        "score = 7\n",
        "if score is greater than 5 then show high\n",
        "if score equals 7\n",
        "    show exact\n",
        "if score exists then show present\n",
        "만약에 score가 7과 같으면 일곱 말해줘\n",
    );
    let expected = concat!(
        "score = 7\n",
        "if (score > 5): print(\"high\")\n",
        "if (score == 7):\n",
        "    print(\"exact\")\n",
        "if (score): print(\"present\")\n",
        "if (score == 7): print(\"일곱\")\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn subject_first_korean_conditions_are_not_mistaken_for_updates() {
    let source = concat!(
        "색은 빨강\n",
        "색이 빨강과 같으면 말해 맞아요\n",
        "이름은 Ada\n",
        "이름이 있으면 안녕하세요 이름 말해줘\n",
    );
    let expected = concat!(
        "색 = \"빨강\"\n",
        "if (색 == \"빨강\"): print(\"맞아요\")\n",
        "이름 = \"Ada\"\n",
        "if (이름): print(\"안녕하세요 \" + str(이름))\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn condition_and_logical_connector_typos_are_recovered() {
    let source = concat!(
        "ready = True\n",
        "score = 3\n",
        "if ready 그리거 score > 2 then show go\n",
        "색은 빨강\n",
        "색이 빨강과 같먄 맞아요 말해줘\n",
    );
    let expected = concat!(
        "ready = True\n",
        "score = 3\n",
        "if ((ready and score > 2)): print(\"go\")\n",
        "색 = \"빨강\"\n",
        "if (색 == \"빨강\"): print(\"맞아요\")\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn subject_first_korean_conditions_can_use_flat_end_blocks() {
    let source = concat!(
        "색은 빨강\n",
        "색이 빨강과 같으면\n",
        "맞아요 말해줘\n",
        "끝\n",
    );
    let expected = concat!(
        "색 = \"빨강\"\n",
        "if (색 == \"빨강\"):\n",
        "    print(\"맞아요\")\n",
        "# end\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn parenthesized_logical_conditions_keep_the_shared_condition_shape() {
    let english = concat!(
        "ready = True\n",
        "score = 3\n",
        "if (ready and score > 2)\n",
        "    show yes\n",
        "end\n",
    );
    let english_expected = concat!(
        "ready = True\n",
        "score = 3\n",
        "if ((ready and score > 2)):\n",
        "    print(\"yes\")\n",
        "# end\n",
    );
    assert_eq!(ok(english), english_expected);

    let korean = concat!(
        "준비는 참\n",
        "점수는 3\n",
        "만약 (준비 그리고 점수 > 2)\n",
        "    성공 말해줘\n",
        "끝\n",
    );
    let korean_expected = concat!(
        "준비 = True\n",
        "점수 = 3\n",
        "if ((준비 and 점수 > 2)):\n",
        "    print(\"성공\")\n",
        "# end\n",
    );
    assert_eq!(ok(korean), korean_expected);

    assert_eq!(
        ok("if (ready and score > 2) then show yes\n"),
        "if ((ready and score > 2)): print(\"yes\")\n",
    );
    assert_eq!(
        ok("만약 (준비 그리고 점수 > 2) 그러면 성공 말해줘\n"),
        "if ((준비 and 점수 > 2)): print(\"성공\")\n",
    );
}

#[test]
fn parenthesized_korean_comparison_endings_keep_the_condition_body_boundary() {
    let source = concat!(
        "점수는 1\n",
        "만약 (점수가 2보다 작으면)\n",
        "작아요 말해줘\n",
        "끝\n",
    );
    let expected = concat!(
        "점수 = 1\n",
        "if (점수 < 2):\n",
        "    print(\"작아요\")\n",
        "# end\n",
    );
    assert_eq!(ok(source), expected);

    let branch = concat!(
        "점수는 3\n",
        "만약 거짓\n",
        "    안 돼 말해줘\n",
        "아니면 만약에 (점수가 4보다 작으면)\n",
        "    작아요 말해줘\n",
        "끝\n",
    );
    let branch_expected = concat!(
        "점수 = 3\n",
        "if (False):\n",
        "    print(\"안 돼\")\n",
        "elif (점수 < 4):\n",
        "    print(\"작아요\")\n",
        "# end\n",
    );
    assert_eq!(ok(branch), branch_expected);
}

#[test]
fn short_korean_condition_endings_accept_natural_equality_and_literals() {
    let source = concat!(
        "이름은 철수\n",
        "이름이 철수면 안녕 말해줘\n",
        "이름이 철수라면 다시 말해줘\n",
        "준비면 준비됐어 말해줘\n",
        "준비는 참\n",
        "만약 준비가 거짓이면 아니야 말해줘\n",
    );
    let expected = concat!(
        "이름 = \"철수\"\n",
        "if (이름 == \"철수\"): print(\"안녕\")\n",
        "if (이름 == \"철수\"): print(\"다시\")\n",
        "if (준비): print(\"준비됐어\")\n",
        "준비 = True\n",
        "if (준비 == False): print(\"아니야\")\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn spoken_condition_typos_and_english_synonyms_stay_unambiguous() {
    let source = concat!(
        "name = \"Ada\"\n",
        "만약에 name이 잇으면 안녕 말해줘\n",
        "이름은 철수\n",
        "이름이 철수먄 맞아 말해줘\n",
        "if score is great than 5 then show high\n",
        "if score is same as 5 then show equal\n",
    );
    let expected = concat!(
        "name = \"Ada\"\n",
        "if (name): print(\"안녕\")\n",
        "이름 = \"철수\"\n",
        "if (이름 == \"철수\"): print(\"맞아\")\n",
        "if (score > 5): print(\"high\")\n",
        "if (score == 5): print(\"equal\")\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn korean_then_connector_does_not_turn_the_subject_into_text() {
    let source = "만약 준비 그리고 점수 > 2 또는 기다림 그러면 성공 말해줘\n";
    assert_eq!(
        ok(source),
        "if (((준비 and 점수 > 2) or 기다림)): print(\"성공\")\n"
    );
}

#[test]
fn a_typo_in_the_korean_condition_starter_still_keeps_the_condition_shape() {
    assert_eq!(
        ok("만악에 이름이 있으면 안녕 말해줘\n"),
        "if (이름): print(\"안녕\")\n"
    );
}

#[test]
fn spaced_korean_particles_and_short_condition_endings_are_mixable() {
    let source = concat!(
        "이름 은 철수\n",
        "이름 이 철수 면 안녕 말해줘\n",
        "준비 가 거짓 이면 아니야 말해줘\n",
        "이름 이 있으면 환영 말해줘\n",
    );
    let expected = concat!(
        "이름 = \"철수\"\n",
        "if (이름 == \"철수\"): print(\"안녕\")\n",
        "if (준비 == False): print(\"아니야\")\n",
        "if (이름): print(\"환영\")\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn korean_condition_connector_can_be_spaced_or_attached() {
    let source = concat!(
        "이름 = \"Ada\"\n",
        "만약에 이름 이면\n",
        "말해 yes\n",
        "끝\n",
        "만약에 이름이면\n",
        "말해 again\n",
        "끝\n",
    );
    let expected = concat!(
        "이름 = \"Ada\"\n",
        "if (이름):\n",
        "    print(\"yes\")\n",
        "# end\n",
        "if (이름):\n",
        "    print(\"again\")\n",
        "# end\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn exact_output_words_are_never_reinterpreted_as_questions() {
    let source = concat!(
        "show task\n",
        "show mask\n",
        "show question\n",
        "show prompt\n",
        "show ask\n",
    );
    let expected = concat!(
        "print(\"task\")\n",
        "print(\"mask\")\n",
        "print(\"question\")\n",
        "print(\"prompt\")\n",
        "print(\"ask\")\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn sentence_names_follow_source_order_and_python_scope() {
    let source = concat!(
        "show Hello world\n",
        "world = \"earth\"\n",
        "def remember():\n",
        "    secret = \"inside\"\n",
        "show secret\n",
    );
    let expected = concat!(
        "print(\"Hello world\")\n",
        "world = \"earth\"\n",
        "def remember():\n",
        "    secret = \"inside\"\n",
        "print(\"secret\")\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn natural_conditions_keep_negation_literals_and_text_values() {
    let source = concat!(
        "set color to red\n",
        "if color equals red then show yes\n",
        "set score to 3\n",
        "if score is not greater than 5 then show expected\n",
        "set ready to false\n",
        "if ready is missing then show missing\n",
        "준비는 거짓\n",
        "값은 없음\n",
    );
    let expected = concat!(
        "color = \"red\"\n",
        "if (color == \"red\"): print(\"yes\")\n",
        "score = 3\n",
        "if (not (score > 5)): print(\"expected\")\n",
        "ready = False\n",
        "if (not (ready)): print(\"missing\")\n",
        "준비 = False\n",
        "값 = None\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn sentence_actions_accept_spaced_korean_phrases_and_particles() {
    let source = concat!(
        "안녕 말해 줘\n",
        "이름 을 물어 봐 이름이 뭐예요?\n",
        "2 번 반복 해 그리고 다시 말해 줘\n",
    );
    let expected = concat!(
        "print(\"안녕\")\n",
        "이름 = input(\"이름이 뭐예요?\" + \" \")\n",
        "for _ in range(2): print(\"다시\")\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn sentence_actions_allow_small_polite_fillers() {
    let source = concat!(
        "please show hello\n",
        "제발 안녕하세요 말해줘\n",
        "제발 물어봐 이름 이름이 뭐예요?\n",
        "좀 보여줘 다시\n",
    );
    let expected = concat!(
        "print(\"hello\")\n",
        "print(\"안녕하세요\")\n",
        "이름 = input(\"이름이 뭐예요?\" + \" \")\n",
        "print(\"다시\")\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn the_shortest_conversation_uses_no_prompt_punctuation_or_formatting() {
    let source = "name ask\nHello name show\n";
    assert_eq!(
        ok(source),
        "name = input()\nprint(\"Hello \" + str(name))\n"
    );
}

#[test]
fn spoken_show_requests_do_not_print_the_request_pronoun() {
    assert_eq!(ok("please show me hello\n"), "print(\"hello\")\n");
    assert_eq!(ok("보여줘 나 안녕\n"), "print(\"안녕\")\n");
}

#[test]
fn natural_questions_infer_a_target_without_ask_syntax() {
    let source = concat!(
        "이름이 뭐예요?\n",
        "내 이름은 뭐예요?\n",
        "이름 은 뭐예요?\n",
        "이름 뭐예요\n",
        "이름이 뭐예요\n",
        "나이 몇 살이에요\n",
        "몇 살이에요?\n",
        "나 몇 살이야\n",
        "나이는 몇 살이에요?\n",
        "안녕하세요 이름!\n",
        "What is your age?\n",
        "How old are you?\n",
        "How old am I\n",
        "What is your name\n",
        "What is my name\n",
        "What's your city?\n",
        "What's your city\n",
        "오늘 어때?\n",
    );
    let expected = concat!(
        "이름 = input(\"이름이 뭐예요?\" + \" \")\n",
        "이름 = input(\"내 이름은 뭐예요?\" + \" \")\n",
        "이름 = input(\"이름 은 뭐예요?\" + \" \")\n",
        "이름 = input(\"이름 뭐예요\" + \" \")\n",
        "이름 = input(\"이름이 뭐예요\" + \" \")\n",
        "나이 = input(\"나이 몇 살이에요\" + \" \")\n",
        "나이 = input(\"몇 살이에요?\" + \" \")\n",
        "나이 = input(\"나 몇 살이야\" + \" \")\n",
        "나이 = input(\"나이는 몇 살이에요?\" + \" \")\n",
        "print(\"안녕하세요 \" + str(이름) + \"!\")\n",
        "age = input(\"What is your age?\" + \" \")\n",
        "age = input(\"How old are you?\" + \" \")\n",
        "age = input(\"How old am I\" + \" \")\n",
        "name = input(\"What is your name\" + \" \")\n",
        "name = input(\"What is my name\" + \" \")\n",
        "city = input(\"What's your city?\" + \" \")\n",
        "city = input(\"What's your city\" + \" \")\n",
        "print(\"오늘 어때?\")\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn a_named_korean_repeat_count_can_keep_the_suffix_attached() {
    let source = "횟수 = 3\n횟수번:\n    안녕 말해줘\n";
    let expected = "횟수 = 3\nfor _ in range(횟수):\n    print(\"안녕\")\n";
    assert_eq!(ok(source), expected);
}

#[test]
fn sentence_repeat_can_omit_the_colon_and_repeat_plain_words() {
    let source = concat!(
        "3번 안녕하세요\n",
        "3 times Welcome to NME\n",
        "3 times and Welcome again\n",
        "3번 반복해 다시 만나요\n",
    );
    let expected = concat!(
        "for _ in range(3): print(\"안녕하세요\")\n",
        "for _ in range(3): print(\"Welcome to NME\")\n",
        "for _ in range(3): print(\"Welcome again\")\n",
        "for _ in range(3): print(\"다시 만나요\")\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn sentence_updates_can_change_a_value_without_plus_or_equals() {
    let source = concat!(
        "점수는 0\n",
        "점수에 1 더해\n",
        "점수에서 1 빼줘\n",
        "add 2 to score\n",
        "increase score by 3\n",
        "subtract 1 from score\n",
        "score add 1\n",
        "score add 1!\n",
    );
    let expected = concat!(
        "점수 = 0\n",
        "점수 = 점수 + 1\n",
        "점수 = 점수 - 1\n",
        "score = score + 2\n",
        "score = score + 3\n",
        "score = score - 1\n",
        "score = score + 1\n",
        "score = score + 1\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn one_word_sentence_output_uses_known_names_and_quotes_unknown_words() {
    let source = "say Hello\nHello = \"hi\"\nsay Hello\n말해 안녕하세요\n";
    let expected = concat!(
        "print(\"Hello\")\n",
        "Hello = \"hi\"\n",
        "print(Hello)\n",
        "print(\"안녕하세요\")\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn english_output_sentences_keep_apostrophes_without_escaping() {
    assert_eq!(
        ok("show I'm happy!\nshow John's book\n"),
        "print(\"I'm happy!\")\nprint(\"John's book\")\n"
    );
}

#[test]
fn module_sentences_accept_the_exact_unquoted_version() {
    let tools = concat!(
        "import random as 랜덤; random = 랜덤; ",
        "random_number = 랜덤.randint; random_pick = 랜덤.choice; ",
        "shuffle = 랜덤.shuffle; 랜덤정수 = 랜덤.randint; ",
        "랜덤선택 = 랜덤.choice; 섞기 = 랜덤.shuffle; ",
        "random_version = 랜덤버전 = \"0.0.1\"\n",
    );
    assert_eq!(ok("use random version 0.0.1\n"), tools);
    assert_eq!(ok("랜덤 사용 버전 0.0.1\n"), tools);
}

#[test]
fn natural_random_choice_accepts_words_instead_of_a_list_literal() {
    assert_eq!(
        ok("색은 빨강 또는 초록 또는 파랑 중에서 랜덤선택\n색 말해줘\n"),
        concat!(
            "색 = __import__(\"random\").choice((\"빨강\", \"초록\", \"파랑\",))\n",
            "print(색)\n",
        )
    );
}

#[test]
fn times_block() {
    let source = "5 times:\n    say \"Hello\"\n";
    let expected = "for _ in range(5):\n    print(\"Hello\")\n";
    assert_eq!(ok(source), expected);
}

#[test]
fn times_block_with_expression_count() {
    let source = "(2 + 3) times:\n    say \"hi\"\n";
    let expected = "for _ in range((2 + 3)):\n    print(\"hi\")\n";
    assert_eq!(ok(source), expected);
}

#[test]
fn times_inline_nme_body() {
    assert_eq!(
        ok("5 times: say \"Hello\"\n"),
        "for _ in range(5): print(\"Hello\")\n"
    );
}

#[test]
fn times_inline_python_body() {
    assert_eq!(
        ok("3 times: print('hey')\n"),
        "for _ in range(3): print('hey')\n"
    );
}

#[test]
fn times_nested_blocks() {
    let source = "3 times:\n    2 times:\n        say \"hi\"\n";
    let expected = "for _ in range(3):\n    for _ in range(2):\n        print(\"hi\")\n";
    assert_eq!(ok(source), expected);
}

#[test]
fn times_nested_inline() {
    let source = "2 times: 3 times: say \"x\"\n";
    let expected = "for _ in range(2): for _ in range(3): print(\"x\")\n";
    assert_eq!(ok(source), expected);
}

#[test]
fn korean_times_supports_attached_and_spaced_spellings() {
    assert_eq!(
        ok("3번:\n    말해 \"안녕\"\n"),
        "for _ in range(3):\n    print(\"안녕\")\n"
    );
    assert_eq!(
        ok("2 번: say \"mixed\"\n"),
        "for _ in range(2): print(\"mixed\")\n"
    );
}

#[test]
fn repeat_colon_headers_can_use_end_without_indentation() {
    assert_eq!(
        ok("3번:\n말해 hi\n끝\n"),
        "for _ in range(3):\n    print(\"hi\")\n# end\n"
    );
    assert_eq!(
        ok("repeat 3 times:\nshow hi\nend\n"),
        "for _ in range(3):\n    print(\"hi\")\n# end\n"
    );
    assert_eq!(
        ok("3 times:\nsay \"nme\"\nprint(\"python\")\n끝\n"),
        "for _ in range(3):\n    print(\"nme\")\n    print(\"python\")\n# end\n"
    );
}

#[test]
fn repeat_prefix_colon_header_accepts_a_beginner_count() {
    assert_eq!(
        ok("repeat 3 times: say \"hi\"\n"),
        "for _ in range(3): print(\"hi\")\n"
    );
}

#[test]
fn when_supports_blocks_inline_bodies_and_both_languages() {
    let source = "ready = True\nwhen ready:\n    say \"go\"\n만약 ready: 말해 \"시작\"\n";
    let expected = "ready = True\nif (ready):\n    print(\"go\")\nif (ready): print(\"시작\")\n";
    assert_eq!(ok(source), expected);
}

#[test]
fn when_keeps_complex_python_expressions_safe() {
    assert_eq!(
        ok("when lambda value: value: say \"callable\"\n"),
        "if (lambda value: value): print(\"callable\")\n"
    );
}

#[test]
fn random_tools_are_ready_after_one_easy_line() {
    let tools = concat!(
        "import random as 랜덤; random = 랜덤; ",
        "random_number = 랜덤.randint; random_pick = 랜덤.choice; ",
        "shuffle = 랜덤.shuffle; 랜덤정수 = 랜덤.randint; ",
        "랜덤선택 = 랜덤.choice; 섞기 = 랜덤.shuffle; ",
        "random_version = 랜덤버전 = \"0.0.1\"\n",
    );
    assert_eq!(
        ok("use random\nsay random_number(1, 6)\n"),
        format!("{tools}print(random_number(1, 6))\n")
    );
    assert_eq!(
        ok("랜덤 사용\n말해 랜덤선택([\"봄\", \"여름\"])\n"),
        format!("{tools}print(랜덤선택([\"봄\", \"여름\"]))\n")
    );
    assert_eq!(ok("랜덤 사용 최신\n"), tools);
    assert_eq!(ok("use latest random\n"), tools);
    assert_eq!(ok("use random version \"0.0.1\"\n"), tools);
    assert_eq!(ok("랜덤 사용 버전 \"0.0.1\"\n"), tools);
}

#[test]
fn file_tools_are_ready_after_one_easy_line() {
    let tools = concat!(
        "import pathlib as 파일경로; ",
        "file_read = lambda 경로: 파일경로.Path(경로).read_text(); ",
        "file_write = lambda 경로, 내용: 파일경로.Path(경로).write_text(내용); ",
        "json_load = lambda 경로: __import__(\"json\").loads(파일경로.Path(경로).read_text()); ",
        "json_save = lambda 경로, 값: 파일경로.Path(경로).write_text(__import__(\"json\").dumps(값, ensure_ascii=False)); ",
        "파일읽기 = file_read; ",
        "파일쓰기 = file_write; ",
        "json읽기 = json_load; ",
        "json저장 = json_save; ",
        "file_version = 파일버전 = \"0.0.1\"\n",
    );
    assert_eq!(
        ok("use file\nshow file_read(\"notes.txt\")\n"),
        format!("{tools}print(file_read(\"notes.txt\"))\n")
    );
    assert_eq!(
        ok("파일 사용\n말해 파일쓰기(\"out.txt\", \"안녕\")\n"),
        format!("{tools}print(파일쓰기(\"out.txt\", \"안녕\"))\n")
    );
    assert_eq!(ok("파일 사용 최신\n"), tools);
    assert_eq!(ok("use latest file\n"), tools);
    assert_eq!(ok("use file version \"0.0.1\"\n"), tools);
    assert_eq!(ok("파일 사용 버전 \"0.0.1\"\n"), tools);
}

#[test]
fn both_modules_can_be_loaded_in_one_program() {
    let source = "use random\nuse file\nshow random_number(1, 6)\nshow file_read(\"x.txt\")\n";
    let python = ok(source);
    assert!(python.contains("random_number = 랜덤.randint"), "{python}");
    assert!(python.contains("file_read = lambda 경로"), "{python}");
    assert!(python.contains("print(random_number(1, 6))"), "{python}");
    assert!(python.contains("print(file_read(\"x.txt\"))"), "{python}");

    // Order and language may mix: file first, then the Korean random
    // spelling, both still ready in one program.
    let mixed =
        "파일 사용 최신\nuse random latest\nshow 랜덤정수(1, 6)\nshow json읽기(\"x.json\")\n";
    let python = ok(mixed);
    assert!(python.contains("랜덤정수 = 랜덤.randint"), "{python}");
    assert!(python.contains("json읽기 = json_load"), "{python}");
    assert!(python.contains("print(랜덤정수(1, 6))"), "{python}");
    assert!(python.contains("print(json읽기(\"x.json\"))"), "{python}");
}

#[test]
fn sentence_file_read_and_write_lower_to_pathlib_lines() {
    let read_line = "memo = __import__(\"pathlib\").Path(\"notes.txt\").read_text()\n";
    assert_eq!(ok("read \"notes.txt\" into memo\n"), read_line);
    assert_eq!(ok("memo read \"notes.txt\"\n"), read_line);
    assert_eq!(ok("memo에 \"notes.txt\" 읽어서\n"), read_line);
    assert_eq!(ok("memo에 \"notes.txt\" 읽어서 저장해\n"), read_line);
    assert_eq!(ok("memo는 \"notes.txt\" 읽고\n"), read_line);

    assert_eq!(
        ok("write \"saved\" to \"out.txt\"\n"),
        "__import__(\"pathlib\").Path(\"out.txt\").write_text(\"saved\")\n"
    );
    assert_eq!(
        ok("\"out.txt\" 파일에 \"저장\"를 저장해\n"),
        "__import__(\"pathlib\").Path(\"out.txt\").write_text(\"저장\")\n"
    );
    assert_eq!(
        ok("\"out.txt\" 파일에 점수를 저장해\n"),
        "__import__(\"pathlib\").Path(\"out.txt\").write_text(점수)\n"
    );
}

#[test]
fn module_imports_lower_to_python_and_report_their_interface() {
    let python = ok("from \"helper.nme\" import greet, score\nshow greet\n");
    assert_eq!(python, "from helper import greet, score\nprint(greet)\n");

    let (source, imports) =
        nme_core::transpile_with_modules("from \"helper.nme\" import greet, score\n").unwrap();
    assert_eq!(source, "from helper import greet, score\n");
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0].file, "helper.nme");
    assert_eq!(
        imports[0].names,
        vec!["greet".to_string(), "score".to_string()]
    );

    let korean = ok("from \"util.nme\" import 안녕\nshow 안녕\n");
    assert_eq!(korean, "from util import 안녕\nprint(안녕)\n");
}

#[test]
fn natural_language_or_equal_conditions_lower_to_cmp_operators() {
    assert_eq!(
        ok("if x is less than or equal to 3\n    show \"yes\"\nend\n"),
        "if (x <= 3):\n    print(\"yes\")\n# end\n"
    );
    assert_eq!(
        ok("if x is greater than or equal to 5\n    show \"yes\"\nend\n"),
        "if (x >= 5):\n    print(\"yes\")\n# end\n"
    );
    assert_eq!(
        ok("if x is less than or equal to 3 and x is greater than or equal to 1\n    show \"in\"\nend\n"),
        "if ((x <= 3 and x >= 1)):\n    print(\"in\")\n# end\n"
    );
}

#[test]
fn korean_or_equal_conditions_lower_to_cmp_operators() {
    assert_eq!(
        ok("만약에 점수가 10보다 작거나 같으면\n    말해 \"작거나 같음\"\n끝\n"),
        "if (점수 <= 10):\n    print(\"작거나 같음\")\n# end\n"
    );
    assert_eq!(
        ok("만약에 점수가 10보다 크거나 같으면\n    말해 \"크거나 같음\"\n끝\n"),
        "if (점수 >= 10):\n    print(\"크거나 같음\")\n# end\n"
    );
}

#[test]
fn a_python_from_import_stays_byte_identical() {
    let python = ok("from helper import greet\nshow greet\n");
    assert_eq!(python, "from helper import greet\nprint(greet)\n");
}

#[test]
fn prose_with_read_or_write_words_stays_sentence_output() {
    assert_eq!(ok("write hello\n"), "print(\"write hello\")\n");
    assert_eq!(ok("read the book\n"), "print(\"read the book\")\n");
    assert_eq!(
        ok("오늘 책을 읽고 싶어\n"),
        "print(\"오늘 책을 읽고 싶어\")\n"
    );
}

#[test]
fn a_program_can_use_korean_vocabulary_and_identifiers() {
    let source = r#"랜덤 사용
후보 = ["고양이", "강아지"]
이름 = "친구"

2번:
    선택 = 랜덤선택(후보)
    만약 이름:
        말해 f"{이름}에게 {선택} 추천"
"#;
    let expected = r#"import random as 랜덤; random = 랜덤; random_number = 랜덤.randint; random_pick = 랜덤.choice; shuffle = 랜덤.shuffle; 랜덤정수 = 랜덤.randint; 랜덤선택 = 랜덤.choice; 섞기 = 랜덤.shuffle; random_version = 랜덤버전 = "0.0.1"
후보 = ["고양이", "강아지"]
이름 = "친구"

for _ in range(2):
    선택 = 랜덤선택(후보)
    if (이름):
        print(f"{이름}에게 {선택} 추천")
"#;
    assert_eq!(ok(source), expected);
}

// ------------------------------------------------------------------ mixing

#[test]
fn python_and_nme_share_a_file() {
    let source = r#"def greet(name):
    say f"Hello, {name}!"   # NME inside a Python function

for i in range(3):
    greet(i)

2 times:
    say "easy"
    print("and python")     # Python inside an NME block
"#;
    let expected = r#"def greet(name):
    print(f"Hello, {name}!")   # NME inside a Python function

for i in range(3):
    greet(i)

for _ in range(2):
    print("easy")
    print("and python")     # Python inside an NME block
"#;
    assert_eq!(ok(source), expected);
}

#[test]
fn output_has_exactly_as_many_lines_as_input() {
    let source = "x = 1\n\n# a comment\n5 times:\n    say \"hi\"\nsay f\"{x}\"\n";
    let output = ok(source);
    assert_eq!(output.lines().count(), source.lines().count());
}

// ------------------------------------------- strings & comments are sacred

#[test]
fn nme_looking_text_in_strings_is_untouched() {
    let source = "text = \"5 times: say hi\"\nx = 'say what?'\n";
    assert_eq!(ok(source), source);
}

#[test]
fn nme_looking_text_in_triple_quoted_strings_is_untouched() {
    let source = "doc = \"\"\"\n5 times:\n    say \"not code\"\n\"\"\"\n";
    assert_eq!(ok(source), source);
}

#[test]
fn nme_looking_text_in_comments_is_untouched() {
    let source = "# 5 times: say \"hi\"\nx = 1  # say something\n";
    assert_eq!(ok(source), source);
}

#[test]
fn nme_looking_text_in_fstring_is_untouched() {
    let source = "print(f\"{5 if x else 2}\")\n";
    assert_eq!(ok(source), source);
}

#[test]
fn trailing_comments_survive_nme_lines() {
    assert_eq!(
        ok("say \"hi\"  # greeting\n"),
        "print(\"hi\")  # greeting\n"
    );
    assert_eq!(
        ok("5 times:  # repeat!\n    say \"hi\"\n"),
        "for _ in range(5):  # repeat!\n    print(\"hi\")\n"
    );
    assert_eq!(
        ok("랜덤 사용  # tools\n물어봐 이름, \"이름? \"  # input\n만약 이름: 말해 이름  # condition\n"),
        concat!(
            "import random as 랜덤; random = 랜덤; random_number = 랜덤.randint; ",
            "random_pick = 랜덤.choice; shuffle = 랜덤.shuffle; ",
            "랜덤정수 = 랜덤.randint; 랜덤선택 = 랜덤.choice; 섞기 = 랜덤.shuffle; ",
            "random_version = 랜덤버전 = \"0.0.1\"  # tools\n",
            "이름 = input(\"이름? \")  # input\n",
            "if (이름): print(이름)  # condition\n",
        )
    );
}

#[test]
fn crlf_line_endings_are_preserved() {
    let source = "say \"crlf\"\r\n5 times:\r\n    say \"x\"\r\n";
    let expected = "print(\"crlf\")\r\nfor _ in range(5):\r\n    print(\"x\")\r\n";
    assert_eq!(ok(source), expected);
}

#[test]
fn missing_trailing_newline_is_fine() {
    assert_eq!(ok("say \"hi\""), "print(\"hi\")");
}

#[test]
fn sentence_while_break_and_end_do_not_need_indentation() {
    let source = concat!(
        "score = 0\n",
        "while score < 3\n",
        "show score\n",
        "add 1 to score\n",
        "if score == 3\n",
        "break\n",
        "end\n",
        "end\n",
    );
    let expected = concat!(
        "score = 0\n",
        "while (score < 3):\n",
        "    print(score)\n",
        "    score = score + 1\n",
        "    if (score == 3):\n",
        "        break\n",
        "    # end\n",
        "# end\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn sentence_logical_conditions_and_branches_are_python_shaped() {
    let source = concat!(
        "ready = True\n",
        "score = 3\n",
        "만약 ready 그리고 score > 2\n",
        "말해 yes\n",
        "아니면 만약 score == 0\n",
        "말해 zero\n",
        "아니면\n",
        "말해 no\n",
        "끝\n",
    );
    let expected = concat!(
        "ready = True\n",
        "score = 3\n",
        "if ((ready and score > 2)):\n",
        "    print(\"yes\")\n",
        "elif (score == 0):\n",
        "    print(\"zero\")\n",
        "else:\n",
        "    print(\"no\")\n",
        "# end\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn all_three_levels_and_both_languages_share_a_flat_control_block() {
    let source = concat!(
        "점수 = 0\n",
        "점수가 3보다 작을 동안\n",
        "    말해 점수\n",
        "add 1 to score\n",
        "만약 점수 그리고 score == 2\n",
        "show middle\n",
        "아니면만약 score == 3\n",
        "말해 done\n",
        "아니면\n",
        "말해 other\n",
        "끝\n",
        "멈춰\n",
        "끝\n",
    );
    let expected = concat!(
        "점수 = 0\n",
        "while (점수 < 3):\n",
        "    print(점수)\n",
        "    score = score + 1\n",
        "    if ((점수 and score == 2)):\n",
        "        print(\"middle\")\n",
        "    elif (score == 3):\n",
        "        print(\"done\")\n",
        "    else:\n",
        "        print(\"other\")\n",
        "    # end\n",
        "    break\n",
        "# end\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn korean_beginner_spellings_are_compact_and_python_mixable() {
    let source = concat!(
        "값 = 1\n",
        "3번: 말해 값\n",
        "물어봐 이름\n",
        "만약 이름:\n",
        "    말해 f\"안녕 {이름}\"\n",
        "동안 값 < 3\n",
        "    값 = 값 + 1\n",
        "    멈춰\n",
        "끝\n",
    );
    let expected = concat!(
        "값 = 1\n",
        "for _ in range(3): print(값)\n",
        "이름 = input()\n",
        "if (이름):\n",
        "    print(f\"안녕 {이름}\")\n",
        "while (값 < 3):\n",
        "    값 = 값 + 1\n",
        "    break\n",
        "# end\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn korean_beginner_save_words_are_explicit_and_mixable() {
    let source = concat!(
        "저장 인사 안녕하세요\n",
        "설정 점수에 3\n",
        "인사 말해줘\n",
        "점수 말해줘\n",
    );
    let expected = concat!(
        "인사 = \"안녕하세요\"\n",
        "점수 = 3\n",
        "print(인사)\n",
        "print(점수)\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn korean_save_word_accepts_boolean_literals() {
    assert_eq!(
        ok("저장 준비 참\n저장 준비 거짓\n"),
        "준비 = True\n준비 = False\n"
    );
}

#[test]
fn spoken_target_first_save_is_a_small_bridge_to_python_assignment() {
    let source = concat!("이름 저장 민수\n", "name save Mina\n");
    assert_eq!(
        ok(source),
        concat!("이름 = \"민수\"\n", "name = \"Mina\"\n")
    );
}

#[test]
fn attached_korean_condition_endings_work_without_spaces() {
    let source = concat!(
        "이름 = \"Ada\"\n",
        "만약에 이름있으면\n",
        "말해 hello\n",
        "아니면만약에 이름없으면\n",
        "말해 missing\n",
        "아니면\n",
        "말해 other\n",
        "끝\n",
    );
    let expected = concat!(
        "이름 = \"Ada\"\n",
        "if (이름):\n",
        "    print(\"hello\")\n",
        "elif (not (이름)):\n",
        "    print(\"missing\")\n",
        "else:\n",
        "    print(\"other\")\n",
        "# end\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn korean_while_sentence_uses_an_explicit_end() {
    let source = concat!(
        "점수는 0\n",
        "점수가 3보다 작을 동안\n",
        "점수 말해줘\n",
        "점수에 1 더해\n",
        "끝\n",
    );
    let expected = concat!(
        "점수 = 0\n",
        "while (점수 < 3):\n",
        "    print(점수)\n",
        "    점수 = 점수 + 1\n",
        "# end\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn english_while_keyword_accepts_a_korean_condition_ending() {
    // The English `while` keyword must not let a trailing Korean `동안`
    // become the loop's inline body.
    let source = concat!(
        "점수는 0\n",
        "while 점수가 3보다 작을 동안\n",
        "점수 말해줘\n",
        "점수에 1 더해\n",
        "끝\n",
    );
    let expected = concat!(
        "점수 = 0\n",
        "while (점수 < 3):\n",
        "    print(점수)\n",
        "    점수 = 점수 + 1\n",
        "# end\n",
    );
    assert_eq!(ok(source), expected);
    assert_eq!(
        ok("while playing 동안 성공 말해줘\n"),
        "while (playing): print(\"성공\")\n"
    );
}

#[test]
fn parenthesized_korean_sentence_while_endings_keep_the_condition_shape() {
    let source = concat!(
        "준비는 참\n",
        "횟수는 0\n",
        "동안 (준비 그리고 횟수가 2보다 작을 동안)\n",
        "횟수에 1 더해\n",
        "끝\n",
    );
    let expected = concat!(
        "준비 = True\n",
        "횟수 = 0\n",
        "while ((준비 and 횟수 < 2)):\n",
        "    횟수 = 횟수 + 1\n",
        "# end\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn korean_negation_connectors_lower_to_not_equals() {
    let source = concat!(
        "만약 점수가 5와 같지 않으면\n",
        "달라요 말해줘\n",
        "끝\n",
        "if 점수가 5와 같지 않으면\n",
        "달라요 말해줘\n",
        "끝\n",
        "만약 점수가 5와 같지않으면 말해 \"단어\"\n",
    );
    let expected = concat!(
        "if (not (점수 == 5)):\n",
        "    print(\"달라요\")\n",
        "# end\n",
        "if (not (점수 == 5)):\n",
        "    print(\"달라요\")\n",
        "# end\n",
        "if (not (점수 == 5)): print(\"단어\")\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn korean_negation_works_inside_a_while_ending() {
    let source = concat!(
        "점수는 0\n",
        "점수가 5와 같지 않을 동안\n",
        "점수에 1 더해\n",
        "끝\n",
    );
    let expected = concat!(
        "점수 = 0\n",
        "while (not (점수 == 5)):\n",
        "    점수 = 점수 + 1\n",
        "# end\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn korean_comparison_endings_combine_with_logical_connectors() {
    let source = concat!(
        "만약 점수가 0보다 크면 그리고 점수가 3보다 작으면\n",
        "말해 \"사이\"\n",
        "끝\n",
        "만약에 점수가 5와 같지 않으면 그리고 점수가 0보다 크면\n",
        "말해 \"둘 다\"\n",
        "끝\n",
        "만약 준비가 거짓이면 그리고 점수가 0보다 크면\n",
        "말해 \"셋\"\n",
        "끝\n",
    );
    let expected = concat!(
        "if ((점수 > 0 and 점수 < 3)):\n",
        "    print(\"사이\")\n",
        "# end\n",
        "if ((not (점수 == 5) and 점수 > 0)):\n",
        "    print(\"둘 다\")\n",
        "# end\n",
        "if ((준비 == False and 점수 > 0)):\n",
        "    print(\"셋\")\n",
        "# end\n",
    );
    assert_eq!(ok(source), expected);
    // An English `then` body may contain logical words without changing the
    // condition parse.
    assert_eq!(ok("if a then show x or y\n"), "if (a): print(\"x or y\")\n");
}

#[test]
fn korean_logical_conditions_never_panic() {
    // Two negation pairs in one condition used to cut at the first ending and
    // then panic on an empty logical operand.
    let source = concat!(
        "만약 점수가 5와 같지 않으면 그리고 점수가 5와 같지 않다면\n",
        "말해 \"y\"\n",
        "끝\n",
    );
    let expected = concat!(
        "if ((not (점수 == 5) and not (점수 == 5))):\n",
        "    print(\"y\")\n",
        "# end\n",
    );
    assert_eq!(ok(source), expected);
    // A `동안` ending on every logical operand is the same single loop.
    let source = concat!(
        "점수는 0\n",
        "점수가 5와 같지 않을 동안 그리고 점수가 0보다 클 동안\n",
        "점수에 1 더해\n",
        "끝\n",
    );
    let expected = concat!(
        "점수 = 0\n",
        "while ((not (점수 == 5) and 점수 > 0)):\n",
        "    점수 = 점수 + 1\n",
        "# end\n",
    );
    assert_eq!(ok(source), expected);
    assert_eq!(
        ok("while 점수가 5와 같지 않을 동안 그리고 점수가 0보다 클 동안 성공 말해줘\n"),
        "while ((not (점수 == 5) and 점수 > 0)): print(\"성공\")\n"
    );
}

#[test]
fn attached_korean_while_endings_are_easy_to_say() {
    assert_eq!(
        ok("준비하는동안 성공 말해줘\n"),
        "while (준비): print(\"성공\")\n"
    );
    assert_eq!(
        ok("준비 하는 동안 성공 말해줘\n"),
        "while (준비): print(\"성공\")\n"
    );
    assert_eq!(
        ok("준비 동안 성공 말해줘\n"),
        "while (준비): print(\"성공\")\n"
    );
}

#[test]
fn korean_break_alias_is_lowered_inside_an_inline_condition() {
    let source = concat!(
        "점수는 0\n",
        "동안 점수가 3보다 작으면\n",
        "점수에 1 더해\n",
        "점수가 2보다 크면 멈춰\n",
        "끝\n",
    );
    let expected = concat!(
        "점수 = 0\n",
        "while (점수 < 3):\n",
        "    점수 = 점수 + 1\n",
        "    if (점수 > 2): break\n",
        "# end\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn attached_korean_else_if_is_supported() {
    let source = concat!(
        "만약 준비\n",
        "말해 one\n",
        "아니면만약 다른준비\n",
        "말해 two\n",
        "아니면\n",
        "말해 three\n",
        "끝\n",
    );
    let expected = concat!(
        "if (준비):\n",
        "    print(\"one\")\n",
        "elif (다른준비):\n",
        "    print(\"two\")\n",
        "else:\n",
        "    print(\"three\")\n",
        "# end\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn attached_korean_repeat_can_use_end_without_indentation() {
    assert_eq!(
        ok("3번\n말해 hi\n끝\n"),
        "for _ in range(3):\n    print(\"hi\")\n# end\n"
    );
}

#[test]
fn korean_hamyeon_connector_can_open_a_flat_block() {
    assert_eq!(
        ok("준비는 참\n만약 준비 하면\n성공 말해줘\n끝\n"),
        "준비 = True\nif (준비):\n    print(\"성공\")\n# end\n"
    );
}

#[test]
fn explicit_block_names_do_not_leak_after_end() {
    let source = concat!(
        "ready = True\n",
        "if ready\n",
        "set secret to hidden\n",
        "show value secret\n",
        "end\n",
        "show secret\n",
    );
    let expected = concat!(
        "ready = True\n",
        "if (ready):\n",
        "    secret = \"hidden\"\n",
        "    print(\"value \" + str(secret))\n",
        "# end\n",
        "print(\"secret\")\n",
    );
    assert_eq!(ok(source), expected);
}

#[test]
fn explicit_blocks_do_not_double_indent_a_body_already_indented() {
    assert_eq!(
        ok("if ready\n    show yes\nend\n"),
        "if (ready):\n    print(\"yes\")\n# end\n"
    );
}

#[test]
fn nested_flat_blocks_keep_python_nesting_and_line_count() {
    let source = "while outer\nif inner\nshow value\nend\nend\n";
    let output = ok(source);
    assert_eq!(
        output,
        "while (outer):\n    if (inner):\n        print(\"value\")\n    # end\n# end\n"
    );
    assert_eq!(output.lines().count(), source.lines().count());
}

#[test]
fn flat_nme_blocks_virtual_indent_an_ordinary_python_suite() {
    let source = concat!(
        "while True\n",
        "if x:\n",
        "    print(x)\n",
        "break\n",
        "end\n",
    );
    let expected = concat!(
        "while (True):\n",
        "    if x:\n",
        "        print(x)\n",
        "    break\n",
        "# end\n",
    );
    assert_eq!(ok(source), expected);
}
