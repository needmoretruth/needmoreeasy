# NME 0.0.1-beta.97

Beta97 documents finite-float truthiness in the native subset and adds an
English/Korean end-to-end native regression test.

- Nonzero finite floats are true in native `if`/`while` conditions; zero is
  false.
- English and Korean native programs exercise the same behavior.
- Language semantics and generated Python behavior remain unchanged.
