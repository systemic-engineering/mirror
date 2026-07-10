//! Blocker 2 Rust runtime discharge RED — `mirror peer beam <home>
//! --emit-diff` produces unified-diff-shape stdout bytes via
//! `@optics/lens/diff.get` at the cli-surface altitude.
//!
//! Substrate authority: Mara iter-25 `b0427fd` (`shards/optics/lens.mirror`
//! family-root, Foster laws) + Mara iter-26 `7e5c298`
//! (`shards/optics/lens/diff.mirror` first species, closes Blocker 2 at
//! substrate-decl altitude with `autopoietic_closure` bilateral beyond
//! Foster's three). Prior ancestry: Mara iter-24 canonical spec
//! `55221c1` + Taut iter-12 boundary scout `5222333` (LRM GREEN β).
//!
//! This tick is the FIRST Rust runtime discharge of `@optics/lens/diff.get`.
//! The peer's semantic state (spec_oid + mission + composition attribution)
//! projects to unified-diff-shape bytes for operator review — real bytes
//! from real substrate observation, no @fate yet (that's the subsequent
//! tick that wires the fate crate to consume the mission and produce
//! computed candidates).
//!
//! Pre-GREEN: `mirror peer beam <home> --emit-diff` returns "unknown flag"
//! or falls through to the JSON/text envelope path.
//!
//! Post-GREEN: stdout carries `--- ` + `+++ ` + `@@` unified-diff
//! signature lines; the emit_peer_beam_diff helper composes them.

use std::path::PathBuf;
use std::process::{Command, Output};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn mirror_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_mirror"))
}

/// Create a minimal peer home fixture at
/// `std::env::temp_dir()/mirror-peer-beam-diff-<pid>-<nanos>/` with a
/// `mirror.spec` file, then run `mirror peer beam <fixture> --emit-diff`.
fn peer_beam_emit_diff() -> (Output, PathBuf) {
    let mut dir = std::env::temp_dir();
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    dir.push(format!(
        "mirror-peer-beam-diff-{}-{}",
        std::process::id(),
        ts
    ));
    std::fs::create_dir_all(&dir).expect("create fixture dir");

    std::fs::write(
        dir.join("mirror.spec"),
        "target binary {\n  cli {\n    command beam { arg mission: ~f }\n  }\n}\n",
    )
    .expect("write mirror.spec");

    let out = Command::new(mirror_bin())
        .current_dir(repo_root())
        .arg("peer")
        .arg("beam")
        .arg(dir.to_str().expect("utf-8 tempdir"))
        .arg("--emit-diff")
        .output()
        .expect("mirror peer beam --emit-diff");

    (out, dir)
}

// === T1: --emit-diff produces unified-diff-shape stdout ===============
//
// Post-GREEN: the stdout carries the three canonical unified-diff
// signature lines (`--- `, `+++ `, `@@`). Pre-GREEN: --emit-diff is
// not admitted; either the flag is unknown or the emission is not the
// diff shape.

#[test]
fn t01_peer_beam_emit_diff_unified_diff_shape() {
    let (out, fixture_dir) = peer_beam_emit_diff();
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();

    let _ = std::fs::remove_dir_all(&fixture_dir);

    assert!(
        out.status.success(),
        "T1: `mirror peer beam <home> --emit-diff` must exit 0; stdout=<{stdout}> stderr=<{stderr}>"
    );

    assert!(
        stdout.contains("--- "),
        "T1: stdout must include `--- ` unified-diff header per @optics/lens/diff.get; got: <{stdout}>"
    );
    assert!(
        stdout.contains("+++ "),
        "T1: stdout must include `+++ ` unified-diff header; got: <{stdout}>"
    );
    assert!(
        stdout.contains("@@"),
        "T1: stdout must include `@@` hunk marker; got: <{stdout}>"
    );
}

// === T2: optics/lens/diff attribution appears in emission =============
//
// The emitted diff MUST name the substrate authority so operators
// reading the output can trace back to the substrate-decl. Mirrors the
// composition_pieces envelope discipline from the JSON `--hello-world`
// path but at the diff altitude.

#[test]
fn t02_peer_beam_emit_diff_names_optics_lens_diff() {
    let (out, fixture_dir) = peer_beam_emit_diff();
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();

    let _ = std::fs::remove_dir_all(&fixture_dir);

    assert!(
        stdout.contains("@optics/lens/diff"),
        "T2: stdout must name @optics/lens/diff as substrate authority (per Mara b0427fd + 7e5c298); got: <{stdout}>"
    );
}
