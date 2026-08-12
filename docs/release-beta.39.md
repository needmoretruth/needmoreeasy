# NME 0.0.1-beta.39

Beta.39 makes native artifacts follow NME’s no-accidental-overwrite rule.

- Refuse `nme native build -o` when the requested executable already exists,
  using E9009.
- Refuse the build when the companion generated C source already exists, also
  using E9009, and leave both existing files unchanged.
- Reject a `.c` output path with E9003 because it would collide with the
  generated C source path.
- Extend the bilingual E9009 lookup explanation and regression coverage.

Language semantics and generated Python behavior are unchanged.
