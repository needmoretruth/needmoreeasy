use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root");

    for relative in [
        "crates/nme-core/src/syntax.rs",
        "crates/nme-core/src/parser.rs",
        "crates/nme-core/src/lower.rs",
    ] {
        let path = root.join(relative);
        let text = fs::read_to_string(&path).expect("read generated ZK source");
        for (number, line) in text.lines().enumerate() {
            if line.contains("ZeroKnowledge")
                || line.contains("ZERO_KNOWLEDGE")
                || line.contains("전사록")
                || line.contains("\"영지식\"")
            {
                println!("cargo:warning=ZK-RUST {relative}:{}: {line}", number + 1);
            }
        }
    }

    panic!("diagnostic-only beta.16 run: inspect generated ZK bindings above");
}
