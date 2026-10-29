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

use std::collections::{BTreeMap, BTreeSet};

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

// ── Addressing (§5.0 bytes; §4.1 atom content) ───────────────────────────

/// Tag bytes, in §4.1's listing order. §4.1 names the tag byte but does not
/// pin its value; no test pins derived-atom hex (audit §3.2) until it does.
const TAG_SYM: u8 = 0x00;
const TAG_NAT: u8 = 0x01;
const TAG_ADDR: u8 = 0x02;

/// The fragment label of `M`'s store-merge fragment. Not pinned by §4.1.
const MODEL_LABEL: &[u8] = b"model";

fn put_u64(h: &mut blake3::Hasher, n: u64) {
    h.update(&n.to_be_bytes());
}

fn put_enc(h: &mut blake3::Hasher, b: &[u8]) {
    put_u64(h, b.len() as u64);
    h.update(b);
}

/// `τ`: the altitude's UTF-8, NUL, then `0x00` (root grammar `@mirror`).
fn put_tau(h: &mut blake3::Hasher, altitude: &str) {
    h.update(altitude.as_bytes());
    h.update(&[0x00, 0x00]);
}

/// `addr = H("splinter\0" ‖ τ ‖ enc(p) ‖ u64(n) ‖ for each argument:
/// tag ‖ enc(bytes))` (§4.1, §5.0).
pub fn addr(altitude: &str, fact: &Fact) -> Addr {
    let mut h = blake3::Hasher::new();
    h.update(b"splinter\0");
    put_tau(&mut h, altitude);
    put_enc(&mut h, fact.pred.as_bytes());
    put_u64(&mut h, fact.args.len() as u64);
    for c in &fact.args {
        match c {
            Const::Sym(s) => {
                h.update(&[TAG_SYM]);
                put_enc(&mut h, s.as_bytes());
            }
            Const::Nat(n) => {
                h.update(&[TAG_NAT]);
                put_enc(&mut h, &n.to_be_bytes());
            }
            Const::Addr(a) => {
                h.update(&[TAG_ADDR]);
                put_enc(&mut h, a);
            }
        }
    }
    *h.finalize().as_bytes()
}

/// `H("fragment\0" ‖ τ ‖ enc(ℓ) ‖ u64(k) ‖ addr(x₀) ‖ …)`, children in the
/// order given (the caller sorts them for a store-merge).
fn fragment(altitude: &str, label: &[u8], children: impl ExactSizeIterator<Item = Addr>) -> Addr {
    let mut h = blake3::Hasher::new();
    h.update(b"fragment\0");
    put_tau(&mut h, altitude);
    put_enc(&mut h, label);
    put_u64(&mut h, children.len() as u64);
    for c in children {
        h.update(&c);
    }
    *h.finalize().as_bytes()
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

fn compile(rule: &Rule) -> Compiled {
    let mut names: Vec<&str> = Vec::new();
    // Number variables in order of first occurrence: positive body, then
    // negative body, then head.
    fn slots<'a>(a: &'a Atom, names: &mut Vec<&'a str>) -> Pattern {
        let args = a
            .args
            .iter()
            .map(|t| match t {
                Term::Const(c) => Slot::Const(c.clone()),
                Term::Var(v) => match names.iter().position(|n| *n == v.as_str()) {
                    Some(i) => Slot::Var(i),
                    None => {
                        names.push(v.as_str());
                        Slot::Var(names.len() - 1)
                    }
                },
            })
            .collect();
        Pattern { pred: a.pred.clone(), args }
    }
    let mut pos = Vec::new();
    let mut neg = Vec::new();
    for l in &rule.body {
        if let Literal::Pos(a) = l {
            pos.push(slots(a, &mut names));
        }
    }
    for l in &rule.body {
        if let Literal::Not(a) = l {
            neg.push(slots(a, &mut names));
        }
    }
    let head = slots(&rule.head, &mut names);
    Compiled { head, pos, neg, vars: names.len() }
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
            Slot::Var(i) => env[*i].clone().expect("safe rule: head variable bound by the body"),
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

/// `M` indexed by each declared key: pred → key values → tuples. With
/// Ford's functionality there is at most one tuple per key, so a join on a
/// bound key is one lookup instead of a scan (math §5.4; linearity stays a
/// CONJECTURE until T6b measures it).
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
        Index { keys: chosen, by_key: BTreeMap::new() }
    }

    fn key(&self, pred: &str) -> Option<&Key> {
        self.keys.iter().find(|k| k.pred == pred)
    }

    fn insert(&mut self, pred: &str, tuple: &[Const]) {
        if let Some(k) = self.key(pred) {
            let kv = key_of(k, tuple);
            self.by_key.entry(pred.to_string()).or_default().entry(kv).or_default().insert(tuple.to_vec());
        }
    }

    /// The tuples of `pat.pred` whose key equals `pat`'s key under `env`,
    /// if `pat.pred` is indexed and the key is fully bound.
    fn lookup(&self, pat: &Pattern, env: &[Option<Const>]) -> Option<Option<&BTreeSet<Vec<Const>>>> {
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

/// `functional_violation(p, key, v, v′)` for every key with more than one
/// value: each value after the least is paired with the least.
fn violations(keys: &[Key], model: &Rel) -> Vec<FunctionalViolation> {
    let mut out = BTreeSet::new();
    for k in keys {
        let Some(tuples) = model.get(&k.pred) else { continue };
        let mut values: BTreeMap<Vec<Const>, BTreeSet<Const>> = BTreeMap::new();
        for t in tuples {
            values.entry(key_of(k, t)).or_default().insert(t[k.value].clone());
        }
        for (key, vs) in values {
            let mut it = vs.into_iter();
            let Some(least) = it.next() else { continue };
            for v_prime in it {
                out.insert(FunctionalViolation {
                    pred: k.pred.clone(),
                    key: key.clone(),
                    v: least.clone(),
                    v_prime,
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
struct Round<'a> {
    rule: &'a Compiled,
    model: &'a Rel,
    index: &'a Index,
    delta: &'a Rel,
    delta_at: Option<usize>,
    ctx: Option<&'a Rel>,
}

impl Round<'_> {
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
/// literals read against `ctx`. Returns `M` with first ticks, the stop,
/// and the functionality findings on `M`.
fn least_model(
    rules: &[Compiled],
    keys: &[Key],
    facts: &[Fact],
    k_max: u64,
    ctx: Option<&Rel>,
) -> (BTreeMap<Fact, u64>, Stop, Vec<FunctionalViolation>) {
    let mut model: Rel = Rel::new();
    let mut index = Index::new(keys);
    let mut tick_of: BTreeMap<Fact, u64> = BTreeMap::new();
    for f in facts {
        if rel_insert(&mut model, &f.pred, f.args.clone()) {
            index.insert(&f.pred, &f.args);
            tick_of.insert(f.clone(), 0);
        }
    }
    // Δ₀ = M₀ = F.
    let mut delta = model.clone();
    let mut t = 0u64;
    loop {
        if t == k_max {
            let found = violations(keys, &model);
            return (tick_of, Stop::Budget(k_max), found);
        }
        t += 1;
        let mut next = Rel::new();
        for r in rules {
            let mut env = vec![None; r.vars];
            if r.pos.is_empty() {
                // A rule with no positive body fires once, in round 1.
                if t == 1 {
                    Round { rule: r, model: &model, index: &index, delta: &delta, delta_at: None, ctx }
                        .join(0, &mut env, &mut next);
                }
                continue;
            }
            for i in 0..r.pos.len() {
                Round { rule: r, model: &model, index: &index, delta: &delta, delta_at: Some(i), ctx }
                    .join(0, &mut env, &mut next);
            }
        }
        if next.values().all(BTreeSet::is_empty) {
            let found = violations(keys, &model);
            return (tick_of, Stop::Fixed(t), found);
        }
        for (p, tuples) in &next {
            for args in tuples {
                rel_insert(&mut model, p, args.clone());
                index.insert(p, args);
                tick_of.insert(Fact { pred: p.clone(), args: args.clone() }, t);
            }
        }
        delta = next;
    }
}

// ── Validation: a program that is not run is dark ────────────────────────

fn atom_vars(a: &Atom) -> BTreeSet<&str> {
    a.args
        .iter()
        .filter_map(|t| match t {
            Term::Var(v) => Some(v.as_str()),
            Term::Const(_) => None,
        })
        .collect()
}

/// Arity, then safety (range restriction), then, for the positive core,
/// negation. Every check walks sorted collections, so the verdict does not
/// depend on rule or fact order.
fn validate(program: &Program, facts: &Facts, allow_negation: bool) -> Result<(), Dark> {
    let mut arities: BTreeMap<&str, BTreeSet<usize>> = BTreeMap::new();
    for f in &facts.atoms {
        arities.entry(&f.pred).or_default().insert(f.args.len());
    }
    for r in &program.rules {
        arities.entry(&r.head.pred).or_default().insert(r.head.args.len());
        for l in &r.body {
            let (Literal::Pos(a) | Literal::Not(a)) = l;
            arities.entry(&a.pred).or_default().insert(a.args.len());
        }
    }
    for (pred, set) in &arities {
        let mut it = set.iter();
        if let (Some(&expected), Some(&found)) = (it.next(), it.next()) {
            return Err(Dark::Arity { pred: pred.to_string(), expected, found });
        }
    }

    let mut rules: Vec<&Rule> = program.rules.iter().collect();
    rules.sort();
    for r in &rules {
        let mut bound = BTreeSet::new();
        let mut needed = atom_vars(&r.head);
        for l in &r.body {
            match l {
                Literal::Pos(a) => bound.extend(atom_vars(a)),
                Literal::Not(a) => needed.extend(atom_vars(a)),
            }
        }
        if let Some(var) = needed.difference(&bound).next() {
            return Err(Dark::Unsafe { rule: r.id.clone(), var: var.to_string() });
        }
    }
    // Declarations: 1-based positions inside the predicate's arity, the
    // value outside the key. A predicate with no atoms anywhere is vacuous.
    let mut decls: Vec<&Functional> = program.functional.iter().collect();
    decls.sort();
    for d in decls {
        let Some(&arity) = arities.get(d.pred.as_str()).and_then(|s| s.iter().next()) else { continue };
        let in_range = |p: usize| (1..=arity).contains(&p);
        if !in_range(d.value) || !d.key.iter().all(|&p| in_range(p)) || d.key.contains(&d.value) {
            return Err(Dark::Functional { pred: d.pred.clone() });
        }
    }
    if !allow_negation {
        if let Some(r) = rules.iter().find(|r| r.body.iter().any(|l| matches!(l, Literal::Not(_)))) {
            return Err(Dark::Negation { rule: r.id.clone() });
        }
    }
    Ok(())
}

/// `|B|`, saturating: every predicate over every constant the program and
/// the facts mention (Prop. 5.4's bound on ticks).
fn herbrand_size(program: &Program, facts: &Facts) -> u64 {
    let mut consts: BTreeSet<&Const> = BTreeSet::new();
    let mut arity: BTreeMap<&str, usize> = BTreeMap::new();
    for f in &facts.atoms {
        consts.extend(&f.args);
        arity.insert(&f.pred, f.args.len());
    }
    for r in &program.rules {
        let atoms = std::iter::once(&r.head).chain(r.body.iter().map(|l| {
            let (Literal::Pos(a) | Literal::Not(a)) = l;
            a
        }));
        for a in atoms {
            arity.insert(&a.pred, a.args.len());
            for t in &a.args {
                if let Term::Const(c) = t {
                    consts.insert(c);
                }
            }
        }
    }
    let n = consts.len() as u64;
    arity
        .values()
        .fold(0u64, |acc, &k| acc.saturating_add(n.saturating_pow(k as u32)))
}

/// The validated declarations, 0-based and sorted (order-independent).
fn keys(program: &Program) -> Vec<Key> {
    let mut out: Vec<Key> = program
        .functional
        .iter()
        .map(|d| Key { pred: d.pred.clone(), key: d.key.iter().map(|p| p - 1).collect(), value: d.value - 1 })
        .collect();
    out.sort();
    out.dedup();
    out
}

/// `E(P, F, k_max) → (M, D, stop)`.
pub fn run(program: &Program, facts: &Facts, k_max: u64) -> Result<Run, Dark> {
    validate(program, facts, false)?;
    let rules: Vec<Compiled> = program.rules.iter().map(compile).collect();
    let (ticks, stop, findings) = least_model(&rules, &keys(program), &facts.atoms, k_max, None);
    // Prop. 5.4: at most |B| ticks carry deposits. Exceeding it is a bug in
    // E, reported, never looped on.
    let last = ticks.values().copied().max().unwrap_or(0);
    let bound = herbrand_size(program, facts);
    if last > bound {
        return Err(Dark::Bound { ticks: last, bound });
    }
    let tau = facts.altitude.as_str();
    let mut model = BTreeMap::new();
    let mut deposits = Vec::with_capacity(ticks.len());
    for (f, t) in ticks {
        let a = addr(tau, &f);
        deposits.push(Deposit { addr: a, tick: t });
        model.insert(a, f);
    }
    deposits.sort_by_key(|d| (d.tick, d.addr));
    let model_addr = fragment(tau, MODEL_LABEL, model.keys().copied().collect::<Vec<_>>().into_iter());
    Ok(Run { model, model_addr, deposits, stop, findings })
}
