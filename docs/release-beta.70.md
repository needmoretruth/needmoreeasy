# NME 0.0.1-beta.70

Beta.70 makes native output options explicit.

- Reject `nme native run <file> -o <path>` with E9031. Use
  `nme native build <file> -o <path>` to keep the executable and generated C
  source.
