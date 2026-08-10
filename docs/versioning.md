# NME versioning and releases

English | [한국어](versioning.ko.md)

NME uses [Semantic Versioning](https://semver.org/) with explicit beta
prereleases while the language is still being designed.

## Current release line

The current release is `0.0.1-beta.9`; the public line began at
`0.0.1-beta.1`. Later public beta releases increase the last number:
`0.0.1-beta.3`, `0.0.1-beta.4`, `0.0.1-beta.5`, `0.0.1-beta.6`, `0.0.1-beta.7`, `0.0.1-beta.8`, `0.0.1-beta.9`, and so on. A Git commit or a branch push is not
automatically a release, so development commits can share the version of the
beta they are preparing.

Each release must update these together:

- the workspace version in `Cargo.toml` and resolved local package versions in
  `Cargo.lock`;
- the changelog and release notes; and
- version references in both English and Korean user documentation.

## `1.0.0` is owner-controlled

Version `1.0.0`, including any `1.0.0-*` prerelease, may be created or
published only after an explicit instruction from the repository owner.
Passing tests, feature completeness, or a release schedule never grants that
permission implicitly.

Until then, NME remains on the `0.x` beta line. Compatibility is still taken
seriously: release notes must call out any changed syntax or generated Python
behavior.
