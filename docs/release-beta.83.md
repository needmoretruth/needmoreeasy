# NME 0.0.1-beta.83

Beta.83 makes the Windows native test path use the compiler environment that
the hosted runner provides.

- Configure the Windows CI job with Visual Studio's developer environment.
- Keep the native tests on the real MSVC path instead of silently skipping them.

Language semantics and generated Python behavior are unchanged.
