# Install NME on Windows, macOS, and Linux

English | [한국어](install.ko.md)

NME `0.0.1-beta.2` currently installs from the beta branch. You need Git,
stable Rust with Cargo, and Python 3.8 or newer. Use the official
[Rust installer](https://www.rust-lang.org/tools/install),
[Python downloads](https://www.python.org/downloads/), and
[Git downloads](https://git-scm.com/downloads).

## Windows 10 or 11

1. Install Git for Windows.
2. Install Python from python.org. Keep the Python launcher (`py`) enabled.
3. Download and run `rustup-init.exe` from the official Rust install page.
   Accept the stable MSVC toolchain. If asked, install Visual Studio C++ Build
   Tools.
4. Close and reopen PowerShell, then run:

```powershell
git clone --branch beta https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
nme --version
```

If `nme` is not found, reopen the terminal and confirm that
`%USERPROFILE%\.cargo\bin` is on `PATH`.

Run with the Windows Python launcher:

```powershell
nme run examples\hello-sentence.nme --python py
```

## macOS

Install Git through Xcode Command Line Tools when needed:

```sh
xcode-select --install
```

Install a current Python from python.org, then install Rust with the command
shown on the official rustup page. Open a new Terminal and run:

```sh
git clone --branch beta https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
nme --version
nme run examples/hello-sentence.nme
```

If the shell cannot find `nme`, load Cargo's environment once and reopen the
terminal:

```sh
source "$HOME/.cargo/env"
```

## Linux

Install Python, Git, a C build toolchain, and `curl` with your distribution's
package manager. Examples:

```sh
# Debian / Ubuntu
sudo apt install python3 git curl build-essential

# Fedora
sudo dnf install python3 git curl gcc

# Arch Linux
sudo pacman -S python git curl base-devel
```

Install stable Rust with rustup, open a new shell, then run:

```sh
git clone --branch beta https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
nme --version
nme run examples/hello-sentence.nme
```

## Verify the complete toolchain

```sh
nme --version
nme modules
nme check examples/three-levels.nme
nme run examples/hello-sentence.nme
```

Expected NME version: `0.0.1-beta.2`. Expected random adapter: `0.0.1`.

## Choose a different Python command

NME uses `python3` by default. If your command is `python` or `py`, use:

```sh
nme run program.nme --python python
nme compile program.nme -o program --python python
```

On Windows, replace the final command with `py`.

## Optional native executable

NME's normal `build` creates Python source and preserves complete Python
compatibility. `compile` uses the external Nuitka compiler to create a
standalone one-file artifact:

```sh
python3 -m pip install -U "Nuitka[app]"
python3 -m nuitka --version
nme compile examples/hello-sentence.nme -o hello
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
