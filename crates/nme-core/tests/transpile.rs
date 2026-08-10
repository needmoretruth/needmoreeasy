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
        "    print(\"성공이라고\")\n",
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
    );
    let expected = concat!(
        "이름 = input(\"이름이 뭐예요\" + \" \")\n",
        "print(\"안녕하세요 \" + str(이름))\n",
        "for _ in range(2): print(\"다시\")\n",
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
