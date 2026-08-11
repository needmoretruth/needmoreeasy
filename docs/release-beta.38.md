# NME 0.0.1-beta.38

Beta.38 aligns native-command folder handling with the other CLI commands.

- Report `nme native <folder>` with E9014, `a folder is not a program`, rather
  than E9007, `a file could not be read`.
- Keep the English-only and Korean-first bilingual paths on the same stable
  diagnostic and actionable folder guidance.
- Add regression coverage for both native command languages.

Language semantics and generated Python behavior are unchanged.
