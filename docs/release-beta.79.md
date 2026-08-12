# NME 0.0.1-beta.79

Beta.79 makes the native CLI use the platform's normal C compiler interface.

- macOS and Linux use `cc` with the existing Unix-style flags.
- Windows uses MSVC `cl` with `/O2` and `/Fe:` from a Developer PowerShell for
  Visual Studio.
- The native guide and installation instructions now explain the compiler
  shell required on Windows.
