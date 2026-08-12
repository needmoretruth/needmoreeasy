# NME 0.0.1-beta.62

Beta.62 hardens native C namespace checks.

- Reserve the macros, typedefs, and declarations exposed by the generated
  `limits.h`, `stdio.h`, `stdlib.h`, and `string.h` headers.
- Reject identifiers that could otherwise be rewritten by preprocessing or
  collide with a C library declaration.
