# NME 0.0.1-beta.30

Beta.30 clarifies the native-build diagnostic guidance.

- E9010 and E9011 now explain both backend paths: `nme compile` uses Nuitka,
  while `nme native` uses a system C compiler such as `cc` or `clang`.
- Add public English/Korean lookup regressions for the two toolchains.

Language semantics and generated Python behavior are unchanged.
