# Install NME on Windows, macOS, and Linux

English | [한국어](install.ko.md)

NME `0.0.1-beta.14` currently installs from the beta branch. You need Git,
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
$env:Path = "$HOME\.cargo\bin;$env:Path"
nme --version
```

The `$env:Path` line makes the just-installed command available in the current
PowerShell session. If a new PowerShell still cannot find `nme`, add
`%USERPROFILE%\.cargo\bin` to your user `PATH`. You do not need to reinstall;
`& "$HOME\.cargo\bin\nme.exe" --version` verifies the installed binary directly.

Run the example. NME chooses the Windows `py` launcher automatically:

```powershell
nme run examples\hello-sentence
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
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"
nme --version
nme run examples/hello-sentence
```

The `export` line must run before the first `nme` command when Cargo warns that
its binary directory is not on `PATH`. To keep it for future terminals, add
the same line to `~/.zshrc` (the default macOS shell) or your shell's profile.
If Rust was installed with rustup, this equivalent command may already exist:

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
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"
nme --version
nme run examples/hello-sentence
```

The `export` line is deliberately before `nme --version`. Fedora's system
`cargo`, and some other package-manager installations, can install the binary
successfully to `~/.cargo/bin` without adding that directory to the shell's
`PATH`. If needed, verify the install directly with
`"${CARGO_HOME:-$HOME/.cargo}/bin/nme" --version`, then add the export line to
`~/.bashrc`, `~/.zshrc`, or the profile used by your shell.

## Verify the complete toolchain

```sh
nme --version
nme modules
nme check examples/three-levels
nme run examples/hello-sentence
```

Expected NME version: `0.0.1-beta.14`. Expected random adapter: `0.0.1`.

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
