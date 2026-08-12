# NME 0.0.1-beta.105

Beta105 fixes native comparisons of two string concatenations.

- Give each concatenation operand its own rotating checked runtime buffer, so
  C evaluation order cannot make a comparison compare the second result with
  itself.
- Cover the corrected English and Korean sentence behavior with a regression
  test.
