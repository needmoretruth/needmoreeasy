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
