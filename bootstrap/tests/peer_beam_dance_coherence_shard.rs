//! Rung 4 RED — `mirror peer beam <home1> --song <song> --dance-with
//! <home2>` fires @song at two peer-homes, computes multi-peer coherence
//! phase-lock, emits @dance envelope naming Kuramoto order parameter +
//! Aumann agreement + shared_root_oid + convergence_verdict classifier.
//!
//! Substrate authority:
//! - Mara `417ec25` — `docs/specs/dance-runtime-rung-4-multi-peer-
//!   coherence-phase-lock.md` Scope B narrowed verdict + canonical
//!   `execute_dance` signature + envelope shape (§3.2-§3.3) + five T-test
//!   spec (§3.4).
//! - Mara `94e55eb` — `shards/song/beat.mirror:453-457` verbatim: Rung 4
//!   substrate reservation: "multi-peer @dance coupling on shared beat;
//!   `bootstrap/src/dance.rs` module reads two peer-homes; measures
//!   phase-difference on cybernetic_coherence deltas; reports Kuramoto
//!   order-parameter; emits aumann_agreement envelope on convergence."
//! - Reed `8e6e517` — cybernetic_coherence = λ₀(Δ_F) at
//!   `shards/cyberpunk.mirror` Path B annotation; Rung 4 extends to
//!   ensemble scale via envelope-bytes-hash stub (Rung 4.5 forward-
//!   promise per §4.5 for actual λ₀(Δ_{F₁⊕F₂}) computation).
//! - Mara `4f079c8` — @dance canonical spec (Path C recognition candidate
//!   `#R-dance-is-coordination-without-signal-on-forster-torus`).
//! - Mara `9e48710` — @resonance Kuramoto coupling formalization §2.4.
//! - Reed `0cc4e11` — Rung 3 GREEN (song.rs tokenize+AST walk; Rung 4
//!   composes on this substrate).
//! - Alex 2026-07-13 in-transcript /loop ladder-climb mandate.
//!
//! Rung 4 discipline: two peer_homes fixture; subprocess-spawn one
//! `mirror peer beam` invocation which internally dispatches to
//! `execute_dance`; both peers execute the SAME `.song` file (shared
//! prior); dance envelope emits Kuramoto order-parameter + Aumann
//! agreement + shared_root_oid + convergence_verdict. Rung 4.5 forward-
//! promises actual λ₀(Δ_F) coherence + N-peer generalization.
//!
//! Five RED tests per Mara `417ec25` §3.4:
//! T1 dispatch acceptance (exit 0); T2 @dance/@resonance/@cyberpunk/
//! @bauchladen authority naming; T3 kuramoto_order_parameter field +
//! parseable f64 in [0,1]; T4 aumann_agreement + shared_root_oid fields;
//! T5 convergence_verdict classifier from {converged/dispersed/chimera}.

use std::process::Command;

fn mirror_bin() -> String {
    std::env::var("MIRROR_BIN")
        .unwrap_or_else(|_| "/Users/reed/.cargo-target/release/mirror".to_string())
}

fn make_two_peer_homes(suffix: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    let base = std::env::temp_dir().join(format!(
        "peer-beam-dance-coherence-{}-{}",
        suffix,
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&base);
    let home_1 = base.join("peer_1");
    let home_2 = base.join("peer_2");
    for home in [&home_1, &home_2] {
        std::fs::create_dir_all(home).expect("create peer_home");
        std::fs::write(
            home.join("mirror.spec"),
            "target binary {\n  cli {\n    command beam { arg mission: ~f }\n  }\n}\n",
        )
        .expect("write mirror.spec");
        std::fs::write(
            home.join("observation.txt"),
            "initial substrate observation\n",
        )
        .expect("write observation");
    }
    (home_1, home_2)
}

fn write_shared_song(base_dir: &std::path::Path) -> std::path::PathBuf {
    // Rung 4 shared prior: minimum-viable @song per Mara `d29d45e` §3.4
    // (also Reed's Rung 3 fixture at peer_beam_song_movement_shard.rs).
    let path = base_dir.join("shared_dance.song");
    std::fs::write(
        &path,
        "song hello_world {\n  movement greet {\n    voice compiler {\n      scope: @mirror/mosaic\n      lines: assemble\n    }\n    beat strike {\n      action: @kintsugi/oscillate\n    }\n  }\n}\n",
    )
    .expect("write shared song file");
    path
}

fn run_dance(
    home_1: &std::path::PathBuf,
    home_2: &std::path::PathBuf,
    song: &std::path::PathBuf,
) -> std::process::Output {
    Command::new(mirror_bin())
        .arg("peer")
        .arg("beam")
        .arg(home_1)
        .arg("--song")
        .arg(song)
        .arg("--dance-with")
        .arg(home_2)
        .output()
        .expect("execute mirror peer beam --song --dance-with")
}

// === T1: --dance-with flag accepted; exit 0 =============================
#[test]
fn t01_dance_with_flag_accepted_exit_zero() {
    let (home_1, home_2) = make_two_peer_homes("exit");
    let song = write_shared_song(home_1.parent().unwrap());
    let out = run_dance(&home_1, &home_2, &song);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(
        out.status.success(),
        "T1: exit 0 required for --song + --dance-with dispatch; \
         stdout=<{stdout}> stderr=<{stderr}>"
    );
}

// === T2: envelope names @dance/@resonance/@cyberpunk/@bauchladen ========
#[test]
fn t02_envelope_names_dance_substrate_authorities() {
    let (home_1, home_2) = make_two_peer_homes("authorities");
    let song = write_shared_song(home_1.parent().unwrap());
    let out = run_dance(&home_1, &home_2, &song);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    for authority in &["@dance", "@resonance", "@cyberpunk", "@bauchladen"] {
        assert!(
            stdout.contains(authority),
            "T2: envelope must name `{authority}` substrate authority (per Mara \
             `417ec25` §3.3 envelope shape); got: <{stdout}>"
        );
    }
}

// === T3: envelope emits kuramoto_order_parameter with parseable f64 =====
#[test]
fn t03_envelope_emits_kuramoto_order_parameter() {
    let (home_1, home_2) = make_two_peer_homes("kuramoto");
    let song = write_shared_song(home_1.parent().unwrap());
    let out = run_dance(&home_1, &home_2, &song);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        stdout.contains("kuramoto_order_parameter:"),
        "T3: envelope must emit `kuramoto_order_parameter:` field (substrate-\
         locked per Mara `94e55eb` shards/song/beat.mirror:456; Mara `9e48710`\
         §2.4 Kuramoto formula); got: <{stdout}>"
    );
    // Extract the value + verify it's a parseable f64 in [0, 1].
    let line = stdout
        .lines()
        .find(|l| l.contains("kuramoto_order_parameter:"))
        .expect("kuramoto_order_parameter line present");
    let value_str = line
        .split("kuramoto_order_parameter:")
        .nth(1)
        .expect("value present after field name")
        .trim();
    let r: f64 = value_str
        .parse()
        .unwrap_or_else(|_| panic!("T3: `{value_str}` must parse as f64"));
    assert!(
        (0.0..=1.0).contains(&r),
        "T3: kuramoto_order_parameter must be in [0, 1]; got: {r}"
    );
}

// === T4: envelope emits aumann_agreement + shared_root_oid ==============
#[test]
fn t04_envelope_emits_aumann_agreement_and_shared_root_oid() {
    let (home_1, home_2) = make_two_peer_homes("aumann");
    let song = write_shared_song(home_1.parent().unwrap());
    let out = run_dance(&home_1, &home_2, &song);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        stdout.contains("aumann_agreement:"),
        "T4: envelope must emit `aumann_agreement:` field (substrate-locked \
         per beat.mirror:456-457; @bauchladen shared-prior convergence \
         witness); got: <{stdout}>"
    );
    let agreement_line = stdout
        .lines()
        .find(|l| l.contains("aumann_agreement:"))
        .expect("aumann_agreement line");
    let value = agreement_line
        .split("aumann_agreement:")
        .nth(1)
        .expect("value")
        .trim();
    assert!(
        value == "true" || value == "false",
        "T4: aumann_agreement value must be true or false; got: <{value}>"
    );

    assert!(
        stdout.contains("shared_root_oid:"),
        "T4: envelope must emit `shared_root_oid:` field (substrate-locked \
         per beat.mirror:493; @bauchladen crystal-OID witness); got: <{stdout}>"
    );
}

// === T5: envelope emits convergence_verdict classifier ==================
#[test]
fn t05_envelope_emits_convergence_verdict() {
    let (home_1, home_2) = make_two_peer_homes("verdict");
    let song = write_shared_song(home_1.parent().unwrap());
    let out = run_dance(&home_1, &home_2, &song);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        stdout.contains("convergence_verdict:"),
        "T5: envelope must emit `convergence_verdict:` classifier (per Mara \
         `417ec25` §5.4 three-way {{converged | dispersed | chimera}}); \
         got: <{stdout}>"
    );
    let verdict_line = stdout
        .lines()
        .find(|l| l.contains("convergence_verdict:"))
        .expect("convergence_verdict line");
    let value = verdict_line
        .split("convergence_verdict:")
        .nth(1)
        .expect("value")
        .trim();
    assert!(
        ["converged", "dispersed", "chimera"].contains(&value),
        "T5: convergence_verdict value must be one of \
         {{converged, dispersed, chimera}}; got: <{value}>"
    );
}
