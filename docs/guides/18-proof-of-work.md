# 18 — Proof of work — mining

English | [한국어](18-proof-of-work.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [17 — Blockchain](17-blockchain.md), [07 — While](07-while.md)
- 주제 (Topic): 채굴 / mining
- 결과물 (Result): nonce를 찾아 블록을 채굴하고 사슬을 검증하기 / mine a block by finding a nonce and verify the chain

Guide [17](17-blockchain.md) built a chain of hashes, but nothing forced anyone
to do work. Proof of work adds a puzzle: a block is only accepted when its hash
starts with a run of zeros, so miners must try many nonces. The program is
`examples/proof-of-work.nme`, a learning project, not investment advice.

```sh
nme run examples/proof-of-work
```

```text
mined 00a1b099ffa6d8 nonce 460
mined 0096aa3c97a498 nonce 75
mined 00815bd82691fe nonce 145
every hash starts with 00
chain valid: True
after changing the first block: False
```

The exact hashes and nonces differ every run, but three mined hashes always
start with `00`, the chain validates, and tampering fails.

## Steps

1. Difficulty fixes how many leading zeros a hash must have. With difficulty 2,
   `target` is `"00"` — a higher difficulty means a harder puzzle:

   ```text
   # part of examples/proof-of-work.nme
   import hashlib
   difficulty = 2
   target = "0" * difficulty
   ```

2. The `mine` function tries `nonce = 0, 1, 2, ...` until the hash of
   `previous_hash + data + nonce` starts with the target. The `while True:`
   loop from guide [07](07-while.md) keeps trying; `return` leaves when a hash
   fits:

   ```text
   # part of examples/proof-of-work.nme
   import hashlib
   difficulty = 2
   target = "0" * difficulty

   def mine(data, previous_hash):
       nonce = 0
       while True:
           text = previous_hash + data + str(nonce)
           block_hash = hashlib.sha256(text.encode()).hexdigest()
           if block_hash.startswith(target):
               return {"data": data, "nonce": nonce, "hash": block_hash}
           nonce = nonce + 1
   ```

   The nonce is the number the miner guesses. There is no faster way than
   trying one after another, which is why the work is real.

3. A chain only stays valid when every block was honestly mined. `is_valid`
   re-hashes every block from the start and compares. The run changes the first
   block's data and checks again — it returns `False`, so the tamper is caught.

4. The Korean twin `examples/proof-of-work.ko.nme` mines `"블록 "` data and
   prints with `말해` — run it and you get the same six lines in Korean.

## Try it yourself

Copy the example to your own folder and raise `difficulty = 2` to
`difficulty = 3`. Each extra zero makes the puzzle roughly 16 times harder, so
the nonce grows.

## What you learned

- Difficulty fixes how many leading zeros a block's hash must start with.
- Mining means trying nonces until `sha256(previous + data + nonce)` fits the
  target.
- `is_valid` re-hashes the whole chain, so a changed block is caught.
- There is no shortcut for finding a nonce — that is what makes the work real.
