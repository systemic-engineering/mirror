//! Smoke tests for the Rust bootstrap's OID stability.
//!
//! These exercise the small constructs that mattered during the C → Rust
//! port — they catch byte-level drift in tokenization, content_oid, and
//! CoincidenceHash going forward.
//!
//! The EXPECTED hashes are the Rust bootstrap's own canonical outputs as
//! of 2026-05-19. The earlier C binary produced different OIDs for some
//! of these forms; the divergence was accepted because the Rust AST
//! captures more structure (keyword, body, sigils). When OID generation
//! moves into mirror itself as @hash/coincidence (Cluster C), these
//! values become the canonical reference.

use std::process::Command;

fn run_compile(file: &str) -> String {
    let exe = env!("CARGO_BIN_EXE_mirror");
    // The binary loads grammars via paths relative to its CWD (e.g.
    // "boot/std/mirror/grammar.mirror"). Run it from the repo root so
    // those paths resolve. CARGO_MANIFEST_DIR is bootstrap/; its parent
    // is the repo root.
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir)
        .parent()
        .expect("bootstrap manifest must have a parent");
    let out = Command::new(exe)
        .current_dir(repo_root)
        .args(["compile", "--no-cache", file])
        .output()
        .expect("binary did not run");
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
        "d3e99e14db24fcc8238c1b93ea54f49a4324ff869d0e002126a2d310c30aca7d"
    );
}

#[test]
fn in_prism_oid() {
    let path = std::env::temp_dir().join("mirror_oid_test_in_prism.mirror");
    std::fs::write(&path, "in @prism\n").unwrap();
    let oid = run_compile(path.to_str().unwrap());
    assert_eq!(
        oid,
        "9836a8ba693f236e974673addbf5c5fd73f922b8d827532676e2ab6541d6c6a2"
    );
}
