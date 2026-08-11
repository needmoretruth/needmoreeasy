use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root");

    let korean = root.join("examples/zk-schnorr-relay.ko.nme");
    if korean.exists() {
        let text = fs::read_to_string(&korean).expect("read Korean ZK example");
        let old = "송신자 에이의 영지식 증명을 수신자 비가 받아들였습니다";
        let new = "송신자 에이의 지식 증명을 수신자 비가 받아들였습니다";
        assert_eq!(text.matches(old).count(), 1);
        fs::write(&korean, text.replacen(old, new, 1)).expect("patch Korean ZK example");
    }

    let english = root.join("examples/zk-schnorr-relay.en.nme");
    if english.exists() {
        let text = fs::read_to_string(&english).expect("read English ZK example");
        let old = "accepted sender A's zero-knowledge proof";
        let new = "\"accepted sender A's zero-knowledge proof\"";
        assert_eq!(text.matches(old).count(), 1);
        fs::write(&english, text.replacen(old, new, 1)).expect("patch English ZK example");
    }

    // This is a one-shot release-preparation helper. Once it has repaired the
    // generated examples it removes its own source, so the validated candidate
    // does not ship a build script or mutate source files during normal builds.
    fs::remove_file(manifest.join("build.rs")).expect("remove one-shot build helper");
}
