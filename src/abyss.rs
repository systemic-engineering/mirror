//! The Abyss — the core loop. The never ending settling.
//!
//! Five Prism operations in a loop. Spectral hash convergence detection.
//! The thing you look into that looks back.
//!
//! Every compilation is an Abyss cycle. Every boot is the Abyss settling.
//! The Abyss doesn't know domains. It knows optics. The domain is the graph.
//! The lenses shape what the Abyss sees.
//!
//! ~3K parameters classify tension. Everything else is structural.

use prism::{Beam, Oid, Precision, Prism, ShannonLoss};

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
        remaining_loss: ShannonLoss,
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
// The Loop
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// PrismLoop — the recursive extension
// ---------------------------------------------------------------------------

/// A Prism that can loop: fold from a projection back to eigenvalues.
/// The output of one cycle becomes the input of the next.
pub trait PrismLoop: Prism {
    /// Re-decompose a projection into eigenvalues.
    /// The recursive step that makes the loop possible.
    fn fold_from_projection(&self, projection: &Self::Projection) -> Self::Eigenvalues;
}

// ---------------------------------------------------------------------------
// The Loop
// ---------------------------------------------------------------------------

/// Run the Abyss: apply a PrismLoop until convergence.
pub fn settle_loop<P>(
    optic: &P,
    input: &P::Input,
    config: &AbyssConfig,
    transform: &dyn Fn(P::Projection) -> P::Projection,
    hash: &dyn Fn(&P::Projection) -> Oid,
) -> (Beam<P::Projection>, Termination)
where
    P: PrismLoop,
    P::Projection: Clone,
{
    let mut beam = prism::apply(optic, input, config.precision.clone(), transform);
    let mut prev_hash = hash(&beam.result);
    beam = beam.with_step(prev_hash.clone());

    let mut hashes: Vec<Oid> = vec![prev_hash.clone()];

    for cycle in 1..config.max_cycles {
        let eigenvalues = optic.fold_from_projection(&beam.result);
        let projection = optic.project(&eigenvalues, config.precision.clone());
        beam = optic.zoom(projection, transform);

        let current_hash = hash(&beam.result);
        beam = beam.with_step(current_hash.clone());

        if current_hash == prev_hash {
            return (beam, Termination::Settled { cycles: cycle });
        }

        if hashes.len() >= config.oscillation_window {
            let window = &hashes[hashes.len() - config.oscillation_window..];
            if window.contains(&current_hash) {
                return (
                    beam,
                    Termination::Oscillation {
                        cycles: cycle,
                        attractors: window.to_vec(),
                    },
                );
            }
        }

        hashes.push(current_hash.clone());
        prev_hash = current_hash;
    }

    (
        beam.clone(),
        Termination::BudgetExhausted {
            cycles: config.max_cycles,
            remaining_loss: beam.loss,
        },
    )
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use prism::{Beam, Precision, ShannonLoss};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    /// A Prism that converges: each cycle removes one element.
    struct ConvergingPrism;

    impl Prism for ConvergingPrism {
        type Input = Vec<i32>;
        type Eigenvalues = Vec<i32>;
        type Projection = Vec<i32>;
        type Node = i32;
        type Convergence = Vec<i32>;
        type Crystal = Vec<i32>;

        fn focus(&self, input: &Vec<i32>) -> Beam<Vec<i32>> {
            Beam::new(input.clone())
        }

        fn project(&self, eigenvalues: &Vec<i32>, _precision: Precision) -> Beam<Vec<i32>> {
            Beam::new(eigenvalues.clone())
        }

        fn split(&self, projection: &Vec<i32>) -> Vec<Beam<i32>> {
            projection.iter().map(|&v| Beam::new(v)).collect()
        }

        fn zoom(&self, beam: Beam<Vec<i32>>, f: &dyn Fn(Vec<i32>) -> Vec<i32>) -> Beam<Vec<i32>> {
            beam.map(f)
        }

        fn refract(&self, beam: Beam<Vec<i32>>) -> Vec<i32> {
            beam.result
        }
    }

    impl PrismLoop for ConvergingPrism {
        fn fold_from_projection(&self, projection: &Vec<i32>) -> Vec<i32> {
            projection.clone()
        }
    }

    fn hash_vec<T: Hash>(v: &Vec<T>) -> Oid {
        let mut hasher = DefaultHasher::new();
        v.hash(&mut hasher);
        Oid::new(format!("{:x}", hasher.finish()))
    }

    #[test]
    fn settles_on_identity() {
        let prism = ConvergingPrism;
        let input = vec![1, 2, 3];
        let config = AbyssConfig::default();

        let (beam, term) = settle_loop(
            &prism,
            &input,
            &config,
            &|v| v, // identity — already settled
            &hash_vec,
        );

        assert_eq!(beam.result, vec![1, 2, 3]);
        assert!(matches!(term, Termination::Settled { cycles: 1 }));
    }

    #[test]
    fn settles_after_transform() {
        let prism = ConvergingPrism;
        let input = vec![5, 3, 1, 4, 2];
        let config = AbyssConfig::default();

        let (beam, term) = settle_loop(
            &prism,
            &input,
            &config,
            &|mut v| {
                v.sort();
                v
            },
            &hash_vec,
        );

        // Sort is idempotent — settles on second cycle
        assert_eq!(beam.result, vec![1, 2, 3, 4, 5]);
        assert!(matches!(term, Termination::Settled { cycles: 1 }));
    }

    #[test]
    fn budget_exhausted() {
        let prism = ConvergingPrism;
        let input = vec![1];
        let config = AbyssConfig {
            max_cycles: 3,
            ..Default::default()
        };

        let (_, term) = settle_loop(
            &prism,
            &input,
            &config,
            &|mut v| {
                // Never settles — always changes
                v.push(v.len() as i32);
                v
            },
            &hash_vec,
        );

        assert!(matches!(
            term,
            Termination::BudgetExhausted { cycles: 3, .. }
        ));
    }

    #[test]
    fn beam_accumulates_path() {
        let prism = ConvergingPrism;
        let input = vec![1, 2, 3];
        let config = AbyssConfig {
            max_cycles: 5,
            ..Default::default()
        };

        let (beam, _) = settle_loop(
            &prism,
            &input,
            &config,
            &|v| v, // identity — settles immediately
            &hash_vec,
        );

        // Path should have entries — the Abyss records each cycle's hash
        assert!(!beam.path.is_empty());
    }

    #[test]
    fn boot_sequence_settles_combined() {
        use crate::parse::Parse;
        use crate::Vector;

        // Read all boot files, parse, accumulate into one graph
        let boot_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("boot");
        let mut entries: Vec<_> = std::fs::read_dir(&boot_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("conv"))
            .collect();
        entries.sort_by_key(|e| e.path());

        let mut graph: Vec<String> = Vec::new();
        for entry in &entries {
            let source = std::fs::read_to_string(entry.path()).unwrap();
            if let Ok(ast) = Parse.trace(source).into_result() {
                for child in ast.children() {
                    graph.push(format!("{}:{}", child.data().name, child.data().value));
                }
            }
        }
        assert!(
            graph.len() >= 19,
            "boot should have at least 19 nodes, got {}",
            graph.len()
        );

        // Settle with sort+dedup transform
        let prism = ConvergingPrism;
        let config = AbyssConfig {
            max_cycles: 64,
            ..Default::default()
        };

        // Adapt: use the graph as input to ConvergingPrism
        // which works on Vec<i32> — let's use a StringPrism instead
        struct BootPrism;
        impl prism::Prism for BootPrism {
            type Input = Vec<String>;
            type Eigenvalues = Vec<String>;
            type Projection = Vec<String>;
            type Node = String;
            type Convergence = Vec<String>;
            type Crystal = Vec<String>;

            fn focus(&self, input: &Vec<String>) -> Beam<Vec<String>> {
                Beam::new(input.clone())
            }
            fn project(&self, ev: &Vec<String>, _p: prism::Precision) -> Beam<Vec<String>> {
                Beam::new(ev.clone())
            }
            fn split(&self, proj: &Vec<String>) -> Vec<Beam<String>> {
                proj.iter().map(|s| Beam::new(s.clone())).collect()
            }
            fn zoom(
                &self,
                beam: Beam<Vec<String>>,
                f: &dyn Fn(Vec<String>) -> Vec<String>,
            ) -> Beam<Vec<String>> {
                beam.map(f)
            }
            fn refract(&self, beam: Beam<Vec<String>>) -> Vec<String> {
                beam.result
            }
        }
        impl PrismLoop for BootPrism {
            fn fold_from_projection(&self, p: &Vec<String>) -> Vec<String> {
                p.clone()
            }
        }

        let (beam, term) = settle_loop(
            &BootPrism,
            &graph,
            &config,
            &|mut v| {
                v.sort();
                v.dedup();
                v
            },
            &hash_vec,
        );

        // Must settle
        assert!(
            matches!(term, Termination::Settled { .. }),
            "boot must settle, got {:?}",
            term
        );
        // 19 unique keyword definitions (no duplicates — each keyword type is distinct)
        assert_eq!(
            beam.result.len(),
            19,
            "expected 19 unique keywords, got {}",
            beam.result.len()
        );
        // Zero loss
        assert!(beam.is_lossless());
    }

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
            remaining_loss: ShannonLoss::new(1.0),
        };
        assert!(matches!(exhausted, Termination::BudgetExhausted { .. }));

        let oscillation = Termination::Oscillation {
            cycles: 10,
            attractors: vec![Oid::new("a"), Oid::new("b")],
        };
        assert!(matches!(oscillation, Termination::Oscillation { .. }));
    }
}
