# NME 0.0.1-beta.118

Beta118 strengthens regression coverage for parenthesized Korean comparison
endings in `elif` conditions.

- Verify exact core lowering and native execution for a branch such as
  `아니면 만약에 (점수가 4보다 작으면)`.
- Keep `elif` on the same shared condition-span path as `if` and `while`, with
  no separate language implementation.
