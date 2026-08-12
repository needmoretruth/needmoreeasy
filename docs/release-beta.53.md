# NME 0.0.1-beta.53

Beta.53 tightens native function declarations and calls.

- Reject duplicate function definitions and unsupported default or varargs
  headers before C generation.
- Reject keyword arguments so native lowering never drops call arguments.
