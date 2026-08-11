use std::fs;
use std::path::PathBuf;

fn quote_korean_output(text: &mut String, prefix: &str) {
    let mut found = 0usize;
    let rewritten = text
        .lines()
        .map(|line| {
            if let Some(body) = line.strip_prefix(prefix).and_then(|rest| rest.strip_suffix(" 말해줘")) {
                found += 1;
                format!("{prefix}\"{body}\" 말해줘")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(found, 1, "expected one Korean output line starting with {prefix}");
    *text = rewritten + "\n";
}

fn quote_english_condition_outputs(text: &mut String) {
    let mut found = 0usize;
    let rewritten = text
        .lines()
        .map(|line| {
            if line.starts_with("if ") && line.ends_with(" show") {
                if let Some((condition, body_with_show)) = line.split_once(" then ") {
                    if let Some(body) = body_with_show.strip_suffix(" show") {
                        found += 1;
                        return format!("{condition} then \"{body}\" show");
                    }
                }
            }
            line.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(found, 5, "expected five English conditional output lines");
    *text = rewritten + "\n";
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
    assert_eq!(text.matches(old).count(), 1, "expected exact ZeroKnowledge binding list");
    text = text.replacen(old, new, 1);
    fs::write(&parser, text).expect("write parser");

    // Keep the Korean program sentence-oriented and Korean-only, while making
    // explanatory output unambiguously literal rather than eligible for name
    // interpolation.
    let korean = root.join("examples/zk-schnorr-relay.ko.nme");
    let mut text = fs::read_to_string(&korean).expect("read Korean ZK example");
    for prefix in [
        "정상검증이 참이면 ",
        "재전송검증이 거짓이면 ",
        "모의검증이 참이면 ",
        "모의재사용검증이 거짓이면 ",
        "중계검증이 참이면 ",
    ] {
        quote_korean_output(&mut text, prefix);
    }
    fs::write(&korean, text).expect("write Korean ZK example");

    // The English twin stays in sentence NME. Quote every conditional output
    // body so possessive apostrophes remain inside a string token.
    let english = root.join("examples/zk-schnorr-relay.en.nme");
    let mut text = fs::read_to_string(&english).expect("read English ZK example");
    quote_english_condition_outputs(&mut text);
    fs::write(&english, text).expect("write English ZK example");

    // One-shot release-preparation helper only. The validated candidate must
    // not ship a build script that mutates source files.
    fs::remove_file(manifest.join("build.rs")).expect("remove one-shot build helper");
}
