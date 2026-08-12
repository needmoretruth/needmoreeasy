# NME 0.0.1-beta.52

Beta.52 validates native function calls before C generation.

- Require a call to name an integer function defined in the same file.
- Require the call’s argument count to match the function definition.
- Report bilingual diagnostics instead of deferring unknown or mismatched calls
  to the C compiler or linker.
