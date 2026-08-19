# NME versioning and releases

English | [한국어](versioning.ko.md)

[Home](../README.md) | [Getting started](getting-started.md) | [Tutorial](tutorial.md) | [Language reference](language.md)

The version is **`0.MINOR.PATCH`**. It moves one step at a time, it moves only
when something is published, and it never reaches `1`.

`scripts/check-version.py` enforces all three of those on every build, and
`scripts/bump-version.py` is the only thing that writes a new number.

## What the three parts mean

- **`0`** — the leading zero is the honest part. NME is still being designed,
  and a program written today may need one line changed tomorrow. Saying `1`
  would promise the opposite.
- **`MINOR`** goes up when you can write something you could not write before:
  a new statement, a spelling the compiler now accepts, a new bundled module.
  Nothing you already wrote has to change for a minor step, unless the release
  notes say so.
- **`PATCH`** goes up when the same programs behave better: a fixed
  mis-compile, a clearer error message, a faster build.

The earlier line was `0.0.1-beta.N`, counting to `beta.160`. The counter is
gone: a number that only ever went up by one said nothing about what changed.

## The version moves only when something is published

A commit is not a release. A branch push is not a release. The number goes up
in the commit that the site is then pinned to — publishing the compiler today
means the site building `nme-web` against one commit of this repository and
shipping the WebAssembly to needmoreeasy.com, so that commit *is* the release.
`bump-version.py` is what writes the number; nothing else edits it by hand.

That is why development commits share the version of the release they are
preparing, and why the changelog carries an `Unreleased` heading between
releases.

Bumping updates all of these together, and `check-version.py` fails the build
if any of them disagrees:

- the workspace version in `Cargo.toml`, and the resolved local packages in
  `Cargo.lock`;
- `CHANGELOG.md` and `CHANGELOG.ko.md`;
- every version reference in the English and Korean documentation;
- the generated syntax reference and the AI prompt files.

## `1.0.0` will not happen

Not "not yet" — the repository refuses it. `check-version.py` rejects any
version whose major number is `1` or higher, and says why.

A release that says `1` promises that programs written today keep working. The
owner has said the opposite in as many words: while the language is on `0.x`,
the compiler may change without keeping old code working, because getting the
language right matters more than protecting what was written during its first
month. Compatibility is still taken seriously in the small: every release note
must call out any changed spelling or changed generated Python.
