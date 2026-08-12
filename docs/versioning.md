# NME versioning and releases

English | [한국어](versioning.ko.md)

[Home](../README.md) | [Getting started](getting-started.md) | [Tutorial](tutorial.md) | [Language reference](language.md)

NME uses [Semantic Versioning](https://semver.org/) with explicit beta
prereleases while the language is still being designed.

## Current release line

The current release is `0.0.1-beta.51`; the public line began at
`0.0.1-beta.1`. Later public beta releases increase the last number by one.
A Git commit or a branch push is not automatically a release, so development
commits can share the version of the beta they are preparing.

Read the [beta.51 release notes](release-beta.51.md) for the current checkpoint;
older beta notes remain available beside it.

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
