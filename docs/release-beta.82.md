# NME 0.0.1-beta.82

Beta.82 makes blank package installation requests deterministic and brings the
workspace into the stable formatting expected by CI.

- Reject blank package names before invoking pip, with bilingual E9025 guidance.
- Format the workspace with the Rust formatter used by the CI toolchain.
- Keep the native backend and tests warning-free under the current Clippy gate.

Language semantics and generated Python behavior are unchanged.
