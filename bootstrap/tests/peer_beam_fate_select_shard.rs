//! Peer beam fate inference RED — `mirror peer beam <home>
//! --fate-select` invokes Fate::excited().resolve on Features::default()
//! and emits the selected Model + mapped prism-op via the bundle tower
//! binding (`boot/std/epistemologic/math/bundle.mirror`).
//!
//! Substrate authority:
//! - Mara `b0427fd` @optics/lens family-root
//! - Mara `f3af5b4` @optics/lens/features species (reuses graph_observation)
//! - Seam `d9b7c35` Adj 1: v0 Rust discharge uses Features::default()
//! - boot/std/epistemologic/math/bundle.mirror: 5-level tower binding
//!   * Abyss   ↔ focus   (Level 0 Fiber)
//!   * Introject ↔ project (Level 1 Connection)
//!   * Cartographer ↔ split (Level 2 Gauge)
//!   * Explorer ↔ shift  (Level 3 Transport)
//!   * Fate    ↔ settle  (Level 4 Closure)
//!
//! v0 semantics: Features::default() (all zeros) per Seam Adj 1 — an
//! explicit "no observation yet" state at the correct altitude. Not a
//! type-lie (blake3 was drift per Seam). Fate::excited() uses xorshift64
//! seeded from system time so decisions are non-deterministic; tests
//! assert PRESENCE of one-of-five model names + one-of-five prism ops
//! rather than specific values.

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

fn peer_beam_fate_select() -> (Output, PathBuf) {
    let mut dir = std::env::temp_dir();
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    dir.push(format!(
        "mirror-peer-beam-fate-{}-{}",
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
        .arg("--fate-select")
        .output()
        .expect("mirror peer beam --fate-select");

    (out, dir)
}

// === T1: --fate-select emits fate_decision field with a Model name ====

#[test]
fn t01_peer_beam_fate_select_emits_fate_decision() {
    let (out, dir) = peer_beam_fate_select();
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();

    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        out.status.success(),
        "T1: exit 0 required; stdout=<{stdout}> stderr=<{stderr}>"
    );

    assert!(
        stdout.contains("fate_decision:"),
        "T1: stdout must include `fate_decision:` field; got: <{stdout}>"
    );

    let has_model = ["Abyss", "Introject", "Cartographer", "Explorer", "Fate"]
        .iter()
        .any(|m| stdout.contains(m));

    assert!(
        has_model,
        "T1: stdout must name one of the five Fate Models \
         (Abyss / Introject / Cartographer / Explorer / Fate); got: <{stdout}>"
    );
}

// === T2: fate decision maps to one of the 5 prism operations ==========
//
// Per boot/std/epistemologic/math/bundle.mirror binding. The peer emits
// both the model name (for provenance) AND the mapped prism-op (for
// consumer semantics).

#[test]
fn t02_peer_beam_fate_select_maps_to_prism_op() {
    let (out, dir) = peer_beam_fate_select();
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();

    let _ = std::fs::remove_dir_all(&dir);

    let has_op = ["focus", "project", "split", "shift", "settle"]
        .iter()
        .any(|op| stdout.contains(op));

    assert!(
        has_op,
        "T2: stdout must name one of the five prism operations \
         (focus / project / split / shift / settle) per bundle tower \
         binding; got: <{stdout}>"
    );
}

// === T3: emitted envelope names @optics/lens/features authority =======

#[test]
fn t03_peer_beam_fate_select_names_optics_lens_features() {
    let (out, dir) = peer_beam_fate_select();
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();

    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        stdout.contains("@optics/lens/features"),
        "T3: stdout must name @optics/lens/features as substrate authority \
         (per Mara f3af5b4); got: <{stdout}>"
    );
}
