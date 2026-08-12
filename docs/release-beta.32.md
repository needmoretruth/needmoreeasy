# NME 0.0.1-beta.32

Beta.32 separates temporary working-folder failures from current-folder reads.

- Add E9027, `a temporary working folder could not be created`, with Korean and
  English explanations available through `nme ko E9027` and `nme en E9027`.
- Preserve the selected command language while staging imported modules, so
  Korean execution failures remain Korean-first and bilingual.
- Add a deterministic Unix regression by blocking the temporary-directory path
  with a file.

Language semantics and generated Python behavior are unchanged.
