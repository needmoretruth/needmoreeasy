# NME 0.0.1-beta.18

Beta.18 adds context-bound Schnorr Fiat-Shamir non-interactive proofs to the built-in zero-knowledge adapter while retaining the beta.17 next-generation release gates.

## API

- `zk_nizk_challenge(public_key, commitment, context)` / `영지식비대화도전`
- `zk_nizk_prove(secret, context)` / `영지식비대화증명`
- `zk_nizk_verify(public_key, proof, context)` / `영지식비대화검증`

The proof is a JSON-friendly two-element list `[commitment, response]`. The challenge uses SHA-256 with an NME-specific Group 15 NIZK domain tag and length-prefixed context. The same proof verifies under its original context and is rejected under a different context.

Context binding is not same-context freshness. Protocols that need replay resistance must include a unique request/session/transaction value in the context.

The adapter remains a mathematically faithful learning/reference implementation, not a side-channel-hardened audited production cryptography library.
