# 0.0.1-beta.16 release candidate

This branch is prepared for the `0.0.1-beta.16` public beta.

## Release gate

Before moving the candidate to `beta`:

1. `cargo fmt --all -- --check`
2. `cargo check --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`
4. `cargo test --workspace`
5. Install the CLI with `cargo install --path crates/nme-cli --locked`.
6. Confirm `nme --version` prints `0.0.1-beta.16`.
7. Run both Schnorr examples:
   - `nme run examples/zk-schnorr-relay.ko`
   - `nme run examples/zk-schnorr-relay.en`
8. Confirm the normal proof passes, saved transcript replay fails, a transcript
   for a preselected challenge can be simulated, that transcript fails on a
   different challenge, and a live relay passes only by forwarding the real
   prover's response.
9. Require the repository's Linux, Windows, and macOS CI matrix to be green.

The project currently distributes this public beta from source on the `beta`
branch. Creating a Git tag or GitHub Release is a separate publishing action
and is intentionally not performed by release preparation.

## Zero-knowledge security scope

The built-in adapter implements the actual finite-field Schnorr
proof-of-knowledge equations with secure randomness and fixed standardized
group parameters. It is intended for learning, testing, and reference use.
CPython's arbitrary-precision integer implementation is not promised to be
constant-time or hardened against local side channels. Production
authentication should use a reviewed cryptographic implementation and bind
proofs to the intended channel/session when relay resistance is required.
