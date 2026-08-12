# NME 0.0.1-beta.159

Beta159 keeps the full-tree rustfmt gate for branch pushes and makes the
pull-request Format job check only Rust files changed by the pull request. This
keeps a stale formatting difference already present on the base branch from
blocking an otherwise formatted contribution.
