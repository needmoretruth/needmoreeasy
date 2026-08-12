//! End-to-end tests for the NME-native backend: compile the core subset to
//! C, build it with the system C compiler, run the native executable, and
//! compare its output with the expected text. Programs outside the core
//! subset must be rejected with a clear diagnostic, never miscompiled.

use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use nme_core::diagnostics::DiagnosticCode;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn native_compiler() -> &'static str {
    if cfg!(windows) {
        "cl"
    } else {
        "cc"
    }
}

fn native_run(source: &str) -> Result<String, String> {
    let c_source = nme_native::native_compile(source).map_err(|problems| {
        problems
            .iter()
            .map(|problem| problem.message.clone())
            .collect::<Vec<_>>()
            .join("; ")
    })?;
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("nme-native-test-{}-{id}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let c_path = dir.join("program.c");
    std::fs::write(&c_path, c_source).unwrap();
    let exe = if cfg!(windows) {
        dir.join("program.exe")
    } else {
        dir.join("program")
    };
    let mut compiler = Command::new(native_compiler());
    if cfg!(windows) {
        compiler
            .arg("/nologo")
            .arg("/O2")
            .arg("/utf-8")
            .arg(format!("/Fe:{}", exe.display()))
            .arg(&c_path);
    } else {
        compiler.arg("-O2").arg(&c_path).arg("-o").arg(&exe);
    }
    let status = compiler
        .current_dir(&dir)
        .stdout(std::process::Stdio::null())
        .status()
        .map_err(|error| format!("could not start {}: {error}", native_compiler()))?;
    if !status.success() {
        return Err("the generated C did not compile".to_string());
    }
    let output = Command::new(&exe)
        .output()
        .map_err(|error| format!("could not run the native program: {error}"))?;
    let _ = std::fs::remove_dir_all(&dir);
    if !output.status.success() {
        return Err(format!(
            "native program failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n"))
}

#[cfg(not(windows))]
#[test]
fn minimal_generated_c_compiles_with_warnings_as_errors() {
    if Command::new("cc").arg("--version").output().is_err() {
        eprintln!("cc not available; skipping strict generated-C test");
        return;
    }
    let c_source = nme_native::native_compile("show 1\n").unwrap();
    let id = COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("nme-native-strict-{}-{id}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let c_path = dir.join("program.c");
    let exe = dir.join("program");
    std::fs::write(&c_path, c_source).unwrap();
    let result = Command::new("cc")
        .args(["-Wall", "-Wextra", "-Werror", "-O2"])
        .arg(&c_path)
        .arg("-o")
        .arg(&exe)
        .output()
        .unwrap();
    let _ = std::fs::remove_dir_all(&dir);
    assert!(
        result.status.success(),
        "strict generated C failed: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

fn native_rejects(source: &str) -> bool {
    nme_native::native_compile(source).is_err()
}

#[test]
fn a_while_loop_countdown_runs_natively() {
    assert_eq!(
        native_run("score = 0\nwhile score is less than 3\n    score add 1\nend\nshow score\n")
            .unwrap(),
        "3\n"
    );
}

#[test]
fn arithmetic_in_say_lowers_to_c() {
    assert_eq!(native_run("x = 2\nshow x * 3 + 1\n").unwrap(), "7\n");
}

#[test]
fn integer_arithmetic_overflow_fails_explicitly() {
    let error = native_run("show 2147483647 + 1\n")
        .expect_err("native integer overflow must not be undefined C behavior");
    assert!(error.contains("integer overflow"), "{error}");
    assert!(error.contains("정수 오버플로"), "{error}");
}

#[test]
fn native_integer_boundaries_and_literal_range_are_checked() {
    assert_eq!(
        native_run("x = 2147483647\nshow x\nx = -2147483648\nshow x\n").unwrap(),
        "2147483647\n-2147483648\n"
    );

    let problems = nme_native::native_compile("show 2147483648\n").unwrap_err();
    assert!(
        problems
            .iter()
            .any(|problem| problem.message.contains("outside that range")),
        "{problems:?}"
    );
    assert!(
        problems.iter().any(|problem| {
            problem
                .message_ko
                .as_deref()
                .is_some_and(|message| message.contains("범위를 벗어납니다"))
        }),
        "{problems:?}"
    );
}

#[test]
fn other_integer_runtime_errors_are_explicit() {
    for (source, expected) in [
        ("show 46341 * 46341\n", "integer overflow"),
        ("x = -2147483648\nshow x - 1\n", "integer overflow"),
        ("show 1 % 0\n", "integer modulo by zero"),
    ] {
        let error = native_run(source).expect_err("invalid native integer arithmetic");
        assert!(error.contains(expected), "{source:?}: {error}");
        assert!(error.contains("정수"), "{source:?}: {error}");
    }
}

#[test]
fn native_functions_reject_float_arguments_and_returns() {
    for source in [
        "def identity(value):\n    return value\n\nshow identity(1.5)\n",
        "def fractional():\n    return 1.5\n\nshow fractional()\n",
    ] {
        let problems = nme_native::native_compile(source).unwrap_err();
        assert!(
            problems
                .iter()
                .any(|problem| { problem.message.contains("accept and return integer values") }),
            "{source:?}: {problems:?}"
        );
        assert!(
            problems.iter().any(|problem| {
                problem
                    .message_ko
                    .as_deref()
                    .is_some_and(|message| message.contains("정수 값만"))
            }),
            "{source:?}: {problems:?}"
        );
    }
}

#[test]
fn native_functions_require_a_return_on_every_path() {
    for source in [
        "def missing(value):\n    value = value + 1\n\nshow missing(1)\n",
        "def conditional(value):\n    if value\n        return 1\n    end\n\nshow conditional(0)\n",
        "def branches(value):\n    if value\n        return 1\n    else\n        return 2\n    end\n\nshow branches(1)\n",
        "def 분기(값):\n    만약 값\n        return 1\n    아니면\n        return 2\n    끝\n\n말해 분기(1)\n",
    ] {
        let problems = nme_native::native_compile(source).unwrap_err();
        assert!(
            problems
                .iter()
                .any(|problem| problem.message.contains("return an integer on every path")),
            "{source:?}: {problems:?}"
        );
        assert!(
            problems.iter().any(|problem| {
                problem
                    .message_ko
                    .as_deref()
                    .is_some_and(|message| message.contains("모든 경로"))
            }),
            "{source:?}: {problems:?}"
        );
    }
}

#[test]
fn unknown_native_function_calls_are_rejected_before_c_generation() {
    let problems = nme_native::native_compile("show missing(1)\n").unwrap_err();
    assert!(
        problems
            .iter()
            .any(|problem| problem.message.contains("unknown native function")),
        "{problems:?}"
    );
    assert!(
        problems.iter().any(|problem| {
            problem
                .message_ko
                .as_deref()
                .is_some_and(|message| message.contains("알 수 없는 네이티브 함수"))
        }),
        "{problems:?}"
    );
}

#[test]
fn native_function_calls_require_the_declared_arity() {
    for source in [
        "def identity(value):\n    return value\n\nshow identity()\n",
        "def identity(value):\n    return value\n\nshow identity(1, 2)\n",
    ] {
        let problems = nme_native::native_compile(source).unwrap_err();
        assert!(
            problems
                .iter()
                .any(|problem| problem.message.contains("expects 1 integer argument(s)")),
            "{source:?}: {problems:?}"
        );
        assert!(
            problems.iter().any(|problem| {
                problem
                    .message_ko
                    .as_deref()
                    .is_some_and(|message| message.contains("정수 인자 1개"))
            }),
            "{source:?}: {problems:?}"
        );
    }
}

#[test]
fn mutually_recursive_native_functions_have_forward_declarations() {
    let source = r"def is_even(value):
    if value is less than 1
        return 1
    end
    return is_odd(value - 1)

def is_odd(value):
    if value is less than 1
        return 0
    end
    return is_even(value - 1)

show is_even(6)
";
    assert_eq!(native_run(source).unwrap(), "1\n");
}

#[test]
fn zero_argument_native_functions_use_void_prototypes() {
    let source = "def answer():\n    return 42\n\nshow answer()\n";
    assert_eq!(native_run(source).unwrap(), "42\n");
}

#[test]
fn native_function_calls_reject_keyword_arguments() {
    let source = "def identity(value):\n    return value\n\nshow identity(1, value=2)\n";
    let problems = nme_native::native_compile(source).unwrap_err();
    assert!(
        problems
            .iter()
            .any(|problem| problem.message.contains("keyword arguments")),
        "{problems:?}"
    );
    assert!(
        problems.iter().any(|problem| {
            problem
                .message_ko
                .as_deref()
                .is_some_and(|message| message.contains("키워드 인자"))
        }),
        "{problems:?}"
    );
}

#[test]
fn duplicate_native_function_definitions_are_rejected() {
    let source = "def identity(value):\n    return value\n\ndef identity(value):\n    return value + 1\n\nshow identity(1)\n";
    let problems = nme_native::native_compile(source).unwrap_err();
    assert!(
        problems
            .iter()
            .any(|problem| problem.message.contains("defined more than once")),
        "{problems:?}"
    );
    assert!(
        problems.iter().any(|problem| {
            problem
                .message_ko
                .as_deref()
                .is_some_and(|message| message.contains("두 번 이상 정의"))
        }),
        "{problems:?}"
    );
}

#[test]
fn unsupported_native_function_headers_are_rejected() {
    for source in [
        "def identity(value=1):\n    return value\n\nshow identity(2)\n",
        "def collect(*values):\n    return values\n\nshow collect(1)\n",
    ] {
        let problems = nme_native::native_compile(source).unwrap_err();
        assert!(
            problems
                .iter()
                .any(|problem| problem.message.contains("function header")),
            "{source:?}: {problems:?}"
        );
        assert!(
            problems.iter().any(|problem| {
                problem
                    .message_ko
                    .as_deref()
                    .is_some_and(|message| message.contains("함수 헤더"))
            }),
            "{source:?}: {problems:?}"
        );
    }
}

#[test]
fn native_names_and_function_values_are_checked_before_c_generation() {
    let cases = [
        (
            "value = missing\nshow value\n",
            "without a prior native binding",
            "먼저 네이티브 바인딩",
        ),
        (
            "def identity(value, value):\n    return value\n\nshow identity(1)\n",
            "listed more than once",
            "두 번 이상 나열",
        ),
        (
            "def 계산(값, 값):\n    return 값\n\nshow 계산(1)\n",
            "listed more than once",
            "두 번 이상 나열",
        ),
        (
            "def identity(value):\n    return value\n\nidentity = 1\nshow identity(1)\n",
            "shadows a native function name",
            "함수 이름을 가리는",
        ),
        (
            "def identity(identity):\n    return identity\n\nshow identity(1)\n",
            "shadows a native function name",
            "함수 이름을 가리는",
        ),
        (
            "def identity(value):\n    return value\n\nvalue = identity\nshow value\n",
            "using native function `identity` as a value",
            "함수 `identity`을(를) 값으로",
        ),
    ];
    for (source, english, korean) in cases {
        let problems = nme_native::native_compile(source).unwrap_err();
        assert!(
            problems
                .iter()
                .any(|problem| problem.message.contains(english)),
            "{source:?}: {problems:?}"
        );
        assert!(
            problems.iter().any(|problem| {
                problem
                    .message_ko
                    .as_deref()
                    .is_some_and(|message| message.contains(korean))
            }),
            "{source:?}: {problems:?}"
        );
    }
}

#[test]
fn nested_native_function_definitions_are_rejected() {
    let source = "def outer(value):\n    def inner(nested):\n        return nested\n    return inner(value)\n\nshow outer(1)\n";
    let problems = nme_native::native_compile(source).unwrap_err();
    assert!(
        problems
            .iter()
            .any(|problem| problem.message.contains("nested function definitions")),
        "{problems:?}"
    );
    assert!(
        problems.iter().any(|problem| {
            problem
                .message_ko
                .as_deref()
                .is_some_and(|message| message.contains("중첩 함수 정의"))
        }),
        "{problems:?}"
    );
}

#[test]
fn a_string_literal_is_printed() {
    assert_eq!(
        native_run("show \"hello world\"\n").unwrap(),
        "hello world\n"
    );
}

#[test]
fn escaped_native_strings_remain_valid_c_literals() {
    assert_eq!(
        native_run("show \"line\\nnext\\tend\"\n").unwrap(),
        "line\nnext\tend\n"
    );

    let problems = nme_native::native_compile("show \"a\\0b\"\n").unwrap_err();
    assert!(
        problems
            .iter()
            .any(|problem| { problem.message.contains("embedded NUL characters") }),
        "{problems:?}"
    );
    assert!(
        problems.iter().any(|problem| {
            problem
                .message_ko
                .as_deref()
                .is_some_and(|message| message.contains("내부 NUL 문자"))
        }),
        "{problems:?}"
    );
}

#[test]
fn python_comments_cannot_inject_c_or_change_function_hoisting() {
    let source = "#include <missing_header.h>\n# } { /* comment */\ndef identity(value):\n    return value\n\nshow identity(1)\n";
    assert_eq!(native_run(source).unwrap(), "1\n");
}

#[test]
fn an_if_break_loop_works() {
    let source = "x = 0\nwhile x is less than 5\n    x add 1\n    if x is greater than 2\n        break\n    end\nend\nshow x\n";
    assert_eq!(native_run(source).unwrap(), "3\n");
}

#[test]
fn break_inside_a_non_loop_native_block_is_rejected_bilingually() {
    for source in ["if true\n    break\nend\n", "만약 True라면\n    멈춰\n끝\n"] {
        let problems = nme_native::native_compile(source).unwrap_err();
        assert!(
            problems.iter().any(|problem| {
                problem.code == DiagnosticCode::BreakOutsideLoop
                    && problem.message.contains("can only be used inside a loop")
            }),
            "{source:?}: {problems:?}"
        );
        assert!(
            problems.iter().any(|problem| {
                problem
                    .message_ko
                    .as_deref()
                    .is_some_and(|message| message.contains("반복문 안에서만"))
            }),
            "{source:?}: {problems:?}"
        );
    }
}

#[test]
fn functions_over_scalars_compile_natively() {
    let source = "def twice(n):\n    return n * 2\n\nshow twice(5)\nshow twice(21)\n";
    assert_eq!(native_run(source).unwrap(), "10\n42\n");
}

#[test]
fn blank_lines_between_native_function_header_and_body_keep_function_scope() {
    let source = "def identity(value):\n\n    return value\n\nshow identity(7)\n";
    assert_eq!(native_run(source).unwrap(), "7\n");

    let commented =
        "def identity(value):\n# keep this comment\n    return value\n\nshow identity(7)\n";
    assert_eq!(native_run(commented).unwrap(), "7\n");
}

#[test]
fn return_outside_a_native_function_is_rejected_before_c_generation() {
    let problems = nme_native::native_compile("return 1\n").unwrap_err();
    assert!(
        problems.iter().any(|problem| {
            problem.code == DiagnosticCode::ReturnOutsideFunction
                && problem.message.contains("inside a function")
        }),
        "{problems:?}"
    );
    assert!(
        problems.iter().any(|problem| {
            problem
                .message_ko
                .as_deref()
                .is_some_and(|message| message.contains("함수 안에서만"))
        }),
        "{problems:?}"
    );
}

#[test]
fn function_locals_do_not_leak_into_main() {
    let source = "def local_value(n):\n    local = 0\n    if n\n        local = n + 1\n    end\n    return local\n\nlocal = 100\nshow local\nshow local_value(2)\n";
    assert_eq!(native_run(source).unwrap(), "100\n3\n");
}

#[test]
fn block_bindings_remain_available_after_native_block() {
    let source = "if true\n    y = 2\n    text = \"hi\"\nend\nshow y + 0\nshow text + \"!\"\n";
    assert_eq!(native_run(source).unwrap(), "2\nhi!\n");
}

#[test]
fn type_changing_assignments_are_rejected_before_c_lowering() {
    for source in [
        "value = 1\nvalue = \"text\"\n",
        "def typed():\n    value = 1\n    value = \"text\"\n    return 1\n",
    ] {
        let problems = nme_native::native_compile(source).unwrap_err();
        assert!(
            problems
                .iter()
                .any(|problem| problem.message.contains("changing the type")),
            "{source:?}: {problems:?}"
        );
        assert!(
            problems.iter().any(|problem| {
                problem
                    .message_ko
                    .as_deref()
                    .is_some_and(|message| message.contains("타입 변경"))
            }),
            "{source:?}: {problems:?}"
        );
    }
}

#[test]
fn value_changes_require_an_existing_binding() {
    let problems = nme_native::native_compile("score add 1\n").unwrap_err();
    assert!(
        problems
            .iter()
            .any(|problem| problem.message.contains("assigned before a value change")),
        "{problems:?}"
    );
    assert!(
        problems.iter().any(|problem| {
            problem
                .message_ko
                .as_deref()
                .is_some_and(|message| message.contains("값을 바꾸기 전에"))
        }),
        "{problems:?}"
    );

    let problems = nme_native::native_compile("text = \"hi\"\ntext add 1\n").unwrap_err();
    assert!(
        problems
            .iter()
            .any(|problem| problem.message.contains("changing a string value")),
        "{problems:?}"
    );
    assert!(
        problems.iter().any(|problem| {
            problem
                .message_ko
                .as_deref()
                .is_some_and(|message| message.contains("문자열 값 변경"))
        }),
        "{problems:?}"
    );
}

#[test]
fn conditional_bindings_are_rejected_after_a_maybe_skipped_branch() {
    let source = "ready = 0\nif ready\n    y = 2\nend\nshow y + 0\n";
    let problems = nme_native::native_compile(source).unwrap_err();
    assert!(
        problems
            .iter()
            .any(|problem| problem.message.contains("before a conditional assignment")),
        "{problems:?}"
    );
    assert!(
        problems.iter().any(|problem| {
            problem
                .message_ko
                .as_deref()
                .is_some_and(|message| message.contains("조건부 대입이 실행되기 전에"))
        }),
        "{problems:?}"
    );
}

#[test]
fn bindings_assigned_in_both_if_else_branches_are_available_afterward() {
    let source = "ready = 1\nif ready\n    result = 2\nelse\n    result = 3\nend\nshow result\n";
    assert_eq!(native_run(source).unwrap(), "2\n");

    let korean = "준비 = 1\n만약 준비\n    결과 = 2\n아니면\n    결과 = 3\n끝\n말해 결과\n";
    assert_eq!(native_run(korean).unwrap(), "2\n");

    let function_source = "def choose(value):\n    if value\n        result = 2\n    else\n        result = 3\n    end\n    return result\n\nshow choose(1)\n";
    assert_eq!(native_run(function_source).unwrap(), "2\n");

    let korean_function = "def 선택(값):\n    만약 값\n        결과 = 2\n    아니면\n        결과 = 3\n    끝\n    return 결과\n\n말해 선택(1)\n";
    assert_eq!(native_run(korean_function).unwrap(), "2\n");

    let maybe_source = "ready = 1\nif ready\n    result = 2\nend\nif ready\n    result = 3\nelse\n    result = 4\nend\nshow result\n";
    assert_eq!(native_run(maybe_source).unwrap(), "3\n");

    let korean_maybe = "준비 = 1\n만약 준비\n    결과 = 2\n끝\n만약 준비\n    결과 = 3\n아니면\n    결과 = 4\n끝\n말해 결과\n";
    assert_eq!(native_run(korean_maybe).unwrap(), "3\n");

    let returning_branch = "def choose(value):\n    if value\n        result = 2\n    else\n        return 3\n    end\n    return result\n\nshow choose(1)\nshow choose(0)\n";
    assert_eq!(native_run(returning_branch).unwrap(), "2\n3\n");

    let korean_returning_branch = "def 선택(값):\n    만약 값\n        결과 = 2\n    아니면\n        return 3\n    끝\n    return 결과\n\n말해 선택(1)\n말해 선택(0)\n";
    assert_eq!(native_run(korean_returning_branch).unwrap(), "2\n3\n");
}

#[test]
fn early_return_branch_works_across_the_native_surface_matrix() {
    let cases = [
        (
            "sentence-en",
            "def choose(value):\n    when value exists\n        set result to 2\n    else\n        return 3\n    end\n    return result\n\nshow choose(1)\nshow choose(0)\n",
        ),
        (
            "sentence-ko",
            "def 선택(값):\n    만약에 값이 있으면\n        저장 결과 2\n    아니면\n        return 3\n    끝\n    return 결과\n\n말해 선택(1)\n말해 선택(0)\n",
        ),
        (
            "beginner-en",
            "def choose(value):\n    if value\n        result = 2\n    else\n        return 3\n    end\n    return result\n\nshow choose(1)\nshow choose(0)\n",
        ),
        (
            "beginner-ko",
            "def 선택(값):\n    만약 값\n        결과 = 2\n    아니면\n        return 3\n    끝\n    return 결과\n\n말해 선택(1)\n말해 선택(0)\n",
        ),
        (
            "advanced-en",
            "def choose(value):\n    when value exists\n        result = 2\n    else\n        return 3\n    end\n    return result\n\nshow choose(1)\nshow choose(0)\n",
        ),
        (
            "advanced-ko",
            "def 선택(값):\n    만약에 값이 있으면\n        결과 = 2\n    아니면\n        return 3\n    끝\n    return 결과\n\n말해 선택(1)\n말해 선택(0)\n",
        ),
    ];

    for (label, source) in cases {
        assert_eq!(
            native_run(source).unwrap(),
            "2\n3\n",
            "native case: {label}"
        );
    }
}

#[test]
fn nested_always_terminating_branches_keep_bindings_reachable_afterward() {
    let cases = [
        (
            "sentence-en",
            "def choose(value):\n    when value exists\n        when true\n            return 1\n        end\n    else\n        set result to 2\n    end\n    return result\n\nshow choose(1)\nshow choose(0)\n",
        ),
        (
            "sentence-ko",
            "def 선택(값):\n    만약에 값이 있으면\n        만약 True라면\n            return 1\n        끝\n    아니면\n        저장 결과 2\n    끝\n    return 결과\n\n말해 선택(1)\n말해 선택(0)\n",
        ),
        (
            "beginner-en",
            "def choose(value):\n    if value\n        if True\n            return 1\n        end\n    else\n        result = 2\n    end\n    return result\n\nshow choose(1)\nshow choose(0)\n",
        ),
        (
            "beginner-ko",
            "def 선택(값):\n    만약 값\n        만약 True\n            return 1\n        끝\n    아니면\n        결과 = 2\n    끝\n    return 결과\n\n말해 선택(1)\n말해 선택(0)\n",
        ),
        (
            "advanced-en",
            "def choose(value):\n    when value exists\n        if True\n            return 1\n        end\n    else\n        result = 2\n    end\n    return result\n\nshow choose(1)\nshow choose(0)\n",
        ),
        (
            "advanced-ko",
            "def 선택(값):\n    만약에 값이 있으면\n        if True\n            return 1\n        끝\n    아니면\n        결과 = 2\n    끝\n    return 결과\n\n말해 선택(1)\n말해 선택(0)\n",
        ),
    ];

    for (label, source) in cases {
        assert_eq!(
            native_run(source).unwrap(),
            "1\n2\n",
            "native case: {label}"
        );
    }
}

#[test]
fn break_branch_does_not_require_a_fallthrough_binding() {
    let source = "value = 0\nwhile value < 2\n    if value == 0\n        break\n    else\n        result = 2\n    end\n    show result\n    value add 1\nend\nshow \"done\"\n";
    assert_eq!(native_run(source).unwrap(), "done\n");

    let korean = "값 = 0\n동안 값이 2보다 작을 동안\n    만약 값이 0과 같으면\n        멈춰\n    아니면\n        결과 = 2\n    끝\n    말해 결과\n    값에 1 더해\n끝\n말해 \"끝\"\n";
    assert_eq!(native_run(korean).unwrap(), "끝\n");
}

#[test]
fn break_branch_works_across_the_native_surface_matrix() {
    let cases = [
        (
            "sentence-en",
            "value = 0\nwhile value is less than 2\n    when value equals 0\n        break\n    else\n        set result to 2\n    end\n    show result\n    value add 1\nend\nshow \"done\"\n",
            "done\n",
        ),
        (
            "sentence-ko",
            "값 = 0\n동안 값이 2보다 작을 동안\n    만약에 값이 0과 같으면\n        멈춰\n    아니면\n        저장 결과 2\n    끝\n    말해 결과\n    값에 1 더해\n끝\n말해 \"끝\"\n",
            "끝\n",
        ),
        (
            "beginner-en",
            "value = 0\nwhile value < 2\n    if value == 0\n        break\n    else\n        result = 2\n    end\n    show result\n    value add 1\nend\nshow \"done\"\n",
            "done\n",
        ),
        (
            "beginner-ko",
            "값 = 0\n동안 값 < 2\n    만약 값 == 0\n        멈춰\n    아니면\n        결과 = 2\n    끝\n    말해 결과\n    값에 1 더해\n끝\n말해 \"끝\"\n",
            "끝\n",
        ),
        (
            "advanced-en",
            "def choose():\n    value = 0\n    while value < 2\n        if value == 0\n            break\n        else\n            result = 2\n        end\n        show result\n        value add 1\n    end\n    return 1\n\nshow choose()\n",
            "1\n",
        ),
        (
            "advanced-ko",
            "def 선택():\n    값 = 0\n    동안 값 < 2\n        만약 값 == 0\n            멈춰\n        아니면\n            결과 = 2\n        끝\n        말해 결과\n        값에 1 더해\n    끝\n    return 1\n\n말해 선택()\n",
            "1\n",
        ),
    ];

    for (label, source, expected) in cases {
        assert_eq!(
            native_run(source).unwrap(),
            expected,
            "native case: {label}"
        );
    }
}

#[test]
fn generated_functions_are_file_scope_portable_c() {
    let source = "def twice(n):\n    return n * 2\n\nshow twice(5)\n";
    let c = nme_native::native_compile(source).unwrap();
    let function_at = c.find("int twice(int n) {").expect("generated function");
    let main_at = c.find("int main(void) {").expect("generated main");
    assert!(function_at < main_at, "function must be outside main:\n{c}");
}

#[test]
fn else_and_else_if_branches_compile_natively() {
    let small =
        "score = 3\nif score is greater than 5\n    show \"big\"\n아니면\n    show \"small\"\n끝\n";
    assert_eq!(native_run(small).unwrap(), "small\n");

    let medium = "x = 4\nif x is greater than 5\n    show \"big\"\n아니면 만약에 x is greater than 2\n    show \"medium\"\n아니면\n    show \"small\"\n끝\n";
    assert_eq!(native_run(medium).unwrap(), "medium\n");
}

#[test]
fn recursive_functions_compile_natively() {
    let source = "def fact(n):\n    if n is less than 2\n        return 1\n    end\n    return n * fact(n - 1)\n\nshow fact(5)\n";
    assert_eq!(native_run(source).unwrap(), "120\n");
}

#[test]
fn less_equal_and_greater_equal_conditions_work() {
    let source = "x = 0\nwhile x <= 3\n    x add 1\nend\nshow x\n";
    assert_eq!(native_run(source).unwrap(), "4\n");

    let greater = "x = 5\nif x >= 5\n    show \"yes\"\nend\n";
    assert_eq!(native_run(greater).unwrap(), "yes\n");
}

#[test]
fn natural_language_or_equal_conditions_compile_natively() {
    let source = "x = 3\nif x is less than or equal to 3\n    show \"lte\"\nend\nif x is greater than or equal to 5\n    show \"gte\"\nend\n";
    assert_eq!(native_run(source).unwrap(), "lte\n");
}

#[test]
fn korean_or_equal_conditions_compile_natively() {
    let source = "점수 = 3\n만약에 점수가 10보다 작거나 같으면\n    말해 \"작거나 같음\"\n끝\n만약에 점수가 10보다 크거나 같으면\n    말해 \"크거나 같음\"\n끝\n";
    assert_eq!(native_run(source).unwrap(), "작거나 같음\n");
}

#[test]
fn string_variables_and_binary_concat_compile_natively() {
    let source = "greeting = \"hello\"\nshow greeting\nname = \"NME\"\nshow name + \" rocks\"\nshow greeting + \" friend\"\n";
    assert_eq!(
        native_run(source).unwrap(),
        "hello\nNME rocks\nhello friend\n"
    );

    let korean = "인사 = \"안녕\"\n말해 인사\n말해 인사 + \"하세요\"\n";
    assert_eq!(native_run(korean).unwrap(), "안녕\n안녕하세요\n");
}

#[test]
fn string_self_assignment_uses_overlap_safe_copying() {
    let source = "text = \"hello\"\ntext = text\nshow text\n";
    let c = nme_native::native_compile(source).unwrap();
    assert!(
        c.contains("memmove(destination, source, length + 1);")
            && !c.contains("memcpy(destination, source, length + 1);"),
        "string self-assignment must not lower through overlapping memcpy: {c}"
    );
    assert_eq!(native_run(source).unwrap(), "hello\n");
}

#[test]
fn string_comparison_and_len_compile_natively() {
    let source = "name = \"NME\"\nif name == \"NME\"\n    show \"match\"\nend\nif name != \"other\"\n    show \"different\"\nend\nshow len(name)\nshow len(\"hello\")\n";
    assert_eq!(native_run(source).unwrap(), "match\ndifferent\n3\n5\n");

    let korean = "이름 = \"안녕\"\n만약 이름이 \"안녕\"와 같으면\n    말해 \"같아요\"\n끝\n";
    assert_eq!(native_run(korean).unwrap(), "같아요\n");
    assert_eq!(native_run("말해 len(\"안녕\")\n").unwrap(), "2\n");
}

#[test]
fn comparing_two_string_concats_does_not_reuse_one_runtime_buffer() {
    let source = "left = \"a\"\nright = \"b\"\nif left + \"x\" == right + \"y\"\n    show \"wrong\"\nelse\n    show \"correct\"\nend\n";
    assert_eq!(native_run(source).unwrap(), "correct\n");

    let korean = "왼쪽 = \"a\"\n오른쪽 = \"b\"\n만약 왼쪽 + \"x\" == 오른쪽 + \"y\"\n    말해 \"틀림\"\n아니면\n    말해 \"맞음\"\n끝\n";
    assert_eq!(native_run(korean).unwrap(), "맞음\n");
}

#[test]
fn modulo_arithmetic_compiles_natively() {
    let source = "x = 7\nshow x % 3\nshow 10 % 4\nshow 2 + 10 % 4\n";
    assert_eq!(native_run(source).unwrap(), "1\n2\n4\n");
}

#[test]
fn modulo_in_conditions_compiles_natively() {
    // `%` inside a sentence `while`/`if` condition is a Python condition
    // and lowers to the same C operator as the arithmetic forms.
    let source = "count = 1\nwhile count % 4 != 0\n    count = count + 1\nend\nshow count\nif 7 % 2 == 1\n    show \"odd\"\nend\n";
    assert_eq!(native_run(source).unwrap(), "4\nodd\n");
}

#[test]
fn float_literals_arithmetic_and_conditions_compile_natively() {
    let source = "pi = 3.14\nshow pi\nshow 1 + 0.5\nr = 2\nshow 3.14 * r * r\nwhole = 5.0\nshow whole\nzero = -0.0\nshow zero\nif pi is greater than 3\n    show \"pi big\"\nend\n";
    assert_eq!(
        native_run(source).unwrap(),
        "3.14\n1.5\n12.56\n5\n-0\npi big\n"
    );
}

#[test]
fn non_finite_float_results_fail_in_all_six_native_surfaces() {
    let cases = [
        ("sentence-en", "set value to 1e308\nshow value * value\n"),
        ("sentence-ko", "저장 값 1e308\n말해 값 * 값\n"),
        (
            "beginner-en",
            "value = 1e308\n1 times:\n    show value * value\n",
        ),
        ("beginner-ko", "값 = 1e308\n1번:\n    말해 값 * 값\n"),
        ("advanced-en", "value = 1e308\nshow value * value\n"),
        ("advanced-ko", "값 = 1e308\n말해 값 * 값\n"),
    ];

    for (label, source) in cases {
        let error = native_run(source).expect_err("float overflow must fail at runtime");
        assert!(
            error.contains("non-finite float result"),
            "native case {label}: {error}"
        );
        assert!(
            error.contains("유한하지 않은 실수 결과"),
            "native case {label}: {error}"
        );
    }
}

#[test]
fn non_finite_native_float_literals_are_rejected() {
    let problems = nme_native::native_compile("show 1e309\n").unwrap_err();
    assert!(
        problems
            .iter()
            .any(|problem| problem.message.contains("finite float literals")),
        "{problems:?}"
    );
    assert!(
        problems.iter().any(|problem| {
            problem
                .message_ko
                .as_deref()
                .is_some_and(|message| message.contains("유한한 실수 리터럴"))
        }),
        "{problems:?}"
    );
}

#[test]
fn the_beginner_times_loop_compiles_natively() {
    let block = "3 times:\n    show \"hi\"\nshow \"done\"\n";
    assert_eq!(native_run(block).unwrap(), "hi\nhi\nhi\ndone\n");

    let inline = "3 times: show \"x\"\n";
    assert_eq!(native_run(inline).unwrap(), "x\nx\nx\n");

    let korean = "3번:\n    말해 \"안녕\"\n";
    assert_eq!(native_run(korean).unwrap(), "안녕\n안녕\n안녕\n");
}

#[test]
fn one_line_nme_repeat_bodies_cover_supported_surfaces_and_reject_python_loops() {
    let cases = [
        (
            "sentence-en",
            "repeat 2 times and show Hi\n",
            Some("Hi\nHi\n"),
        ),
        (
            "sentence-ko",
            "2번 반복해서 안녕 말해줘\n",
            Some("안녕\n안녕\n"),
        ),
        (
            "beginner-en",
            "2 times: say \"beginner\"\n",
            Some("beginner\nbeginner\n"),
        ),
        ("beginner-ko", "2번: 말해 \"초급\"\n", Some("초급\n초급\n")),
        (
            "advanced-en",
            "for _ in range(2):\n    print(\"advanced\")\n",
            None,
        ),
        (
            "advanced-ko",
            "for _ in range(2):\n    print(\"고급\")\n",
            None,
        ),
    ];

    for (label, source, expected) in cases {
        if let Some(expected) = expected {
            let actual = native_run(source).unwrap_or_else(|error| {
                panic!("native case failed: {label}: {error}");
            });
            assert_eq!(actual, expected, "native case: {label}");
        } else {
            let problems = nme_native::native_compile(source).unwrap_err();
            assert!(
                problems.iter().any(|problem| {
                    problem
                        .message
                        .contains("the native backend does not support")
                }),
                "native case should stay outside the subset: {label}: {problems:?}"
            );
            assert!(
                problems.iter().any(|problem| {
                    problem
                        .message_ko
                        .as_deref()
                        .is_some_and(|message| message.contains("네이티브 백엔드는 아직"))
                }),
                "native case should have Korean guidance: {label}: {problems:?}"
            );
        }
    }
}

#[test]
fn one_line_nme_break_bodies_work_across_the_native_surface_matrix() {
    let cases = [
        (
            "sentence-en",
            "count = 0\nwhile true\n    count add 1\n    when count == 2 then break here\nend\nshow count\n",
        ),
        (
            "sentence-ko",
            "횟수는 0\n동안 참\n    횟수에 1 더해\n    만약 횟수가 2와 같으면 여기서 멈춰\n끝\n횟수 말해줘\n",
        ),
        (
            "beginner-en",
            "set count to 0\nwhile True\n    count add 1\n    if count == 2 then break\nend\nshow count\n",
        ),
        (
            "beginner-ko",
            "저장 횟수 0\n동안 True\n    횟수에 1 더해\n    만약 횟수 == 2 그러면 멈춰\n끝\n말해 횟수\n",
        ),
        (
            "advanced-en",
            "count = 0\nwhile (True)\n    count = count + 1\n    if (count == 2) then break\nend\nshow count\n",
        ),
        (
            "advanced-ko",
            "횟수 = 0\n동안 ((참 그리고 참))\n    횟수 = 횟수 + 1\n    만약 ((횟수 == 2 그리고 참)) 그러면 멈춰\n끝\n말해 횟수\n",
        ),
    ];

    for (label, source) in cases {
        let actual = native_run(source).unwrap_or_else(|error| {
            panic!("native case failed: {label}: {error}");
        });
        assert_eq!(actual, "2\n", "native case: {label}");
    }

    let inline_loop_cases = [
        (
            "sentence-en",
            "while true then break here\nshow \"done\"\n",
            "done\n",
        ),
        (
            "sentence-ko",
            "동안 참 그러면 여기서 멈춰\n말해 \"끝\"\n",
            "끝\n",
        ),
        (
            "beginner-en",
            "while True then break\nshow \"done\"\n",
            "done\n",
        ),
        (
            "beginner-ko",
            "동안 True 그러면 멈춰\n말해 \"끝\"\n",
            "끝\n",
        ),
        (
            "advanced-en",
            "while (True) then break\nshow \"done\"\n",
            "done\n",
        ),
        (
            "advanced-ko",
            "동안 ((참 그리고 참)) 그러면 멈춰\n말해 \"끝\"\n",
            "끝\n",
        ),
    ];

    for (label, source, expected) in inline_loop_cases {
        let actual = native_run(source).unwrap_or_else(|error| {
            panic!("native case failed: {label}: {error}");
        });
        assert_eq!(actual, expected, "native case: {label}");
    }

    assert_eq!(
        native_run("3 times: break\nshow \"done\"\n").unwrap(),
        "done\n"
    );
    assert_eq!(native_run("3번: 멈춰\n말해 \"끝\"\n").unwrap(), "끝\n");
    assert_eq!(
        native_run("repeat 3 times and break here\nshow \"done\"\n").unwrap(),
        "done\n"
    );
    assert_eq!(
        native_run("3번 반복해서 여기서 멈춰\n말해 \"끝\"\n").unwrap(),
        "끝\n"
    );

    assert_eq!(
        native_run(
            "count = 0\nwhile true\n    count add 1\n    when false\n        show \"never\"\n    else if count == 2 then break\n    end\nend\nshow count\n"
        )
        .unwrap(),
        "2\n"
    );

    for source in ["if True then break\n", "만약 참 그러면 멈춰\n"] {
        let problems = nme_native::native_compile(source).unwrap_err();
        assert!(
            problems.iter().any(|problem| {
                problem.code == DiagnosticCode::BreakOutsideLoop
                    && problem.message.contains("can only be used inside a loop")
            }),
            "{source:?}: {problems:?}"
        );
        assert!(
            problems.iter().any(|problem| {
                problem
                    .message_ko
                    .as_deref()
                    .is_some_and(|message| message.contains("반복문 안에서만"))
            }),
            "{source:?}: {problems:?}"
        );
    }
}

#[test]
fn one_line_nme_else_if_break_bodies_work_across_the_native_surface_matrix() {
    let cases = [
        (
            "sentence-en",
            "count = 0\nwhile true\n    count add 1\n    when false\n        show \"never\"\n    else if count == 2 then break here\n    end\nend\nshow count\n",
        ),
        (
            "sentence-ko",
            "횟수는 0\n동안 참\n    횟수에 1 더해\n    만약 거짓\n        말해 \"안 돼\"\n    아니면 만약에 횟수가 2와 같으면 여기서 멈춰\n    끝\n끝\n횟수 말해줘\n",
        ),
        (
            "beginner-en",
            "set count to 0\nwhile True\n    count add 1\n    if False\n        show \"never\"\n    else if count == 2 then break\n    end\nend\nshow count\n",
        ),
        (
            "beginner-ko",
            "저장 횟수 0\n동안 True\n    횟수에 1 더해\n    만약 거짓\n        말해 \"안 돼\"\n    아니면 만약 횟수 == 2 그러면 멈춰\n    끝\n끝\n말해 횟수\n",
        ),
        (
            "advanced-en",
            "count = 0\nwhile (True)\n    count = count + 1\n    if (False)\n        show \"never\"\n    else if (count == 2) then break\n    end\nend\nshow count\n",
        ),
        (
            "advanced-ko",
            "횟수 = 0\n동안 ((참 그리고 참))\n    횟수 = 횟수 + 1\n    만약 ((거짓 그리고 참))\n        말해 \"안 돼\"\n    아니면 만약에 ((횟수 == 2 그리고 참)) 그러면 멈춰\n    끝\n끝\n말해 횟수\n",
        ),
    ];

    for (label, source) in cases {
        let actual = native_run(source).unwrap_or_else(|error| {
            panic!("native case failed: {label}: {error}");
        });
        assert_eq!(actual, "2\n", "native case: {label}");
    }
}

#[test]
fn boolean_literals_in_truthy_conditions_compile_natively() {
    let source = "if true\n    show \"always\"\nend\nif false\n    show \"never\"\nend\n";
    assert_eq!(native_run(source).unwrap(), "always\n");
}

#[test]
fn boolean_literals_print_with_python_spelling() {
    let english = "show true\nshow false\n";
    assert_eq!(native_run(english).unwrap(), "True\nFalse\n");

    let korean = "말해 참\n말해 거짓\n";
    assert_eq!(native_run(korean).unwrap(), "True\nFalse\n");
}

#[test]
fn boolean_bindings_work_as_native_conditions_across_the_surface_matrix() {
    let cases = [
        (
            "sentence-en",
            "ready save true\nshow ready\nwhen ready\n    show \"yes\"\nend\nready save false\nshow ready\nwhen ready\n    show \"no\"\nend\n",
        ),
        (
            "sentence-ko",
            "준비는 참\n말해 준비\n만약 준비\n    말해 \"예\"\n끝\n준비는 거짓\n말해 준비\n만약 준비\n    말해 \"아니요\"\n끝\n",
        ),
        (
            "beginner-en",
            "set ready to True\nshow ready\nif ready\n    show \"yes\"\nend\nset ready to False\nshow ready\nif ready\n    show \"no\"\nend\n",
        ),
        (
            "beginner-ko",
            "저장 준비 True\n말해 준비\n만약 준비\n    말해 \"예\"\n끝\n저장 준비 False\n말해 준비\n만약 준비\n    말해 \"아니요\"\n끝\n",
        ),
        (
            "advanced-en",
            "ready = True\nshow ready\nif ready\n    show \"yes\"\nend\nready = False\nshow ready\nif ready\n    show \"no\"\nend\n",
        ),
        (
            "advanced-ko",
            "준비 = True\n말해 준비\n만약 준비\n    말해 \"예\"\n끝\n준비 = False\n말해 준비\n만약 준비\n    말해 \"아니요\"\n끝\n",
        ),
    ];

    for (label, source) in cases {
        let expected = if label.ends_with("ko") {
            "True\n예\nFalse\n"
        } else {
            "True\nyes\nFalse\n"
        };
        let output =
            native_run(source).unwrap_or_else(|error| panic!("native case {label}: {error}"));
        assert_eq!(output, expected, "native case: {label}");
    }
}

#[test]
fn boolean_expression_bindings_work_across_the_surface_matrix() {
    let cases = [
        (
            "sentence-en",
            "set ready to 1 == 1\nwhen ready\n    show \"yes\"\nend\n",
        ),
        (
            "sentence-ko",
            "준비는 1 == 1\n만약 준비\n    말해 \"예\"\n끝\n",
        ),
        (
            "beginner-en",
            "save ready to 1 == 1\nif ready\n    show \"yes\"\nend\n",
        ),
        (
            "beginner-ko",
            "저장 준비 1 == 1\n만약 준비\n    말해 \"예\"\n끝\n",
        ),
        (
            "advanced-en",
            "ready = 1 == 1\nif ready\n    show \"yes\"\nend\n",
        ),
        (
            "advanced-ko",
            "준비 = 1 == 1\n만약 준비\n    말해 \"예\"\n끝\n",
        ),
    ];

    for (label, source) in cases {
        let expected = if label.ends_with("ko") {
            "예\n"
        } else {
            "yes\n"
        };
        let output =
            native_run(source).unwrap_or_else(|error| panic!("native case {label}: {error}"));
        assert_eq!(output, expected, "native case: {label}");
    }
}

#[test]
fn native_booleans_remain_distinct_from_integer_values() {
    for source in [
        "ready = True\nshow ready + 1\n",
        "준비 = True\n말해 준비 + 1\n",
    ] {
        let problems = nme_native::native_compile(source).unwrap_err();
        assert!(
            problems
                .iter()
                .any(|problem| problem.message.contains("a boolean in arithmetic")),
            "{source:?}: {problems:?}"
        );
        assert!(
            problems.iter().any(|problem| {
                problem
                    .message_ko
                    .as_deref()
                    .is_some_and(|message| message.contains("불리언"))
            }),
            "{source:?}: {problems:?}"
        );
    }

    for source in [
        "ready = True\nready add 1\n",
        "준비 = True\n준비에 1 더해\n",
    ] {
        let problems = nme_native::native_compile(source).unwrap_err();
        assert!(
            problems
                .iter()
                .any(|problem| problem.message.contains("changing a boolean value")),
            "{source:?}: {problems:?}"
        );
    }

    for source in ["ready = True\nready = 1\n", "준비 = True\n준비 = 1\n"] {
        let problems = nme_native::native_compile(source).unwrap_err();
        assert!(
            problems
                .iter()
                .any(|problem| problem.message.contains("changing the type")),
            "{source:?}: {problems:?}"
        );
        assert!(
            problems.iter().any(|problem| {
                problem
                    .message_ko
                    .as_deref()
                    .is_some_and(|message| message.contains("타입 변경"))
            }),
            "{source:?}: {problems:?}"
        );
    }

    for source in [
        "def identity(value):\n    return value\n\nshow identity(True)\n",
        "def 준비(값):\n    return 값\n\n말해 준비(True)\n",
        "def ready():\n    return True\n\nshow ready()\n",
        "def 준비됨():\n    return True\n\n말해 준비됨()\n",
    ] {
        let problems = nme_native::native_compile(source).unwrap_err();
        assert!(
            problems
                .iter()
                .any(|problem| { problem.message.contains("accept and return integer values") }),
            "{source:?}: {problems:?}"
        );
    }
}

#[test]
fn boolean_bindings_work_through_native_loops_and_branch_merges() {
    let english = "ready = True\nsame = ready == True\ndifferent = ready != False\nshow same\nshow different\nwhile ready\n    show \"once\"\n    ready = False\nend\nflag = True\nif flag\n    result = True\nelse\n    result = False\nend\nshow result\n";
    assert_eq!(native_run(english).unwrap(), "True\nTrue\nonce\nTrue\n");

    let korean = "준비 = True\n같음 = 준비 == True\n다름 = 준비 != False\n말해 같음\n말해 다름\n동안 준비\n    말해 \"한 번\"\n    준비 = False\n끝\n표시 = True\n만약 표시\n    저장 결과 참\n아니면\n    저장 결과 거짓\n끝\n말해 결과\n";
    assert_eq!(native_run(korean).unwrap(), "True\nTrue\n한 번\nTrue\n");
}

#[test]
fn logical_conditions_compile_across_the_native_surface_matrix() {
    let cases = [
        (
            "sentence-en",
            "ready save true\nscore save 3\nwhen ready and score is greater than 2\n    show \"yes\"\nend\nwhen false or ready\n    show \"or\"\nend\n",
            "yes\nor\n",
        ),
        (
            "sentence-ko",
            "준비는 참\n점수는 3\n만약 준비 그리고 점수가 2보다 크면\n    말해 \"예\"\n끝\n만약 거짓 또는 준비\n    말해 \"또는\"\n끝\n",
            "예\n또는\n",
        ),
        (
            "beginner-en",
            "set ready to True\nscore = 3\nif ready and score > 2\n    show \"yes\"\nend\nif False or ready\n    show \"or\"\nend\n",
            "yes\nor\n",
        ),
        (
            "beginner-ko",
            "저장 준비 True\n점수 = 3\n만약 준비 그리고 점수 > 2\n    말해 \"예\"\n끝\n만약 거짓 또는 준비\n    말해 \"또는\"\n끝\n",
            "예\n또는\n",
        ),
        (
            "advanced-en",
            "ready = True\nscore = 3\nwhen ready and score > 2\n    show \"yes\"\nend\nwhen False or ready\n    show \"or\"\nend\n",
            "yes\nor\n",
        ),
        (
            "advanced-ko",
            "준비 = True\n점수 = 3\n만약 준비 그리고 점수 > 2\n    말해 \"예\"\n끝\n만약 거짓 또는 준비\n    말해 \"또는\"\n끝\n",
            "예\n또는\n",
        ),
    ];

    for (label, source, expected) in cases {
        assert_eq!(
            native_run(source).unwrap(),
            expected,
            "native case: {label}"
        );
    }
}

#[test]
fn logical_conditions_work_in_native_while_loops_across_the_surface_matrix() {
    let cases = [
        (
            "sentence-en",
            "ready save true\nturns save 0\nwhile ready and turns is less than 2\n    turns add 1\n    if turns equals 2\n        ready save false\n    end\nend\nshow turns\n",
            "2\n",
        ),
        (
            "sentence-ko",
            "준비는 참\n횟수는 0\n동안 준비 그리고 횟수가 2보다 작을 동안\n    횟수에 1 더해\n    만약 횟수가 2와 같으면\n        준비는 거짓\n    끝\n끝\n횟수 말해줘\n",
            "2\n",
        ),
        (
            "beginner-en",
            "set ready to True\nturns = 0\nwhile ready and turns < 2\n    turns add 1\n    if turns == 2\n        set ready to False\n    end\nend\nshow turns\n",
            "2\n",
        ),
        (
            "beginner-ko",
            "저장 준비 True\n횟수 = 0\n동안 준비 그리고 횟수 < 2\n    횟수에 1 더해\n    만약 횟수 == 2\n        저장 준비 False\n    끝\n끝\n말해 횟수\n",
            "2\n",
        ),
        (
            "advanced-en",
            "ready = True\nturns = 0\nwhile ready and turns < 2\n    turns add 1\n    if turns == 2\n        ready = False\n    end\nend\nshow turns\n",
            "2\n",
        ),
        (
            "advanced-ko",
            "준비 = True\n횟수 = 0\n동안 준비 그리고 횟수 < 2\n    횟수에 1 더해\n    만약 횟수 == 2\n        준비 = False\n    끝\n끝\n말해 횟수\n",
            "2\n",
        ),
    ];

    for (label, source, expected) in cases {
        assert_eq!(
            native_run(source).unwrap(),
            expected,
            "native case: {label}"
        );
    }
}

#[test]
fn parenthesized_logical_conditions_compile_across_the_native_surface_matrix() {
    let cases = [
        (
            "sentence-en",
            "ready save true\nscore save 3\nif (ready and score > 2)\n    show \"yes\"\nend\n",
            "yes\n",
        ),
        (
            "sentence-ko",
            "준비는 참\n점수는 3\n만약 (준비 그리고 점수 > 2)\n    말해 \"예\"\n끝\n",
            "예\n",
        ),
        (
            "beginner-en",
            "set ready to True\nscore = 3\nif (ready and score > 2)\n    show \"yes\"\nend\n",
            "yes\n",
        ),
        (
            "beginner-ko",
            "저장 준비 True\n점수 = 3\n만약 (준비 그리고 점수 > 2)\n    말해 \"예\"\n끝\n",
            "예\n",
        ),
        (
            "advanced-en",
            "ready = True\nscore = 3\nif ((ready and score > 2))\n    show \"yes\"\nend\n",
            "yes\n",
        ),
        (
            "advanced-ko",
            "준비 = True\n점수 = 3\n만약 ((준비 그리고 점수 > 2))\n    말해 \"예\"\n끝\n",
            "예\n",
        ),
    ];

    for (label, source, expected) in cases {
        let actual = native_run(source).unwrap_or_else(|error| {
            panic!("native case failed: {label}: {error}");
        });
        assert_eq!(actual, expected, "native case: {label}");
    }
}

#[test]
fn one_line_nme_control_bodies_compile_across_the_native_surface_matrix() {
    let cases = [
        (
            "sentence-en",
            "ready save true\nif ready then show \"yes\"\n",
            "yes\n",
        ),
        (
            "sentence-ko",
            "준비는 참\n만약 준비 그러면 말해 \"예\"\n",
            "예\n",
        ),
        (
            "beginner-en",
            "set ready to True\nif ready then show \"yes\"\n",
            "yes\n",
        ),
        (
            "beginner-ko",
            "저장 준비 True\n만약 준비 그러면 말해 \"예\"\n",
            "예\n",
        ),
        (
            "advanced-en",
            "ready = True\nif (ready) then show \"yes\"\n",
            "yes\n",
        ),
        (
            "advanced-ko",
            "준비 = True\n만약 (준비) 그러면 말해 \"예\"\n",
            "예\n",
        ),
    ];

    for (label, source, expected) in cases {
        let actual = native_run(source).unwrap_or_else(|error| {
            panic!("native case failed: {label}: {error}");
        });
        assert_eq!(actual, expected, "native case: {label}");
    }
}

#[test]
fn one_line_nme_while_bodies_compile_across_the_native_surface_matrix() {
    let cases = [
        (
            "sentence-en",
            "turns save 0\nwhile turns is greater than 0 then show \"never\"\nshow turns\n",
            "0\n",
        ),
        (
            "sentence-ko",
            "횟수는 0\n동안 횟수가 0보다 크면 횟수 말해줘\n횟수 말해줘\n",
            "0\n",
        ),
        (
            "beginner-en",
            "set turns to 0\nwhile turns > 0 then show \"never\"\nshow turns\n",
            "0\n",
        ),
        (
            "beginner-ko",
            "저장 횟수 0\n동안 횟수 > 0 그러면 말해 \"안 돼\"\n말해 횟수\n",
            "0\n",
        ),
        (
            "advanced-en",
            "turns = 0\nwhile (turns > 0) then show \"never\"\nshow turns\n",
            "0\n",
        ),
        (
            "advanced-ko",
            "횟수 = 0\n동안 (횟수 > 0) 그러면 말해 \"안 돼\"\n말해 횟수\n",
            "0\n",
        ),
    ];

    for (label, source, expected) in cases {
        let actual = native_run(source).unwrap_or_else(|error| {
            panic!("native case failed: {label}: {error}");
        });
        assert_eq!(actual, expected, "native case: {label}");
    }
}

#[test]
fn one_line_nme_branch_bodies_compile_across_the_native_surface_matrix() {
    let cases = [
        (
            "sentence-en",
            "when false\n    show \"no\"\nelse if true then show \"middle\"\nelse show \"fallback\"\nend\nwhen false\n    show \"no\"\nelse if false then show \"never\"\nelse show \"fallback\"\nend\n",
            "middle\nfallback\n",
        ),
        (
            "sentence-ko",
            "만약 거짓\n    안 돼 말해줘\n아니면 만약에 참 그러면 중간 말해줘\n아니면 대체 말해줘\n끝\n만약 거짓\n    안 돼 말해줘\n아니면 만약에 거짓 그러면 안 돼 말해줘\n아니면 대체 말해줘\n끝\n",
            "중간\n대체\n",
        ),
        (
            "beginner-en",
            "set ready to False\nif ready\n    show \"no\"\nelse if True then show \"middle\"\nelse show \"fallback\"\nend\nif ready\n    show \"no\"\nelse if False then show \"never\"\nelse show \"fallback\"\nend\n",
            "middle\nfallback\n",
        ),
        (
            "beginner-ko",
            "저장 준비 거짓\n만약 준비\n    말해 \"아니\"\n아니면 만약에 참 그러면 말해 \"중간\"\n아니면 말해 \"대체\"\n끝\n만약 준비\n    말해 \"아니\"\n아니면 만약에 거짓 그러면 말해 \"안 돼\"\n아니면 말해 \"대체\"\n끝\n",
            "중간\n대체\n",
        ),
        (
            "advanced-en",
            "ready = False\nif (ready)\n    show \"no\"\nelse if (True) then show \"middle\"\nelse show \"fallback\"\nend\nif (ready)\n    show \"no\"\nelse if (False) then show \"never\"\nelse show \"fallback\"\nend\n",
            "middle\nfallback\n",
        ),
        (
            "advanced-ko",
            "준비 = False\n만약 ((준비 그리고 참))\n    말해 \"아니\"\n아니면 만약에 (True) 그러면 말해 \"중간\"\n아니면 말해 \"대체\"\n끝\n만약 ((준비 그리고 참))\n    말해 \"아니\"\n아니면 만약에 (False) 그러면 말해 \"안 돼\"\n아니면 말해 \"대체\"\n끝\n",
            "중간\n대체\n",
        ),
    ];

    for (label, source, expected) in cases {
        let actual = native_run(source).unwrap_or_else(|error| {
            panic!("native case failed: {label}: {error}");
        });
        assert_eq!(actual, expected, "native case: {label}");
    }
}

#[test]
fn one_line_nme_say_bodies_compile_across_the_native_surface_matrix() {
    let cases = [
        (
            "sentence-en",
            "when true then say \"sentence\"\n",
            "sentence\n",
        ),
        ("sentence-ko", "만약 참 그러면 say \"문장\"\n", "문장\n"),
        (
            "beginner-en",
            "if True then say \"beginner\"\n",
            "beginner\n",
        ),
        (
            "beginner-ko",
            "저장 준비 참\n만약 준비 그러면 say \"초급\"\n",
            "초급\n",
        ),
        (
            "advanced-en",
            "if (True) then say \"advanced\"\n",
            "advanced\n",
        ),
        (
            "advanced-ko",
            "준비 = True\n만약 ((준비 그리고 참)) 그러면 say \"고급\"\n",
            "고급\n",
        ),
    ];

    for (label, source, expected) in cases {
        let actual = native_run(source).unwrap_or_else(|error| {
            panic!("native case failed: {label}: {error}");
        });
        assert_eq!(actual, expected, "native case: {label}");
    }
}

#[test]
fn native_control_bodies_reject_python_inline_statements() {
    for source in [
        "ready = True\nif ready then print(\"yes\")\n",
        "준비 = True\n만약 준비 그러면 print(\"예\")\n",
    ] {
        let problems = nme_native::native_compile(source).unwrap_err();
        assert!(
            problems
                .iter()
                .any(|problem| problem.message.contains("this inline body")),
            "{source:?}: {problems:?}"
        );
    }
}

#[test]
fn parenthesized_korean_comparison_endings_compile_natively() {
    let source = "점수는 1\n만약 (점수가 2보다 작으면)\n    작아요 말해줘\n끝\n";
    assert_eq!(native_run(source).unwrap(), "작아요\n");

    let branch = "점수는 3\n만약 거짓\n    안 돼 말해줘\n아니면 만약에 (점수가 4보다 작으면)\n    작아요 말해줘\n끝\n";
    assert_eq!(native_run(branch).unwrap(), "작아요\n");

    let logical_branch = "점수는 3\n준비는 참\n만약 거짓\n    안 돼 말해줘\n아니면 만약에 (점수가 2보다 크면 그리고 준비)\n    성공 말해줘\n끝\n";
    assert_eq!(native_run(logical_branch).unwrap(), "성공\n");

    let logical =
        "점수는 3\n준비는 참\n만약 (점수가 2보다 크면 그리고 준비)\n    성공 말해줘\n끝\n";
    assert_eq!(native_run(logical).unwrap(), "성공\n");

    let mixed = "점수는 3\n준비는 참\n만약 (점수가 2보다 크면 and 준비)\n    성공 말해줘\n끝\n";
    assert_eq!(native_run(mixed).unwrap(), "성공\n");

    let inline = "점수는 3\n준비는 참\n만약 (점수가 2보다 크면 그리고 준비) 그러면 성공 말해줘\n";
    assert_eq!(native_run(inline).unwrap(), "성공\n");

    let inline_branch = "점수는 3\n준비는 참\n만약 거짓\n    안 돼 말해줘\n아니면 만약에 (점수가 2보다 크면 그리고 준비) 그러면 성공 말해줘\n아니면 끝났어 말해줘\n끝\n";
    assert_eq!(native_run(inline_branch).unwrap(), "성공\n");

    let disjunction =
        "점수는 3\n준비는 거짓\n만약 (점수가 2보다 작으면 또는 준비)\n    실패 말해줘\n끝\n";
    assert_eq!(native_run(disjunction).unwrap(), "");

    let mixed_disjunction =
        "점수는 3\n준비는 거짓\n만약 (점수가 2보다 작으면 or 준비)\n    실패 말해줘\n끝\n";
    assert_eq!(native_run(mixed_disjunction).unwrap(), "");
}

#[test]
fn parenthesized_korean_while_endings_compile_natively() {
    let korean = "준비는 참\n횟수는 0\n동안 (횟수가 2보다 작을 동안 그리고 준비)\n    횟수에 1 더해\n끝\n횟수 말해줘\n";
    assert_eq!(native_run(korean).unwrap(), "2\n");

    let english_keyword = "준비 = True\n횟수 = 0\nwhile (횟수가 2보다 작을 동안 그리고 준비)\n    횟수 add 1\nend\nshow 횟수\n";
    assert_eq!(native_run(english_keyword).unwrap(), "2\n");

    let disjunction = "준비는 거짓\n횟수는 0\n동안 (횟수가 2보다 작을 동안 또는 준비)\n    횟수에 1 더해\n끝\n횟수 말해줘\n";
    assert_eq!(native_run(disjunction).unwrap(), "2\n");

    let inline = "횟수는 0\n동안 횟수가 0보다 크면 횟수 말해줘\n횟수 말해줘\n";
    assert_eq!(native_run(inline).unwrap(), "0\n");
}

#[test]
fn parenthesized_logical_conditions_work_in_native_while_loops_across_the_surface_matrix() {
    let cases = [
        (
            "sentence-en",
            "ready save true\nturns save 0\nwhile (ready and turns is less than 2)\n    turns add 1\n    if turns equals 2\n        ready save false\n    end\nend\nshow turns\n",
            "2\n",
        ),
        (
            "sentence-ko",
            "준비는 참\n횟수는 0\n동안 (준비 그리고 횟수가 2보다 작을 동안)\n    횟수에 1 더해\n    만약 횟수가 2와 같으면\n        준비는 거짓\n    끝\n끝\n횟수 말해줘\n",
            "2\n",
        ),
        (
            "beginner-en",
            "set ready to True\nturns = 0\nwhile (ready and turns < 2)\n    turns add 1\n    if turns == 2\n        set ready to False\n    end\nend\nshow turns\n",
            "2\n",
        ),
        (
            "beginner-ko",
            "저장 준비 True\n횟수 = 0\n동안 (준비 그리고 횟수 < 2)\n    횟수에 1 더해\n    만약 횟수 == 2\n        저장 준비 False\n    끝\n끝\n말해 횟수\n",
            "2\n",
        ),
        (
            "advanced-en",
            "ready = True\nturns = 0\nwhile (ready and turns < 2)\n    turns add 1\n    if turns == 2\n        ready = False\n    end\nend\nshow turns\n",
            "2\n",
        ),
        (
            "advanced-ko",
            "준비 = True\n횟수 = 0\nwhile (준비 그리고 횟수 < 2)\n    횟수에 1 더해\n    만약 횟수 == 2\n        준비 = False\n    끝\n끝\n말해 횟수\n",
            "2\n",
        ),
    ];

    for (label, source, expected) in cases {
        let actual = native_run(source).unwrap_or_else(|error| {
            panic!("native case failed: {label}: {error}");
        });
        assert_eq!(actual, expected, "native case: {label}");
    }
}

#[test]
fn logical_conditions_keep_short_circuit_evaluation() {
    let english = "def mark():\n    show \"called\"\n    return 1\n\nif true or false and false\n    show \"precedence\"\nend\nif false and mark() == 1\n    show \"bad and\"\nend\nif true or mark() == 1\n    show \"short\"\nend\n";
    assert_eq!(native_run(english).unwrap(), "precedence\nshort\n");

    let korean = "def 표시():\n    말해 \"호출\"\n    return 1\n\n만약 참 또는 거짓 그리고 거짓\n    말해 \"우선순위\"\n끝\n만약 거짓 그리고 표시() == 1\n    말해 \"잘못된 그리고\"\n끝\n만약 참 또는 표시() == 1\n    말해 \"짧게\"\n끝\n";
    assert_eq!(native_run(korean).unwrap(), "우선순위\n짧게\n");
}

#[test]
fn logical_conditions_mix_languages_and_syntax_levels() {
    let mixed = "ready save true\nscore = 3\n만약 ready and score > 2 또는 거짓\n    show \"혼합 우선순위\"\n끝\nwhen true or false 그리고 false\n    말해 \"precedence\"\nend\n";
    assert_eq!(native_run(mixed).unwrap(), "혼합 우선순위\nprecedence\n");
}

#[test]
fn truthy_conditions_compile_natively() {
    let source = "ready = 1\nif ready\n    show \"ready yes\"\nend\nready = 0\nif ready\n    show \"no\"\nend\nturns = 3\nwhile turns\n    show turns\n    turns add -1\nend\n";
    assert_eq!(native_run(source).unwrap(), "ready yes\n3\n2\n1\n");
}

#[test]
fn finite_float_truthiness_compiles_in_english_and_korean() {
    let english = "value = 0.5\nif value\n    show \"positive\"\nend\nvalue = 0.0\nif value\n    show \"zero\"\nend\n";
    assert_eq!(native_run(english).unwrap(), "positive\n");

    let korean =
        "값 = 0.5\n만약 값\n    말해 \"양수\"\n끝\n값 = 0.0\n만약 값\n    말해 \"영\"\n끝\n";
    assert_eq!(native_run(korean).unwrap(), "양수\n");
}

#[test]
fn string_concat_at_first_assignment_compiles_natively() {
    let source = "greeting = \"hello\" + \" world\"\nshow greeting\nname = \"NME\"\nname = name + \" rocks\"\nshow name\n";
    assert_eq!(native_run(source).unwrap(), "hello world\nNME rocks\n");
}

#[test]
fn string_concat_into_variables_works() {
    let source = "greeting = \"hello\"\ngreeting = greeting + \" world\"\nshow greeting\nname = \"NME\"\nname = \"great \" + name\nshow name\n";
    assert_eq!(native_run(source).unwrap(), "hello world\ngreat NME\n");
}

#[test]
fn oversized_string_values_fail_without_buffer_overflow() {
    let literal = "x".repeat(8192);
    let source = format!("text = \"{literal}\"\nshow text\n");
    let error = native_run(&source).expect_err("an oversized string must fail cleanly");
    assert!(error.contains("string value exceeds 8191 bytes"), "{error}");

    let prefix = "x".repeat(8191);
    let concat_source = format!("text = \"{prefix}\"\nshow text + \"y\"\n");
    let concat_error =
        native_run(&concat_source).expect_err("an oversized concatenation must fail cleanly");
    assert!(
        concat_error.contains("string value exceeds 8191 bytes"),
        "{concat_error}"
    );
}

#[test]
fn nested_string_concat_is_rejected_not_miscompiled() {
    assert!(native_rejects(
        "greeting = \"hi\"\nshow greeting + \" \" + \"friend\"\n"
    ));
}

#[test]
fn c_keyword_names_are_rejected_not_miscompiled() {
    assert!(native_rejects("double = 3\nshow double\n"));
    assert!(native_rejects(
        "def double(n):\n    return n\nshow double(2)\n"
    ));
}

#[test]
fn c_implementation_reserved_names_are_rejected_not_miscompiled() {
    for source in [
        "__attribute__ = 1\nshow __attribute__\n",
        "__속성 = 1\n말해 __속성\n",
        "def _Foo(value):\n    return value\n\nshow _Foo(1)\n",
        "def _helper(value):\n    return value\n\nshow _helper(1)\n",
        "def _함수(값):\n    return 값\n\n말해 _함수(1)\n",
    ] {
        let problems = nme_native::native_compile(source).unwrap_err();
        assert!(
            problems.iter().any(|problem| {
                problem
                    .message
                    .contains("C implementation-reserved identifier")
            }),
            "{source:?}: {problems:?}"
        );
        assert!(
            problems.iter().any(|problem| {
                problem
                    .message_ko
                    .as_deref()
                    .is_some_and(|message| message.contains("C 구현 예약 식별자"))
            }),
            "{source:?}: {problems:?}"
        );
    }
    assert_eq!(native_run("_value = 1\nshow _value\n").unwrap(), "1\n");
}

#[test]
fn native_runtime_names_are_rejected_not_miscompiled() {
    for name in [
        "NME_UNUSED",
        "NME_STRING_CAPACITY",
        "_nme_i",
        "INT_MAX",
        "INT_MIN",
        "DBL_MAX",
        "EOF",
        "NULL",
        "FILE",
        "malloc",
        "free",
        "strncat",
        "abs",
        "nme_add_int",
        "nme_add_float",
        "nme_cat",
        "nme_copy",
        "nme_cat_index",
        "nme_float_result",
        "nme_integer_division_by_zero",
        "nme_integer_overflow",
        "nme_len",
        "nme_mod_int",
        "nme_mul_int",
        "nme_mul_float",
        "nme_neg_int",
        "nme_non_finite_float",
        "nme_sub_float",
        "nme_sub_int",
        "printf",
        "len",
    ] {
        let source = format!("{name} = 1\nshow {name}\n");
        let problems = nme_native::native_compile(&source).unwrap_err();
        assert!(
            problems
                .iter()
                .any(|problem| problem.message.contains("reserved native runtime name")),
            "{name}: {problems:?}"
        );
        assert!(
            problems.iter().any(|problem| {
                problem
                    .message_ko
                    .as_deref()
                    .is_some_and(|message| message.contains("네이티브 런타임 예약 이름"))
            }),
            "{name}: {problems:?}"
        );
    }
}

#[test]
fn native_runtime_names_are_rejected_in_function_parameters() {
    for name in [
        "NME_UNUSED",
        "NME_STRING_CAPACITY",
        "_nme_i",
        "nme_copy",
        "printf",
        "len",
    ] {
        let source = format!("def identity({name}):\n    return 1\n\nshow identity(1)\n");
        let problems = nme_native::native_compile(&source).unwrap_err();
        assert!(
            problems
                .iter()
                .any(|problem| problem.message.contains("reserved native runtime name")),
            "{name}: {problems:?}"
        );
        assert!(
            problems.iter().any(|problem| {
                problem
                    .message_ko
                    .as_deref()
                    .is_some_and(|message| message.contains("네이티브 런타임 예약 이름"))
            }),
            "{name}: {problems:?}"
        );
    }
}

#[test]
fn a_flat_block_body_gets_virtual_indentation() {
    let source = "x = 0\nwhile x is less than 2\nx add 1\nend\nshow x\n";
    assert_eq!(native_run(source).unwrap(), "2\n");
}

#[test]
fn korean_spellings_compile_natively() {
    let source = "점수 = 0\n동안 점수가 3보다 작을 동안\n    점수에 1 더해\n끝\n점수 말해줘\n";
    assert_eq!(native_run(source).unwrap(), "3\n");
}

#[test]
fn native_surface_acceptance_matrix_runs_all_six_forms() {
    let cases = [
        (
            "sentence-en",
            "score = 5\nwhile score is less than 10\n    score add 1\nend\nshow score\n",
            "10\n",
        ),
        (
            "sentence-ko",
            "점수 = 5\n동안 점수가 10보다 작을 동안\n    점수에 1 더해\n끝\n점수 말해줘\n",
            "10\n",
        ),
        (
            "beginner-en",
            "score = 5\n5 times:\n    score add 1\nshow score\n",
            "10\n",
        ),
        (
            "beginner-ko",
            "점수 = 5\n5번:\n    점수에 1 더해\n점수 말해줘\n",
            "10\n",
        ),
        (
            "advanced-en",
            "def twice(value):\n    return value * 2\n\nshow twice(5)\n",
            "10\n",
        ),
        (
            "advanced-ko",
            "def 두배(값):\n    return 값 * 2\n\n말해 두배(5)\n",
            "10\n",
        ),
    ];

    for (label, source, expected) in cases {
        assert_eq!(
            native_run(source).unwrap(),
            expected,
            "native case: {label}"
        );
    }
}

#[test]
fn unreachable_true_branch_alternatives_do_not_export_bindings() {
    for source in [
        "if true\n    show \"yes\"\nelse\n    hidden = 1\nend\nvalue = hidden\nshow value\n",
        "만약 True라면\n    말해 \"예\"\n아니면\n    숨김 = 1\n끝\n값 = 숨김\n말해 값\n",
    ] {
        let problems = nme_native::native_compile(source).unwrap_err();
        assert!(
            problems
                .iter()
                .any(|problem| problem.message.contains("without a prior native binding")),
            "{source:?}: {problems:?}"
        );
        assert!(
            problems.iter().any(|problem| {
                problem
                    .message_ko
                    .as_deref()
                    .is_some_and(|message| message.contains("먼저 네이티브 바인딩"))
            }),
            "{source:?}: {problems:?}"
        );
    }
}

#[test]
fn sibling_native_branches_do_not_share_bindings() {
    for source in [
        "ready = 0\nif ready\n    hidden = 1\nelse\n    value = hidden\nend\nshow value\n",
        "준비 = 0\n만약 준비 하면\n    숨김 = 1\n아니면\n    값 = 숨김\n끝\n말해 값\n",
    ] {
        let problems = nme_native::native_compile(source).unwrap_err();
        assert!(
            problems
                .iter()
                .any(|problem| problem.message.contains("without a prior native binding")),
            "{source:?}: {problems:?}"
        );
        assert!(
            problems.iter().any(|problem| {
                problem
                    .message_ko
                    .as_deref()
                    .is_some_and(|message| message.contains("먼저 네이티브 바인딩"))
            }),
            "{source:?}: {problems:?}"
        );
    }
}

#[test]
fn a_python_colon_while_header_is_rejected_not_miscompiled() {
    // `while x < 3:` is valid Python, so it stays Python (Python-wins) and
    // the native core, which lowers the sentence `while` form, rejects it.
    assert!(native_rejects(
        "x = 0\nwhile x < 3:\n    x = x + 1\nshow x\n"
    ));
}

#[test]
fn a_rejected_line_reports_its_own_source_position() {
    // The diagnostic must point at the offending line, not at the first
    // line of the file (a real bug fixed after the native core shipped).
    let source = "x = 1\nprint(\"hi\")\n";
    let problems = nme_native::native_compile(source).unwrap_err();
    assert_eq!(problems.len(), 1);
    let span = problems[0].span;
    assert_eq!(
        &source[span.start..span.end],
        "print(\"hi\")",
        "the diagnostic must span the offending Python line"
    );
}

#[test]
fn input_and_modules_are_rejected_not_miscompiled() {
    assert!(native_rejects("ask name, \"name? \"\n"));
    assert!(native_rejects(
        "use random latest\nshow random_number(1, 6)\n"
    ));
    assert!(native_rejects("from \"helper.nme\" import greet\n"));
}

#[test]
fn unsupported_native_hint_lists_finite_float_values() {
    let problems = nme_native::native_compile("ask name, \"name? \"\n").unwrap_err();
    assert!(
        problems.iter().any(|problem| {
            problem
                .hint
                .as_deref()
                .is_some_and(|hint| hint.contains("finite-float"))
        }),
        "{problems:?}"
    );
    assert!(
        problems.iter().any(|problem| {
            problem
                .hint_ko
                .as_deref()
                .is_some_and(|hint| hint.contains("유한 실수"))
        }),
        "{problems:?}"
    );
}
