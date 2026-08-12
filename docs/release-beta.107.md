# NME 0.0.1-beta.107

Beta107 completes the native branch-merge fix for conditional bindings.

- A previously maybe-initialized name becomes available after a later
  `if`/`else` only when every arm assigns it concretely.
- Preserve conservative behavior for one-sided branches and loops that may not
  run, with English and Korean regression coverage.
