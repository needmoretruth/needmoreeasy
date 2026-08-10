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
fn version_reports_the_current_beta() {
    let output = nme(&["--version"]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(stdout(&output), "nme 0.0.1-beta.2\n");
}

#[test]
fn modules_reports_the_bundled_random_version_in_both_languages() {
    for command in ["modules", "모듈"] {
        let output = nme(&[command]);
        assert!(output.status.success(), "{}", stderr(&output));
        assert_eq!(
            stdout(&output),
            "random / 랜덤  0.0.1  bundled, latest / 내장, 최신\n"
        );
    }
}

#[test]
fn convert_turns_python_into_a_chosen_nme_level_and_language() {
    let dir = std::env::temp_dir().join(format!("nme-cli-convert-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("hello.py");
    std::fs::write(
        &file,
        "name = input(\"이름이 뭐예요?\")\nif name:\n    print(\"안녕하세요!\")\n",
    )
    .unwrap();

    let output = nme(&[
        "convert",
        &file.to_string_lossy(),
        "--level",
        "sentence",
        "--language",
        "ko",
    ]);
    assert!(output.status.success(), "{}", stderr(&output));
    let converted = stdout(&output);
    assert_eq!(
        converted,
        "name을 물어봐 \"이름이 뭐예요?\"\n만약에 name\n    보여줘 \"안녕하세요!\"\n"
    );
    let converted_file = dir.join("hello.nme");
    std::fs::write(&converted_file, converted).unwrap();
    let checked = nme(&["check", &converted_file.to_string_lossy()]);
    assert!(checked.status.success(), "{}", stderr(&checked));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn convert_can_write_beginner_nme_to_a_file() {
    let dir = std::env::temp_dir().join(format!("nme-cli-convert-out-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("hello.py");
    let output_file = dir.join("hello.nme");
    std::fs::write(&input, "print(\"Hello\")\n").unwrap();

    let output = nme(&[
        "변환",
        &input.to_string_lossy(),
        "--level",
        "초급",
        "--language",
        "영어",
        "-o",
        &output_file.to_string_lossy(),
    ]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(
        std::fs::read_to_string(&output_file).unwrap(),
        "say \"Hello\"\n"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn compile_invokes_the_native_backend_and_creates_the_requested_artifact() {
    use std::os::unix::fs::PermissionsExt as _;

    let dir = std::env::temp_dir().join(format!("nme-cli-native-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("hello.nme");
    let output_file = dir.join("hello-app");
    let fake_python = dir.join("fake-python");
    let arguments = dir.join("arguments.txt");
    std::fs::write(&input, "show Hello native build\n").unwrap();
    std::fs::write(
        &fake_python,
        format!(
            "#!/bin/sh\nprintf '%s\\n' \"$@\" > '{}'\nfor arg in \"$@\"; do\n  case \"$arg\" in\n    --output-dir=*) out_dir=${{arg#--output-dir=}} ;;\n    --output-filename=*) out_name=${{arg#--output-filename=}} ;;\n  esac\ndone\ntouch \"$out_dir/$out_name\"\n",
            arguments.display()
        ),
    )
    .unwrap();
    let mut permissions = std::fs::metadata(&fake_python).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&fake_python, permissions).unwrap();

    let output = nme(&[
        "compile",
        &input.to_string_lossy(),
        "-o",
        &output_file.to_string_lossy(),
        "--python",
        &fake_python.to_string_lossy(),
    ]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert!(output_file.is_file());
    let invoked = std::fs::read_to_string(arguments).unwrap();
    assert!(invoked.contains("-m\nnuitka\n"), "{invoked}");
    assert!(invoked.contains("--onefile"), "{invoked}");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn compile_refuses_to_overwrite_an_existing_artifact() {
    let dir = std::env::temp_dir().join(format!("nme-cli-native-safe-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let input = dir.join("hello.nme");
    let output_file = dir.join("already-here");
    std::fs::write(&input, "show Hello\n").unwrap();
    std::fs::write(&output_file, "keep me").unwrap();

    let output = nme(&[
        "compile",
        &input.to_string_lossy(),
        "-o",
        &output_file.to_string_lossy(),
    ]);
    assert!(!output.status.success());
    assert!(stderr(&output).contains("refusing to overwrite"));
    assert_eq!(std::fs::read_to_string(&output_file).unwrap(), "keep me");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn check_accepts_good_nme() {
    let output = nme(&["check", &example("hello.nme")]);
    assert!(output.status.success(), "{}", stderr(&output));
}

#[test]
fn every_documented_example_passes_the_compiler() {
    let examples = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples");
    let mut files = std::fs::read_dir(examples)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "nme"))
        .collect::<Vec<_>>();
    files.sort();
    for file in files {
        let output = nme(&["check", &file.to_string_lossy()]);
        assert!(
            output.status.success(),
            "{} failed:\n{}",
            file.display(),
            stderr(&output)
        );
    }
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
    assert_eq!(stdout(&output), "What is your name? Hello Mina!\n");
}

#[test]
fn run_executes_sentence_syntax_without_code_punctuation() {
    if !python_available() {
        eprintln!("python3 not available; skipping execution test");
        return;
    }
    let output = nme(&["run", &example("hello-sentence.nme")]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(
        stdout(&output),
        "Hello, world!\nNME is easy\nNME is easy\nNME is easy\n"
    );
}

#[test]
fn run_executes_all_three_levels_and_both_languages_together() {
    if !python_available() {
        eprintln!("python3 not available; skipping execution test");
        return;
    }
    let output = nme(&["run", &example("three-levels.nme")]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(
        stdout(&output),
        "beginner Korean\nbeginner Korean\n한국어 문장형도 함께 작동해요\n한국어 문장형도 함께 작동해요\nHello Ada!\nHello Grace!\n"
    );
}

#[test]
fn run_executes_the_compiler_written_in_nme() {
    if !python_available() {
        eprintln!("python3 not available; skipping execution test");
        return;
    }
    let output = nme(&["run", &example("tiny-compiler.nme")]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(
        stdout(&output),
        "생성된 Python 코드\nprint('안녕하세요')\nfor _ in range(3): print('NME로 컴파일러를 만들었어요')\n"
    );
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
