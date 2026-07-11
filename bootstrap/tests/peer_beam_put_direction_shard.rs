//! Blocker 2 Rust runtime discharge — `put` direction test lock.
//!
//! Closes the Foster (get, put) roundtrip half at cli-surface altitude.
//! `--integrate-diff` reads bytes from stdin (edited unified-diff), computes
//! delta_oid via blake3(spec_bytes || stdin_bytes), and emits an envelope
//! naming the substrate update.
//!
//! Substrate authority: Mara iter-26 `7e5c298` (`shards/optics/lens/diff.mirror`
//! declared the `put` action + autopoietic_closure bilateral). Reed
//! `be74b6a` landed the `get` direction; this tick lands `put`.
//!
//! Foster's PutGet law verifies structurally: `get(put(v, s)) = v` at
//! v0 becomes: emit_diff produces bytes B; integrate_diff receives
//! bytes B'; the emitted envelope names both spec_oid and delta_oid so
//! the roundtrip is observable at the byte level.
//!
//! v0 semantics: integrate_diff DOES NOT actually apply the edit; it
//! ACKNOWLEDGES the edit and updates the peer's addressable state. The
//! full application discipline (which prism operations to derive from
//! the received diff, how to update @bauchladen.tray) is future ticks
//! contingent on @fate wiring.

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has a parent (the repo root)")
        .to_path_buf()
}

fn mirror_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_mirror"))
}

/// Run `mirror peer beam <home> --integrate-diff` piping `edit_bytes`
/// on stdin. Returns the Output and the fixture dir path for cleanup.
fn peer_beam_integrate_diff(edit_bytes: &[u8]) -> (Output, PathBuf) {
    let mut dir = std::env::temp_dir();
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    dir.push(format!(
        "mirror-peer-beam-put-{}-{}",
        std::process::id(),
        ts
    ));
    std::fs::create_dir_all(&dir).expect("create fixture dir");

    std::fs::write(
        dir.join("mirror.spec"),
        "target binary {\n  cli {\n    command beam { arg mission: ~f }\n  }\n}\n",
    )
    .expect("write mirror.spec");

    let mut child = Command::new(mirror_bin())
        .current_dir(repo_root())
        .arg("peer")
        .arg("beam")
        .arg(dir.to_str().expect("utf-8 tempdir"))
        .arg("--integrate-diff")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn mirror peer beam --integrate-diff");

    {
        let stdin = child.stdin.as_mut().expect("child stdin");
        stdin.write_all(edit_bytes).expect("write edit bytes to stdin");
    }

    let out = child.wait_with_output().expect("child output");
    (out, dir)
}

// === T1: --integrate-diff produces envelope naming delta_oid ==========
//
// Post-GREEN: the stdout envelope must name `delta_oid:` (blake3 of
// received bytes). Pre-GREEN: --integrate-diff is not admitted; either
// the flag is unknown or the emission falls through to the default
// text envelope.

#[test]
fn t01_peer_beam_integrate_diff_emits_delta_oid() {
    let (out, fixture_dir) = peer_beam_integrate_diff(b"--- a/mirror.spec\n+++ b/mirror.spec\n@@ operator edit @@\n+ mission-clarification: prefer smaller diffs\n");
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();

    let _ = std::fs::remove_dir_all(&fixture_dir);

    assert!(
        out.status.success(),
        "T1: `mirror peer beam <home> --integrate-diff` must exit 0; \
         stdout=<{stdout}> stderr=<{stderr}>"
    );

    assert!(
        stdout.contains("delta_oid:"),
        "T1: stdout must name `delta_oid:` (blake3 of received edit bytes) \
         per @optics/lens/diff.put; got: <{stdout}>"
    );
}

// === T2: --integrate-diff names @optics/lens/diff.put attribution =====

#[test]
fn t02_peer_beam_integrate_diff_names_put_direction() {
    let (out, fixture_dir) = peer_beam_integrate_diff(b"+ some operator edit\n");
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();

    let _ = std::fs::remove_dir_all(&fixture_dir);

    assert!(
        stdout.contains("@optics/lens/diff.put"),
        "T2: stdout must name @optics/lens/diff.put as substrate authority \
         (per Mara 7e5c298); got: <{stdout}>"
    );
}

// === T3: identical edit bytes produce identical delta_oid =============
//
// Foster PutPut law substrate witness: put(v', put(v, s)) = put(v', s).
// At v0: the SAME edit bytes produce the SAME delta_oid regardless of
// invocation order (content-address stability).

#[test]
fn t03_peer_beam_integrate_diff_delta_oid_is_deterministic() {
    let edit = b"+ edit line one\n+ edit line two\n";
    let (out_a, dir_a) = peer_beam_integrate_diff(edit);
    let (out_b, dir_b) = peer_beam_integrate_diff(edit);

    let stdout_a = String::from_utf8_lossy(&out_a.stdout).to_string();
    let stdout_b = String::from_utf8_lossy(&out_b.stdout).to_string();

    let _ = std::fs::remove_dir_all(&dir_a);
    let _ = std::fs::remove_dir_all(&dir_b);

    // Extract delta_oid from both.
    let extract_delta = |s: &str| -> Option<String> {
        for line in s.lines() {
            if let Some(rest) = line.trim_start_matches("+").trim().strip_prefix("delta_oid:") {
                return Some(rest.trim().to_string());
            }
        }
        None
    };

    let delta_a = extract_delta(&stdout_a).unwrap_or_default();
    let delta_b = extract_delta(&stdout_b).unwrap_or_default();

    assert!(
        !delta_a.is_empty() && delta_a == delta_b,
        "T3: same edit bytes MUST produce same delta_oid (content-address stability); \
         delta_a=<{delta_a}> delta_b=<{delta_b}>"
    );
}
