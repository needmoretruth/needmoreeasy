# NME 0.0.1-beta.33

Beta.33 makes two module-related CLI failures precise.

- Add E9028, `two imported modules have the same name`, instead of labeling a
  multi-file module collision as an option-value error.
- Add E9029, `module imports are not supported by nme compile`, for the current
  native compilation limitation, with Korean and English explanations through
  `nme ko E9029` and `nme en E9029`.
- Add bilingual CLI regressions for both failures and keep E9003 reserved for
  missing or invalid option values.

Language semantics and generated Python behavior are unchanged.
