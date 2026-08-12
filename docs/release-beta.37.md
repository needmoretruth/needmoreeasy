# NME 0.0.1-beta.37

Beta.37 keeps filesystem failures for existing program files separate from
missing-program diagnostics.

- Report an existing but unreadable `.nme` program with E9007, `a file could
  not be read`, for both `nme run` and `nme native`.
- Keep E9015, `the program file does not exist`, for paths that are actually
  missing.
- Add a deterministic bilingual Unix regression using a permission-restricted
  program file.

Language semantics and generated Python behavior are unchanged.
