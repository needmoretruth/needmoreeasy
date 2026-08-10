//! Executes transpiled Python with the real Python runtime.
//!
//! NME never interprets code itself. The transpiled program is written to a
//! temporary file and run with the system's Python interpreter, so every
//! Python runtime feature and every installed library works exactly as it
//! does for native Python programs.
//!
//! Because `nme-core` lowering is line-preserving, the line numbers in
//! Python tracebacks match the lines of the original `.nme` file. (Mapping
//! the displayed *file name* back to the `.nme` path is future work; see
//! `docs/architecture.md`.)

use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

/// Runs `python_source` with the given Python command (e.g. `python3`).
///
/// The program is written to a temporary file named after the original
/// `.nme` file (`<stem>.py`) so tracebacks show a recognizable name, then
/// executed with inherited stdio. The temporary directory is removed
/// afterwards on a best-effort basis.
pub fn run_python(python_source: &str, stem: &str, python: &str) -> io::Result<ExitStatus> {
    let dir = std::env::temp_dir().join(format!("nme-run-{}", std::process::id()));
    std::fs::create_dir_all(&dir)?;
    let program = dir.join(format!("{stem}.py"));
    std::fs::write(&program, python_source)?;

    let status = Command::new(python).arg(&program).status();

    let _ = std::fs::remove_file(&program);
    let _ = std::fs::remove_dir(&dir);
    status
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
