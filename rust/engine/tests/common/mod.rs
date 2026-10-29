//! Test-side fixture helpers. Fixtures are data in the test (audit §4):
//! a hand-written splitter turns `path(?x, ?z) :- path(?x, ?y), edge(?y, ?z)`
//! into `engine` values. `?x` is a variable (the pact vectors' convention);
//! a run of digits is a `Nat`; anything else is a `Sym`. No JSON, no serde.

#![allow(dead_code)]

use engine::{
    Atom, Budget, Const, Dark, Fact, Facts, Instance, Literal, Nat, NormalProgram, NormalRule,
    PositiveProgram, PositiveRule, Predicate, RuleId, Sym, Term, Var,
};

pub fn sym(s: &str) -> Const {
    Const::Sym(Sym::new(s).unwrap())
}

pub fn nat(n: u64) -> Const {
    Const::Nat(Nat(n))
}

pub fn pred(s: &str) -> Predicate {
    Predicate::new(s).unwrap()
}

pub fn var(s: &str) -> Var {
    Var::new(s).unwrap()
}

pub fn rule_id(s: &str) -> RuleId {
    RuleId::new(s).unwrap()
}

pub fn alt(s: &str) -> engine::Altitude {
    engine::Altitude::new(s).unwrap()
}

/// Split at top-level commas (outside parentheses).
fn split_top(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    for ch in s.chars() {
        match ch {
            '(' => {
                depth += 1;
                cur.push(ch);
            }
            ')' => {
                depth -= 1;
                cur.push(ch);
            }
            ',' if depth == 0 => {
                out.push(cur.trim().to_string());
                cur.clear();
            }
            _ => cur.push(ch),
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out
}

fn constant(s: &str) -> Const {
    if !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) {
        nat(s.parse().unwrap())
    } else {
        sym(s)
    }
}

fn term(s: &str) -> Term {
    match s.strip_prefix('?') {
        Some(v) => Term::Var(var(v)),
        None => Term::Const(constant(s)),
    }
}

/// `p(a, ?x)` or `p`.
pub fn atom(s: &str) -> Atom {
    let s = s.trim();
    match s.find('(') {
        None => Atom {
            pred: pred(s),
            args: vec![],
        },
        Some(open) => {
            let inner = &s[open + 1..s.rfind(')').expect("closing paren")];
            Atom {
                pred: pred(s[..open].trim()),
                args: split_top(inner).iter().map(|a| term(a)).collect(),
            }
        }
    }
}

/// A ground atom.
pub fn fact(s: &str) -> Fact {
    let a = atom(s);
    Fact {
        pred: a.pred,
        args: a
            .args
            .into_iter()
            .map(|t| match t {
                Term::Const(c) => c,
                Term::Var(v) => panic!("fact {s:?} has variable ?{}", v.as_str()),
            })
            .collect(),
    }
}

/// `head :- b1, not b2` or `head` (k = 0).
pub fn normal_rule(id: &str, s: &str) -> NormalRule {
    let (head, body) = match s.split_once(":-") {
        Some((h, b)) => (h, b),
        None => (s, ""),
    };
    let body = split_top(body)
        .iter()
        .map(|l| match l.strip_prefix("not ") {
            Some(rest) => Literal::Not(atom(rest)),
            None => Literal::Pos(atom(l)),
        })
        .collect();
    NormalRule {
        id: rule_id(id),
        head: atom(head),
        body,
    }
}

/// A positive rule; panics on `not` (the type can't hold it).
pub fn rule(id: &str, s: &str) -> PositiveRule {
    let r = normal_rule(id, s);
    PositiveRule {
        id: r.id,
        head: r.head,
        body: r
            .body
            .into_iter()
            .map(|l| match l {
                Literal::Pos(a) => a,
                Literal::Not(_) => panic!("{s:?}: negation in a positive rule"),
            })
            .collect(),
    }
}

pub fn try_program(rules: &[(&str, &str)]) -> Result<PositiveProgram, Dark> {
    PositiveProgram::new(rules.iter().map(|(id, s)| rule(id, s)).collect(), vec![])
}

pub fn program(rules: &[(&str, &str)]) -> PositiveProgram {
    try_program(rules).unwrap()
}

pub fn try_normal(rules: &[(&str, &str)]) -> Result<NormalProgram, Dark> {
    NormalProgram::new(
        rules.iter().map(|(id, s)| normal_rule(id, s)).collect(),
        vec![],
    )
}

pub fn normal(rules: &[(&str, &str)]) -> NormalProgram {
    try_normal(rules).unwrap()
}

/// The same program over other rules (for permutations and unions).
pub fn reprogram(p: &PositiveProgram, rules: Vec<PositiveRule>) -> PositiveProgram {
    PositiveProgram::new(rules, p.functional().to_vec()).unwrap()
}

pub const TEST_ALTITUDE: &str = "@datalog/fact";

pub fn facts(atoms: &[&str]) -> Facts {
    Facts {
        altitude: alt(TEST_ALTITUDE),
        atoms: atoms.iter().map(|s| fact(s)).collect(),
    }
}

/// `run_to_fixpoint` on `(p, f)`; the instance must build.
pub fn fixpoint(p: &PositiveProgram, f: &Facts) -> engine::Run<engine::Fixed> {
    engine::run_to_fixpoint(&Instance::new(p, f.clone()).expect("instance"))
}

/// `run_bounded` on `(p, f)` with budget `k ≥ 1`.
pub fn bounded(p: &PositiveProgram, f: &Facts, k: u64) -> engine::Run<engine::Bounded> {
    engine::run_bounded(
        &Instance::new(p, f.clone()).expect("instance"),
        Budget::new(k).unwrap(),
    )
}

/// `W` on `(p, f)`.
pub fn wf(p: &NormalProgram, f: &Facts) -> engine::WellFounded {
    engine::well_founded(&Instance::new(p, f.clone()).expect("instance"))
}

/// Deterministic LCG (Knuth MMIX constants). No `rand`.
pub struct Lcg(pub u64);

impl Lcg {
    pub fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 33
    }
    /// Uniform-ish in `0..n`.
    pub fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
    pub fn chance(&mut self, num: u64, den: u64) -> bool {
        self.below(den) < num
    }
}

/// A fixed-sample permutation of `v` by Fisher-Yates over the LCG.
pub fn shuffled<T: Clone>(v: &[T], rng: &mut Lcg) -> Vec<T> {
    let mut out = v.to_vec();
    for i in (1..out.len()).rev() {
        let j = rng.below(i as u64 + 1) as usize;
        out.swap(i, j);
    }
    out
}

/// `fact → tick` for a run, joining the model with the deposits by address.
pub fn ticks<S>(run: &engine::Run<S>) -> std::collections::BTreeMap<Fact, u64> {
    let by_addr: std::collections::BTreeMap<_, _> =
        run.deposits.iter().map(|d| (d.addr, d.tick.0)).collect();
    run.model
        .iter()
        .map(|(a, f)| {
            (
                f.clone(),
                *by_addr.get(a).expect("every atom of M is deposited"),
            )
        })
        .collect()
}
