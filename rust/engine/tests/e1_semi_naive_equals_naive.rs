//! E1 (spec §4.1; audit §4 item 2): semi-naive = naive (math Prop. 5.4),
//! equal `M` and equal tick per atom. A deposit's tick is the first naive
//! round whose model contains the atom.
//!
//! The naive evaluator below is written from math §5.2 (Def. 5.3 with
//! `T` applied to the whole model each round, i.e. without the delta),
//! not from `E`'s source. It has its own term and atom types and shares
//! nothing with `engine` but the final comparison. It is the first diverse
//! implementation of the core semantics for Prop. 6.4's DDC, so it stays
//! independent: do not refactor it to call into `engine`.

mod common;

use common::{alt, facts, fixpoint, nat, pred, program, rule_id, sym, ticks, var, Lcg, TEST_ALTITUDE};
use std::collections::{BTreeMap, BTreeSet};

// ── The independent naive evaluator ──────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum C {
    S(String),
    N(u64),
}

#[derive(Clone, Debug)]
enum T {
    V(String),
    K(C),
}

type G = (String, Vec<C>); // a ground atom

#[derive(Clone, Debug)]
struct NRule {
    head: (String, Vec<T>),
    body: Vec<(String, Vec<T>)>,
}

/// Every substitution that grounds `body[i..]` inside `m`, extending `env`.
fn ground(
    body: &[(String, Vec<T>)],
    m: &BTreeSet<G>,
    env: &mut BTreeMap<String, C>,
    out: &mut Vec<BTreeMap<String, C>>,
) {
    let Some(((p, args), rest)) = body.split_first() else {
        out.push(env.clone());
        return;
    };
    for (q, vals) in m {
        if q != p || vals.len() != args.len() {
            continue;
        }
        let mut bound = Vec::new();
        let mut ok = true;
        for (t, v) in args.iter().zip(vals) {
            match t {
                T::K(c) => {
                    if c != v {
                        ok = false;
                        break;
                    }
                }
                T::V(x) => match env.get(x) {
                    Some(w) if w != v => {
                        ok = false;
                        break;
                    }
                    Some(_) => {}
                    None => {
                        env.insert(x.clone(), v.clone());
                        bound.push(x.clone());
                    }
                },
            }
        }
        if ok {
            ground(rest, m, env, out);
        }
        for x in bound {
            env.remove(&x);
        }
    }
}

/// Naive: `M₀ = F`; `M_{t+1} = M_t ∪ T(M_t)`; stop when nothing is added.
/// Returns each atom with the first round that contains it.
fn naive(rules: &[NRule], f: &BTreeSet<G>) -> BTreeMap<G, u64> {
    let mut first: BTreeMap<G, u64> = f.iter().map(|a| (a.clone(), 0)).collect();
    let mut m: BTreeSet<G> = f.clone();
    let mut t = 0u64;
    loop {
        t += 1;
        let mut next = m.clone();
        for r in rules {
            let mut subs = Vec::new();
            ground(&r.body, &m, &mut BTreeMap::new(), &mut subs);
            for s in subs {
                let head: Vec<C> = r
                    .head
                    .1
                    .iter()
                    .map(|t| match t {
                        T::K(c) => c.clone(),
                        T::V(x) => s[x].clone(),
                    })
                    .collect();
                next.insert((r.head.0.clone(), head));
            }
        }
        if next.len() == m.len() {
            return first;
        }
        for a in &next {
            first.entry(a.clone()).or_insert(t);
        }
        m = next;
    }
}

// ── Conversion to `engine` (the only shared surface) ─────────────────────

fn to_const(c: &C) -> engine::Const {
    match c {
        C::S(s) => sym(s),
        C::N(n) => nat(*n),
    }
}

fn to_term(t: &T) -> engine::Term {
    match t {
        T::V(x) => engine::Term::Var(var(x)),
        T::K(c) => engine::Term::Const(to_const(c)),
    }
}

fn to_atom((p, args): &(String, Vec<T>)) -> engine::Atom {
    engine::Atom { pred: pred(p), args: args.iter().map(to_term).collect() }
}

fn to_fact((p, args): &G) -> engine::Fact {
    engine::Fact { pred: pred(p), args: args.iter().map(to_const).collect() }
}

fn to_program(rules: &[NRule]) -> Result<engine::PositiveProgram, engine::Dark> {
    engine::PositiveProgram::new(
        rules
            .iter()
            .enumerate()
            .map(|(i, r)| engine::PositiveRule {
                id: rule_id(&format!("r{i}")),
                head: to_atom(&r.head),
                body: r.body.iter().map(to_atom).collect(),
            })
            .collect(),
        vec![],
    )
}

/// Run both and compare `M` and the tick of every atom.
fn agree(rules: &[NRule], f: &BTreeSet<G>, label: &str) {
    let expected: BTreeMap<engine::Fact, u64> =
        naive(rules, f).iter().map(|(a, t)| (to_fact(a), *t)).collect();
    let store = engine::Facts { altitude: alt(TEST_ALTITUDE), atoms: f.iter().map(to_fact).collect() };
    let program = to_program(rules).unwrap_or_else(|d| panic!("{label}: safe positive program went dark: {d:?}"));
    let run = fixpoint(&program, &store);
    assert_eq!(run.deposits.len(), run.model.len(), "{label}: one deposit per atom of M");
    let got = ticks(&run);
    assert_eq!(got, expected, "{label}: semi-naive ≠ naive\nrules: {rules:#?}\nfacts: {f:?}");
}

// ── Fixtures ──────────────────────────────────────────────────────────────

fn v(x: &str) -> T {
    T::V(x.to_string())
}
fn s(x: &str) -> C {
    C::S(x.to_string())
}

fn tc_rules() -> Vec<NRule> {
    vec![
        NRule {
            head: ("path".into(), vec![v("x"), v("y")]),
            body: vec![("edge".into(), vec![v("x"), v("y")])],
        },
        NRule {
            head: ("path".into(), vec![v("x"), v("z")]),
            body: vec![
                ("path".into(), vec![v("x"), v("y")]),
                ("edge".into(), vec![v("y"), v("z")]),
            ],
        },
    ]
}

#[test]
fn e1_transitive_closure_over_pact_vector_01_graph() {
    // pact vector 01's graph: the cycle b → c → b.
    let f: BTreeSet<G> = [("a", "b"), ("b", "c"), ("c", "b"), ("c", "d")]
        .iter()
        .map(|(x, y)| ("edge".to_string(), vec![s(x), s(y)]))
        .collect();
    agree(&tc_rules(), &f, "vector 01");
}

#[test]
fn e1_chain_of_length_10() {
    let f: BTreeSet<G> =
        (0..10).map(|i| ("edge".to_string(), vec![C::N(i), C::N(i + 1)])).collect();
    agree(&tc_rules(), &f, "chain 10");
}

#[test]
fn e1_fixture_helpers_agree_with_naive_types() {
    // The same program through the text helpers gives the same model:
    // guards the helpers themselves.
    let p = program(&[
        ("path-base", "path(?x, ?y) :- edge(?x, ?y)"),
        ("path-step", "path(?x, ?z) :- path(?x, ?y), edge(?y, ?z)"),
    ]);
    let run = fixpoint(&p, &facts(&["edge(a, b)", "edge(b, c)"]));
    let got: BTreeSet<_> = run.model.values().cloned().collect();
    let want: BTreeSet<_> = ["edge(a, b)", "edge(b, c)", "path(a, b)", "path(b, c)", "path(a, c)"]
        .iter()
        .map(|s| common::fact(s))
        .collect();
    assert_eq!(got, want);
}

// ── The property: ≥ 1000 random programs from a fixed seed ──────────────

struct Gen {
    rng: Lcg,
}

impl Gen {
    fn constant(&mut self, n_sym: u64, n_nat: u64) -> C {
        let k = self.rng.below(n_sym + n_nat);
        if k < n_sym {
            C::S(format!("c{k}"))
        } else {
            C::N(k - n_sym)
        }
    }

    fn program(&mut self) -> (Vec<NRule>, BTreeSet<G>) {
        let n_pred = 2 + self.rng.below(4); // 2..=5
        let arity: Vec<usize> = (0..n_pred)
            .map(|_| if self.rng.chance(1, 10) { 0 } else { 1 + self.rng.below(2) as usize })
            .collect();
        let n_sym = 1 + self.rng.below(3);
        let n_nat = self.rng.below(2);
        let vars = ["X", "Y", "Z", "W"];

        let mut f = BTreeSet::new();
        for _ in 0..self.rng.below(9) {
            let p = self.rng.below(n_pred) as usize;
            let args = (0..arity[p]).map(|_| self.constant(n_sym, n_nat)).collect();
            f.insert((format!("p{p}"), args));
        }

        let mut rules = Vec::new();
        for _ in 0..1 + self.rng.below(5) {
            let h = self.rng.below(n_pred) as usize;
            let body_len = if self.rng.chance(1, 12) { 0 } else { 1 + self.rng.below(3) as usize };
            let mut body = Vec::new();
            let mut body_vars: Vec<&str> = Vec::new();
            for _ in 0..body_len {
                let p = self.rng.below(n_pred) as usize;
                let args = (0..arity[p])
                    .map(|_| {
                        if self.rng.chance(4, 5) {
                            let x = vars[self.rng.below(vars.len() as u64) as usize];
                            if !body_vars.contains(&x) {
                                body_vars.push(x);
                            }
                            v(x)
                        } else {
                            T::K(self.constant(n_sym, n_nat))
                        }
                    })
                    .collect();
                body.push((format!("p{p}"), args));
            }
            let head_args = (0..arity[h])
                .map(|_| {
                    if !body_vars.is_empty() && self.rng.chance(5, 6) {
                        v(body_vars[self.rng.below(body_vars.len() as u64) as usize])
                    } else {
                        T::K(self.constant(n_sym, n_nat))
                    }
                })
                .collect();
            rules.push(NRule { head: (format!("p{h}"), head_args), body });
        }
        (rules, f)
    }
}

fn recursive(rules: &[NRule]) -> bool {
    rules.iter().any(|r| r.body.iter().any(|b| b.0 == r.head.0))
}

fn mutually_recursive(rules: &[NRule]) -> bool {
    rules.iter().any(|r1| {
        rules.iter().any(|r2| {
            r1.head.0 != r2.head.0
                && r1.body.iter().any(|b| b.0 == r2.head.0)
                && r2.body.iter().any(|b| b.0 == r1.head.0)
        })
    })
}

fn repeated_variable(rules: &[NRule]) -> bool {
    rules.iter().any(|r| {
        let mut seen = Vec::new();
        for (_, args) in &r.body {
            for t in args {
                if let T::V(x) = t {
                    if seen.contains(x) {
                        return true;
                    }
                    seen.push(x.clone());
                }
            }
        }
        false
    })
}

#[test]
fn e1_semi_naive_equals_naive_on_1000_random_programs() {
    let mut gen = Gen { rng: Lcg(0x6d69_7272_6f72_2045) }; // "mirror E"
    let (mut rec, mut mutual, mut repeated, mut multi_tick) = (0, 0, 0, 0);
    for i in 0..1000 {
        let (rules, f) = gen.program();
        rec += recursive(&rules) as u32;
        mutual += mutually_recursive(&rules) as u32;
        repeated += repeated_variable(&rules) as u32;
        multi_tick += (naive(&rules, &f).values().max().copied().unwrap_or(0) >= 2) as u32;
        agree(&rules, &f, &format!("program {i}"));
    }
    // The sample must actually exercise what Prop. 5.4 is about.
    assert!(rec >= 100, "only {rec} recursive programs");
    assert!(mutual >= 20, "only {mutual} mutually recursive programs");
    assert!(repeated >= 100, "only {repeated} programs with repeated variables");
    assert!(multi_tick >= 100, "only {multi_tick} programs deriving past tick 1");
}
