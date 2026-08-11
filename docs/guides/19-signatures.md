# 19 — Signatures — proving identity

English | [한국어](19-signatures.ko.md)

[Home](../../README.md) | [Install](../install.md) | [Getting started](../getting-started.md) | [Tutorial](../tutorial.md) | [Language reference](../language.md) | [Guides](index.md)

- 난이도 (Difficulty): ★★★★★ (5/5)
- 선수 지식 (Prerequisites): [17 — Blockchain](17-blockchain.md)
- 주제 (Topic): 서명 / signatures
- 결과물 (Result): hmac으로 메시지 서명하고 검증하기 / sign and verify a message with hmac

A hash proves a message did not change. A signature proves who wrote it. The
program `examples/signatures.nme` uses Python's `hmac` standard library to sign
a message with a shared secret and verify it. Real coins use public-key
signatures from a cryptography library; this is a learning project, not
investment advice.

```sh
nme run examples/signatures
```

```text
message: pay Mina 10
signature: 0973ca76d4ef4258...
signed message verifies: True
tampered message verifies: False
a wrong key cannot forge it: False
```

## Steps

1. Signing needs a secret that only the sender and receiver know. The `sign`
   function folds that secret into a hash of the message:

   ```text
   # part of examples/signatures.nme
   import hmac
   import hashlib
   secret = "shared-secret-key"
   message = "pay Mina 10"

   def sign(text):
       return hmac.new(secret.encode(), text.encode(), hashlib.sha256).hexdigest()

   signature = sign(message)
   show "message: " + message
   show "signature: " + signature[:16] + "..."
   ```

   `hmac` uses the same `sha256` hash from guide [17](17-blockchain.md), but
   the secret is mixed in, so the signature cannot be forged without it.

2. To verify, the receiver signs the message again with the same secret and
   compares. `compare_digest` compares without leaking timing information:

   ```text
   # part of examples/signatures.nme
   signature = sign(message)
   show "signed message verifies: " + str(hmac.compare_digest(sign(message), signature))
   tampered = "pay Mina 100"
   show "tampered message verifies: " + str(hmac.compare_digest(sign(tampered), signature))
   ```

   The same message verifies as `True`; changing one digit in the message
   makes it `False`. The last lines of the example sign with a different secret
   and compare — a wrong key cannot forge a signature either.

3. The Korean twin `examples/signatures.ko.nme` signs `"민수에게 10 지불"` and
   prints with `말해` — run it and you get the same five lines in Korean.

## Try it yourself

Copy the example to your own folder and change `secret` to your own key. A
different secret makes a different signature, and a message signed with the old
secret stops verifying.

## What you learned

- A signature proves who wrote a message; a hash alone only proves it did not
  change.
- `hmac.new(key, message, hashlib.sha256)` signs with a shared secret.
- `hmac.compare_digest(...)` verifies a signature safely.
- A wrong key or a changed message both fail verification.
