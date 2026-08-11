# 20 — 합의 — 사슬에 동의하기

[English](20-consensus.md) | 한국어

[README](../../README.ko.md) | [설치](../install.ko.md) | [시작하기](../getting-started.ko.md) | [학습 과정](../tutorial.ko.md) | [문법 안내](../language.ko.md) | [가이드](index.ko.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [18 — 작업 증명](18-proof-of-work.ko.md)
- 주제 (Topic): 합의 / consensus
- 결과물 (Result): 가장 긴 사슬 규칙으로 갈라짐을 해결하는 두 노드 / two nodes resolving a fork by the longest-chain rule

두 채굴자가 동시에 블록을 찾을 수 있으므로 네트워크가 갈라질 수 있습니다.
그래도 모두가 어떻게 같은 사슬에 도달할까요? 프로그램
`examples/consensus.ko.nme`는 두 노드, 갈라짐, 가장 긴 사슬 규칙을
시뮬레이션합니다. 실제 네트워크에는 서명과 많은 노드, 시간 개념이
추가됩니다. 이 프로젝트는 학습용이며 투자 조언이 아닙니다.

```sh
nme run examples/consensus.ko
```

```text
두 채굴자가 genesis 블록에서 시작합니다
A가 첫 블록을 채굴했습니다: 2 대 1
B가 A의 블록을 듣고 복사합니다
두 노드 모두 2개 블록 보유
B가 위에 경쟁 블록을 채굴해 사슬을 갈라지게 합니다
B는 3개, A는 2개
A가 더 긴 사슬을 듣고 채택합니다 (가장 긴 사슬 규칙)
양쪽 합의: True 그리고 True
```

## 단계

1. 모든 사슬은 `genesis` 블록에서 시작하고, [18](18-proof-of-work.ko.md)
   가이드의 `mine` 함수가 이제 사슬 전체를 받습니다. 마지막 블록
   `chain[-1]` 위에 만듭니다:

   ```text
   # examples/consensus.ko.nme의 일부
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

   `chain[-1]`이 가장 새로운 블록이므로 새 블록은 항상 사슬의 끝을
   가리킵니다.

2. `node_a`와 `node_b` 두 노드가 같은 genesis 블록에서 시작합니다. A가 첫
   블록을 채굴하면 B가 그것을 복사합니다. 그러고 나서 B가 위에 경쟁 블록을
   채굴합니다 — 사슬이 갈라집니다:

   ```text
   # examples/consensus.ko.nme의 일부
   genesis = {"data": "genesis", "nonce": 0, "hash": "0", "prev": ""}
   node_a = [genesis]
   node_b = [genesis]
   a_block = mine(node_a, "A: 블록 1")
   node_a.append(a_block)
   node_b.append(a_block)
   b_block = mine(node_b, "B: 경쟁")
   node_b.append(b_block)
   말해 "B는 " + str(len(node_b)) + "개, A는 " + str(len(node_a)) + "개"
   ```

   이제 두 사슬이 서로 다릅니다: B는 3개, A는 2개입니다.

3. 갈라짐은 규칙 하나로 해결됩니다: 가장 긴 유효한 사슬을 유지합니다. A는
   B의 더 긴 사슬을 듣고 `node_a = list(node_b)`로 복사합니다. 예제는
   [18](18-proof-of-work.ko.md) 가이드의 `is_valid`로 다시 확인해
   `True 그리고 True`를 출력합니다 — 양쪽이 합의합니다.

4. 영어 쌍둥이 `examples/consensus.nme`는 같은 두 채굴자를 `show`로
   출력합니다 — 실행하면 같은 이야기를 영어로 볼 수 있습니다.

## 직접 해보기

예제를 내 폴더에 복사하고 A가 위에 블록을 하나 더 채굴해 더 긴 사슬을
가지게 한 다음, `node_a = list(node_b)`로 B가 그것을 채택하게 해 보세요.

## 배운 것

- 채굴자가 동시에 블록을 찾으면 사슬이 갈라져 포크가 됩니다.
- `mine`은 새 블록을 사슬 끝 `chain[-1]` 위에 만듭니다.
- 가장 긴 사슬 규칙은 노드들이 더 긴 유효한 사슬을 복사하게 합니다.
- `is_valid`는 양쪽이 모든 검사를 통과하는 사슬에 도달했는지 확인합니다.
