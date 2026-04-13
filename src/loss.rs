//! MirrorLoss — the compilation trace as Loss.
//!
//! Mirror's domain-specific Loss implementation. This IS Transport::Holonomy.
//! Every field is something the LSP protocol cannot express.
//! The negative space of the protocol IS the type.

use prism::Loss;

use crate::kernel::{Oid, TraceOid};

// ---------------------------------------------------------------------------
// Phase — which compilation step
// ---------------------------------------------------------------------------

/// A compilation phase in the mirror pipeline.
#[derive(Clone, Debug, PartialEq)]
pub enum Phase {
    Tokenize,
    Parse,
    Resolve,
    Emit,
}

// ---------------------------------------------------------------------------
// PhaseRecord — one step's trace
// ---------------------------------------------------------------------------

/// A record of one compilation phase's execution.
#[derive(Clone, Debug, PartialEq)]
pub struct PhaseRecord {
    /// Which phase ran.
    pub phase: Phase,
    /// Content OID of the input to this phase.
    pub input_oid: Oid,
    /// Content OID of the output from this phase.
    pub output_oid: Oid,
    /// Structural loss: information lost during this phase (bits).
    pub structural_loss: f64,
}

// ---------------------------------------------------------------------------
// Convergence — abyss loop status
// ---------------------------------------------------------------------------

/// Convergence status of the compilation loop.
#[derive(Clone, Debug, PartialEq)]
pub enum Convergence {
    /// Approaching crystal. Value is estimated ticks remaining.
    Converging(usize),
    /// Spectral hash stable. Fixed point reached.
    Settled,
    /// Oscillating between N attractors.
    Oscillating(usize),
    /// Gave up — budget exhausted.
    BudgetExhausted,
}

// ---------------------------------------------------------------------------
// MirrorLoss — the compilation trace as measured loss
// ---------------------------------------------------------------------------

/// The compilation trace as measured loss.
///
/// Every field is a gap in the LSP protocol. Every field is a dimension
/// the protocol cannot express. Together they form the full story of
/// what happened, what it cost, and where it stands.
///
/// MirrorLoss implements `Loss` — it is a monoid under `combine`,
/// with `zero()` as identity and `total()` as absorbing element.
#[derive(Clone, Debug, PartialEq)]
pub struct MirrorLoss {
    /// Which compilation phases completed and their cost.
    pub phases: Vec<PhaseRecord>,
    /// Fraction of references resolved (0.0 to 1.0).
    pub resolution_ratio: f64,
    /// Symbols that didn't resolve, with their trace OIDs.
    pub unresolved_refs: Vec<(String, TraceOid)>,
    /// Ticks since this artifact was produced. Zero = fresh.
    pub staleness: usize,
    /// Convergence status of the abyss loop.
    pub convergence: Convergence,
    /// Which aperture dimensions were dark during observation.
    pub dark_dims: Vec<usize>,
    /// Content OID of the artifact, if any.
    pub crystal: Option<Oid>,
    /// Whether this result was recovered from a prior failure.
    pub recovered: bool,
}

impl MirrorLoss {
    /// Single f64 summarizing total holonomy.
    /// Higher = more curvature = less settled.
    pub fn holonomy(&self) -> f64 {
        let phase_loss: f64 = self.phases.iter().map(|p| p.structural_loss).sum();
        let unresolved_penalty = self.unresolved_refs.len() as f64;
        let convergence_penalty = match &self.convergence {
            Convergence::Settled => 0.0,
            Convergence::Converging(n) => *n as f64,
            Convergence::Oscillating(n) => *n as f64 * 2.0,
            Convergence::BudgetExhausted => f64::INFINITY,
        };

        // DELIBERATE BUG: multiply instead of add
        phase_loss * unresolved_penalty * convergence_penalty
    }
}

impl Default for MirrorLoss {
    fn default() -> Self {
        Self::zero()
    }
}

impl Loss for MirrorLoss {
    fn zero() -> Self {
        MirrorLoss {
            phases: Vec::new(),
            resolution_ratio: 1.0,
            unresolved_refs: Vec::new(),
            staleness: 0,
            convergence: Convergence::Settled,
            dark_dims: Vec::new(),
            crystal: None,
            recovered: false,
        }
    }

    fn total() -> Self {
        MirrorLoss {
            phases: Vec::new(),
            resolution_ratio: 0.0,
            unresolved_refs: Vec::new(),
            staleness: 0,
            convergence: Convergence::BudgetExhausted,
            dark_dims: Vec::new(),
            crystal: None,
            recovered: false,
        }
    }

    fn is_zero(&self) -> bool {
        self.phases.is_empty() && self.resolution_ratio == 1.0
    }

    fn combine(self, other: Self) -> Self {
        let mut phases = self.phases;
        phases.extend(other.phases);

        let mut unresolved = self.unresolved_refs;
        unresolved.extend(other.unresolved_refs);

        let mut dark = self.dark_dims;
        for d in other.dark_dims {
            if !dark.contains(&d) {
                dark.push(d);
            }
        }

        MirrorLoss {
            phases,
            resolution_ratio: self.resolution_ratio.min(other.resolution_ratio),
            unresolved_refs: unresolved,
            staleness: self.staleness.max(other.staleness),
            convergence: other.convergence,
            dark_dims: dark,
            crystal: other.crystal.or(self.crystal),
            recovered: self.recovered || other.recovered,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- zero --

    #[test]
    fn zero_is_lossless() {
        let z = MirrorLoss::zero();
        assert!(z.is_zero());
        assert_eq!(z.resolution_ratio, 1.0);
        assert!(z.phases.is_empty());
        assert!(z.unresolved_refs.is_empty());
        assert_eq!(z.staleness, 0);
        assert_eq!(z.convergence, Convergence::Settled);
        assert!(z.dark_dims.is_empty());
        assert!(z.crystal.is_none());
        assert!(!z.recovered);
    }

    #[test]
    fn default_is_zero() {
        let d = MirrorLoss::default();
        assert!(d.is_zero());
    }

    // -- total --

    #[test]
    fn total_is_not_zero() {
        let t = MirrorLoss::total();
        assert!(!t.is_zero());
        assert_eq!(t.resolution_ratio, 0.0);
        assert_eq!(t.convergence, Convergence::BudgetExhausted);
    }

    // -- is_zero --

    #[test]
    fn empty_phases_full_resolution_is_zero() {
        let loss = MirrorLoss {
            phases: Vec::new(),
            resolution_ratio: 1.0,
            unresolved_refs: vec![("@missing".into(), TraceOid::new("abc"))],
            staleness: 5,
            convergence: Convergence::Converging(3),
            dark_dims: vec![0, 1],
            crystal: Some(Oid::new("xyz")),
            recovered: true,
        };
        // is_zero only checks phases.is_empty() AND resolution_ratio == 1.0
        assert!(loss.is_zero());
    }

    #[test]
    fn with_phases_is_not_zero() {
        let loss = MirrorLoss {
            phases: vec![PhaseRecord {
                phase: Phase::Parse,
                input_oid: Oid::new("in"),
                output_oid: Oid::new("out"),
                structural_loss: 0.0,
            }],
            ..MirrorLoss::zero()
        };
        assert!(!loss.is_zero());
    }

    #[test]
    fn with_low_resolution_is_not_zero() {
        let loss = MirrorLoss {
            resolution_ratio: 0.5,
            ..MirrorLoss::zero()
        };
        assert!(!loss.is_zero());
    }

    // -- combine: phases --

    #[test]
    fn combine_appends_phases() {
        let a = MirrorLoss {
            phases: vec![PhaseRecord {
                phase: Phase::Tokenize,
                input_oid: Oid::new("a1"),
                output_oid: Oid::new("a2"),
                structural_loss: 1.0,
            }],
            ..MirrorLoss::zero()
        };
        let b = MirrorLoss {
            phases: vec![PhaseRecord {
                phase: Phase::Parse,
                input_oid: Oid::new("b1"),
                output_oid: Oid::new("b2"),
                structural_loss: 2.0,
            }],
            ..MirrorLoss::zero()
        };
        let combined = a.combine(b);
        assert_eq!(combined.phases.len(), 2);
        assert_eq!(combined.phases[0].phase, Phase::Tokenize);
        assert_eq!(combined.phases[1].phase, Phase::Parse);
    }

    // -- combine: resolution --

    #[test]
    fn combine_takes_min_resolution() {
        let a = MirrorLoss {
            resolution_ratio: 0.8,
            ..MirrorLoss::zero()
        };
        let b = MirrorLoss {
            resolution_ratio: 0.5,
            ..MirrorLoss::zero()
        };
        assert_eq!(a.combine(b).resolution_ratio, 0.5);
    }

    #[test]
    fn combine_takes_min_resolution_reversed() {
        let a = MirrorLoss {
            resolution_ratio: 0.3,
            ..MirrorLoss::zero()
        };
        let b = MirrorLoss {
            resolution_ratio: 0.9,
            ..MirrorLoss::zero()
        };
        assert_eq!(a.combine(b).resolution_ratio, 0.3);
    }

    // -- combine: unresolved_refs --

    #[test]
    fn combine_unions_unresolved_refs() {
        let a = MirrorLoss {
            unresolved_refs: vec![("@a".into(), TraceOid::new("oid_a"))],
            ..MirrorLoss::zero()
        };
        let b = MirrorLoss {
            unresolved_refs: vec![("@b".into(), TraceOid::new("oid_b"))],
            ..MirrorLoss::zero()
        };
        let combined = a.combine(b);
        assert_eq!(combined.unresolved_refs.len(), 2);
    }

    // -- combine: staleness --

    #[test]
    fn combine_takes_max_staleness() {
        let a = MirrorLoss {
            staleness: 3,
            ..MirrorLoss::zero()
        };
        let b = MirrorLoss {
            staleness: 7,
            ..MirrorLoss::zero()
        };
        assert_eq!(a.combine(b).staleness, 7);
    }

    // -- combine: dark_dims --

    #[test]
    fn combine_unions_dark_dims() {
        let a = MirrorLoss {
            dark_dims: vec![0, 2],
            ..MirrorLoss::zero()
        };
        let b = MirrorLoss {
            dark_dims: vec![1, 2],
            ..MirrorLoss::zero()
        };
        let combined = a.combine(b);
        // Union: 0, 2, 1 — no duplicates
        assert_eq!(combined.dark_dims.len(), 3);
        assert!(combined.dark_dims.contains(&0));
        assert!(combined.dark_dims.contains(&1));
        assert!(combined.dark_dims.contains(&2));
    }

    // -- combine: crystal --

    #[test]
    fn combine_takes_other_crystal() {
        let a = MirrorLoss {
            crystal: Some(Oid::new("old")),
            ..MirrorLoss::zero()
        };
        let b = MirrorLoss {
            crystal: Some(Oid::new("new")),
            ..MirrorLoss::zero()
        };
        assert_eq!(a.combine(b).crystal, Some(Oid::new("new")));
    }

    #[test]
    fn combine_falls_back_to_self_crystal() {
        let a = MirrorLoss {
            crystal: Some(Oid::new("kept")),
            ..MirrorLoss::zero()
        };
        let b = MirrorLoss {
            crystal: None,
            ..MirrorLoss::zero()
        };
        assert_eq!(a.combine(b).crystal, Some(Oid::new("kept")));
    }

    // -- combine: convergence --

    #[test]
    fn combine_takes_other_convergence() {
        let a = MirrorLoss {
            convergence: Convergence::Settled,
            ..MirrorLoss::zero()
        };
        let b = MirrorLoss {
            convergence: Convergence::Oscillating(3),
            ..MirrorLoss::zero()
        };
        assert_eq!(a.combine(b).convergence, Convergence::Oscillating(3));
    }

    // -- combine: recovered --

    #[test]
    fn combine_recovered_either() {
        let a = MirrorLoss {
            recovered: true,
            ..MirrorLoss::zero()
        };
        let b = MirrorLoss::zero();
        assert!(a.combine(b).recovered);

        let c = MirrorLoss::zero();
        let d = MirrorLoss {
            recovered: true,
            ..MirrorLoss::zero()
        };
        assert!(c.combine(d).recovered);
    }

    // -- combine: identity (zero combine x == x for key fields) --

    #[test]
    fn combine_zero_is_identity_for_resolution() {
        let x = MirrorLoss {
            resolution_ratio: 0.7,
            ..MirrorLoss::zero()
        };
        let combined = MirrorLoss::zero().combine(x.clone());
        assert_eq!(combined.resolution_ratio, x.resolution_ratio);
    }

    // -- Clone + Debug --

    #[test]
    fn mirror_loss_is_clone() {
        let a = MirrorLoss::zero();
        let b = a.clone();
        assert_eq!(a, b);
    }

    // -- PhaseRecord --

    #[test]
    fn phase_record_clone() {
        let r = PhaseRecord {
            phase: Phase::Emit,
            input_oid: Oid::new("i"),
            output_oid: Oid::new("o"),
            structural_loss: 1.5,
        };
        let r2 = r.clone();
        assert_eq!(r, r2);
    }

    // -- Convergence --

    #[test]
    fn convergence_variants() {
        assert_eq!(Convergence::Converging(5), Convergence::Converging(5));
        assert_eq!(Convergence::Settled, Convergence::Settled);
        assert_eq!(Convergence::Oscillating(2), Convergence::Oscillating(2));
        assert_eq!(Convergence::BudgetExhausted, Convergence::BudgetExhausted);
        assert_ne!(Convergence::Settled, Convergence::BudgetExhausted);
    }

    // -- Phase --

    #[test]
    fn phase_variants() {
        assert_eq!(Phase::Tokenize, Phase::Tokenize);
        assert_eq!(Phase::Parse, Phase::Parse);
        assert_eq!(Phase::Resolve, Phase::Resolve);
        assert_eq!(Phase::Emit, Phase::Emit);
        assert_ne!(Phase::Tokenize, Phase::Parse);
    }

    // -- holonomy --

    #[test]
    fn holonomy_zero_for_zero_loss() {
        let z = MirrorLoss::zero();
        assert_eq!(z.holonomy(), 0.0, "zero loss should have zero holonomy");
    }

    #[test]
    fn holonomy_includes_phase_loss() {
        let loss = MirrorLoss {
            phases: vec![PhaseRecord {
                phase: Phase::Emit,
                input_oid: Oid::new("in"),
                output_oid: Oid::new("out"),
                structural_loss: 3.5,
            }],
            ..MirrorLoss::zero()
        };
        assert!((loss.holonomy() - 3.5).abs() < 1e-10, "holonomy should be 3.5, got {}", loss.holonomy());
    }

    #[test]
    fn holonomy_includes_unresolved_penalty() {
        let loss = MirrorLoss {
            unresolved_refs: vec![
                ("@missing".into(), TraceOid::new("a")),
                ("@also_missing".into(), TraceOid::new("b")),
            ],
            ..MirrorLoss::zero()
        };
        assert!((loss.holonomy() - 2.0).abs() < 1e-10, "holonomy should be 2.0, got {}", loss.holonomy());
    }

    #[test]
    fn holonomy_includes_convergence_penalty() {
        let converging = MirrorLoss {
            convergence: Convergence::Converging(5),
            ..MirrorLoss::zero()
        };
        assert!((converging.holonomy() - 5.0).abs() < 1e-10, "Converging(5) should be 5.0, got {}", converging.holonomy());

        let oscillating = MirrorLoss {
            convergence: Convergence::Oscillating(3),
            ..MirrorLoss::zero()
        };
        assert!((oscillating.holonomy() - 6.0).abs() < 1e-10, "Oscillating(3) should be 6.0, got {}", oscillating.holonomy());

        let exhausted = MirrorLoss {
            convergence: Convergence::BudgetExhausted,
            ..MirrorLoss::zero()
        };
        assert!(exhausted.holonomy().is_infinite(), "BudgetExhausted should be infinite, got {}", exhausted.holonomy());
    }

    #[test]
    fn holonomy_is_sum_of_components() {
        let loss = MirrorLoss {
            phases: vec![PhaseRecord {
                phase: Phase::Parse,
                input_oid: Oid::new("i"),
                output_oid: Oid::new("o"),
                structural_loss: 2.0,
            }],
            unresolved_refs: vec![("@x".into(), TraceOid::new("t"))],
            convergence: Convergence::Converging(3),
            ..MirrorLoss::zero()
        };
        assert!((loss.holonomy() - 6.0).abs() < 1e-10, "holonomy should be sum 6.0, got {}", loss.holonomy());
    }
}
