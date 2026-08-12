# Install NME on Windows, macOS, and Linux

English | [한국어](install.ko.md)

[Home](../README.md) | [Getting started](getting-started.md) | [Tutorial](tutorial.md) | [Language reference](language.md)

NME `0.0.1-beta.147` currently installs from the beta branch. You need Git,
stable Rust with Cargo, and Python 3.8 or newer. Use the official
[Rust installer](https://www.rust-lang.org/tools/install),
[Python downloads](https://www.python.org/downloads/), and
[Git downloads](https://git-scm.com/downloads). Pick your operating system
below; each section is complete on its own.

## Windows 11

1. Install Git for Windows.
2. Install Python from python.org. Keep the Python launcher (`py`) enabled.
3. Download and run `rustup-init.exe` from the official Rust install page.
   Accept the stable MSVC toolchain. If asked, install Visual Studio C++ Build
   Tools (free Microsoft components Cargo needs to build).
4. Close and reopen PowerShell, then run. To use `nme native`, open the
   Developer PowerShell for Visual Studio so `cl.exe` is on `PATH`; ordinary
   PowerShell is enough for the CPython commands:

```powershell
git clone --branch beta https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
$env:Path = "$HOME\.cargo\bin;$env:Path"
nme --version
```

Expected NME version: `0.0.1-beta.147`. The `$env:Path` line makes the
just-installed command available in the current PowerShell session.

**Set up PATH for future terminals.** If a new PowerShell still cannot find
`nme`, add `%USERPROFILE%\.cargo\bin` to your user `PATH`. You do not need to
reinstall; `& "$HOME\.cargo\bin\nme.exe" --version` verifies the installed
binary directly.

**First NME run** (from the `needmoreeasy` folder where you cloned the
repository). NME chooses the Windows `py` launcher automatically:

```powershell
nme run examples\hello-sentence
```

**Common errors.**
- `nme` is not recognized: `~\.cargo\bin` is not on your user `PATH`. Add
  `%USERPROFILE%\.cargo\bin` there, or confirm the binary exists with
  `& "$HOME\.cargo\bin\nme.exe" --version`.
- The Cargo build fails with a link or compiler error: the Visual Studio C++
  Build Tools from step 3 are missing. Run `rustup-init.exe` again and choose
  to install them.
- `nme run` cannot find Python: Python 3.8 or newer must be installed with the
  Python launcher (`py`) enabled (step 2).

## Windows 10

Windows 10 uses the same install as Windows 11. The complete steps are
repeated here so this section can be followed on its own:

1. Install Git for Windows.
2. Install Python from python.org. Keep the Python launcher (`py`) enabled.
3. Download and run `rustup-init.exe` from the official Rust install page.
   Accept the stable MSVC toolchain. If asked, install Visual Studio C++ Build
   Tools (free Microsoft components Cargo needs to build).
4. Close and reopen PowerShell, then run. To use `nme native`, open the
   Developer PowerShell for Visual Studio so `cl.exe` is on `PATH`; ordinary
   PowerShell is enough for the CPython commands:

```powershell
git clone --branch beta https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
$env:Path = "$HOME\.cargo\bin;$env:Path"
nme --version
```

Expected NME version: `0.0.1-beta.147`. The `$env:Path` line makes the
just-installed command available in the current PowerShell session.

**Set up PATH for future terminals.** If a new PowerShell still cannot find
`nme`, add `%USERPROFILE%\.cargo\bin` to your user `PATH`. You do not need to
reinstall; `& "$HOME\.cargo\bin\nme.exe" --version` verifies the installed
binary directly.

**First NME run** (from the `needmoreeasy` folder where you cloned the
repository). NME chooses the Windows `py` launcher automatically:

```powershell
nme run examples\hello-sentence
```

**Common errors.**
- `nme` is not recognized: `~\.cargo\bin` is not on your user `PATH`. Add
  `%USERPROFILE%\.cargo\bin` there, or confirm the binary exists with
  `& "$HOME\.cargo\bin\nme.exe" --version`.
- The Cargo build fails with a link or compiler error: the Visual Studio C++
  Build Tools from step 3 are missing. Run `rustup-init.exe` again and choose
  to install them.
- `nme run` cannot find Python: Python 3.8 or newer must be installed with the
  Python launcher (`py`) enabled (step 2).

## Older Windows (7 and 8)

The official path is Windows 10 or 11. Current versions of the rustup
installer and the Visual Studio C++ Build Tools require Windows 10 or newer,
so building the NME command-line tool from source is not supported on
Windows 7 or 8.

What still works: NME programs become plain Python, and Python 3.8 is the
newest Python that runs on Windows 7 and 8. A program built on a Windows 10/11
machine can be copied to a Windows 7/8 machine and run with that Python. The
NME command-line tool itself needs Windows 10 or 11.

## macOS

Install Git through Xcode Command Line Tools when needed:

```sh
xcode-select --install
```

Install a current Python from python.org, then install Rust with the command
shown on the official rustup page. The same steps work on both Intel and
Apple Silicon (M-series) Macs; the python.org installer covers both. Open a
new Terminal and run (from the `needmoreeasy` folder where you cloned the
repository):

```sh
git clone --branch beta https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"
nme --version
```

Expected NME version: `0.0.1-beta.147`. The `export` line must run before the
first `nme` command when Cargo warns that its binary directory is not on
`PATH`.

**Set up PATH for future terminals.** To keep the `export` line for future
terminals, add it to `~/.zshrc` (the default macOS shell) or your shell's
profile. If Rust was installed with rustup, this equivalent command may
already exist:

```sh
source "$HOME/.cargo/env"
```

**First NME run:**

```sh
nme run examples/hello-sentence
```

**Common errors.**
- `nme` is not found in a new Terminal: the `export` line is not saved yet.
  Add it to `~/.zshrc` or your shell's profile.
- The Cargo build fails with a compiler error: Xcode Command Line Tools are
  missing. Run `xcode-select --install` and try again.

## Debian and Ubuntu

Install Python, Git, a C build toolchain, and `curl` with apt, then install
stable Rust with rustup (the command shown on the official rustup page):

```sh
sudo apt install python3 python3-pip git curl build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

`python3-pip` is needed only for the optional Nuitka step below. Open a new
shell, then run (from the `needmoreeasy` folder where you cloned the
repository):

```sh
git clone --branch beta https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"
nme --version
```

Expected NME version: `0.0.1-beta.147`. The `export` line is deliberately
before `nme --version`.

**First NME run:**

```sh
nme run examples/hello-sentence
```

**Set up PATH for future terminals.** If `nme` is not found in a new
terminal, add the export line to `~/.bashrc`, `~/.zshrc`, or the profile used
by your shell. Package-manager Cargo installations can install the binary
successfully to `~/.cargo/bin` without adding that directory to the shell's
`PATH`; verify the install directly with
`"${CARGO_HOME:-$HOME/.cargo}/bin/nme" --version`.

**Common errors.**
- `nme` is not found: add the export line to `~/.bashrc` (or your shell's
  profile) and open a new terminal.
- `cargo` or `rustup` is not found right after the rustup command: the
  current shell was opened before the install. Open a new shell or run
  `source "$HOME/.cargo/env"`.
- The optional Nuitka step reports "No module named pip": install
  `python3-pip` (it is included in the install command above).

## Fedora

Install Python, Git, a C build toolchain, and `curl` with dnf, then install
stable Rust with rustup (the command shown on the official rustup page):

```sh
sudo dnf install python3 python3-pip git curl gcc
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

`python3-pip` is needed only for the optional Nuitka step below. Open a new
shell, then run (from the `needmoreeasy` folder where you cloned the
repository):

```sh
git clone --branch beta https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"
nme --version
```

Expected NME version: `0.0.1-beta.147`. The `export` line is deliberately
before `nme --version`.

**First NME run:**

```sh
nme run examples/hello-sentence
```

**Set up PATH for future terminals.** Fedora's system `cargo`, and some other
package-manager installations, can install the binary successfully to
`~/.cargo/bin` without adding that directory to the shell's `PATH`. If needed,
verify the install directly with
`"${CARGO_HOME:-$HOME/.cargo}/bin/nme" --version`, then add the export line to
`~/.bashrc`, `~/.zshrc`, or the profile used by your shell.

**Common errors.**
- `nme` is not found: add the export line to `~/.bashrc` (or your shell's
  profile) and open a new terminal.
- `cargo` or `rustup` is not found right after the rustup command: the
  current shell was opened before the install. Open a new shell or run
  `source "$HOME/.cargo/env"`.
- The optional Nuitka step reports "No module named pip": install
  `python3-pip` (it is included in the install command above).

## Arch Linux

Install Python, Git, a C build toolchain, and `curl` with pacman, then install
stable Rust with rustup (the command shown on the official rustup page):

```sh
sudo pacman -S python python-pip git curl base-devel
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

`python-pip` is needed only for the optional Nuitka step below. Open a new
shell, then run (from the `needmoreeasy` folder where you cloned the
repository):

```sh
git clone --branch beta https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"
nme --version
```

Expected NME version: `0.0.1-beta.147`. The `export` line is deliberately
before `nme --version`.

**First NME run:**

```sh
nme run examples/hello-sentence
```

**Set up PATH for future terminals.** If `nme` is not found in a new
terminal, add the export line to `~/.bashrc`, `~/.zshrc`, or the profile used
by your shell. Package-manager Cargo installations can install the binary
successfully to `~/.cargo/bin` without adding that directory to the shell's
`PATH`; verify the install directly with
`"${CARGO_HOME:-$HOME/.cargo}/bin/nme" --version`.

**Common errors.**
- `nme` is not found: add the export line to `~/.bashrc` (or your shell's
  profile) and open a new terminal.
- `cargo` or `rustup` is not found right after the rustup command: the
  current shell was opened before the install. Open a new shell or run
  `source "$HOME/.cargo/env"`.
- The optional Nuitka step reports "No module named pip": install `python-pip`
  (it is included in the install command above).

## Verify the complete toolchain

```sh
nme --version
nme modules
nme check examples/three-levels
nme run examples/hello-sentence
```

Expected NME version: `0.0.1-beta.147`. Expected random adapter: `0.0.1`.

## Advanced: choose a different Python command

NME automatically uses `py` on Windows and `python3` elsewhere. Only unusual
setups need the advanced override:

```sh
nme run program --python python
nme compile program -o program --python python
```

On Windows, replace the final command with `py`.

## Optional native executable

NME's normal `build` creates Python source and preserves complete Python
compatibility. `compile` uses the external Nuitka compiler to create a
standalone one-file artifact:

```sh
python3 -m pip install -U "Nuitka[app]"
python3 -m nuitka --version
nme compile examples/hello-sentence -o hello
```

Nuitka requires a platform C compiler and recommends testing standalone mode
before one-file distribution for complex programs. NME's command uses one-file
mode for the simplest handoff. Read the official
[Nuitka setup guide](https://nuitka.net/user-documentation/tutorial-setup-and-build.html)
and [user manual](https://nuitka.net/user-documentation/user-manual.html) when
your program includes data files or native packages.

Build on each target operating system; a Windows executable is not produced by
running the command on macOS or Linux.

## Update or uninstall

Update the beta checkout and reinstall:

```sh
git switch beta
git pull --ff-only
cargo install --path crates/nme-cli --locked --force
```

Remove the installed CLI:

```sh
cargo uninstall nme-cli
```

The binary command is `nme`, while the Cargo package name is `nme-cli`.
