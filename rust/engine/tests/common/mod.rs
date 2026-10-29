//! Test-side fixture helpers. Fixtures are data in the test (audit §4):
//! a hand-written splitter turns `path(?x, ?z) :- path(?x, ?y), edge(?y, ?z)`
//! into `engine` values. `?x` is a variable (the pact vectors' convention);
//! a run of digits is a `Nat`; anything else is a `Sym`. No JSON, no serde.

#![allow(dead_code)]

use engine::{Atom, Const, Fact, Facts, Literal, Program, Rule, Term};

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
        Const::Nat(s.parse().unwrap())
    } else {
        Const::Sym(s.to_string())
    }
}

fn term(s: &str) -> Term {
    match s.strip_prefix('?') {
        Some(v) => Term::Var(v.to_string()),
        None => Term::Const(constant(s)),
    }
}

/// `p(a, ?x)` or `p`.
pub fn atom(s: &str) -> Atom {
    let s = s.trim();
    match s.find('(') {
        None => Atom { pred: s.to_string(), args: vec![] },
        Some(open) => {
            let pred = s[..open].trim().to_string();
            let inner = &s[open + 1..s.rfind(')').expect("closing paren")];
            Atom { pred, args: split_top(inner).iter().map(|a| term(a)).collect() }
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
                Term::Var(v) => panic!("fact {s:?} has variable ?{v}"),
            })
            .collect(),
    }
}

/// `head :- b1, not b2` or `head` (k = 0).
pub fn rule(id: &str, s: &str) -> Rule {
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
    Rule { id: id.to_string(), head: atom(head), body }
}

pub fn program(rules: &[(&str, &str)]) -> Program {
    Program {
        rules: rules.iter().map(|(id, s)| rule(id, s)).collect(),
        functional: vec![],
    }
}

pub const TEST_ALTITUDE: &str = "@datalog/fact";

pub fn facts(atoms: &[&str]) -> Facts {
    Facts { altitude: TEST_ALTITUDE.to_string(), atoms: atoms.iter().map(|s| fact(s)).collect() }
}

/// Deterministic LCG (Knuth MMIX constants). No `rand`.
pub struct Lcg(pub u64);

impl Lcg {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
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
pub fn ticks(run: &engine::Run) -> std::collections::BTreeMap<Fact, u64> {
    let by_addr: std::collections::BTreeMap<_, _> =
        run.deposits.iter().map(|d| (d.addr, d.tick)).collect();
    run.model
        .iter()
        .map(|(a, f)| (f.clone(), *by_addr.get(a).expect("every atom of M is deposited")))
        .collect()
}
