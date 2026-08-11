//! Integration tests for the `nme` command line tool, including real
//! execution through the system's Python interpreter.

use std::io::Write as _;
use std::path::{Path, PathBuf};
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
    String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n")
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n")
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
    let version = env!("CARGO_PKG_VERSION");
    let output = nme(&["--version"]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(stdout(&output), format!("nme {version}\n"));

    let korean = nme(&["버전"]);
    assert!(korean.status.success(), "{}", stderr(&korean));
    assert_eq!(
        stdout(&korean),
        format!("NME 버전: {version}\nnme version: {version}\n")
    );
}

#[test]
fn modules_uses_the_language_of_the_command() {
    let english = nme(&["modules"]);
    assert!(english.status.success(), "{}", stderr(&english));
    assert_eq!(
        stdout(&english),
        "random  0.0.1  bundled, latest\nfile  0.0.1  bundled, latest\nzero_knowledge  0.0.2  bundled, latest\n"
    );
    assert!(!stdout(&english).contains("내장"));

    let bilingual = nme(&["모듈"]);
    assert!(bilingual.status.success(), "{}", stderr(&bilingual));
    assert_eq!(
        stdout(&bilingual),
        "랜덤  0.0.1  내장, 최신\nrandom  0.0.1  bundled, latest\n파일  0.0.1  내장, 최신\nfile  0.0.1  bundled, latest\n영지식  0.0.2  내장, 최신\nzero_knowledge  0.0.2  bundled, latest\n"
    );
}

#[test]
fn help_is_english_only_for_english_commands_and_bilingual_for_korean_help() {
    let english = nme(&["help"]);
    assert!(english.status.success(), "{}", stderr(&english));
    let english_help = stdout(&english);
    assert!(english_help.contains("START HERE:"), "{english_help}");
    assert!(english_help.contains("nme run hello"), "{english_help}");
    assert!(english_help.contains("SHORTCUTS:"), "{english_help}");
    assert!(english_help.contains("nme r hello"), "{english_help}");
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
    assert!(bilingual_help.contains("File names work\nwith or without .nme"));
}

fn temporary_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "nme-cli-test-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).expect("temp dir must be created");
    dir
}

fn write_nme(dir: &std::path::Path, name: &str, source: &str) {
    std::fs::write(dir.join(name), source).expect("example must be written");
}

#[test]
fn command_shortcuts_run_check_build_and_modules() {
    let output = nme(&["r", &example("hello-sentence.nme")]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert!(
        stdout(&output).contains("Hello, world!"),
        "{}",
        stdout(&output)
    );

    let checked = nme(&["c", &example("three-levels.nme")]);
    assert!(checked.status.success(), "{}", stderr(&checked));

    let build_dir = temporary_dir("build-alias");
    let build_path = build_dir.join("nme-build-alias-test.py");
    let built = nme(&[
        "b",
        &example("hello.nme"),
        "-o",
        &build_path.to_string_lossy(),
    ]);
    assert!(built.status.success(), "{}", stderr(&built));
    let _ = std::fs::remove_dir_all(&build_dir);

    let modules = nme(&["m"]);
    assert!(modules.status.success(), "{}", stderr(&modules));
    assert!(stdout(&modules).contains("random"), "{}", stdout(&modules));
    assert!(stdout(&modules).contains("file"), "{}", stdout(&modules));
    assert!(
        stdout(&modules).contains("zero_knowledge"),
        "{}",
        stdout(&modules)
    );

    let version = nme(&["v"]);
    assert!(version.status.success(), "{}", stderr(&version));
    assert!(
        stdout(&version).contains(&format!("nme {}", env!("CARGO_PKG_VERSION"))),
        "{}",
        stdout(&version)
    );

    let help = nme(&["h"]);
    assert!(help.status.success(), "{}", stderr(&help));
    assert!(stdout(&help).contains("SHORTCUTS:"), "{}", stdout(&help));
}

#[test]
fn run_passes_program_arguments_to_the_program() {
    if !python_available() {
        return;
    }
    let dir = temporary_dir("args");
    write_nme(
        &dir,
        "args.nme",
        "import sys\nshow f\"program: {sys.argv[0]}\"\nshow f\"args: {sys.argv[1:]}\"\n",
    );
    let program = dir.join("args.nme");
    let output = nme(&["run", &program.to_string_lossy(), "one", "two"]);
    assert!(output.status.success(), "{}", stderr(&output));
    let text = stdout(&output);
    assert!(text.contains("program: "), "{text}");
    assert!(text.contains("args: ['one', 'two']"), "{text}");
    assert!(
        !stderr(&output).contains("unexpected"),
        "{}",
        stderr(&output)
    );

    let bare = nme(&["run", &program.to_string_lossy(), "-5", "3.5"]);
    assert!(bare.status.success(), "{}", stderr(&bare));
    assert!(
        stdout(&bare).contains("args: ['-5', '3.5']"),
        "{}",
        stdout(&bare)
    );
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn run_with_shortcut_and_korean_command_passes_arguments() {
    if !python_available() {
        return;
    }
    let dir = temporary_dir("args-ko");
    write_nme(
        &dir,
        "hello.nme",
        "import sys\nshow f\"hello {sys.argv[1]}\"\n",
    );
    let program = dir.join("hello.nme");
    let via_r = nme(&["r", &program.to_string_lossy(), "Mina"]);
    assert!(via_r.status.success(), "{}", stderr(&via_r));
    assert!(stdout(&via_r).contains("hello Mina"), "{}", stdout(&via_r));

    let via_korean = nme(&["실행", &program.to_string_lossy(), "미나"]);
    assert!(via_korean.status.success(), "{}", stderr(&via_korean));
    assert!(
        stdout(&via_korean).contains("hello 미나"),
        "{}",
        stdout(&via_korean)
    );
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn the_file_module_reads_and_writes_text_and_json() {
    if !python_available() {
        eprintln!("Python not available; skipping file module test");
        return;
    }
    let dir = std::env::temp_dir().join(format!("nme-cli-file-module-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("roundtrip.nme"),
        "use file\n\
         file_write(\"out.txt\", \"hello 파일\")\n\
         show file_read(\"out.txt\")\n\
         점수 = {\"이름\": \"민수\", \"점수\": 3}\n\
         json_save(\"save.json\", 점수)\n\
         보관 = json_load(\"save.json\")\n\
         show 보관[\"이름\"]\n\
         show file_version\n",
    )
    .unwrap();

    let output = run_in(&dir, &["run", "roundtrip.nme"], None);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(stdout(&output), "hello 파일\n민수\n0.0.1\n");

    let saved = std::fs::read_to_string(dir.join("save.json")).unwrap();
    assert!(saved.contains("민수"), "{saved}");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_korean_file_module_spelling_works() {
    if !python_available() {
        eprintln!("Python not available; skipping Korean file module test");
        return;
    }
    let dir = std::env::temp_dir().join(format!("nme-cli-file-ko-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("roundtrip.ko.nme"),
        "파일 사용 최신\n\
         파일쓰기(\"out.txt\", \"안녕\")\n\
         말해 파일읽기(\"out.txt\")\n",
    )
    .unwrap();

    let output = run_in(&dir, &["run", "roundtrip.ko.nme"], None);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(stdout(&output), "안녕\n");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn nme_module_imports_run_across_files() {
    if !python_available() {
        eprintln!("Python not available; skipping module import test");
        return;
    }
    let dir = std::env::temp_dir().join(format!("nme-cli-module-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("helper.nme"),
        "인사말 = \"안녕하세요\"\ndef double(n):\n    return n * 2\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("main.nme"),
        "from \"helper.nme\" import 인사말, double\nshow 인사말\nshow double(21)\n",
    )
    .unwrap();

    let output = run_in(&dir, &["run", "main"], None);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(stdout(&output), "안녕하세요\n42\n");

    let checked = run_in(&dir, &["check", "main"], None);
    assert!(checked.status.success(), "{}", stderr(&checked));

    let built = run_in(&dir, &["b", "main"], None);
    assert!(built.status.success(), "{}", stderr(&built));
    assert!(
        stdout(&built).contains("from helper import 인사말, double"),
        "{}",
        stdout(&built)
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_missing_module_gets_a_clear_error() {
    let dir = std::env::temp_dir().join(format!("nme-cli-module-missing-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("main.nme"),
        "from \"nope.nme\" import greet\nshow greet\n",
    )
    .unwrap();

    let output = run_in(&dir, &["run", "main"], None);
    assert!(!output.status.success());
    assert!(
        stderr(&output).contains("couldn't read module"),
        "{}",
        stderr(&output)
    );
    assert!(stderr(&output).contains("nope.nme"), "{}", stderr(&output));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn module_imports_reach_nested_imports() {
    if !python_available() {
        eprintln!("Python not available; skipping nested module test");
        return;
    }
    let dir = std::env::temp_dir().join(format!("nme-cli-module-nested-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("base.nme"), "기본값 = 100\n").unwrap();
    std::fs::write(
        dir.join("helper.nme"),
        "from \"base.nme\" import 기본값\n값 = 기본값 + 1\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("main.nme"),
        "from \"helper.nme\" import 값\nshow 값\n",
    )
    .unwrap();

    let output = run_in(&dir, &["run", "main"], None);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(stdout(&output), "101\n");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_native_example_matches_the_python_path_output() {
    if Command::new("cc").arg("--version").output().is_err() || !python_available() {
        eprintln!("cc or Python not available; skipping native parity test");
        return;
    }
    let python_output = nme(&["run", &example("native-count.nme")]);
    assert!(python_output.status.success(), "{}", stderr(&python_output));

    let dir = std::env::temp_dir().join(format!("nme-cli-native-parity-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::copy(
        Path::new(&example("native-count.nme")),
        dir.join("native-count.nme"),
    )
    .unwrap();
    let native_output = run_in(&dir, &["native", "native-count"], None);
    assert!(native_output.status.success(), "{}", stderr(&native_output));
    assert_eq!(stdout(&native_output), stdout(&python_output));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_native_command_compiles_and_runs_a_core_program() {
    if Command::new("cc").arg("--version").output().is_err() {
        eprintln!("cc not available; skipping native test");
        return;
    }
    let dir = std::env::temp_dir().join(format!("nme-cli-native-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("count.nme"),
        "score = 0\nwhile score is less than 3\n    score add 1\nend\nshow score\nshow \"done\"\n",
    )
    .unwrap();

    let run = run_in(&dir, &["native", "count"], None);
    assert!(run.status.success(), "{}", stderr(&run));
    assert_eq!(stdout(&run), "3\ndone\n");

    let built = run_in(&dir, &["native", "build", "count", "-o", "count_out"], None);
    assert!(built.status.success(), "{}", stderr(&built));
    let executable = if cfg!(windows) {
        dir.join("count_out.exe")
    } else {
        dir.join("count_out")
    };
    assert!(executable.exists(), "no executable written");
    assert!(dir.join("count_out.c").exists(), "no C source written");

    let rejected = run_in(&dir, &["native", "ask.nme"], None);
    assert!(!rejected.status.success());

    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(unix)]
#[test]
fn native_run_start_failures_use_a_native_diagnostic() {
    use std::os::unix::fs::PermissionsExt as _;

    let dir = std::env::temp_dir().join(format!(
        "nme-cli-native-start-failure-{}",
        std::process::id()
    ));
    let tools = dir.join("tools");
    std::fs::create_dir_all(&tools).unwrap();
    std::fs::write(dir.join("hello.nme"), "say 1\n").unwrap();

    let fake_cc = tools.join("cc");
    std::fs::write(&fake_cc, "#!/bin/sh\nexit 0\n").unwrap();
    let mut permissions = std::fs::metadata(&fake_cc).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&fake_cc, permissions).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_nme"))
        .args(["native", "hello"])
        .current_dir(&dir)
        .env("PATH", &tools)
        .output()
        .unwrap();
    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(
        error.contains("error[E9026]: couldn't start the native program"),
        "{error}"
    );
    assert!(!error.contains("E9013"), "{error}");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn install_requires_a_package_and_explains_pip_failures() {
    let no_package = nme(&["install"]);
    assert!(!no_package.status.success());
    assert!(
        stderr(&no_package).contains("which package should I install"),
        "{}",
        stderr(&no_package)
    );

    // An empty requirement is rejected by pip before it can contact a package
    // index, so this checks the failure code without a network dependency.
    let invalid_package = nme(&["install", ""]);
    assert!(!invalid_package.status.success());
    assert!(
        stderr(&invalid_package).contains("error[E9025]: pip failed to install"),
        "{}",
        stderr(&invalid_package)
    );

    let invalid_korean_package = nme(&["설치", ""]);
    assert!(!invalid_korean_package.status.success());
    let korean_error = stderr(&invalid_korean_package);
    assert!(
        korean_error.contains("오류[E9025]: pip이"),
        "{korean_error}"
    );
    assert!(
        korean_error.contains("error[E9025]: pip failed to install"),
        "{korean_error}"
    );

    let two = nme(&["install", "a", "b"]);
    assert!(!two.status.success());

    // pip is not installed in this environment; the error must be friendly.
    let missing_pip = Command::new(python_command())
        .args(["-m", "pip", "--version"])
        .output()
        .map_or(true, |out| !out.status.success());
    if missing_pip {
        let install = nme(&["install", "requests"]);
        assert!(!install.status.success());
        assert!(
            stderr(&install).contains("pip failed to install"),
            "{}",
            stderr(&install)
        );
    }
}

#[test]
fn the_terminal_menu_example_runs_with_scripted_input() {
    if !python_available() {
        eprintln!("Python not available; skipping terminal menu test");
        return;
    }
    for (file, expected) in [
        ("terminal-menu.nme", ["hello!", "bye"]),
        ("terminal-menu.ko.nme", ["안녕하세요!", "안녕히 가세요"]),
    ] {
        let output = nme_with_input(&["run", &example(file)], "1\n3\n");
        assert!(output.status.success(), "{file}: {}", stderr(&output));
        let out = stdout(&output);
        for fragment in expected {
            assert!(out.contains(fragment), "{file}: {out}");
        }
    }
}

#[test]
fn the_cryptocurrency_learning_examples_run() {
    if !python_available() {
        eprintln!("Python not available; skipping cryptocurrency example test");
        return;
    }
    for name in [
        "needmorecoin-sentence.ko",
        "needmorecoin-sentence.en",
        "needmorecoin-beginner.ko",
        "needmorecoin-beginner.en",
        "needmorecoin-advanced.ko",
        "needmorecoin-advanced.en",
        "native-factorial",
        "bootstrap",
        "bootstrap.ko",
    ] {
        let output = nme(&["run", &example(&format!("{name}.nme"))]);
        assert!(output.status.success(), "{name}: {}", stderr(&output));
        assert!(!stdout(&output).is_empty(), "{name}: no output");
    }
}

#[test]
fn sentence_file_forms_read_and_write() {
    if !python_available() {
        eprintln!("Python not available; skipping sentence file test");
        return;
    }
    let dir = std::env::temp_dir().join(format!("nme-cli-sentence-file-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("data.txt"), "sentence hello").unwrap();
    std::fs::write(
        dir.join("program.nme"),
        "read \"data.txt\" into memo\n\
         show memo\n\
         write \"sentence saved\" to \"out.txt\"\n\
         memo에 \"data.txt\" 읽어서 저장해\n\
         memo 말해줘\n\
         \"한글.txt\" 파일에 \"저장됨\"를 저장해\n",
    )
    .unwrap();

    let output = run_in(&dir, &["run", "program.nme"], None);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(stdout(&output), "sentence hello\nsentence hello\n");

    assert_eq!(
        std::fs::read_to_string(dir.join("out.txt")).unwrap(),
        "sentence saved"
    );
    assert_eq!(
        std::fs::read_to_string(dir.join("한글.txt")).unwrap(),
        "저장됨"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

fn run_in(dir: &std::path::Path, args: &[&str], input: Option<&str>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_nme"));
    command.args(args).current_dir(dir);
    if let Some(text) = input {
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("nme must run");
        child
            .stdin
            .take()
            .expect("stdin must be piped")
            .write_all(text.as_bytes())
            .expect("test input must be writable");
        child.wait_with_output().expect("nme must finish")
    } else {
        command.output().expect("nme must run")
    }
}

#[test]
fn bare_run_discovers_the_only_program_in_the_folder() {
    let dir = temporary_dir("solo");
    write_nme(&dir, "solo.nme", "show Hello from the only program!\n");
    let output = run_in(&dir, &["r"], None);
    assert!(output.status.success(), "{}", stderr(&output));
    assert!(
        stdout(&output).contains("Hello from the only program!"),
        "{}",
        stdout(&output)
    );
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn bare_run_asks_which_program_when_several_exist() {
    let dir = temporary_dir("several");
    write_nme(&dir, "a.nme", "show Program A\n");
    write_nme(&dir, "b.nme", "show Program B\n");
    let output = run_in(&dir, &["r"], Some("2\n"));
    assert!(output.status.success(), "{}", stderr(&output));
    let shown = stdout(&output);
    assert!(shown.contains("1. a.nme"), "{shown}");
    assert!(shown.contains("2. b.nme"), "{shown}");
    assert!(shown.contains("Program B"), "{shown}");

    let by_name = run_in(&dir, &["r"], Some("b.nme\n"));
    assert!(by_name.status.success(), "{}", stderr(&by_name));
    assert!(
        stdout(&by_name).contains("Program B"),
        "{}",
        stdout(&by_name)
    );

    let invalid = run_in(&dir, &["r"], Some("9\n"));
    assert!(!invalid.status.success());
    assert!(
        stderr(&invalid).contains("not one of the programs"),
        "{}",
        stderr(&invalid)
    );
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn bare_check_and_build_discover_like_run() {
    let dir = temporary_dir("bare-check");
    write_nme(&dir, "one.nme", "show Hello\n");
    let checked = run_in(&dir, &["c"], None);
    assert!(checked.status.success(), "{}", stderr(&checked));
    let built = run_in(&dir, &["b", "-o", "one.py"], None);
    assert!(built.status.success(), "{}", stderr(&built));
    assert!(dir.join("one.py").exists(), "build output must be written");
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn bare_run_without_programs_explains_what_to_do() {
    let dir = temporary_dir("bare-empty");
    let output = run_in(&dir, &["r"], None);
    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(error.contains("no .nme program here"), "{error}");
    assert!(error.contains("nme run hello"), "{error}");
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn korean_bare_run_asks_in_korean_first() {
    let dir = temporary_dir("bare-ko");
    write_nme(&dir, "가.nme", "안녕하세요! 말해줘\n");
    write_nme(&dir, "나.nme", "반가워요! 말해줘\n");
    let output = run_in(&dir, &["실행"], Some("1\n"));
    assert!(output.status.success(), "{}", stderr(&output));
    let shown = stdout(&output);
    assert!(shown.contains("현재 폴더의 .nme 프로그램"), "{shown}");
    assert!(shown.contains("안녕하세요!"), "{shown}");
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn command_errors_follow_the_command_language() {
    let english = nme(&["run", "--not-an-option"]);
    assert!(!english.status.success());
    let english_error = stderr(&english);
    assert!(
        english_error.contains("error[E9004]: unknown option"),
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
    let korean_position = bilingual_error
        .find("오류[E9004]: 알 수 없는 옵션")
        .unwrap();
    let english_position = bilingual_error
        .find("error[E9004]: unknown option")
        .unwrap();
    assert!(korean_position < english_position, "{bilingual_error}");

    let native = nme(&["네이티브", "--not-an-option"]);
    assert!(!native.status.success());
    let native_error = stderr(&native);
    let native_korean_position = native_error
        .find("오류[E9004]: 알 수 없는 옵션입니다")
        .unwrap();
    let native_english_position = native_error
        .find("error[E9004]: unknown option")
        .unwrap();
    assert!(
        native_korean_position < native_english_position,
        "{native_error}"
    );

    let install = nme(&["설치"]);
    assert!(!install.status.success());
    let install_error = stderr(&install);
    let install_korean_position = install_error
        .find("오류[E9003]: 어떤 패키지를 설치할까요?")
        .unwrap();
    let install_english_position = install_error
        .find("error[E9003]: which package should I install?")
        .unwrap();
    assert!(
        install_korean_position < install_english_position,
        "{install_error}"
    );
}

#[test]
fn korean_missing_file_errors_are_substantively_bilingual() {
    let output = nme(&["실행", "nme-file-that-does-not-exist"]);
    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(
        error.contains("nme-file-that-does-not-exist 파일을 읽을 수 없습니다"),
        "{error}"
    );
    assert!(
        error.contains("couldn't read nme-file-that-does-not-exist"),
        "{error}"
    );
    assert!(
        error.contains("nme-file-that-does-not-exist.nme"),
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
    let output_file = dir.join("already-here.exe");
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

    let english = nme(&["run", "--python", missing_python, &file]);
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

    let bilingual = nme(&["실행", "--python", missing_python, &file]);
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

    // `run` accepts anything after the program name as program arguments
    // (they become sys.argv[1:]), but options before it are still checked.
    let run_unknown = nme(&["run", "--not-an-nme-option", &first]);
    assert!(!run_unknown.status.success(), "run ignored an option");
    assert!(
        stderr(&run_unknown).contains("unknown option"),
        "{}",
        stderr(&run_unknown)
    );

    for command in ["build", "check"] {
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
    assert!(error.contains("error[E0501]:"), "{error}");
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

#[test]
fn a_ko_suffixed_name_resolves_to_the_korean_twin_file() {
    let dir = std::env::temp_dir().join(format!("nme-cli-ko-twin-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let stem = dir.join("game");
    std::fs::write(stem.with_extension("nme"), "show english game\n").unwrap();
    std::fs::write(
        Path::new(&format!("{}.ko", stem.display())).with_extension("nme"),
        "show korean game\n",
    )
    .unwrap();
    let korean_stem = format!("{}.ko", stem.display());

    let output = nme(&["build", &korean_stem]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(stdout(&output), "print(\"korean game\")\n");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_directory_argument_explains_that_it_is_a_folder() {
    let dir = std::env::temp_dir().join(format!("nme-cli-folder-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let output = nme(&["run", &dir.to_string_lossy()]);
    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(error.contains("is a folder, not a program"), "{error}");

    let korean = nme(&["실행", &dir.to_string_lossy()]);
    assert!(!korean.status.success());
    let korean_error = stderr(&korean);
    assert!(
        korean_error.contains("폴더이지 프로그램이 아니에요"),
        "{korean_error}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_miscased_or_shortened_name_runs_and_a_misspelled_name_suggests() {
    if !python_available() {
        eprintln!("Python not available; skipping name resolution test");
        return;
    }
    let dir = std::env::temp_dir().join(format!("nme-cli-suggest-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let stem = dir.join("guessing-game");
    std::fs::write(stem.with_extension("nme"), "show guessing\n").unwrap();

    let miscased = nme(&["run", &dir.join("GUESSING-GAME").to_string_lossy()]);
    assert!(miscased.status.success(), "{}", stderr(&miscased));
    assert_eq!(stdout(&miscased), "guessing\n");

    let truncated = nme(&["run", &dir.join("guessing-gam").to_string_lossy()]);
    assert!(truncated.status.success(), "{}", stderr(&truncated));
    assert_eq!(stdout(&truncated), "guessing\n");

    let misspelled = nme(&["run", &dir.join("game").to_string_lossy()]);
    assert!(!misspelled.status.success());
    let misspelled_error = stderr(&misspelled);
    assert!(
        misspelled_error.contains("did you mean `guessing-game.nme`"),
        "{misspelled_error}"
    );
    assert!(
        misspelled_error.contains("Try `nme run guessing-game`"),
        "{misspelled_error}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_unique_name_prefix_runs_the_matching_program() {
    if !python_available() {
        eprintln!("Python not available; skipping prefix run test");
        return;
    }
    let dir = std::env::temp_dir().join(format!("nme-cli-prefix-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    write_nme(&dir, "alpha.nme", "show alpha program\n");
    write_nme(&dir, "beta.nme", "show beta program\n");

    for (command, expected) in [("run", "alpha program"), ("실행", "alpha program")] {
        let output = nme(&[command, &dir.join("a").to_string_lossy()]);
        assert!(output.status.success(), "{command}: {}", stderr(&output));
        assert_eq!(stdout(&output), format!("{expected}\n"));
    }

    let built = nme(&["b", &dir.join("bet").to_string_lossy()]);
    assert!(built.status.success(), "{}", stderr(&built));
    assert_eq!(stdout(&built), "print(\"beta program\")\n");

    let checked = nme(&["c", &dir.join("b").to_string_lossy()]);
    assert!(checked.status.success(), "{}", stderr(&checked));

    let korean_checked = nme(&["검사", &dir.join("a").to_string_lossy()]);
    assert!(
        korean_checked.status.success(),
        "{}",
        stderr(&korean_checked)
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn an_ambiguous_name_prefix_lists_candidates_instead_of_guessing() {
    let dir = std::env::temp_dir().join(format!("nme-cli-ambiguous-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    write_nme(&dir, "same-a.nme", "show first\n");
    write_nme(&dir, "same-b.nme", "show second\n");

    let output = nme(&["run", &dir.join("same").to_string_lossy()]);
    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(error.contains("several programs match"), "{error}");
    assert!(error.contains("same-a.nme"), "{error}");
    assert!(error.contains("same-b.nme"), "{error}");
    assert!(error.contains("type more of the name"), "{error}");

    let korean = nme(&["실행", &dir.join("same").to_string_lossy()]);
    assert!(!korean.status.success());
    let korean_error = stderr(&korean);
    assert!(
        korean_error.contains("일치하는 프로그램이 여러 개예요"),
        "{korean_error}"
    );
    assert!(
        korean_error.contains("이름을 더 길게 적어 주세요"),
        "{korean_error}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_bare_path_prefix_is_a_run_shortcut() {
    if !python_available() {
        eprintln!("Python not available; skipping bare prefix test");
        return;
    }
    let dir = std::env::temp_dir().join(format!("nme-cli-bare-prefix-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    write_nme(&dir, "alpha.nme", "show bare prefix\n");
    write_nme(&dir, "beta.nme", "show other\n");

    let output = nme(&[&dir.join("al").to_string_lossy()]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(stdout(&output), "bare prefix\n");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn an_ambiguous_bare_prefix_lists_candidates() {
    let dir = std::env::temp_dir().join(format!("nme-cli-bare-ambig-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    write_nme(&dir, "same-a.nme", "show first\n");
    write_nme(&dir, "same-b.nme", "show second\n");

    let output = nme(&[&dir.join("same").to_string_lossy()]);
    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(error.contains("several programs match"), "{error}");
    assert!(error.contains("same-a.nme"), "{error}");
    assert!(error.contains("same-b.nme"), "{error}");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_numbered_pick_accepts_bare_names_and_unique_prefixes() {
    let dir = std::env::temp_dir().join(format!("nme-cli-pick-prefix-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    write_nme(&dir, "alpha.nme", "show alpha picked\n");
    write_nme(&dir, "beta.nme", "show beta picked\n");

    let by_bare_name = run_in(&dir, &["r"], Some("alpha"));
    assert!(by_bare_name.status.success(), "{}", stderr(&by_bare_name));
    assert!(
        stdout(&by_bare_name).contains("alpha picked\n"),
        "{}",
        stdout(&by_bare_name)
    );

    let by_prefix = run_in(&dir, &["r"], Some("b"));
    assert!(by_prefix.status.success(), "{}", stderr(&by_prefix));
    assert!(
        stdout(&by_prefix).contains("beta picked\n"),
        "{}",
        stdout(&by_prefix)
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn an_ambiguous_pick_answer_lists_the_matching_programs() {
    let dir = std::env::temp_dir().join(format!("nme-cli-pick-ambig-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    write_nme(&dir, "same-a.nme", "show first\n");
    write_nme(&dir, "same-b.nme", "show second\n");

    let output = run_in(&dir, &["r"], Some("same"));
    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(error.contains("matches several programs"), "{error}");
    assert!(error.contains("same-a.nme"), "{error}");
    assert!(error.contains("same-b.nme"), "{error}");
    assert!(error.contains("pick a number"), "{error}");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn an_empty_pick_answer_gets_a_friendly_message() {
    let dir = std::env::temp_dir().join(format!("nme-cli-empty-pick-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    write_nme(&dir, "one.nme", "show one\n");
    write_nme(&dir, "two.nme", "show two\n");

    let output = run_in(&dir, &["r"], Some(""));
    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(
        error.contains("no answer given — type a number or a program name"),
        "{error}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn help_and_help_shortcut_ignore_extra_arguments() {
    let output = nme(&["h", "extra"]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert!(
        stdout(&output).contains("START HERE:"),
        "{}",
        stdout(&output)
    );

    let korean = nme(&["도움", "extra"]);
    assert!(korean.status.success(), "{}", stderr(&korean));
    assert!(
        stdout(&korean).contains("처음 시작:"),
        "{}",
        stdout(&korean)
    );

    let english_help = nme(&["h"]);
    assert!(
        stdout(&english_help).contains("stay unique"),
        "{}",
        stdout(&english_help)
    );
    assert!(
        stdout(&english_help).contains("nme ko E0001"),
        "{}",
        stdout(&english_help)
    );
    let korean_help = nme(&["도움"]);
    assert!(
        stdout(&korean_help).contains("줄여 쓸 수"),
        "{}",
        stdout(&korean_help)
    );
}

#[test]
fn error_lookup_commands_print_the_requested_explanation() {
    let korean = nme(&["ko", "E0101"]);
    assert!(korean.status.success(), "{}", stderr(&korean));
    let korean_out = stdout(&korean);
    assert!(korean_out.contains("E0101"), "{korean_out}");
    assert!(korean_out.contains("열린 블록이 없는 `끝`"), "{korean_out}");
    assert!(
        korean_out.contains("E0101 — an `end` with no open block"),
        "{korean_out}"
    );

    let english = nme(&["en", "E0101"]);
    assert!(english.status.success(), "{}", stderr(&english));
    let english_out = stdout(&english);
    assert!(
        english_out.contains("an `end` with no open block"),
        "{english_out}"
    );
    assert!(!english_out.contains("열린 블록"), "{english_out}");

    let korean_alias = nme(&["에러", "E0101"]);
    assert!(korean_alias.status.success(), "{}", stderr(&korean_alias));
    assert!(
        stdout(&korean_alias).contains("열린 블록이 없는 `끝`"),
        "{}",
        stdout(&korean_alias)
    );

    let english_alias = nme(&["error", "E0101"]);
    assert!(english_alias.status.success(), "{}", stderr(&english_alias));
    assert!(
        stdout(&english_alias).contains("an `end` with no open block"),
        "{}",
        stdout(&english_alias)
    );

    let package_english = nme(&["en", "E9025"]);
    assert!(
        package_english.status.success(),
        "{}",
        stderr(&package_english)
    );
    assert!(
        stdout(&package_english).contains("pip could not install the package"),
        "{}",
        stdout(&package_english)
    );

    let package_korean = nme(&["ko", "E9025"]);
    assert!(
        package_korean.status.success(),
        "{}",
        stderr(&package_korean)
    );
    assert!(
        stdout(&package_korean).contains("pip이 패키지를 설치하지 못했습니다"),
        "{}",
        stdout(&package_korean)
    );

    let native_failure = nme(&["en", "E9010"]);
    assert!(
        native_failure.status.success(),
        "{}",
        stderr(&native_failure)
    );
    let native_failure_out = stdout(&native_failure);
    assert!(
        native_failure_out.contains("nme compile"),
        "{native_failure_out}"
    );
    assert!(
        native_failure_out.contains("nme native"),
        "{native_failure_out}"
    );

    let native_start = nme(&["ko", "E9011"]);
    assert!(
        native_start.status.success(),
        "{}",
        stderr(&native_start)
    );
    let native_start_out = stdout(&native_start);
    assert!(
        native_start_out.contains("nme 컴파일"),
        "{native_start_out}"
    );
    assert!(
        native_start_out.contains("시스템 C 컴파일러"),
        "{native_start_out}"
    );

    let native_run = nme(&["en", "E9026"]);
    assert!(native_run.status.success(), "{}", stderr(&native_run));
    assert!(
        stdout(&native_run).contains("the native program could not be started"),
        "{}",
        stdout(&native_run)
    );

    let native_run_korean = nme(&["ko", "E9026"]);
    assert!(
        native_run_korean.status.success(),
        "{}",
        stderr(&native_run_korean)
    );
    assert!(
        stdout(&native_run_korean).contains("네이티브 프로그램을 시작할 수 없습니다"),
        "{}",
        stdout(&native_run_korean)
    );

    let unknown = nme(&["ko", "E9999"]);
    assert!(!unknown.status.success());
    let unknown_error = stderr(&unknown);
    assert!(
        unknown_error.contains("there is no error code `E9999`"),
        "{unknown_error}"
    );
    assert!(unknown_error.contains("`nme ko`"), "{unknown_error}");
}

#[test]
fn error_lookup_without_a_code_lists_every_code() {
    let english = nme(&["en"]);
    assert!(english.status.success(), "{}", stderr(&english));
    let out = stdout(&english);
    assert!(out.contains("E0001"), "{out}");
    assert!(out.contains("E0702"), "{out}");
    assert!(out.contains("E9025"), "{out}");
    assert!(out.contains("E0102  `break` outside a loop"), "{out}");

    let korean = nme(&["ko"]);
    assert!(korean.status.success(), "{}", stderr(&korean));
    let korean_out = stdout(&korean);
    assert!(
        korean_out.contains("E0102  반복문 밖의 `break`"),
        "{korean_out}"
    );
}

#[test]
fn a_real_error_reports_its_lookup_code() {
    let dir = std::env::temp_dir().join(format!("nme-cli-code-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("loop.nme");
    std::fs::write(&file, "break here\n").unwrap();

    let output = nme(&["check", &file.to_string_lossy()]);
    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(error.contains("error[E0102]:"), "{error}");
    assert!(error.contains("inside a loop"), "{error}");

    let korean = nme(&["검사", &file.to_string_lossy()]);
    assert!(!korean.status.success());
    let korean_error = stderr(&korean);
    assert!(korean_error.contains("오류[E0102]:"), "{korean_error}");

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn cli_errors_carry_lookup_codes() {
    let unknown_command = nme(&["this-command-does-not-exist"]);
    assert!(!unknown_command.status.success());
    assert!(
        stderr(&unknown_command).contains("error[E9001]:"),
        "{}",
        stderr(&unknown_command)
    );

    let missing_file = nme(&["run", "definitely-not-a-program.nme"]);
    assert!(!missing_file.status.success());
    assert!(
        stderr(&missing_file).contains("error[E9015]:"),
        "{}",
        stderr(&missing_file)
    );

    let cli_code_page = nme(&["en", "E9004"]);
    assert!(cli_code_page.status.success(), "{}", stderr(&cli_code_page));
    assert!(
        stdout(&cli_code_page).contains("E9004 — unknown option"),
        "{}",
        stdout(&cli_code_page)
    );

    let cli_code_korean = nme(&["ko", "E9001"]);
    assert!(
        cli_code_korean.status.success(),
        "{}",
        stderr(&cli_code_korean)
    );
    assert!(
        stdout(&cli_code_korean).contains("알 수 없는 명령"),
        "{}",
        stdout(&cli_code_korean)
    );
}

#[test]
fn schnorr_nizk_context_examples_reject_cross_context_reuse() {
    if !python_available() {
        eprintln!("Python not available; skipping context-bound NIZK example test");
        return;
    }

    let korean = nme(&["run", &example("zk-nizk-context.ko.nme")]);
    assert!(korean.status.success(), "{}", stderr(&korean));
    let korean_text = stdout(&korean);
    assert!(
        korean_text.contains("문맥에 묶인 비대화 영지식 증명을 검증했습니다"),
        "{korean_text}"
    );
    assert!(
        korean_text.contains("같은 증명은 다른 문맥으로 재사용할 수 없습니다"),
        "{korean_text}"
    );

    let english = nme(&["run", &example("zk-nizk-context.en.nme")]);
    assert!(english.status.success(), "{}", stderr(&english));
    let english_text = stdout(&english);
    assert!(
        english_text.contains("Context-bound non-interactive proof verified."),
        "{english_text}"
    );
    assert!(
        english_text.contains("The same proof was rejected under a different context."),
        "{english_text}"
    );
}

#[test]
fn schnorr_zero_knowledge_examples_run_end_to_end() {
    if !python_available() {
        eprintln!("Python not available; skipping zero-knowledge example test");
        return;
    }

    let korean = nme(&["run", &example("zk-schnorr-relay.ko.nme")]);
    assert!(korean.status.success(), "{}", stderr(&korean));
    let korean_text = stdout(&korean);
    assert!(korean_text.contains("영지식 증명을 수신자 비가 받아들였습니다"));
    assert!(korean_text.contains("저장 전사록 재전송은 새 도전에서 실패했습니다"));
    assert!(korean_text.contains("비밀값 없이도 미리 고른 도전에 맞는 전사록을 모의할 수 있습니다"));
    assert!(korean_text.contains("모의 전사록은 수신자 비의 다른 도전에 재사용할 수 없습니다"));
    assert!(korean_text.contains("실시간 중계는 통과하지만 비밀 위조가 아니라"));

    let english = nme(&["run", &example("zk-schnorr-relay.en.nme")]);
    assert!(english.status.success(), "{}", stderr(&english));
    let english_text = stdout(&english);
    assert!(english_text.contains("accepted sender A's zero-knowledge proof"));
    assert!(english_text.contains("cannot replay the saved transcript"));
    assert!(english_text.contains("can be simulated without the secret"));
    assert!(english_text.contains("cannot answer receiver B's different challenge"));
    assert!(english_text.contains("A live relay can pass"));
}
