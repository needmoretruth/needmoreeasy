# Context-bound Schnorr NIZK

NME's bundled `zero_knowledge` adapter version 0.0.2 keeps the interactive Schnorr tools and adds a Fiat-Shamir non-interactive proof.

```nme
use zero_knowledge latest
proof = zk_nizk_prove(secret, context)
valid = zk_nizk_verify(public_key, proof, context)
```

The proof is `[commitment, response]`, so it can be serialized with ordinary JSON. The deterministic challenge is SHA-256 over an NME domain tag, the Group 15 generator, the commitment, the public key, and a length-prefixed UTF-8 context. The transcript order follows the Schnorr Fiat-Shamir construction (`g`, commitment `V`, public value `A`) and the explicit context plays the role of protocol/user binding information.

Changing the context changes the challenge and makes the same proof fail. This is context binding, not a complete anti-replay protocol: if the same context is accepted twice, the same valid proof is still valid twice. Include a unique request ID, session nonce, transaction identifier, or equivalent freshness value in the context when replay resistance is required.

Korean sentence syntax is available without punctuation:

```nme
증명값은 비밀값과 문맥값으로 영지식 비대화 증명 만들기
검증값은 공개값과 증명값과 문맥값으로 영지식 비대화 검증
도전값은 공개값과 약속값과 문맥값으로 영지식 비대화 도전 만들기
```

This remains learning/reference cryptography rather than a side-channel-hardened audited production implementation.
