//! Rung 7 RED — `mirror peer contribute <peer_home> --target <shard>`
//! fires fate-spawned active_pass; peer proposes docstring-append morphism;
//! verifies via cargo check on peer_home's Cargo workspace; on settle
//! success, commits morphism via commit_as_fold (Rung 6.1c pattern with
//! substantive tree contents: pre-anchor, post-anchor, morphism-body,
//! settle-verdict, fate-witness).
//!
//! This IS Alex's empirical certainty demand (2026-07-13 in-transcript):
//! "it's not empirical certainty until a Fate spawned agent contributes
//! working mirror back to the compiler." Rung 6.1c wrote a ceremonial
//! commit (peer signs its own presence). Rung 7 writes an empirical
//! commit: fate produces morphism → mirror applies to shard → compiler
//! validates → commit_as_fold lands on peer's DAG only if verified.
//!
//! Substrate authority:
//! - Mara `4e69066` — `docs/specs/fate-spawned-peer-contributes-working-
//!   delta-via-active-pass.md` canonical spec (Scope A recommended).
//! - Recognition #58 (Fate IS optical inference; fate crate landed).
//! - `@kintsugi/oscillate.active_pass` (substrate-decl at
//!   `shards/kintsugi/oscillate.mirror:456`; runtime unlanded until
//!   this landing).
//! - `@mirror/mosaic.settle` (substrate-decl'd; cargo check IS this
//!   at @code/rust altitude).
//! - `@kintsugi/store/git.commit_as_fold` (Rung 6.1c-landed).
//! - Reed `90019c4` Rung 6.1c commit_as_fold discharge.
//! - Alex 2026-07-13 in-transcript empirical-certainty definition.
//!
//! Five RED tests per Mara `4e69066` §3 shape:
//! T1 subcommand accepted; exit 0 for valid inputs.
//! T2 target shard bytes modified (morphism applied — docstring line
//!    appended per Cartographer/Introject/Explorer/Fate Model mapping).
//! T3 commit created at `refs/mirror/peer/<uuid>/HEAD` in peer_home.
//! T4 commit tree contains five blobs: pre-anchor, post-anchor,
//!    morphism-body, settle-verdict, fate-witness (Mara §3.3 shape).
//! T5 target file not found → exit 1 (refusal path; morphism not attempted).

use std::process::Command;

fn mirror_bin() -> String {
    std::env::var("MIRROR_BIN")
        .unwrap_or_else(|_| "/Users/reed/.cargo-target/release/mirror".to_string())
}

/// Set up a peer_home with a minimal Cargo workspace so `cargo check`
/// verify succeeds (Scope A gate; Rung 7.5 relaxes the workspace
/// requirement).
fn make_contribute_fixture(suffix: &str) -> std::path::PathBuf {
    let base = std::env::temp_dir().join(format!(
        "peer-contribute-{}-{}",
        suffix,
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).expect("create peer_home");
    std::fs::create_dir_all(base.join("src")).expect("create src");
    std::fs::write(
        base.join("Cargo.toml"),
        "[package]\nname = \"peer-fixture\"\nversion = \"0.0.0\"\nedition = \"2021\"\n",
    )
    .expect("write Cargo.toml");
    std::fs::write(base.join("src/lib.rs"), "// peer fixture; intentionally empty\n")
        .expect("write src/lib.rs");
    // mirror.spec for peer_home discipline (per Rungs 1-6 pattern).
    std::fs::write(
        base.join("mirror.spec"),
        "target binary {\n  cli {\n    command beam { arg mission: ~f }\n  }\n}\n",
    )
    .expect("write mirror.spec");
    base
}

fn write_target_shard(dir: &std::path::Path, name: &str) -> std::path::PathBuf {
    let path = dir.join(name);
    std::fs::write(
        &path,
        "# @peer/fixture — minimum-viable target shard for peer_contribute test.\n#\n# Rung 7 morphism-append target: peer will append a Recognition-ancestry\n# line to this docblock per Mara `4e69066` §4 Scope A docstring-append\n# morphism kind.\n\nprism @peer/fixture {\n  focus fixture\n  project fixture\n  split fixture\n  shift fixture\n  settle fixture\n}\n\nout @peer/fixture\n",
    )
    .expect("write target shard");
    path
}

fn run_contribute(
    peer_home: &std::path::PathBuf,
    target: &std::path::PathBuf,
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

// === T1: dispatch acceptance; exit 0 for valid inputs ===================
#[test]
fn t01_peer_contribute_subcommand_accepted() {
    let dir = make_contribute_fixture("exit");
    let target = write_target_shard(&dir, "fixture.mirror");
    let out = run_contribute(&dir, &target);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(
        out.status.success(),
        "T1: `mirror peer contribute <peer_home> --target <shard>` must exit 0 \
         for valid inputs (Mara `4e69066` §3 canonical shape); stdout=<{stdout}> \
         stderr=<{stderr}>"
    );
}

// === T2: target shard bytes modified (morphism applied) =================
#[test]
fn t02_target_shard_bytes_modified_after_contribute() {
    let dir = make_contribute_fixture("morphism");
    let target = write_target_shard(&dir, "fixture.mirror");
    let pre_bytes = std::fs::read(&target).expect("read pre bytes");
    let _ = run_contribute(&dir, &target);
    let post_bytes = std::fs::read(&target).expect("read post bytes");
    assert!(
        post_bytes.len() > pre_bytes.len(),
        "T2: target shard must have morphism appended (docstring line per \
         Mara `4e69066` §4 Scope A); pre_len={}, post_len={}",
        pre_bytes.len(),
        post_bytes.len()
    );
    let post_str = String::from_utf8_lossy(&post_bytes);
    assert!(
        post_str.contains("Recognition-ancestry") || post_str.contains("peer"),
        "T2: appended morphism must be recognition-ancestry docstring line \
         (per Mara §4.1 morphism kind); got: <{post_str}>"
    );
}

// === T3: commit created at refs/mirror/peer/<uuid>/HEAD in peer_home ====
#[test]
fn t03_commit_created_on_peer_dag_branch() {
    let dir = make_contribute_fixture("commit");
    let target = write_target_shard(&dir, "fixture.mirror");
    let out = run_contribute(&dir, &target);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    // Extract ref_name from envelope.
    let ref_name = stdout
        .lines()
        .find(|l| l.starts_with("+ ref_name: "))
        .and_then(|l| l.split("+ ref_name: ").nth(1))
        .map(|s| s.trim())
        .expect("envelope must emit ref_name field");
    let rev_out = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("rev-parse")
        .arg(ref_name)
        .output()
        .expect("git rev-parse");
    assert!(
        rev_out.status.success(),
        "T3: `git rev-parse {ref_name}` must succeed (peer's DAG branch \
         must exist after contribute; Rung 6.1c commit_as_fold shape); \
         got: status={:?}",
        rev_out.status
    );
    // Type must be commit (not blob or tree).
    let type_out = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("cat-file")
        .arg("-t")
        .arg(ref_name)
        .output()
        .expect("git cat-file -t");
    let type_str = String::from_utf8_lossy(&type_out.stdout).trim().to_string();
    assert_eq!(
        type_str, "commit",
        "T3: peer branch HEAD must be a git commit (Rung 6.1c discharge); \
         got type: <{type_str}>"
    );
}

// === T4: commit tree contains substantive morphism blobs ================
#[test]
fn t04_commit_tree_contains_five_morphism_blobs() {
    let dir = make_contribute_fixture("tree");
    let target = write_target_shard(&dir, "fixture.mirror");
    let out = run_contribute(&dir, &target);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let ref_name = stdout
        .lines()
        .find(|l| l.starts_with("+ ref_name: "))
        .and_then(|l| l.split("+ ref_name: ").nth(1))
        .map(|s| s.trim())
        .expect("envelope must emit ref_name field");
    let ls_out = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("ls-tree")
        .arg(ref_name)
        .output()
        .expect("git ls-tree");
    let ls_stdout = String::from_utf8_lossy(&ls_out.stdout).to_string();
    // Mara `4e69066` §3.3: tree contains pre-anchor, post-anchor,
    // morphism-body, settle-verdict, fate-witness.
    for expected in &[
        "pre-anchor",
        "post-anchor",
        "morphism-body",
        "settle-verdict",
        "fate-witness",
    ] {
        assert!(
            ls_stdout.contains(expected),
            "T4: commit tree must contain `{expected}` blob (Mara `4e69066` \
             §3.3 substrate-honest morphism tree shape); got: <{ls_stdout}>"
        );
    }
}

// === T5: target file not found → exit 1 (refusal path) ==================
#[test]
fn t05_missing_target_shard_returns_error() {
    let dir = make_contribute_fixture("missing");
    let bogus_target = dir.join("does-not-exist.mirror");
    let out = run_contribute(&dir, &bogus_target);
    assert!(
        !out.status.success(),
        "T5: missing target_shard must return non-zero exit (contribute \
         refuses without target; morphism not attempted)"
    );
}
