# NME 0.0.1-beta.119

Beta119 fixes Korean comparison endings before logical connectors inside a
parenthesized condition.

- Support a form such as `만약 (점수가 2보다 크면 그리고 준비)` without treating
  `크면` as the end of the whole header.
- Scan a fully wrapped condition at its effective logical depth while keeping
  nested operand parentheses opaque.
- Cover the corrected lowering and native execution in English/Korean shared
  parser and backend paths.
