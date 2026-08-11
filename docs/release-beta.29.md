# NME 0.0.1-beta.29

Beta.29 gives failed Python package installs their own stable diagnostic.

- Add E9025, `pip could not install the package`, with Korean and English
  explanations available through `nme ko E9025` and `nme en E9025`.
- Keep E9010 reserved for native-compiler failures.
- Add a deterministic CLI regression that uses an invalid empty requirement and
  does not contact a package index.

Language semantics and generated Python behavior are unchanged.
