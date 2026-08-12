# NME 0.0.1-beta.86

Beta.86 repairs the Windows release-style CLI smoke test.

- Run the Windows `cargo install` and CLI smoke commands under PowerShell,
  where the MSVC developer environment resolves the correct linker.
- Keep Bash for the Unix smoke-test jobs.
- Leave the language, compiler, and native-backend behavior unchanged.

This fixes CI validation without changing user-facing NME semantics.
