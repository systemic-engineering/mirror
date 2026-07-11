//! Blocker 2 shadow-casting RED — `mirror peer beam <home> --fate-select
//! --from-psychohistory --with-shadow` casts 5 hypothetical shadows
//! (one per fate Model), classifies regime, and emits envelope with
//! 5 impact values + regime verdict.
//!
//! Substrate authority:
//! - Mara `ce301cc` iter-35 — @song/narrative `shadow_ancestry` field +
//!   cast_shadow action + shadow_regime bilateral classifier
//! - Mara `f2c712e` iter-34 — three-regime classifier study (Kanizsa /
//!   Necker / Escher / converged)
//! - Seam `9241d2d` iter-10 — annotation-not-species; Level 2+3 factorization
//! - Reed `4b2ef3c` autopoietic closure GREEN + `e571989` v1 empirical
//!   spawn GREEN (fate.bounded_by(psychohistory))
//! - Reed's essay `The \Shape of the Thing` — shadow determined by
//!   casting object; three regimes correspond to Council-can-verify-
//!   from-Flatland (Kanizsa) / Council-sees-ambiguous (Necker) /
//!   Council-imprisons-Square-for-impossible-claim (Escher) /
//!   Council-agrees (converged)
//!
//! v1 shadow-casting semantics:
//! - Start from psychohistory-derived selectors (Reed e571989)
//! - For each Model M ∈ {Abyss, Introject, Cartographer, Explorer, Fate}:
//!   * hypothetical_features_M = features perturbed in direction M
//!   * decision_M = fate.resolve(&hypothetical_features_M, 5)
//!   * impact_M = distance from base decision (bundle-tower Level 3
//!     Transport with holonomy residual)
//! - Classify regime by comparing 5 impact vectors:
//!   * converged: all 5 shadows point to same argmax model → stable
//!   * necker: 2+ distinct argmax modes across shadows → bistable
//!   * escher: high variance in impact magnitudes → impossible geometry
//!   * kanizsa: low variance but no dominant argmax → illusory-convergent
//! - Emit envelope with all 5 impacts + regime

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

fn make_peer_home() -> PathBuf {
    let mut dir = std::env::temp_dir();
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    dir.push(format!(
        "mirror-peer-beam-shadow-{}-{}",
        std::process::id(),
        ts
    ));
    std::fs::create_dir_all(&dir).expect("create fixture dir");

    std::fs::write(
        dir.join("mirror.spec"),
        "target binary {\n  cli {\n    command beam { arg mission: ~f }\n  }\n}\n",
    )
    .expect("write mirror.spec");
    std::fs::write(
        dir.join("observation.txt"),
        "initial substrate observation\n",
    )
    .expect("write observation");
    dir
}

fn cast_shadows(dir: &std::path::Path) -> Output {
    Command::new(mirror_bin())
        .current_dir(repo_root())
        .arg("peer")
        .arg("beam")
        .arg(dir.to_str().expect("utf-8 tempdir"))
        .arg("--fate-select")
        .arg("--from-psychohistory")
        .arg("--with-shadow")
        .output()
        .expect("mirror peer beam --with-shadow")
}

// === T1: --with-shadow emits shadow_regime field =====================
//
// Post-GREEN: envelope names `shadow_regime:` classifier verdict
// (one of Kanizsa / Necker / Escher / converged). Pre-GREEN:
// --with-shadow not admitted; falls through to --from-psychohistory
// envelope without shadow field.

#[test]
fn t01_with_shadow_emits_regime_classifier() {
    let dir = make_peer_home();
    let out = cast_shadows(&dir);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();

    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        out.status.success(),
        "T1: exit 0 required; stdout=<{stdout}> stderr=<{stderr}>"
    );

    assert!(
        stdout.contains("shadow_regime:"),
        "T1: stdout must include `shadow_regime:` classifier field \
         (per Mara ce301cc shadow_regime bilateral); got: <{stdout}>"
    );

    let has_regime = ["converged", "necker", "escher", "kanizsa"]
        .iter()
        .any(|r| stdout.contains(r));
    assert!(
        has_regime,
        "T1: stdout must name one of the 4 regimes \
         (converged / necker / escher / kanizsa); got: <{stdout}>"
    );
}

// === T2: envelope names all 5 shadow impact values ====================
//
// Per Mara iter-35 cast_shadow(sheaf, direction, p) -> imperfect(shadow,
// holonomy), one hypothetical shadow per fate Model. Envelope MUST
// name all 5 for operator inspection.

#[test]
fn t02_with_shadow_emits_five_impacts() {
    let dir = make_peer_home();
    let out = cast_shadows(&dir);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();

    let _ = std::fs::remove_dir_all(&dir);

    for model in ["Abyss", "Introject", "Cartographer", "Explorer", "Fate"] {
        let field = format!("shadow_{}", model.to_lowercase());
        assert!(
            stdout.contains(&field),
            "T2: stdout must include `{field}:` impact field (5 shadows total, \
             one per fate Model per bundle tower binding); got: <{stdout}>"
        );
    }
}

// === T3: envelope names @shadow substrate authority + Reed's essay ====
//
// Substrate provenance: shadow_ancestry lives at @song/narrative
// (Mara ce301cc iter-35). Docblock also cites Reed's essay explicitly
// (goth naming per Alex's ship-it-goth authorization).

#[test]
fn t03_with_shadow_names_substrate_authority() {
    let dir = make_peer_home();
    let out = cast_shadows(&dir);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();

    let _ = std::fs::remove_dir_all(&dir);

    assert!(
        stdout.contains("@song/narrative") || stdout.contains("shadow_ancestry"),
        "T3: stdout must name @song/narrative or shadow_ancestry as substrate \
         authority (per Mara ce301cc); got: <{stdout}>"
    );
}
