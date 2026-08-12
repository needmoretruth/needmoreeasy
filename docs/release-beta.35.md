# NME 0.0.1-beta.35

Beta.35 uses the file-read diagnostic for imported module read failures.

- Report an unreadable imported `.nme` module with E9007, `a file could not be
  read`, instead of E9015, which is reserved for a missing top-level program.
- Extend the bilingual E9007 explanation and add English/Korean CLI regression
  coverage for a missing imported module.

Language semantics and generated Python behavior are unchanged.
