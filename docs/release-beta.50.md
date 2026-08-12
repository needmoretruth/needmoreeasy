# NME 0.0.1-beta.50

Beta.50 gives native integer arithmetic an explicit bounded policy.

- Accept signed 32-bit integer literals only.
- Check native addition, subtraction, multiplication, negation, and modulo for
  overflow or a zero divisor.
- Report bilingual runtime errors instead of relying on undefined C behavior.
- Reject float arguments and return values in native functions instead of
  silently converting them through C `int` parameters and returns.
