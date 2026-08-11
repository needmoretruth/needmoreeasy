# 19 — Transaction proofs — wallet ownership and replay prevention

English | [한국어](19-signatures.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

[18 — Proof of work](18-proof-of-work.md) | [Full NeedMoreCoin guide](cryptocurrency.md)

- Difficulty: ★★★★★
- Prerequisites: [17 — Cryptocurrency ledger](17-blockchain.md)
- Result: validate transaction authorization with a public value and reject replay

NeedMoreCoin replaces the old shared-secret HMAC lesson with a Schnorr-style
proof that can be checked using a public value. The wallet owner keeps the
secret; validators use the public value.

A transaction proof is bound to sender, receiver, amount, fee, and transaction
nonce. The bundled adapter performs secure random generation, 3072-bit MODP
Group 15 subgroup calculations, and a SHA-256 Fiat-Shamir challenge. In this
example the context-bound non-interactive proof acts as the transaction-signing
mechanism.

The program deliberately changes a valid 25-coin transfer to 250 while reusing
the old proof. Verification fails because the context changed, and the block
hash also differs.

A valid signature is still not enough to prevent replay. Each sender stores the
last accepted nonce. After Alice uses nonce 1, the next acceptable value is 2;
re-submitting the old nonce-1 transaction is rejected.

```sh
nme run examples/needmorecoin-sentence.en
```

Next, [20 — Full-chain validation](20-consensus.md) stops trusting construction-
time variables and recomputes the entire chain from genesis state.
