//! `engine`: mirror's evaluator core `E`.
//!
//! The contract is `docs/specs/mirror-fixpoint-parser.md` §4.1, typed per
//! §4.2; the scope is `docs/audits/2026-10-29-seam-engine-core-scope.md`
//! §3.1–§3.4. Math: `docs/math/2026-10-28-mara-mirror-fixpoint-parser-math-foundation.md`
//! §5.2–§5.3.
//!
//! `E(P, F) → (M, D, stop)`: semi-naive evaluation of a positive,
//! function-free program over a finite domain. A tick is a round; only the
//! delta joins; the run stops at the first empty delta. Each atom of `M` is
//! deposited once, as `(addr, tick)`. The tick lives in the deposit, never
//! in the address.
//!
//! Illegal states are unrepresentable (§4.2): a program is checked when it
//! is built ([`PositiveProgram::new`], [`NormalProgram::new`]) and its facts
//! when the instance is built ([`Instance::new`]); after that a run is total
//! and never `dark`. Negation can't be written in a [`PositiveRule`].
//!
//! Pure: no I/O, no threads, no clock, no randomness, no `unsafe`. Every
//! collection that is iterated is a `BTreeMap`/`BTreeSet` or a sorted `Vec`.

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::num::{NonZeroU32, NonZeroU64};

// ── Newtypes (§4.2) ──────────────────────────────────────────────────────

/// A 32-byte BLAKE3 address (spec §5.0). Made by [`addr`], by `E`'s own
/// fragments, or by the checked parse [`Addr::from_hex`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Addr([u8; 32]);

impl Addr {
    /// Exactly 64 lowercase or uppercase hex digits.
    pub fn from_hex(s: &str) -> Option<Addr> {
        let b = s.as_bytes();
        if b.len() != 64 {
            return None;
        }
        let digit = |c: u8| (c as char).to_digit(16).map(|d| d as u8);
        let mut out = [0u8; 32];
        for (i, o) in out.iter_mut().enumerate() {
            *o = digit(b[2 * i])? << 4 | digit(b[2 * i + 1])?;
        }
        Some(Addr(out))
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// An altitude `τ`: starts with `@`, no NUL, no empty path segment.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Altitude(String);

impl Altitude {
    pub fn new(s: &str) -> Option<Altitude> {
        let rest = s.strip_prefix('@')?;
        if s.contains('\0') || rest.split('/').any(str::is_empty) {
            return None;
        }
        Some(Altitude(s.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

macro_rules! name {
    ($(#[$doc:meta])* $t:ident) => {
        $(#[$doc])*
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $t(String);

        impl $t {
            /// UTF-8 with no NUL.
            pub fn new(s: &str) -> Option<$t> {
                (!s.contains('\0')).then(|| $t(s.to_string()))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

name!(
    /// A `sym` constant.
    Sym
);
name!(
    /// A predicate name.
    Predicate
);
name!(
    /// A variable name.
    Var
);
name!(
    /// A rule id.
    RuleId
);

/// A `nat` domain constant (positions), never a count.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nat(pub u64);

/// The arity a predicate is declared at.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Arity(pub u32);

/// A 1-based argument position. Checked against the predicate's [`Arity`]
/// when a program is built.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position(NonZeroU32);

impl Position {
    /// `None` for 0.
    pub fn new(p: u32) -> Option<Position> {
        NonZeroU32::new(p).map(Position)
    }

    pub fn get(self) -> u32 {
        self.0.get()
    }

    fn index(self) -> usize {
        self.0.get() as usize - 1
    }
}

/// A core round; 0 holds the facts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tick(pub u64);

/// A round of `W`'s outer loop; not comparable with a [`Tick`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Round(pub u64);

/// A round budget `k_max ≥ 1`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Budget(NonZeroU64);

impl Budget {
    /// `None` for 0.
    pub fn new(k: u64) -> Option<Budget> {
        NonZeroU64::new(k).map(Budget)
    }

    pub fn get(self) -> u64 {
        self.0.get()
    }
}

// ── Terms, atoms, rules ──────────────────────────────────────────────────

/// A constant: one of three tagged kinds (§4.1 "Terms").
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Const {
    Sym(Sym),
    Nat(Nat),
    Addr(Addr),
}

/// A term: a variable or a constant. Function-free: there is no third case.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Term {
    Var(Var),
    Const(Const),
}

/// `p(t₁, …, t_n)`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Atom {
    pub pred: Predicate,
    pub args: Vec<Term>,
}

/// A ground atom.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fact {
    pub pred: Predicate,
    pub args: Vec<Const>,
}

/// A rule of the positive core: `id: h :- b₁, …, b_k` (k ≥ 0). The body is
/// atoms, so negation can't be written:
///
/// ```
/// use engine::{Atom, PositiveRule, Predicate, RuleId};
/// let q = Atom { pred: Predicate::new("q").unwrap(), args: vec![] };
/// let _ = PositiveRule { id: RuleId::new("r").unwrap(), head: q.clone(), body: vec![q] };
/// ```
///
/// ```compile_fail
/// use engine::{Atom, Literal, PositiveRule, Predicate, RuleId};
/// let q = Atom { pred: Predicate::new("q").unwrap(), args: vec![] };
/// let _ = PositiveRule { id: RuleId::new("r").unwrap(), head: q.clone(), body: vec![Literal::Not(q)] };
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PositiveRule {
    pub id: RuleId,
    pub head: Atom,
    pub body: Vec<Atom>,
}

/// A body literal of a normal rule, for the well-founded wrapper.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Literal {
    Pos(Atom),
    Not(Atom),
}

/// A rule that may carry `not a`, for `W`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NormalRule {
    pub id: RuleId,
    pub head: Atom,
    pub body: Vec<Literal>,
}

/// `functional p(key…) → value`: at most one value of argument `value` per
/// value of the arguments `key`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Functional {
    pub pred: Predicate,
    pub key: Vec<Position>,
    pub value: Position,
}

/// A program that is not run, and why. Only construction finds these.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Dark {
    /// A variable that is not range-restricted (head, or a negative literal).
    Unsafe { rule: RuleId, var: Var },
    /// A predicate at two arities. Inside a program: `expected` is the
    /// least, `found` the next. Facts against a program: `expected` is the
    /// program's arity, `found` the fact's.
    Arity {
        pred: Predicate,
        expected: Arity,
        found: Arity,
    },
    /// A declaration whose `position` lies outside `1..=arity`, or whose
    /// value position sits inside its key (then `position` is the value).
    Functional { pred: Predicate, position: Position },
}

// ── Programs (typestates) ────────────────────────────────────────────────

/// A positive program: built only through the arity, safety and
/// declaration checks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PositiveProgram {
    rules: Vec<PositiveRule>,
    functional: Vec<Functional>,
    arities: BTreeMap<String, u32>,
}

/// A normal program (negative literals allowed), for `W`. Same checks;
/// a negative literal's variables must also be range-restricted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NormalProgram {
    rules: Vec<NormalRule>,
    functional: Vec<Functional>,
    arities: BTreeMap<String, u32>,
}

/// A rule seen through its literals, for the shared checks.
struct View<'a> {
    id: &'a RuleId,
    head: &'a Atom,
    pos: Vec<&'a Atom>,
    neg: Vec<&'a Atom>,
}

impl PositiveRule {
    fn view(&self) -> View<'_> {
        View {
            id: &self.id,
            head: &self.head,
            pos: self.body.iter().collect(),
            neg: vec![],
        }
    }
}

impl NormalRule {
    fn view(&self) -> View<'_> {
        let mut pos = Vec::new();
        let mut neg = Vec::new();
        for l in &self.body {
            match l {
                Literal::Pos(a) => pos.push(a),
                Literal::Not(a) => neg.push(a),
            }
        }
        View {
            id: &self.id,
            head: &self.head,
            pos,
            neg,
        }
    }
}

fn least_two(pred: &str, set: &BTreeSet<u32>) -> Result<u32, Dark> {
    let mut it = set.iter();
    let first = *it.next().expect("non-empty");
    match it.next() {
        Some(&found) => Err(Dark::Arity {
            pred: Predicate(pred.to_string()),
            expected: Arity(first),
            found: Arity(found),
        }),
        None => Ok(first),
    }
}

fn atom_vars(a: &Atom) -> BTreeSet<&Var> {
    a.args
        .iter()
        .filter_map(|t| match t {
            Term::Var(v) => Some(v),
            Term::Const(_) => None,
        })
        .collect()
}

/// The declarations, in sorted order, against the arities known so far.
/// A declaration on a predicate with no known arity is left for later.
fn check_decls(functional: &[Functional], arities: &BTreeMap<String, u32>) -> Result<(), Dark> {
    let mut decls: Vec<&Functional> = functional.iter().collect();
    decls.sort();
    for d in decls {
        let Some(&arity) = arities.get(d.pred.as_str()) else {
            continue;
        };
        for &p in d.key.iter().chain(std::iter::once(&d.value)) {
            if p.get() > arity {
                return Err(Dark::Functional {
                    pred: d.pred.clone(),
                    position: p,
                });
            }
        }
        if d.key.contains(&d.value) {
            return Err(Dark::Functional {
                pred: d.pred.clone(),
                position: d.value,
            });
        }
    }
    Ok(())
}

/// Arity, then range restriction, then declaration positions. Sorted walks
/// make the verdict independent of rule order.
fn check_program(
    views: &[View<'_>],
    functional: &[Functional],
) -> Result<BTreeMap<String, u32>, Dark> {
    let mut seen: BTreeMap<&str, BTreeSet<u32>> = BTreeMap::new();
    for v in views {
        for a in std::iter::once(v.head)
            .chain(v.pos.iter().copied())
            .chain(v.neg.iter().copied())
        {
            seen.entry(a.pred.as_str())
                .or_default()
                .insert(a.args.len() as u32);
        }
    }
    let mut arities = BTreeMap::new();
    for (p, set) in &seen {
        arities.insert(p.to_string(), least_two(p, set)?);
    }
    let mut order: Vec<&View<'_>> = views.iter().collect();
    order.sort_by(|a, b| (a.id, a.head, &a.pos, &a.neg).cmp(&(b.id, b.head, &b.pos, &b.neg)));
    for v in order {
        let bound: BTreeSet<&Var> = v.pos.iter().flat_map(|a| atom_vars(a)).collect();
        let mut needed = atom_vars(v.head);
        for a in &v.neg {
            needed.extend(atom_vars(a));
        }
        if let Some(var) = needed.difference(&bound).next() {
            return Err(Dark::Unsafe {
                rule: v.id.clone(),
                var: (*var).clone(),
            });
        }
    }
    check_decls(functional, &arities)?;
    Ok(arities)
}

impl PositiveProgram {
    pub fn new(
        rules: Vec<PositiveRule>,
        functional: Vec<Functional>,
    ) -> Result<PositiveProgram, Dark> {
        let views: Vec<View<'_>> = rules.iter().map(PositiveRule::view).collect();
        let arities = check_program(&views, &functional)?;
        Ok(PositiveProgram {
            rules,
            functional,
            arities,
        })
    }

    pub fn rules(&self) -> &[PositiveRule] {
        &self.rules
    }

    pub fn functional(&self) -> &[Functional] {
        &self.functional
    }
}

impl NormalProgram {
    pub fn new(rules: Vec<NormalRule>, functional: Vec<Functional>) -> Result<NormalProgram, Dark> {
        let views: Vec<View<'_>> = rules.iter().map(NormalRule::view).collect();
        let arities = check_program(&views, &functional)?;
        Ok(NormalProgram {
            rules,
            functional,
            arities,
        })
    }

    pub fn rules(&self) -> &[NormalRule] {
        &self.rules
    }

    pub fn functional(&self) -> &[Functional] {
        &self.functional
    }
}

// ── Facts and the instance ───────────────────────────────────────────────

/// The fact store `F`: ground atoms at one altitude `τ`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Facts {
    pub altitude: Altitude,
    pub atoms: Vec<Fact>,
}

/// A program with a fact store whose arities agree with its signature.
/// After [`Instance::new`], every run is total.
#[derive(Clone, Debug)]
pub struct Instance<'p, P> {
    program: &'p P,
    facts: Facts,
    sufficient: Budget,
}

fn check_facts(
    arities: &BTreeMap<String, u32>,
    functional: &[Functional],
    facts: &Facts,
) -> Result<(), Dark> {
    let mut extra: BTreeMap<&str, BTreeSet<u32>> = BTreeMap::new();
    let mut atoms: Vec<&Fact> = facts.atoms.iter().collect();
    atoms.sort();
    for f in atoms {
        let n = f.args.len() as u32;
        match arities.get(f.pred.as_str()) {
            Some(&expected) if expected != n => {
                return Err(Dark::Arity {
                    pred: f.pred.clone(),
                    expected: Arity(expected),
                    found: Arity(n),
                });
            }
            Some(_) => {}
            None => {
                extra.entry(f.pred.as_str()).or_default().insert(n);
            }
        }
    }
    let mut all = arities.clone();
    for (p, set) in &extra {
        all.insert(p.to_string(), least_two(p, set)?);
    }
    check_decls(functional, &all)
}

/// `|B| − |F| + 1` (§4.1 A3, PROVED): `|B|` is every predicate over every
/// constant the rules and facts mention, saturating.
fn sufficient(rule_atoms: &[&Atom], facts: &Facts) -> Budget {
    let mut consts: BTreeSet<&Const> = BTreeSet::new();
    let mut arity: BTreeMap<&str, usize> = BTreeMap::new();
    for f in &facts.atoms {
        consts.extend(&f.args);
        arity.insert(f.pred.as_str(), f.args.len());
    }
    for a in rule_atoms {
        arity.insert(a.pred.as_str(), a.args.len());
        for t in &a.args {
            if let Term::Const(c) = t {
                consts.insert(c);
            }
        }
    }
    let n = consts.len() as u64;
    let b = arity.values().fold(0u64, |acc, &k| {
        acc.saturating_add(n.saturating_pow(k as u32))
    });
    let f = facts.atoms.iter().collect::<BTreeSet<_>>().len() as u64;
    Budget::new(b.saturating_sub(f).saturating_add(1)).expect("≥ 1")
}

mod sealed {
    pub trait Sealed {
        fn arities(&self) -> &std::collections::BTreeMap<String, u32>;
        fn decls(&self) -> &[super::Functional];
        fn rule_atoms(&self) -> Vec<&super::Atom>;
    }
}

/// [`PositiveProgram`] or [`NormalProgram`]: something an [`Instance`] can
/// hold. Sealed.
pub trait Program: sealed::Sealed {}

impl sealed::Sealed for PositiveProgram {
    fn arities(&self) -> &BTreeMap<String, u32> {
        &self.arities
    }
    fn decls(&self) -> &[Functional] {
        &self.functional
    }
    fn rule_atoms(&self) -> Vec<&Atom> {
        self.rules
            .iter()
            .flat_map(|r| std::iter::once(&r.head).chain(&r.body))
            .collect()
    }
}

impl sealed::Sealed for NormalProgram {
    fn arities(&self) -> &BTreeMap<String, u32> {
        &self.arities
    }
    fn decls(&self) -> &[Functional] {
        &self.functional
    }
    fn rule_atoms(&self) -> Vec<&Atom> {
        self.rules
            .iter()
            .flat_map(|r| {
                std::iter::once(&r.head).chain(r.body.iter().map(|l| match l {
                    Literal::Pos(a) | Literal::Not(a) => a,
                }))
            })
            .collect()
    }
}

impl Program for PositiveProgram {}
impl Program for NormalProgram {}

impl<'p, P: Program> Instance<'p, P> {
    /// Facts against the program's signature: a fact at another arity is
    /// `Dark::Arity { expected: program's, found: fact's }`.
    pub fn new(program: &'p P, facts: Facts) -> Result<Self, Dark> {
        check_facts(program.arities(), program.decls(), &facts)?;
        let sufficient = sufficient(&program.rule_atoms(), &facts);
        Ok(Instance {
            program,
            facts,
            sufficient,
        })
    }
    /// `|B| − |F| + 1`: a budget under which a positive run can't stop at
    /// its budget.
    pub fn sufficient_budget(&self) -> Budget {
        self.sufficient
    }

    pub fn facts(&self) -> &Facts {
        &self.facts
    }
}

// ── Runs (typestates) ────────────────────────────────────────────────────

/// One deposit: an atom's address and the tick it first fell at.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Deposit {
    pub addr: Addr,
    pub tick: Tick,
}

/// The stop of [`run_to_fixpoint`]: `fixed at T`, round `T` had an empty
/// delta. There is no budget case.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fixed(pub Tick);

/// The stop of [`run_bounded`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bounded {
    /// `fixed at T`.
    Fixed(Tick),
    /// `budget at k_max`: round `k_max` still had a non-empty delta.
    Budget(Budget),
}

/// `functional_violation(p, key, least, other)`: a reported finding, never
/// a crash. `least < other` in `Const` order.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionalViolation {
    pred: Predicate,
    key: Vec<Const>,
    least: Const,
    other: Const,
}

impl FunctionalViolation {
    /// `None` unless `least < other`.
    pub fn new(pred: Predicate, key: Vec<Const>, least: Const, other: Const) -> Option<Self> {
        (least < other).then_some(FunctionalViolation {
            pred,
            key,
            least,
            other,
        })
    }

    pub fn pred(&self) -> &Predicate {
        &self.pred
    }

    pub fn key(&self) -> &[Const] {
        &self.key
    }

    pub fn least(&self) -> &Const {
        &self.least
    }

    pub fn other(&self) -> &Const {
        &self.other
    }
}

/// `(M, D, stop)`, plus functionality findings. `S` is [`Fixed`] or
/// [`Bounded`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Run<S> {
    /// `M`, keyed (and so sorted) by address.
    pub model: BTreeMap<Addr, Fact>,
    /// The store-merge fragment address of `M` (children sorted by address).
    pub model_addr: Addr,
    /// `D`: one deposit per atom of `M`, sorted by `(tick, addr)`.
    pub deposits: Vec<Deposit>,
    pub stop: S,
    pub findings: Vec<FunctionalViolation>,
}

// ── W ────────────────────────────────────────────────────────────────────

/// A well-founded status (closed world for facts only).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    True,
    Undefined,
    False,
}

/// `clear` iff nothing is undefined, else `opaque`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Clear,
    Opaque,
}

/// A status splinter: `fact` moved from undefined to false at `round`
/// (an atom of `over_{round−1}` missing from `over_round`). Nothing is deleted.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Retraction {
    /// The retracted atom's address.
    pub addr: Addr,
    pub fact: Fact,
    pub round: Round,
    /// The status splinter deposited for it: the atom
    /// `status(addr, undefined, false)` at `@pact/status`. The round is a
    /// deposit coordinate, never in this address.
    pub status_addr: Addr,
}

/// The well-founded model: `true = under`, `undefined = over ∖ under`,
/// `false` = the rest.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WellFounded {
    pub altitude: Altitude,
    pub true_atoms: BTreeMap<Addr, Fact>,
    pub undefined: BTreeMap<Addr, Fact>,
    /// Sorted by `(round, addr)`.
    pub retractions: Vec<Retraction>,
    pub outcome: Outcome,
    /// Functionality findings on `true` only: definite violations.
    /// Undefined atoms can't violate definitively, and `γ`'s intermediate
    /// findings are not reported (§4.1 A5).
    pub findings: Vec<FunctionalViolation>,
}

impl WellFounded {
    /// The status of any ground atom at this model's altitude. Closed world
    /// for facts only: an atom neither true nor undefined is false.
    pub fn status(&self, fact: &Fact) -> Status {
        let a = addr(&self.altitude, fact);
        if self.true_atoms.contains_key(&a) {
            Status::True
        } else if self.undefined.contains_key(&a) {
            Status::Undefined
        } else {
            Status::False
        }
    }
}

/// `W`: the alternating fixpoint (math §5.3), two core calls per round,
/// stopping at consecutive equality of `under`. Each `γ` runs to its fixed
/// point (§4.1 A5).
pub fn well_founded(instance: &Instance<'_, NormalProgram>) -> WellFounded {
    let program = instance.program;
    let facts = &instance.facts;
    let rules: Vec<Compiled> = program.rules.iter().map(|r| compile(&r.view())).collect();
    let keys = keys(&program.functional);
    // γ(S): one core run with each `not a` read as the EDB test `a ∉ S`.
    let gamma = |s: &Rel| -> Rel {
        let (ticks, _, _) = least_model(&rules, &keys, &facts.atoms, None, Some(s));
        let mut m = Rel::new();
        for f in ticks.into_keys() {
            rel_insert(&mut m, f.pred.as_str(), f.args);
        }
        m
    };
    let tau = &facts.altitude;
    let status_tau = Altitude(STATUS_ALTITUDE.to_string());

    // under₀ = F; over_k = γ(under_k); under_{k+1} = γ(over_k).
    let mut under = Rel::new();
    for f in &facts.atoms {
        rel_insert(&mut under, f.pred.as_str(), f.args.clone());
    }
    let mut over = gamma(&under);
    let mut retractions = Vec::new();
    let mut round = 0u64;
    loop {
        let next_under = gamma(&over);
        // Non-monotone outer loop: stop on consecutive equality (Prop. 1.9).
        if next_under == under {
            break;
        }
        under = next_under;
        let next_over = gamma(&under);
        round += 1;
        // γ is antitone (Prop. 5.5): over only shrinks. Each atom that
        // leaves it moved from undefined to false: a status splinter.
        for (p, tuples) in &over {
            for args in tuples {
                if !rel_contains(&next_over, p, args) {
                    let fact = Fact {
                        pred: Predicate(p.clone()),
                        args: args.clone(),
                    };
                    let a = addr(tau, &fact);
                    let status = Fact {
                        pred: Predicate("status".into()),
                        args: vec![
                            Const::Addr(a),
                            Const::Sym(Sym("undefined".into())),
                            Const::Sym(Sym("false".into())),
                        ],
                    };
                    let status_addr = addr(&status_tau, &status);
                    retractions.push(Retraction {
                        addr: a,
                        fact,
                        round: Round(round),
                        status_addr,
                    });
                }
            }
        }
        over = next_over;
    }
    retractions.sort_by(|a, b| (a.round, a.addr).cmp(&(b.round, b.addr)));

    let addressed = |r: &Rel, minus: Option<&Rel>| -> BTreeMap<Addr, Fact> {
        let mut out = BTreeMap::new();
        for (p, tuples) in r {
            for args in tuples {
                if minus.is_some_and(|m| rel_contains(m, p, args)) {
                    continue;
                }
                let fact = Fact {
                    pred: Predicate(p.clone()),
                    args: args.clone(),
                };
                out.insert(addr(tau, &fact), fact);
            }
        }
        out
    };
    let true_atoms = addressed(&under, None);
    let undefined = addressed(&over, Some(&under));
    let outcome = if undefined.is_empty() {
        Outcome::Clear
    } else {
        Outcome::Opaque
    };
    let findings = violations(&keys, &under);
    WellFounded {
        altitude: facts.altitude.clone(),
        true_atoms,
        undefined,
        retractions,
        outcome,
        findings,
    }
}

// ── Addressing (§5.0 bytes; §4.1 atom content) ───────────────────────────

/// Tag bytes, pinned by §4.1: sym 0x00, nat 0x01, addr 0x02.
const TAG_SYM: u8 = 0x00;
const TAG_NAT: u8 = 0x01;
const TAG_ADDR: u8 = 0x02;

/// The fragment label of `M`'s store-merge fragment (§4.1).
const MODEL_LABEL: &[u8] = b"model";

/// The altitude of W's status splinters (§4.1).
const STATUS_ALTITUDE: &str = "@pact/status";

fn put_u64(h: &mut blake3::Hasher, n: u64) {
    h.update(&n.to_be_bytes());
}

fn put_enc(h: &mut blake3::Hasher, b: &[u8]) {
    put_u64(h, b.len() as u64);
    h.update(b);
}

/// `τ`: the altitude's UTF-8, NUL, then `0x00` (root grammar `@mirror`).
fn put_tau(h: &mut blake3::Hasher, altitude: &Altitude) {
    h.update(altitude.as_str().as_bytes());
    h.update(&[0x00, 0x00]);
}

fn push_enc(out: &mut Vec<u8>, b: &[u8]) {
    out.extend_from_slice(&(b.len() as u64).to_be_bytes());
    out.extend_from_slice(b);
}

/// `content = enc(p) ‖ u64(n) ‖ arg₁ ‖ … ‖ arg_n`, `arg = tag ‖ enc(bytes)`.
fn content(fact: &Fact) -> Vec<u8> {
    let mut c = Vec::new();
    push_enc(&mut c, fact.pred.as_str().as_bytes());
    c.extend_from_slice(&(fact.args.len() as u64).to_be_bytes());
    for a in &fact.args {
        match a {
            Const::Sym(s) => {
                c.push(TAG_SYM);
                push_enc(&mut c, s.as_str().as_bytes());
            }
            Const::Nat(n) => {
                c.push(TAG_NAT);
                push_enc(&mut c, &n.0.to_be_bytes());
            }
            Const::Addr(x) => {
                c.push(TAG_ADDR);
                push_enc(&mut c, x.as_bytes());
            }
        }
    }
    c
}

/// `addr = H("splinter\0" ‖ τ ‖ enc(content))` (§5.0; §4.1 "Fact store").
/// One formula for every atom in the system.
pub fn addr(altitude: &Altitude, fact: &Fact) -> Addr {
    let mut h = blake3::Hasher::new();
    h.update(b"splinter\0");
    put_tau(&mut h, altitude);
    put_enc(&mut h, &content(fact));
    Addr(*h.finalize().as_bytes())
}

/// `H("fragment\0" ‖ τ ‖ enc(ℓ) ‖ u64(k) ‖ addr(x₀) ‖ …)`, children in the
/// order given (the caller sorts them for a store-merge).
fn fragment(altitude: &Altitude, label: &[u8], children: &[Addr]) -> Addr {
    let mut h = blake3::Hasher::new();
    h.update(b"fragment\0");
    put_tau(&mut h, altitude);
    put_enc(&mut h, label);
    put_u64(&mut h, children.len() as u64);
    for c in children {
        h.update(c.as_bytes());
    }
    Addr(*h.finalize().as_bytes())
}

// ── The compiled program ─────────────────────────────────────────────────

/// A term with variables numbered per rule.
#[derive(Clone, Debug)]
enum Slot {
    Var(usize),
    Const(Const),
}

#[derive(Clone, Debug)]
struct Pattern {
    pred: String,
    args: Vec<Slot>,
}

#[derive(Clone, Debug)]
struct Compiled {
    head: Pattern,
    pos: Vec<Pattern>,
    neg: Vec<Pattern>,
    vars: usize,
}

fn compile(rule: &View<'_>) -> Compiled {
    let mut names: Vec<&Var> = Vec::new();
    // Number variables in order of first occurrence: positive body, then
    // negative body, then head.
    fn slots<'a>(a: &'a Atom, names: &mut Vec<&'a Var>) -> Pattern {
        let args = a
            .args
            .iter()
            .map(|t| match t {
                Term::Const(c) => Slot::Const(c.clone()),
                Term::Var(v) => match names.iter().position(|n| *n == v) {
                    Some(i) => Slot::Var(i),
                    None => {
                        names.push(v);
                        Slot::Var(names.len() - 1)
                    }
                },
            })
            .collect();
        Pattern {
            pred: a.pred.as_str().to_string(),
            args,
        }
    }
    let pos = rule.pos.iter().map(|a| slots(a, &mut names)).collect();
    let neg = rule.neg.iter().map(|a| slots(a, &mut names)).collect();
    let head = slots(rule.head, &mut names);
    Compiled {
        head,
        pos,
        neg,
        vars: names.len(),
    }
}

// ── Relations ────────────────────────────────────────────────────────────

/// Ground atoms grouped by predicate; tuples sorted.
type Rel = BTreeMap<String, BTreeSet<Vec<Const>>>;

fn rel_contains(r: &Rel, pred: &str, args: &[Const]) -> bool {
    r.get(pred).is_some_and(|s| s.contains(args))
}

fn rel_insert(r: &mut Rel, pred: &str, args: Vec<Const>) -> bool {
    r.entry(pred.to_string()).or_default().insert(args)
}

/// Bind `pat` against `tuple`; on success return the newly bound slots.
fn unify(pat: &Pattern, tuple: &[Const], env: &mut [Option<Const>]) -> Option<Vec<usize>> {
    if pat.args.len() != tuple.len() {
        return None;
    }
    let mut bound = Vec::new();
    for (s, v) in pat.args.iter().zip(tuple) {
        let ok = match s {
            Slot::Const(c) => c == v,
            Slot::Var(i) => match &env[*i] {
                Some(w) => w == v,
                None => {
                    env[*i] = Some(v.clone());
                    bound.push(*i);
                    true
                }
            },
        };
        if !ok {
            for i in bound {
                env[i] = None;
            }
            return None;
        }
    }
    Some(bound)
}

fn instantiate(pat: &Pattern, env: &[Option<Const>]) -> Vec<Const> {
    pat.args
        .iter()
        .map(|s| match s {
            Slot::Const(c) => c.clone(),
            Slot::Var(i) => env[*i]
                .clone()
                .expect("safe rule: head variable bound by the body"),
        })
        .collect()
}

// ── The functionality index (§3.3) ───────────────────────────────────────

/// A declaration with 0-based positions: `key` arguments determine `value`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    pred: String,
    key: Vec<usize>,
    value: usize,
}

fn key_of(k: &Key, tuple: &[Const]) -> Vec<Const> {
    k.key.iter().map(|&p| tuple[p].clone()).collect()
}

/// The checked declarations, 0-based and sorted (order-independent).
fn keys(functional: &[Functional]) -> Vec<Key> {
    let mut out: Vec<Key> = functional
        .iter()
        .map(|d| Key {
            pred: d.pred.as_str().to_string(),
            key: d.key.iter().map(|p| p.index()).collect(),
            value: d.value.index(),
        })
        .collect();
    out.sort();
    out.dedup();
    out
}

/// `M` indexed by each declared key: pred → key values → tuples. With
/// Ford's functionality there is at most one tuple per key, so a join on a
/// bound key is one lookup instead of a scan (math §5.4; linearity stays a
/// CONJECTURE until T6c measures it).
#[derive(Default)]
struct Index {
    keys: Vec<Key>,
    by_key: BTreeMap<String, BTreeMap<Vec<Const>, BTreeSet<Vec<Const>>>>,
}

impl Index {
    fn new(keys: &[Key]) -> Self {
        // One key per predicate drives the join; the first in sorted order.
        let mut chosen: Vec<Key> = Vec::new();
        for k in keys {
            if !chosen.iter().any(|c| c.pred == k.pred) {
                chosen.push(k.clone());
            }
        }
        Index {
            keys: chosen,
            by_key: BTreeMap::new(),
        }
    }

    fn key(&self, pred: &str) -> Option<&Key> {
        self.keys.iter().find(|k| k.pred == pred)
    }

    fn insert(&mut self, pred: &str, tuple: &[Const]) {
        if let Some(k) = self.key(pred) {
            let kv = key_of(k, tuple);
            self.by_key
                .entry(pred.to_string())
                .or_default()
                .entry(kv)
                .or_default()
                .insert(tuple.to_vec());
        }
    }

    /// The tuples of `pat.pred` whose key equals `pat`'s key under `env`,
    /// if `pat.pred` is indexed and the key is fully bound.
    fn lookup(
        &self,
        pat: &Pattern,
        env: &[Option<Const>],
    ) -> Option<Option<&BTreeSet<Vec<Const>>>> {
        let k = self.key(&pat.pred)?;
        let mut kv = Vec::with_capacity(k.key.len());
        for &p in &k.key {
            match pat.args.get(p)? {
                Slot::Const(c) => kv.push(c.clone()),
                Slot::Var(i) => kv.push(env[*i].clone()?),
            }
        }
        Some(self.by_key.get(&pat.pred).and_then(|m| m.get(&kv)))
    }
}

/// `functional_violation(p, key, least, other)` for every key with more
/// than one value: each value after the least is paired with the least.
fn violations(keys: &[Key], model: &Rel) -> Vec<FunctionalViolation> {
    let mut out = BTreeSet::new();
    for k in keys {
        let Some(tuples) = model.get(&k.pred) else {
            continue;
        };
        let mut values: BTreeMap<Vec<Const>, BTreeSet<Const>> = BTreeMap::new();
        for t in tuples {
            values
                .entry(key_of(k, t))
                .or_default()
                .insert(t[k.value].clone());
        }
        for (key, vs) in values {
            let mut it = vs.into_iter();
            let Some(least) = it.next() else { continue };
            for other in it {
                out.insert(FunctionalViolation {
                    pred: Predicate(k.pred.clone()),
                    key: key.clone(),
                    least: least.clone(),
                    other,
                });
            }
        }
    }
    out.into_iter().collect()
}

/// One semi-naive round for one rule: literal `delta_at` ranges over `delta`,
/// every other positive literal over `model` (through the index when its
/// key is bound). Negative literals are tests against the fixed context
/// `ctx` (the core gets none).
struct RoundJoin<'a> {
    rule: &'a Compiled,
    model: &'a Rel,
    index: &'a Index,
    delta: &'a Rel,
    delta_at: Option<usize>,
    ctx: Option<&'a Rel>,
}

impl RoundJoin<'_> {
    fn join(&self, i: usize, env: &mut Vec<Option<Const>>, out: &mut Rel) {
        if i == self.rule.pos.len() {
            if let Some(ctx) = self.ctx {
                for n in &self.rule.neg {
                    if rel_contains(ctx, &n.pred, &instantiate(n, env)) {
                        return;
                    }
                }
            }
            let head = instantiate(&self.rule.head, env);
            if !rel_contains(self.model, &self.rule.head.pred, &head) {
                rel_insert(out, &self.rule.head.pred, head);
            }
            return;
        }
        let pat = &self.rule.pos[i];
        let tuples = if self.delta_at == Some(i) {
            self.delta.get(&pat.pred)
        } else {
            match self.index.lookup(pat, env) {
                Some(hit) => hit,
                None => self.model.get(&pat.pred),
            }
        };
        let Some(tuples) = tuples else { return };
        for t in tuples {
            if let Some(bound) = unify(pat, t, env) {
                self.join(i + 1, env, out);
                for b in bound {
                    env[b] = None;
                }
            }
        }
    }
}

/// The least model of `rules` over `facts`, semi-naively, with negative
/// literals read against `ctx`. `budget = None` runs to the fixed point.
/// Returns `M` with first ticks, the stop, and the findings on `M`.
fn least_model(
    rules: &[Compiled],
    keys: &[Key],
    facts: &[Fact],
    budget: Option<Budget>,
    ctx: Option<&Rel>,
) -> (BTreeMap<Fact, u64>, Bounded, Vec<FunctionalViolation>) {
    let mut model: Rel = Rel::new();
    let mut index = Index::new(keys);
    let mut tick_of: BTreeMap<Fact, u64> = BTreeMap::new();
    for f in facts {
        if rel_insert(&mut model, f.pred.as_str(), f.args.clone()) {
            index.insert(f.pred.as_str(), &f.args);
            tick_of.insert(f.clone(), 0);
        }
    }
    // Δ₀ = M₀ = F.
    let mut delta = model.clone();
    let mut t = 0u64;
    loop {
        if let Some(b) = budget {
            if t == b.get() {
                let found = violations(keys, &model);
                return (tick_of, Bounded::Budget(b), found);
            }
        }
        t += 1;
        let mut next = Rel::new();
        for r in rules {
            let mut env = vec![None; r.vars];
            if r.pos.is_empty() {
                // A rule with no positive body fires once, in round 1.
                if t == 1 {
                    RoundJoin {
                        rule: r,
                        model: &model,
                        index: &index,
                        delta: &delta,
                        delta_at: None,
                        ctx,
                    }
                    .join(0, &mut env, &mut next);
                }
                continue;
            }
            for i in 0..r.pos.len() {
                RoundJoin {
                    rule: r,
                    model: &model,
                    index: &index,
                    delta: &delta,
                    delta_at: Some(i),
                    ctx,
                }
                .join(0, &mut env, &mut next);
            }
        }
        if next.values().all(BTreeSet::is_empty) {
            let found = violations(keys, &model);
            return (tick_of, Bounded::Fixed(Tick(t)), found);
        }
        for (p, tuples) in &next {
            for args in tuples {
                rel_insert(&mut model, p, args.clone());
                index.insert(p, args);
                tick_of.insert(
                    Fact {
                        pred: Predicate(p.clone()),
                        args: args.clone(),
                    },
                    t,
                );
            }
        }
        delta = next;
    }
}

/// Address `M` and lay out `D`.
fn record(
    altitude: &Altitude,
    ticks: BTreeMap<Fact, u64>,
) -> (BTreeMap<Addr, Fact>, Addr, Vec<Deposit>) {
    let mut model = BTreeMap::new();
    let mut deposits = Vec::with_capacity(ticks.len());
    for (f, t) in ticks {
        let a = addr(altitude, &f);
        deposits.push(Deposit {
            addr: a,
            tick: Tick(t),
        });
        model.insert(a, f);
    }
    deposits.sort_by_key(|d| (d.tick, d.addr));
    let children: Vec<Addr> = model.keys().copied().collect();
    let model_addr = fragment(altitude, MODEL_LABEL, &children);
    (model, model_addr, deposits)
}

fn core(
    instance: &Instance<'_, PositiveProgram>,
    budget: Option<Budget>,
) -> (
    BTreeMap<Addr, Fact>,
    Addr,
    Vec<Deposit>,
    Bounded,
    Vec<FunctionalViolation>,
) {
    let program = instance.program;
    let rules: Vec<Compiled> = program.rules.iter().map(|r| compile(&r.view())).collect();
    let (ticks, stop, findings) = least_model(
        &rules,
        &keys(&program.functional),
        &instance.facts.atoms,
        budget,
        None,
    );
    // Prop. 5.4 / §4.1 A3: T ≤ |B| − |F| + 1. Unreachable by proof.
    if let Bounded::Fixed(Tick(t)) = stop {
        debug_assert!(
            t <= instance.sufficient.get(),
            "Prop. 5.4: fixed at {t} > |B| − |F| + 1"
        );
    }
    let (model, model_addr, deposits) = record(&instance.facts.altitude, ticks);
    (model, model_addr, deposits, stop, findings)
}

/// `E(P, F) → (M, D, fixed at T)`: runs to the least fixed point. Total.
pub fn run_to_fixpoint(instance: &Instance<'_, PositiveProgram>) -> Run<Fixed> {
    let (model, model_addr, deposits, stop, findings) = core(instance, None);
    let Bounded::Fixed(t) = stop else {
        unreachable!("no budget was given")
    };
    Run {
        model,
        model_addr,
        deposits,
        stop: Fixed(t),
        findings,
    }
}

/// `E(P, F, k_max) → (M, D, stop)`: stops at the fixed point or at round
/// `k_max`, whichever comes first.
pub fn run_bounded(instance: &Instance<'_, PositiveProgram>, budget: Budget) -> Run<Bounded> {
    let (model, model_addr, deposits, stop, findings) = core(instance, Some(budget));
    Run {
        model,
        model_addr,
        deposits,
        stop,
        findings,
    }
}
