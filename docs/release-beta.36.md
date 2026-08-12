# NME 0.0.1-beta.36

Beta.36 separates `nme compile` staging failures from compiler startup failures.

- Report a blocked temporary working folder with E9027, `a temporary working
  folder could not be created`, rather than E9011.
- Report a failure writing the temporary Python source with E9008, while E9011
  remains the diagnostic for not starting Python/Nuitka.
- Add a deterministic bilingual Unix regression using a file as `TMPDIR`.

Language semantics and generated Python behavior are unchanged.
