# NME 0.0.1-beta.17

`beta` is the repository's next-generation release line.

## Public beta invariant

Every commit on public `beta` must advance `beta.N` to exactly `beta.N+1`. The release commit subject, workspace manifest, and the three workspace package entries in `Cargo.lock` must agree. The CI guard checks the real first parent with a two-commit checkout.

## Compatibility gate

The release gate runs format, locked check, Clippy with warnings denied, locked full workspace tests, CLI installation, and smoke tests on Ubuntu, Windows, and macOS. Beta and pull requests also test with CPython 3.10, 3.12, and 3.14.
