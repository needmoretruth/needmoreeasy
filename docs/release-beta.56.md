# NME 0.0.1-beta.56

Beta.56 hardens native floating-point literals.

- Reject non-finite float literals before generating C.
- Emit finite whole-number float literals as explicit C `double` values so
  literals such as `5.0` and `-0.0` retain their floating-point type and sign.
