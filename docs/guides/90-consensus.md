# 90 — Full-chain validation — recompute from genesis

English | [한국어](90-consensus.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

[89 — Transaction proofs](89-signatures.md) | [Full NeedMoreCoin guide](cryptocurrency.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [88 — Proof of work](88-proof-of-work.md), [89 — Transaction proofs](89-signatures.md)
- Topic: cryptocurrency
- Result: a single-node validator that replays every transaction and block from genesis

The core rule is simple: **do not trust the program state that existed while a
block was being built**. A validator should take chain data, start from genesis,
and independently execute the same rules again.

NeedMoreCoin rechecks block height, previous-hash linkage, the recomputed block
hash, proof-of-work target, transaction proof, positive amount, balance, fee,
transaction nonce, minted reward, and final supply conservation.

The Korean advanced `사슬검증` function and English advanced `validate_chain`
function make that replay explicit with data structures and functions. The pure
Korean sentence version demonstrates the same principle with a separate set of
verification-state variables.

```sh
nme run examples/needmorecoin-advanced.ko
nme run examples/needmorecoin-advanced.en
```

## Validation is not network consensus

This project implements a **single-node chain-validation core**. It does not
claim to implement agreement among competing nodes. A real network still needs
peer-to-peer propagation, fork choice and reorganization, persistent storage,
difficulty adjustment or another consensus design, resource limits, testnets,
and security review.

That distinction matters: “this chain is valid” and “many distributed nodes
select the same chain” are separate problems.

Continue with the [full NeedMoreCoin guide](cryptocurrency.md) to compare all six
syntax variants and extend the coin yourself.
