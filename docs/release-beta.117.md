# NME 0.0.1-beta.117

Beta117 fixes parenthesized NME conditions with Korean sentence endings.

- Preserve the closing parenthesis and body boundary in a form such as
  `동안 (준비 그리고 횟수가 2보다 작을 동안)`.
- Apply the same shared boundary handling to `if`/`elif`, including
  `만약 (점수가 2보다 작으면)`.
- Keep the shared parser path working for English and Korean sentence,
  beginner, and advanced native conditions.
- Document that whole-condition parentheses remain colon-free NME syntax while
  valid Python calls and colon headers retain Python priority.
