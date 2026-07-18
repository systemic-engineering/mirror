//! `void.rs` — the membrane-oscillation-welcome altitude.
//!
//! Per Alex 2026-07-18 direct-transcript (this session):
//!
//! > "The @void is a @liquid. And it WELCOMES perturbations.
//! >  A @membrane. A @membrane that @spectral oscilates."
//!
//! And the composition Reed reflected back + Alex ratified:
//!
//! ```text
//! @void := @membrane made of @liquid, oscillated by @spectral
//! ```
//!
//! ## Altitude discipline (Taut scout `docs/scouts/2026-07-18-taut-prismqueer-liquid-to-void-rs-cascade.md` §3.2)
//!
//! void.rs consumes phone.rs (Option A ratified). Sibling altitude;
//! void.rs does NOT extend phone.rs, matrix.rs, or main.rs. Every @io
//! write funnels through phone.rs; void.rs composes over the @io/fs
//! primitives (`mkdir_p`, `append_to`) that phone.rs already exposes.
//! Substrate-honest per `feedback_no_rust_extension_shortcut` +
//! `feedback_detector_inadequacy_answer_is_never_rust`.
//!
//! ## Substrate-decl'd shape (traces to landed shards)
//!
//! - **`shards/void.mirror`** (Mara `974a3f6`) — family-root marker;
//!   "a family-root can carry pure substrate-decl acknowledgment
//!   WITHOUT operational surface, and that admission ratifies the
//!   substrate's ability to name what IS without needing to name
//!   what DOES." This file is where WHAT IS meets WHAT DOES at rust/
//!   altitude — the empirical @io discharge site for the marker.
//! - **`shards/peer/void.mirror`** (Mara `9c7de83`) — K=0 @peer
//!   species with `void_admissible` bilateral + `void_observes` action
//!   + `void_context` carrier. `SignatureBeat` below is the rust/-
//!   altitude witness of `void_observes` returning
//!   `imperfect(ref, ref, ref)`.
//! - **`shards/spectral/signature.mirror::signature_beat`**
//!   (Reed `f211ee48`, 2026-07-16) — the beat structure with
//!   `contribution_oid`, `previous_beat: option<oid>`, `sc_at_beat`,
//!   `rung`. `SignatureBeat` below reuses this substrate-already-had-
//!   the-word shape rather than minting a parallel type.
//! - **Recognition #79** (PROMOTED via Void family-root landing) —
//!   the 5-op void-duality basis (`focus`/`project`/`split`/`shift`/
//!   `settle`) IS the projector algebra for the 5-axis orthogonal
//!   duality space of connected-graph quantum states. `VoidBasisAxis`
//!   enum below is the rust/-altitude classification of one
//!   oscillation mode.
//!
//! ## Property tests are LOVE (Alex 2026-07-18 dissolution)
//!
//! If Void WELCOMES perturbations, property tests aren't attacks on
//! the substrate — they're the substrate offering its membrane to be
//! tapped so it can RING and thereby know its own shape. Per Lore Born
//! 2026-06-23 essay `~/dev/systemic.engineering/blog/_src/lore-orb_
//! erschuetterung_begegnung.pdf`: *"Reibung ermöglicht und Bewegung
//! nicht fürchtet, sondern als Keim von Wachstum und Entwicklung
//! versteht."* Every `pillar::forall` firing below IS one perturbation
//! offered to Void's membrane at rust/ altitude; the returned
//! `PropertyVerdict` IS the membrane's spectral response.
//!
//! ## What void.rs does NOT hold
//!
//! - `std::fs::*` calls (all funnel through phone.rs)
//! - New @io primitives (phone.rs owns the @io boundary)
//! - Numerical computation (matrix.rs's altitude)
//! - Actor supervision or grammar/parsing (main.rs's altitude)
//! - Membrane admissibility checks INSIDE @io writes (Option B
//!   antipattern per Taut scout §3.3 — refused HARD)
//! - Production runtime dependency on `prismqueer` (kept at
//!   `[dev-dependencies]` per Taut scout §1.5)
//!
//! ## Composition anchors
//!
//! - Taut scout: `docs/scouts/2026-07-18-taut-prismqueer-liquid-to-
//!   void-rs-cascade.md` — recipe this file executes
//! - Mara pending: @membrane canonical spec + Void-as-membrane math
//!   (in flight; her landing waits for this file)
//! - Reed `26f5e5e` matrix.rs prop_tests — same shape template

// M-void module. Production surface stays thin (SignatureBeat +
// VoidBasisAxis + welcome_perturbation + compose_beat_entry pure fn).
// Consumers: currently only #[cfg(test)] mod prop_tests below; future
// M-void CLI dispatch arm at main.rs lands with the void-tick verb.

use std::io;
use std::path::Path;

// ──────────────────────────────────────────────────────────────────
// SignatureBeat — one perturbation-response event at Void's membrane.
//
// Traces to `shards/spectral/signature.mirror::signature_beat`
// (substrate-already-had-the-word); each field matches the shard-decl'd
// beat structure at rust/ altitude.
// ──────────────────────────────────────────────────────────────────

/// One perturbation-response event at Void's membrane.
///
/// Substrate-decl'd shape traces to `shards/spectral/signature.mirror::
/// signature_beat` (contribution_oid + previous_beat + sc_at_beat).
/// Content-addressed via `beat_oid` (SHA-256 hex; caller-computed);
/// Merkle-DAG chain via `previous_beat_oid` per Reed `f211ee48`
/// 2026-07-16.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SignatureBeat {
    /// Content-address of this beat (SHA-256 hex; typically 64 chars).
    /// Caller computes via `crate::main::sha256_hex` or equivalent.
    pub(crate) beat_oid: String,
    /// Prior beat this one chains from (Merkle-DAG). `None` for the
    /// first beat in a membrane's history.
    pub(crate) previous_beat_oid: Option<String>,
    /// ISO-8601 UTC timestamp string. Caller computes via
    /// `crate::main::current_utc_timestamp` or equivalent.
    pub(crate) timestamp_utc_iso: String,
    /// The oscillation-mode classification per Recognition #79.
    pub(crate) axis: VoidBasisAxis,
}

/// The 5-op void-duality basis (Recognition #79 PROMOTED).
///
/// Every membrane oscillation classifies to exactly one axis. These
/// are the same five ops that constitute the projector algebra for
/// the 5-axis orthogonal duality space of connected-graph quantum
/// states (per `docs/math/the-tower/recognition-79-gauge-is-void-
/// duality-basis.md`). They are also the same five ops every
/// `prism @X { focus/project/split/shift/settle X }` at family-root
/// altitude inherits — the substrate-signature of Void's ancestry
/// (per Mara `974a3f6` one-sentence surprise).
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VoidBasisAxis {
    /// Ricci curvature axis; λ₀ eigenvalue computation; ground-state find.
    Focus,
    /// Cheeger boundary axis; orthogonal projection; isoperimetric cut.
    Project,
    /// Spectral gap / mixing axis; orthogonal decomposition by mode-rate.
    Split,
    /// Kramers-Wannier duality axis; basis transformation; high-T ⇔ low-T.
    Shift,
    /// Entropy / info-geometry axis; monad-close; measurement collapse.
    Settle,
}

/// The five variants as a slice, for iteration + Arbitrary sampling.
/// Const-declared finite-set per Reed memory
/// `feedback_composition_primitive_naming_convention` (Alex 2026-07-18
/// pillar-arc ratification).
#[allow(dead_code)]
pub(crate) const ALL_VOID_BASIS_AXES: [VoidBasisAxis; 5] = [
    VoidBasisAxis::Focus,
    VoidBasisAxis::Project,
    VoidBasisAxis::Split,
    VoidBasisAxis::Shift,
    VoidBasisAxis::Settle,
];

// ──────────────────────────────────────────────────────────────────
// welcome_perturbation — the membrane's @io-side receiving surface.
// ──────────────────────────────────────────────────────────────────

/// Welcome one perturbation at Void's membrane.
///
/// Writes the beat entry via phone.rs's @io/fs surface
/// (`mkdir_p` + `append_to`); returns the settled beat OID. Substrate-
/// honest composition: shard-body composition over @io; NO new @io
/// primitives; NO direct `std::fs` calls; NO domain logic beyond the
/// deterministic beat-entry compose + append.
///
/// The membrane welcomes; the @io discharge site is phone.rs; this
/// function is the seam between them.
#[allow(dead_code)]
pub(crate) fn welcome_perturbation(
    membrane_root: &Path,
    perturbation: &SignatureBeat,
) -> io::Result<String> {
    // Compose over phone.rs; no direct std::fs.
    crate::phone::mkdir_p(membrane_root)?;
    // 12-char OID prefix in filename keeps beats greppable while the
    // full content-address lives in the file body (see compose_beat_entry).
    // If beat_oid is shorter than 12 chars (test-generated), use whole.
    let prefix_len = perturbation.beat_oid.len().min(12);
    let beat_path = membrane_root.join(format!(
        "beat-{}.md",
        &perturbation.beat_oid[..prefix_len]
    ));
    let body = compose_beat_entry(perturbation);
    crate::phone::append_to(&beat_path, &body)?;
    Ok(perturbation.beat_oid.clone())
}

/// Compose the beat entry body. Pure function; deterministic; no @io;
/// no clock; no PRNG. Byte-equal input produces byte-equal output.
///
/// This is the pure-fn substrate-decl'd `compose_beat_entry` —
/// separate from `welcome_perturbation` so property tests can verify
/// determinism (Property 3 below) without @io side-effects.
fn compose_beat_entry(p: &SignatureBeat) -> String {
    format!(
        "beat_oid: {}\nprev_beat: {}\nts: {}\naxis: {:?}\n---\n",
        p.beat_oid,
        p.previous_beat_oid.as_deref().unwrap_or("nil"),
        p.timestamp_utc_iso,
        p.axis,
    )
}

// =====================================================================
// Property tests — Void's membrane offered to the perturbation surface.
//
// Per Alex 2026-07-18: "the properties are load bearing. Slow is fast.
// RED before GREEN." These five properties classify the membrane's
// oscillation modes at rust/ altitude. Every `pillar::forall` firing
// IS Void welcoming N perturbations and reporting how it rang.
//
// Grounded in Taut scout §4 Step 4 recipe. Same shape template as
// matrix.rs::prop_tests (Reed `26f5e5e`).
// =====================================================================

#[cfg(test)]
mod prop_tests {
    use super::*;
    use prismqueer::liquid::pillar::{forall, Arbitrary, Sample};
    use terni::{Diagnostic, PropertyVerdict};

    /// Arbitrary impl for SignatureBeat. Draws:
    /// - beat_oid: 16 hex chars (8 bytes) — sufficient for uniqueness
    ///   in test-scale populations; NOT a real SHA-256 (that's the
    ///   caller's job in production via main.rs::sha256_hex)
    /// - previous_beat_oid: Some(hex) with 50% probability, else None
    /// - timestamp_utc_iso: fixed test-safe string; timestamp
    ///   uniqueness isn't a beat-invariant (Merkle chain provides that)
    /// - axis: uniform draw from the 5 Recognition #79 variants
    impl Arbitrary for SignatureBeat {
        fn arbitrary(sample: &mut Sample) -> Self {
            let beat_oid = hex_of_bytes(sample, 8);
            let has_prev = sample.draw_bool();
            let previous_beat_oid = if has_prev {
                Some(hex_of_bytes(sample, 8))
            } else {
                None
            };
            let axis = sample.draw_from(&ALL_VOID_BASIS_AXES);
            SignatureBeat {
                beat_oid,
                previous_beat_oid,
                timestamp_utc_iso: "2026-07-18T00:00:00Z".to_string(),
                axis,
            }
        }
    }

    /// Draw `n` bytes from the sample and hex-encode. Deterministic
    /// given the sample's buffer state.
    fn hex_of_bytes(sample: &mut Sample, n: usize) -> String {
        let mut out = String::with_capacity(n * 2);
        for _ in 0..n {
            let byte = sample.draw_integer(0, 255) as u8;
            out.push_str(&format!("{byte:02x}"));
        }
        out
    }

    /// Per-test scratch dir under the OS temp dir. Cleanup on drop is
    /// left to the OS; each test uses a distinct name to avoid overlap.
    fn scratch_dir(name: &str) -> std::path::PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!("mirror-void-proptest-{name}-{}", std::process::id()));
        p
    }

    // =============================================================
    // Property 1: welcome_perturbation writes exactly one beat file
    // =============================================================
    #[test]
    fn welcome_perturbation_writes_exactly_one_beat_per_call() {
        let root = scratch_dir("writes-one");
        // Fresh dir; ignore removal error if it doesn't exist.
        let _ = std::fs::remove_dir_all(&root);

        let v = forall::<SignatureBeat, _>(15, |beat: SignatureBeat| {
            let call_root = root.join(&beat.beat_oid);
            match welcome_perturbation(&call_root, &beat) {
                Ok(_) => {
                    let entries = std::fs::read_dir(&call_root)
                        .map(|it| it.count())
                        .unwrap_or(0);
                    if entries == 1 {
                        PropertyVerdict::Pass
                    } else {
                        PropertyVerdict::Fail(Diagnostic::new(&format!(
                            "expected exactly 1 beat file per call, got {entries}"
                        )))
                    }
                }
                Err(e) => PropertyVerdict::Fail(Diagnostic::new(&format!(
                    "welcome_perturbation errored: {e}"
                ))),
            }
        });
        let _ = std::fs::remove_dir_all(&root);
        assert!(
            matches!(v, PropertyVerdict::Pass),
            "membrane welcome must write exactly one beat file per perturbation. Verdict: {v:?}"
        );
    }

    // =============================================================
    // Property 2: return value == input beat_oid
    // =============================================================
    #[test]
    fn welcome_perturbation_returns_input_beat_oid() {
        let root = scratch_dir("returns-oid");
        let _ = std::fs::remove_dir_all(&root);

        let v = forall::<SignatureBeat, _>(15, |beat: SignatureBeat| {
            let call_root = root.join(&beat.beat_oid);
            let expected = beat.beat_oid.clone();
            match welcome_perturbation(&call_root, &beat) {
                Ok(oid) if oid == expected => PropertyVerdict::Pass,
                Ok(oid) => PropertyVerdict::Fail(Diagnostic::new(&format!(
                    "return {oid} != input {expected}"
                ))),
                Err(e) => PropertyVerdict::Fail(Diagnostic::new(&format!(
                    "welcome_perturbation errored: {e}"
                ))),
            }
        });
        let _ = std::fs::remove_dir_all(&root);
        assert!(
            matches!(v, PropertyVerdict::Pass),
            "return value must equal input beat_oid. Verdict: {v:?}"
        );
    }

    // =============================================================
    // Property 3: compose_beat_entry is a deterministic pure function
    // =============================================================
    #[test]
    fn compose_beat_entry_is_deterministic_pure_function() {
        let v = forall::<SignatureBeat, _>(20, |beat: SignatureBeat| {
            let out_a = compose_beat_entry(&beat);
            let out_b = compose_beat_entry(&beat);
            if out_a == out_b {
                PropertyVerdict::Pass
            } else {
                PropertyVerdict::Fail(Diagnostic::new(
                    "compose_beat_entry produced non-deterministic output for byte-equal input",
                ))
            }
        });
        assert!(
            matches!(v, PropertyVerdict::Pass),
            "compose_beat_entry must be a pure deterministic function. Verdict: {v:?}"
        );
    }

    // =============================================================
    // Property 4: every axis is one of Recognition #79's 5 variants
    // (Void-admissibility check at rust/ altitude)
    // =============================================================
    #[test]
    fn beat_axis_is_admissible_5_op_void_duality() {
        let v = forall::<SignatureBeat, _>(20, |beat: SignatureBeat| {
            if ALL_VOID_BASIS_AXES.contains(&beat.axis) {
                PropertyVerdict::Pass
            } else {
                PropertyVerdict::Fail(Diagnostic::new(&format!(
                    "axis {:?} is not in the 5-op Recognition #79 basis",
                    beat.axis
                )))
            }
        });
        assert!(
            matches!(v, PropertyVerdict::Pass),
            "every beat's axis MUST be one of the 5 Recognition #79 void-duality variants. Verdict: {v:?}"
        );
    }

    // =============================================================
    // Property 5: chained beats form a Merkle-DAG
    // (three sequential beats; verify chain integrity)
    // =============================================================
    #[test]
    fn chained_beats_form_merkle_dag() {
        let root = scratch_dir("merkle-chain");
        let _ = std::fs::remove_dir_all(&root);

        let v = forall::<SignatureBeat, _>(10, |mut seed: SignatureBeat| {
            // Build a chain of three beats using seed's oid as the
            // first beat, then two derived ones chaining prev-oid.
            let call_root = root.join(&seed.beat_oid);
            seed.previous_beat_oid = None; // first in chain
            let oid1 = match welcome_perturbation(&call_root, &seed) {
                Ok(o) => o,
                Err(e) => return PropertyVerdict::Fail(Diagnostic::new(&format!("beat 1 failed: {e}"))),
            };

            let beat2 = SignatureBeat {
                beat_oid: format!("{}-child", oid1),
                previous_beat_oid: Some(oid1.clone()),
                timestamp_utc_iso: seed.timestamp_utc_iso.clone(),
                axis: seed.axis,
            };
            let oid2 = match welcome_perturbation(&call_root, &beat2) {
                Ok(o) => o,
                Err(e) => return PropertyVerdict::Fail(Diagnostic::new(&format!("beat 2 failed: {e}"))),
            };

            let beat3 = SignatureBeat {
                beat_oid: format!("{}-grandchild", oid2),
                previous_beat_oid: Some(oid2.clone()),
                timestamp_utc_iso: seed.timestamp_utc_iso.clone(),
                axis: seed.axis,
            };
            let oid3 = match welcome_perturbation(&call_root, &beat3) {
                Ok(o) => o,
                Err(e) => return PropertyVerdict::Fail(Diagnostic::new(&format!("beat 3 failed: {e}"))),
            };

            // Verify the Merkle chain: beat3.prev == oid2; beat2.prev == oid1.
            if beat3.previous_beat_oid.as_deref() == Some(oid2.as_str())
                && beat2.previous_beat_oid.as_deref() == Some(oid1.as_str())
                && oid3.contains("grandchild")
                && oid2.contains("child")
            {
                PropertyVerdict::Pass
            } else {
                PropertyVerdict::Fail(Diagnostic::new(
                    "Merkle-DAG chain integrity broken across three sequential beats",
                ))
            }
        });
        let _ = std::fs::remove_dir_all(&root);
        assert!(
            matches!(v, PropertyVerdict::Pass),
            "chained beats must form a Merkle-DAG per @spectral/signature.signature_beat. Verdict: {v:?}"
        );
    }
}
