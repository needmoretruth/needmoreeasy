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
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn native_rejects(source: &str) -> bool {
    nme_native::native_compile(source).is_err()
}

#[test]
fn a_while_loop_countdown_runs_natively() {
    assert_eq!(
        native_run("score = 0\nwhile score is less than 3\n    score add 1\nend\nshow score\n").unwrap(),
        "3\n"
    );
}

#[test]
fn arithmetic_in_say_lowers_to_c() {
    assert_eq!(native_run("x = 2\nshow x * 3 + 1\n").unwrap(), "7\n");
}

#[test]
fn a_string_literal_is_printed() {
    assert_eq!(native_run("show \"hello world\"\n").unwrap(), "hello world\n");
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
fn else_and_else_if_branches_compile_natively() {
    let small = "score = 3\nif score is greater than 5\n    show \"big\"\n아니면\n    show \"small\"\n끝\n";
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
    assert_eq!(
        native_run(source).unwrap(),
        "match\ndifferent\n3\n5\n"
    );

    let korean = "이름 = \"안녕\"\n만약 이름이 \"안녕\"와 같으면\n    말해 \"같아요\"\n끝\n";
    assert_eq!(native_run(korean).unwrap(), "같아요\n");
}

#[test]
fn float_literals_arithmetic_and_conditions_compile_natively() {
    let source = "pi = 3.14\nshow pi\nshow 1 + 0.5\nr = 2\nshow 3.14 * r * r\nif pi is greater than 3\n    show \"pi big\"\nend\n";
    assert_eq!(native_run(source).unwrap(), "3.14\n1.5\n12.56\npi big\n");
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
fn nested_string_concat_is_rejected_not_miscompiled() {
    assert!(native_rejects("greeting = \"hi\"\nshow greeting + \" \" + \"friend\"\n"));
}

#[test]
fn c_keyword_names_are_rejected_not_miscompiled() {
    assert!(native_rejects("double = 3\nshow double\n"));
    assert!(native_rejects("def double(n):\n    return n\nshow double(2)\n"));
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
fn a_python_colon_while_header_is_rejected_not_miscompiled() {
    // `while x < 3:` is valid Python, so it stays Python (Python-wins) and
    // the native core, which lowers the sentence `while` form, rejects it.
    assert!(native_rejects("x = 0\nwhile x < 3:\n    x = x + 1\nshow x\n"));
}

#[test]
fn input_and_modules_are_rejected_not_miscompiled() {
    assert!(native_rejects("ask name, \"name? \"\n"));
    assert!(native_rejects("use random latest\nshow random_number(1, 6)\n"));
    assert!(native_rejects("from \"helper.nme\" import greet\n"));
}
