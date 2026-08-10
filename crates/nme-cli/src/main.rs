//! `nme` — the NeedMoreEasy command line toolchain.
//!
//! This crate is a thin driver around `nme_core`: it handles all IO
//! (reading files, writing files, running Python) so the compiler core can
//! stay pure and trivially testable.
//!
//! ```text
//! nme run   hello.nme              transpile and execute with Python
//! nme build hello.nme [-o out.py]  transpile and write/print the Python
//! nme check hello.nme              transpile only; report problems
//! ```

mod exec;

use std::path::Path;
use std::process::ExitCode;

const HELP: &str = r"nme — NeedMoreEasy: programming, easier than Python.

USAGE:
    nme run <file.nme> [--python <command>]   Run an NME program with Python
    nme build <file.nme> [-o <output.py>]     Transpile to Python and print it
    nme check <file.nme>                      Check for problems without running
    nme --help                                Show this help
    nme --version                             Show the version

Every valid Python program is already a valid NME program. NME adds a small
English/Korean starter vocabulary for output, input, repetition, conditions,
and Python's bundled random tools. See README.md or README.ko.md to begin.
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("run") => command_run(&args[1..]),
        Some("build") => command_build(&args[1..]),
        Some("check") => command_check(&args[1..]),
        Some("--version" | "-V") => {
            println!("nme {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Some("--help" | "-h") => {
            print!("{HELP}");
            ExitCode::SUCCESS
        }
        _ => {
            eprint!("{HELP}");
            ExitCode::FAILURE
        }
    }
}

/// `nme run`: transpile, then execute with the real Python runtime.
fn command_run(args: &[String]) -> ExitCode {
    let mut python = "python3".to_string();
    let mut file = None;
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--python" => match rest.next() {
                Some(command) => python.clone_from(command),
                None => return fail("--python needs a command, e.g. --python python3"),
            },
            flag if flag.starts_with('-') => return fail(&format!("unknown option: {flag}")),
            path => file = Some(path.to_string()),
        }
    }
    let Some(file) = file else {
        return fail("which file should I run? e.g. nme run hello.nme");
    };

    let (path, python_source) = match transpile_file(&file) {
        Ok(ok) => ok,
        Err(code) => return code,
    };
    let stem = path
        .file_stem()
        .map_or("program", |s| s.to_str().unwrap_or("program"));
    match exec::run_python(&python_source, stem, &python) {
        Ok(status) => ExitCode::from(u8::try_from(status.code().unwrap_or(1)).unwrap_or(1)),
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
            path => file = Some(path.to_string()),
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

/// `nme check`: transpile only; report problems.
fn command_check(args: &[String]) -> ExitCode {
    match args.first() {
        Some(file) if !file.starts_with('-') => match transpile_file(file) {
            Ok(_) => ExitCode::SUCCESS,
            Err(code) => code,
        },
        _ => fail("which file should I check? e.g. nme check hello.nme"),
    }
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
