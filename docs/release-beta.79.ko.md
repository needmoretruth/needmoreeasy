# NME 0.0.1-beta.79

beta.79에서는 네이티브 CLI가 운영체제의 일반적인 C 컴파일러 인터페이스를
사용합니다.

- macOS와 Linux에서는 기존 Unix 방식 옵션과 함께 `cc`를 사용합니다.
- Windows에서는 Visual Studio Developer PowerShell에서 MSVC `cl`을 `/O2`와
  `/Fe:` 옵션으로 사용합니다.
- 네이티브 가이드와 설치 안내에 Windows에서 필요한 컴파일러 셸을
  설명합니다.
