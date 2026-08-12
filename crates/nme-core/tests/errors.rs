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
    let valid_class = "def outer():\n    value = 1\n    class C:\n        nonlocal value\n";
    assert_eq!(transpile(valid_class).unwrap(), valid_class);
    let valid_method =
        "def outer():\n    value = 1\n    class C:\n        def method(self):\n            nonlocal value\n";
    assert_eq!(transpile(valid_method).unwrap(), valid_method);

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

    let class = transpile("class Loader:\n    from helper import *\n")
        .expect_err("star imports inside classes must be rejected by the shared parser");
    assert_eq!(class.len(), 1, "{class:?}");
    assert_eq!(class[0].code, DiagnosticCode::ImportStarOutsideModule);
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
            problem.message.contains("open condition"),
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
    let message = err("use math\n");
    assert!(
        message.contains("bundles `use random`, `use file`, and `use zero_knowledge`"),
        "{message}"
    );
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
    assert!(message.contains("overwrite existing name"), "{message}");
    assert!(message.contains("random_number"), "{message}");

    let imported = err("import random_number\nuse random\n");
    assert!(imported.contains("overwrite existing name"), "{imported}");
    assert!(imported.contains("random_number"), "{imported}");
}

#[test]
fn file_module_does_not_overwrite_existing_names() {
    let message = err("file_read = \"mine\"\nuse file\n");
    assert!(message.contains("overwrite existing name"), "{message}");
    assert!(message.contains("file_read"), "{message}");

    let korean = bilingual_err("파일버전 = 1\n파일 사용\n");
    assert!(korean.contains("덮어쓸 수 있어요"), "{korean}");
}

#[test]
fn two_modules_on_one_line_are_rejected() {
    let message = err("use random and file\n");
    assert!(
        message.contains("bundles `use random`, `use file`, and `use zero_knowledge`"),
        "{message}"
    );
}

#[test]
fn a_file_read_without_a_target_is_reported() {
    let message = err("read \"notes.txt\"\n");
    assert!(message.contains("target name"), "{message}");
}

#[test]
fn a_module_import_needs_a_nme_path_and_names() {
    let not_nme = err("from \"helper.py\" import greet\n");
    assert!(not_nme.contains(".nme"), "{not_nme}");

    let no_names = err("from \"helper.nme\" import\n");
    assert!(no_names.contains("module import"), "{no_names}");

    let bad_shape = err("from \"helper.nme\" import greet 1\n");
    assert!(bad_shape.contains("module import"), "{bad_shape}");
}

#[test]
fn a_module_import_needs_a_python_identifier_file_name() {
    let dashed = err("from \"my-helper.nme\" import greet\n");
    assert!(dashed.contains("Python identifier"), "{dashed}");

    let dotted = err("from \"shapes.ko.nme\" import rect\n");
    assert!(dotted.contains("Python identifier"), "{dotted}");
}

#[test]
fn a_file_write_without_a_path_is_reported() {
    let message = err("write \"hello\" to\n");
    assert!(message.contains("quoted path"), "{message}");
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

#[test]
fn a_stray_end_after_nme_code_is_reported() {
    let message = err("say \"hi\"\nend\n");
    assert!(message.contains("no open NME block"), "{message}");
    assert!(message.contains("hint"), "{message}");

    let korean = bilingual_err("안녕 말해줘\n끝\n");
    assert!(
        korean.contains("이 `끝`을 닫을 열린 NME 블록이 없어요"),
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
    assert!(message.contains("missing its closing `end`"), "{message}");
}
