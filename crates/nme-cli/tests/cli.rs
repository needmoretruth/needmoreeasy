//! Integration tests for the `nme` command line tool, including real
//! execution through the system's Python interpreter.

use std::io::Write as _;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

fn nme(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_nme"))
        .args(args)
        .output()
        .expect("the nme binary must run")
}

fn nme_with_input(args: &[&str], input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_nme"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the nme binary must run");
    child
        .stdin
        .take()
        .expect("stdin must be piped")
        .write_all(input.as_bytes())
        .expect("test input must be writable");
    child.wait_with_output().expect("nme must finish")
}

fn example(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples")
        .join(name);
    path.to_string_lossy().into_owned()
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn python_available() -> bool {
    Command::new("python3")
        .arg("--version")
        .output()
        .is_ok_and(|out| out.status.success())
}

#[test]
fn build_prints_transpiled_python() {
    let output = nme(&["build", &example("hello.nme")]);
    assert!(output.status.success(), "{}", stderr(&output));
    let python = stdout(&output);
    assert!(python.contains("print(\"Hello, world!\")"), "{python}");
    assert!(python.contains("for _ in range(3):"), "{python}");
    // Comments and blank lines are preserved.
    assert!(python.contains("# Pure NME"), "{python}");
}

#[test]
fn version_reports_the_first_beta() {
    let output = nme(&["--version"]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(stdout(&output), "nme 0.0.1-beta.1\n");
}

#[test]
fn check_accepts_good_nme() {
    let output = nme(&["check", &example("hello.nme")]);
    assert!(output.status.success(), "{}", stderr(&output));
}

#[test]
fn check_rejects_broken_nme_with_a_friendly_error() {
    let dir = std::env::temp_dir().join(format!("nme-cli-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("broken.nme");
    std::fs::write(&file, "5 times:\nsay \"not indented\"\n").unwrap();

    let output = nme(&["check", &file.to_string_lossy()]);
    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(error.contains("error:"), "{error}");
    assert!(error.contains("indented"), "{error}");
    assert!(error.contains("hint:"), "{error}");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn missing_file_is_a_clean_error() {
    let output = nme(&["run", "does-not-exist.nme"]);
    assert!(!output.status.success());
    assert!(
        stderr(&output).contains("couldn't read"),
        "{}",
        stderr(&output)
    );
}

#[test]
fn run_executes_pure_nme_with_python() {
    if !python_available() {
        eprintln!("python3 not available; skipping execution test");
        return;
    }
    let output = nme(&["run", &example("hello.nme")]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(
        stdout(&output),
        "Hello, world!\nNME is easy\nNME is easy\nNME is easy\none line is fine too\none line is fine too\n"
    );
}

#[test]
fn run_reads_input_with_ask() {
    if !python_available() {
        eprintln!("python3 not available; skipping execution test");
        return;
    }
    let output = nme_with_input(&["run", &example("ask.nme")], "Mina\n");
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(stdout(&output), "What is your name? Hello, Mina!\n");
}

#[test]
fn run_executes_mixed_python_and_nme() {
    if !python_available() {
        eprintln!("python3 not available; skipping execution test");
        return;
    }
    let output = nme(&["run", &example("mixed.nme")]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(
        stdout(&output),
        "Go, Ada!\nGo, Grace!\nNME block\npython block\nNME block\npython block\n"
    );
}

#[test]
fn run_executes_korean_nme_with_bundled_random() {
    if !python_available() {
        eprintln!("python3 not available; skipping execution test");
        return;
    }
    let output = nme(&["run", &example("korean.nme")]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(
        stdout(&output),
        "친구에게 고양이 추천!\n친구에게 고양이 추천!\n"
    );
}

#[test]
fn run_executes_pure_python_unchanged() {
    if !python_available() {
        eprintln!("python3 not available; skipping execution test");
        return;
    }
    let output = nme(&["run", &example("pure_python.nme")]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert!(
        stdout(&output).contains("r=3 area=28.27"),
        "{}",
        stdout(&output)
    );
}

#[test]
fn runtime_errors_keep_the_original_line_numbers() {
    if !python_available() {
        eprintln!("python3 not available; skipping execution test");
        return;
    }
    let dir = std::env::temp_dir().join(format!("nme-cli-test-err-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("boom.nme");
    // The error is on line 3 of the .nme file; lowering preserves lines.
    std::fs::write(&file, "say \"first\"\nsay \"second\"\nsay 1 / 0\n").unwrap();

    let output = nme(&["run", &file.to_string_lossy()]);
    assert!(!output.status.success());
    assert_eq!(stdout(&output), "first\nsecond\n");
    let error = stderr(&output);
    assert!(error.contains("ZeroDivisionError"), "{error}");
    assert!(error.contains("line 3"), "{error}");

    let _ = std::fs::remove_dir_all(&dir);
}
