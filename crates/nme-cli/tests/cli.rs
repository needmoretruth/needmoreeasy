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

fn python_command() -> &'static str {
    if cfg!(windows) {
        "py"
    } else {
        "python3"
    }
}

fn python_available() -> bool {
    Command::new(python_command())
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
    assert_eq!(stdout(&output), "nme 0.0.1-beta.15\n");

    let korean = nme(&["버전"]);
    assert!(korean.status.success(), "{}", stderr(&korean));
    assert_eq!(
        stdout(&korean),
        "NME 버전: 0.0.1-beta.15\nnme version: nme 0.0.1-beta.15\n"
    );
}

#[test]
fn modules_uses_the_language_of_the_command() {
    let english = nme(&["modules"]);
    assert!(english.status.success(), "{}", stderr(&english));
    assert_eq!(stdout(&english), "random  0.0.1  bundled, latest\n");
    assert!(!stdout(&english).contains("내장"));

    let bilingual = nme(&["모듈"]);
    assert!(bilingual.status.success(), "{}", stderr(&bilingual));
    assert_eq!(
        stdout(&bilingual),
        "랜덤  0.0.1  내장, 최신\nrandom  0.0.1  bundled, latest\n"
    );
}

#[test]
fn help_is_english_only_for_english_commands_and_bilingual_for_korean_help() {
    let english = nme(&["help"]);
    assert!(english.status.success(), "{}", stderr(&english));
    let english_help = stdout(&english);
    assert!(english_help.contains("START HERE:"), "{english_help}");
    assert!(english_help.contains("nme run hello"), "{english_help}");
    assert!(!english_help.contains("처음 시작"), "{english_help}");
    assert!(!english_help.contains("nme 실행"), "{english_help}");
    let beginner_section = english_help
        .split("MORE COMMANDS:")
        .next()
        .expect("help has a beginner section");
    assert!(!beginner_section.contains("--python"), "{english_help}");

    let bilingual = nme(&["도움"]);
    assert!(bilingual.status.success(), "{}", stderr(&bilingual));
    let bilingual_help = stdout(&bilingual);
    let korean_position = bilingual_help.find("처음 시작:").unwrap();
    let english_position = bilingual_help.find("START HERE:").unwrap();
    assert!(korean_position < english_position, "{bilingual_help}");
    assert!(
        bilingual_help.contains("nme 실행 hello"),
        "{bilingual_help}"
    );
    assert!(bilingual_help.contains("nme run hello"), "{bilingual_help}");
    assert!(bilingual_help.contains("파일 이름의 .nme는 생략"));
    assert!(bilingual_help.contains("File names may be\nwritten with or without .nme"));
}

#[test]
fn command_errors_follow_the_command_language() {
    let english = nme(&["run", "--not-an-option"]);
    assert!(!english.status.success());
    let english_error = stderr(&english);
    assert!(
        english_error.contains("error: unknown option"),
        "{english_error}"
    );
    assert!(!english_error.contains("오류:"), "{english_error}");
    assert!(
        !english_error.contains("알 수 없는 옵션"),
        "{english_error}"
    );

    let bilingual = nme(&["실행", "--not-an-option"]);
    assert!(!bilingual.status.success());
    let bilingual_error = stderr(&bilingual);
    let korean_position = bilingual_error.find("오류: 알 수 없는 옵션").unwrap();
    let english_position = bilingual_error.find("error: unknown option").unwrap();
    assert!(korean_position < english_position, "{bilingual_error}");
}

#[test]
fn korean_missing_file_errors_are_substantively_bilingual() {
    let output = nme(&["실행", "nme-file-that-does-not-exist"]);
    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(
        error.contains("nme-file-that-does-not-exist.nme 파일을 읽을 수 없습니다"),
        "{error}"
    );
    assert!(
        error.contains("couldn't read nme-file-that-does-not-exist.nme"),
        "{error}"
    );
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
        "name을 물어봐 \"이름이 뭐예요?\"\nif name:\n    보여줘 \"안녕하세요!\"\n"
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
    assert!(
        stdout(&output).is_empty(),
        "check must not execute the program"
    );
}

#[test]
fn check_uses_cpython_to_reject_invalid_advanced_syntax() {
    if !python_available() {
        eprintln!("Python not available; skipping CPython check test");
        return;
    }
    let dir = std::env::temp_dir().join(format!("nme-cli-python-check-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("broken-advanced.nme");
    std::fs::write(&file, "if True:\nprint('not indented')\n").unwrap();

    let output = nme(&[
        "check",
        &file.to_string_lossy(),
        "--python",
        python_command(),
    ]);
    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(error.contains("IndentationError"), "{error}");
    assert!(
        error.contains(&file.to_string_lossy().to_string()),
        "{error}"
    );
    assert!(!error.contains("<string>"), "{error}");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn build_validates_with_cpython_before_writing_output() {
    if !python_available() {
        eprintln!("Python not available; skipping CPython build validation test");
        return;
    }
    let dir = std::env::temp_dir().join(format!("nme-cli-build-check-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("broken.nme");
    let built = dir.join("must-not-exist.py");
    std::fs::write(&file, "if True:\nprint('not indented')\n").unwrap();

    let english = nme(&[
        "build",
        &file.to_string_lossy(),
        "-o",
        &built.to_string_lossy(),
    ]);
    assert!(!english.status.success());
    let english_error = stderr(&english);
    assert!(
        english_error.contains("IndentationError"),
        "{english_error}"
    );
    assert!(
        english_error.contains("generated Python did not pass CPython's syntax check"),
        "{english_error}"
    );
    assert!(!english_error.contains("오류:"), "{english_error}");
    assert!(!built.exists(), "a failed build must not write its output");

    let bilingual = nme(&[
        "빌드",
        &file.to_string_lossy(),
        "-o",
        &built.to_string_lossy(),
    ]);
    assert!(!bilingual.status.success());
    let bilingual_error = stderr(&bilingual);
    assert!(
        bilingual_error.contains("만들어진 Python이 CPython 문법 검사를 통과하지 못했습니다"),
        "{bilingual_error}"
    );
    assert!(
        bilingual_error.contains("generated Python did not pass CPython's syntax check"),
        "{bilingual_error}"
    );
    let korean_position = bilingual_error
        .find("만들어진 Python이 CPython 문법 검사를 통과하지 못했습니다")
        .unwrap();
    let python_position = bilingual_error.find("IndentationError").unwrap();
    assert!(korean_position < python_position, "{bilingual_error}");
    assert!(!built.exists(), "a failed build must not write its output");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn python_launch_errors_follow_the_command_language() {
    let file = example("hello.nme");
    let missing_python = "nme-python-command-that-does-not-exist";

    let english = nme(&["run", &file, "--python", missing_python]);
    assert!(!english.status.success());
    let english_error = stderr(&english);
    assert!(
        english_error.contains("couldn't start Python"),
        "{english_error}"
    );
    assert!(
        english_error.contains("install Python 3"),
        "{english_error}"
    );
    assert!(!english_error.contains("오류:"), "{english_error}");

    let bilingual = nme(&["실행", &file, "--python", missing_python]);
    assert!(!bilingual.status.success());
    let bilingual_error = stderr(&bilingual);
    assert!(bilingual_error.contains("Python("), "{bilingual_error}");
    assert!(
        bilingual_error.contains("시작할 수 없습니다"),
        "{bilingual_error}"
    );
    assert!(
        bilingual_error.contains("couldn't start Python"),
        "{bilingual_error}"
    );
}

#[test]
fn run_build_and_check_reject_extra_files_and_unknown_options() {
    let first = example("hello.nme");
    let second = example("pure_python.nme");
    for command in ["run", "build", "check"] {
        let extra = nme(&[command, &first, &second]);
        assert!(!extra.status.success(), "{command} accepted two files");
        assert!(
            stderr(&extra).contains("unexpected extra file"),
            "{}",
            stderr(&extra)
        );

        let unknown = nme(&[command, &first, "--not-an-nme-option"]);
        assert!(!unknown.status.success(), "{command} ignored an option");
        assert!(
            stderr(&unknown).contains("unknown option"),
            "{}",
            stderr(&unknown)
        );
    }
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

    let bilingual = nme(&["검사", &file.to_string_lossy()]);
    assert!(!bilingual.status.success());
    let bilingual_error = stderr(&bilingual);
    let korean_position = bilingual_error
        .find("반복할 다음 줄은 들여써야 해요")
        .unwrap();
    let english_position = bilingual_error
        .find("the lines that should repeat must be indented")
        .unwrap();
    assert!(korean_position < english_position, "{bilingual_error}");
    assert!(bilingual_error.contains("도움말:"), "{bilingual_error}");

    assert!(!error.contains("반복할 다음 줄"), "{error}");
    assert!(!error.contains("오류:"), "{error}");

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
fn extensionless_names_work_with_run_check_build_and_the_direct_shortcut() {
    if !python_available() {
        eprintln!("Python not available; skipping extensionless command test");
        return;
    }
    let dir = std::env::temp_dir().join(format!("nme-cli-extensionless-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let stem = dir.join("program");
    std::fs::write(stem.with_extension("nme"), "show extensionless works\n").unwrap();
    let stem = stem.to_string_lossy().into_owned();

    for command in ["run", "실행"] {
        let output = nme(&[command, &stem]);
        assert!(output.status.success(), "{command}: {}", stderr(&output));
        assert_eq!(stdout(&output), "extensionless works\n");
    }
    for command in ["check", "검사"] {
        let output = nme(&[command, &stem]);
        assert!(output.status.success(), "{command}: {}", stderr(&output));
        assert!(stdout(&output).is_empty());
    }
    for command in ["build", "빌드"] {
        let output = nme(&[command, &stem]);
        assert!(output.status.success(), "{command}: {}", stderr(&output));
        assert_eq!(stdout(&output), "print(\"extensionless works\")\n");
    }

    let direct = nme(&[&stem]);
    assert!(direct.status.success(), "{}", stderr(&direct));
    assert_eq!(stdout(&direct), "extensionless works\n");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn an_exact_extensionless_path_wins_over_the_nme_fallback() {
    if !python_available() {
        eprintln!("Python not available; skipping exact path command test");
        return;
    }
    let dir = std::env::temp_dir().join(format!("nme-cli-exact-path-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let stem = dir.join("program");
    std::fs::write(&stem, "show exact path\n").unwrap();
    std::fs::write(stem.with_extension("nme"), "show fallback path\n").unwrap();
    let stem = stem.to_string_lossy().into_owned();

    let output = nme(&["build", &stem]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(stdout(&output), "print(\"exact path\")\n");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn run_executes_pure_nme_with_python() {
    if !python_available() {
        eprintln!("Python not available; skipping execution test");
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
        eprintln!("Python not available; skipping execution test");
        return;
    }
    let output = nme_with_input(&["run", &example("ask.nme")], "Mina\n");
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(
        stdout(&output),
        "이름이 뭐예요? Hello Mina!\n환영합니다\n환영합니다\n환영합니다\n"
    );
}

#[test]
fn run_executes_sentence_syntax_without_code_punctuation() {
    if !python_available() {
        eprintln!("Python not available; skipping execution test");
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
fn a_bare_nme_path_is_a_run_shortcut() {
    if !python_available() {
        eprintln!("Python not available; skipping shortcut execution test");
        return;
    }
    let output = nme(&[&example("hello-sentence.nme")]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(
        stdout(&output),
        "Hello, world!\nNME is easy\nNME is easy\nNME is easy\n"
    );
}

#[test]
fn run_preserves_the_original_path_imports_resources_and_argv() {
    if !python_available() {
        eprintln!("Python not available; skipping path execution test");
        return;
    }
    let dir = std::env::temp_dir().join(format!("nme cli path with spaces {}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let helper = dir.join("helper.py");
    let resource = dir.join("message.txt");
    let file = dir.join("context.nme");
    std::fs::write(&helper, "VALUE = 'sibling import works'\n").unwrap();
    std::fs::write(&resource, "resource works").unwrap();
    std::fs::write(
        &file,
        concat!(
            "from pathlib import Path\n",
            "import sys\n",
            "from helper import VALUE\n",
            "MAIN_VALUE = 'main module works'\n",
            "print(VALUE)\n",
            "import __main__\n",
            "print(__main__.MAIN_VALUE)\n",
            "print(Path(__file__).name)\n",
            "print(Path(__file__).with_name('message.txt').read_text(encoding='utf-8'))\n",
            "print(Path(sys.path[0]).resolve() == Path(__file__).parent.resolve())\n",
            "print(Path(sys.argv[0]).resolve() == Path(__file__).resolve())\n",
            "print(len(sys.argv))\n",
        ),
    )
    .unwrap();

    let output = nme(&["run", &file.to_string_lossy()]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(
        stdout(&output),
        "sibling import works\nmain module works\ncontext.nme\nresource works\nTrue\nTrue\n1\n"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn run_executes_all_three_levels_and_both_languages_together() {
    if !python_available() {
        eprintln!("Python not available; skipping execution test");
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
        eprintln!("Python not available; skipping execution test");
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
        eprintln!("Python not available; skipping execution test");
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
        eprintln!("Python not available; skipping execution test");
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
        eprintln!("Python not available; skipping execution test");
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
        eprintln!("Python not available; skipping execution test");
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
    assert!(
        error.contains(&file.to_string_lossy().to_string()),
        "{error}"
    );
    assert!(!error.contains("<string>"), "{error}");
    assert!(!error.contains("nme-run-"), "{error}");

    let _ = std::fs::remove_dir_all(&dir);
}
