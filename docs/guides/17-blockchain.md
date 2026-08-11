# 17 — Cryptocurrency ledger — state and transactions

English | [한국어](17-blockchain.ko.md)

[README](../../README.md) | [Guides](index.md) | [Full NeedMoreCoin guide](cryptocurrency.md)

- Difficulty: ★★★★☆
- Prerequisites: [03 — Set](03-set.md), [06 — If](06-if.md), [07 — While](07-while.md)
- Result: a small cryptocurrency ledger with balances, fees, supply, and transaction nonces

The blockchain learning track now uses one coherent `NeedMoreCoin` example set
instead of separate toy hash/signature programs. Six variants cover sentence,
beginner, and advanced NME in Korean and English.

Start with:

```sh
nme check examples/needmorecoin-sentence.en
nme run examples/needmorecoin-sentence.en
```

## State rules

The ledger stores balances, each address's last transaction nonce, and total
supply. A transaction is applied only after its proof, positive amount,
`amount + fee` balance requirement, and exact next nonce all validate.

The nonce prevents an already accepted signed transaction from being applied a
second time. Fees move existing coins from sender to miner; mining rewards mint
new coins. Therefore the validator must preserve:

```text
sum of wallet balances = initial supply + minted mining rewards
```

## Six variants

- `needmorecoin-sentence.ko.nme`
- `needmorecoin-sentence.en.nme`
- `needmorecoin-beginner.ko.nme`
- `needmorecoin-beginner.en.nme`
- `needmorecoin-advanced.ko.nme`
- `needmorecoin-advanced.en.nme`

Next, [18 — Proof of work](18-proof-of-work.md) binds this state to previous
block hashes and requires real SHA-256 work before a block is accepted.
