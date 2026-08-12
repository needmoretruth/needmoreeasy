# NME 0.0.1-beta.86

beta.86에서는 Windows 출시 방식의 CLI 스모크 테스트를 수정합니다.

- Windows의 `cargo install`과 CLI 스모크 명령을 PowerShell에서 실행하여
  MSVC 개발 환경이 올바른 링커를 찾게 합니다.
- Unix 스모크 테스트 작업은 Bash를 유지합니다.
- 언어, 컴파일러, 네이티브 백엔드 동작은 바뀌지 않습니다.

사용자에게 보이는 NME 의미는 바꾸지 않고 CI 검증만 고칩니다.
