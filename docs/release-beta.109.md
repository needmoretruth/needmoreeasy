# NME 0.0.1-beta.109

Beta109 extends native fall-through analysis to loop exits.

- A branch that breaks out of its enclosing native loop no longer counts as a
  fall-through path when deciding whether a later binding is definitely
  initialized.
- Keep conservative behavior for loops that may not execute and preserve the
  required top-level integer return for native functions.
- Cover English and Korean sentence, beginner, and mixed advanced forms, with
  synchronized native references and guides.
