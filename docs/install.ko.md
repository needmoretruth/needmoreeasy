# Windows, macOS, Linux에 NME 설치하기

[English](install.md) | 한국어

현재 NME `0.0.1-beta.2`는 beta 브랜치의 소스에서 설치합니다. Git, Cargo가
포함된 안정 Rust, Python 3.8 이상이 필요합니다. 공식
[Rust 설치](https://www.rust-lang.org/tools/install),
[Python 다운로드](https://www.python.org/downloads/),
[Git 다운로드](https://git-scm.com/downloads)를 이용하세요.

## Windows 10 또는 11

1. Git for Windows를 설치합니다.
2. python.org에서 Python을 설치하고 Python 실행기 `py`를 유지합니다.
3. 공식 Rust 설치 페이지에서 `rustup-init.exe`를 받아 실행합니다. 안정 MSVC
   도구 체인을 고릅니다. 요청이 나오면 Visual Studio C++ Build Tools도
   설치합니다.
4. PowerShell을 닫았다 다시 열고 실행합니다.

```powershell
git clone --branch beta https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
$env:Path = "$HOME\.cargo\bin;$env:Path"
nme --version
```

`$env:Path` 줄은 방금 설치한 명령을 현재 PowerShell에서 바로 찾게 합니다. 새
PowerShell에서도 `nme`를 찾지 못하면 `%USERPROFILE%\.cargo\bin`을 사용자
`PATH`에 추가하세요. 다시 설치할 필요는 없으며
`& "$HOME\.cargo\bin\nme.exe" --version`으로 설치된 파일을 직접 확인할 수
있습니다.

예제를 실행합니다. NME가 Windows의 `py` 실행기를 자동으로 고릅니다.

```powershell
nme 실행 examples\hello-sentence
```

## macOS

Git이 없다면 Xcode 명령줄 도구를 설치합니다.

```sh
xcode-select --install
```

python.org에서 현재 Python을 설치하고 공식 rustup 페이지의 명령으로 Rust를
설치합니다. 새 터미널을 열고 실행하세요.

```sh
git clone --branch beta https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"
nme --version
nme 실행 examples/hello-sentence
```

Cargo가 바이너리 폴더가 `PATH`에 없다고 경고하면 첫 `nme` 명령보다 `export`
줄을 먼저 실행해야 합니다. 다음 터미널에도 유지하려면 같은 줄을 macOS 기본
셸의 `~/.zshrc` 또는 사용 중인 셸의 프로필에 추가하세요. rustup으로 Rust를
설치했다면 다음 명령이 같은 설정을 이미 제공할 수 있습니다.

```sh
source "$HOME/.cargo/env"
```

## Linux

배포판의 패키지 관리자로 Python, Git, C 빌드 도구, `curl`을 설치합니다.

```sh
# Debian / Ubuntu
sudo apt install python3 git curl build-essential

# Fedora
sudo dnf install python3 git curl gcc

# Arch Linux
sudo pacman -S python git curl base-devel
```

rustup으로 안정 Rust를 설치하고 새 셸에서 실행합니다.

```sh
git clone --branch beta https://github.com/needmoretruth/needmoreeasy.git
cd needmoreeasy
cargo install --path crates/nme-cli --locked
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"
nme --version
nme 실행 examples/hello-sentence
```

`export` 줄을 의도적으로 첫 `nme` 명령보다 앞에 두었습니다. Fedora의 시스템
`cargo` 같은 패키지 관리자 버전은 설치에 성공해도 `~/.cargo/bin`을 셸의
`PATH`에 자동으로 넣지 않을 수 있습니다. 필요하면
`"${CARGO_HOME:-$HOME/.cargo}/bin/nme" --version`으로 설치 파일을 직접 확인한
뒤, 같은 export 줄을 `~/.bashrc`, `~/.zshrc` 또는 사용 중인 셸 프로필에
추가하세요.

## 전체 도구 확인

```sh
nme --version
nme 모듈
nme 검사 examples/three-levels
nme 실행 examples/hello-sentence
```

NME는 `0.0.1-beta.2`, 랜덤 어댑터는 `0.0.1`이 표시되어야 합니다.

## 고급: 다른 Python 명령 고르기

NME가 Windows에서는 `py`, 그 밖의 운영체제에서는 `python3`를 자동으로
사용합니다. 특수한 환경에서만 고급 옵션을 씁니다.

```sh
nme 실행 program --python python
nme 컴파일 program -o program --python python
```

Windows에서는 마지막 값을 `py`로 바꾸세요.

## 선택적인 독립 실행 파일

평범한 `빌드`는 완전한 Python 호환성을 유지하는 Python 소스를 만듭니다.
`컴파일`은 외부 Nuitka 컴파일러로 한 파일짜리 독립 실행 결과물을 만듭니다.

```sh
python3 -m pip install -U "Nuitka[app]"
python3 -m nuitka --version
nme 컴파일 examples/hello-sentence -o hello
```

Nuitka에는 운영체제용 C 컴파일러가 필요합니다. 복잡한 프로그램은 한 파일 모드
전에 standalone 모드로 검사할 것을 Nuitka가 권장합니다. NME 명령은 가장 쉬운
전달을 위해 한 파일 모드를 사용합니다. 데이터 파일이나 네이티브 패키지가 있으면
공식 [Nuitka 시작 안내](https://nuitka.net/user-documentation/tutorial-setup-and-build.html)와
[사용자 설명서](https://nuitka.net/user-documentation/user-manual.html)를 읽으세요.

Windows용 파일은 Windows에서, macOS용은 macOS에서, Linux용은 Linux에서
각각 빌드해야 합니다.

## 업데이트 또는 삭제

beta를 업데이트하고 다시 설치합니다.

```sh
git switch beta
git pull --ff-only
cargo install --path crates/nme-cli --locked --force
```

설치한 CLI를 지웁니다.

```sh
cargo uninstall nme-cli
```

실행 명령은 `nme`, Cargo 패키지 이름은 `nme-cli`입니다.
