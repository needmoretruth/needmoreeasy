# 17 — Blockchain — a hash-linked ledger

English | [한국어](17-blockchain.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★☆ (4/5)
- 선수 지식 (Prerequisites): [13 — Files](13-files.md), [03 — Set](03-set.md), [05 — Repeat](05-repeat.md)
- 주제 (Topic): 블록체인 / blockchain
- 결과물 (Result): 해시로 연결된 블록들의 이해 / an understanding of blocks linked by hashes

A blockchain is a list of records in which every record points back at the one
before it. The example `examples/blockchain-ledger.nme` builds three linked
blocks with a hash function. It is a learning project, not investment advice.

```sh
nme run examples/blockchain-ledger
```

```text
block 84851753 -> prev 0
block 5fecc8a2 -> prev 84851753
block 2064c320 -> prev 5fecc8a2
the chain has 3 linked blocks
```

The hashes change every run — the data is random — but each block still names
the block before it.

## Steps

1. A hash turns any text into a fixed-length fingerprint. The program imports
   Python's `hashlib` and hashes the previous hash plus the new data:

   ```text
   # part of examples/blockchain-ledger.nme
   import hashlib
   previous_hash = "0"
   data = "message 1"
   combined = previous_hash + data
   block_hash = hashlib.sha256(combined.encode()).hexdigest()
   show f"block {block_hash[:8]} -> prev {previous_hash}"
   ```

   `sha256` returns the same 64 characters for the same input, and a different
   input gives a completely different hash.

2. Each block stores its data, the previous hash, and its own hash. Saving
   `previous_hash` is the link — the next block's `prev` will be this hash:

   ```text
   # part of examples/blockchain-ledger.nme
   import hashlib
   previous_hash = "0"
   data = "message 1"
   combined = previous_hash + data
   block_hash = hashlib.sha256(combined.encode()).hexdigest()
   block = {"data": data, "prev": previous_hash[:8], "hash": block_hash[:8]}
   chain = [block]
   show f"block {block['hash']} -> prev {block['prev']}"
   ```

   The `3 times:` loop from guide [05](05-repeat.md) builds one block per
   round and `chain.append(block)` collects them. If anyone changes a middle
   block, the next block's `prev` no longer matches its hash — that mismatch
   is the tamper evidence that makes the chain useful.

3. The Korean twin `examples/blockchain-ledger.ko.nme` writes the same program
   with `랜덤 사용 최신`, `말해`, and `"메시지 "` data. Run it the same way
   and you get the same shape in Korean.

## Try it yourself

Copy the example to your own folder and change `3 times:` to `5 times:` — you
will see five blocks, each pointing at the previous one.

## What you learned

- A blockchain is a list of blocks, each linked to the previous one by a hash.
- `hashlib.sha256(...)` turns text into a fixed 64-character fingerprint.
- `chain.append(block)` adds a block; `len(chain)` counts them.
- Each block stores the previous hash, which is how tampering becomes visible.
