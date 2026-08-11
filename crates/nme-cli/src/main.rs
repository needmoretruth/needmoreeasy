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

use std::fmt::Write as _;
use std::io::Write as _;
use std::path::{Path, PathBuf};
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

Program names may be shortened while they stay unique: `nme r gue` runs
`guessing-game.nme`. If several programs match, nme lists them and asks you
to type more of the name.

MORE COMMANDS:
    nme compile hello -o hello    Build an executable with Nuitka
    nme convert app.py [options]  Convert safe Python patterns to NME
        --level advanced|beginner|sentence
        --language en|ko
        -o <output.nme>
    nme modules                   Show bundled modules and versions
    nme native run hello          Compile a core-subset program to native code
    nme native build hello -o h   and run it / keep the executable and C
    nme ko E0001                  Korean explanation of error code E0001
    nme en E0001                  English explanation of error code E0001
    nme help                      Show this help
    nme --version                 Show the version

ADVANCED OPTIONS:
    --python <command>            Override the automatically selected Python

You may mix conversational sentences, beginner syntax, and ordinary Python in
one file. English and Korean NME spellings may be mixed too. File names work
with or without .nme: type `nme run hello`, not `nme run hello.nme`. If the
exact file you type exists, it is used as-is, so Python files such as
`program.py` run too. `nme check` prints nothing when the program is fine.
Every error message carries a stable code such as E0001; `nme ko E0001` reads
its full Korean explanation and `nme en E0001` the English one.
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

프로그램 이름은 겹치지 않는 범위에서 줄여 쓸 수 있습니다: `nme r gue`는
`guessing-game.nme`를 실행합니다. 여러 프로그램이 일치하면 목록을 보여 주고
이름을 더 입력하라고 안내합니다.

더 많은 명령:
    nme 컴파일 hello -o hello    Nuitka로 실행 파일 만들기
    nme 변환 app.py [옵션]       안전한 Python 형태를 NME로 변환
        --level advanced|beginner|sentence
        --language en|ko
        -o <출력.nme>
    nme 모듈                       내장 모듈과 버전 보기
    nme 네이티브 실행 hello        코어 부분집합 프로그램을 네이티브 코드로 컴파일해 실행
    nme 네이티브 빌드 hello -o h   실행 파일과 C 소스를 저장
    nme ko E0001                   오류 코드 E0001의 한국어 설명 보기
    nme en E0001                   오류 코드 E0001의 영어 설명 보기
    nme 도움                       이 도움말 보기
    nme 버전                       버전 보기

고급 옵션:
    --python <명령>                자동으로 선택된 Python 명령 바꾸기

한 파일에 문장형, 초급 문법, 일반 Python을 섞어 쓸 수 있습니다.
영어와 한국어 NME도 섞어 쓸 수 있습니다. 파일 이름의 .nme는 생략해도
됩니다(`nme 실행 hello`처럼 쓰면 됩니다). 적은 경로 그대로 파일이 있으면
그 파일을 사용하므로 `program.py` 같은 Python 파일도 실행할 수 있습니다.
`nme 검사`는 프로그램이 정상이면 아무것도 출력하지 않습니다.
모든 오류 메시지에는 E0001 같은 안정적인 코드가 붙어 있습니다.
`nme ko E0001`은 자세한 한국어 설명을, `nme en E0001`은 영어 설명을 보여 줍니다.
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
        Some("native" | "네이티브") => {
            command_native(&args[1..], MessageLanguage::English)
        }
        Some("ko" | "error" | "에러") => {
            command_error_lookup(&args[1..], MessageLanguage::KoreanAndEnglish)
        }
        Some("en") => command_error_lookup(&args[1..], MessageLanguage::English),
        Some("버전") if args.len() == 1 => {
            print_out(&format!(
                "NME 버전: {}\nnme version: {}\n",
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_VERSION")
            ));
            ExitCode::SUCCESS
        }
        Some("--version" | "-V" | "version" | "v") if args.len() == 1 => {
            print_out(&format!("nme {}\n", env!("CARGO_PKG_VERSION")));
            ExitCode::SUCCESS
        }
        Some("--help" | "-h" | "help" | "h") => {
            print_out(HELP_ENGLISH);
            ExitCode::SUCCESS
        }
        Some("도움" | "도움말") => {
            print_bilingual_help();
            ExitCode::SUCCESS
        }
        Some(path) => match resolve_program(Path::new(path)) {
            NameResolution::Found(_) => command_run(&args, MessageLanguage::English),
            NameResolution::Ambiguous(names) => {
                let language = if contains_korean(path) {
                    MessageLanguage::KoreanAndEnglish
                } else {
                    MessageLanguage::English
                };
                let (english, korean) = ambiguous_program_message(path, &names, "run", "실행");
                fail(
                    nme_core::diagnostics::DiagnosticCode::CliAmbiguousProgramPrefix,
                    language,
                    &english,
                    &korean,
                )
            }
            NameResolution::None => {
                let language = if contains_korean(path) {
                    MessageLanguage::KoreanAndEnglish
                } else {
                    MessageLanguage::English
                };
                fail(
                    nme_core::diagnostics::DiagnosticCode::CliUnknownCommand,
                    language,
                    &format!(
                        "I don't know the command `{path}`. Run `nme help` to see the commands.\n\
                         Tip: `nme r` runs the single .nme program in the current folder."
                    ),
                    &format!(
                        "`{path}` 명령을 알 수 없습니다. `nme 도움`으로 명령을 확인하세요.\n\
                         팁: 현재 폴더에 .nme 파일이 하나뿐이면 `nme r`만으로 실행할 수 있어요."
                    ),
                )
            }
        },
        _ => {
            eprint!("{HELP_ENGLISH}");
            ExitCode::FAILURE
        }
    }
}

fn print_bilingual_help() {
    print_out(&format!("{HELP_KOREAN}\nENGLISH / 영어\n\n{HELP_ENGLISH}"));
}

fn command_modules(args: &[String], language: MessageLanguage) -> ExitCode {
    if let Some(extra) = args.first() {
        return fail(
            nme_core::diagnostics::DiagnosticCode::CliModulesExtraArgument,
            language,
            &format!("`modules` does not take `{extra}`. Try `nme modules`."),
            &format!("`모듈` 명령에는 `{extra}`을(를) 적지 않습니다. `nme 모듈`을 사용하세요."),
        );
    }
    let mut list = String::new();
    for module in nme_core::syntax::BundledModuleId::ALL {
        match language {
            MessageLanguage::English => {
                let _ = writeln!(
                    list,
                    "{}  {}  bundled, latest",
                    module.name_en(),
                    module.version()
                );
            }
            MessageLanguage::KoreanAndEnglish => {
                let _ = writeln!(
                    list,
                    "{}  {}  내장, 최신\n{}  {}  bundled, latest",
                    module.name_ko(),
                    module.version(),
                    module.name_en(),
                    module.version()
                );
            }
        }
    }
    print_out(&list);
    ExitCode::SUCCESS
}

/// `nme native run <file>` (or just `nme native <file>`) compiles the native
/// core subset of a program to C, builds it with the system C compiler, and
/// runs the executable. `nme native build <file> [-o out]` keeps the C source
/// and the executable. Programs outside the documented core subset are
/// rejected; they still run with `nme run` on CPython.
fn command_native(args: &[String], language: MessageLanguage) -> ExitCode {
    let mut action = "run";
    let mut file = None;
    let mut output = None;
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "run" | "실행" => action = "run",
            "build" | "빌드" => action = "build",
            "-o" | "--output" => match rest.next() {
                Some(path) => output = Some(path.clone()),
                None => {
                    return fail(
                        nme_core::diagnostics::DiagnosticCode::CliInvalidOptionValue,
                        language,
                        "-o needs an output path, e.g. -o hello",
                        "-o 뒤에 출력 경로가 필요합니다. 예: -o hello",
                    );
                }
            },
            flag if flag.starts_with('-') => {
                return fail(
                    nme_core::diagnostics::DiagnosticCode::CliUnknownOption,
                    language,
                    &format!("unknown option: {flag}"),
                    &format!("알 수 없는 옵션입니다: {flag}"),
                );
            }
            path if file.is_none() => file = Some(path.to_string()),
            path => {
                return fail(
                    nme_core::diagnostics::DiagnosticCode::CliUnexpectedExtraFile,
                    language,
                    &format!("unexpected extra file: {path}"),
                    &format!("파일은 하나만 적어 주세요. 추가로 적힌 파일: {path}"),
                );
            }
        }
    }
    let file = match file {
        Some(file) => file,
        None => match discover_current_program(language, "native run", "네이티브 실행") {
            Ok(found) => found,
            Err(code) => return code,
        },
    };
    let path = match resolve_program(Path::new(&file)) {
        NameResolution::Found(path) => path,
        NameResolution::Ambiguous(names) => {
            let (english, korean) =
                ambiguous_program_message(&file, &names, "native run", "네이티브 실행");
            return fail(
                nme_core::diagnostics::DiagnosticCode::CliAmbiguousProgramPrefix,
                language,
                &english,
                &korean,
            );
        }
        NameResolution::None => resolve_nme_path(Path::new(&file)),
    };
    let shown_path = path.to_string_lossy();
    let source = match std::fs::read_to_string(&path) {
        Ok(source) => source,
        Err(err) => {
            return fail(
                nme_core::diagnostics::DiagnosticCode::CliMissingProgram,
                language,
                &format!("couldn't read {shown_path}: {err}"),
                &format!("{shown_path} 파일을 읽을 수 없습니다: {err}"),
            );
        }
    };
    let c_source = match nme_native::native_compile(&source) {
        Ok(c) => c,
        Err(problems) => {
            eprint!(
                "{}",
                render_diagnostics(&problems, &source, &shown_path, language)
            );
            return ExitCode::FAILURE;
        }
    };
    let stem = path
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("program");
    let dir = std::env::temp_dir().join(format!("nme-native-run-{}", std::process::id()));
    if let Err(err) = std::fs::create_dir_all(&dir) {
        return fail(
            nme_core::diagnostics::DiagnosticCode::CliFolderReadFailed,
            language,
            &format!("couldn't create the native build folder: {err}"),
            &format!("네이티브 빌드 폴더를 만들 수 없습니다: {err}"),
        );
    }
    let c_path = dir.join(format!("{stem}.c"));
    if let Err(err) = std::fs::write(&c_path, c_source) {
        return fail(
            nme_core::diagnostics::DiagnosticCode::CliFileWriteFailed,
            language,
            &format!("couldn't write the C source: {err}"),
            &format!("C 소스를 저장할 수 없습니다: {err}"),
        );
    }
    let exe = dir.join(stem);
    let compile_status = std::process::Command::new("cc")
        .arg(&c_path)
        .arg("-o")
        .arg(&exe)
        .status();
    match compile_status {
        Ok(status) if status.success() => {}
        Ok(status) => {
            let _ = std::fs::remove_dir_all(&dir);
            return fail(
                nme_core::diagnostics::DiagnosticCode::CliNativeCompileFailed,
                language,
                &format!("the native compiler (cc) failed with {status}\n\
                          hint: install a C compiler, or run this program with `nme run`"),
                &format!("네이티브 컴파일러(cc)가 실패했습니다: {status}\n\
                          도움말: C 컴파일러를 설치하거나 `nme run`으로 실행하세요"),
            );
        }
        Err(error) => {
            let _ = std::fs::remove_dir_all(&dir);
            return fail(
                nme_core::diagnostics::DiagnosticCode::CliNativeCompileStartFailed,
                language,
                &format!("couldn't start the C compiler: {error}\n\
                          hint: install a C compiler, or run this program with `nme run`"),
                &format!("C 컴파일러를 시작할 수 없습니다: {error}\n\
                          도움말: C 컴파일러를 설치하거나 `nme run`으로 실행하세요"),
            );
        }
    }
    if action == "build" {
        let out = output
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(stem));
        let copy_exe = std::fs::copy(&exe, &out).is_ok();
        let copy_c = std::fs::copy(&c_path, out.with_extension("c")).is_ok();
        let _ = std::fs::remove_dir_all(&dir);
        if copy_exe && copy_c {
            ExitCode::SUCCESS
        } else {
            fail(
                nme_core::diagnostics::DiagnosticCode::CliFileWriteFailed,
                language,
                "couldn't write the native artifact",
                "네이티브 결과물을 저장할 수 없습니다",
            )
        }
    } else {
        let run_status = std::process::Command::new(&exe).status();
        let _ = std::fs::remove_dir_all(&dir);
        match run_status {
            Ok(status) => exit_code(status),
            Err(err) => fail(
                nme_core::diagnostics::DiagnosticCode::CliPythonStartFailed,
                language,
                &format!("couldn't run the native program: {err}"),
                &format!("네이티브 프로그램을 실행할 수 없습니다: {err}"),
            ),
        }
    }
}

/// `nme ko E0001` prints the long Korean explanation of one error code, with
/// the English explanation after it; `nme en E0001` prints English only.
/// With no code, every code is listed so a beginner can browse.
fn command_error_lookup(args: &[String], language: MessageLanguage) -> ExitCode {
    let Some(code) = args.first() else {
        let mut list = String::new();
        for code in nme_core::diagnostics::DiagnosticCode::ALL {
            let explanation = code.explanation();
            match language {
                MessageLanguage::English => {
                    let _ = writeln!(list, "{}  {}", explanation.code, explanation.title_en);
                }
                MessageLanguage::KoreanAndEnglish => {
                    let _ = writeln!(
                        list,
                        "{}  {} / {}",
                        explanation.code, explanation.title_ko, explanation.title_en
                    );
                }
            }
        }
        print_out(&list);
        return ExitCode::SUCCESS;
    };
    if args.len() > 1 {
        return fail(
            nme_core::diagnostics::DiagnosticCode::CliErrorLookupInvalidArgs,
            language,
            &format!(
                "`{code}`: one error code at a time. Try `nme ko {code}` or `nme ko` for the list."
            ),
            &format!(
                "`{code}`: 오류 코드는 한 번에 하나씩 확인할 수 있어요. `nme ko {code}` 또는 목록을 보려면 `nme ko`를 사용하세요."
            ),
        );
    }
    let Some(code) = nme_core::diagnostics::DiagnosticCode::from_code(code) else {
        return fail(
            nme_core::diagnostics::DiagnosticCode::CliErrorLookupUnknownCode,
            language,
            &format!(
                "there is no error code `{code}`. Run `nme ko` to list every code."
            ),
            &format!("`{code}` 오류 코드는 없어요. `nme ko`를 실행하면 모든 코드를 볼 수 있습니다."),
        );
    };
    let explanation = code.explanation();
    match language {
        MessageLanguage::English => print_out(&format!(
            "{} — {}\n\n{}\n\nSee it in your program: the compiler prints this code\nnext to the error, for example `error[{0}]`. You can also run\n`nme ko {0}` anytime to read this page again.\n",
            explanation.code, explanation.title_en, explanation.detail_en
        )),
        MessageLanguage::KoreanAndEnglish => print_out(&format!(
            "{} — {}\n\n{}\n\n{}\n\n{} — {}\n\n{}\n",
            explanation.code,
            explanation.title_ko,
            explanation.detail_ko,
            format!(
                "프로그램에서 이 코드를 보면 오류 옆에 `error[{}]`처럼 표시됩니다. 궁금할 때 `nme ko {}`를 다시 실행하면 이 설명을 볼 수 있습니다.",
                explanation.code, explanation.code
            ),
            explanation.code,
            explanation.title_en,
            explanation.detail_en
        )),
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
                        nme_core::diagnostics::DiagnosticCode::CliInvalidOptionValue,
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
                            nme_core::diagnostics::DiagnosticCode::CliInvalidOptionValue,
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
                        nme_core::diagnostics::DiagnosticCode::CliInvalidOptionValue,
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
                            nme_core::diagnostics::DiagnosticCode::CliInvalidOptionValue,
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
                        nme_core::diagnostics::DiagnosticCode::CliInvalidOptionValue,
                        language,
                        "-o needs a path, e.g. -o program.nme",
                        "-o 뒤에 저장할 경로가 필요합니다. 예: -o program.nme",
                    );
                }
            },
            flag if flag.starts_with('-') => {
                return fail(
                    nme_core::diagnostics::DiagnosticCode::CliUnknownOption,
                    language,
                    &format!("unknown option: {flag}"),
                    &format!("알 수 없는 옵션입니다: {flag}"),
                );
            }
            path if file.is_none() => file = Some(path.to_string()),
            path => {
                return fail(
                    nme_core::diagnostics::DiagnosticCode::CliUnexpectedExtraFile,
                    language,
                    &format!("unexpected extra file: {path}"),
                    &format!("파일은 하나만 적어 주세요. 추가로 적힌 파일: {path}"),
                );
            }
        }
    }
    let Some(file) = file else {
        return fail(
            nme_core::diagnostics::DiagnosticCode::CliConvertNeedsFile,
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
                nme_core::diagnostics::DiagnosticCode::CliFileReadFailed,
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
                nme_core::diagnostics::DiagnosticCode::CliFileWriteFailed,
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
    let source_path = match resolve_program(Path::new(&file)) {
        NameResolution::Found(path) => path,
        NameResolution::Ambiguous(names) => {
            let (english, korean) = ambiguous_program_message(&file, &names, "compile", "컴파일");
            return fail(
                nme_core::diagnostics::DiagnosticCode::CliAmbiguousProgramPrefix,
                language,
                &english,
                &korean,
            );
        }
        NameResolution::None => resolve_nme_path(Path::new(&file)),
    };
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
            nme_core::diagnostics::DiagnosticCode::CliRefuseOverwrite,
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
    let compiled = match transpile_file(&file, language, "compile", "컴파일") {
        Ok(ok) => ok,
        Err(code) => return code,
    };
    if !compiled.imports.is_empty() {
        return fail(
            nme_core::diagnostics::DiagnosticCode::CliInvalidOptionValue,
            language,
            "module imports are not supported by `nme compile` yet; run the program with `nme run`",
            "`nme compile`은 아직 모듈 가져오기를 지원하지 않습니다. `nme run`으로 실행하세요",
        );
    }
    match exec::compile_native(&compiled.source, stem, &python, &output) {
        Ok(status) if status.success() => {
            if output.exists() {
                ExitCode::SUCCESS
            } else {
                fail(
                    nme_core::diagnostics::DiagnosticCode::CliNativeCompileFailed,
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
            nme_core::diagnostics::DiagnosticCode::CliNativeCompileFailed,
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
            nme_core::diagnostics::DiagnosticCode::CliNativeCompileStartFailed,
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
                        nme_core::diagnostics::DiagnosticCode::CliInvalidOptionValue,
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
                        nme_core::diagnostics::DiagnosticCode::CliInvalidOptionValue,
                        language,
                        "-o needs an executable path, e.g. -o hello",
                        "-o 뒤에 만들 실행 파일 경로가 필요합니다. 예: -o hello",
                    ));
                }
            },
            flag if flag.starts_with('-') => {
                return Err(fail(
                    nme_core::diagnostics::DiagnosticCode::CliUnknownOption,
                    language,
                    &format!("unknown option: {flag}"),
                    &format!("알 수 없는 옵션입니다: {flag}"),
                ));
            }
            path if file.is_none() => file = Some(path.to_string()),
            path => {
                return Err(fail(
                    nme_core::diagnostics::DiagnosticCode::CliUnexpectedExtraFile,
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
                        nme_core::diagnostics::DiagnosticCode::CliInvalidOptionValue,
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
                    nme_core::diagnostics::DiagnosticCode::CliUnknownOption,
                    language,
                    &format!("unknown option: {flag}"),
                    &format!("알 수 없는 옵션입니다: {flag}"),
                );
            }
            path if file.is_none() => file = Some(path.to_string()),
            path => {
                return fail(
                    nme_core::diagnostics::DiagnosticCode::CliUnexpectedExtraFile,
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

    let compiled = match transpile_file(&file, language, "run", "실행") {
        Ok(ok) => ok,
        Err(code) => return code,
    };
    let main_dir = compiled
        .path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    let modules = match transpile_modules(&main_dir, &compiled.imports, language) {
        Ok(modules) => modules,
        Err(code) => return code,
    };
    let module_dir = if modules.is_empty() {
        None
    } else {
        match write_modules_to_temp(&modules) {
            Ok(dir) => Some(dir),
            Err(code) => return code,
        }
    };
    let run_status = exec::run_python(&compiled.source, &compiled.path, &python, module_dir.as_deref());
    if let Some(dir) = module_dir {
        let _ = std::fs::remove_dir_all(dir);
    }
    match run_status {
        Ok(status) => exit_code(status),
        Err(err) => fail(
            nme_core::diagnostics::DiagnosticCode::CliPythonStartFailed,
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
                        nme_core::diagnostics::DiagnosticCode::CliInvalidOptionValue,
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
                        nme_core::diagnostics::DiagnosticCode::CliInvalidOptionValue,
                        language,
                        "-o needs a path, e.g. -o hello.py",
                        "-o 뒤에 저장할 경로가 필요합니다. 예: -o hello.py",
                    );
                }
            },
            flag if flag.starts_with('-') => {
                return fail(
                    nme_core::diagnostics::DiagnosticCode::CliUnknownOption,
                    language,
                    &format!("unknown option: {flag}"),
                    &format!("알 수 없는 옵션입니다: {flag}"),
                );
            }
            path if file.is_none() => file = Some(path.to_string()),
            path => {
                return fail(
                    nme_core::diagnostics::DiagnosticCode::CliUnexpectedExtraFile,
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

    let compiled = match transpile_file(&file, language, "build", "빌드") {
        Ok(ok) => ok,
        Err(code) => return code,
    };
    let main_dir = compiled
        .path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    let modules = match transpile_modules(&main_dir, &compiled.imports, language) {
        Ok(modules) => modules,
        Err(code) => return code,
    };
    if let Err(code) = check_modules(&modules, language, "build") {
        return code;
    }
    match exec::check_python(&compiled.source, &compiled.path, &python) {
        Ok(output) if output.status.success() => write_stderr(&output.stderr),
        Ok(output) => {
            return fail_with_details(
                nme_core::diagnostics::DiagnosticCode::CliCpythonValidationFailed,
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
                nme_core::diagnostics::DiagnosticCode::CliPythonStartFailed,
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
        match std::fs::write(&path, &compiled.source) {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => fail(
                nme_core::diagnostics::DiagnosticCode::CliFileWriteFailed,
                language,
                &format!("couldn't write {path}: {err}"),
                &format!("{path} 파일을 저장할 수 없습니다: {err}"),
            ),
        }
    } else {
        print!("{}", compiled.source);
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
                        nme_core::diagnostics::DiagnosticCode::CliInvalidOptionValue,
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
                    nme_core::diagnostics::DiagnosticCode::CliUnknownOption,
                    language,
                    &format!("unknown option: {flag}"),
                    &format!("알 수 없는 옵션입니다: {flag}"),
                );
            }
            path if file.is_none() => file = Some(path.to_string()),
            path => {
                return fail(
                    nme_core::diagnostics::DiagnosticCode::CliUnexpectedExtraFile,
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
    let compiled = match transpile_file(&file, language, "check", "검사") {
        Ok(ok) => ok,
        Err(code) => return code,
    };
    let main_dir = compiled
        .path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    let modules = match transpile_modules(&main_dir, &compiled.imports, language) {
        Ok(modules) => modules,
        Err(code) => return code,
    };
    if let Err(code) = check_modules(&modules, language, "check") {
        return code;
    }
    match exec::check_python(&compiled.source, &compiled.path, &python) {
        Ok(output) if output.status.success() => {
            write_stderr(&output.stderr);
            ExitCode::SUCCESS
        }
        Ok(output) => fail_with_details(
            nme_core::diagnostics::DiagnosticCode::CliCpythonValidationFailed,
            language,
            "CPython found a syntax or indentation problem in the generated program\n\
             hint: fix the problem shown below, then check again",
            "CPython이 만들어진 프로그램에서 문법 또는 들여쓰기 문제를 찾았습니다\n\
             도움말: 아래에 표시된 문제를 고친 뒤 다시 검사하세요",
            &output.stderr,
        ),
        Err(error) => fail(
            nme_core::diagnostics::DiagnosticCode::CliPythonStartFailed,
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

/// Possible real files for a typed name. The exact path wins, then the name
/// with `.nme` appended (`hello.ko` → `hello.ko.nme`), then the name with its
/// extension replaced (`hello` → `hello.nme`, `app.py` → `app.nme`).
fn candidate_paths(path: &Path) -> [std::path::PathBuf; 3] {
    let appended = PathBuf::from(format!("{}.nme", path.display()));
    let replaced = path.with_extension("nme");
    [path.to_path_buf(), appended, replaced]
}

fn resolve_nme_path(path: &Path) -> std::path::PathBuf {
    candidate_paths(path)
        .iter()
        .find(|candidate| candidate.exists())
        .cloned()
        .unwrap_or_else(|| path.to_path_buf())
}

/// How a typed program name resolved to a real file. A unique name may be
/// shortened while it still names exactly one `.nme` program; when several
/// programs match, they are reported instead of guessing.
enum NameResolution {
    Found(std::path::PathBuf),
    None,
    Ambiguous(Vec<String>),
}

/// Sorted `.nme` file names in a folder (hidden files excluded). Returns
/// `None` when the folder cannot be read.
fn sibling_nme_names(folder: &Path) -> Option<Vec<String>> {
    let mut names: Vec<String> = std::fs::read_dir(folder)
        .ok()?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|kind| kind.is_file()).unwrap_or(false))
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            (is_nme_path(&name) && !name.starts_with('.')).then_some(name)
        })
        .collect();
    names.sort();
    Some(names)
}

fn file_stem_lower(name: &str) -> String {
    Path::new(name)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase()
}

/// Resolves a typed program name. Exact candidate paths win first; then a
/// case-insensitive exact stem match; then a unique case-insensitive prefix
/// of a sibling `.nme` stem. Several prefix matches are `Ambiguous`.
fn resolve_program(typed: &Path) -> NameResolution {
    if let Some(found) = candidate_paths(typed)
        .iter()
        .find(|candidate| candidate.exists())
    {
        return NameResolution::Found(found.clone());
    }
    let Some(wanted) = typed
        .file_name()
        .map(|name| name.to_string_lossy().to_lowercase())
    else {
        return NameResolution::None;
    };
    if wanted.is_empty() {
        return NameResolution::None;
    }
    let folder = typed
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let Some(names) = sibling_nme_names(folder) else {
        return NameResolution::None;
    };
    let exact: Vec<String> = names
        .iter()
        .filter(|name| file_stem_lower(name) == wanted)
        .cloned()
        .collect();
    match exact.len() {
        1 => return NameResolution::Found(folder.join(&exact[0])),
        _ if exact.len() > 1 => return NameResolution::Ambiguous(exact),
        _ => {}
    }
    let prefixes: Vec<String> = names
        .iter()
        .filter(|name| file_stem_lower(name).starts_with(&wanted))
        .cloned()
        .collect();
    match prefixes.len() {
        1 => NameResolution::Found(folder.join(&prefixes[0])),
        0 => NameResolution::None,
        _ => NameResolution::Ambiguous(prefixes),
    }
}

fn ambiguous_program_message(
    wanted: &str,
    names: &[String],
    action_en: &str,
    action_ko: &str,
) -> (String, String) {
    let listed = names.join(", ");
    let stem = names
        .first()
        .map(|name| name.trim_end_matches(".nme"))
        .unwrap_or(wanted);
    let english = format!(
        "several programs match `{wanted}`: {listed}\n\
         hint: type more of the name, e.g. `nme {action_en} {stem}`"
    );
    let korean = format!(
        "`{wanted}`(와)과 일치하는 프로그램이 여러 개예요: {listed}\n\
         도움말: 이름을 더 길게 적어 주세요. 예: `nme {action_ko} {stem}`"
    );
    (english, korean)
}

/// Finds a nearby `.nme` program that the typed name almost matches, so a
/// beginner who misspells a file name gets a "did you mean" hint instead of a
/// bare read error.
fn suggest_program(typed: &Path) -> Option<String> {
    let folder = typed
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let wanted = typed.file_name()?.to_string_lossy().to_lowercase();
    let names = sibling_nme_names(folder)?;
    let mut matches: Vec<&String> = names
        .iter()
        .filter(|name| {
            let lower = name.to_lowercase();
            let stem = file_stem_lower(name);
            lower == wanted || stem == wanted || lower.contains(&wanted)
        })
        .collect();
    matches.sort_by_key(|name| {
        let lower = name.to_lowercase();
        let stem = file_stem_lower(name);
        if lower == wanted {
            0
        } else if stem == wanted {
            1
        } else {
            2
        }
    });
    matches.first().map(|name| (*name).clone())
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
                nme_core::diagnostics::DiagnosticCode::CliFolderReadFailed,
                language,
                &format!("couldn't read the current folder: {err}"),
                &format!("현재 폴더를 읽을 수 없습니다: {err}"),
            ));
        }
    };
    found.sort();
    match found.as_slice() {
        [] => Err(fail(
            nme_core::diagnostics::DiagnosticCode::CliNoProgramHere,
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
                    nme_core::diagnostics::DiagnosticCode::CliPickAnswerUnreadable,
                    language,
                    "couldn't read your answer",
                    "대답을 읽을 수 없습니다",
                ));
            }
            let answer = answer.trim();
            if answer.is_empty() {
                return Err(fail(
                    nme_core::diagnostics::DiagnosticCode::CliEmptyPickAnswer,
                    language,
                    "no answer given — type a number or a program name, then press Enter",
                    "대답이 입력되지 않았어요 — 숫자나 프로그램 이름을 적고 Enter를 누르세요",
                ));
            }
            if let Ok(number) = answer.parse::<usize>() {
                if let Some(name) = many.get(number.checked_sub(1).unwrap_or(usize::MAX)) {
                    return Ok(name.clone());
                }
            }
            if let Some(name) = many.iter().find(|name| name.eq_ignore_ascii_case(answer)) {
                return Ok(name.clone());
            }
            if let Some(name) = many
                .iter()
                .find(|name| file_stem_lower(name).eq_ignore_ascii_case(answer))
            {
                return Ok(name.clone());
            }
            let lower_answer = answer.to_lowercase();
            let prefixes: Vec<&String> = many
                .iter()
                .filter(|name| file_stem_lower(name).starts_with(&lower_answer))
                .collect();
            match prefixes.len() {
                1 => return Ok(prefixes[0].clone()),
                0 => {}
                _ => {
                    let listed = prefixes
                        .iter()
                        .map(|name| (*name).clone())
                        .collect::<Vec<String>>()
                        .join(", ");
                    return Err(fail(
                        nme_core::diagnostics::DiagnosticCode::CliAmbiguousPickAnswer,
                        language,
                        &format!(
                            "`{answer}` matches several programs: {listed}\n\
                             Tip: pick a number from the list above, or type more of the name"
                        ),
                        &format!(
                            "`{answer}`(와)과 일치하는 프로그램이 여러 개예요: {listed}\n\
                             팁: 위 목록에서 숫자를 고르거나 이름을 더 길게 입력하세요"
                        ),
                    ));
                }
            }
            Err(fail(
                nme_core::diagnostics::DiagnosticCode::CliInvalidPickAnswer,
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
/// A transpiled program plus its `.nme` module imports, ready for the CLI to
/// transpile those modules too and make them importable at runtime.
struct Compiled {
    path: std::path::PathBuf,
    source: String,
    imports: Vec<nme_core::ModuleImport>,
}

fn transpile_file(
    file: &str,
    language: MessageLanguage,
    action_en: &str,
    action_ko: &str,
) -> Result<Compiled, ExitCode> {
    let path = match resolve_program(Path::new(file)) {
        NameResolution::Found(path) => path,
        NameResolution::Ambiguous(names) => {
            let (english, korean) =
                ambiguous_program_message(file, &names, action_en, action_ko);
            return Err(fail(
                nme_core::diagnostics::DiagnosticCode::CliAmbiguousProgramPrefix,
                language,
                &english,
                &korean,
            ));
        }
        NameResolution::None => resolve_nme_path(Path::new(file)),
    };
    let shown_path = path.to_string_lossy();
    let source = match std::fs::read_to_string(&path) {
        Ok(source) => source,
        Err(err) => {
            if path.is_dir() {
                return Err(fail(
                    nme_core::diagnostics::DiagnosticCode::CliFolderNotProgram,
                    language,
                    &format!(
                        "`{}` is a folder, not a program.\n\
                         hint: run `nme r` inside a folder that contains a .nme program, or type the program name",
                        path.display()
                    ),
                    &format!(
                        "`{}`은(는) 폴더이지 프로그램이 아니에요.\n\
                         도움말: .nme 프로그램이 있는 폴더에서 `nme r`을 실행하거나, 프로그램 이름을 적어 주세요",
                        path.display()
                    ),
                ));
            }
            let suggestion = suggest_program(&path);
            let create_name = PathBuf::from(format!("{}.nme", shown_path));
            let english_hint = match &suggestion {
                Some(name) => format!(
                    "hint: did you mean `{name}`? Try `nme run {stem}`",
                    stem = name.trim_end_matches(".nme")
                ),
                None => format!(
                    "hint: create a program named `{}` in this folder, or run `nme r`\n\
                     to run the single .nme program here",
                    create_name.display()
                ),
            };
            let korean_hint = match &suggestion {
                Some(name) => format!(
                    "도움말: `{name}`을(를) 찾으셨나요? `nme 실행 {stem}`으로 실행할 수 있어요",
                    stem = name.trim_end_matches(".nme")
                ),
                None => format!(
                    "도움말: 이 폴더에 `{}` 프로그램을 만들거나, `nme r`을 실행하면\n\
                     이 폴더에 있는 .nme 프로그램을 실행합니다",
                    create_name.display()
                ),
            };
            return Err(fail(
                nme_core::diagnostics::DiagnosticCode::CliMissingProgram,
                language,
                &format!("couldn't read {shown_path}: {err}\n{english_hint}"),
                &format!("{shown_path} 파일을 읽을 수 없습니다: {err}\n{korean_hint}"),
            ));
        }
    };
    match nme_core::transpile_with_modules(&source) {
        Ok((python, imports)) => Ok(Compiled {
            path,
            source: python,
            imports,
        }),
        Err(problems) => {
            eprint!(
                "{}",
                render_diagnostics(&problems, &source, &shown_path, language)
            );
            Err(ExitCode::FAILURE)
        }
    }
}

/// Transpiles every `.nme` module the main program imports (transitively),
/// returning `(stem, python_source)` pairs. Duplicate stems are rejected
/// because the generated Python imports modules by their file stem.
fn transpile_modules(
    main_dir: &Path,
    imports: &[nme_core::ModuleImport],
    language: MessageLanguage,
) -> Result<Vec<(String, String)>, ExitCode> {
    let mut modules = Vec::<(String, String)>::new();
    let mut seen = std::collections::HashSet::new();
    let mut pending: Vec<nme_core::ModuleImport> = imports.to_vec();
    while let Some(import) = pending.pop() {
        if !seen.insert(import.file.clone()) {
            continue;
        }
        let module_path = main_dir.join(&import.file);
        let stem = module_path
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("module")
            .to_string();
        if let Some((existing, _)) = modules.iter().find(|(name, _)| name == &stem) {
            return Err(fail(
                nme_core::diagnostics::DiagnosticCode::CliInvalidOptionValue,
                language,
                &format!(
                    "two imported modules are both named `{existing}`; rename one of them"
                ),
                &format!("가져온 모듈 두 개가 모두 `{existing}`라는 이름입니다. 하나를 바꾸세요"),
            ));
        }
        let source = match std::fs::read_to_string(&module_path) {
            Ok(source) => source,
            Err(err) => {
                return Err(fail(
                    nme_core::diagnostics::DiagnosticCode::CliMissingProgram,
                    language,
                    &format!(
                        "couldn't read module {}: {err}\n\
                         hint: the module must sit next to the main program",
                        module_path.display()
                    ),
                    &format!(
                        "{} 모듈을 읽을 수 없습니다: {err}\n\
                         도움말: 모듈은 주 프로그램 옆에 있어야 합니다",
                        module_path.display()
                    ),
                ));
            }
        };
        let (python, sub_imports) = match nme_core::transpile_with_modules(&source) {
            Ok(ok) => ok,
            Err(problems) => {
                eprint!(
                    "{}",
                    render_diagnostics(
                        &problems,
                        &source,
                        &module_path.to_string_lossy(),
                        language
                    )
                );
                return Err(ExitCode::FAILURE);
            }
        };
        modules.push((stem, python));
        pending.extend(sub_imports);
    }
    Ok(modules)
}

/// Asks CPython to validate each transpiled module's Python, reporting the
/// module file in any failure.
fn check_modules(modules: &[(String, String)], language: MessageLanguage, action: &str) -> Result<(), ExitCode> {
    for (stem, python) in modules {
        let module_path = PathBuf::from(format!("{stem}.nme"));
        match exec::check_python(python, &module_path, DEFAULT_PYTHON) {
            Ok(output) if output.status.success() => {}
            Ok(output) => {
                return Err(fail_with_details(
                    nme_core::diagnostics::DiagnosticCode::CliCpythonValidationFailed,
                    language,
                    &format!(
                        "the module {stem} did not pass CPython's syntax check during `nme {action}`"
                    ),
                    &format!(
                        "{stem} 모듈이 `nme {action}` 중 CPython 문법 검사를 통과하지 못했습니다"
                    ),
                    &output.stderr,
                ));
            }
            Err(error) => {
                return Err(fail(
                    nme_core::diagnostics::DiagnosticCode::CliPythonStartFailed,
                    language,
                    &format!("couldn't start Python to check the module {stem}: {error}"),
                    &format!("{stem} 모듈을 검사할 Python을 시작할 수 없습니다: {error}"),
                ));
            }
        }
    }
    Ok(())
}

/// Writes transpiled modules to a fresh temporary folder and returns it.
fn write_modules_to_temp(modules: &[(String, String)]) -> Result<PathBuf, ExitCode> {
    let dir = std::env::temp_dir().join(format!("nme-modules-{}", std::process::id()));
    if let Err(err) = std::fs::create_dir_all(&dir) {
        return Err(fail(
            nme_core::diagnostics::DiagnosticCode::CliFolderReadFailed,
            MessageLanguage::English,
            &format!("couldn't create the module folder: {err}"),
            &format!("모듈 폴더를 만들 수 없습니다: {err}"),
        ));
    }
    for (stem, python) in modules {
        if let Err(err) = std::fs::write(dir.join(format!("{stem}.py")), python) {
            return Err(fail(
                nme_core::diagnostics::DiagnosticCode::CliFileWriteFailed,
                MessageLanguage::English,
                &format!("couldn't write the module {stem}.py: {err}"),
                &format!("{stem}.py 모듈을 저장할 수 없습니다: {err}"),
            ));
        }
    }
    Ok(dir)
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

fn fail(
    code: nme_core::diagnostics::DiagnosticCode,
    language: MessageLanguage,
    english: &str,
    korean: &str,
) -> ExitCode {
    if language == MessageLanguage::KoreanAndEnglish {
        eprintln!("오류[{}]: {korean}", code.code());
    }
    eprintln!("error[{}]: {english}", code.code());
    ExitCode::FAILURE
}

fn fail_with_details(
    code: nme_core::diagnostics::DiagnosticCode,
    language: MessageLanguage,
    english: &str,
    korean: &str,
    details: &[u8],
) -> ExitCode {
    let exit = fail(code, language, english, korean);
    write_stderr(details);
    exit
}

/// Prints without panicking when the reader closes the pipe early
/// (for example `nme help | head`). Rust's std ignores SIGPIPE, so a broken
/// pipe surfaces as a write error; we treat it like a normal exit.
fn print_out(text: &str) {
    let mut stdout = std::io::stdout();
    let _ = stdout.write_all(text.as_bytes());
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
