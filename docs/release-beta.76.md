# NME 0.0.1-beta.76

Beta.76 makes native function calls work independently of source order.

- Emit C prototypes before native definitions so forward calls and mutual
  recursion compile correctly.
- Use explicit `void` signatures for zero-argument native functions and cover
  both cases with native end-to-end regressions.
