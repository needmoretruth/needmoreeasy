# 88 — Proof of work — mining transaction blocks

English | [한국어](88-proof-of-work.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

[87 — Cryptocurrency ledger](87-blockchain.md) | [Full NeedMoreCoin guide](cryptocurrency.md)

- Difficulty: ★★★☆☆ (3/5)
- Prerequisites: [87 — Cryptocurrency ledger](87-blockchain.md), [10 — While](10-while.md)
- Topic: cryptocurrency
- Result: link transactions to the previous block and perform SHA-256 proof of work

A valid transaction is not yet a blockchain. Every new block must commit to the
previous block hash, and miners must find candidate data whose hash satisfies a
work target.

The pure Korean sentence example reuses the bundled zero-knowledge adapter's
SHA-256 Fiat-Shamir challenge as its 256-bit work candidate. It generates fresh
nonce/commitment material until the candidate falls below the target.

The advanced examples show conventional proof of work directly: serialize the
height, previous hash, transaction, proof, miner, reward, and work nonce; hash
that payload with `hashlib.sha256`; increment the work nonce until the hex hash
starts with `00`.

The English sentence form exposes the mining loop directly:

```nme
while genesismining
genesisrandom save zero knowledge nonce make
genesiscandidatecommitment save genesisrandom zero knowledge commitment make
genesiscandidatehash save mineraddress genesiscandidatecommitment genesiscontext zero knowledge challenge make
add 1 to genesisattempts
if genesiscandidatehash is less than worktarget
genesiscommitment save genesiscandidatecommitment
genesishash save genesiscandidatehash
genesismining save 0
end
end
```

```sh
nme run examples/needmorecoin-advanced.en
```

Difficulty is intentionally low for learning. Raising it can greatly increase
runtime.

Block 1 commits to the genesis hash, and block 2 commits to block 1's hash. If
an old transaction changes, its block hash changes and the next block's stored
previous hash no longer matches.

Next, [89 — Transaction proofs](89-signatures.md) verifies who authorized a
transaction and why the same valid transaction cannot be replayed.
