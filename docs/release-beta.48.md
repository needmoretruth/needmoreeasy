# NME 0.0.1-beta.48

Beta.48 makes native binding types and value changes explicit.

- Reject assignments that change a native name between integer, float, and
  string types before generating incompatible C.
- Require `add`/`subtract` targets to have an earlier numeric assignment and
  explain invalid targets bilingually.
