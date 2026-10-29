//! E7 (spec §4.1; audit §3.5): no serde. The crate's only dependency is
//! `blake3`, and nothing named serde appears in its manifest, directly or
//! as a feature. Reads the manifest as text: no TOML parser.

const MANIFEST: &str = include_str!("../Cargo.toml");

#[test]
fn e7_manifest_names_no_serde() {
    for line in MANIFEST.lines() {
        let code = line.split('#').next().unwrap();
        assert!(!code.contains("serde"), "serde in engine/Cargo.toml: {line}");
    }
}

#[test]
fn e7_blake3_is_the_only_dependency() {
    let mut section = "";
    let mut deps = Vec::new();
    for line in MANIFEST.lines() {
        let code = line.split('#').next().unwrap().trim();
        if code.starts_with('[') {
            section = code;
            continue;
        }
        if section.contains("dependencies") && !code.is_empty() {
            deps.push((section, code.split('=').next().unwrap().trim()));
        }
    }
    assert_eq!(deps, vec![("[dependencies]", "blake3")]);
}
