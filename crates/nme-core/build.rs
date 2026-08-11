use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root");

    for relative in [
        "examples/zk-schnorr-relay.ko.nme",
        "examples/zk-schnorr-relay.en.nme",
    ] {
        let path = root.join(relative);
        if path.exists() {
            let text = fs::read_to_string(&path).expect("read generated ZK example");
            for (number, line) in text.lines().enumerate() {
                println!("cargo:warning=ZK-SOURCE {relative}:{}: {line}", number + 1);
            }
        }
    }

    panic!("diagnostic-only beta.16 run: inspect generated ZK examples above");
}
