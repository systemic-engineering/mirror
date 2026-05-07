//! The Abyss — the core loop. The never ending settling.
//!
//! Three Prism operations in a loop. Spectral hash convergence detection.
//! The thing you look into that looks back.
//!
//! Every compilation is an Abyss cycle. Every boot is the Abyss settling.
//! The Abyss doesn't know domains. It knows optics. The domain is the graph.
//! The lenses shape what the Abyss sees.
//!
//! ~3K parameters classify tension. Everything else is structural.

use prism::{Beam, Oid, Precision, Prism, ScalarLoss};

// ---------------------------------------------------------------------------
// Convergence
// ---------------------------------------------------------------------------

/// How the loop terminated.
#[derive(Clone, Debug, PartialEq)]
pub enum Termination {
    /// Spectral hash matched previous cycle. Fixed point reached.
    Settled { cycles: usize },
    /// Tension budget exhausted before convergence.
    BudgetExhausted {
        cycles: usize,
        remaining_loss: ScalarLoss,
    },
    /// Hash oscillates between attractors. Gödelian boundary.
    Oscillation { cycles: usize, attractors: Vec<Oid> },
}

/// Configuration for the Abyss loop.
pub struct AbyssConfig {
    /// Maximum cycles before budget exhaustion.
    pub max_cycles: usize,
    /// Precision for each prism operation.
    pub precision: Precision,
    /// Number of previous hashes to check for oscillation.
    pub oscillation_window: usize,
}

impl Default for AbyssConfig {
    fn default() -> Self {
        AbyssConfig {
            max_cycles: 64,
            precision: Precision::new(1e-6),
            oscillation_window: 4,
        }
    }
}

// ---------------------------------------------------------------------------
// PrismLoop — the recursive extension
// ---------------------------------------------------------------------------

/// A Prism that can loop: fold from a projection back to the focused form.
/// The output of one cycle becomes the input of the next.
///
/// TODO: migrate to new Prism trait — uses Projected/Focused instead of old Projection/Eigenvalues.
pub trait PrismLoop: Prism {
    /// Re-decompose a projection into a focused form.
    /// The recursive step that makes the loop possible.
    fn fold_from_projected(&self, projected: &Self::Projected) -> Self::Focused;
}

// ---------------------------------------------------------------------------
// The Loop
// ---------------------------------------------------------------------------

/// Run the Abyss: apply a PrismLoop until convergence.
///
/// TODO: body needs migration to new Prism trait (focus/project/refract changed signatures).
pub fn settle_loop<P>(
    _optic: &P,
    _input: P::Input,
    _config: &AbyssConfig,
    _hash: &dyn Fn(&<P::Projected as Beam>::Out) -> Oid,
) -> (P::Projected, Termination)
where
    P: PrismLoop,
    P::Input: Clone,
    P::Projected: Clone,
    P::Focused: Clone,
{
    // TODO: implement convergence loop with new Prism trait
    todo!("settle_loop: needs migration to new Prism trait")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use prism::{Beam, Precision, ScalarLoss};

    #[test]
    fn default_config() {
        let config = AbyssConfig::default();
        assert_eq!(config.max_cycles, 64);
        assert_eq!(config.oscillation_window, 4);
    }

    #[test]
    fn termination_types() {
        let settled = Termination::Settled { cycles: 5 };
        assert!(matches!(settled, Termination::Settled { .. }));

        let exhausted = Termination::BudgetExhausted {
            cycles: 64,
            remaining_loss: ScalarLoss::new(1.0),
        };
        assert!(matches!(exhausted, Termination::BudgetExhausted { .. }));

        let oscillation = Termination::Oscillation {
            cycles: 10,
            attractors: vec![Oid::new("a"), Oid::new("b")],
        };
        assert!(matches!(oscillation, Termination::Oscillation { .. }));
    }
}
