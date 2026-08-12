# NME 0.0.1-beta.122

Beta122 adds mixed-language coverage for Korean comparison endings before
English logical connectors inside parenthesized conditions.

- Verify shared lowering of `만약 (점수가 2보다 크면 and 준비)` and
  `만약 (점수가 2보다 작으면 or 준비)`.
- Verify the same `and`/`or` forms through the restricted native backend.
