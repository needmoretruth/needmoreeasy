# NME 0.0.1-beta.19

Beta.19 makes the repository topology match the operating model: `beta` is the next-generation branch and `main` is an actual ancestor of it.

The final public release is intentionally a two-parent commit:

1. first parent: public beta.18, preserving the beta first-parent release line;
2. second parent: the current `main` tip, making main part of beta's ancestry without modifying main.

The release tree is validated before that merge commit is created. The public beta-version guard still checks the first parent, so beta.18 -> beta.19 remains an exact +1 transition.

No feature from beta.18 is removed; this release is a topology and release-engineering convergence point for future beta-first development.
