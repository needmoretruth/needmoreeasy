//! `nme` — the NeedMoreEasy command line toolchain.
//!
//! This crate is a thin driver around `nme_core`: it handles all IO
//! (reading files, writing files, running Python) so the compiler core can
//! stay pure and trivially testable.
//!
//! ```text
//! nme run   hello.nme              transpile and execute with Python
//! nme build hello.nme [-o out.py]  transpile and write/print the Python
//! nme compile hello.nme -o hello    compile a standalone native executable
//! nme check hello.nme              transpile only; report problems
//! nme convert app.py --level sentence --language ko
//! ```

mod exec;

use std::io::Write as _;
use std::path::Path;
use std::process::ExitCode;

const HELP_ENGLISH: &str = r"nme — NeedMoreEasy: start simpler, then grow into Python.

START HERE:
    nme run hello                 Run hello.nme (`nme hello` also works)
    nme check hello               Check hello.nme without running it
    nme build hello -o hello.py   Turn hello.nme into readable Python

SHORTCUTS:
    nme r hello                   run
    nme c hello                   check
    nme b hello                   build
    nme m                         modules (same as `nme modules`)
    nme v                         version (same as `nme --version`)
    nme h                         help
    nme comp hello                compile (Nuitka native build)
    nme conv app.py               convert (Python to NME)

With no file name, `nme r` runs the single .nme program in the current
folder; with several, it asks which one to run. `nme c` and `nme b` do the
same for checking and building.

MORE COMMANDS:
    nme compile hello -o hello    Build an executable with Nuitka
    nme convert app.py [options]  Convert safe Python patterns to NME
        --level advanced|beginner|sentence
        --language en|ko
        -o <output.nme>
    nme modules                   Show bundled modules and versions
    nme help                      Show this help
    nme --version                 Show the version

ADVANCED OPTIONS:
    --python <command>            Override the automatically selected Python

You may mix conversational sentences, beginner syntax, and ordinary Python in
one file. English and Korean NME spellings may be mixed too. File names may be
written with or without .nme. Exact existing paths always win.
";

const HELP_KOREAN: &str = r"nme — NeedMoreEasy: 더 쉽게 시작해서 Python으로 성장하세요.

처음 시작:
    nme 실행 hello                 hello.nme 실행 (`nme hello`도 가능)
    nme 검사 hello                 실행하지 않고 hello.nme 검사
    nme 빌드 hello -o hello.py   hello.nme를 읽기 쉬운 Python으로 변환

짧은 명령:
    nme r hello                   실행
    nme c hello                   검사
    nme b hello                   빌드
    nme m                         모듈 보기 (`nme modules`와 같음)
    nme v                         버전 보기 (`nme --version`과 같음)
    nme h                         도움말
    nme comp hello                컴파일 (Nuitka 실행 파일)
    nme conv app.py               변환 (Python을 NME로)

파일 이름 없이 `nme r`을 실행하면 현재 폴더의 .nme 프로그램이 하나일 때
그것을 실행하고, 여러 개일 때는 어느 것을 실행할지 물어봅니다.
`nme c`와 `nme b`도 검사·빌드에서 같은 방식으로 동작합니다.

더 많은 명령:
    nme 컴파일 hello -o hello    Nuitka로 실행 파일 만들기
    nme 변환 app.py [옵션]       안전한 Python 형태를 NME로 변환
        --level advanced|beginner|sentence
        --language en|ko
        -o <출력.nme>
    nme 모듈                       내장 모듈과 버전 보기
    nme 도움                       이 도움말 보기
    nme 버전                       버전 보기

고급 옵션:
    --python <명령>                자동으로 선택된 Python 명령 바꾸기

한 파일에 문장형, 초급 문법, 일반 Python을 섞어 쓸 수 있습니다.
영어와 한국어 NME도 섞어 쓸 수 있습니다. 파일 이름의 .nme는 생략해도 됩니다.
이미 있는 경로를 입력하면 그 경로를 우선합니다.
";

const DEFAULT_PYTHON: &str = if cfg!(windows) { "py" } else { "python3" };

#[derive(Clone, Copy, Eq, PartialEq)]
enum MessageLanguage {
    English,
    KoreanAndEnglish,
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("run" | "r") => command_run(&args[1..], MessageLanguage::English),
        Some("실행") => command_run(&args[1..], MessageLanguage::KoreanAndEnglish),
        Some("build" | "b") => command_build(&args[1..], MessageLanguage::English),
        Some("빌드") => command_build(&args[1..], MessageLanguage::KoreanAndEnglish),
        Some("compile" | "comp") => command_compile(&args[1..], MessageLanguage::English),
        Some("컴파일") => command_compile(&args[1..], MessageLanguage::KoreanAndEnglish),
        Some("check" | "c") => command_check(&args[1..], MessageLanguage::English),
        Some("검사") => command_check(&args[1..], MessageLanguage::KoreanAndEnglish),
        Some("convert" | "conv") => command_convert(&args[1..], MessageLanguage::English),
        Some("변환") => command_convert(&args[1..], MessageLanguage::KoreanAndEnglish),
        Some("modules" | "module" | "m") => command_modules(&args[1..], MessageLanguage::English),
        Some("모듈") => command_modules(&args[1..], MessageLanguage::KoreanAndEnglish),
        Some("버전") if args.len() == 1 => {
            println!(
                "NME 버전: {}\nnme version: nme {}",
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_VERSION")
            );
            ExitCode::SUCCESS
        }
        Some("--version" | "-V" | "version" | "v") if args.len() == 1 => {
            println!("nme {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Some("--help" | "-h" | "help" | "h") if args.len() == 1 => {
            print!("{HELP_ENGLISH}");
            ExitCode::SUCCESS
        }
        Some("도움" | "도움말") if args.len() == 1 => {
            print_bilingual_help();
            ExitCode::SUCCESS
        }
        Some(path) if is_direct_program(path) => command_run(&args, MessageLanguage::English),
        Some(command) => {
            let language = if contains_korean(command) {
                MessageLanguage::KoreanAndEnglish
            } else {
                MessageLanguage::English
            };
            fail(
                language,
                &format!(
                    "I don't know the command `{command}`. Run `nme help` to see the commands.\n\
                     Tip: `nme r` runs the single .nme program in the current folder."
                ),
                &format!(
                    "`{command}` 명령을 알 수 없습니다. `nme 도움`으로 명령을 확인하세요.\n\
                     팁: 현재 폴더에 .nme 파일이 하나뿐이면 `nme r`만으로 실행할 수 있어요."
                ),
            )
        }
        _ => {
            eprint!("{HELP_ENGLISH}");
            ExitCode::FAILURE
        }
    }
}

fn print_bilingual_help() {
    print!("{HELP_KOREAN}\nENGLISH / 영어\n\n{HELP_ENGLISH}");
}

fn command_modules(args: &[String], language: MessageLanguage) -> ExitCode {
    if let Some(extra) = args.first() {
        return fail(
            language,
            &format!("`modules` does not take `{extra}`. Try `nme modules`."),
            &format!("`모듈` 명령에는 `{extra}`을(를) 적지 않습니다. `nme 모듈`을 사용하세요."),
        );
    }
    match language {
        MessageLanguage::English => println!(
            "random  {}  bundled, latest",
            nme_core::syntax::RANDOM_MODULE_VERSION
        ),
        MessageLanguage::KoreanAndEnglish => println!(
            "랜덤  {}  내장, 최신\nrandom  {}  bundled, latest",
            nme_core::syntax::RANDOM_MODULE_VERSION,
            nme_core::syntax::RANDOM_MODULE_VERSION
        ),
    }
    ExitCode::SUCCESS
}

fn command_convert(args: &[String], language: MessageLanguage) -> ExitCode {
    let mut file = None;
    let mut output = None;
    let mut level = nme_core::SyntaxLevel::Sentence;
    let mut output_language = nme_core::Language::English;
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--level" => {
                let Some(value) = rest.next() else {
                    return fail(
                        language,
                        "--level needs advanced, beginner, or sentence",
                        "--level 뒤에 advanced, beginner, sentence 중 하나를 적어 주세요",
                    );
                };
                level = match value.as_str() {
                    "advanced" | "고급" => nme_core::SyntaxLevel::Advanced,
                    "beginner" | "초급" => nme_core::SyntaxLevel::Beginner,
                    "sentence" | "문장" | "문장형" => nme_core::SyntaxLevel::Sentence,
                    _ => {
                        return fail(
                            language,
                            "--level needs advanced, beginner, or sentence",
                            "--level 뒤에 advanced, beginner, sentence 중 하나를 적어 주세요",
                        );
                    }
                };
            }
            "--language" | "--lang" => {
                let Some(value) = rest.next() else {
                    return fail(
                        language,
                        "--language needs en or ko",
                        "--language 뒤에 en(영어) 또는 ko(한국어)를 적어 주세요",
                    );
                };
                output_language = match value.as_str() {
                    "en" | "english" | "영어" => nme_core::Language::English,
                    "ko" | "korean" | "한국어" => nme_core::Language::Korean,
                    _ => {
                        return fail(
                            language,
                            "--language needs en or ko",
                            "--language 뒤에 en(영어) 또는 ko(한국어)를 적어 주세요",
                        );
                    }
                };
            }
            "-o" | "--output" => match rest.next() {
                Some(path) => output = Some(path.clone()),
                None => {
                    return fail(
                        language,
                        "-o needs a path, e.g. -o program.nme",
                        "-o 뒤에 저장할 경로가 필요합니다. 예: -o program.nme",
                    );
                }
            },
            flag if flag.starts_with('-') => {
                return fail(
                    language,
                    &format!("unknown option: {flag}"),
                    &format!("알 수 없는 옵션입니다: {flag}"),
                );
            }
            path if file.is_none() => file = Some(path.to_string()),
            path => {
                return fail(
                    language,
                    &format!("unexpected extra file: {path}"),
                    &format!("파일은 하나만 적어 주세요. 추가로 적힌 파일: {path}"),
                );
            }
        }
    }
    let Some(file) = file else {
        return fail(
            language,
            "which Python file should I convert? e.g. nme convert app.py",
            "변환할 Python 파일을 적어 주세요. 예: nme 변환 app.py",
        );
    };
    convert_file(&file, output, level, output_language, language)
}

fn convert_file(
    file: &str,
    output: Option<String>,
    level: nme_core::SyntaxLevel,
    output_language: nme_core::Language,
    message_language: MessageLanguage,
) -> ExitCode {
    let source = match std::fs::read_to_string(file) {
        Ok(source) => source,
        Err(error) => {
            return fail(
                message_language,
                &format!("couldn't read {file}: {error}"),
                &format!("{file} 파일을 읽을 수 없습니다: {error}"),
            );
        }
    };
    let conversion = match nme_core::convert_python(&source, level, output_language) {
        Ok(conversion) => conversion,
        Err(problems) => {
            eprint!(
                "{}",
                render_diagnostics(&problems, &source, file, message_language)
            );
            return ExitCode::FAILURE;
        }
    };
    if let Some(path) = output {
        match std::fs::write(&path, conversion.source) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => fail(
                message_language,
                &format!("couldn't write {path}: {error}"),
                &format!("{path} 파일을 저장할 수 없습니다: {error}"),
            ),
        }
    } else {
        print!("{}", conversion.source);
        ExitCode::SUCCESS
    }
}

fn command_compile(args: &[String], language: MessageLanguage) -> ExitCode {
    let (file, output, python) = match compile_arguments(args, language) {
        Ok(arguments) => arguments,
        Err(code) => return code,
    };
    let source_path = resolve_nme_path(Path::new(&file));
    let stem = source_path
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("program");
    let mut output = output.unwrap_or_else(|| {
        let name = if cfg!(windows) {
            format!("{stem}.exe")
        } else {
            stem.to_string()
        };
        std::path::PathBuf::from(name)
    });
    if cfg!(windows) && output.extension().is_none() {
        output.set_extension("exe");
    }
    if output.exists() {
        return fail(
            language,
            &format!(
                "refusing to overwrite existing output: {}",
                output.display()
            ),
            &format!(
                "이미 있는 결과 파일을 덮어쓰지 않습니다: {}",
                output.display()
            ),
        );
    }
    let (_, python_source) = match transpile_file(&file, language) {
        Ok(ok) => ok,
        Err(code) => return code,
    };
    match exec::compile_native(&python_source, stem, &python, &output) {
        Ok(status) if status.success() => {
            if output.exists() {
                ExitCode::SUCCESS
            } else {
                fail(
                    language,
                    &format!(
                        "native compiler succeeded but did not create {}",
                        output.display()
                    ),
                    &format!(
                        "네이티브 컴파일러는 성공으로 종료했지만 {} 파일을 만들지 않았습니다",
                        output.display()
                    ),
                )
            }
        }
        Ok(status) => fail(
            language,
            &format!(
                "native compilation failed with {status}\n\
                 hint: install Nuitka with `{python} -m pip install nuitka` and make sure a C compiler is available"
            ),
            &format!(
                "네이티브 컴파일이 실패했습니다: {status}\n\
                 도움말: `{python} -m pip install nuitka`로 Nuitka를 설치하고 C 컴파일러가 있는지 확인하세요"
            ),
        ),
        Err(error) => fail(
            language,
            &format!(
                "couldn't start native compilation: {error}\n\
                 hint: install Python and Nuitka, then run this command again\n\
                 advanced: use --python <command> only if Python has another command name"
            ),
            &format!(
                "네이티브 컴파일을 시작할 수 없습니다: {error}\n\
                 도움말: Python과 Nuitka를 설치한 뒤 이 명령을 다시 실행하세요\n\
                 고급: Python 명령 이름이 다를 때만 --python <명령>을 사용하세요"
            ),
        ),
    }
}

fn compile_arguments(
    args: &[String],
    language: MessageLanguage,
) -> Result<(String, Option<std::path::PathBuf>, String), ExitCode> {
    let mut python = DEFAULT_PYTHON.to_string();
    let mut file = None;
    let mut output = None;
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--python" => match rest.next() {
                Some(command) => python.clone_from(command),
                None => {
                    return Err(fail(
                        language,
                        &format!("--python needs a command, e.g. --python {DEFAULT_PYTHON}"),
                        &format!(
                            "--python 뒤에 Python 명령이 필요합니다. 예: --python {DEFAULT_PYTHON}"
                        ),
                    ));
                }
            },
            "-o" | "--output" => match rest.next() {
                Some(path) => output = Some(std::path::PathBuf::from(path)),
                None => {
                    return Err(fail(
                        language,
                        "-o needs an executable path, e.g. -o hello",
                        "-o 뒤에 만들 실행 파일 경로가 필요합니다. 예: -o hello",
                    ));
                }
            },
            flag if flag.starts_with('-') => {
                return Err(fail(
                    language,
                    &format!("unknown option: {flag}"),
                    &format!("알 수 없는 옵션입니다: {flag}"),
                ));
            }
            path if file.is_none() => file = Some(path.to_string()),
            path => {
                return Err(fail(
                    language,
                    &format!("unexpected extra file: {path}"),
                    &format!("파일은 하나만 적어 주세요. 추가로 적힌 파일: {path}"),
                ));
            }
        }
    }
    let file = match file {
        Some(file) => file,
        None => match discover_current_program(language, "compile", "컴파일") {
            Ok(found) => found,
            Err(code) => return Err(code),
        },
    };
    Ok((file, output, python))
}

/// `nme run`: transpile, then execute with the real Python runtime.
fn command_run(args: &[String], language: MessageLanguage) -> ExitCode {
    let mut python = DEFAULT_PYTHON.to_string();
    let mut file = None;
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--python" => match rest.next() {
                Some(command) => python.clone_from(command),
                None => {
                    return fail(
                        language,
                        &format!("--python needs a command, e.g. --python {DEFAULT_PYTHON}"),
                        &format!(
                            "--python 뒤에 Python 명령이 필요합니다. 예: --python {DEFAULT_PYTHON}"
                        ),
                    );
                }
            },
            flag if flag.starts_with('-') => {
                return fail(
                    language,
                    &format!("unknown option: {flag}"),
                    &format!("알 수 없는 옵션입니다: {flag}"),
                );
            }
            path if file.is_none() => file = Some(path.to_string()),
            path => {
                return fail(
                    language,
                    &format!("unexpected extra file: {path}"),
                    &format!("파일은 하나만 적어 주세요. 추가로 적힌 파일: {path}"),
                );
            }
        }
    }
    let file = match file {
        Some(file) => file,
        None => match discover_current_program(language, "run", "실행") {
            Ok(found) => found,
            Err(code) => return code,
        },
    };

    let (path, python_source) = match transpile_file(&file, language) {
        Ok(ok) => ok,
        Err(code) => return code,
    };
    match exec::run_python(&python_source, &path, &python) {
        Ok(status) => exit_code(status),
        Err(err) => fail(
            language,
            &format!(
                "couldn't start Python ({python}): {err}\n\
                 hint: install Python 3, then run this command again\n\
                 advanced: use --python <command> only if Python has another command name"
            ),
            &format!(
                "Python({python})을 시작할 수 없습니다: {err}\n\
                 도움말: Python 3를 설치한 뒤 이 명령을 다시 실행하세요\n\
                 고급: Python 명령 이름이 다를 때만 --python <명령>을 사용하세요"
            ),
        ),
    }
}

/// `nme build`: transpile and print (or write) the Python program.
fn command_build(args: &[String], language: MessageLanguage) -> ExitCode {
    let mut python = DEFAULT_PYTHON.to_string();
    let mut output = None;
    let mut file = None;
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--python" => match rest.next() {
                Some(command) => python.clone_from(command),
                None => {
                    return fail(
                        language,
                        &format!("--python needs a command, e.g. --python {DEFAULT_PYTHON}"),
                        &format!(
                            "--python 뒤에 Python 명령이 필요합니다. 예: --python {DEFAULT_PYTHON}"
                        ),
                    );
                }
            },
            "-o" | "--output" => match rest.next() {
                Some(path) => output = Some(path.clone()),
                None => {
                    return fail(
                        language,
                        "-o needs a path, e.g. -o hello.py",
                        "-o 뒤에 저장할 경로가 필요합니다. 예: -o hello.py",
                    );
                }
            },
            flag if flag.starts_with('-') => {
                return fail(
                    language,
                    &format!("unknown option: {flag}"),
                    &format!("알 수 없는 옵션입니다: {flag}"),
                );
            }
            path if file.is_none() => file = Some(path.to_string()),
            path => {
                return fail(
                    language,
                    &format!("unexpected extra file: {path}"),
                    &format!("파일은 하나만 적어 주세요. 추가로 적힌 파일: {path}"),
                );
            }
        }
    }
    let file = match file {
        Some(file) => file,
        None => match discover_current_program(language, "build", "빌드") {
            Ok(found) => found,
            Err(code) => return code,
        },
    };

    let (path, python_source) = match transpile_file(&file, language) {
        Ok(ok) => ok,
        Err(code) => return code,
    };
    match exec::check_python(&python_source, &path, &python) {
        Ok(output) if output.status.success() => write_stderr(&output.stderr),
        Ok(output) => {
            return fail_with_details(
                language,
                "the generated Python did not pass CPython's syntax check\n\
                 hint: fix the Python syntax or indentation shown below, then build again",
                "만들어진 Python이 CPython 문법 검사를 통과하지 못했습니다\n\
                 도움말: 아래에 표시된 Python 문법이나 들여쓰기를 고친 뒤 다시 빌드하세요",
                &output.stderr,
            );
        }
        Err(error) => {
            return fail(
                language,
                &format!(
                    "couldn't start Python ({python}) to check the build: {error}\n\
                     hint: install Python 3, then run this command again\n\
                     advanced: use --python <command> only if Python has another command name"
                ),
                &format!(
                    "빌드를 검사하기 위한 Python({python})을 시작할 수 없습니다: {error}\n\
                     도움말: Python 3를 설치한 뒤 이 명령을 다시 실행하세요\n\
                     고급: Python 명령 이름이 다를 때만 --python <명령>을 사용하세요"
                ),
            );
        }
    }
    if let Some(path) = output {
        match std::fs::write(&path, &python_source) {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => fail(
                language,
                &format!("couldn't write {path}: {err}"),
                &format!("{path} 파일을 저장할 수 없습니다: {err}"),
            ),
        }
    } else {
        print!("{python_source}");
        ExitCode::SUCCESS
    }
}

/// `nme check`: transpile, then ask CPython to compile without executing.
fn command_check(args: &[String], language: MessageLanguage) -> ExitCode {
    let mut python = DEFAULT_PYTHON.to_string();
    let mut file = None;
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--python" => match rest.next() {
                Some(command) => python.clone_from(command),
                None => {
                    return fail(
                        language,
                        &format!("--python needs a command, e.g. --python {DEFAULT_PYTHON}"),
                        &format!(
                            "--python 뒤에 Python 명령이 필요합니다. 예: --python {DEFAULT_PYTHON}"
                        ),
                    );
                }
            },
            flag if flag.starts_with('-') => {
                return fail(
                    language,
                    &format!("unknown option: {flag}"),
                    &format!("알 수 없는 옵션입니다: {flag}"),
                );
            }
            path if file.is_none() => file = Some(path.to_string()),
            path => {
                return fail(
                    language,
                    &format!("unexpected extra file: {path}"),
                    &format!("파일은 하나만 적어 주세요. 추가로 적힌 파일: {path}"),
                );
            }
        }
    }
    let file = match file {
        Some(file) => file,
        None => match discover_current_program(language, "check", "검사") {
            Ok(found) => found,
            Err(code) => return code,
        },
    };
    let (path, python_source) = match transpile_file(&file, language) {
        Ok(ok) => ok,
        Err(code) => return code,
    };
    match exec::check_python(&python_source, &path, &python) {
        Ok(output) if output.status.success() => {
            write_stderr(&output.stderr);
            ExitCode::SUCCESS
        }
        Ok(output) => fail_with_details(
            language,
            "CPython found a syntax or indentation problem in the generated program\n\
             hint: fix the problem shown below, then check again",
            "CPython이 만들어진 프로그램에서 문법 또는 들여쓰기 문제를 찾았습니다\n\
             도움말: 아래에 표시된 문제를 고친 뒤 다시 검사하세요",
            &output.stderr,
        ),
        Err(error) => fail(
            language,
            &format!(
                "couldn't start Python ({python}): {error}\n\
                 hint: install Python 3, then run this command again\n\
                 advanced: use --python <command> only if Python has another command name"
            ),
            &format!(
                "Python({python})을 시작할 수 없습니다: {error}\n\
                 도움말: Python 3를 설치한 뒤 이 명령을 다시 실행하세요\n\
                 고급: Python 명령 이름이 다를 때만 --python <명령>을 사용하세요"
            ),
        ),
    }
}

fn is_nme_path(path: &str) -> bool {
    Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("nme"))
}

fn is_direct_program(path: &str) -> bool {
    let path = Path::new(path);
    path.exists()
        || is_nme_path(path.to_string_lossy().as_ref())
        || (path.extension().is_none() && path.with_extension("nme").exists())
}

fn resolve_nme_path(path: &Path) -> std::path::PathBuf {
    if path.exists() || path.extension().is_some() {
        path.to_path_buf()
    } else {
        path.with_extension("nme")
    }
}

/// Picks a program when no file name was given. With no `.nme` file in the
/// current folder it explains what to do; with exactly one it returns it;
/// with several it lists them and asks which one to use.
fn discover_current_program(
    language: MessageLanguage,
    action_en: &str,
    action_ko: &str,
) -> Result<String, ExitCode> {
    let mut found: Vec<String> = match std::fs::read_dir(".") {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().map(|kind| kind.is_file()).unwrap_or(false))
            .filter_map(|entry| {
                let name = entry.file_name().to_string_lossy().into_owned();
                (is_nme_path(&name) && !name.starts_with('.')).then_some(name)
            })
            .collect(),
        Err(err) => {
            return Err(fail(
                language,
                &format!("couldn't read the current folder: {err}"),
                &format!("현재 폴더를 읽을 수 없습니다: {err}"),
            ));
        }
    };
    found.sort();
    match found.as_slice() {
        [] => Err(fail(
            language,
            &format!(
                "no .nme program here. Create one (e.g. hello.nme), or name the file:\n\
                 nme {action_en} hello"
            ),
            &format!(
                "여기에 .nme 프로그램이 없어요. 파일을 만들거나(예: hello.nme) 이름을 적어 주세요:\n\
                 nme {action_ko} hello"
            ),
        )),
        [only] => Ok(only.clone()),
        many => {
            if language == MessageLanguage::KoreanAndEnglish {
                println!("현재 폴더의 .nme 프로그램:");
            } else {
                println!(".nme programs in this folder:");
            }
            for (index, name) in many.iter().enumerate() {
                println!("  {}. {name}", index + 1);
            }
            if language == MessageLanguage::KoreanAndEnglish {
                print!("어느 것을 선택할까요? (1-{}) ", many.len());
            } else {
                print!("Which one? (1-{}) ", many.len());
            }
            std::io::stdout().flush().ok();
            let mut answer = String::new();
            if std::io::stdin().read_line(&mut answer).is_err() {
                return Err(fail(
                    language,
                    "couldn't read your answer",
                    "대답을 읽을 수 없습니다",
                ));
            }
            let answer = answer.trim();
            if let Ok(number) = answer.parse::<usize>() {
                if let Some(name) = many.get(number.checked_sub(1).unwrap_or(usize::MAX)) {
                    return Ok(name.clone());
                }
            }
            if let Some(name) = many.iter().find(|name| name.eq_ignore_ascii_case(answer)) {
                return Ok(name.clone());
            }
            Err(fail(
                language,
                &format!(
                    "`{answer}` is not one of the programs above.\n\
                     Tip: `nme {action_en} <file>` picks a program by name."
                ),
                &format!(
                    "`{answer}`은(는) 위 목록에 없어요.\n\
                     팁: `nme {action_ko} <파일>`처럼 이름을 적으면 됩니다."
                ),
            ))
        }
    }
}

fn contains_korean(text: &str) -> bool {
    text.chars().any(|character| {
        matches!(
            character,
            '\u{1100}'..='\u{11ff}' | '\u{3130}'..='\u{318f}' | '\u{ac00}'..='\u{d7af}'
        )
    })
}

fn exit_code(status: std::process::ExitStatus) -> ExitCode {
    ExitCode::from(u8::try_from(status.code().unwrap_or(1)).unwrap_or(1))
}

/// Reads and transpiles one `.nme` file, reporting all problems nicely.
fn transpile_file(
    file: &str,
    language: MessageLanguage,
) -> Result<(std::path::PathBuf, String), ExitCode> {
    let path = resolve_nme_path(Path::new(file));
    let shown_path = path.to_string_lossy();
    let source = match std::fs::read_to_string(&path) {
        Ok(source) => source,
        Err(err) => {
            return Err(fail(
                language,
                &format!(
                    "couldn't read {shown_path}: {err}\n\
                     hint: create {shown_path} in this folder, or run `nme r` to run the\n\
                     single .nme program here"
                ),
                &format!(
                    "{shown_path} 파일을 읽을 수 없습니다: {err}\n\
                     도움말: 이 폴더에 {shown_path} 파일을 만들거나, `nme r`을 실행하면\n\
                     이 폴더에 있는 .nme 프로그램을 실행합니다"
                ),
            ));
        }
    };
    match nme_core::transpile(&source) {
        Ok(python) => Ok((path, python)),
        Err(problems) => {
            eprint!(
                "{}",
                render_diagnostics(&problems, &source, &shown_path, language)
            );
            Err(ExitCode::FAILURE)
        }
    }
}

fn render_diagnostics(
    problems: &[nme_core::diagnostics::Diagnostic],
    source: &str,
    path: &str,
    language: MessageLanguage,
) -> String {
    match language {
        MessageLanguage::English => nme_core::diagnostics::render_all(problems, source, path),
        MessageLanguage::KoreanAndEnglish => {
            nme_core::diagnostics::render_all_bilingual(problems, source, path)
        }
    }
}

fn fail(language: MessageLanguage, english: &str, korean: &str) -> ExitCode {
    if language == MessageLanguage::KoreanAndEnglish {
        eprintln!("오류: {korean}");
    }
    eprintln!("error: {english}");
    ExitCode::FAILURE
}

fn fail_with_details(
    language: MessageLanguage,
    english: &str,
    korean: &str,
    details: &[u8],
) -> ExitCode {
    let code = fail(language, english, korean);
    write_stderr(details);
    code
}

fn write_stderr(details: &[u8]) {
    if details.is_empty() {
        return;
    }
    let stderr = std::io::stderr();
    let mut stderr = stderr.lock();
    let _ = stderr.write_all(details);
    if !details.ends_with(b"\n") {
        let _ = stderr.write_all(b"\n");
    }
}
