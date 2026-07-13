//! Rung 7' RED — `mirror peer contribute <peer_home> --target <shard>`
//! discharged via Fate::bounded with 4-subtree tree shape per Mara `2c64060`
//! (§7 correction of Rung 7's four errors under @fractal-as-substrate).
//!
//! ## Substrate authority
//!
//! - Mara `2c64060` — `docs/specs/fractal-family-root-mandelbrot-substrate.md`
//!   §7 four-error correction. @fractal underlies @kintsugi/consent per
//!   Alex 2026-07-13 outside-view correction.
//! - Mara `3ffa8ed` — `docs/math/2026-07-13-fractal-mandelbrot-substrate.md`
//!   Mandelbrot identification; sheaf-Laplacian Bodnar 2022; Julia ↔ M
//!   correspondence ⇒ coordination-without-signal.
//! - Taut `b52b008` — `docs/scouts/2026-07-13-taut-fractal-underlies-consent-
//!   coherence-empirical-scout.md`. Fiedler 0.0612 = 6% H¹(F) obstruction.
//! - `fragmentation::Fractal<E, H>` at `/Users/alexwolf/dev/projects/
//!   fragmentation/src/fragment.rs:88` — substrate primitive at Rust
//!   altitude (Shard / Branch / Lens).
//! - `fate_bounded_by_psychohistory_peer_beam` in `bootstrap/src/lib.rs`
//!   — the composed-idiom pattern for `Fate::bounded` (Fate::untrained() +
//!   selectors_from_psychohistory_root).
//! - Recognition #43 (mirror IS content-addressed build system) — EXTENDS to
//!   mirror IS content-addressed AI-inference substrate via Rung 7 empirical
//!   discharge and Rung 7' Fate-bounded correction.
//! - Recognition #55 (form/process partition) — commit_as_fold IS the
//!   renormalization operator (Mara math §4).
//! - Recognitions #80/#107 — M∘ / ∂M interior/boundary decomposition
//!   (Mara math §2.3).
//! - Alex 2026-07-13 in-transcript: "With @fractal the compiler becomes
//!   basically a Mandlebrot set."
//!
//! ## The four errors Rung 7' corrects
//!
//! 1. **Fate::excited → Fate::bounded** (sheaf-mathematics grounding).
//! 2. **Jurisdictional separation of tree** (5-flat-blob → 4-subtree per
//!    Asher tripartition: `anchors/` + `gates/` + `witnesses/` +
//!    `morphism-body`).
//! 4. **Witness-in-encoding not witness-in-content** — fate metadata
//!    (fate_model + peer_uuid + psychohistory_root_oid) folds into
//!    commit's naked_oid via commit message (git plumbing already does
//!    this at commit-tree altitude); NOT as a blob in the content tree.
//!    Preserves "same content, different witness, different commit,
//!    same tree OID" discipline.
//!
//! Error 3 (direction inversion / dark_pass) forward-promised to Rung 7.5.
//! Scope A' = Errors 1 + 2 + 4.
//!
//! ## RED contract (7 T-tests)
//!
//! T1 — `mirror peer contribute` accepts inputs; exit 0 on settle green.
//! T2 — Envelope names `fate_source: bounded (psychohistory-derived)` and
//!      emits `psychohistory_root_oid` (Fate::bounded discharge witness).
//! T3 — Commit tree contains 4 top-level entries: `anchors/`, `gates/`,
//!      `witnesses/`, `morphism-body`. NO top-level `fate-witness`,
//!      `pre-anchor`, `post-anchor`, `settle-verdict` (those moved into
//!      the tripartition subtrees).
//! T4 — `anchors/` subtree contains `pre` and `post` blobs.
//! T5 — `gates/` subtree contains `settle-verdict` blob.
//! T6 — Commit MESSAGE (naked_oid input) contains `fate_model:`,
//!      `fate_prism_op:`, `psychohistory_root_oid:` — witness lives in
//!      encoding, not content.
//! T7 — Rung 6.2a parent chain preserved: second invocation's commit
//!      has parent pointer to first commit.

use std::path::PathBuf;
use std::process::Command;

fn mirror_bin() -> String {
    std::env::var("MIRROR_BIN")
        .unwrap_or_else(|_| "/Users/reed/.cargo-target/release/mirror".to_string())
}

fn make_contribute_fixture(suffix: &str) -> PathBuf {
    let base = std::env::temp_dir().join(format!(
        "peer-contribute-tri-{}-{}",
        suffix,
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).expect("create peer_home");
    std::fs::create_dir_all(base.join("src")).expect("create src");
    std::fs::write(
        base.join("Cargo.toml"),
        "[package]\nname = \"peer-fixture-tri\"\nversion = \"0.0.0\"\nedition = \"2021\"\n",
    )
    .expect("write Cargo.toml");
    std::fs::write(base.join("src/lib.rs"), "// peer fixture; intentionally empty\n")
        .expect("write src/lib.rs");
    std::fs::write(
        base.join("mirror.spec"),
        "target binary {\n  cli {\n    command beam { arg mission: ~f }\n  }\n}\n",
    )
    .expect("write mirror.spec");
    base
}

fn write_target_shard(dir: &std::path::Path, name: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(
        &path,
        "# @peer/fixture — minimum-viable target shard for peer_contribute (tripartition).\n#\n# Rung 7' morphism target: peer will append docstring line per Mara `2c64060`\n# §7 Scope A' Fate::bounded discharge.\n\nprism @peer/fixture {\n  focus fixture\n  project fixture\n  split fixture\n  shift fixture\n  settle fixture\n}\n\nout @peer/fixture\n",
    )
    .expect("write target shard");
    path
}

fn run_contribute(
    peer_home: &PathBuf,
    target: &PathBuf,
) -> std::process::Output {
    Command::new(mirror_bin())
        .arg("peer")
        .arg("contribute")
        .arg(peer_home)
        .arg("--target")
        .arg(target)
        .output()
        .expect("execute mirror peer contribute")
}

fn extract_field(stdout: &str, field: &str) -> Option<String> {
    stdout
        .lines()
        .find(|l| l.starts_with(&format!("+ {}: ", field)))
        .and_then(|l| l.split(&format!("+ {}: ", field)).nth(1))
        .map(|s| s.trim().to_string())
}

// === T1: dispatch acceptance; exit 0 on settle green ==================
#[test]
fn t01_peer_contribute_tripartition_exit_zero() {
    let dir = make_contribute_fixture("t1");
    let target = write_target_shard(&dir, "fixture.mirror");
    let out = run_contribute(&dir, &target);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(
        out.status.success(),
        "T1: `mirror peer contribute` must exit 0 for valid inputs (settle \
         green); stdout=<{stdout}> stderr=<{stderr}>"
    );
}

// === T2: envelope emits Fate::bounded discharge witness ================
#[test]
fn t02_envelope_names_fate_bounded_and_psychohistory_root() {
    let dir = make_contribute_fixture("t2");
    let target = write_target_shard(&dir, "fixture.mirror");
    let out = run_contribute(&dir, &target);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        stdout.contains("bounded"),
        "T2: envelope must name Fate::bounded discharge (per Mara `2c64060` \
         §7.1 Error 1 correction; retire Fate::excited); got: <{stdout}>"
    );
    let root = extract_field(&stdout, "psychohistory_root_oid");
    assert!(
        root.is_some(),
        "T2: envelope must emit `psychohistory_root_oid` field (sheaf \
         Rayleigh descent witness); got: <{stdout}>"
    );
    let root_str = root.unwrap();
    assert_eq!(
        root_str.len(),
        64,
        "T2: psychohistory_root_oid must be 64-hex canonical_hash / blake3; \
         got: <{root_str}>"
    );
}

// === T3: top-level tree has 4 tripartition entries =====================
#[test]
fn t03_top_tree_has_tripartition_shape() {
    let dir = make_contribute_fixture("t3");
    let target = write_target_shard(&dir, "fixture.mirror");
    let out = run_contribute(&dir, &target);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let ref_name = extract_field(&stdout, "ref_name").expect("ref_name field");
    let ls_out = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("ls-tree")
        .arg(&ref_name)
        .output()
        .expect("git ls-tree");
    let ls = String::from_utf8_lossy(&ls_out.stdout).to_string();

    // Positive: 4 tripartition entries present.
    for expected in &["anchors", "gates", "witnesses", "morphism-body"] {
        assert!(
            ls.contains(expected),
            "T3: top-level tree must contain `{expected}` (Mara `2c64060` \
             §7.2 tripartition subtrees); got: <{ls}>"
        );
    }

    // Negative: old flat-5-blob entries MUST NOT appear at top level.
    // (`fate-witness`, `pre-anchor`, `post-anchor`, `settle-verdict`
    // move into subtrees under Rung 7' correction).
    for banned in &["pre-anchor", "post-anchor", "settle-verdict", "fate-witness"] {
        assert!(
            !ls.lines().any(|l| l.ends_with(&format!("\t{}", banned))),
            "T3: top-level tree must NOT contain `{banned}` (moved to \
             tripartition subtree per Rung 7' correction; Rung 7 \
             jurisdictional violation was the whole point); got: <{ls}>"
        );
    }
}

// === T4: anchors/ subtree contains pre + post blobs ====================
#[test]
fn t04_anchors_subtree_contains_pre_and_post() {
    let dir = make_contribute_fixture("t4");
    let target = write_target_shard(&dir, "fixture.mirror");
    let out = run_contribute(&dir, &target);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let ref_name = extract_field(&stdout, "ref_name").expect("ref_name field");
    let ls_out = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("ls-tree")
        .arg(format!("{}:anchors", ref_name))
        .output()
        .expect("git ls-tree anchors/");
    let ls = String::from_utf8_lossy(&ls_out.stdout).to_string();
    for expected in &["pre", "post"] {
        assert!(
            ls.contains(expected),
            "T4: anchors/ subtree must contain `{expected}` blob (base \
             fabric per Asher p.14 \"Base Fabric preserves raw, unresolved \
             and typed-unknown states\"); got: <{ls}>"
        );
    }
}

// === T5: gates/ subtree contains settle-verdict blob ===================
#[test]
fn t05_gates_subtree_contains_settle_verdict() {
    let dir = make_contribute_fixture("t5");
    let target = write_target_shard(&dir, "fixture.mirror");
    let out = run_contribute(&dir, &target);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let ref_name = extract_field(&stdout, "ref_name").expect("ref_name field");
    let ls_out = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("ls-tree")
        .arg(format!("{}:gates", ref_name))
        .output()
        .expect("git ls-tree gates/");
    let ls = String::from_utf8_lossy(&ls_out.stdout).to_string();
    assert!(
        ls.contains("settle-verdict"),
        "T5: gates/ subtree must contain `settle-verdict` blob \
         (constitutional gate per Asher p.10; NOT evidential witness); \
         got: <{ls}>"
    );
}

// === T6: commit MESSAGE carries witness (naked_oid via git plumbing) ==
#[test]
fn t06_commit_message_carries_witness_in_encoding() {
    let dir = make_contribute_fixture("t6");
    let target = write_target_shard(&dir, "fixture.mirror");
    let out = run_contribute(&dir, &target);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let ref_name = extract_field(&stdout, "ref_name").expect("ref_name field");
    let show_out = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("log")
        .arg("-1")
        .arg("--format=%B")
        .arg(&ref_name)
        .output()
        .expect("git log commit message");
    let msg = String::from_utf8_lossy(&show_out.stdout).to_string();
    for expected in &["fate_model:", "fate_prism_op:", "psychohistory_root_oid:"] {
        assert!(
            msg.contains(expected),
            "T6: commit message must contain `{expected}` (witness folds \
             into naked_oid via git commit metadata; Mara `2c64060` \
             §7.4 Error 4 correction: witness-in-encoding not witness-\
             in-content); got: <{msg}>"
        );
    }
}

// === T7: Rung 6.2a parent chain preserved ==============================
#[test]
fn t07_second_invocation_chains_from_parent() {
    let dir = make_contribute_fixture("t7");
    let target = write_target_shard(&dir, "fixture.mirror");
    let out1 = run_contribute(&dir, &target);
    let stdout1 = String::from_utf8_lossy(&out1.stdout).to_string();
    let ref_name = extract_field(&stdout1, "ref_name").expect("ref_name field");

    let commit1 = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("rev-parse")
        .arg(&ref_name)
        .output()
        .expect("rev-parse ref");
    let commit1_sha = String::from_utf8_lossy(&commit1.stdout).trim().to_string();
    assert!(!commit1_sha.is_empty(), "T7: first commit must exist");

    // Second invocation (target already has first morphism appended)
    let out2 = run_contribute(&dir, &target);
    assert!(
        out2.status.success(),
        "T7: second invocation must succeed for parent chain; stderr=<{}>",
        String::from_utf8_lossy(&out2.stderr)
    );
    let commit2 = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("rev-parse")
        .arg(&ref_name)
        .output()
        .expect("rev-parse ref 2");
    let commit2_sha = String::from_utf8_lossy(&commit2.stdout).trim().to_string();
    assert_ne!(
        commit1_sha, commit2_sha,
        "T7: second commit must differ from first (parent pointer changes \
         commit content-hash); got same SHA <{commit1_sha}>"
    );
    let parents = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("rev-list")
        .arg("--parents")
        .arg("-n")
        .arg("1")
        .arg(&commit2_sha)
        .output()
        .expect("git rev-list --parents");
    let parents_stdout = String::from_utf8_lossy(&parents.stdout).trim().to_string();
    assert!(
        parents_stdout.contains(&commit1_sha),
        "T7: second commit's parent must be first commit (Rung 6.2a DAG \
         chain preserved under Rung 7' tree reshape); got parents-line: \
         <{parents_stdout}>"
    );
}
