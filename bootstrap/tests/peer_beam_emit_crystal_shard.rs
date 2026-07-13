//! Rung 6' RED — `mirror peer beam <home> --emit-crystal` emits a
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
        oid.len() == 16 && oid.chars().all(|c| c.is_ascii_hexdigit()),
        "T2: crystal_oid must be 16 lowercase hex digits (FNV-1a stub \
         same discipline as Rung 4/5 dance/deploy hash pattern; Rung 6.1 \
         lifts to actual @mirror/store.insert_persistent(bytes) -> oid); \
         got: <{oid}>"
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
