# NME 0.0.1-beta.104

Beta104 corrects the native-backend control-flow contract.

- Document that `break` works inside an `if` nested in a native loop and is
  rejected only outside loops.
- Keep the English and Korean native-backend descriptions aligned with the
  implementation and regression tests.
