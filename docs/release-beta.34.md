# NME 0.0.1-beta.34

Beta.34 gives the missing package argument its own CLI diagnostic.

- Add E9030, `the package name is missing`, for `nme install` without a
  package name instead of reusing E9003, which describes missing or invalid
  option values.
- Add English and Korean public lookup coverage for E9030.

Language semantics and generated Python behavior are unchanged.
