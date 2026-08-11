//! Checks and executes transpiled Python with the real Python runtime.
//!
//! NME never interprets code itself. The transpiled program is checked or run
//! by the selected system Python interpreter, so Python runtime features and
//! installed libraries work as they do for native Python programs.
//!
//! Because `nme-core` lowering is line-preserving, the line numbers in
//! Python tracebacks match the lines of the original `.nme` file. The run
//! bootstrap also compiles with the original path and restores normal script
//! values for `__file__`, `sys.argv[0]`, and `sys.path[0]`.

use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Output, Stdio};

const CHECK_BOOTSTRAP: &str = r#"import os, sys
path = os.path.abspath(sys.argv[1])
source = sys.stdin.buffer.read().decode("utf-8")
try:
    compile(source, path, "exec", dont_inherit=True)
except SyntaxError as error:
    error = error.with_traceback(None)
    sys.excepthook(type(error), error, None)
    raise SystemExit(1) from None
"#;

const RUN_BOOTSTRAP: &str = r#"def _nme_main():
    import os, sys
    path = os.path.abspath(sys.argv[1])
    size = int(sys.stdin.buffer.readline())
    source = sys.stdin.buffer.read(size).decode("utf-8")
    sys.argv[:] = [path, *sys.argv[2:]]
    source_dir = os.path.dirname(path)
    if sys.path:
        sys.path[0] = source_dir
    else:
        sys.path.insert(0, source_dir)
    module_path = os.environ.get("NME_MODULE_PATH")
    if module_path:
        for entry in reversed(module_path.split(os.pathsep)):
            sys.path.insert(0, entry)
    scope = sys.modules["__main__"].__dict__
    scope.update({
        "__name__": "__main__",
        "__file__": path,
        "__package__": None,
        "__cached__": None,
        "__loader__": None,
        "__spec__": None,
    })
    scope.pop("_nme_main", None)
    try:
        code = compile(source, path, "exec", dont_inherit=True)
        exec(code, scope, scope)
    except Exception as error:
        traceback = error.__traceback__
        error = error.with_traceback(traceback.tb_next if traceback else None)
        sys.excepthook(type(error), error, error.__traceback__)
        raise SystemExit(1) from None
_nme_main()
"#;

/// Asks CPython to compile generated source without executing it.
///
/// Feeding the UTF-8 source on standard input avoids command-line length and
/// quoting limits. CPython receives the original NME path as the compile
/// filename, so its syntax diagnostics point to the user's file.
pub fn check_python(python_source: &str, source_path: &Path, python: &str) -> io::Result<Output> {
    let source_path = absolute_path(source_path)?;
    let mut child = Command::new(python)
        .arg("-c")
        .arg(CHECK_BOOTSTRAP)
        .arg(source_path)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut child_stdin = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("Python standard input was not piped"))?;
    if let Err(error) = child_stdin.write_all(python_source.as_bytes()) {
        stop_after_input_error(&mut child);
        return Err(error);
    }
    drop(child_stdin);
    child.wait_with_output()
}

/// Runs `python_source` with the given Python command (e.g. `python3`).
///
/// A small static bootstrap receives the UTF-8 program on its standard input,
/// compiles it with the original `.nme` path, and executes it as a normal main
/// script. After the framed source has been sent, a detached forwarding thread
/// connects the user's standard input to the program so `input()` remains
/// interactive without creating a temporary Python file.
pub fn run_python(
    python_source: &str,
    source_path: &Path,
    python: &str,
    module_dir: Option<&Path>,
) -> io::Result<ExitStatus> {
    let source_path = absolute_path(source_path)?;
    let mut command = Command::new(python);
    if let Some(dir) = module_dir {
        command.env("NME_MODULE_PATH", dir);
    }
    let mut child = command
        .arg("-c")
        .arg(RUN_BOOTSTRAP)
        .arg(source_path)
        .stdin(Stdio::piped())
        .spawn()?;
    let mut child_stdin = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("Python standard input was not piped"))?;

    let source_bytes = python_source.as_bytes();
    let write_result = writeln!(child_stdin, "{}", source_bytes.len())
        .and_then(|()| child_stdin.write_all(source_bytes))
        .and_then(|()| child_stdin.flush());
    if let Err(error) = write_result {
        stop_after_input_error(&mut child);
        return Err(error);
    }

    std::thread::spawn(move || {
        let input = io::stdin();
        let mut input = input.lock();
        let _ = io::copy(&mut input, &mut child_stdin);
    });
    child.wait()
}

fn stop_after_input_error(child: &mut std::process::Child) {
    let _ = child.kill();
    let _ = child.wait();
}

/// Compiles transpiled Python to a native executable through Nuitka.
///
/// Nuitka and a platform C compiler are intentionally external tools: the NME
/// binary stays small, while users who want a standalone artifact can opt in.
pub fn compile_native(
    python_source: &str,
    stem: &str,
    python: &str,
    output: &Path,
) -> io::Result<ExitStatus> {
    let output = absolute_path(output)?;
    let output_dir = output.parent().unwrap_or_else(|| Path::new("."));
    if !output_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("output directory does not exist: {}", output_dir.display()),
        ));
    }
    let output_name = output
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "output needs a file name"))?;

    let dir = std::env::temp_dir().join(format!("nme-compile-{}", std::process::id()));
    std::fs::create_dir_all(&dir)?;
    let program = dir.join(format!("{stem}.py"));
    std::fs::write(&program, python_source)?;

    let status = Command::new(python)
        .arg("-m")
        .arg("nuitka")
        .arg("--onefile")
        .arg("--remove-output")
        .arg(format!(
            "--output-filename={}",
            output_name.to_string_lossy()
        ))
        .arg(format!("--output-dir={}", output_dir.display()))
        .arg(&program)
        .status();

    let _ = std::fs::remove_file(&program);
    let _ = std::fs::remove_dir(&dir);
    status
}

fn absolute_path(path: &Path) -> io::Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}
