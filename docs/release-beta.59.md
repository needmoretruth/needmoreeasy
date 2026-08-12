# NME 0.0.1-beta.59

Beta.59 fixes native definite-binding analysis for unreachable branches.

- Bindings introduced only in an unreachable `else` or `else if` after
  `if true` no longer escape the block as initialized names.
- Such branches are still checked for native type compatibility, but later
  reads correctly receive the uninitialized-name diagnostic.
