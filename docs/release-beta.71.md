# NME 0.0.1-beta.71

Beta.71 makes native action selection explicit.

- Reject `nme native run build <file>` and other repeated `run`/`build`
  action words with E9032. Choose exactly one action.
- Make native CLI integration-test directories collision-resistant when tests
  run in parallel.
