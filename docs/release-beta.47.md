# NME 0.0.1-beta.47

Beta.47 fixes native declarations inside control blocks.

- Hoist new scalar and string declarations to the active function scope.
- Keep assignments inside their original `if`/`while` control flow so later
  expressions can use the binding without out-of-scope C.
