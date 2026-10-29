//! `engine`: mirror's evaluator core `E`.
//!
//! The contract is `docs/specs/mirror-fixpoint-parser.md` §4.1; the scope is
//! `docs/audits/2026-10-29-seam-engine-core-scope.md` §3.1–§3.4. Math:
//! `docs/math/2026-10-28-mara-mirror-fixpoint-parser-math-foundation.md`
//! §5.2–§5.3.
//!
//! `E(P, F, k_max) → (M, D, stop)`: semi-naive evaluation of a positive,
//! function-free program over a finite domain. A tick is a round; only the
//! delta joins; the run stops at the first empty delta. Each atom of `M` is
//! deposited once, as `(addr, tick)`. The tick lives in the deposit, never
//! in the address.
//!
//! Pure: no I/O, no threads, no clock, no randomness, no `unsafe`. Every
//! collection that is iterated is a `BTreeMap`/`BTreeSet` or a sorted `Vec`.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;

/// A 32-byte BLAKE3 address (spec §5.0).
pub type Addr = [u8; 32];

/// A constant: one of three tagged kinds (§4.1 "Terms").
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Const {
    /// A UTF-8 name.
    Sym(String),
    /// An unsigned integer (positions).
    Nat(u64),
    /// A splinter address.
    Addr(Addr),
}

/// A term: a variable or a constant. Function-free: there is no third case.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Term {
    Var(String),
    Const(Const),
}

/// `p(t₁, …, t_n)`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Atom {
    pub pred: String,
    pub args: Vec<Term>,
}

/// A body literal. The core sees `Pos` only; `Not` is for the
/// well-founded wrapper (§4.1 "The well-founded wrapper").
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Literal {
    Pos(Atom),
    Not(Atom),
}

/// `id: h :- b₁, …, b_k` (k ≥ 0).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rule {
    pub id: String,
    pub head: Atom,
    pub body: Vec<Literal>,
}

/// `functional p(key…) → value`: at most one value of argument `value` per
/// value of the arguments `key`. Positions are 1-based, as in §4.1's
/// `functional p(1, 2) → 3`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Functional {
    pub pred: String,
    pub key: Vec<usize>,
    pub value: usize,
}

/// A program: rules plus optional functionality declarations.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Program {
    pub rules: Vec<Rule>,
    pub functional: Vec<Functional>,
}

/// A ground atom.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fact {
    pub pred: String,
    pub args: Vec<Const>,
}

/// The fact store `F`: a set of ground atoms, all splinters at one
/// altitude `τ` (`@parse/item`, `@pact/fact`, or `@datalog/fact`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Facts {
    pub altitude: String,
    pub atoms: Vec<Fact>,
}

/// One deposit: an atom's address and the tick it first fell at.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Deposit {
    pub addr: Addr,
    pub tick: u64,
}

/// Why a run stopped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    /// `fixed at T`: round `T` had an empty delta.
    Fixed(u64),
    /// `budget at k_max`: round `k_max` still had a non-empty delta.
    Budget(u64),
}

/// `functional_violation(p, key, v, v′)`: a reported finding, never a
/// crash. `v < v′` in `Const` order.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionalViolation {
    pub pred: String,
    pub key: Vec<Const>,
    pub v: Const,
    pub v_prime: Const,
}

/// `(M, D, stop)`, plus functionality findings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Run {
    /// `M`, keyed (and so sorted) by address.
    pub model: BTreeMap<Addr, Fact>,
    /// The store-merge fragment address of `M` (children sorted by address).
    pub model_addr: Addr,
    /// `D`: one deposit per atom of `M`, sorted by `(tick, addr)`.
    pub deposits: Vec<Deposit>,
    pub stop: Stop,
    pub findings: Vec<FunctionalViolation>,
}

/// A program that is not run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Dark {
    /// A variable that is not range-restricted (head, or a negative literal).
    Unsafe { rule: String, var: String },
    /// A predicate used at two arities.
    Arity { pred: String, expected: usize, found: usize },
    /// A negative literal handed to the positive core.
    Negation { rule: String },
    /// A functionality declaration naming positions the predicate lacks.
    Functional { pred: String },
    /// A run that exceeded Prop. 5.4's bound. A bug in `E` if ever seen.
    Bound { ticks: u64, bound: u64 },
}

/// `addr = H("splinter\0" ‖ τ ‖ enc(p) ‖ u64(n) ‖ for each argument:
/// tag ‖ enc(bytes))` (§4.1, §5.0).
pub fn addr(_altitude: &str, _fact: &Fact) -> Addr {
    todo!("E: §4.1 atom address")
}

/// `E(P, F, k_max) → (M, D, stop)`.
pub fn run(_program: &Program, _facts: &Facts, _k_max: u64) -> Result<Run, Dark> {
    todo!("E: semi-naive core (audit §3.1)")
}
