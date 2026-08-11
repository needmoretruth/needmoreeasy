# 20 — Consensus — agreeing on a chain

English | [한국어](20-consensus.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [18 — Proof of work](18-proof-of-work.md)
- 주제 (Topic): 합의 / consensus
- 결과물 (Result): 가장 긴 사슬 규칙으로 갈라짐을 해결하는 두 노드 / two nodes resolving a fork by the longest-chain rule

Two miners can find blocks at the same time, so the network may split. How does
everyone end up on the same chain? The program `examples/consensus.nme`
simulates two nodes, a fork, and the longest-chain rule. A real network adds
signatures, many peers, and timing. This is a learning project, not
investment advice.

```sh
nme run examples/consensus
```

```text
two miners start from the genesis block
A mined the first block: 2 vs 1
B hears about A's block and copies it
both nodes hold 2 blocks
B mines a rival block on top, forking the chain
B now has 3 blocks, A has 2
A hears about the longer chain and adopts it (longest chain rule)
both agree: True and True
```

## Steps

1. Every chain starts from a `genesis` block, and the `mine` function from
   guide [18](18-proof-of-work.md) now takes a whole chain. It builds on the
   last block, `chain[-1]`:

   ```text
   # part of examples/consensus.nme
   import hashlib
   difficulty = 2
   target = "0" * difficulty

   def mine(chain, data):
       previous = chain[-1]["hash"]
       nonce = 0
       while True:
           text = previous + data + str(nonce)
           block_hash = hashlib.sha256(text.encode()).hexdigest()
           if block_hash.startswith(target):
               return {"data": data, "nonce": nonce, "hash": block_hash, "prev": previous}
           nonce = nonce + 1
   ```

   `chain[-1]` is the newest block, so a new block always points at the tip.

2. Two nodes, `node_a` and `node_b`, both start from the same genesis block.
   When A mines the first block, B copies it. Then B mines a rival block on
   top — the chain forks:

   ```text
   # part of examples/consensus.nme
   genesis = {"data": "genesis", "nonce": 0, "hash": "0", "prev": ""}
   node_a = [genesis]
   node_b = [genesis]
   a_block = mine(node_a, "A: block 1")
   node_a.append(a_block)
   node_b.append(a_block)
   b_block = mine(node_b, "B: rival")
   node_b.append(b_block)
   show "B now has " + str(len(node_b)) + " blocks, A has " + str(len(node_a))
   ```

   Now the two chains disagree: B has 3 blocks and A has 2.

3. The fork is resolved by a rule: keep the longest valid chain. A hears about
   B's longer chain and copies it with `node_a = list(node_b)`. The example
   re-checks with `is_valid` from guide [18](18-proof-of-work.md) and prints
   `True and True` — both nodes agree.

4. The Korean twin `examples/consensus.ko.nme` simulates the same two miners
   with `말해` output — run it and you get the same story in Korean.

## Try it yourself

Copy the example to your own folder and make A mine an extra block on top so A
has the longer chain, then have B adopt it with `node_a = list(node_b)`.

## What you learned

- Miners can find blocks at the same time, which splits the chain into a fork.
- `mine` builds each new block on the tip of a chain, `chain[-1]`.
- The longest-chain rule makes nodes copy the longer valid chain.
- `is_valid` confirms both nodes end on a chain that still passes every check.
