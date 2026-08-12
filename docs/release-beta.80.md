# NME 0.0.1-beta.80

Beta.80 keeps generated native C readable and clean under common warning-as-error
settings.

- GCC and Clang mark unused generated runtime helpers explicitly; MSVC receives
  the corresponding warning suppression.
- `NME_UNUSED`, the generated helper macro, is reserved so a user identifier
  cannot be rewritten by the C preprocessor.
