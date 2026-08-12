# NME 0.0.1-beta.58

Beta.58 hardens native name handling.

- Reject unresolved names and bare native function values before C emission.
- Reject duplicate function parameters and bindings or parameters that shadow
  a native function name.
