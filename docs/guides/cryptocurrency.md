# Build NeedMoreCoin — one blockchain core in six NME styles

[한국어](cryptocurrency.ko.md) | English

[README](../../README.md) | [Language reference](../language.md) | [Guides](index.md) | [Example authoring](example-authoring.md)

This guide implements the same small cryptocurrency across NME's three syntax
levels and two languages. The point is not price or investing. The point is to
make the state and validation rules of a blockchain executable and inspectable.

This is a learning-oriented, single-process blockchain. It does not implement
a peer-to-peer node network, persistent database, fork choice, mempool,
difficulty adjustment, production key storage, or a security audit. Do not use
it to hold real money or present it as an investment product.

## Six examples

| Syntax level | Korean | English | Main purpose |
| --- | --- | --- | --- |
| Sentence | [`needmorecoin-sentence.ko.nme`](../../examples/needmorecoin-sentence.ko.nme) | [`needmorecoin-sentence.en.nme`](../../examples/needmorecoin-sentence.en.nme) | Read the blockchain flow as sentences |
| Beginner | [`needmorecoin-beginner.ko.nme`](../../examples/needmorecoin-beginner.ko.nme) | [`needmorecoin-beginner.en.nme`](../../examples/needmorecoin-beginner.en.nme) | Add quotes, operators, colons, and indentation |
| Advanced | [`needmorecoin-advanced.ko.nme`](../../examples/needmorecoin-advanced.ko.nme) | [`needmorecoin-advanced.en.nme`](../../examples/needmorecoin-advanced.en.nme) | Implement data structures and verification functions in Python-compatible NME |

The files teach the same model but are not forced to be line-for-line
translations. Each level uses the clearest spelling available at that level.

### Strong purity rules for both sentence files

`needmorecoin-sentence.ko.nme` contains only Hangul, decimal digits, and
whitespace in executable source. It has no ASCII identifiers, underscores,
quotes, parentheses, commas, colons, operators, Python `import`, or even source
comments.

`needmorecoin-sentence.en.nme` is equally strict: executable source contains
only ASCII letters, decimal digits, and whitespace. It has no underscores,
quotes, parentheses, commas, colons, operators, Python calls, or source
comments. Phrases such as `zero knowledge secret make` and `secret zero
knowledge public make` lower to the real bundled zero-knowledge operations.
Regression tests check both character sets and verify that every non-empty line
in both sentence files is actually lowered by NME, so Python cannot be hidden
inside a file that claims to be pure sentence syntax.

## What is really computed

The sentence and beginner examples use NME's bundled `zero_knowledge` / `영지식`
adapter. It performs secure random generation, finite-field Schnorr
proof-of-knowledge calculations in the 3072-bit MODP Group 15 subgroup, and a
SHA-256 Fiat-Shamir challenge. The examples use the context-bound
non-interactive proof as a transaction-signature mechanism: changing the
transaction context invalidates the old proof.

The advanced examples implement the same principles directly with Python's
standard library. Wallet secrets come from `secrets`, public values and Schnorr
responses use modular exponentiation, and block proof of work hashes a canonical
block payload with `hashlib.sha256`.

All six examples demonstrate the same security properties:

1. **Wallet ownership** — only the holder of a secret can create a proof for the transaction context.
2. **Transaction tamper detection** — changing 25 coins to 250 invalidates the original proof.
3. **Replay prevention** — each sender has a monotonically increasing transaction nonce.
4. **Proof of work** — candidates are recomputed until the configured target is satisfied.
5. **Block linkage** — each new block commits to the previous block hash.
6. **State replay** — validation starts from initial state and reapplies transactions.
7. **Supply conservation** — initial supply plus mining rewards must equal the sum of wallet balances.

## Run the matrix

```sh
nme check examples/needmorecoin-sentence.ko
nme run examples/needmorecoin-sentence.ko
nme check examples/needmorecoin-sentence.en
nme run examples/needmorecoin-sentence.en
nme check examples/needmorecoin-beginner.ko
nme run examples/needmorecoin-beginner.ko
nme check examples/needmorecoin-beginner.en
nme run examples/needmorecoin-beginner.en
nme check examples/needmorecoin-advanced.ko
nme run examples/needmorecoin-advanced.ko
nme check examples/needmorecoin-advanced.en
nme run examples/needmorecoin-advanced.en
```

Mining uses changing nonce material, so hashes and attempt counts may differ
between runs. A healthy run should still report successful chain validation,
tamper rejection, replay rejection, and supply conservation.

## Step 1 — define monetary state rules first

Do not begin by choosing a hash function. Begin by defining deterministic state
rules that every validator can execute identically. NeedMoreCoin uses:

- initial supply: 100
- block reward: 10 per transaction block
- fee: 1 per transaction
- amount: must be greater than zero
- sender balance: must cover amount + fee
- sender nonce: must increase exactly by one
- proof of work: block hash must satisfy the configured target

A blockchain is more than a list of hashes. Its useful core is a deterministic
state transition function that lets independent validators reach the same
result from the same chain.

## Step 2 — create wallets

The Korean sentence version shows the smallest punctuation-free form:

```text
영지식 사용 최신
민수비밀은 영지식 비밀 만들기
민수주소는 민수비밀로 영지식 공개값 만들기
```

The private value must remain secret. The public value can be used by other
participants to verify transaction proofs. This learning project uses the full
public integer as an address; production systems often add key serialization,
checksums, and an address encoding layer.

## Step 3 — bind a signature to a transaction context

A signature must cover an unambiguous, reproducible transaction representation.
The examples bind at least:

- sender address
- receiver address
- amount
- fee
- transaction nonce

The Korean sentence version constructs the context and proof like this:

```text
거래하나내용은 민수주소 에서 지수주소 에게 거래하나금액 코인 전송 수수료 거래하나수수료 거래번호 거래하나번호
거래하나서명은 민수비밀과 거래하나내용으로 영지식 비대화 증명 만들기
거래하나서명검증은 민수주소와 거래하나서명과 거래하나내용으로 영지식 비대화 검증
```

If the amount changes, the context changes, so the old proof no longer verifies.

## Step 4 — validate state before mutating it

A correct signature is necessary but not sufficient. Validation should check,
in order:

1. Does the proof match the public value and exact transaction context?
2. Is the amount positive?
3. Does the sender have at least amount + fee?
4. Is the nonce exactly the previous nonce + 1?
5. Only after all checks pass, mutate balances and the stored nonce.

Keeping mutation after validation makes failure paths much easier to reason
about and test.

## Step 5 — link blocks with the previous hash

A transaction block commits to:

- block height
- previous block hash
- transaction context
- transaction proof
- miner address
- reward
- proof-of-work nonce or candidate material

The previous hash is what forms the chain. Editing block 1 changes its hash,
which no longer matches the previous-hash field committed by block 2.

## Step 6 — mine a proof-of-work block

The Korean sentence example reuses the bundled adapter's SHA-256 Fiat-Shamir
challenge as a 256-bit block-hash candidate. It generates fresh nonce material
until the candidate falls below `작업목표`.

The advanced examples show the more conventional layout directly: serialize the
block payload, hash it with SHA-256, increment an integer work nonce, and stop
when the hexadecimal hash starts with `0`. Difficulty is intentionally low for
learning. Increasing it can make execution much slower.

## Step 7 — revalidate instead of trusting construction-time variables

A validator should recompute, not trust, the values that were present when a
block was built. Revalidation covers:

- transaction context
- transaction proof
- nonce and balance rules
- block hash
- proof-of-work target
- previous-hash linkage
- minted rewards
- final balance sum

The advanced `validate_chain` function replays this from genesis. The sentence
version maintains a separate verification state to demonstrate the same idea
without requiring Python data structures.

## Step 8 — attack your own example

A security example should include negative tests.

### Amount tampering

The program changes the original 25-coin transfer to 250 while reusing its old
proof. Signature verification fails, and the corresponding block hash no longer
matches.

### Replay

After Alice has used nonce 1, the same transaction tries to use nonce 1 again.
The validator expects nonce 2 and rejects it.

When you add a security rule, add an attack case that should fail because of
that rule.

## Turn it into your own coin

Keep the structure fixed at first and change one dimension at a time:

1. Rename the coin.
2. Change initial supply.
3. Change mining reward and fee.
4. Change the two transfer amounts and confirm validation still succeeds.
5. Adjust the sentence target or advanced mining prefix and compare work counts.
6. Add a third wallet.
7. Add another signed transaction and a block linked to the current tip.
8. Extend the validator to replay the new transaction and block.
9. Reuse an old nonce or edit an amount and confirm rejection.

Run `nme check` before `nme run` after each small change.

## What a real network still needs

A production-oriented design must separately address:

- canonical binary serialization
- persistent chain state and crash recovery
- a mempool and duplicate handling
- peer-to-peer networking, authentication, and rate limits
- competing-chain selection and reorganization
- difficulty adjustment or another consensus mechanism
- encrypted key storage and backup
- network/chain identifiers
- time and resource limits, including maximum block and transaction sizes
- testnets, fuzzing, reproducible vectors, and independent security review

Without those pieces, calling this a production cryptocurrency would be
misleading. NeedMoreCoin is intentionally a small, verifiable single-node core
that prepares learners to understand those later systems.

## Continue

- [17 — cryptocurrency ledger](17-blockchain.md)
- [18 — proof of work](18-proof-of-work.md)
- [19 — transaction signatures and replay](19-signatures.md)
- [20 — full-chain validation](20-consensus.md)
- [How to write strong NME examples](example-authoring.md)
- [Example template](example-template.md)
