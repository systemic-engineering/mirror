//! Blocker 2 v1 empirical spawn RED — `mirror peer beam <home>
//! --fate-select --from-psychohistory` produces fate decision grounded
//! in peer's psychohistory (v1 stub: blake3-derived selectors from
//! peer_home content graph).
//!
//! Substrate authority:
//! - Mara `2c26537` iter-31 — `psychohistory_sheaf` + `psychohistory_from_tray`
//!   + `psychohistory_consistent` substrate-decl'd at @song/narrative
//! - Mara `ce9745f` iter-32 — `bounded_by(sheaf) -> tournament_result`
//!   at @fate/tournament (with holonomy + Lawvere depth bilaterals)
//! - Mara `96ff532` iter-30 — Fate::bounded canonical spec (bundle-
//!   tower type schema; sheaf-Laplacian Rayleigh descent navigation)
//! - Taut `e90daf1` iter-15 — storage primitive substrate-correct
//!   (@bauchladen already lifts @mirror/store; zero fault-planes)
//! - Reed `9cf1e3b` — fate-select v0 GREEN (Features::default() +
//!   Fate::excited(); v1 lift is THIS tick)
//!
//! v1 semantics (stub for full sheaf-Laplacian Rayleigh descent):
//! walks peer_home dir, blake3-hashes each file's content as one
//! "moment_oid", aggregates all moment_oids into a psychohistory_root_oid,
//! derives Fate::selectors deterministically from that root (32 bytes
//! → 450 f64 params). Fate::untrained() as base for the private
//! connection field; selectors mutated to psychohistory-derived weights.
//!
//! Not full sheaf-Laplacian Δ_F Rayleigh direction (that's a multi-tick
//! numerical arc). But: real content-address of peer, real derived
//! weights, real deterministic decisions grounded in peer's content.
//! Two peers with different content produce different decisions from
//! the SAME mission. That's autopoietic-adjacent — the missing piece
//! is the sheaf-Laplacian eigenvector computation replacing the blake3
//! mapping.

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

fn make_peer_home(seed_marker: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    dir.push(format!(
        "mirror-peer-beam-psycho-{}-{}-{}",
        seed_marker,
        std::process::id(),
        ts
    ));
    std::fs::create_dir_all(&dir).expect("create fixture dir");

    std::fs::write(
        dir.join("mirror.spec"),
        "target binary {\n  cli {\n    command beam { arg mission: ~f }\n  }\n}\n",
    )
    .expect("write mirror.spec");

    // Seed some "psychohistory moments" as files whose content differs
    // per seed_marker. Each file becomes one moment_oid in the aggregate.
    std::fs::write(
        dir.join("moment_1.txt"),
        format!("first observation for {seed_marker}\n"),
    )
    .expect("write moment_1");
    std::fs::write(
        dir.join("moment_2.txt"),
        format!("second observation for {seed_marker}\n"),
    )
    .expect("write moment_2");

    dir
}

fn peer_beam_from_psychohistory(seed_marker: &str) -> (Output, PathBuf) {
    let dir = make_peer_home(seed_marker);
    let out = Command::new(mirror_bin())
        .current_dir(repo_root())
        .arg("peer")
        .arg("beam")
        .arg(dir.to_str().expect("utf-8 tempdir"))
        .arg("--fate-select")
        .arg("--from-psychohistory")
        .output()
        .expect("mirror peer beam --fate-select --from-psychohistory");
    (out, dir)
}

// === T1: --from-psychohistory emits psychohistory_root_oid ============
//
// Post-GREEN: envelope names psychohistory_root_oid (aggregate blake3
// over peer_home moments). Pre-GREEN: --from-psychohistory is not
// admitted; envelope falls through to v0 stub with Features::default().

#[test]
fn t01_from_psychohistory_emits_root_oid() {
    let (out, dir) = peer_beam_from_psychohistory("alpha");
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();

    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        out.status.success(),
        "T1: exit 0 required; stdout=<{stdout}> stderr=<{stderr}>"
    );

    assert!(
        stdout.contains("psychohistory_root_oid:"),
        "T1: stdout must include `psychohistory_root_oid:` field; got: <{stdout}>"
    );

    // Names the substrate authority for the bounded_by action.
    assert!(
        stdout.contains("@fate/tournament.bounded_by") || stdout.contains("bounded_by"),
        "T1: stdout must name @fate/tournament.bounded_by as substrate authority \
         (per Mara ce9745f); got: <{stdout}>"
    );
}

// === T2: different peer content produces different psychohistory root =
//
// Two peer homes with different content must produce different
// psychohistory_root_oid values. This is the autopoietic ground: the
// peer's decisions ARE bounded by their own history, so different
// histories mean different bounds.

#[test]
fn t02_different_peer_content_produces_different_root_oids() {
    let (out_alpha, dir_alpha) = peer_beam_from_psychohistory("alpha");
    let (out_beta, dir_beta) = peer_beam_from_psychohistory("beta");

    let stdout_alpha = String::from_utf8_lossy(&out_alpha.stdout).to_string();
    let stdout_beta = String::from_utf8_lossy(&out_beta.stdout).to_string();

    let _ = std::fs::remove_dir_all(&dir_alpha);
    let _ = std::fs::remove_dir_all(&dir_beta);

    let extract_root = |s: &str| -> Option<String> {
        for line in s.lines() {
            let trimmed = line.trim_start_matches("+").trim();
            if let Some(rest) = trimmed.strip_prefix("psychohistory_root_oid:") {
                return Some(rest.trim().to_string());
            }
        }
        None
    };

    let root_alpha = extract_root(&stdout_alpha).unwrap_or_default();
    let root_beta = extract_root(&stdout_beta).unwrap_or_default();

    assert!(
        !root_alpha.is_empty() && !root_beta.is_empty(),
        "T2: both invocations must emit non-empty psychohistory_root_oid; \
         alpha=<{root_alpha}> beta=<{root_beta}>"
    );

    assert_ne!(
        root_alpha, root_beta,
        "T2: different peer content MUST produce different psychohistory_root_oid \
         (autopoietic ground: peer's decisions are bounded by their own history); \
         alpha=<{root_alpha}> beta=<{root_beta}>"
    );
}

// === T3: identical peer content produces identical psychohistory root =
//
// Same content → same root_oid (content-address stability). This is the
// Foster PutPut-like witness at psychohistory altitude: deterministic
// derivation of weights from peer content.

#[test]
fn t03_identical_content_produces_identical_root_oids() {
    let (out_a, dir_a) = peer_beam_from_psychohistory("gamma");
    let (out_b, dir_b) = peer_beam_from_psychohistory("gamma");

    let stdout_a = String::from_utf8_lossy(&out_a.stdout).to_string();
    let stdout_b = String::from_utf8_lossy(&out_b.stdout).to_string();

    let _ = std::fs::remove_dir_all(&dir_a);
    let _ = std::fs::remove_dir_all(&dir_b);

    let extract_root = |s: &str| -> Option<String> {
        for line in s.lines() {
            let trimmed = line.trim_start_matches("+").trim();
            if let Some(rest) = trimmed.strip_prefix("psychohistory_root_oid:") {
                return Some(rest.trim().to_string());
            }
        }
        None
    };

    let root_a = extract_root(&stdout_a).unwrap_or_default();
    let root_b = extract_root(&stdout_b).unwrap_or_default();

    assert_eq!(
        root_a, root_b,
        "T3: identical peer content MUST produce identical psychohistory_root_oid \
         (content-address stability); a=<{root_a}> b=<{root_b}>"
    );
}
