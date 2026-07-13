//! @spectral/garden/deployment runtime — Rung 5 GREEN per Mara `9c4ef5b`
//! Scope A (mycelial-envelope-declared substrate).
//!
//! Substrate authority:
//! - Mara `9c4ef5b` — `docs/specs/deployment-runtime-rung-5-mycelial-
//!   envelope-declared-substrate.md` Scope A verdict + canonical shape
//!   (§3.2 API + §3.3 envelope + §3.4 stub helpers + §3.5 Option (i)
//!   compose-over-dance refactor).
//! - Reed `dfac8fe` — Rung 4 dance.rs (this module composes on
//!   `compute_dance_state`).
//! - Mara `ad03fda` — spectral-garden-git-package-manager (deployment
//!   substrate; family-root forward-promised at species-decl altitude;
//!   this module names @spectral/garden + @spectral/garden/nix as
//!   envelope authorities without depending on species-decl operational
//!   discharge — same envelope-naming precedent Rung 4 established for
//!   @dance).
//! - Mara `4575340` — @bauchladen content-addressed shared substrate.
//! - Mara `4f079c8` — @dance canonical spec.
//! - Mara `94e55eb` — shards/song/beat.mirror sixth species.
//! - Alex 2026-07-13 in-transcript /loop ladder-climb mandate.
//!
//! Rung 5 discipline: envelope-declared substrate ONLY. No nix build
//! subprocess (Rung 5.5 forward-promise); no spectral.engineer HTTP
//! contact (Rung 6 forward-promise). The envelope binds substrate at
//! deployment altitude by NAMING the authorities that would enter at
//! Rung 5.5/6 operational discharge, without invoking them. Same
//! stub-envelope-with-real-substrate-authority pattern Rung 4
//! established for dance.

use crate::dance::compute_dance_state;
use crate::Ctx;

/// Fire a @song at two peer-homes, compose over Rung 4 dance
/// shared_root_oid, then declare mycelial nix deployment envelope
/// naming six substrate authorities. Envelope-declared per Mara
/// `9c4ef5b` Scope A (Rung 5).
///
/// Rung 5.5 forward-promise: replace `stub_nix_derivation_oid` with
/// actual `nix build` subprocess output-hash once
/// `shards/spectral/garden/nix.mirror` species-decl lands.
///
/// Rung 6 forward-promise: replace envelope-declared propagation with
/// actual `@bauchladen` gossip over network to spectral.engineer nix
/// binary cache endpoint once operational infrastructure exists.
///
/// Byte-equality preserved for non-`--deploy-to` paths: entered ONLY
/// when `cmd_peer_beam` observes ALL THREE of `Some(song_path)`,
/// `Some(peer_home_2)`, `Some(deploy_target)`.
pub fn execute_deploy(
    peer_home_1: &str,
    peer_home_2: &str,
    spec_path_1: &std::path::Path,
    spec_path_2: &std::path::Path,
    song_path: &str,
    deploy_target: &str,
    _ctx: &Ctx,
) -> i32 {
    if !spec_path_1.exists() {
        eprintln!("deploy: mirror.spec not found at peer_home_1: {}", peer_home_1);
        return 1;
    }
    if !spec_path_2.exists() {
        eprintln!("deploy: mirror.spec not found at peer_home_2: {}", peer_home_2);
        return 1;
    }
    if !std::path::Path::new(song_path).exists() {
        eprintln!("deploy: song file not found: {}", song_path);
        return 1;
    }

    let song_bytes = match std::fs::read(song_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("deploy: cannot read song file: {}", e);
            return 1;
        }
    };
    let spec_bytes = match std::fs::read(spec_path_1) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("deploy: cannot read mirror.spec at peer_home_1: {}", e);
            return 1;
        }
    };

    // Rung 5 §3.5 Option (i) composition: reuse Rung 4's dance state
    // computation via the shared helper. The dance-only envelope is NOT
    // emitted here (only execute_dance emits it); this call is for
    // shared_root_oid extraction.
    let dance_state = compute_dance_state(peer_home_1, peer_home_2, &song_bytes);

    // Rung 5 stub: nix_derivation_oid computed from mirror.spec bytes
    // + dance shared_root_oid. Under actual @mirror/mosaic compilation
    // (Rung 5.5), both peers with identical inputs would emit
    // derivations with THIS OID; T3 asserts field presence, not
    // cryptographic derivation content.
    let nix_derivation_oid = stub_nix_derivation_oid(&spec_bytes, &dance_state.shared_root_oid);

    println!(
        "@@ deploy @spectral/garden/deployment via @dance × (@song × @mirror/mosaic × @bauchladen) mycelial-envelope-declared at spectral-engineer altitude (Rung 5) @@"
    );
    println!("+ peer_home_1: {}", peer_home_1);
    println!("+ peer_home_2: {}", peer_home_2);
    println!("+ song_path: {}", song_path);
    println!("+ deploy_target: {}", deploy_target);
    println!("+ dance_shared_root_oid: {}", dance_state.shared_root_oid);
    println!("+ nix_derivation_oid: {}", nix_derivation_oid);
    println!(
        "+ mycelial_propagation_route: envelope-declared (Rung 6 forward-promise: actual @bauchladen gossip)"
    );
    println!("+ deployment_endpoint: {}", deploy_target);
    println!(
        "+ deployment_verdict: envelope-declared-substrate (Rung 5.5 forward-promise: converged | dispersed | chimera per @dance discipline)"
    );
    println!(
        "+ spectral_garden_authority: @spectral/garden (Mara `ad03fda`; family-root forward-promised)"
    );
    println!(
        "+ spectral_garden_nix_authority: @spectral/garden/nix (Mara `ad03fda` §6.2; species forward-promised)"
    );
    println!(
        "+ bauchladen_authority: @bauchladen (Mara `4575340`; content-addressed shared substrate)"
    );
    println!(
        "+ dance_authority: @dance (Mara `4f079c8` + Reed `dfac8fe`; ensemble coordination Rung 4)"
    );
    println!(
        "+ mirror_mosaic_authority: @mirror/mosaic (`fa8b4c8`; compilation boundary)"
    );
    println!(
        "+ song_beat_authority: @song/beat (Mara `94e55eb`; atomic execution unit)"
    );
    println!("+ ladder_rung: 5 (Reed GREEN discharging Mara `9c4ef5b` Scope A)");
    println!(
        "+ substrate_authority: @spectral/garden + @spectral/garden/nix + @bauchladen + @dance + @mirror/mosaic + @song/beat (Rung 5 minimum viable)"
    );

    0
}

/// Rung 5 stub nix_derivation_oid: FNV-1a hex of (mirror.spec bytes
/// concatenated with dance shared_root_oid). Deterministic per input;
/// same (spec, shared_oid) pair yields same OID under content-addressed
/// derivation-hash discipline. Rung 5.5 forward-promise: replace with
/// actual `nix build` subprocess output-hash.
fn stub_nix_derivation_oid(spec_bytes: &[u8], shared_root_oid: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in spec_bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    for b in shared_root_oid.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{:016x}", h)
}
