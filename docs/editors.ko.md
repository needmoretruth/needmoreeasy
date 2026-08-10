# VS Code, Cursor, Zed에서 NME 사용하기

[English](editors.md) | 한국어

저장소에는 세 편집기에서 쓸 실행, 검사, 빌드 작업이 들어 있습니다. NME를 먼저
설치하고 파일 하나가 아니라 저장소 폴더를 여세요.

## VS Code

추적되는 `.vscode/settings.json`은 기본 색상을 위해 `*.nme`를 Python 파일로
연결합니다. `.vscode/tasks.json`에는 다음 작업이 있습니다.

- `NME: run current file`
- `NME: check current file`
- `NME: build current file`

명령 팔레트에서 **Tasks: Run Task**를 고른 뒤 작업을 선택하세요. VS Code의
공식 [워크스페이스 문서](https://code.visualstudio.com/docs/editing/workspaces/workspaces)와
[설정 안내](https://code.visualstudio.com/docs/configure/settings)는 폴더 작업과
`.vscode` 설정 방식을 설명합니다.

편집기는 NME 파일을 Python으로 보고 NME는 추가 문법을 제공하므로 Python
확장이 올바른 초급·문장형 줄에 빨간 밑줄을 표시할 수 있습니다. `nme 검사`의
결과가 기준입니다. 이번 베타에는 NME 언어 서버가 아직 없습니다.

## Cursor

같은 폴더를 Cursor로 여세요. 포함된 VS Code 호환 설정과 작업을 쓰거나 통합
터미널에서 `nme 실행`, `nme 검사`, `nme 빌드`를 실행합니다.

Cursor Agent에는 [AI 코딩 도우미](ai-assistants.ko.md)의 전달 문장을 붙여
넣으세요. Cursor는 `@Link`로 웹 링크를 바로 문맥에 넣을 수 있으며 공식
[문맥 안내](https://docs.cursor.com/context/%40-symbols/overview)에 설명되어
있습니다.

Cursor 프로젝트 규칙은 공식 [Cursor Rules 안내](https://docs.cursor.com/context/rules)에
따라 보통 `.cursor/rules`에 둡니다. NME는 도구별 메타데이터를 프로젝트에
넣지 않도록 규칙 파일을 추적하지 않으며, 공용 전달 링크만으로 충분합니다.

## Zed

추적되는 `.zed/settings.json`은 `*.nme`를 Python으로 연결하고,
`.zed/tasks.json`은 같은 세 작업을 정의합니다. macOS는 `Cmd+Shift+R`,
Linux/Windows는 `Ctrl+Shift+R`로 작업 선택기를 열고 NME 작업을 고릅니다.

Zed 공식 [언어 설정](https://zed.dev/docs/configuring-languages)은 파일 확장자
연결을, [Tasks 안내](https://zed.dev/docs/tasks)는 프로젝트의
`.zed/tasks.json`을 설명합니다.

VS Code와 마찬가지로 Python 진단은 문장형을 모르므로 전용 NME 확장이 생기기
전까지 `nme 검사`를 기준으로 삼으세요.

## 어떤 편집기에서든 터미널 사용하기

편집기가 제공된 작업을 불러오지 못해도 다음 명령은 같습니다.

```sh
nme 실행 path/to/program.nme
nme 검사 path/to/program.nme
nme 빌드 path/to/program.nme -o program.py
nme 컴파일 path/to/program.nme -o program
```

Windows에서 Python 명령이 `python3`가 아니면 `실행` 또는 `컴파일`에
`--python py`를 추가하세요.
