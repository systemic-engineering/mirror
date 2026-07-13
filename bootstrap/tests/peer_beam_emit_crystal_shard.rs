//! Rung 6.1b test additions — `mirror peer beam <home> --emit-crystal` emits a
//! crystal OID on @mirror/store internal ref (`refs/mirror/peer/<uuid>/
//! HEAD`) instead of stdout envelope. Peer inference stays @magic-
//! native; envelope-bytes-hash IS the peer's terminal crystal address;
//! actual @mirror/store insertion + `commit_as_fold` materialization
//! forward-promised to Rung 6.1.
//!
//! Substrate authority:
//! - Mara `d2de1ee` — `docs/specs/mirror-store-bounded-peer-runtime-
//!   materialization-as-single-io-crossing.md` canonical spec (Scope A
//!   annotation-scale + Scope B forward-promised `materialize` action +
//!   Scope C full peer runtime rewire).
//! - Taut `8e98a24` — `docs/scouts/2026-07-13-taut-io-minimization-
//!   mirror-store-peer-runtime-scout.md` §5 (Reed's Rung 6' spec: peer
//!   emits crystal on `refs/mirror/peer/<uuid>/HEAD` instead of stdout
//!   envelope; ONE tick-pair; ZERO @io crossings; all substrate + Rust
//!   primitives landed).
//! - Recognition #43 — mirror IS content-addressed build system;
//!   @mirror/store IS the substrate truth (not disk, not git).
//! - Recognition #55 — form/process partition; @mirror/store form-side
//!   + @kintsugi transformation-side.
//! - Recognition #58 — fate optical inference; @magic-native
//!   computation surface.
//! - Recognition #80 — @magic altitude for gauge-bounded computation.
//! - Alex 2026-07-13 in-transcript: "@peer spawn stayed fully outside
//!   the @io boundary... operated purely within the bounds of
//!   @mirror/store... each peer spawn becomes a @mirror/store/branch"
//!
//! Rung 6' discipline (per Mara §2 + Taut §2): peer inference at @magic
//! altitude (non-linear-eigenvalue land per Yang-Mills gauge/matter
//! substrate); peer state = crystal OIDs on @mirror/store branch ref
//! (content-addressed, bounded, deterministic); materialization = ONE
//! @io crossing via `@kintsugi/store/git.commit_as_fold` (forward-
//! promised to Rung 6.1). Rung 6' MVP: envelope-declared crystal OID
//! emission preserving the pattern established at Rungs 4-5.
//!
//! Five RED tests:
//! T1 --emit-crystal flag accepted; exit 0.
//! T2 envelope emits `crystal_oid: <hex>` field with 16-hex format.
//! T3 envelope names @mirror/store + @kintsugi substrate authorities.
//! T4 envelope emits `ref_name:` pointing to `refs/mirror/peer/<uuid>/`.
//! T5 no --emit-crystal preserves Rungs 1-5 backward-compat.

use std::process::Command;

fn mirror_bin() -> String {
    std::env::var("MIRROR_BIN")
        .unwrap_or_else(|_| "/Users/reed/.cargo-target/release/mirror".to_string())
}

fn make_peer_home(suffix: &str) -> std::path::PathBuf {
    let base = std::env::temp_dir().join(format!(
        "peer-beam-emit-crystal-{}-{}",
        suffix,
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).expect("create peer_home");
    std::fs::write(
        base.join("mirror.spec"),
        "target binary {\n  cli {\n    command beam { arg mission: ~f }\n  }\n}\n",
    )
    .expect("write mirror.spec");
    std::fs::write(
        base.join("observation.txt"),
        "initial substrate observation\n",
    )
    .expect("write observation");
    base
}

fn run_emit_crystal(dir: &std::path::PathBuf) -> std::process::Output {
    Command::new(mirror_bin())
        .arg("peer")
        .arg("beam")
        .arg(dir)
        .arg("--emit-crystal")
        .output()
        .expect("execute mirror peer beam --emit-crystal")
}

fn run_no_crystal_baseline(dir: &std::path::PathBuf) -> std::process::Output {
    Command::new(mirror_bin())
        .arg("peer")
        .arg("beam")
        .arg(dir)
        .output()
        .expect("execute mirror peer beam (no --emit-crystal)")
}

fn extract_field<'a>(stdout: &'a str, key: &str) -> Option<&'a str> {
    stdout
        .lines()
        .find(|l| l.starts_with(&format!("+ {}: ", key)))
        .and_then(|l| l.split(&format!("+ {}: ", key)).nth(1))
        .map(|s| s.trim())
}

// === T1: --emit-crystal flag accepted; exit 0 ===========================
#[test]
fn t01_emit_crystal_flag_accepted_exit_zero() {
    let dir = make_peer_home("exit");
    let out = run_emit_crystal(&dir);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(
        out.status.success(),
        "T1: exit 0 required with --emit-crystal flag; stdout=<{stdout}> stderr=<{stderr}>"
    );
}

// === T2: envelope emits crystal_oid as 16-hex ===========================
#[test]
fn t02_envelope_emits_crystal_oid_hex() {
    let dir = make_peer_home("oid");
    let out = run_emit_crystal(&dir);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let oid = extract_field(&stdout, "crystal_oid")
        .expect("T2: envelope must emit `crystal_oid:` field");
    assert!(
        oid.len() == 64 && oid.chars().all(|c| c.is_ascii_hexdigit()),
        "T2: crystal_oid must be 64 lowercase hex digits from `canonical_hash` \
         (CoincidenceHash<5,5> per hash.rs; substrate's own SHA-256-based \
         5-d basis projection used arc-wide); Rung 6.1a collapse of \
         Rung 6' FNV-1a stub → real content-address; got: <{oid}>"
    );
}

// === T3: envelope names @mirror/store + @kintsugi authorities ==========
#[test]
fn t03_envelope_names_store_and_kintsugi_authorities() {
    let dir = make_peer_home("authorities");
    let out = run_emit_crystal(&dir);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    for authority in &["@mirror/store", "@kintsugi"] {
        assert!(
            stdout.contains(authority),
            "T3: envelope must name `{authority}` substrate authority (per \
             Mara `d2de1ee` §3 canonical shape + Recognition #43/#55 \
             substrate ancestors); got: <{stdout}>"
        );
    }
}

// === T4: envelope emits ref_name pointing to refs/mirror/peer/ =========
#[test]
fn t04_envelope_emits_peer_branch_ref_name() {
    let dir = make_peer_home("ref");
    let out = run_emit_crystal(&dir);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let ref_name = extract_field(&stdout, "ref_name")
        .expect("T4: envelope must emit `ref_name:` field");
    assert!(
        ref_name.starts_with("refs/mirror/peer/"),
        "T4: ref_name must start with `refs/mirror/peer/` (per Alex \
         2026-07-13 in-transcript \"each peer spawn becomes a @mirror/\
         store/branch\"; Taut `8e98a24` §5 Reed's Rung 6' spec); got: \
         <{ref_name}>"
    );
    assert!(
        ref_name.ends_with("/HEAD"),
        "T4: ref_name must end with `/HEAD` (peer's branch HEAD per \
         set_ref convention); got: <{ref_name}>"
    );
}

// === T6: after invocation, `refs/mirror/peer/<uuid>/HEAD` exists =======
//
// Rung 6.1b: actual `git update-ref` writes the peer's branch HEAD to
// peer_home's git store (Recognition #55 form/process partition; the
// ONE @io crossing per peer spawn). Test verifies the ref exists via
// git rev-parse from peer_home. Note: peer_home is git-initialized on
// first --emit-crystal invocation (runtime discipline).
#[test]
fn t06_peer_branch_ref_exists_after_invocation() {
    let dir = make_peer_home("ref-exists");
    let out = run_emit_crystal(&dir);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let ref_name = extract_field(&stdout, "ref_name").expect("ref_name field");
    let rev_out = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("rev-parse")
        .arg(ref_name)
        .output()
        .expect("git rev-parse");
    let rev_stdout = String::from_utf8_lossy(&rev_out.stdout)
        .trim()
        .to_string();
    assert!(
        rev_out.status.success() && !rev_stdout.is_empty(),
        "T6: `git rev-parse {ref_name}` must succeed with non-empty output \
         (Rung 6.1b: peer branch ref materialized to peer_home git store; \
         Mara `d2de1ee` Scope B discharge); got: status={:?} stdout=<{rev_stdout}>",
        rev_out.status
    );
}

// === T7: peer crystal content-addressable via peer branch commit =======
//
// Rung 6.1c commit_as_fold discharge shape: peer branch HEAD IS a git
// commit (verified in T8) whose tree contains `peer-crystal` blob
// containing crystal_oid (verified in T9). T7 spot-checks via
// `git show <ref>:peer-crystal` returns crystal_oid — the peer's
// content-addressable substrate.
#[test]
fn t07_peer_crystal_content_addressable_via_commit_tree() {
    let dir = make_peer_home("content-address");
    let out = run_emit_crystal(&dir);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let ref_name = extract_field(&stdout, "ref_name").expect("ref_name field");
    let crystal_oid = extract_field(&stdout, "crystal_oid").expect("crystal_oid field");
    let show_out = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("show")
        .arg(format!("{}:peer-crystal", ref_name))
        .output()
        .expect("git show peer-crystal");
    let show_stdout = String::from_utf8_lossy(&show_out.stdout)
        .trim()
        .to_string();
    assert!(
        show_out.status.success(),
        "T7: `git show {ref_name}:peer-crystal` must succeed (Rung 6.1c \
         commit_as_fold discharge; peer's crystal_oid content-addressable \
         via peer branch commit → tree → blob per Recognition #43); got: \
         status={:?}",
        show_out.status
    );
    assert_eq!(
        show_stdout, crystal_oid,
        "T7: peer-crystal blob content must equal crystal_oid \
         (content-addressable peer crystal); got blob=<{show_stdout}>, \
         crystal_oid=<{crystal_oid}>"
    );
}

// === T8: peer branch HEAD is a git commit (not blob or tree) ===========
//
// Rung 6.1c collapse: `commit_as_fold` discharge per Recognition #55
// form/process partition. Peer's ref points to a REAL git commit
// object (built via mktree + commit-tree). Verifies substrate-honest
// materialization shape @kintsugi/store/git.commit_as_fold canonicalizes.
#[test]
fn t08_peer_branch_head_is_git_commit() {
    let dir = make_peer_home("commit-shape");
    let out = run_emit_crystal(&dir);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let ref_name = extract_field(&stdout, "ref_name").expect("ref_name field");
    let type_out = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("cat-file")
        .arg("-t")
        .arg(ref_name)
        .output()
        .expect("git cat-file -t");
    let type_str = String::from_utf8_lossy(&type_out.stdout)
        .trim()
        .to_string();
    assert_eq!(
        type_str, "commit",
        "T8: `git cat-file -t {ref_name}` must return `commit` (Rung 6.1c: \
         commit_as_fold discharge per Recognition #55 folds blob into \
         tree into commit; peer branch HEAD IS a real git commit); \
         got: <{type_str}>"
    );
}

// === T9: peer commit tree contains crystal blob at `peer-crystal` ======
//
// Verifies the substrate-honest materialization shape: the peer's
// commit tree contains one entry "peer-crystal" pointing to the blob
// containing crystal_oid. Full form/process discharge witnessed.
#[test]
fn t09_peer_commit_tree_contains_peer_crystal_blob() {
    let dir = make_peer_home("tree-shape");
    let out = run_emit_crystal(&dir);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let ref_name = extract_field(&stdout, "ref_name").expect("ref_name field");
    let crystal_oid = extract_field(&stdout, "crystal_oid").expect("crystal_oid field");
    // Read the tree via `git ls-tree`.
    let ls_out = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("ls-tree")
        .arg(ref_name)
        .output()
        .expect("git ls-tree");
    let ls_stdout = String::from_utf8_lossy(&ls_out.stdout).to_string();
    assert!(
        ls_stdout.contains("peer-crystal"),
        "T9: `git ls-tree {ref_name}` must contain `peer-crystal` entry \
         (Recognition #55 substrate-honest materialization); got: <{ls_stdout}>"
    );
    // Read the blob content via path.
    let blob_out = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("show")
        .arg(format!("{}:peer-crystal", ref_name))
        .output()
        .expect("git show peer-crystal");
    let blob_content = String::from_utf8_lossy(&blob_out.stdout).trim().to_string();
    assert_eq!(
        blob_content, crystal_oid,
        "T9: `git show {ref_name}:peer-crystal` must equal crystal_oid; \
         got: <{blob_content}>"
    );
}

// === T10: parent chain — second invocation chains from first ==========
//
// Rung 6.2a collapse: @mirror/store IS a DAG per Recognition #43 +
// splinter_graph trichotomy. Peer beams should CHAIN via git commit
// parent linkage. Test: two --emit-crystal invocations on the same
// peer_home; second commit's parent should be first commit.
#[test]
fn t10_second_invocation_chains_from_parent() {
    let dir = make_peer_home("parent-chain");
    // First invocation — creates root commit (no parent).
    let out1 = run_emit_crystal(&dir);
    let stdout1 = String::from_utf8_lossy(&out1.stdout).to_string();
    let ref_name = extract_field(&stdout1, "ref_name").expect("ref_name field");
    let commit1 = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("rev-parse")
        .arg(ref_name)
        .output()
        .expect("rev-parse");
    let commit1_sha = String::from_utf8_lossy(&commit1.stdout).trim().to_string();
    assert!(!commit1_sha.is_empty(), "T10: first invocation must produce commit");

    // Second invocation — must chain from first commit.
    let out2 = run_emit_crystal(&dir);
    let stdout2 = String::from_utf8_lossy(&out2.stdout).to_string();
    assert!(
        out2.status.success(),
        "T10: second invocation must succeed; stdout=<{stdout2}>"
    );
    let commit2 = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("rev-parse")
        .arg(ref_name)
        .output()
        .expect("rev-parse");
    let commit2_sha = String::from_utf8_lossy(&commit2.stdout).trim().to_string();
    // Note: identical inputs → deterministic content → identical crystals.
    // The blob/tree are byte-equal to first invocation, but the commit
    // itself has a parent pointer so its SHA differs.
    assert_ne!(
        commit1_sha, commit2_sha,
        "T10: second commit must differ from first (has parent pointer per \
         Rung 6.2a chain); got commit1={commit1_sha}, commit2={commit2_sha}"
    );
    // Verify parent linkage: second commit's parent IS first commit.
    let parents = Command::new("git")
        .arg("-C")
        .arg(&dir)
        .arg("rev-list")
        .arg("--parents")
        .arg("-n")
        .arg("1")
        .arg(&commit2_sha)
        .output()
        .expect("rev-list --parents");
    let parents_stdout = String::from_utf8_lossy(&parents.stdout)
        .trim()
        .to_string();
    assert!(
        parents_stdout.contains(&commit1_sha),
        "T10: second commit's parent must be first commit (@mirror/store IS \
         a DAG per Recognition #43); got: <{parents_stdout}>"
    );
}

// === T5: no --emit-crystal preserves Rungs 1-5 backward-compat =========
#[test]
fn t05_no_emit_crystal_preserves_backward_compat() {
    let dir = make_peer_home("regression");
    let out = run_no_crystal_baseline(&dir);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        out.status.success(),
        "T5: exit 0 required for peer beam without --emit-crystal (Rungs \
         1-5 backward-compat)"
    );
    // Verify Rung 6' envelope did NOT emit:
    for absent in &["crystal_oid:", "ref_name: refs/mirror/peer/"] {
        assert!(
            !stdout.contains(absent),
            "T5: base peer beam MUST NOT emit `{absent}` when --emit-crystal \
             is absent (byte-equality guard for Rungs 1-5); got: <{stdout}>"
        );
    }
}
