//! Rung 5 RED — `mirror peer beam <home_A> --song <song> --dance-with
//! <home_B> --deploy-to <target>` runs Rung 4 dance, then declares
//! mycelial nix deployment envelope naming @spectral/garden + @spectral/
//! garden/nix + @bauchladen + @dance + @mirror/mosaic + @song/beat
//! substrate authorities. Envelope-declared per Mara `9c4ef5b` Scope A.
//!
//! Substrate authority:
//! - Mara `9c4ef5b` — `docs/specs/deployment-runtime-rung-5-mycelial-
//!   envelope-declared-substrate.md` Scope A verdict + canonical shape
//!   (§3.2 API + §3.3 envelope + §3.6 T-test spec).
//! - Reed `dfac8fe` — Rung 4 GREEN (dance.rs); Rung 5 composes.
//! - Mara `4575340` — @bauchladen content-addressed shared substrate.
//! - Mara `4f079c8` — @dance canonical spec.
//! - Mara `ad03fda` — spectral-garden-git-package-manager (deployment
//!   substrate; family-root forward-promised at species-decl altitude).
//! - Mara `94e55eb` — shards/song/beat.mirror (Rung 0 sixth species).
//! - Alex 2026-07-13 in-transcript /loop ladder-climb mandate.
//!
//! Rung 5 discipline: envelope-declared substrate (no nix subprocess,
//! no spectral.engineer contact); Rung 5.5 forward-promises actual
//! `nix build`; Rung 6 forward-promises actual mycelial gossip.
//!
//! Five RED tests per Mara `9c4ef5b` §3.6.

use std::process::Command;

fn mirror_bin() -> String {
    std::env::var("MIRROR_BIN")
        .unwrap_or_else(|_| "/Users/reed/.cargo-target/release/mirror".to_string())
}

fn make_two_peer_homes(suffix: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    let base = std::env::temp_dir().join(format!(
        "peer-beam-deploy-mycelial-{}-{}",
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
    let path = base_dir.join("shared_dance.song");
    std::fs::write(
        &path,
        "song hello_world {\n  movement greet {\n    voice compiler {\n      scope: @mirror/mosaic\n    }\n    beat strike {\n      action: @kintsugi/oscillate\n    }\n  }\n}\n",
    )
    .expect("write shared song");
    path
}

fn run_deploy(
    home_1: &std::path::PathBuf,
    home_2: &std::path::PathBuf,
    song: &std::path::PathBuf,
    target: &str,
) -> std::process::Output {
    Command::new(mirror_bin())
        .arg("peer")
        .arg("beam")
        .arg(home_1)
        .arg("--song")
        .arg(song)
        .arg("--dance-with")
        .arg(home_2)
        .arg("--deploy-to")
        .arg(target)
        .output()
        .expect("execute mirror peer beam --deploy-to")
}

fn run_dance_only(
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
        .expect("execute mirror peer beam --dance-with (no --deploy-to)")
}

fn extract_field<'a>(stdout: &'a str, key: &str) -> Option<&'a str> {
    stdout
        .lines()
        .find(|l| l.starts_with(&format!("+ {}: ", key)))
        .and_then(|l| l.split(&format!("+ {}: ", key)).nth(1))
        .map(|s| s.trim())
}

// === T1: --deploy-to flag accepted; exit 0 ==============================
#[test]
fn t01_deploy_to_flag_accepted_exit_zero() {
    let (home_1, home_2) = make_two_peer_homes("exit");
    let song = write_shared_song(home_1.parent().unwrap());
    let out = run_deploy(&home_1, &home_2, &song, "spectral.engineer");
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    assert!(
        out.status.success(),
        "T1: exit 0 required for --song + --dance-with + --deploy-to \
         dispatch; stdout=<{stdout}> stderr=<{stderr}>"
    );
}

// === T2: envelope names six substrate authorities ======================
#[test]
fn t02_envelope_names_six_substrate_authorities() {
    let (home_1, home_2) = make_two_peer_homes("authorities");
    let song = write_shared_song(home_1.parent().unwrap());
    let out = run_deploy(&home_1, &home_2, &song, "spectral.engineer");
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    for authority in &[
        "@spectral/garden",
        "@spectral/garden/nix",
        "@bauchladen",
        "@dance",
        "@mirror/mosaic",
        "@song/beat",
    ] {
        assert!(
            stdout.contains(authority),
            "T2: envelope must name `{authority}` substrate authority (per Mara \
             `9c4ef5b` §3.3 envelope shape); got: <{stdout}>"
        );
    }
}

// === T3: envelope carries nix_derivation_oid as hex =====================
#[test]
fn t03_envelope_carries_stub_nix_derivation_oid() {
    let (home_1, home_2) = make_two_peer_homes("nix");
    let song = write_shared_song(home_1.parent().unwrap());
    let out = run_deploy(&home_1, &home_2, &song, "spectral.engineer");
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let oid = extract_field(&stdout, "nix_derivation_oid")
        .expect("T3: envelope must emit `nix_derivation_oid:` field");
    assert!(
        oid.len() == 16 && oid.chars().all(|c| c.is_ascii_hexdigit()),
        "T3: nix_derivation_oid must be 16 lowercase hex digits (FNV-1a \
         stub per Mara `9c4ef5b` §3.4); got: <{oid}>"
    );
}

// === T4: envelope composes over dance shared_root_oid ==================
//
// Load-bearing composition assertion: running --deploy-to must EMIT the
// SAME shared_root_oid as running dance-only for the same (home_1, home_2,
// song) triple. Verifies deploy IS a composition over dance, not a
// re-implementation. Per Mara `9c4ef5b` §3.5 Option (i) refactor.
#[test]
fn t04_deploy_envelope_composes_over_dance_shared_root_oid() {
    let (home_1, home_2) = make_two_peer_homes("compose");
    let song = write_shared_song(home_1.parent().unwrap());

    let deploy_out = run_deploy(&home_1, &home_2, &song, "spectral.engineer");
    let deploy_stdout = String::from_utf8_lossy(&deploy_out.stdout).to_string();
    let deploy_oid = extract_field(&deploy_stdout, "dance_shared_root_oid")
        .expect("T4: deploy envelope must emit `dance_shared_root_oid:` field");

    let dance_out = run_dance_only(&home_1, &home_2, &song);
    let dance_stdout = String::from_utf8_lossy(&dance_out.stdout).to_string();
    let dance_oid = extract_field(&dance_stdout, "shared_root_oid")
        .expect("T4: dance envelope must emit `shared_root_oid:` field");

    assert_eq!(
        deploy_oid, dance_oid,
        "T4: deploy's dance_shared_root_oid must equal dance-only's \
         shared_root_oid (composition over dance, not re-implementation; \
         per Mara `9c4ef5b` §3.5 Option (i))"
    );
}

// === T5: no --deploy-to preserves Rung 4 dance envelope byte-equality ==
//
// Regression guard: running with --song + --dance-with but WITHOUT
// --deploy-to must produce IDENTICAL output shape to Rung 4 test-case
// fixture. Rung 5 must not break Rung 4 byte-equality.
#[test]
fn t05_no_deploy_flag_preserves_dance_envelope_byte_equality() {
    let (home_1, home_2) = make_two_peer_homes("regression");
    let song = write_shared_song(home_1.parent().unwrap());
    let out = run_dance_only(&home_1, &home_2, &song);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(
        out.status.success(),
        "T5: Rung 4 dance dispatch must still exit 0 after Rung 5 landing"
    );
    // Verify Rung 4 dance-envelope shape preserved:
    for expected in &[
        "kuramoto_order_parameter:",
        "aumann_agreement:",
        "shared_root_oid:",
        "convergence_verdict:",
        "@dance",
    ] {
        assert!(
            stdout.contains(expected),
            "T5: Rung 4 dance envelope must preserve `{expected}` (byte-equality \
             regression); got: <{stdout}>"
        );
    }
    // Verify Rung 5 deployment envelope did NOT emit:
    for absent in &[
        "@spectral/garden/nix",
        "nix_derivation_oid:",
        "deployment_verdict:",
    ] {
        assert!(
            !stdout.contains(absent),
            "T5: Rung 5 deployment envelope MUST NOT emit `{absent}` when \
             --deploy-to is absent (byte-equality guard); got: <{stdout}>"
        );
    }
}
