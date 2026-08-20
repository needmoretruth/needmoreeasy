//! Broken input must produce friendly, beginner-oriented diagnostics —
//! never silently broken Python output.

use nme_core::diagnostics::{render_all, render_all_bilingual, DiagnosticCode};
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

/// The code of the single diagnostic a broken source produces.
fn error_code(source: &str) -> String {
    let problems = transpile(source).expect_err("expected an error");
    problems[0].code.code().to_string()
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
    // The message quotes the half-finished sum rather than saying only that
    // something could not be understood.
    assert!(
        message.contains("`1 +` is a sum with a piece missing"),
        "{message}"
    );
}

#[test]
fn inline_block_cannot_open_a_block() {
    let message = err("2 times: 3 times:\n    say \"x\"\n");
    assert!(message.contains("nothing to do in it"), "{message}");
}

#[test]
fn inline_body_allows_only_one_statement() {
    let message = err("5 times: say \"a\"; say \"b\"\n");
    assert!(message.contains("only one thing to do"), "{message}");
}

#[test]
fn unterminated_string_is_reported_gently() {
    let message = err("say \"oops\n");
    assert!(message.contains("never closed"), "{message}");
}

#[test]
fn all_problems_are_reported_at_once() {
    let problems = transpile("5 times:\nsay 1 +\n").unwrap_err();
    assert_eq!(problems.len(), 2, "{problems:?}");
}

#[test]
fn diagnostics_render_with_location_code_and_hint() {
    let source = "say \"ok\"\nsay 1 +\n";
    let problems = transpile(source).unwrap_err();
    let rendered = render_all(&problems, source, "hello.nme");
    assert!(rendered.contains("error[E0201]:"), "{rendered}");
    // The caret points at the offending expression on line 2.
    assert!(rendered.contains("hello.nme:2:5"), "{rendered}");
    assert!(rendered.contains("say 1 +"), "{rendered}");
    assert!(rendered.contains("hint:"), "{rendered}");
}

#[test]
fn ask_requires_a_simple_target() {
    let target = err("ask 123, \"Number? \"\n");
    assert!(
        target.contains("where to put what the person types"),
        "{target}"
    );
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
    assert!(colon.contains("nothing follows this condition"), "{colon}");

    let condition = err("when:\n    say \"no\"\n");
    assert!(condition.contains("condition is missing"), "{condition}");

    let body = err("when ready:\nsay \"not indented\"\n");
    assert!(body.contains("indented"), "{body}");
}

#[test]
fn korean_forms_return_korean_guidance() {
    let say = bilingual_err("말해 1 +\n");
    assert!(say.contains("계산이 하다 만 채로 끝나서"), "{say}");
    assert!(say.contains("a sum with a piece missing"), "{say}");

    let repeat = bilingual_err("3번:\n말해 \"들여쓰기 없음\"\n");
    assert!(repeat.contains("들여쓴 줄이 없어서"), "{repeat}");
    assert!(
        repeat.contains("nothing below this line is indented"),
        "{repeat}"
    );

    let when = bilingual_err("만약 준비됨\n");
    assert!(when.contains("조건이 맞아도 할 일이 없습니다"), "{when}");
    assert!(when.contains("nothing follows this condition"), "{when}");
}

#[test]
fn inline_break_outside_a_loop_gets_the_stable_bilingual_diagnostic() {
    let cases = [
        ("sentence-en", "when true then break here\n"),
        ("sentence-ko", "만약 참 그러면 멈춰\n"),
        ("beginner-en", "if True then break\n"),
        ("beginner-ko", "만약 True 그러면 멈춰\n"),
        ("advanced-en", "if (True) then break\n"),
        ("advanced-ko", "만약 ((참 그리고 참)) 그러면 멈춰\n"),
    ];

    for (label, source) in cases {
        let problems = match transpile(source) {
            Ok(output) => panic!("expected inline break diagnostic for {label}, got {output:?}"),
            Err(problems) => problems,
        };
        assert_eq!(problems.len(), 1, "core case: {label}: {problems:?}");
        let problem = &problems[0];
        assert_eq!(
            problem.code,
            DiagnosticCode::BreakOutsideLoop,
            "core case: {label}"
        );
        assert!(
            problem.message.contains("inside a loop"),
            "{label}: {problem:?}"
        );
        assert!(
            problem
                .message_ko
                .as_deref()
                .is_some_and(|message| message.contains("반복문 안에서만")),
            "{label}: {problem:?}"
        );
        assert!(
            problem
                .hint
                .as_deref()
                .is_some_and(|hint| hint.contains("Python") && hint.contains("for")),
            "{label}: {problem:?}"
        );
        assert!(
            problem
                .hint_ko
                .as_deref()
                .is_some_and(|hint| hint.contains("Python") && hint.contains("for")),
            "{label}: {problem:?}"
        );
    }
}

#[test]
fn return_outside_a_function_gets_the_stable_bilingual_diagnostic() {
    let cases = [
        ("top-level", "return 1\n"),
        ("sentence-en", "when true then return 1\n"),
        ("sentence-ko", "만약 참 그러면 return 1\n"),
        ("beginner-en", "if True then return 1\n"),
        ("beginner-ko", "만약 True 그러면 return 1\n"),
        ("advanced-en", "if (True) then return 1\n"),
        ("advanced-ko", "만약 ((참 그리고 참)) 그러면 return 1\n"),
    ];

    for (label, source) in cases {
        let problems = match transpile(source) {
            Ok(output) => panic!("expected return diagnostic for {label}, got {output:?}"),
            Err(problems) => problems,
        };
        assert_eq!(problems.len(), 1, "core case: {label}: {problems:?}");
        let problem = &problems[0];
        assert_eq!(
            problem.code,
            DiagnosticCode::ReturnOutsideFunction,
            "core case: {label}"
        );
        assert!(
            problem.message.contains("inside a function"),
            "{label}: {problem:?}"
        );
        assert!(
            problem
                .message_ko
                .as_deref()
                .is_some_and(|message| message.contains("함수 안에서만")),
            "{label}: {problem:?}"
        );
    }
}

#[test]
fn continue_outside_a_loop_gets_a_stable_diagnostic() {
    let cases = [
        ("top-level", "continue\n"),
        ("sentence-en", "when true then continue\n"),
        ("sentence-ko", "만약 참 그러면 continue\n"),
        ("beginner-en", "if True then continue\n"),
        ("beginner-ko", "만약 True 그러면 continue\n"),
        ("advanced-en", "if (True) then continue\n"),
        ("advanced-ko", "만약 ((참 그리고 참)) 그러면 continue\n"),
    ];

    for (label, source) in cases {
        let problems = match transpile(source) {
            Ok(output) => panic!("expected continue diagnostic for {label}, got {output:?}"),
            Err(problems) => problems,
        };
        assert_eq!(problems.len(), 1, "core case: {label}: {problems:?}");
        let problem = &problems[0];
        assert_eq!(
            problem.code,
            DiagnosticCode::ContinueOutsideLoop,
            "core case: {label}"
        );
        assert!(
            problem.message.contains("inside a loop"),
            "{label}: {problem:?}"
        );
        assert!(
            problem
                .message_ko
                .as_deref()
                .is_some_and(|message| message.contains("반복문 안에서만")),
            "{label}: {problem:?}"
        );
    }
}

#[test]
fn yield_outside_a_function_gets_a_stable_bilingual_diagnostic() {
    let cases = [
        ("top-level", "yield 1\n"),
        ("sentence-en", "when true then yield 1\n"),
        ("sentence-ko", "만약 참 그러면 yield 1\n"),
        ("beginner-en", "if True then yield 1\n"),
        ("beginner-ko", "만약 True 그러면 yield 1\n"),
        ("advanced-en", "if (True) then yield 1\n"),
        ("advanced-ko", "만약 ((참 그리고 참)) 그러면 yield 1\n"),
    ];

    for (label, source) in cases {
        let problems = match transpile(source) {
            Ok(output) => panic!("expected yield diagnostic for {label}, got {output:?}"),
            Err(problems) => problems,
        };
        assert_eq!(problems.len(), 1, "core case: {label}: {problems:?}");
        let problem = &problems[0];
        assert_eq!(
            problem.code,
            DiagnosticCode::YieldOutsideFunction,
            "core case: {label}"
        );
        assert!(
            problem.message.contains("inside a function"),
            "{label}: {problem:?}"
        );
        assert!(
            problem
                .message_ko
                .as_deref()
                .is_some_and(|message| message.contains("함수 안에서만")),
            "{label}: {problem:?}"
        );
    }
}

#[test]
fn await_outside_an_async_function_gets_a_stable_bilingual_diagnostic() {
    let cases = [
        ("top-level", "await work()\n"),
        ("sentence-en", "when true then await work()\n"),
        ("sentence-ko", "만약 참 그러면 await work()\n"),
        ("beginner-en", "if True then await work()\n"),
        ("beginner-ko", "만약 True 그러면 await work()\n"),
        ("advanced-en", "if (True) then await work()\n"),
        ("advanced-ko", "만약 ((참 그리고 참)) 그러면 await work()\n"),
    ];

    for (label, source) in cases {
        let problems = match transpile(source) {
            Ok(output) => panic!("expected await diagnostic for {label}, got {output:?}"),
            Err(problems) => problems,
        };
        assert_eq!(problems.len(), 1, "core case: {label}: {problems:?}");
        let problem = &problems[0];
        assert_eq!(
            problem.code,
            DiagnosticCode::AwaitOutsideAsyncFunction,
            "core case: {label}"
        );
        assert!(
            problem.message.contains("inside an async function"),
            "{label}: {problem:?}"
        );
        assert!(
            problem
                .message_ko
                .as_deref()
                .is_some_and(|message| message.contains("비동기 함수 안에서만")),
            "{label}: {problem:?}"
        );
    }
}

#[test]
fn python_context_diagnostics_follow_nested_function_and_class_scopes() {
    let class_return = transpile("def outer():\n    class Inner:\n        return 1\n")
        .expect_err("return in a class body must not inherit the outer function scope");
    assert_eq!(class_return.len(), 1, "{class_return:?}");
    assert_eq!(class_return[0].code, DiagnosticCode::ReturnOutsideFunction);

    let inline_class_return = transpile("class Inner: return 1\n")
        .expect_err("return in an inline class body must be rejected");
    assert_eq!(inline_class_return.len(), 1, "{inline_class_return:?}");
    assert_eq!(
        inline_class_return[0].code,
        DiagnosticCode::ReturnOutsideFunction
    );
    let inline_class_return_after_statement = transpile("class Inner: value = 1; return 1\n")
        .expect_err("a return after an inline class statement must be rejected");
    assert_eq!(
        inline_class_return_after_statement[0].code,
        DiagnosticCode::ReturnOutsideFunction
    );

    let inline_function_continue = transpile("def inner(): continue\n")
        .expect_err("continue in an inline function body needs a loop");
    assert_eq!(
        inline_function_continue.len(),
        1,
        "{inline_function_continue:?}"
    );
    assert_eq!(
        inline_function_continue[0].code,
        DiagnosticCode::ContinueOutsideLoop
    );
    let inline_function_continue_with_tail = transpile("def inner(): continue; value = 1\n")
        .expect_err("a direct inline continue with a tail still needs a loop");
    assert_eq!(
        inline_function_continue_with_tail[0].code,
        DiagnosticCode::ContinueOutsideLoop
    );
    let inline_function_continue_after_statement = transpile("def inner(): value = 1; continue\n")
        .expect_err("a continue after an inline statement still needs a loop");
    assert_eq!(
        inline_function_continue_after_statement[0].code,
        DiagnosticCode::ContinueOutsideLoop
    );

    let inline_function_break = transpile("def inner(): break\n")
        .expect_err("break in an inline function body needs a loop");
    assert_eq!(inline_function_break.len(), 1, "{inline_function_break:?}");
    assert_eq!(
        inline_function_break[0].code,
        DiagnosticCode::BreakOutsideLoop
    );
    let inline_function_break_with_tail = transpile("def inner(): break; value = 1\n")
        .expect_err("a direct inline break with a tail still needs a loop");
    assert_eq!(
        inline_function_break_with_tail[0].code,
        DiagnosticCode::BreakOutsideLoop
    );
    let inline_function_break_after_statement = transpile("def inner(): value = 1; break\n")
        .expect_err("a break after an inline statement still needs a loop");
    assert_eq!(
        inline_function_break_after_statement[0].code,
        DiagnosticCode::BreakOutsideLoop
    );

    let nested_inline_continue = transpile("for item in values:\n    def inner(): continue\n")
        .expect_err("an inline function must not inherit an outer loop");
    assert_eq!(
        nested_inline_continue.len(),
        1,
        "{nested_inline_continue:?}"
    );
    assert_eq!(
        nested_inline_continue[0].code,
        DiagnosticCode::ContinueOutsideLoop
    );

    let class_yield = transpile("def outer():\n    class Inner:\n        yield 1\n")
        .expect_err("yield in a class body must not inherit the outer function scope");
    assert_eq!(class_yield.len(), 1, "{class_yield:?}");
    assert_eq!(class_yield[0].code, DiagnosticCode::YieldOutsideFunction);

    let nested_await = transpile("async def outer():\n    def inner():\n        await work()\n")
        .expect_err("a nested ordinary function is not async");
    assert_eq!(nested_await.len(), 1, "{nested_await:?}");
    assert_eq!(
        nested_await[0].code,
        DiagnosticCode::AwaitOutsideAsyncFunction
    );

    let generator = "def generator():\n    yield 1\n";
    assert_eq!(transpile(generator).unwrap(), generator);
    let async_generator = "async def generator():\n    yield 1\n";
    assert_eq!(transpile(async_generator).unwrap(), async_generator);
    let async_function = "async def worker():\n    await work()\n";
    assert_eq!(transpile(async_function).unwrap(), async_function);

    let generator_lambda = "generator = lambda: (yield 1)\n";
    assert_eq!(transpile(generator_lambda).unwrap(), generator_lambda);
    let inline_generator_lambda = "when true then (lambda: (yield 1))\n";
    assert_eq!(
        transpile(inline_generator_lambda).unwrap(),
        "if (True): (lambda: (yield 1))\n"
    );
    let class_generator_lambda = "class C:\n    generator = lambda: (yield 1)\n";
    assert_eq!(
        transpile(class_generator_lambda).unwrap(),
        class_generator_lambda
    );
    let async_generator_lambda = "async def outer():\n    worker = lambda: (yield from values)\n";
    assert_eq!(
        transpile(async_generator_lambda).unwrap(),
        async_generator_lambda
    );

    let async_lambda_await = transpile("async def outer():\n    worker = lambda: (await work())\n")
        .expect_err("await is not valid inside a normal lambda");
    assert_eq!(async_lambda_await.len(), 1, "{async_lambda_await:?}");
    assert_eq!(
        async_lambda_await[0].code,
        DiagnosticCode::AwaitOutsideAsyncFunction
    );

    let async_yield_from = transpile("async def generator():\n    yield from values\n")
        .expect_err("yield from is not valid in an async function");
    assert_eq!(async_yield_from.len(), 1, "{async_yield_from:?}");
    assert_eq!(
        async_yield_from[0].code,
        DiagnosticCode::YieldFromAsyncFunction
    );

    let inline_async_yield_from =
        transpile("async def generator():\n    if True then yield from values\n")
            .expect_err("inline yield from is not valid in an async function");
    assert_eq!(
        inline_async_yield_from.len(),
        1,
        "{inline_async_yield_from:?}"
    );
    assert_eq!(
        inline_async_yield_from[0].code,
        DiagnosticCode::YieldFromAsyncFunction
    );

    let generator_from = "def generator():\n    yield from values\n";
    assert_eq!(transpile(generator_from).unwrap(), generator_from);

    let yield_default = transpile("def f(value=(yield 1)):\n    return value\n")
        .expect_err("yield in a function default is outside the function body");
    assert_eq!(yield_default.len(), 1, "{yield_default:?}");
    assert_eq!(yield_default[0].code, DiagnosticCode::YieldOutsideFunction);
}

#[test]
fn yield_from_inside_async_functions_gets_a_stable_bilingual_diagnostic() {
    let cases = [
        (
            "sentence-en",
            "async def generator():\n    when true then yield from values\n",
        ),
        (
            "sentence-ko",
            "async def generator():\n    만약 참 그러면 yield from values\n",
        ),
        (
            "beginner-en",
            "async def generator():\n    if True then yield from values\n",
        ),
        (
            "beginner-ko",
            "async def generator():\n    만약 True 그러면 yield from values\n",
        ),
        (
            "advanced-en",
            "async def generator():\n    if (True) then yield from values\n",
        ),
        (
            "advanced-ko",
            "async def generator():\n    만약 ((참 그리고 참)) 그러면 yield from values\n",
        ),
    ];

    for (label, source) in cases {
        let problems = transpile(source).expect_err("expected yield-from diagnostic");
        assert_eq!(problems.len(), 1, "core case: {label}: {problems:?}");
        let problem = &problems[0];
        assert_eq!(
            problem.code,
            DiagnosticCode::YieldFromAsyncFunction,
            "core case: {label}"
        );
        assert!(
            problem
                .message
                .contains("cannot be used inside an async function"),
            "{label}: {problem:?}"
        );
        assert!(
            problem
                .message_ko
                .as_deref()
                .is_some_and(|message| message.contains("비동기 함수 안에서는")),
            "{label}: {problem:?}"
        );
    }
}

#[test]
fn async_for_and_with_outside_async_functions_get_stable_diagnostics() {
    let async_for_top = transpile("async for item in stream():\n    pass\n")
        .expect_err("top-level async for must be rejected");
    assert_eq!(async_for_top.len(), 1, "{async_for_top:?}");
    assert_eq!(
        async_for_top[0].code,
        DiagnosticCode::AsyncForOutsideAsyncFunction
    );
    assert!(async_for_top[0]
        .message
        .contains("inside an async function"));
    assert!(async_for_top[0]
        .message_ko
        .as_deref()
        .is_some_and(|message| message.contains("비동기 함수 안에서만")));

    let async_for_sync = transpile("def f():\n    async for item in stream():\n        pass\n")
        .expect_err("async for in a normal function must be rejected");
    assert_eq!(async_for_sync.len(), 1, "{async_for_sync:?}");
    assert_eq!(
        async_for_sync[0].code,
        DiagnosticCode::AsyncForOutsideAsyncFunction
    );

    let async_for_class = transpile(
        "async def outer():\n    class C:\n        async for item in stream():\n            pass\n",
    )
    .expect_err("async for in a class body must not inherit an outer async function");
    assert_eq!(async_for_class.len(), 1, "{async_for_class:?}");
    assert_eq!(
        async_for_class[0].code,
        DiagnosticCode::AsyncForOutsideAsyncFunction
    );

    let async_with_top = transpile("async with resource():\n    pass\n")
        .expect_err("top-level async with must be rejected");
    assert_eq!(async_with_top.len(), 1, "{async_with_top:?}");
    assert_eq!(
        async_with_top[0].code,
        DiagnosticCode::AsyncWithOutsideAsyncFunction
    );
    assert!(async_with_top[0]
        .message
        .contains("inside an async function"));
    assert!(async_with_top[0]
        .message_ko
        .as_deref()
        .is_some_and(|message| message.contains("비동기 함수 안에서만")));

    let async_with_sync = transpile("def f():\n    async with resource():\n        pass\n")
        .expect_err("async with in a normal function must be rejected");
    assert_eq!(async_with_sync.len(), 1, "{async_with_sync:?}");
    assert_eq!(
        async_with_sync[0].code,
        DiagnosticCode::AsyncWithOutsideAsyncFunction
    );

    let async_with_class = transpile(
        "async def outer():\n    class C:\n        async with resource():\n            pass\n",
    )
    .expect_err("async with in a class body must not inherit an outer async function");
    assert_eq!(async_with_class.len(), 1, "{async_with_class:?}");
    assert_eq!(
        async_with_class[0].code,
        DiagnosticCode::AsyncWithOutsideAsyncFunction
    );

    let valid = "async def worker():\n    async for item in stream():\n        async with resource(item):\n            pass\n";
    assert_eq!(transpile(valid).unwrap(), valid);
}

#[test]
fn nonlocal_without_an_enclosing_function_gets_a_stable_diagnostic() {
    let top_level = transpile("nonlocal value\n")
        .expect_err("module-level nonlocal must be rejected by the shared parser");
    assert_eq!(top_level.len(), 1, "{top_level:?}");
    assert_eq!(top_level[0].code, DiagnosticCode::NonlocalOutsideFunction);
    assert!(top_level[0].message.contains("inside a nested function"));
    assert!(top_level[0]
        .message_ko
        .as_deref()
        .is_some_and(|message| message.contains("중첩 함수 안에서만")));

    let top_level_class = transpile("class C:\n    nonlocal value\n")
        .expect_err("a top-level class has no enclosing function");
    assert_eq!(top_level_class.len(), 1, "{top_level_class:?}");
    assert_eq!(
        top_level_class[0].code,
        DiagnosticCode::NonlocalOutsideFunction
    );

    let function_without_outer = transpile("def only():\n    nonlocal value\n")
        .expect_err("a function without an outer function has no nonlocal scope");
    assert_eq!(
        function_without_outer.len(),
        1,
        "{function_without_outer:?}"
    );
    assert_eq!(
        function_without_outer[0].code,
        DiagnosticCode::NonlocalOutsideFunction
    );

    let inline_function = transpile("def only(): nonlocal value\n")
        .expect_err("an inline function without an outer function has no nonlocal scope");
    assert_eq!(inline_function.len(), 1, "{inline_function:?}");
    assert_eq!(
        inline_function[0].code,
        DiagnosticCode::NonlocalOutsideFunction
    );

    let method_without_outer =
        transpile("class C:\n    def method(self):\n        nonlocal value\n")
            .expect_err("a method in a top-level class has no enclosing function");
    assert_eq!(method_without_outer.len(), 1, "{method_without_outer:?}");
    assert_eq!(
        method_without_outer[0].code,
        DiagnosticCode::NonlocalOutsideFunction
    );

    let valid = "def outer():\n    value = 1\n    def inner():\n        nonlocal value\n        value += 1\n";
    assert_eq!(transpile(valid).unwrap(), valid);
    let valid_inline = "def outer():\n    value = 1\n    def inner(): nonlocal value; value += 1\n";
    assert_eq!(transpile(valid_inline).unwrap(), valid_inline);
    let valid_class = "def outer():\n    value = 1\n    class C:\n        nonlocal value\n";
    assert_eq!(transpile(valid_class).unwrap(), valid_class);
    let valid_inline_class = "def outer():\n    value = 1\n    class C: nonlocal value\n";
    assert_eq!(transpile(valid_inline_class).unwrap(), valid_inline_class);
    let valid_method =
        "def outer():\n    value = 1\n    class C:\n        def method(self):\n            nonlocal value\n";
    assert_eq!(transpile(valid_method).unwrap(), valid_method);
    let valid_inline_method =
        "def outer():\n    value = 1\n    class C:\n        def method(self): nonlocal value\n";
    assert_eq!(transpile(valid_inline_method).unwrap(), valid_inline_method);

    let missing_binding = "def outer():\n    def inner():\n        nonlocal value\n";
    assert_eq!(
        transpile(missing_binding).unwrap(),
        missing_binding,
        "CPython retains missing-binding validation"
    );
}

#[test]
fn star_import_inside_python_scope_gets_a_stable_diagnostic() {
    let module_level = "from helper import *\n";
    assert_eq!(transpile(module_level).unwrap(), module_level);

    let module_condition = "if ready:\n    from helper import *\n";
    assert_eq!(transpile(module_condition).unwrap(), module_condition);

    let function = transpile("def load():\n    from helper import *\n")
        .expect_err("star imports inside functions must be rejected by the shared parser");
    assert_eq!(function.len(), 1, "{function:?}");
    assert_eq!(function[0].code, DiagnosticCode::ImportStarOutsideModule);

    let inline_function = transpile("def load(): value = 1; from helper import *\n")
        .expect_err("star imports after an inline statement must be rejected");
    assert_eq!(inline_function.len(), 1, "{inline_function:?}");
    assert_eq!(
        inline_function[0].code,
        DiagnosticCode::ImportStarOutsideModule
    );

    let inline_function = transpile("def load(): from helper import *\n")
        .expect_err("star imports inside inline functions must be rejected");
    assert_eq!(inline_function.len(), 1, "{inline_function:?}");
    assert_eq!(
        inline_function[0].code,
        DiagnosticCode::ImportStarOutsideModule
    );

    let class = transpile("class Loader:\n    from helper import *\n")
        .expect_err("star imports inside classes must be rejected by the shared parser");
    assert_eq!(class.len(), 1, "{class:?}");
    assert_eq!(class[0].code, DiagnosticCode::ImportStarOutsideModule);

    let inline_class = transpile("class Loader: from helper import *\n")
        .expect_err("star imports inside inline classes must be rejected");
    assert_eq!(inline_class.len(), 1, "{inline_class:?}");
    assert_eq!(
        inline_class[0].code,
        DiagnosticCode::ImportStarOutsideModule
    );
    let inline_class_after_statement = transpile("class Loader: value = 1; from helper import *\n")
        .expect_err("star imports after an inline class statement must be rejected");
    assert_eq!(
        inline_class_after_statement[0].code,
        DiagnosticCode::ImportStarOutsideModule
    );
}

#[test]
fn control_flow_inside_except_star_gets_a_stable_diagnostic() {
    let cases = [
        (
            "return",
            "def load():\n    try:\n        pass\n    except* Exception:\n        return\n",
        ),
        (
            "break",
            "while True:\n    try:\n        pass\n    except* Exception:\n        break\n",
        ),
        (
            "continue",
            "while True:\n    try:\n        pass\n    except* Exception:\n        continue\n",
        ),
        (
            "nested return",
            "def load():\n    try:\n        pass\n    except* Exception:\n        if ready:\n            return\n",
        ),
        (
            "inline break",
            "while True:\n    try:\n        pass\n    except* Exception:\n        when ready then break\n",
        ),
        (
            "break after statement",
            "while True:\n    try:\n        pass\n    except* Exception:\n        value = 1; break\n",
        ),
        (
            "continue after statement",
            "while True:\n    try:\n        pass\n    except* Exception:\n        value = 1; continue\n",
        ),
        (
            "return after statement",
            "def load():\n    try:\n        pass\n    except* Exception:\n        value = 1; return\n",
        ),
    ];
    for (label, source) in cases {
        let problems = transpile(source)
            .expect_err("control flow inside except* must be rejected by the shared parser");
        assert_eq!(problems.len(), 1, "{label}: {problems:?}");
        assert_eq!(
            problems[0].code,
            DiagnosticCode::ControlFlowInExceptStar,
            "{label}: {problems:?}"
        );
    }

    let normal_except =
        "def load():\n    try:\n        pass\n    except Exception:\n        return\n";
    assert_eq!(transpile(normal_except).unwrap(), normal_except);
    let valid_except_star = "try:\n    pass\nexcept* Exception:\n    pass\n";
    assert_eq!(transpile(valid_except_star).unwrap(), valid_except_star);
    let nested_function =
        "def outer():\n    try:\n        pass\n    except* Exception:\n        def inner():\n            return\n";
    assert_eq!(transpile(nested_function).unwrap(), nested_function);
    let nested_method = "def outer():\n    try:\n        pass\n    except* Exception:\n        class C:\n            def method(self):\n                return\n";
    assert_eq!(transpile(nested_method).unwrap(), nested_method);
    let return_after_except_star =
        "def load():\n    try:\n        pass\n    except* Exception:\n        pass\n    return\n";
    assert_eq!(
        transpile(return_after_except_star).unwrap(),
        return_after_except_star
    );
    let malformed_header = "except* Exception:\n    break\n";
    assert_eq!(transpile(malformed_header).unwrap(), malformed_header);
}

#[test]
fn yield_inside_comprehension_gets_a_stable_diagnostic() {
    let cases = [
        (
            "list comprehension",
            "def collect(values):\n    return [(yield value) for value in values]\n",
        ),
        (
            "generator expression",
            "def collect(values):\n    return (yield value for value in values)\n",
        ),
        (
            "set comprehension",
            "def collect(values):\n    return {(yield value) for value in values}\n",
        ),
        (
            "dictionary comprehension",
            "def collect(values):\n    return {value: (yield value) for value in values}\n",
        ),
        (
            "comprehension inside a generator lambda",
            "def collect(values):\n    return (lambda: [(yield value) for value in values])\n",
        ),
        (
            "inline body",
            "def collect(values):\n    when True then [(yield value) for value in values]\n",
        ),
    ];
    for (label, source) in cases {
        let problems = transpile(source)
            .expect_err("yield inside a comprehension must be rejected by the shared parser");
        assert_eq!(problems.len(), 1, "{label}: {problems:?}");
        assert_eq!(
            problems[0].code,
            DiagnosticCode::YieldInsideComprehension,
            "{label}: {problems:?}"
        );
    }

    let yield_group = "def collect():\n    return (yield value)\n";
    assert_eq!(transpile(yield_group).unwrap(), yield_group);
    let lambda_inside_comprehension =
        "def collect(values):\n    return [(lambda: (yield value)) for value in values]\n";
    assert_eq!(
        transpile(lambda_inside_comprehension).unwrap(),
        lambda_inside_comprehension
    );
}

#[test]
fn async_comprehension_outside_async_function_gets_a_stable_diagnostic() {
    let cases = [
        ("top level", "values = [item async for item in stream()]\n"),
        (
            "synchronous function",
            "def collect():\n    return [item async for item in stream()]\n",
        ),
        (
            "sentence-en",
            "when true then [item async for item in stream()]\n",
        ),
        (
            "sentence-ko",
            "만약 참 그러면 [item async for item in stream()]\n",
        ),
        (
            "beginner-en",
            "if True then [item async for item in stream()]\n",
        ),
        (
            "beginner-ko",
            "만약 True 그러면 [item async for item in stream()]\n",
        ),
        (
            "advanced-en",
            "if (True) then [item async for item in stream()]\n",
        ),
        (
            "advanced-ko",
            "만약 ((참 그리고 참)) 그러면 [item async for item in stream()]\n",
        ),
        (
            "normal lambda inside async function",
            "async def outer():\n    return lambda: [item async for item in stream()]\n",
        ),
    ];
    for (label, source) in cases {
        let problems =
            transpile(source).expect_err("async comprehensions need an async function context");
        assert_eq!(problems.len(), 1, "{label}: {problems:?}");
        assert_eq!(
            problems[0].code,
            DiagnosticCode::AsyncComprehensionOutsideAsyncFunction,
            "{label}: {problems:?}"
        );
    }

    let valid = "async def collect():\n    return [item async for item in stream()]\n";
    assert_eq!(transpile(valid).unwrap(), valid);
    let nested_async =
        "def outer():\n    async def inner():\n        return [item async for item in stream()]\n";
    assert_eq!(transpile(nested_async).unwrap(), nested_async);
    let valid_inline =
        "async def collect():\n    when ready then [item async for item in stream()]\n";
    assert!(transpile(valid_inline).is_ok());
}

#[test]
fn return_value_inside_async_generator_gets_a_stable_diagnostic() {
    let cases = [
        (
            "return after yield",
            "async def stream():\n    yield 1\n    return 2\n",
        ),
        (
            "return before yield",
            "async def stream():\n    return 2\n    yield 1\n",
        ),
        (
            "one-line Python suite",
            "async def stream(): yield 1; return 2\n",
        ),
        (
            "sentence-en",
            "async def stream():\n    yield 1\n    when True then return 2\n",
        ),
        (
            "sentence-ko",
            "async def stream():\n    yield 1\n    만약 참 그러면 return 2\n",
        ),
        (
            "beginner-en",
            "async def stream():\n    yield 1\n    if True then return 2\n",
        ),
        (
            "beginner-ko",
            "async def stream():\n    yield 1\n    만약 True 그러면 return 2\n",
        ),
        (
            "advanced-en",
            "async def stream():\n    yield 1\n    if (True) then return 2\n",
        ),
        (
            "advanced-ko",
            "async def stream():\n    yield 1\n    만약 ((참 그리고 참)) 그러면 return 2\n",
        ),
    ];
    for (label, source) in cases {
        let problems = transpile(source).expect_err("an async generator cannot return a value");
        assert_eq!(problems.len(), 1, "{label}: {problems:?}");
        assert_eq!(
            problems[0].code,
            DiagnosticCode::ReturnValueInAsyncGenerator,
            "{label}: {problems:?}"
        );
    }

    let async_function = "async def compute():\n    return 2\n";
    assert_eq!(transpile(async_function).unwrap(), async_function);
    let bare_return = "async def stream():\n    yield 1\n    return\n";
    assert_eq!(transpile(bare_return).unwrap(), bare_return);
    let nested_function = "async def outer():\n    yield 1\n    def inner():\n        return 2\n";
    assert_eq!(transpile(nested_function).unwrap(), nested_function);
    let nested_async_function =
        "async def outer():\n    yield 1\n    async def inner():\n        return 2\n";
    assert_eq!(
        transpile(nested_async_function).unwrap(),
        nested_async_function
    );
}

#[test]
fn one_line_python_function_suites_keep_contextual_keywords_in_scope() {
    let valid_cases = [
        ("normal generator", "def stream(): yield 1\n"),
        ("async generator", "async def stream(): yield 1\n"),
        ("async await", "async def wait(): await value\n"),
        (
            "bare return after yield",
            "async def stream(): yield 1; return\n",
        ),
        (
            "nested normal generator",
            "async def outer():\n    def inner(): yield 1\n    return 2\n",
        ),
        (
            "nested async generator",
            "async def outer():\n    async def inner(): yield 1; return\n",
        ),
    ];
    for (label, source) in valid_cases {
        assert_eq!(transpile(source).unwrap(), source, "{label}");
    }

    let nested_invalid = "async def outer():\n    async def inner(): yield 1; return 2\n";
    let problems = transpile(nested_invalid).expect_err("nested async generator return value");
    assert_eq!(problems.len(), 1, "{problems:?}");
    assert_eq!(
        problems[0].code,
        DiagnosticCode::ReturnValueInAsyncGenerator
    );
}

#[test]
fn conflicting_global_and_nonlocal_declarations_are_rejected_before_cpython() {
    let global_cases = [
        "def update():\n    value = 1\n    global value\n",
        "def read():\n    print(value)\n    global value\n",
        "def parameter(value):\n    global value\n",
        "def annotated():\n    fn: value = 1\n    global value\n",
        "def update(): value = 1; global value\n",
        "def read(): print(value); global value\n",
        "def parameter(value): global value\n",
        "def annotated(): fn: value = 1; global value\n",
        "def annotated(): value: other = 1; global value\n",
        "def annotated(): global value; value: other = 1\n",
        "def annotated():\n    value: other = 1\n    global value\n",
        "value: other\nglobal value\n",
    ];
    for source in global_cases {
        let problems =
            transpile(source).expect_err("global declaration conflict should not reach CPython");
        assert_eq!(problems.len(), 1, "global case: {problems:?}");
        assert_eq!(
            problems[0].code,
            DiagnosticCode::GlobalDeclarationConflict,
            "global case: {problems:?}"
        );
    }

    let nonlocal_cases = [
        "def outer():\n    value = 1\n    def update():\n        value = 2\n        nonlocal value\n",
        "def outer():\n    value = 1\n    def read():\n        print(value)\n        nonlocal value\n",
        "def outer():\n    value = 1\n    def parameter(value):\n        nonlocal value\n",
        "def outer():\n    value = 1\n    def update(): value = 2; nonlocal value\n",
        "def outer():\n    value = 1\n    def read(): print(value); nonlocal value\n",
        "def outer():\n    value = 1\n    def parameter(value): nonlocal value\n",
        "def outer():\n    value = 1\n    def annotated(): value: other = 1; nonlocal value\n",
        "def outer():\n    value = 1\n    def annotated(): nonlocal value; value: other = 1\n",
    ];
    for source in nonlocal_cases {
        let problems =
            transpile(source).expect_err("nonlocal declaration conflict should not reach CPython");
        assert_eq!(problems.len(), 1, "nonlocal case: {problems:?}");
        assert_eq!(
            problems[0].code,
            DiagnosticCode::NonlocalDeclarationConflict,
            "nonlocal case: {problems:?}"
        );
    }

    let valid_global = "def update():\n    global value\n    value = 1\n";
    assert_eq!(transpile(valid_global).unwrap(), valid_global);
    let valid_nonlocal =
        "def outer():\n    value = 1\n    def update():\n        nonlocal value\n        value = 2\n";
    assert_eq!(transpile(valid_nonlocal).unwrap(), valid_nonlocal);
    let valid_comprehension =
        "def collect():\n    values = [value for value in items]\n    global value\n";
    assert_eq!(transpile(valid_comprehension).unwrap(), valid_comprehension);
    let valid_one_line_function = "value = 1\ndef read(): global value\n";
    assert_eq!(
        transpile(valid_one_line_function).unwrap(),
        valid_one_line_function
    );
    let valid_lambda = "def read():\n    fn = lambda value: value\n    global value\n";
    assert_eq!(transpile(valid_lambda).unwrap(), valid_lambda);
    let valid_annotation = "def read():\n    fn: other = 1\n    global value\n";
    assert_eq!(transpile(valid_annotation).unwrap(), valid_annotation);
    let valid_module_annotation_global = "global value\nvalue: other\n";
    assert_eq!(
        transpile(valid_module_annotation_global).unwrap(),
        valid_module_annotation_global
    );
}

#[test]
fn inline_branches_without_an_open_condition_get_a_stable_diagnostic() {
    let cases = [
        ("sentence-en", "when true then else show no\n"),
        ("sentence-ko", "만약 참 그러면 아니면 말해 아니요\n"),
        ("beginner-en", "if True then else show no\n"),
        ("beginner-ko", "만약 True 그러면 아니면 말해 아니요\n"),
        ("advanced-en", "if (True) then else show no\n"),
        (
            "advanced-ko",
            "만약 ((참 그리고 참)) 그러면 아니면 말해 아니요\n",
        ),
    ];

    for (label, source) in cases {
        let problems = match transpile(source) {
            Ok(output) => panic!("expected inline branch diagnostic for {label}, got {output:?}"),
            Err(problems) => problems,
        };
        assert_eq!(problems.len(), 1, "core case: {label}: {problems:?}");
        let problem = &problems[0];
        assert_eq!(
            problem.code,
            DiagnosticCode::BranchWithoutCondition,
            "core case: {label}"
        );
        assert!(
            problem.message.contains("condition block open above it"),
            "{label}: {problem:?}"
        );
        assert!(
            problem
                .message_ko
                .as_deref()
                .is_some_and(|message| message.contains("열린 조건")),
            "{label}: {problem:?}"
        );
    }
}

#[test]
fn sentence_repeat_inline_branches_without_a_condition_get_a_stable_diagnostic() {
    let cases = [
        ("sentence-en", "repeat 3 times and else show no\n"),
        ("sentence-ko", "3번 반복해서 아니면 말해 아니요\n"),
    ];

    for (label, source) in cases {
        let problems = match transpile(source) {
            Ok(output) => panic!("expected repeat branch diagnostic for {label}, got {output:?}"),
            Err(problems) => problems,
        };
        assert_eq!(problems.len(), 1, "core case: {label}: {problems:?}");
        let problem = &problems[0];
        assert_eq!(
            problem.code,
            DiagnosticCode::BranchWithoutCondition,
            "core case: {label}"
        );
    }
}

#[test]
fn only_the_bundled_modules_are_available() {
    // This test used to name `use math` as the module NME does not bundle.
    // NME bundles it now, so the example moved rather than the rule: a name
    // NME has never shipped still gets the whole list back.
    let message = err("use network\n");
    assert!(
        message.contains("not one of the seven modules NME carries"),
        "{message}"
    );
    assert!(message.contains("`zero_knowledge`"), "{message}");
}

#[test]
fn unavailable_random_version_reports_the_bundled_version() {
    let message = err("use random version \"9.9.9\"\n");
    assert!(message.contains("9.9.9"), "{message}");
    assert!(message.contains("0.0.1"), "{message}");
}

#[test]
fn random_module_does_not_overwrite_existing_names() {
    let message = err("random_number = 42\nuse random\n");
    assert!(message.contains("would take over names"), "{message}");
    assert!(message.contains("random_number"), "{message}");

    let imported = err("import random_number\nuse random\n");
    assert!(imported.contains("would take over names"), "{imported}");
    assert!(imported.contains("random_number"), "{imported}");
}

#[test]
fn file_module_does_not_overwrite_existing_names() {
    let message = err("file_read = \"mine\"\nuse file\n");
    assert!(message.contains("would take over names"), "{message}");
    assert!(message.contains("file_read"), "{message}");

    let korean = bilingual_err("파일버전 = 1\n파일 사용\n");
    assert!(korean.contains("이미 만든 이름을 가져갑니다"), "{korean}");
}

#[test]
fn two_modules_on_one_line_are_rejected() {
    let message = err("use random and file\n");
    assert!(
        message.contains("not one of the seven modules NME carries"),
        "{message}"
    );
}

#[test]
fn a_file_read_without_a_target_is_reported() {
    let message = err("read \"notes.txt\"\n");
    assert!(
        message.contains("does not say where to put what it reads"),
        "{message}"
    );
}

#[test]
fn a_module_import_needs_a_nme_path_and_names() {
    let not_nme = err("from \"helper.py\" import greet\n");
    assert!(not_nme.contains(".nme"), "{not_nme}");

    let no_names = err("from \"helper.nme\" import\n");
    assert!(no_names.contains("module import"), "{no_names}");

    let bad_shape = err("from \"helper.nme\" import greet 1\n");
    assert!(bad_shape.contains("module import"), "{bad_shape}");
    // Each of the two mistakes now has a code of its own, so a reader looking
    // one up is not sent to a page about missing actions.
    assert_eq!(error_code("from \"helper.py\" import greet\n"), "E0407");
    assert_eq!(error_code("from \"helper.nme\" import\n"), "E0408");
}

#[test]
fn a_module_import_needs_a_python_identifier_file_name() {
    let dashed = err("from \"my-helper.nme\" import greet\n");
    assert!(dashed.contains("letters, numbers, and `_`"), "{dashed}");

    let dotted = err("from \"shapes.ko.nme\" import rect\n");
    assert!(dotted.contains("letters, numbers, and `_`"), "{dotted}");
}

#[test]
fn a_file_write_without_a_path_is_reported() {
    let message = err("write \"hello\" to\n");
    assert!(message.contains("not inside quotation marks"), "{message}");
}

#[test]
fn sentence_punctuation_can_be_plain_output_without_an_action() {
    assert_eq!(
        transpile("Hello there!\n").unwrap(),
        "print(\"Hello there!\")\n"
    );
}

#[test]
fn action_typos_must_have_one_unambiguous_meaning() {
    let say_or_ask = err("asy name Hello\n");
    assert!(say_or_ask.contains("more than one action"), "{say_or_ask}");

    let ask_or_use = err("usk random latest\n");
    assert!(ask_or_use.contains("more than one action"), "{ask_or_use}");
}

#[test]
fn plain_prose_and_common_action_typos_are_easy_output() {
    assert_eq!(
        transpile("hello world\n").unwrap(),
        "print(\"hello world\")\n"
    );
    assert_eq!(transpile("shwoe Hello\n").unwrap(), "print(\"Hello\")\n");
}

#[test]
fn condition_templates_reject_unexplained_middle_words() {
    let message = err("ready = True\nif ready banana exists then show no\n");
    assert!(message.contains("condition"), "{message}");
}

#[test]
fn incomplete_english_comparisons_do_not_become_identity_python() {
    let message = err("if score is greater then show high\n");
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
    assert_eq!(
        transpile("score = 7\nif score is greater than 5 thne show high\n").unwrap(),
        "score = 7\nif (score > 5): print(\"high\")\n"
    );
}

#[test]
fn module_action_and_latest_typos_are_recovered_when_the_shape_is_clear() {
    assert_eq!(
        transpile("use random lates\n").unwrap(),
        concat!(
            "import random as 랜덤; random = 랜덤; random_number = 랜덤.randint; ",
            "random_pick = 랜덤.choice; shuffle = 랜덤.shuffle; ",
            "랜덤정수 = 랜덤.randint; 랜덤선택 = 랜덤.choice; 섞기 = 랜덤.shuffle; ",
            "random_version = 랜덤버전 = \"0.0.1\"\n",
        )
    );
    assert_eq!(
        transpile("랜덤 사요 최신\n").unwrap(),
        concat!(
            "import random as 랜덤; random = 랜덤; random_number = 랜덤.randint; ",
            "random_pick = 랜덤.choice; shuffle = 랜덤.shuffle; ",
            "랜덤정수 = 랜덤.randint; 랜덤선택 = 랜덤.choice; 섞기 = 랜덤.shuffle; ",
            "random_version = 랜덤버전 = \"0.0.1\"\n",
        )
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
    assert!(
        missing.contains("is still open at the end of the file"),
        "{missing}"
    );
    let unmatched = err("else\n");
    assert!(
        unmatched.contains("`else` needs a condition block open above it"),
        "{unmatched}"
    );
    let outside = err("break\n");
    assert!(outside.contains("inside a loop"), "{outside}");
}

#[test]
fn incomplete_value_changes_get_a_friendly_diagnostic() {
    let message = err("score add\n");
    assert!(message.contains("value change"), "{message}");
    assert!(message.contains("score add 1"), "{message}");
}

#[test]
fn a_stray_end_after_nme_code_is_reported() {
    let message = err("say \"hi\"\nend\n");
    assert!(message.contains("no open NME block"), "{message}");
    assert!(message.contains("hint"), "{message}");

    let korean = bilingual_err("안녕 말해줘\n끝\n");
    assert!(
        korean.contains("이 `끝`을 닫을 열린 NME 블록이 없습니다"),
        "{korean}"
    );
}

#[test]
fn an_extra_end_after_a_closed_block_is_reported() {
    let message = err("if true\n    say \"hi\"\nend\nend\n");
    assert!(message.contains("no open NME block"), "{message}");
}

#[test]
fn a_flat_block_still_requires_its_own_end() {
    let message = err("점수가 5와 같으면\n만약 true라면\n    say \"a\"\n끝\n");
    assert!(
        message.contains("is still open at the end of the file"),
        "{message}"
    );
}

#[test]
fn arithmetic_on_a_line_of_its_own_is_refused() {
    // `score+1` looks like the shortest way to say "add one" and is the one
    // shape where doing nothing is indistinguishable from working: Python
    // computes the answer and drops it.
    for line in ["score+1", "score + 1", "score*2"] {
        let message = err(&format!("set score to 0\n{line}\n"));
        assert!(message.contains("throws it away"), "{message}");
        assert!(message.contains("add 1 to score"), "{message}");
    }
    let korean = bilingual_err("점수는 0\n점수+1\n");
    assert!(korean.contains("버립니다"), "{korean}");
    // Doing something with the answer is not this error.
    assert!(transpile("set score to 0\nshow score + 1\n").is_ok());
}

#[test]
fn a_name_the_language_needs_is_refused() {
    // `the total of marks` is `sum(marks)`, so a value called `sum` takes the
    // reading away from every later line — and the error used to land on one
    // of those lines, saying `'int' object is not callable`.
    let problems = transpile("set marks to list of 1, 2\nset sum to 3\n")
        .expect_err("expected this to be refused");
    assert_eq!(problems[0].code.code(), "E0237");
    assert!(problems[0].message.contains("sum"), "{problems:?}");
    // Python written as Python is left alone: whoever writes `sum = 0` in
    // Python has said what they mean.
    assert_eq!(
        transpile("sum = 0\nprint(sum)\n").unwrap(),
        "sum = 0\nprint(sum)\n"
    );
    assert!(transpile("set total to 5\n").is_ok());
}

#[test]
fn a_name_written_as_two_words_is_named() {
    // `set full name to Mina` made a name called `full` holding the words
    // `name to Mina`, and printing it showed all of them.
    let problems = transpile("set full name to Mina\n").expect_err("expected this to be refused");
    assert_eq!(problems[0].code.code(), "E0230");
    assert!(
        problems[0]
            .hint
            .as_ref()
            .is_some_and(|hint| hint.contains("full_name")),
        "{problems:?}"
    );
    // No connector anywhere is the documented short form and still saves.
    assert_eq!(
        transpile("set greeting Hello world\n").unwrap(),
        "greeting = \"Hello world\"\n"
    );
}

/// A saving word written on its own — a beginner who typed `set` and stopped,
/// or who dictated `저장해` and never said what into. The audit that read the
/// code thought this branch was unreachable; running it says otherwise, so it
/// is pinned here.
#[test]
fn a_saving_word_on_its_own_says_the_name_is_missing() {
    for source in [
        "set\n",
        "save\n",
        "store\n",
        "remember\n",
        "let\n",
        "make\n",
        "저장해\n",
        "기억해\n",
    ] {
        assert_eq!(error_code(source), "E0413", "{source}");
    }
    let message = err("set\n");
    assert!(message.contains("name to save is missing"), "{message}");
}

/// A module tool written with nothing to work on used to compile to
/// `print(<function <lambda>>)`: a program that runs, says nothing anybody
/// wanted, and never says why.
#[test]
fn a_module_tool_on_its_own_says_what_is_missing() {
    for source in [
        "use list\nshow count\n",
        "목록 사용\n개수 말해줘\n",
        "use math\nshow root\n",
        "날짜 사용\n며칠뒤 말해줘\n",
    ] {
        assert_eq!(error_code(source), "E0410", "{source}");
    }
    let message = err("use list\nshow count\n");
    assert!(message.contains("`count` is a tool"), "{message}");
}

/// The six date names that answer with nothing written after them. Without
/// this a whole date program could not be written in sentences.
#[test]
fn the_date_names_answer_when_they_stand_alone() {
    for (source, expected) in [
        ("use date\nshow today\n", "print(today())"),
        ("use date\nshow weekday\n", "print(weekday())"),
        ("use date\nset stamp to year\n", "stamp = year()"),
        ("날짜 사용\n오늘 말해줘\n", "print(오늘())"),
        ("날짜 사용\n요일 말해줘\n", "print(요일())"),
        ("날짜 사용\n올해날짜는 올해\n", "올해날짜 = 올해()"),
    ] {
        let python = transpile(source).expect("this program compiles");
        assert!(python.contains(expected), "{source} -> {python}");
    }
    // A fixed value is not a tool and is shown as it stands.
    let python = transpile("use math\nshow pi\n").expect("this program compiles");
    assert!(python.contains("print(pi)"), "{python}");
}
