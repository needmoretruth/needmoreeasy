# NME 0.0.1-beta.116

Beta116 fixes parenthesized logical `while` conditions with Korean sentence
endings.

- Preserve the closing parenthesis and body boundary in a form such as
  `동안 (준비 그리고 횟수가 2보다 작을 동안)`.
- Keep the shared parser path working for English and Korean sentence,
  beginner, and advanced native `while` surfaces.
- Document that whole-condition parentheses are available in colon-free `if`
  and `while` headers while valid Python calls and colon headers retain Python
  priority.
