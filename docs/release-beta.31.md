# NME 0.0.1-beta.31

Beta.31 gives native executable startup failures their own stable diagnostic.

- Add E9026, `the native program could not be started`, with Korean and English
  explanations available through `nme ko E9026` and `nme en E9026`.
- Keep E9013 reserved for failures to start the Python command.
- Add a deterministic Unix regression using a compiler stub that omits the
  executable, so the operating-system launch path is tested without a broken
  host compiler.

Language semantics and generated Python behavior are unchanged.
