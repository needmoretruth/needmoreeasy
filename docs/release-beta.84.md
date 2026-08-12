# NME 0.0.1-beta.84

Beta.84 makes Windows native output match the source stem and keeps compiler
diagnostics separate from successful program output.

- Preserve `.c` and `.ko` stems in implicit Windows executable names such as
  `count.c.exe` and `count.ko.exe`.
- Suppress the MSVC success banner while retaining the native program's output.
- Align the native-backend documentation with the cross-platform artifact names.

Language semantics and generated Python behavior are unchanged.
