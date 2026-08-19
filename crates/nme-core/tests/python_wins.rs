//! The golden rule of NME: **valid Python always wins** over look-alike
//! NME syntax. These tests prove that code a Python programmer could
//! plausibly write is never hijacked by NME.

use nme_core::transpile;

fn unchanged(source: &str) {
    assert_eq!(
        transpile(source).as_deref(),
        Ok(source),
        "valid Python must come out byte-identical"
    );
}

#[test]
fn say_as_a_function_call() {
    unchanged("say(\"hi\")\n");
    unchanged("say(1 + 1)\n");
}

#[test]
fn say_as_an_attribute_or_subscript() {
    unchanged("say.x = 1\n");
    unchanged("say[0] = 'a'\n");
    unchanged("print(say.text)\n");
}

#[test]
fn say_as_a_variable() {
    unchanged("say = print\nsay(\"hi\")\n");
    unchanged("say = 5\nprint(say + 1)\n");
}

#[test]
fn a_bare_name_wins_for_python_once_the_program_has_made_it() {
    // A bare name expression is valid Python, and while the program has made
    // that name it stays untouched — however it was made.
    unchanged("say = print\nsay\n");
    unchanged("import say\nsay\n");
    unchanged("from mod import say\nsay\n");
    unchanged("def say():\n    pass\nsay\n");
    // A name nothing ever made is a different thing. It used to come out as
    // itself and raise `NameError` at run time, on a line the writer had read
    // as a command; since 2026-08-19 such a line is read as NME instead, and
    // an action word alone is told what it is missing.
    assert_eq!(transpile("say\n").unwrap_err()[0].code.code(), "E0204");
}

#[test]
fn times_as_a_variable() {
    unchanged("times = 5\nprint(times)\n");
    unchanged("times = [1, 2]\ntimes.append(3)\n");
}

#[test]
fn times_in_python_compound_headers() {
    // `if times:` only parses as Python when followed by a body; the
    // parser must try that form too before claiming a line for NME.
    unchanged("times = 3\nif times:\n    print(times)\n");
    unchanged("times = 3\nwhile times:\n    times -= 1\n");
}

#[test]
fn times_in_match_case() {
    unchanged("match command:\n    case times:\n        print(times)\n");
}

#[test]
fn times_inside_brackets_never_matches() {
    unchanged("x = data[times:]\n");
    unchanged("d = {times: 3}\n");
    unchanged("f(times=3)\n");
}

#[test]
fn times_lambda_is_python() {
    unchanged("f = lambda times: times * 2\n");
}

#[test]
fn annotated_names_are_untouched() {
    unchanged("times: int = 5\n");
    unchanged("say: str = 'x'\n");
}

#[test]
fn new_english_spellings_are_still_ordinary_python_names() {
    unchanged("ask = input\nask('name?')\n");
    unchanged("when = True\nprint(when)\n");
    unchanged("use = lambda value: value\nuse(random)\n");
    unchanged("show = print\nshow('hello')\n");
    unchanged("repeat = 3\nset = {'answer': 7}\nprint(repeat, set)\n");
    unchanged("add = 1\nincrease = add + 1\nprint(increase)\n");
}

#[test]
fn when_calls_with_parenthesized_conditions_are_python() {
    unchanged("when (ready and score > 2)\n");
    unchanged("when(ready and score > 2)\n");
}

#[test]
fn korean_spellings_are_still_ordinary_python_names() {
    unchanged("말해 = print\n말해('안녕')\n");
    unchanged("물어봐 = input\n물어봐('이름?')\n");
    unchanged("번 = 3\n만약 = True\nprint(번, 만약)\n");
    unchanged("랜덤 = object()\n사용 = 랜덤\n");
    unchanged("아니면 = True\nprint(아니면)\n");
    unchanged("아니면.foo\n");
    unchanged("아니면 + 1\n");
    unchanged("멈춰 = 1\n멈춰\n");
    // Alone, with no loop above it, `멈춰` is the writer asking to leave a
    // loop that is not there — which is what they are told.
    assert_eq!(transpile("멈춰\n").unwrap_err()[0].code.code(), "E0102");
    unchanged("보여줘 = print\n보여줘('안녕')\n");
    unchanged("반복해 = 3\n설정해 = {'정답': 7}\nprint(반복해, 설정해)\n");
}

#[test]
fn korean_condition_word_call_with_parentheses_is_python() {
    unchanged("만약 = lambda value: value\n준비 = True\n만약 (준비)\n");
}

#[test]
fn future_python_call_shapes_are_left_for_the_selected_cpython() {
    // CPython 3.14 accepts template strings. rustpython-parser 0.4 does not
    // know that grammar yet, so the invocation shape is the compatibility
    // boundary that prevents NME from hijacking this as `say` syntax.
    unchanged("say = print\nsay(t\"hello\")\n");
    unchanged("say = print; say(t\"hello\")\n");
}

#[test]
fn control_words_in_valid_python_keep_python_priority() {
    unchanged("end = 1\nprint(end)\n");
    unchanged("end = 1\nend\n");
    // `end` and `끝` with nothing above them close no block and name nothing,
    // so they say themselves rather than becoming a `NameError`.
    assert_eq!(transpile("end\n").as_deref(), Ok("print(\"end\")\n"));
    assert_eq!(transpile("끝\n").as_deref(), Ok("print(\"끝\")\n"));
    unchanged("obj.end = 1\n");
    unchanged("breakpoint()\n");
    unchanged("while True:\n    break\n");
    unchanged("if ready:\n    else_value = 1\n");
}
