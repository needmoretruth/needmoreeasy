# NME 0.0.1-beta.108

Beta108 makes native branch-return flow agree with its binding analysis.

- A branch that returns early no longer counts as a fall-through path when the
  native backend decides whether a later binding is definitely initialized.
- Keep the required top-level integer return and conservative loop behavior.
- Cover English and Korean sentence, beginner, and mixed advanced native forms,
  with synchronized reference, guide, and language documentation.
