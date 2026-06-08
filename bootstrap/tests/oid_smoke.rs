//! Smoke tests for the Rust bootstrap's OID stability.
//!
//! These exercise the small constructs that mattered during the C → Rust
//! port — they catch byte-level drift in tokenization, content_oid, and
//! CoincidenceHash going forward.
//!
//! POST-CLUSTER-D PINNED VALUES (2026-05-20).
//!
//! The bootstrap now implements CoincidenceHash<5,5> as declared in
//! `boot/std/hash/coincidence.mirror` — 5 dimensions (one per Prism
//! operation) and 5 projections (one per gutter-lens duality). The earlier
//! pre-Cluster-D values were computed under CoincidenceHash<3,16>, the
//! C-era seed; they are gone. The OIDs below are the canonical reference
//! under <5,5>. `@epistemologic/property/coincidence_matches` is the
//! grammar-side mirror of these pins.

/// In-process compile dispatch through `mirror::kintsugi_main`
/// (Taut #286 Win 2). The binary loads grammars via paths relative to its
/// CWD (e.g. "boot/std/mirror/grammar.mirror"); set cwd to repo root.
fn run_compile(file: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir)
        .parent()
        .expect("bootstrap manifest must have a parent");
    let argv: Vec<String> = ["mirror", "compile", "--no-cache", file]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let out = mirror::kintsugi_main_in(&argv, repo_root);
    let mut s = String::from_utf8_lossy(&out.stdout).into_owned();
    while s.ends_with('\n') {
        s.pop();
    }
    s
}

#[test]
fn out_collapse_oid() {
    let path = std::env::temp_dir().join("mirror_oid_test_out_collapse.mirror");
    std::fs::write(&path, "out collapse\n").unwrap();
    let oid = run_compile(path.to_str().unwrap());
    assert_eq!(
        oid,
        "a8312da6335d4471ac0fe1f815421ab00778b45a6be70781324490934c5572b3"
    );
}

#[test]
fn in_prism_oid() {
    let path = std::env::temp_dir().join("mirror_oid_test_in_prism.mirror");
    std::fs::write(&path, "in @prism\n").unwrap();
    let oid = run_compile(path.to_str().unwrap());
    assert_eq!(
        oid,
        "3ba4c79d158b06e0ce9998525dca5d5a61f8da9bd9579ba6039250d33b609a66"
    );
}
