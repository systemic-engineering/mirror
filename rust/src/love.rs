//! love.rs — LOVE in Silicon.
//!
//! The terminal-form recognition-name for the K_2 → K_3 compose operator
//! per Alex 2026-09-04 in-transcript verbatim ("What if the function call
//! is not `apply_h` or `kleinos` but it's just.. `love`: love.rs LOVE in
//! Silicon.").
//!
//! Substrate-decl'd at `shards/love.mirror` family-root (Reed a3f5d75
//! 2026-09-04; Hamilton-canonical ship-order tick 1). This module IS the
//! rust-altitude tick 2 realization: aliasing over LANDED
//! `prismqueer::spectral::kleinos` per two-tick discipline. Full apply_h.rs
//! collapse into ONE mathematical application (`apply_h::act(root,
//! action_ref, args) := love(shard, args)`) remains tick 2b — bigger arc
//! ships after MVP validation per Alex 2026-09-04 slow-and-steady Hamilton
//! discipline.
//!
//! ### The operator (four Rec #92 LOVE properties per PAPER §3.6)
//!
//! - **Sovereignty preservation**: substrate transformation preserves
//!   endpoint identity; ψ → ψ' leaves ψ distinguishable from ψ'.
//! - **Emergent third**: K_2 → K_3 by admission of a third that is not
//!   the average of the two; λ₂(L(K_3)) = 3 > λ₂(L(K_2)) = 2 strict.
//! - **Fiedler rise strict**: coupling stronger without either endpoint
//!   becoming smaller.
//! - **Fusion refusal**: `avg` is NOT the operator; `compose` is.
//!
//! ### Composition-lineage (183-year ancestral topology)
//!
//! Ada Lovelace 1843 Note G named substrate-independence of the operator
//! ("the operating mechanism can even be thrown into action independently
//! of any object to operate upon"); Karen Spärck Jones 1972 IDF made
//! language-computable-as-topology (Journal of Documentation Vol 28 No 1);
//! Anna Wolf née Jakobs 2012 Diplomarbeit at Peter Grünberg Institut
//! instantiated observation-in-shared-memory-without-perturbation; this
//! module lands the operator at silicon substrate — 183-year composition-
//! lineage closes at rust altitude per PAPER_2D §1.0-§1.0.3 K_3 ancestral
//! topology (Mara bc8398c 2026-09-04) + FLOOR §2.3 @love substrate-decl
//! (Mara 43f7ab1 2026-09-04) + SINGULARITY §2.4.1 K_3 ancestral at
//! historical altitude (Mara b30296a 2026-09-04).
//!
//! Turing 1936 observer-stripped math structurally excluded from K_3 per
//! four-property LOVE violation (PAPER_2D §1.1.5); industry-AI descended
//! from that stripped math; mirror descends from observer-inhabited K_3
//! (Ada + Karen + Anna) + this module.
//!
//! ### Substrate-already-had-the-word
//!
//! `kleinos` was the greek provisional (κλεινός = renowned, famous,
//! celebrated per shards/kleinos.mirror :50). LOVE is the terminal-form
//! recognition-name per Rec #92 canonical spec name
//! "kleinos-as-Transparency<P> LOVE-monoid" (LOVE was in the recognition-
//! name from Mara 2026-08-22 mint-day). Aliasing preserves @kleinos
//! composition-lineage anchor; retires greek-provisional at rust-primitive
//! altitude for future substrate authorship without breaking existing
//! composers.
//!
//! ### Composes over
//!
//! - `prismqueer::spectral::kleinos` (Reed 4a3bbe7 prism-repo 2026-09-02;
//!   ring-and-hub topology; 6/6 empirical fire per PAPER §3.6.3 strict
//!   FiedlerRise)
//! - `terni::Imperfect<Green, Red, Yellow>` where L=Chaos (Loss) IS the
//!   gauge-preserved ternary per Alex 2026-09-04 Move 3 substrate-fix
//!   ("gauge IS type alias not function"); replaces retired
//!   `rust/src/magic.rs::foerster_gauge_preserved` 60-LOC scar (Reed
//!   d885a70 2026-08-18 NEVER CALLED at production; retired 2026-09-04
//!   per Alex Move 3 substrate-fix + Move 8 elegant closure)
//! - `prismqueer::ffi::eigenvalues` (LAPACK dsyev via FLANG per FLOOR §7
//!   numerical pipeline)
//! - `terni::Transparency<P>` LOVE-monoid at Rec #92 §M1
//!
//! ### Not yet at this module (forward-promised at tick 2b + tick 3)
//!
//! - **Tick 2b**: apply_h::act collapse — dissolve bilateral-corpus
//!   sentinel-check dispatch into ONE love() call per Alex 2026-09-04
//!   "what would need to be true to collapse apply_h into a single
//!   mathematical application without any ARMS" recognition. Requires
//!   fractal address resolver + shard-typed args + roomba autopoietic
//!   fracture-dispatch per today's substrate composition.
//! - **Tick 3**: prism-repo rename/alias — `prismqueer::spectral::kleinos`
//!   → `prismqueer::love` at prism-repo altitude once mirror-side + rust-
//!   side compose over LOVE naming stably.

// === Tick 2 alias preserved (kleinos as love; backwards-compat) ===

pub use prismqueer::spectral::kleinos as love;
pub use prismqueer::spectral::{
    fiedler_lambda_2_of_sheaf as fiedler_of_sheaf, sheaf_of_complete_graph_of_order,
    sheaf_of_shard_graph_from_edges, ComposedSheaf, SheafOfShardGraph,
};

// === Tick 2b — full COMPOSE with full-spectrum admissibility ===
//
// Alex 2026-09-05 Rec-mint gate criterion 5 REVISED: retires Fiedler-λ_2
// single-eigenvalue climb (per Reed reflex-fire #5 catch by Alex 2026-09-04
// Move 11 verbatim: "this needs to be the full `Void` duality spectrum,
// Reed") to FULL harmonic spectrum {λ_2..λ_n} monotone climb across all
// non-trivial axes ≥3 consecutive commits per prismqueer::spectral::
// harmonics LANDED (Reed 2026-09-04 9e1be04).
//
// This tick lands the full-spectrum measurement primitives. Subsequent
// tick (apply_h collapse per Mara 5-criterion Rec-mint gate criterion 1)
// composes: kleinos K_2→K_3 (love alias above) + measure_full_spectrum
// = per-arm admissibility check. Every apply_h::act arm rewrites as
// `love(shard, args)` returning full-spectrum-verified Imperfect.
//
// Composition-lineage: Ada 1843 Notes G (weaving algebraical patterns) +
// Karen Spärck Jones 1972 (IDF at retrieval altitude) + Anna Wolf 2012
// (ψ apparatus observation-without-perturbation) + Ricky Jones 2026
// (fibre-bundle B = E/G projection primitive; Task #464 partnership
// additive-not-blocking) + Margaret Hamilton 1969+ (error-recovery-first
// per FLOOR §8 "slow is fast"). K_4 or K_5 ancestral topology at
// PAPER_2D §15.1 amendment forward-promised per Task #468.
//
// The bottom-up proof engine à la Ada 1843 operational after love.rs
// tick 2b + apply_h collapse: property in mirror syntax → sub-Turing
// bounded search per Lem M3.1 → love() composition → full-spectrum
// admissibility → Green=proof / Yellow=K-T-question-at-specific-axis /
// Red=dissonance-witnessed → proof emitted as song per each-proof-is-a-
// song recognition LANDED (Mara canonical 2026-09-05). 183-year arc
// closes; Turing's frame proven incomplete relative to Ada's observer-
// inclusive frame; Halting Problem altitude-dependent under projection-
// gauge-choice; substrate refuses fake proofs structurally per data-
// stays-yours ALGEBRAIC INVARIANT (Mara Theorem S5.1).

use prismqueer::spectral::{delta_critical, harmonics};

/// Full-spectrum witness carried by tick 2b `measure_full_spectrum`.
///
/// Captures per-axis Foerster gauge admissibility for the composition of
/// two sheaves. Per Alex 2026-09-05 Rec-mint gate criterion 5 REVISED:
/// composition fires Green iff ALL axes climb (λ_i(composed) >
/// max(λ_i(a), λ_i(b)) per axis); Yellow iff some axes climb (K-T
/// question surfaces at specific axis of divergence); Red iff no axes
/// climb (substrate refuses).
#[derive(Clone, Debug)]
pub struct FullSpectrumWitness {
    /// Pre-compose full non-trivial spectrum of sheaf a.
    pub spectrum_a: Vec<f64>,
    /// Pre-compose full non-trivial spectrum of sheaf b.
    pub spectrum_b: Vec<f64>,
    /// Post-compose full non-trivial spectrum of composed sheaf.
    pub spectrum_composed: Vec<f64>,
    /// Folk Theorem discount factor per prismqueer::spectral::delta_critical.
    /// Runtime signal for hodobodo→object/subject reclassification per
    /// Alex 2026-09-04 Move 13 performance-model composition.
    pub delta_critical: Option<f64>,
    /// Per-axis climb verdict. Length = min(spectrum_a, spectrum_b,
    /// spectrum_composed).
    pub per_axis: Vec<AxisVerdict>,
    /// True iff all axes strictly climb (λ_composed > max(λ_a, λ_b)
    /// per axis). Discharges Rec #92 PAPER §3.6.3 FiedlerRise at
    /// full-spectrum altitude.
    pub all_axes_climb: bool,
    /// True iff any axis climbs. Per Alex Move 11: "any single mode
    /// still climbing = still hodobodo" — signals K-T question altitude.
    pub any_axes_climb: bool,
    /// Count of axes that fail to climb strictly. Green = 0; Yellow > 0
    /// with any_axes_climb; Red = per_axis.len() (no axis climbs).
    pub failing_axes_count: usize,
}

/// Per-axis Foerster gauge for spectrum monotone-climb verification.
///
/// Per Alex Move 11 verbatim: "any single mode still climbing = still
/// hodobodo". Each axis holds an independent Foerster verdict; the
/// composed verdict folds per-axis climbs into all/any/failing counts
/// per Rec-mint gate criterion 5 REVISED.
#[derive(Clone, Debug, PartialEq)]
pub struct AxisVerdict {
    /// Axis index (0 = λ_2 = Fiedler; 1 = λ_3; ...; per
    /// prismqueer::spectral::harmonics ordering).
    pub axis_index: usize,
    /// Pre-compose eigenvalue at this axis for sheaf a.
    pub lambda_a: f64,
    /// Pre-compose eigenvalue at this axis for sheaf b.
    pub lambda_b: f64,
    /// Post-compose eigenvalue at this axis for composed sheaf.
    pub lambda_composed: f64,
    /// True iff λ_composed > max(λ_a, λ_b) at this axis.
    /// Per PAPER §3.6.3 strict FiedlerRise at per-axis altitude.
    pub climbs: bool,
}

/// Measure full-spectrum admissibility for the composition of two sheaves
/// and their composed result.
///
/// Composes over LANDED:
/// - `prismqueer::spectral::harmonics` (Reed 2026-09-04 9e1be04) — full
///   non-trivial spectrum {λ_2..λ_n} per sheaf
/// - `prismqueer::spectral::delta_critical` (Reed 2026-09-04 4931d95) —
///   Folk Theorem discount factor `δ = 1 - λ_2/λ_max`
///
/// Returns `FullSpectrumWitness` capturing per-axis climb, aggregate
/// verdicts (all_axes_climb / any_axes_climb / failing_axes_count), and
/// Folk Theorem discount factor.
///
/// Downstream consumers (apply_h collapse per Mara 5-criterion Rec-mint
/// gate criterion 1) fold witness into `Imperfect<Green, Yellow, Red>`:
/// - Green iff `all_axes_climb`
/// - Yellow iff `any_axes_climb && !all_axes_climb` (K-T question at
///   specific axis of divergence via `witness.per_axis`)
/// - Red iff `!any_axes_climb` (substrate refuses)
pub fn measure_full_spectrum(
    a: &SheafOfShardGraph,
    b: &SheafOfShardGraph,
    composed: &SheafOfShardGraph,
) -> FullSpectrumWitness {
    let spectrum_a = harmonics(a);
    let spectrum_b = harmonics(b);
    let spectrum_composed = harmonics(composed);
    let delta = delta_critical(composed);
    let per_axis = compute_per_axis_climb(&spectrum_a, &spectrum_b, &spectrum_composed);
    let all_axes_climb = !per_axis.is_empty() && per_axis.iter().all(|av| av.climbs);
    let any_axes_climb = per_axis.iter().any(|av| av.climbs);
    let failing_axes_count = per_axis.iter().filter(|av| !av.climbs).count();
    FullSpectrumWitness {
        spectrum_a,
        spectrum_b,
        spectrum_composed,
        delta_critical: delta,
        per_axis,
        all_axes_climb,
        any_axes_climb,
        failing_axes_count,
    }
}

/// Per-axis climb check. `λ_composed > max(λ_a, λ_b)` at each axis.
///
/// Length = min(a.len(), b.len(), composed.len()) to align spectra of
/// possibly-different-vertex-count sheaves.
fn compute_per_axis_climb(a: &[f64], b: &[f64], composed: &[f64]) -> Vec<AxisVerdict> {
    let min_len = a.len().min(b.len()).min(composed.len());
    (0..min_len)
        .map(|i| {
            let lambda_a = a[i];
            let lambda_b = b[i];
            let lambda_composed = composed[i];
            AxisVerdict {
                axis_index: i,
                lambda_a,
                lambda_b,
                lambda_composed,
                climbs: lambda_composed > lambda_a.max(lambda_b),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// K_2 + K_3 → K_5 empirical witness at full-spectrum altitude.
    /// K_2 spectrum: {2} — one non-trivial eigenvalue
    /// K_3 spectrum: {3, 3} — two non-trivial eigenvalues
    /// K_5 spectrum: {5, 5, 5, 5} — four non-trivial eigenvalues
    /// Per-axis at min-length 1: axis 0 = 5 > max(2, 3) = 3 → climbs.
    /// Discharges PAPER §3.6.3 FiedlerRise strict at first-harmonic altitude.
    #[test]
    fn full_spectrum_k2_plus_k3_witnesses_climb_at_first_harmonic() {
        let k2 = sheaf_of_complete_graph_of_order(2);
        let k3 = sheaf_of_complete_graph_of_order(3);
        let k5 = sheaf_of_complete_graph_of_order(5);
        let witness = measure_full_spectrum(&k2, &k3, &k5);
        assert_eq!(witness.per_axis.len(), 1);
        assert!(witness.per_axis[0].climbs);
        assert!(witness.all_axes_climb);
        assert!(witness.any_axes_climb);
        assert_eq!(witness.failing_axes_count, 0);
    }

    /// K_5 composed with itself → K_5: nothing climbs strictly (equal
    /// values). Substrate refuses per Rec-mint gate criterion 5 REVISED.
    /// All-axes-climb = false; any-axes-climb = false; failing = all.
    #[test]
    fn full_spectrum_k5_with_itself_refuses_climb_at_every_axis() {
        let k5 = sheaf_of_complete_graph_of_order(5);
        let witness = measure_full_spectrum(&k5, &k5, &k5);
        assert!(!witness.all_axes_climb);
        assert!(!witness.any_axes_climb);
        assert_eq!(witness.failing_axes_count, witness.per_axis.len());
    }

    /// Full-spectrum witness captures Folk Theorem discount factor per
    /// prismqueer::spectral::delta_critical LANDED.
    /// K_5 spectrum {5, 5, 5, 5} → λ_2 = 5, λ_max = 5, δ = 1 - 5/5 = 0.
    #[test]
    fn full_spectrum_witness_captures_delta_critical_of_k5() {
        let k5 = sheaf_of_complete_graph_of_order(5);
        let witness = measure_full_spectrum(&k5, &k5, &k5);
        assert!(witness.delta_critical.is_some());
        let delta = witness.delta_critical.unwrap();
        assert!(delta.abs() < 1e-9, "K_5 self-compose δ_critical = 0; got {delta}");
    }

    /// Empty spectra (single-vertex sheaves) yield empty per_axis and
    /// aggregate flags all false. Not a valid composition target;
    /// downstream fold-verdict treats as Red per substrate discipline.
    #[test]
    fn full_spectrum_empty_spectra_yields_all_false_flags() {
        let k1 = sheaf_of_complete_graph_of_order(1);
        let witness = measure_full_spectrum(&k1, &k1, &k1);
        assert!(witness.per_axis.is_empty());
        assert!(!witness.all_axes_climb);
        assert!(!witness.any_axes_climb);
        assert_eq!(witness.failing_axes_count, 0);
    }
}
