#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path

OLD = "0.0.1-beta.17"
NEW = "0.0.1-beta.18"


def replace_once(path: str, old: str, new: str) -> None:
    file = Path(path)
    text = file.read_text(encoding="utf-8")
    count = text.count(old)
    assert count == 1, (path, count, old[:100])
    file.write_text(text.replace(old, new, 1), encoding="utf-8")


def replace_all(path: str, old: str, new: str, minimum: int = 1) -> None:
    file = Path(path)
    text = file.read_text(encoding="utf-8")
    count = text.count(old)
    assert count >= minimum, (path, count, old)
    file.write_text(text.replace(old, new), encoding="utf-8")


# Public workspace version.
for name in [
    "Cargo.toml",
    "Cargo.lock",
    "README.md",
    "README.ko.md",
    "docs/install.md",
    "docs/install.ko.md",
    "docs/versioning.md",
    "docs/versioning.ko.md",
    "crates/nme-cli/tests/cli.rs",
]:
    text = Path(name).read_text(encoding="utf-8")
    if OLD in text:
        Path(name).write_text(text.replace(OLD, NEW), encoding="utf-8")

# Bundled zero-knowledge module version and syntax surface.
replace_once(
    "crates/nme-core/src/syntax.rs",
    'pub const ZERO_KNOWLEDGE_MODULE_VERSION: &str = "0.0.1";',
    'pub const ZERO_KNOWLEDGE_MODULE_VERSION: &str = "0.0.2";',
)
replace_once(
    "crates/nme-core/src/syntax.rs",
    "    SimulatedResponse,\n",
    """    /// Fiat-Shamir challenge bound to the public key, commitment, and explicit context.\n    NizkChallenge {\n        public_key: Code,\n        commitment: Code,\n        context: Code,\n    },\n    /// JSON-friendly non-interactive Schnorr proof `[commitment, response]`.\n    NizkProof {\n        secret: Code,\n        context: Code,\n    },\n    /// Verify a context-bound non-interactive Schnorr proof.\n    NizkVerify {\n        public_key: Code,\n        proof: Code,\n        context: Code,\n    },\n    SimulatedResponse,\n""",
)

# Add Korean sentence forms. English users can use the bundled zk_nizk_* API directly.
parser_anchor = """fn parse_zero_knowledge_value(tokens: &[Token]) -> Option<Value> {\n    use crate::syntax::ZeroKnowledgeValue as Zk;\n"""
parser_cases = """

    if tokens.len() == 7
        && token_matches_exact(&tokens[3], &["영지식"])
        && token_matches_exact(&tokens[4], &["비대화"])
        && token_matches_exact(&tokens[5], &["도전"])
        && token_matches_exact(&tokens[6], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::NizkChallenge {
            public_key: zero_knowledge_code_with_particle(&tokens[0], &["과", "와"] )?,
            commitment: zero_knowledge_code_with_particle(&tokens[1], &["과", "와"] )?,
            context: zero_knowledge_code_with_particle(&tokens[2], &["으로", "로"] )?,
        }));
    }

    if tokens.len() == 6
        && token_matches_exact(&tokens[2], &["영지식"])
        && token_matches_exact(&tokens[3], &["비대화"])
        && token_matches_exact(&tokens[4], &["증명"])
        && token_matches_exact(&tokens[5], &["만들기"])
    {
        return Some(Value::ZeroKnowledge(Zk::NizkProof {
            secret: zero_knowledge_code_with_particle(&tokens[0], &["과", "와"] )?,
            context: zero_knowledge_code_with_particle(&tokens[1], &["으로", "로"] )?,
        }));
    }

    if tokens.len() == 6
        && token_matches_exact(&tokens[3], &["영지식"])
        && token_matches_exact(&tokens[4], &["비대화"])
        && token_matches_exact(&tokens[5], &["검증"])
    {
        return Some(Value::ZeroKnowledge(Zk::NizkVerify {
            public_key: zero_knowledge_code_with_particle(&tokens[0], &["과", "와"] )?,
            proof: zero_knowledge_code_with_particle(&tokens[1], &["과", "와"] )?,
            context: zero_knowledge_code_with_particle(&tokens[2], &["으로", "로"] )?,
        }));
    }
"""
replace_once("crates/nme-core/src/parser.rs", parser_anchor, parser_anchor + parser_cases)

# Protect all names injected by the bundled adapter from accidental overwrites.
replace_once(
    "crates/nme-core/src/parser.rs",
    '            "zero_knowledge_version",\n            "영지식버전",',
    '''            "zk_group_bytes",\n            "영지식그룹바이트",\n            "_nme_zk_context_bytes",\n            "_nme_zk_int_bytes",\n            "_nme_zk_context_frame",\n            "zk_nizk_challenge",\n            "영지식비대화도전",\n            "zk_nizk_prove",\n            "영지식비대화증명",\n            "zk_nizk_verify",\n            "영지식비대화검증",\n            "zero_knowledge_version",\n            "영지식버전",''',
)

# Extend the bundled Python adapter. The challenge follows the Schnorr/Fiat-Shamir
# transcript order g || V(commitment) || A(public key), with a domain tag and a
# length-prefixed UTF-8 context frame to make concatenation unambiguous.
replace_once(
    "crates/nme-core/src/lower.rs",
    '    "zero_knowledge_version = 영지식버전 = ",\n',
    '''    "zk_group_bytes = 영지식그룹바이트 = (zk_prime.bit_length() + 7) // 8; ",\n    "_nme_zk_context_bytes = lambda 문맥값: 문맥값 if isinstance(문맥값, bytes) else str(문맥값).encode(\\\"utf-8\\\"); ",\n    "_nme_zk_int_bytes = lambda 값: int(값).to_bytes(zk_group_bytes, \\\"big\\\"); ",\n    "_nme_zk_context_frame = lambda 문맥값: (lambda 바이트: len(바이트).to_bytes(8, \\\"big\\\") + 바이트)(_nme_zk_context_bytes(문맥값)); ",\n    "zk_nizk_challenge = 영지식비대화도전 = lambda 공개값, 약속값, 문맥값: int.from_bytes(__import__(\\\"hashlib\\\").sha256(b\\\"NME-SCHNORR-GROUP15-NIZK-v1\\\\0\\\" + _nme_zk_int_bytes(zk_generator) + _nme_zk_int_bytes(약속값) + _nme_zk_int_bytes(공개값) + _nme_zk_context_frame(문맥값)).digest(), \\\"big\\\"); ",\n    "zk_nizk_prove = 영지식비대화증명 = lambda 비밀값, 문맥값: (lambda 일회값: (lambda 약속값: (lambda 도전값: [약속값, (일회값 - 비밀값 * 도전값) % zk_order])(zk_nizk_challenge(zk_public(비밀값), 약속값, 문맥값)))(zk_commitment(일회값)))(zk_nonce()); ",\n    "zk_nizk_verify = 영지식비대화검증 = lambda 공개값, 증명값, 문맥값: (isinstance(증명값, (list, tuple)) and len(증명값) == 2 and zk_verify(공개값, 증명값[0], zk_nizk_challenge(공개값, 증명값[0], 문맥값), 증명값[1])); ",\n    "zero_knowledge_version = 영지식버전 = ",\n''',
)

# Sentence forms lower to the bundled helpers, so one import line owns the
# transcript encoding and domain separation logic.
replace_once(
    "crates/nme-core/src/lower.rs",
    "        ZeroKnowledgeValue::SimulatedResponse => format!(\n",
    '''        ZeroKnowledgeValue::NizkChallenge {\n            public_key,\n            commitment,\n            context,\n        } => format!(\n            "zk_nizk_challenge({}, {}, {})",\n            lower_code(public_key, source),\n            lower_code(commitment, source),\n            lower_code(context, source)\n        ),\n        ZeroKnowledgeValue::NizkProof { secret, context } => format!(\n            "zk_nizk_prove({}, {})",\n            lower_code(secret, source),\n            lower_code(context, source)\n        ),\n        ZeroKnowledgeValue::NizkVerify {\n            public_key,\n            proof,\n            context,\n        } => format!(\n            "zk_nizk_verify({}, {}, {})",\n            lower_code(public_key, source),\n            lower_code(proof, source),\n            lower_code(context, source)\n        ),\n        ZeroKnowledgeValue::SimulatedResponse => format!(\n''',
)

# Parser/lowering regression test for the punctuation-free Korean surface.
parser_test_anchor = """    #[test]\n    fn zero_knowledge_sentence_values_lower_without_python_punctuation() {\n"""
parser_test = '''    #[test]\n    fn zero_knowledge_nizk_sentences_bind_an_explicit_context() {\n        let source = "영지식 사용 최신\n비밀값은 영지식 비밀 만들기\n공개값은 비밀값으로 영지식 공개값 만들기\n문맥값은 결제 승인 요청\n증명값은 비밀값과 문맥값으로 영지식 비대화 증명 만들기\n검증값은 공개값과 증명값과 문맥값으로 영지식 비대화 검증\n일회값은 영지식 일회값 만들기\n약속값은 일회값으로 영지식 약속 만들기\n도전값은 공개값과 약속값과 문맥값으로 영지식 비대화 도전 만들기\n";\n        let python = transpile(source).expect("context-bound NIZK sentences must transpile");\n        assert!(python.contains("증명값 = zk_nizk_prove(비밀값, 문맥값)"), "{python}");\n        assert!(\n            python.contains("검증값 = zk_nizk_verify(공개값, 증명값, 문맥값)"),\n            "{python}"\n        );\n        assert!(\n            python.contains("도전값 = zk_nizk_challenge(공개값, 약속값, 문맥값)"),\n            "{python}"\n        );\n    }\n\n'''
replace_once("crates/nme-core/src/parser.rs", parser_test_anchor, parser_test + parser_test_anchor)

# New executable examples.
Path("examples/zk-nizk-context.ko.nme").write_text(
    """# 문맥에 묶인 슈노르 비대화형 영지식 증명\n# Fiat-Shamir 도전은 공개값, 약속값, 문맥을 SHA-256으로 함께 묶습니다.\n# 같은 문맥 안의 재전송 자체를 막는 기능은 아니므로 실제 프로토콜에서는 문맥에 요청 식별자나 고유 nonce도 넣으세요.\n\n영지식 사용 최신\n\n비밀값은 영지식 비밀 만들기\n공개값은 비밀값으로 영지식 공개값 만들기\n문맥값은 결제 승인 요청 칠번\n증명값은 비밀값과 문맥값으로 영지식 비대화 증명 만들기\n정상검증은 공개값과 증명값과 문맥값으로 영지식 비대화 검증\n정상검증이 참이면 문맥에 묶인 비대화 영지식 증명을 검증했습니다 말해줘\n\n다른문맥값은 관리자 권한 위임 요청 팔번\n재사용검증은 공개값과 증명값과 다른문맥값으로 영지식 비대화 검증\n재사용검증이 거짓이면 같은 증명은 다른 문맥으로 재사용할 수 없습니다 말해줘\n""",
    encoding="utf-8",
)
Path("examples/zk-nizk-context.en.nme").write_text(
    """# Context-bound non-interactive Schnorr proof.\n# The Fiat-Shamir challenge hashes the public key, commitment, and explicit context.\n# For same-context replay protection, include a unique request ID or nonce in the context.\n\nuse zero_knowledge latest\n\nsecret = zk_secret()\npublic_key = zk_public(secret)\ncontext = \"payment approval request 7\"\nproof = zk_nizk_prove(secret, context)\nvalid = zk_nizk_verify(public_key, proof, context)\nif valid: print(\"Context-bound non-interactive proof verified.\")\n\nother_context = \"administrator delegation request 8\"\nreused = zk_nizk_verify(public_key, proof, other_context)\nif not reused: print(\"The same proof was rejected under a different context.\")\n""",
    encoding="utf-8",
)

# CLI integration coverage. Also update the bundled module version shown by `nme modules`.
replace_all("crates/nme-cli/tests/cli.rs", "zero_knowledge  0.0.1", "zero_knowledge  0.0.2", 1)
replace_all("crates/nme-cli/tests/cli.rs", "영지식  0.0.1", "영지식  0.0.2", 1)
cli_anchor = """#[test]\nfn schnorr_zero_knowledge_examples_run_end_to_end() {\n"""
cli_test = '''#[test]\nfn schnorr_nizk_context_examples_reject_cross_context_reuse() {\n    if !python_available() {\n        eprintln!("Python not available; skipping context-bound NIZK example test");\n        return;\n    }\n\n    let korean = nme(&["run", &example("zk-nizk-context.ko.nme")]);\n    assert!(korean.status.success(), "{}", stderr(&korean));\n    let korean_text = stdout(&korean);\n    assert!(\n        korean_text.contains("문맥에 묶인 비대화 영지식 증명을 검증했습니다"),\n        "{korean_text}"\n    );\n    assert!(\n        korean_text.contains("같은 증명은 다른 문맥으로 재사용할 수 없습니다"),\n        "{korean_text}"\n    );\n\n    let english = nme(&["run", &example("zk-nizk-context.en.nme")]);\n    assert!(english.status.success(), "{}", stderr(&english));\n    let english_text = stdout(&english);\n    assert!(\n        english_text.contains("Context-bound non-interactive proof verified."),\n        "{english_text}"\n    );\n    assert!(\n        english_text.contains("The same proof was rejected under a different context."),\n        "{english_text}"\n    );\n}\n\n'''
replace_once("crates/nme-cli/tests/cli.rs", cli_anchor, cli_test + cli_anchor)

# README documentation: keep the old interactive flow and add the NIZK flow beside it.
replace_once(
    "README.md",
    "`nme modules` to see installed module versions.\n",
    """`nme modules` to see installed module versions.\n\nThe zero-knowledge adapter is version `0.0.2` in beta.18 and also provides a\ncontext-bound Fiat-Shamir non-interactive proof. `zk_nizk_prove(secret, context)`\nreturns a JSON-friendly `[commitment, response]` proof, and\n`zk_nizk_verify(public_key, proof, context)` recomputes the SHA-256 challenge\nfrom the Group 15 generator, commitment, public key, and a length-prefixed UTF-8\ncontext under an NME-specific domain tag. A proof therefore fails under a\ndifferent context. This does not by itself stop replay in the *same* context;\nput a unique request ID or nonce in the context when freshness matters. See\n[`zk-nizk-context.ko.nme`](examples/zk-nizk-context.ko.nme) and its English twin\n[`zk-nizk-context.en.nme`](examples/zk-nizk-context.en.nme).\n""",
)
replace_once(
    "README.ko.md",
    "`nme 모듈`로 설치된 버전과 이름을 확인합니다.\n",
    """`nme 모듈`로 설치된 버전과 이름을 확인합니다.\n\nbeta.18의 영지식 어댑터 버전은 `0.0.2`이며 문맥에 묶인 Fiat-Shamir\n비대화형 증명도 제공합니다. `zk_nizk_prove` / `영지식비대화증명`은\nJSON에 저장하기 쉬운 `[약속값, 응답값]` 증명을 만들고, 검증은 Group 15\n생성원·약속·공개값·길이 구분된 UTF-8 문맥을 NME 전용 도메인 태그 아래\nSHA-256으로 다시 해시합니다. 따라서 다른 문맥으로 복사한 증명은 실패합니다.\n단, 같은 문맥 안의 재전송까지 자동으로 막는 것은 아니므로 freshness가 필요하면\n문맥에 고유 요청 ID나 nonce를 넣으세요. 한국어 문장형 예제\n[`zk-nizk-context.ko.nme`](examples/zk-nizk-context.ko.nme)와 영어판\n[`zk-nizk-context.en.nme`](examples/zk-nizk-context.en.nme)을 참고하세요.\n""",
)

# Language docs with exact Korean sentence spellings.
Path("docs/zero-knowledge-nizk.md").write_text(
    """# Context-bound Schnorr NIZK\n\nNME's bundled `zero_knowledge` adapter version 0.0.2 keeps the interactive Schnorr tools and adds a Fiat-Shamir non-interactive proof.\n\n```text\nuse zero_knowledge latest\nproof = zk_nizk_prove(secret, context)\nvalid = zk_nizk_verify(public_key, proof, context)\n```\n\nThe proof is `[commitment, response]`, so it can be serialized with ordinary JSON. The deterministic challenge is SHA-256 over an NME domain tag, the Group 15 generator, the commitment, the public key, and a length-prefixed UTF-8 context. The transcript order follows the Schnorr Fiat-Shamir construction (`g`, commitment `V`, public value `A`) and the explicit context plays the role of protocol/user binding information.\n\nChanging the context changes the challenge and makes the same proof fail. This is context binding, not a complete anti-replay protocol: if the same context is accepted twice, the same valid proof is still valid twice. Include a unique request ID, session nonce, transaction identifier, or equivalent freshness value in the context when replay resistance is required.\n\nKorean sentence syntax is available without punctuation:\n\n```text\n증명값은 비밀값과 문맥값으로 영지식 비대화 증명 만들기\n검증값은 공개값과 증명값과 문맥값으로 영지식 비대화 검증\n도전값은 공개값과 약속값과 문맥값으로 영지식 비대화 도전 만들기\n```\n\nThis remains learning/reference cryptography rather than a side-channel-hardened audited production implementation.\n""",
    encoding="utf-8",
)
Path("docs/zero-knowledge-nizk.ko.md").write_text(
    """# 문맥에 묶인 슈노르 비대화형 영지식 증명\n\nNME 내장 `zero_knowledge` / `영지식` 어댑터 0.0.2는 기존 대화형 슈노르 도구를 유지하면서 Fiat-Shamir 비대화형 증명을 추가합니다.\n\n```text\n영지식 사용 최신\n증명값은 비밀값과 문맥값으로 영지식 비대화 증명 만들기\n검증값은 공개값과 증명값과 문맥값으로 영지식 비대화 검증\n```\n\n증명은 `[약속값, 응답값]`이라 일반 JSON으로 저장할 수 있습니다. 결정적 도전은 NME 전용 도메인 태그, Group 15 생성원, 약속값, 공개값, 길이를 붙인 UTF-8 문맥을 SHA-256으로 해시해 만듭니다. 슈노르 Fiat-Shamir의 전사록 순서(`g`, 약속 `V`, 공개값 `A`)를 따르며 명시적 문맥은 프로토콜·사용자·행동을 묶는 정보 역할을 합니다.\n\n문맥이 달라지면 도전도 달라져 같은 증명은 실패합니다. 하지만 이것만으로 같은 문맥 안의 재전송까지 막지는 않습니다. 재전송 방지가 필요하면 요청 ID, 세션 nonce, 거래 식별자처럼 매번 달라지는 freshness 값을 문맥에 포함하세요.\n\n도전 자체도 문장형으로 만들 수 있습니다.\n\n```text\n도전값은 공개값과 약속값과 문맥값으로 영지식 비대화 도전 만들기\n```\n\n이 구현은 학습·기준 구현이며 부채널 방어까지 감사된 실서비스 암호 라이브러리를 대체하지 않습니다.\n""",
    encoding="utf-8",
)

# Changelog and release note.
for path, marker, entry in [
    (
        "CHANGELOG.md",
        "## Unreleased\n",
        """\n## 0.0.1-beta.18 — 2026-08-12\n\n- Extend the bundled Schnorr adapter to version `0.0.2` with context-bound Fiat-Shamir non-interactive proofs. The SHA-256 challenge binds the Group 15 generator, commitment, public key, and a length-prefixed explicit context under an NME domain tag.\n- Add `zk_nizk_challenge`, `zk_nizk_prove`, and `zk_nizk_verify` plus Korean sentence forms. Proofs are JSON-friendly `[commitment, response]` values and cross-context reuse is rejected.\n- Add Korean/English executable examples, parser/lowering and CLI end-to-end coverage, and explicit documentation that context binding does not replace same-context freshness/replay controls.\n""",
    ),
    (
        "CHANGELOG.ko.md",
        "## 미출시 (Unreleased)\n",
        """\n## 0.0.1-beta.18 — 2026-08-12\n\n- 내장 슈노르 어댑터를 `0.0.2`로 올리고 문맥에 묶인 Fiat-Shamir 비대화형 증명을 추가했습니다. SHA-256 도전은 NME 전용 도메인 태그 아래 Group 15 생성원, 약속값, 공개값, 길이 구분된 명시적 문맥을 함께 묶습니다.\n- `zk_nizk_challenge`, `zk_nizk_prove`, `zk_nizk_verify`와 한국어 문장형을 추가했습니다. 증명은 JSON 친화적인 `[약속값, 응답값]`이며 다른 문맥으로 재사용하면 검증이 실패합니다.\n- 한국어/영어 실행 예제, 파서·lowering·CLI 종단간 테스트를 추가하고 문맥 결박만으로 같은 문맥 안의 freshness/replay 문제가 해결되는 것은 아니라는 경계를 문서화했습니다.\n""",
    ),
]:
    file = Path(path)
    text = file.read_text(encoding="utf-8")
    assert marker in text and "## 0.0.1-beta.18" not in text
    file.write_text(text.replace(marker, marker + entry, 1), encoding="utf-8")

Path("docs/release-beta.18.md").write_text(
    """# NME 0.0.1-beta.18\n\nBeta.18 adds context-bound Schnorr Fiat-Shamir non-interactive proofs to the built-in zero-knowledge adapter while retaining the beta.17 next-generation release gates.\n\n## API\n\n- `zk_nizk_challenge(public_key, commitment, context)` / `영지식비대화도전`\n- `zk_nizk_prove(secret, context)` / `영지식비대화증명`\n- `zk_nizk_verify(public_key, proof, context)` / `영지식비대화검증`\n\nThe proof is a JSON-friendly two-element list `[commitment, response]`. The challenge uses SHA-256 with an NME-specific Group 15 NIZK domain tag and length-prefixed context. The same proof verifies under its original context and is rejected under a different context.\n\nContext binding is not same-context freshness. Protocols that need replay resistance must include a unique request/session/transaction value in the context.\n\nThe adapter remains a mathematically faithful learning/reference implementation, not a side-channel-hardened audited production cryptography library.\n""",
    encoding="utf-8",
)

print("materialized", NEW)
