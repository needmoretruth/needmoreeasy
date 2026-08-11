use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root");

    for (relative, start, end) in [
        ("crates/nme-core/src/parser.rs", 3965usize, 4015usize),
        ("crates/nme-core/src/lower.rs", 40usize, 85usize),
    ] {
        let path = root.join(relative);
        let text = fs::read_to_string(&path).expect("read generated ZK source");
        for (number, line) in text.lines().enumerate() {
            let one_based = number + 1;
            if one_based >= start && one_based <= end {
                println!("cargo:warning=ZK-RANGE {relative}:{one_based}: {line}");
            }
        }
    }

    panic!("diagnostic-only beta.16 run: inspect generated ZK binding ranges above");
}
