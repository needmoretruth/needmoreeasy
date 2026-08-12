# NME 0.0.1-beta.121

Beta121 fixes Korean `while` comparison endings before logical connectors
inside a parenthesized condition.

- Support `동안 (횟수가 2보다 작을 동안 그리고 준비)` without treating the
  inner `동안` as the loop-body boundary.
- Keep the same shared token and condition path for `그리고`/`또는` and
  `and`/`or`, including the native backend.
