# NME 0.0.1-beta.155

Beta155 extends the shared Python scope diagnostics to one-line function and
class suites. `global`/`nonlocal` conflicts, annotated targets, invalid
`nonlocal` placement, and star imports now receive stable diagnostics there,
while valid nested scopes and module-level ordering remain unchanged.
