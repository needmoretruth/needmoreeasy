# NME 0.0.1-beta.28

Beta.28 repairs the language selection for two Korean CLI aliases.

- `nme 네이티브` now reports native-command errors in Korean first, followed by
  the equivalent English diagnostic.
- `nme 설치` now reports package-install errors in the same Korean-first
  bilingual format.
- Add CLI regression coverage for both aliases while keeping English commands
  English-only.

Language semantics and generated Python behavior are unchanged.
