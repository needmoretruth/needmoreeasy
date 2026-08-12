# NME 0.0.1-beta.63

Beta.63 hardens native loop-control validation.

- Reject `break` inside a non-loop native block with the stable bilingual
  `E0102` diagnostic before C generation.
