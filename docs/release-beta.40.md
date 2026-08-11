# NME 0.0.1-beta.40

Beta.40 prevents stale temporary staging files from affecting later runs.

- Give imported-module, native, and Nuitka staging fresh per-invocation
  directories with collision checks.
- Avoid reusing process-ID-only directories after a crash or process-ID reuse.
- Add a deterministic Unix regression proving a stale Python module cannot
  shadow an ordinary import in a later invocation.

Language semantics and generated Python behavior are unchanged.
