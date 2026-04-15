//! MirrorLoss — the compilation trace as Loss.
//!
//! Mirror's domain-specific Loss implementation. This IS Transport::Holonomy.
//! Every field is something the LSP protocol cannot express.
//! The negative space of the protocol IS the type.
//!
//! The compilation pipeline has four folds, each returns Imperfect:
//!
//! ```text
//! source  <= ast                    parse fold
//! ast     <= resolved(ast)          resolution fold
//! resolved <= verdict per property  property fold
//! resolved <= crystal               emit fold
//! ```

use prism::{Imperfect, Loss};

use crate::declaration::{DeclKind, OpticOp};
use crate::kernel::{Oid, TraceOid};

// ---------------------------------------------------------------------------
// AstPosition — where in the tree a warning occurred
// ---------------------------------------------------------------------------

/// Position in the AST where a parse warning was generated.
#[derive(Clone, Debug, PartialEq)]
pub enum AstPosition {
    TopLevel,
    Grammar(Oid),
    Type(Oid),
    Action(Oid),
    Property(Oid),
    Prism(Oid),
    Fold(Oid),
    Split(Oid),
    Zoom(Oid),
    Refract(Oid),
}

// ---------------------------------------------------------------------------
// ParseWarning — typed parse-phase warnings (replaces UnrecognizedDecl)
// ---------------------------------------------------------------------------

/// A typed warning from the parse phase. Each variant carries position and
/// line information. These are measured loss: information that existed in the
/// source but did not survive the parse fold.
#[derive(Clone, Debug, PartialEq)]
pub enum ParseWarning {
    /// A token the parser did not recognize as a declaration keyword.
    UnknownToken { at: AstPosition, line: usize },
    /// A declaration keyword that has been deprecated in favor of another.
    DeprecatedKind {
        kind: DeclKind,
        replacement: DeclKind,
        at: AstPosition,
        line: usize,
    },
    /// A declaration keyword that requires a name but has none.
    MissingName {
        kind: DeclKind,
        at: AstPosition,
        line: usize,
    },
    /// Two declarations of the same kind share the same name in the same scope.
    DuplicateName {
        kind: DeclKind,
        first_line: usize,
        second_line: usize,
        at: AstPosition,
    },
    /// A parent reference that could not be resolved.
    UnresolvedParent {
        parent_name: String,
        at: AstPosition,
        line: usize,
    },
    /// An operator token that is syntactically malformed.
    MalformedOperator {
        operator: OpticOp,
        at: AstPosition,
        line: usize,
    },
}

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
// ParseLoss — source <= ast
// ---------------------------------------------------------------------------

/// Loss from the parse fold: source <= ast.
///
/// Typed warnings are measured loss: the information existed in
/// the source but did not survive the parse phase.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ParseLoss {
    pub warnings: Vec<ParseWarning>,
}

impl ParseLoss {
    pub fn zero() -> Self {
        ParseLoss {
            warnings: Vec::new(),
        }
    }

    pub fn holonomy(&self) -> f64 {
        self.warnings.len() as f64
    }

    pub fn is_zero(&self) -> bool {
        self.warnings.is_empty()
    }

    pub fn combine(mut self, other: Self) -> Self {
        self.warnings.extend(other.warnings);
        self
    }
}

// ---------------------------------------------------------------------------
// ResolutionLoss — ast <= resolved(ast)
// ---------------------------------------------------------------------------

/// Loss from the resolution fold: ast <= resolved(ast).
///
/// Unresolved refs are symbols that the resolver could not find.
/// resolution_ratio is the fraction that DID resolve (1.0 = perfect).
#[derive(Clone, Debug, PartialEq)]
pub struct ResolutionLoss {
    pub unresolved_refs: Vec<(String, TraceOid)>,
    pub resolution_ratio: f64,
}

impl ResolutionLoss {
    pub fn zero() -> Self {
        ResolutionLoss {
            unresolved_refs: Vec::new(),
            resolution_ratio: 1.0,
        }
    }

    pub fn holonomy(&self) -> f64 {
        self.unresolved_refs.len() as f64
    }

    pub fn is_zero(&self) -> bool {
        self.unresolved_refs.is_empty() && self.resolution_ratio == 1.0
    }

    pub fn combine(mut self, other: Self) -> Self {
        self.unresolved_refs.extend(other.unresolved_refs);
        self.resolution_ratio = self.resolution_ratio.min(other.resolution_ratio);
        self
    }
}

// ---------------------------------------------------------------------------
// PropertyLoss — resolved <= verdict per property
// ---------------------------------------------------------------------------

/// A verdict on a single property check.
#[derive(Clone, Debug, PartialEq)]
pub struct PropertyVerdict {
    pub property: String,
    /// Verdict IS Imperfect — not a separate enum.
    /// Success = pass, Partial = pass with loss, Failure = error(observation).
    /// The String in Failure IS error(observation) — what the property saw.
    /// The f64 is the loss.
    pub verdict: Imperfect<(), String, f64>,
}

/// Loss from the property fold: resolved <= verdict per property.
#[derive(Clone, Debug, PartialEq)]
pub struct PropertyLoss {
    pub verdicts: Vec<PropertyVerdict>,
}

impl PropertyLoss {
    pub fn zero() -> Self {
        PropertyLoss {
            verdicts: Vec::new(),
        }
    }

    pub fn holonomy(&self) -> f64 {
        self.verdicts
            .iter()
            .map(|v| match &v.verdict {
                Imperfect::Success(_) => 0.0,
                Imperfect::Partial(_, loss) => *loss,
                Imperfect::Failure(_, loss) => *loss,
            })
            .sum()
    }

    pub fn is_zero(&self) -> bool {
        self.verdicts.is_empty()
    }

    pub fn combine(mut self, other: Self) -> Self {
        self.verdicts.extend(other.verdicts);
        self
    }
}

// ---------------------------------------------------------------------------
// EmitLoss — resolved <= crystal
// ---------------------------------------------------------------------------

/// Loss from the emit fold: resolved <= crystal.
///
/// Phase records trace the compilation steps. Staleness measures how old
/// the artifact is. Dark dims are aperture dimensions that were not observed.
#[derive(Clone, Debug, PartialEq)]
pub struct EmitLoss {
    pub phases: Vec<PhaseRecord>,
    pub staleness: usize,
    pub dark_dims: Vec<usize>,
}

impl EmitLoss {
    pub fn zero() -> Self {
        EmitLoss {
            phases: Vec::new(),
            staleness: 0,
            dark_dims: Vec::new(),
        }
    }

    pub fn holonomy(&self) -> f64 {
        self.phases.iter().map(|p| p.structural_loss).sum()
    }

    pub fn is_zero(&self) -> bool {
        self.phases.is_empty()
    }

    pub fn combine(mut self, other: Self) -> Self {
        self.phases.extend(other.phases);
        self.staleness = self.staleness.max(other.staleness);
        for d in other.dark_dims {
            if !self.dark_dims.contains(&d) {
                self.dark_dims.push(d);
            }
        }
        self
    }
}

// ---------------------------------------------------------------------------
// MirrorLoss — the four-fold compilation trace as measured loss
// ---------------------------------------------------------------------------

/// The compilation trace as measured loss, organized by fold.
///
/// Every field is a gap in the LSP protocol. Every field is a dimension
/// the protocol cannot express. Together they form the full story of
/// what happened, what it cost, and where it stands.
///
/// MirrorLoss implements `Loss` — it is a monoid under `combine`,
/// with `zero()` as identity and `total()` as absorbing element.
#[derive(Clone, Debug, PartialEq)]
pub struct MirrorLoss {
    /// source <= ast
    pub parse: ParseLoss,
    /// ast <= resolved(ast)
    pub resolution: ResolutionLoss,
    /// resolved <= verdict per property
    pub properties: PropertyLoss,
    /// resolved <= crystal
    pub emit: EmitLoss,
    /// the combined measurement
    pub convergence: Convergence,
    /// Content OID of the artifact, if any.
    pub crystal: Option<Oid>,
    /// Whether this result was recovered from a prior failure.
    pub recovered: bool,
}

impl MirrorLoss {
    /// Single f64 summarizing total holonomy.
    /// Higher = more curvature = less settled.
    pub fn holonomy(&self) -> f64 {
        let convergence_penalty = match &self.convergence {
            Convergence::Settled => 0.0,
            Convergence::Converging(n) => *n as f64,
            Convergence::Oscillating(n) => *n as f64 * 2.0,
            Convergence::BudgetExhausted => f64::INFINITY,
        };

        self.parse.holonomy()
            + self.resolution.holonomy()
            + self.properties.holonomy()
            + self.emit.holonomy()
            + convergence_penalty
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
            parse: ParseLoss::zero(),
            resolution: ResolutionLoss::zero(),
            properties: PropertyLoss::zero(),
            emit: EmitLoss::zero(),
            convergence: Convergence::Settled,
            crystal: None,
            recovered: false,
        }
    }

    fn total() -> Self {
        MirrorLoss {
            parse: ParseLoss::zero(),
            resolution: ResolutionLoss {
                unresolved_refs: Vec::new(),
                resolution_ratio: 0.0,
            },
            properties: PropertyLoss::zero(),
            emit: EmitLoss::zero(),
            convergence: Convergence::BudgetExhausted,
            crystal: None,
            recovered: false,
        }
    }

    fn is_zero(&self) -> bool {
        self.parse.is_zero()
            && self.resolution.is_zero()
            && self.properties.is_zero()
            && self.emit.is_zero()
    }

    fn combine(self, other: Self) -> Self {
        MirrorLoss {
            parse: self.parse.combine(other.parse),
            resolution: self.resolution.combine(other.resolution),
            properties: self.properties.combine(other.properties),
            emit: self.emit.combine(other.emit),
            convergence: other.convergence,
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
        assert_eq!(z.resolution.resolution_ratio, 1.0);
        assert!(z.emit.phases.is_empty());
        assert!(z.resolution.unresolved_refs.is_empty());
        assert_eq!(z.emit.staleness, 0);
        assert_eq!(z.convergence, Convergence::Settled);
        assert!(z.emit.dark_dims.is_empty());
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
        assert_eq!(t.resolution.resolution_ratio, 0.0);
        assert_eq!(t.convergence, Convergence::BudgetExhausted);
    }

    // -- is_zero checks all four folds --

    #[test]
    fn is_zero_checks_parse() {
        let mut loss = MirrorLoss::zero();
        loss.parse.warnings.push(ParseWarning::UnknownToken {
            at: AstPosition::TopLevel,
            line: 1,
        });
        assert!(
            !loss.is_zero(),
            "parse warning should break is_zero"
        );
    }

    #[test]
    fn is_zero_checks_resolution() {
        let mut loss = MirrorLoss::zero();
        loss.resolution.resolution_ratio = 0.5;
        assert!(!loss.is_zero());
    }

    #[test]
    fn is_zero_checks_emit() {
        let mut loss = MirrorLoss::zero();
        loss.emit.phases.push(PhaseRecord {
            phase: Phase::Emit,
            input_oid: Oid::new("in"),
            output_oid: Oid::new("out"),
            structural_loss: 0.0,
        });
        assert!(!loss.is_zero());
    }

    #[test]
    fn is_zero_checks_properties() {
        let mut loss = MirrorLoss::zero();
        loss.properties.verdicts.push(PropertyVerdict {
            property: "test".into(),
            verdict: Imperfect::Success(()),
        });
        assert!(!loss.is_zero());
    }

    #[test]
    fn with_unresolved_refs_but_full_ratio_is_not_zero() {
        let mut loss = MirrorLoss::zero();
        loss.resolution
            .unresolved_refs
            .push(("@missing".into(), TraceOid::new("abc")));
        // unresolved_refs means resolution.is_zero() is false
        assert!(!loss.is_zero());
    }

    // -- combine: emit phases --

    #[test]
    fn combine_appends_phases() {
        let a = MirrorLoss {
            emit: EmitLoss {
                phases: vec![PhaseRecord {
                    phase: Phase::Tokenize,
                    input_oid: Oid::new("a1"),
                    output_oid: Oid::new("a2"),
                    structural_loss: 1.0,
                }],
                ..EmitLoss::zero()
            },
            ..MirrorLoss::zero()
        };
        let b = MirrorLoss {
            emit: EmitLoss {
                phases: vec![PhaseRecord {
                    phase: Phase::Parse,
                    input_oid: Oid::new("b1"),
                    output_oid: Oid::new("b2"),
                    structural_loss: 2.0,
                }],
                ..EmitLoss::zero()
            },
            ..MirrorLoss::zero()
        };
        let combined = a.combine(b);
        assert_eq!(combined.emit.phases.len(), 2);
        assert_eq!(combined.emit.phases[0].phase, Phase::Tokenize);
        assert_eq!(combined.emit.phases[1].phase, Phase::Parse);
    }

    // -- combine: resolution --

    #[test]
    fn combine_takes_min_resolution() {
        let a = MirrorLoss {
            resolution: ResolutionLoss {
                resolution_ratio: 0.8,
                ..ResolutionLoss::zero()
            },
            ..MirrorLoss::zero()
        };
        let b = MirrorLoss {
            resolution: ResolutionLoss {
                resolution_ratio: 0.5,
                ..ResolutionLoss::zero()
            },
            ..MirrorLoss::zero()
        };
        assert_eq!(a.combine(b).resolution.resolution_ratio, 0.5);
    }

    #[test]
    fn combine_takes_min_resolution_reversed() {
        let a = MirrorLoss {
            resolution: ResolutionLoss {
                resolution_ratio: 0.3,
                ..ResolutionLoss::zero()
            },
            ..MirrorLoss::zero()
        };
        let b = MirrorLoss {
            resolution: ResolutionLoss {
                resolution_ratio: 0.9,
                ..ResolutionLoss::zero()
            },
            ..MirrorLoss::zero()
        };
        assert_eq!(a.combine(b).resolution.resolution_ratio, 0.3);
    }

    // -- combine: unresolved_refs --

    #[test]
    fn combine_unions_unresolved_refs() {
        let a = MirrorLoss {
            resolution: ResolutionLoss {
                unresolved_refs: vec![("@a".into(), TraceOid::new("oid_a"))],
                ..ResolutionLoss::zero()
            },
            ..MirrorLoss::zero()
        };
        let b = MirrorLoss {
            resolution: ResolutionLoss {
                unresolved_refs: vec![("@b".into(), TraceOid::new("oid_b"))],
                ..ResolutionLoss::zero()
            },
            ..MirrorLoss::zero()
        };
        let combined = a.combine(b);
        assert_eq!(combined.resolution.unresolved_refs.len(), 2);
    }

    // -- combine: staleness --

    #[test]
    fn combine_takes_max_staleness() {
        let a = MirrorLoss {
            emit: EmitLoss {
                staleness: 3,
                ..EmitLoss::zero()
            },
            ..MirrorLoss::zero()
        };
        let b = MirrorLoss {
            emit: EmitLoss {
                staleness: 7,
                ..EmitLoss::zero()
            },
            ..MirrorLoss::zero()
        };
        assert_eq!(a.combine(b).emit.staleness, 7);
    }

    // -- combine: dark_dims --

    #[test]
    fn combine_unions_dark_dims() {
        let a = MirrorLoss {
            emit: EmitLoss {
                dark_dims: vec![0, 2],
                ..EmitLoss::zero()
            },
            ..MirrorLoss::zero()
        };
        let b = MirrorLoss {
            emit: EmitLoss {
                dark_dims: vec![1, 2],
                ..EmitLoss::zero()
            },
            ..MirrorLoss::zero()
        };
        let combined = a.combine(b);
        // Union: 0, 2, 1 — no duplicates
        assert_eq!(combined.emit.dark_dims.len(), 3);
        assert!(combined.emit.dark_dims.contains(&0));
        assert!(combined.emit.dark_dims.contains(&1));
        assert!(combined.emit.dark_dims.contains(&2));
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
            resolution: ResolutionLoss {
                resolution_ratio: 0.7,
                ..ResolutionLoss::zero()
            },
            ..MirrorLoss::zero()
        };
        let combined = MirrorLoss::zero().combine(x.clone());
        assert_eq!(
            combined.resolution.resolution_ratio,
            x.resolution.resolution_ratio
        );
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
            emit: EmitLoss {
                phases: vec![PhaseRecord {
                    phase: Phase::Emit,
                    input_oid: Oid::new("in"),
                    output_oid: Oid::new("out"),
                    structural_loss: 3.5,
                }],
                ..EmitLoss::zero()
            },
            ..MirrorLoss::zero()
        };
        assert!(
            (loss.holonomy() - 3.5).abs() < 1e-10,
            "holonomy should be 3.5, got {}",
            loss.holonomy()
        );
    }

    #[test]
    fn holonomy_includes_unresolved_penalty() {
        let loss = MirrorLoss {
            resolution: ResolutionLoss {
                unresolved_refs: vec![
                    ("@missing".into(), TraceOid::new("a")),
                    ("@also_missing".into(), TraceOid::new("b")),
                ],
                ..ResolutionLoss::zero()
            },
            ..MirrorLoss::zero()
        };
        assert!(
            (loss.holonomy() - 2.0).abs() < 1e-10,
            "holonomy should be 2.0, got {}",
            loss.holonomy()
        );
    }

    #[test]
    fn holonomy_includes_convergence_penalty() {
        let converging = MirrorLoss {
            convergence: Convergence::Converging(5),
            ..MirrorLoss::zero()
        };
        assert!(
            (converging.holonomy() - 5.0).abs() < 1e-10,
            "Converging(5) should be 5.0, got {}",
            converging.holonomy()
        );

        let oscillating = MirrorLoss {
            convergence: Convergence::Oscillating(3),
            ..MirrorLoss::zero()
        };
        assert!(
            (oscillating.holonomy() - 6.0).abs() < 1e-10,
            "Oscillating(3) should be 6.0, got {}",
            oscillating.holonomy()
        );

        let exhausted = MirrorLoss {
            convergence: Convergence::BudgetExhausted,
            ..MirrorLoss::zero()
        };
        assert!(
            exhausted.holonomy().is_infinite(),
            "BudgetExhausted should be infinite, got {}",
            exhausted.holonomy()
        );
    }

    #[test]
    fn holonomy_is_sum_of_four_folds() {
        let loss = MirrorLoss {
            emit: EmitLoss {
                phases: vec![PhaseRecord {
                    phase: Phase::Parse,
                    input_oid: Oid::new("i"),
                    output_oid: Oid::new("o"),
                    structural_loss: 2.0,
                }],
                ..EmitLoss::zero()
            },
            resolution: ResolutionLoss {
                unresolved_refs: vec![("@x".into(), TraceOid::new("t"))],
                ..ResolutionLoss::zero()
            },
            convergence: Convergence::Converging(3),
            ..MirrorLoss::zero()
        };
        assert!(
            (loss.holonomy() - 6.0).abs() < 1e-10,
            "holonomy should be sum 6.0, got {}",
            loss.holonomy()
        );
    }

    #[test]
    fn holonomy_includes_property_loss() {
        let loss = MirrorLoss {
            properties: PropertyLoss {
                verdicts: vec![PropertyVerdict {
                    property: "test".into(),
                    verdict: Imperfect::Partial((), 1.5),
                }],
            },
            ..MirrorLoss::zero()
        };
        assert!(
            (loss.holonomy() - 1.5).abs() < 1e-10,
            "holonomy should include property loss 1.5, got {}",
            loss.holonomy()
        );
    }

    #[test]
    fn holonomy_includes_parse_loss() {
        let loss = MirrorLoss {
            parse: ParseLoss {
                warnings: vec![ParseWarning::UnknownToken {
                    at: AstPosition::TopLevel,
                    line: 1,
                }],
            },
            ..MirrorLoss::zero()
        };
        assert!(
            (loss.holonomy() - 1.0).abs() < 1e-10,
            "holonomy should include parse loss 1.0, got {}",
            loss.holonomy()
        );
    }

    // -- sub-loss holonomy --

    #[test]
    fn parse_loss_holonomy() {
        let p = ParseLoss::zero();
        assert_eq!(p.holonomy(), 0.0);

        let p2 = ParseLoss {
            warnings: vec![ParseWarning::UnknownToken {
                at: AstPosition::TopLevel,
                line: 1,
            }],
        };
        assert_eq!(p2.holonomy(), 1.0);
    }

    #[test]
    fn resolution_loss_holonomy() {
        let r = ResolutionLoss::zero();
        assert_eq!(r.holonomy(), 0.0);

        let r2 = ResolutionLoss {
            unresolved_refs: vec![("@a".into(), TraceOid::new("t"))],
            resolution_ratio: 0.5,
        };
        assert_eq!(r2.holonomy(), 1.0);
    }

    #[test]
    fn property_loss_holonomy() {
        let p = PropertyLoss::zero();
        assert_eq!(p.holonomy(), 0.0);

        let p2 = PropertyLoss {
            verdicts: vec![
                PropertyVerdict {
                    property: "a".into(),
                    verdict: Imperfect::Success(()),
                },
                PropertyVerdict {
                    property: "b".into(),
                    verdict: Imperfect::Failure("fail".into(), 2.0),
                },
            ],
        };
        assert_eq!(p2.holonomy(), 2.0);
    }

    #[test]
    fn emit_loss_holonomy() {
        let e = EmitLoss::zero();
        assert_eq!(e.holonomy(), 0.0);

        let e2 = EmitLoss {
            phases: vec![PhaseRecord {
                phase: Phase::Emit,
                input_oid: Oid::new("i"),
                output_oid: Oid::new("o"),
                structural_loss: 4.0,
            }],
            staleness: 0,
            dark_dims: Vec::new(),
        };
        assert_eq!(e2.holonomy(), 4.0);
    }

    // -- combine: parse --

    #[test]
    fn combine_parse_loss() {
        let a = ParseLoss {
            warnings: vec![ParseWarning::UnknownToken {
                at: AstPosition::TopLevel,
                line: 1,
            }],
        };
        let b = ParseLoss {
            warnings: vec![ParseWarning::UnknownToken {
                at: AstPosition::TopLevel,
                line: 2,
            }],
        };
        let c = a.combine(b);
        assert_eq!(c.warnings.len(), 2);
    }

    // -- combine: properties --

    #[test]
    fn combine_property_loss() {
        let a = PropertyLoss {
            verdicts: vec![PropertyVerdict {
                property: "a".into(),
                verdict: Imperfect::Success(()),
            }],
        };
        let b = PropertyLoss {
            verdicts: vec![PropertyVerdict {
                property: "b".into(),
                verdict: Imperfect::Failure("err".into(), 1.0),
            }],
        };
        let c = a.combine(b);
        assert_eq!(c.verdicts.len(), 2);
    }

    // -- combine: emit --

    #[test]
    fn combine_emit_loss() {
        let a = EmitLoss {
            phases: vec![PhaseRecord {
                phase: Phase::Emit,
                input_oid: Oid::new("a"),
                output_oid: Oid::new("b"),
                structural_loss: 1.0,
            }],
            staleness: 3,
            dark_dims: vec![0],
        };
        let b = EmitLoss {
            phases: Vec::new(),
            staleness: 5,
            dark_dims: vec![0, 1],
        };
        let c = a.combine(b);
        assert_eq!(c.phases.len(), 1);
        assert_eq!(c.staleness, 5);
        assert_eq!(c.dark_dims.len(), 2);
    }

    // -- PropertyVerdict --

    #[test]
    fn property_verdict_clone() {
        let v = PropertyVerdict {
            property: "p".into(),
            verdict: Imperfect::Partial((), 0.5),
        };
        let v2 = v.clone();
        assert_eq!(v, v2);
    }
}
