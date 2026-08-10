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

use std::path::Path;
use std::process::ExitCode;

const HELP: &str = r"nme — NeedMoreEasy: programming, easier than Python.

USAGE:
    nme <file.nme> [--python <command>]       Shortcut for `nme run`
    nme run <file.nme> [--python <command>]   Run an NME program with Python
    nme build <file.nme> [-o <output.py>]     Transpile to Python and print it
    nme compile <file.nme> [-o <executable>]  Build a native executable (Nuitka)
        --python <command>                     Select Python for the native build
    nme check <file.nme> [--python <command>] Check with NME and CPython
    nme convert <file.py> [options]           Convert Python to an NME level
        --level advanced|beginner|sentence    Choose the syntax level
        --language en|ko                      Choose generated words (default: en)
        -o <output.nme>                       Write instead of printing
    nme modules                               Show bundled modules and versions
    nme --help                                Show this help
    nme --version                             Show the version

Every valid Python program is already a valid NME program. NME offers advanced
Python, compact beginner syntax, and conversational sentence syntax.
English and Korean may be mixed. See README.md or README.ko.md to begin.
Python defaults to `py` on Windows and `python3` on macOS and Linux.
";

const DEFAULT_PYTHON: &str = if cfg!(windows) { "py" } else { "python3" };

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("run" | "실행") => command_run(&args[1..]),
        Some("build" | "빌드") => command_build(&args[1..]),
        Some("compile" | "컴파일") => command_compile(&args[1..]),
        Some("check" | "검사") => command_check(&args[1..]),
        Some("convert" | "변환") => command_convert(&args[1..]),
        Some("modules" | "module" | "모듈") if args.len() == 1 => command_modules(),
        Some("--version" | "-V") => {
            println!("nme {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Some("--help" | "-h") => {
            print!("{HELP}");
            ExitCode::SUCCESS
        }
        Some(path) if is_nme_path(path) => command_run(&args),
        _ => {
            eprint!("{HELP}");
            ExitCode::FAILURE
        }
    }
}

fn command_modules() -> ExitCode {
    println!(
        "random / 랜덤  {}  bundled, latest / 내장, 최신",
        nme_core::syntax::RANDOM_MODULE_VERSION
    );
    ExitCode::SUCCESS
}

fn command_convert(args: &[String]) -> ExitCode {
    let mut file = None;
    let mut output = None;
    let mut level = nme_core::SyntaxLevel::Sentence;
    let mut language = nme_core::Language::English;
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--level" => {
                let Some(value) = rest.next() else {
                    return fail("--level needs advanced, beginner, or sentence");
                };
                level = match value.as_str() {
                    "advanced" | "고급" => nme_core::SyntaxLevel::Advanced,
                    "beginner" | "초급" => nme_core::SyntaxLevel::Beginner,
                    "sentence" | "문장" | "문장형" => nme_core::SyntaxLevel::Sentence,
                    _ => return fail("--level needs advanced, beginner, or sentence"),
                };
            }
            "--language" | "--lang" => {
                let Some(value) = rest.next() else {
                    return fail("--language needs en or ko");
                };
                language = match value.as_str() {
                    "en" | "english" | "영어" => nme_core::Language::English,
                    "ko" | "korean" | "한국어" => nme_core::Language::Korean,
                    _ => return fail("--language needs en or ko"),
                };
            }
            "-o" | "--output" => match rest.next() {
                Some(path) => output = Some(path.clone()),
                None => return fail("-o needs a path, e.g. -o program.nme"),
            },
            flag if flag.starts_with('-') => return fail(&format!("unknown option: {flag}")),
            path if file.is_none() => file = Some(path.to_string()),
            path => return fail(&format!("unexpected extra file: {path}")),
        }
    }
    let Some(file) = file else {
        return fail("which Python file should I convert? e.g. nme convert app.py");
    };
    let source = match std::fs::read_to_string(&file) {
        Ok(source) => source,
        Err(error) => return fail(&format!("couldn't read {file}: {error}")),
    };
    let conversion = match nme_core::convert_python(&source, level, language) {
        Ok(conversion) => conversion,
        Err(problems) => {
            eprint!(
                "{}",
                nme_core::diagnostics::render_all(&problems, &source, &file)
            );
            return ExitCode::FAILURE;
        }
    };
    if let Some(path) = output {
        match std::fs::write(&path, conversion.source) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => fail(&format!("couldn't write {path}: {error}")),
        }
    } else {
        print!("{}", conversion.source);
        ExitCode::SUCCESS
    }
}

fn command_compile(args: &[String]) -> ExitCode {
    let mut python = DEFAULT_PYTHON.to_string();
    let mut file = None;
    let mut output = None;
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--python" => match rest.next() {
                Some(command) => python.clone_from(command),
                None => {
                    return fail(&format!(
                        "--python needs a command, e.g. --python {DEFAULT_PYTHON}"
                    ));
                }
            },
            "-o" | "--output" => match rest.next() {
                Some(path) => output = Some(std::path::PathBuf::from(path)),
                None => return fail("-o needs an executable path, e.g. -o hello"),
            },
            flag if flag.starts_with('-') => return fail(&format!("unknown option: {flag}")),
            path if file.is_none() => file = Some(path.to_string()),
            path => return fail(&format!("unexpected extra file: {path}")),
        }
    }
    let Some(file) = file else {
        return fail("which file should I compile? e.g. nme compile hello.nme");
    };
    let source_path = Path::new(&file);
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
        return fail(&format!(
            "refusing to overwrite existing output: {}",
            output.display()
        ));
    }
    let (_, python_source) = match transpile_file(&file) {
        Ok(ok) => ok,
        Err(code) => return code,
    };
    match exec::compile_native(&python_source, stem, &python, &output) {
        Ok(status) if status.success() => {
            if output.exists() {
                ExitCode::SUCCESS
            } else {
                fail(&format!(
                    "native compiler succeeded but did not create {}",
                    output.display()
                ))
            }
        }
        Ok(status) => fail(&format!(
            "native compilation failed with {status}\n\
             hint: install Nuitka with `{python} -m pip install nuitka` and make sure a C compiler is available"
        )),
        Err(error) => fail(&format!(
            "couldn't start native compilation: {error}\n\
             hint: install Python and Nuitka, or choose Python with --python <command>"
        )),
    }
}

/// `nme run`: transpile, then execute with the real Python runtime.
fn command_run(args: &[String]) -> ExitCode {
    let mut python = DEFAULT_PYTHON.to_string();
    let mut file = None;
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--python" => match rest.next() {
                Some(command) => python.clone_from(command),
                None => {
                    return fail(&format!(
                        "--python needs a command, e.g. --python {DEFAULT_PYTHON}"
                    ));
                }
            },
            flag if flag.starts_with('-') => return fail(&format!("unknown option: {flag}")),
            path if file.is_none() => file = Some(path.to_string()),
            path => return fail(&format!("unexpected extra file: {path}")),
        }
    }
    let Some(file) = file else {
        return fail("which file should I run? e.g. nme run hello.nme");
    };

    let (path, python_source) = match transpile_file(&file) {
        Ok(ok) => ok,
        Err(code) => return code,
    };
    match exec::run_python(&python_source, &path, &python) {
        Ok(status) => exit_code(status),
        Err(err) => fail(&format!(
            "couldn't start Python ({python}): {err}\n\
             hint: make sure Python is installed, or pass --python <command>"
        )),
    }
}

/// `nme build`: transpile and print (or write) the Python program.
fn command_build(args: &[String]) -> ExitCode {
    let mut output = None;
    let mut file = None;
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "-o" | "--output" => match rest.next() {
                Some(path) => output = Some(path.clone()),
                None => return fail("-o needs a path, e.g. -o hello.py"),
            },
            flag if flag.starts_with('-') => return fail(&format!("unknown option: {flag}")),
            path if file.is_none() => file = Some(path.to_string()),
            path => return fail(&format!("unexpected extra file: {path}")),
        }
    }
    let Some(file) = file else {
        return fail("which file should I build? e.g. nme build hello.nme");
    };

    let (_, python_source) = match transpile_file(&file) {
        Ok(ok) => ok,
        Err(code) => return code,
    };
    if let Some(path) = output {
        match std::fs::write(&path, &python_source) {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => fail(&format!("couldn't write {path}: {err}")),
        }
    } else {
        print!("{python_source}");
        ExitCode::SUCCESS
    }
}

/// `nme check`: transpile, then ask CPython to compile without executing.
fn command_check(args: &[String]) -> ExitCode {
    let mut python = DEFAULT_PYTHON.to_string();
    let mut file = None;
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--python" => match rest.next() {
                Some(command) => python.clone_from(command),
                None => {
                    return fail(&format!(
                        "--python needs a command, e.g. --python {DEFAULT_PYTHON}"
                    ));
                }
            },
            flag if flag.starts_with('-') => return fail(&format!("unknown option: {flag}")),
            path if file.is_none() => file = Some(path.to_string()),
            path => return fail(&format!("unexpected extra file: {path}")),
        }
    }
    let Some(file) = file else {
        return fail("which file should I check? e.g. nme check hello.nme");
    };
    let (path, python_source) = match transpile_file(&file) {
        Ok(ok) => ok,
        Err(code) => return code,
    };
    match exec::check_python(&python_source, &path, &python) {
        Ok(status) => exit_code(status),
        Err(error) => fail(&format!(
            "couldn't start Python ({python}): {error}\n\
             hint: make sure Python is installed, or pass --python <command>"
        )),
    }
}

fn is_nme_path(path: &str) -> bool {
    Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("nme"))
}

fn exit_code(status: std::process::ExitStatus) -> ExitCode {
    ExitCode::from(u8::try_from(status.code().unwrap_or(1)).unwrap_or(1))
}

/// Reads and transpiles one `.nme` file, reporting all problems nicely.
fn transpile_file(file: &str) -> Result<(std::path::PathBuf, String), ExitCode> {
    let path = Path::new(file);
    let source = match std::fs::read_to_string(path) {
        Ok(source) => source,
        Err(err) => {
            return Err(fail(&format!("couldn't read {file}: {err}")));
        }
    };
    match nme_core::transpile(&source) {
        Ok(python) => Ok((path.to_path_buf(), python)),
        Err(problems) => {
            eprint!(
                "{}",
                nme_core::diagnostics::render_all(&problems, &source, file)
            );
            Err(ExitCode::FAILURE)
        }
    }
}

fn fail(message: &str) -> ExitCode {
    eprintln!("error: {message}");
    ExitCode::FAILURE
}
