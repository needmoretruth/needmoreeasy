//! End-to-end tests for the NME-native backend: compile the core subset to
//! C, build it with the system C compiler, run the native executable, and
//! compare its output with the expected text. Programs outside the core
//! subset must be rejected with a clear diagnostic, never miscompiled.

use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

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
    let exe = dir.join("program");
    let status = Command::new("cc")
        .arg("-O2")
        .arg(&c_path)
        .arg("-o")
        .arg(&exe)
        .status()
        .map_err(|error| format!("could not start cc: {error}"))?;
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
        (
            "x = -2147483648\nshow x - 1\n",
            "integer overflow",
        ),
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
            problems.iter().any(|problem| {
                problem
                    .message
                    .contains("accept and return integer values")
            }),
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
            problems.iter().any(|problem| problem.message.contains(english)),
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
        problems.iter().any(|problem| {
            problem
                .message
                .contains("embedded NUL characters")
        }),
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
fn functions_over_scalars_compile_natively() {
    let source = "def twice(n):\n    return n * 2\n\nshow twice(5)\nshow twice(21)\n";
    assert_eq!(native_run(source).unwrap(), "10\n42\n");
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
fn string_comparison_and_len_compile_natively() {
    let source = "name = \"NME\"\nif name == \"NME\"\n    show \"match\"\nend\nif name != \"other\"\n    show \"different\"\nend\nshow len(name)\nshow len(\"hello\")\n";
    assert_eq!(native_run(source).unwrap(), "match\ndifferent\n3\n5\n");

    let korean = "이름 = \"안녕\"\n만약 이름이 \"안녕\"와 같으면\n    말해 \"같아요\"\n끝\n";
    assert_eq!(native_run(korean).unwrap(), "같아요\n");
    assert_eq!(native_run("말해 len(\"안녕\")\n").unwrap(), "2\n");
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
fn boolean_literals_in_truthy_conditions_compile_natively() {
    let source = "if true\n    show \"always\"\nend\nif false\n    show \"never\"\nend\n";
    assert_eq!(native_run(source).unwrap(), "always\n");
}

#[test]
fn truthy_conditions_compile_natively() {
    let source = "ready = 1\nif ready\n    show \"ready yes\"\nend\nready = 0\nif ready\n    show \"no\"\nend\nturns = 3\nwhile turns\n    show turns\n    turns add -1\nend\n";
    assert_eq!(native_run(source).unwrap(), "ready yes\n3\n2\n1\n");
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
fn native_runtime_names_are_rejected_not_miscompiled() {
    for name in [
        "NME_STRING_CAPACITY",
        "_nme_i",
        "INT_MAX",
        "INT_MIN",
        "EOF",
        "NULL",
        "FILE",
        "malloc",
        "free",
        "strncat",
        "abs",
        "nme_add_int",
        "nme_cat",
        "nme_copy",
        "nme_integer_division_by_zero",
        "nme_integer_overflow",
        "nme_len",
        "nme_mod_int",
        "nme_mul_int",
        "nme_neg_int",
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
    for name in ["NME_STRING_CAPACITY", "_nme_i", "nme_copy", "printf", "len"] {
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
