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
