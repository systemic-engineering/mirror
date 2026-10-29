//! The ruler crate's dependency edges (audit §2.1, §2.4, §3 item 3):
//! `engine` and `blake3`, nothing else. No serde, no crypto, no keys.
//! Reads the manifest as text: no TOML parser.

const MANIFEST: &str = include_str!("../Cargo.toml");

fn code(line: &str) -> &str {
    line.split('#').next().unwrap().trim()
}

#[test]
fn engine_and_blake3_are_the_only_dependencies() {
    let mut section = "";
    let mut deps = Vec::new();
    for line in MANIFEST.lines() {
        let c = code(line);
        if c.starts_with('[') {
            section = c;
            continue;
        }
        if section.contains("dependencies") && !c.is_empty() {
            deps.push((section, c.split('=').next().unwrap().trim()));
        }
    }
    assert_eq!(deps, vec![("[dependencies]", "engine"), ("[dependencies]", "blake3")]);
}

#[test]
fn no_serde_and_no_crypto_names_in_the_manifest() {
    for line in MANIFEST.lines() {
        let c = code(line);
        for banned in ["serde", "ed25519", "ssh-key", "ring", "rsa", "p256", "k256", "x25519"] {
            assert!(!c.contains(banned), "{banned} in rust/ruler/Cargo.toml: {line}");
        }
    }
}

#[test]
fn the_source_holds_no_key_material_and_no_process() {
    let src = include_str!("../src/lib.rs");
    for banned in ["PRIVATE KEY", "ssh-keygen", "std::process", "std::fs", "std::net", "Approved", "fn verify", "serde"] {
        for (i, line) in src.lines().enumerate() {
            if line.trim_start().starts_with("//") {
                continue; // prose may name what is absent
            }
            assert!(!line.contains(banned), "rust/ruler/src/lib.rs:{}: {banned}", i + 1);
        }
    }
}
