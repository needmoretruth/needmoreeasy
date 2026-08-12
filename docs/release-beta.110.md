# NME 0.0.1-beta.110

Beta110 gives the native backend real boolean bindings.

- `True`/`False` and sentence `true`/`false`/`참`/`거짓` literals can be
  assigned to names as a static type distinct from integers.
- Boolean names support truthy `if`/`while` conditions, equality/inequality,
  and `show` output as `True`/`False`.
- Boolean arithmetic, `add`/`subtract` updates, and integer-only native
  function arguments or returns are rejected clearly, even though the C
  representation uses `int`.
- English and Korean sentence, beginner, and advanced surfaces share the same
  tests and synchronized native documentation.
