# NME 0.0.1-beta.51

Beta.51 makes native function fallthrough explicit.

- Require every native function to have an unconditional top-level integer
  return.
- Reject functions that only return from a possibly skipped branch instead of
  allowing undefined C return values.
