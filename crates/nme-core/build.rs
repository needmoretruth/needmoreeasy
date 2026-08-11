use std::fs;
use std::path::PathBuf;

fn replace_once(text: &mut String, old: &str, new: &str) {
    assert_eq!(text.matches(old).count(), 1, "expected one exact beta.16 repair target: {old}");
    *text = text.replacen(old, new, 1);
}

fn main() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root");

    // Module spelling is syntax, not a runtime binding. Remembering it as a
    // binding makes ordinary prose such as `영지식 증명` interpolate a name
    // that does not exist in generated Python.
    let parser = root.join("crates/nme-core/src/parser.rs");
    let mut text = fs::read_to_string(&parser).expect("read parser");
    let old = "        BundledModuleId::ZeroKnowledge => &[\n            ZERO_KNOWLEDGE_MODULE,\n            ZERO_KNOWLEDGE_MODULE_KO,\n            \"영지식비밀난수\",";
    let new = "        BundledModuleId::ZeroKnowledge => &[\n            \"영지식비밀난수\",";
    replace_once(&mut text, old, new);
    fs::write(&parser, text).expect("write parser");

    // Keep the Korean program sentence-oriented and Korean-only, while making
    // explanatory output unambiguously literal rather than eligible for name
    // interpolation. Quotes are normal sentence punctuation in NME values.
    let korean = root.join("examples/zk-schnorr-relay.ko.nme");
    let mut text = fs::read_to_string(&korean).expect("read Korean ZK example");
    for (old, new) in [
        (
            "정상검증이 참이면 송신자 에이의 영지식 증명을 수신자 비가 받아들였습니다 말해줘",
            "정상검증이 참이면 \"송신자 에이의 영지식 증명을 수신자 비가 받아들였습니다\" 말해줘",
        ),
        (
            "재전송검증이 거짓이면 악성 중계자 씨의 저장 전사록 재전송은 새 도전에서 실패했습니다 말해줘",
            "재전송검증이 거짓이면 \"악성 중계자 씨의 저장 전사록 재전송은 새 도전에서 실패했습니다\" 말해줘",
        ),
        (
            "모의검증이 참이면 비밀값 없이도 미리 고른 도전에 맞는 전사록을 모의할 수 있습니다 말해줘",
            "모의검증이 참이면 \"비밀값 없이도 미리 고른 도전에 맞는 전사록을 모의할 수 있습니다\" 말해줘",
        ),
        (
            "모의재사용검증이 거짓이면 모의 전사록은 수신자 비의 다른 도전에는 재사용할 수 없습니다 말해줘",
            "모의재사용검증이 거짓이면 \"모의 전사록은 수신자 비의 다른 도전에는 재사용할 수 없습니다\" 말해줘",
        ),
        (
            "중계검증이 참이면 실시간 중계는 통과하지만 비밀 위조가 아니라 송신자 에이의 실제 응답을 전달한 것입니다 말해줘",
            "중계검증이 참이면 \"실시간 중계는 통과하지만 비밀 위조가 아니라 송신자 에이의 실제 응답을 전달한 것입니다\" 말해줘",
        ),
    ] {
        replace_once(&mut text, old, new);
    }
    fs::write(&korean, text).expect("write Korean ZK example");

    // The English twin stays in NME sentence syntax too. Double-quoted output
    // keeps English possessive apostrophes inside a string token.
    let english = root.join("examples/zk-schnorr-relay.en.nme");
    let mut text = fs::read_to_string(&english).expect("read English ZK example");
    for (old, new) in [
        (
            "if normal then Receiver B accepted sender A's zero-knowledge proof. show",
            "if normal then \"Receiver B accepted sender A's zero-knowledge proof.\" show",
        ),
        (
            "if replay_ok is false then Intermediary C cannot replay the saved transcript against a fresh challenge. show",
            "if replay_ok is false then \"Intermediary C cannot replay the saved transcript against a fresh challenge.\" show",
        ),
        (
            "if sim_ok then A transcript can exist for one chosen challenge without sender A's secret. show",
            "if sim_ok then \"A transcript can exist for one chosen challenge without sender A's secret.\" show",
        ),
        (
            "if sim_reuse_ok is false then The simulated transcript cannot answer a different verifier challenge. show",
            "if sim_reuse_ok is false then \"The simulated transcript cannot answer a different verifier challenge.\" show",
        ),
        (
            "if relay_ok then Live relay passes because intermediary C forwarded sender A's real response. show",
            "if relay_ok then \"Live relay passes because intermediary C forwarded sender A's real response.\" show",
        ),
    ] {
        replace_once(&mut text, old, new);
    }
    fs::write(&english, text).expect("write English ZK example");

    // One-shot release-preparation helper only. The validated candidate must
    // not ship a build script that mutates source files.
    fs::remove_file(manifest.join("build.rs")).expect("remove one-shot build helper");
}
