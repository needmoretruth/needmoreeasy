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
    assert_eq!(
        ok("use random\nsay random_number(1, 6)\n"),
        concat!(
            "import random; random_number = random.randint; ",
            "random_pick = random.choice; shuffle = random.shuffle\n",
            "print(random_number(1, 6))\n",
        )
    );
    assert_eq!(
        ok("랜덤 사용\n말해 랜덤선택([\"봄\", \"여름\"])\n"),
        concat!(
            "import random as 랜덤; 랜덤정수 = 랜덤.randint; ",
            "랜덤선택 = 랜덤.choice; 섞기 = 랜덤.shuffle\n",
            "print(랜덤선택([\"봄\", \"여름\"]))\n",
        )
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
    let expected = r#"import random as 랜덤; 랜덤정수 = 랜덤.randint; 랜덤선택 = 랜덤.choice; 섞기 = 랜덤.shuffle
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
            "import random as 랜덤; 랜덤정수 = 랜덤.randint; ",
            "랜덤선택 = 랜덤.choice; 섞기 = 랜덤.shuffle  # tools\n",
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
