# NME 0.0.1-beta.45

Beta.45 closes a native function-parameter namespace gap.

- Reject generated native-runtime names in function parameters before emitting C.
- Catch unused parameters such as `nme_copy`, `nme_cat`, `len`, and `_nme_i` with
  the same precise bilingual diagnostic used for other native identifiers.
