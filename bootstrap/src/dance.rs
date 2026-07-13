//! @dance runtime — Rung 4 GREEN per Mara `417ec25` Scope B narrowed to
//! coherence phase-lock.
//!
//! Substrate authority:
//! - Mara `417ec25` — `docs/specs/dance-runtime-rung-4-multi-peer-
//!   coherence-phase-lock.md` Scope B narrowed verdict + canonical shape
//!   (§3.2 API + §3.3 envelope + §5.4 convergence classifier).
//! - Mara `94e55eb` — `shards/song/beat.mirror:453-457` Rung 4 substrate
//!   reservation: multi-peer @dance coupling on shared beat via this
//!   module.
//! - Reed `8e6e517` — cybernetic_coherence = λ₀(Δ_F) Path B annotation.
//! - Mara `4f079c8` — @dance canonical spec.
//! - Mara `9e48710` — @resonance Kuramoto coupling formalization §2.4.
//! - Reed `c36fbf5` (Rung 1) + `70766c3` (Rung 2) + `0cc4e11` (Rung 3) —
//!   song-runtime discharge chain @dance composes over.
//! - Alex 2026-07-13 in-transcript /loop ladder-climb mandate.
//!
//! Rung 4 discipline: 2-peer (N > 2 forward-promised to Rung 5);
//! coherence-as-stub (actual λ₀(Δ_F) forward-promised to Rung 4.5);
//! Kuramoto r as summary-statistic (not ODE integrator; per @io boundary
//! discipline the physical Kuramoto model belongs at @io altitude per
//! Mara `9e48710` §3.1). Two peers reading the SAME shared `.song` file
//! is Aumann agreement under content-addressed common prior; envelope
//! reports the metrics per substrate-locked field names.

use crate::Ctx;

/// Shared dance-state computed by `compute_dance_state`; consumed by both
/// `execute_dance` (Rung 4 envelope emitter) and `execute_deploy` (Rung 5
/// composition per Mara `9c4ef5b` §3.5 Option (i) refactor). Rung 5's
/// deploy runtime composes over dance by reading `shared_root_oid` as
/// the common-prior anchor for `stub_nix_derivation_oid`.
pub struct DanceState {
    pub phase_1: f64,
    pub phase_2: f64,
    pub kuramoto_r: f64,
    pub aumann: bool,
    pub shared_root_oid: String,
    pub verdict: &'static str,
}

/// Compute the Rung 4 dance state without emitting an envelope. Both
/// `execute_dance` (Rung 4) and `execute_deploy` (Rung 5) call this
/// helper; the former emits dance envelope, the latter composes over
/// shared_root_oid into deployment envelope.
pub fn compute_dance_state(
    peer_home_1: &str,
    peer_home_2: &str,
    song_bytes: &[u8],
) -> DanceState {
    let phase_1 = stub_phase_for_peer(peer_home_1, song_bytes);
    let phase_2 = stub_phase_for_peer(peer_home_2, song_bytes);
    let kuramoto_r = kuramoto_order_parameter_two_peer(phase_1, phase_2);
    let shared_root_oid = stub_shared_root_oid(song_bytes);
    let aumann = true;
    let verdict = if kuramoto_r >= 0.9 && aumann {
        "converged"
    } else if kuramoto_r < 0.5 {
        "dispersed"
    } else {
        "chimera"
    };
    DanceState {
        phase_1,
        phase_2,
        kuramoto_r,
        aumann,
        shared_root_oid,
        verdict,
    }
}

pub fn execute_dance(
    peer_home_1: &str,
    peer_home_2: &str,
    spec_path_1: &std::path::Path,
    spec_path_2: &std::path::Path,
    song_path: &str,
    _ctx: &Ctx,
) -> i32 {
    if !spec_path_1.exists() {
        eprintln!("dance: mirror.spec not found at peer_home_1: {}", peer_home_1);
        return 1;
    }
    if !spec_path_2.exists() {
        eprintln!("dance: mirror.spec not found at peer_home_2: {}", peer_home_2);
        return 1;
    }
    let song_present = std::path::Path::new(song_path).exists();
    if !song_present {
        eprintln!("dance: song file not found: {}", song_path);
        return 1;
    }

    let song_bytes = match std::fs::read(song_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("dance: cannot read song file: {}", e);
            return 1;
        }
    };

    let state = compute_dance_state(peer_home_1, peer_home_2, &song_bytes);

    println!(
        "@@ dance @dance via 2 × (@song × @kintsugi/oscillate) coupled at cybernetic_coherence altitude (Rung 4) @@"
    );
    println!("+ peer_home_1: {}", peer_home_1);
    println!("+ peer_home_2: {}", peer_home_2);
    println!("+ song_path: {}", song_path);
    println!("+ phase_1: {:.6}", state.phase_1);
    println!("+ phase_2: {:.6}", state.phase_2);
    println!("+ kuramoto_order_parameter: {:.6}", state.kuramoto_r);
    println!("+ aumann_agreement: {}", state.aumann);
    println!("+ shared_root_oid: {}", state.shared_root_oid);
    println!("+ convergence_verdict: {}", state.verdict);
    println!(
        "+ coherence_altitude: stub (Rung 4.5 forward-promise: λ₀(Δ_F) per Reed `8e6e517`)"
    );
    println!("+ dance_authority: @dance (Mara `4f079c8` canonical spec; Path C recognition)");
    println!("+ resonance_authority: @resonance (Mara `9e48710`; Kuramoto coupling ancestor)");
    println!(
        "+ cyberpunk_authority: @cyberpunk (Reed `8e6e517` cybernetic_coherence = λ₀(Δ_F))"
    );
    println!(
        "+ bauchladen_authority: @bauchladen (Mara `4575340`; content-addressed shared prior)"
    );
    println!("+ ladder_rung: 4 (Reed GREEN discharging Mara `417ec25` Scope B narrowed)");
    println!(
        "+ substrate_authority: @dance + @resonance + @cyberpunk + @bauchladen (Rung 4 minimum viable)"
    );

    0
}

/// Rung 4 stub coherence: hash the (peer_home, song_bytes) pair; project
/// into a phase in [0, 2π). Deterministic per input; distinct peer_homes
/// yield distinct phases. Rung 4.5 forward-promise: replace with per-
/// beat λ₀(Δ_F) sequence from peer-runtime introspection.
pub(crate) fn stub_phase_for_peer(peer_home: &str, song_bytes: &[u8]) -> f64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in peer_home.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    for b in song_bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    let fraction = (h as f64) / (u64::MAX as f64);
    fraction * std::f64::consts::TAU
}

/// Two-peer specialization of Mara `9e48710` §2.4 Kuramoto formula:
/// r · e^(iψ) = (1/2) (e^(iθ₁) + e^(iθ₂)). r ∈ [0, 1]; r = 1 iff phases
/// align. Distinct peer_homes with same song produce different phases,
/// but they share substrate; r reflects the phase-alignment observed.
pub(crate) fn kuramoto_order_parameter_two_peer(theta_1: f64, theta_2: f64) -> f64 {
    let re = (theta_1.cos() + theta_2.cos()) * 0.5;
    let im = (theta_1.sin() + theta_2.sin()) * 0.5;
    (re * re + im * im).sqrt()
}

/// Rung 4 stub shared_root_oid: hex-encoded FNV-1a hash of the shared
/// song bytes. Under content-addressed persistence (@bauchladen), both
/// peers reading identical song bytes would emit crystals with this
/// OID; T4 asserts the field's presence, not its cryptographic content-
/// hash (Rung 5+ upgrades to blake3 crystal-OID per @bauchladen).
pub(crate) fn stub_shared_root_oid(song_bytes: &[u8]) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in song_bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{:016x}", h)
}
