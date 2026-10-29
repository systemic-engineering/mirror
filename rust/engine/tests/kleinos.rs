//! Kleinos at the engine layer (spec §4.4; math
//! `docs/math/2026-10-29-mara-engine-frequency-and-kleinos.md` §2).
//!
//! Composition is rule union: `L = lfp(P₁ ∪ P₂, F₁ ∪ F₂)`,
//! `E₃ = L ∖ (L₁ ∪ L₂)` as a set of addresses. `lfp` is
//! `run_to_fixpoint` (§4.2), whose stop is `Fixed` by type.
//!
//! K-E5 (no Fiedler law) is math only: a recorded counterexample, not a
//! test. F-E1 (frequency) needs §4.3's `Frequency` API, which is outside
//! Seam's engine-core audit; not here.

mod common;

use common::{
    alt, fact, facts, fixpoint, normal, pred, program, rule_id, sym, var, wf, Lcg, TEST_ALTITUDE,
};
use engine::{
    Addr, Atom, Const, Fact, Facts, Fixed, Literal, NormalProgram, NormalRule, PositiveProgram,
    PositiveRule, Status, Term, Var,
};
use std::collections::{BTreeMap, BTreeSet};

type Side = (PositiveProgram, Facts);

// ── lfp, union, E₃ ───────────────────────────────────────────────────────

/// `run_to_fixpoint`: its stop is `Fixed` by type.
fn lfp(p: &PositiveProgram, f: &Facts) -> engine::Run<Fixed> {
    fixpoint(p, f)
}

fn union(a: &Side, b: &Side) -> Side {
    let mut rules = a.0.rules().to_vec();
    rules.extend(b.0.rules().iter().cloned());
    let mut decls = a.0.functional().to_vec();
    decls.extend(b.0.functional().iter().cloned());
    let mut f = a.1.clone();
    f.atoms.extend(b.1.atoms.iter().cloned());
    (
        PositiveProgram::new(rules, decls).expect("shared signature"),
        f,
    )
}

fn keys(run: &engine::Run<Fixed>) -> BTreeSet<Addr> {
    run.model.keys().copied().collect()
}

/// `(L₁, L₂, L, E₃)`.
fn compose(
    a: &Side,
    b: &Side,
) -> (
    BTreeSet<Addr>,
    BTreeSet<Addr>,
    BTreeSet<Addr>,
    BTreeSet<Addr>,
) {
    let l1 = keys(&lfp(&a.0, &a.1));
    let l2 = keys(&lfp(&b.0, &b.1));
    let (p, f) = union(a, b);
    let l = keys(&lfp(&p, &f));
    let e3 = l
        .iter()
        .filter(|x| !l1.contains(*x) && !l2.contains(*x))
        .copied()
        .collect();
    (l1, l2, l, e3)
}

fn addrs(atoms: &[&str]) -> BTreeSet<Addr> {
    atoms
        .iter()
        .map(|s| engine::addr(&alt(TEST_ALTITUDE), &fact(s)))
        .collect()
}

/// Prop. 2.4's crossing edge: a body predicate of a rule of one side that
/// the other side heads by a rule or holds facts of.
fn crosses(a: &Side, b: &Side) -> bool {
    let one_way = |x: &Side, y: &Side| {
        let produced: BTreeSet<&str> =
            y.0.rules()
                .iter()
                .map(|r| r.head.pred.as_str())
                .chain(y.1.atoms.iter().map(|f| f.pred.as_str()))
                .collect();
        x.0.rules()
            .iter()
            .any(|r| r.body.iter().any(|at| produced.contains(at.pred.as_str())))
    };
    one_way(a, b) || one_way(b, a)
}

// ── Seeded pairs of positive programs over a shared vocabulary ───────────

struct Gen(Lcg);

impl Gen {
    fn constant(&mut self) -> Const {
        sym(&format!("c{}", self.0.below(3)))
    }

    /// One side over predicates `q0..q4` at the shared `arity`.
    fn side(&mut self, arity: &[usize]) -> Side {
        let vars = ["X", "Y", "Z"];
        let n = arity.len() as u64;
        let mut rules = Vec::new();
        for k in 0..self.0.below(4) {
            let h = self.0.below(n) as usize;
            let mut body = Vec::new();
            let mut bound: Vec<&str> = Vec::new();
            for _ in 0..1 + self.0.below(2) {
                let p = self.0.below(n) as usize;
                let args = (0..arity[p])
                    .map(|_| {
                        if self.0.chance(4, 5) {
                            let v = vars[self.0.below(3) as usize];
                            if !bound.contains(&v) {
                                bound.push(v);
                            }
                            Term::Var(var(v))
                        } else {
                            Term::Const(self.constant())
                        }
                    })
                    .collect();
                body.push(Atom {
                    pred: pred(&format!("q{p}")),
                    args,
                });
            }
            let args = (0..arity[h])
                .map(|_| {
                    if !bound.is_empty() && self.0.chance(5, 6) {
                        Term::Var(var(bound[self.0.below(bound.len() as u64) as usize]))
                    } else {
                        Term::Const(self.constant())
                    }
                })
                .collect();
            rules.push(PositiveRule {
                id: rule_id(&format!("r{k}")),
                head: Atom {
                    pred: pred(&format!("q{h}")),
                    args,
                },
                body,
            });
        }
        let mut atoms = Vec::new();
        for _ in 0..self.0.below(5) {
            let p = self.0.below(n) as usize;
            atoms.push(Fact {
                pred: pred(&format!("q{p}")),
                args: (0..arity[p]).map(|_| self.constant()).collect(),
            });
        }
        (
            PositiveProgram::new(rules, vec![]).expect("safe by construction"),
            Facts {
                altitude: alt(TEST_ALTITUDE),
                atoms,
            },
        )
    }

    fn pair(&mut self) -> (Side, Side) {
        let arity: Vec<usize> = (0..5).map(|_| 1 + self.0.below(2) as usize).collect();
        (self.side(&arity), self.side(&arity))
    }
}

fn pairs() -> Vec<(Side, Side)> {
    let mut g = Gen(Lcg(0x6b6c_6569_6e6f_7321)); // "kleinos!"
    (0..500).map(|_| g.pair()).collect()
}

// ── K-E1 sovereignty ─────────────────────────────────────────────────────

#[test]
fn k_e1_sovereignty_on_500_seeded_pairs() {
    let mut grew = 0;
    for (i, (a, b)) in pairs().iter().enumerate() {
        let (l1, l2, l, e3) = compose(a, b);
        assert!(l1.is_subset(&l), "pair {i}: L₁ ⊄ L");
        assert!(l2.is_subset(&l), "pair {i}: L₂ ⊄ L");
        grew += (!e3.is_empty()) as u32;
    }
    assert!(
        grew >= 50,
        "only {grew} pairs with an emergent third: the sample is too tame"
    );
}

// ── K-E2 the emergent third, as addresses ────────────────────────────────

#[test]
fn k_e2_a_chain_across_the_seam_emerges_one_atom() {
    let a = (program(&[("q", "q(?x) :- p(?x)")]), facts(&["p(a)"]));
    let b = (program(&[("r", "r(?x) :- q(?x)")]), facts(&[]));
    let (_, _, _, e3) = compose(&a, &b);
    assert_eq!(e3, addrs(&["r(a)"]));
}

#[test]
fn k_e2_crossing_on_p_with_nothing_new_emerges_nothing() {
    let a = (program(&[("q", "q(?x) :- p(?x)")]), facts(&["p(a)"]));
    let b = (program(&[]), facts(&["p(a)"]));
    assert!(crosses(&a, &b), "they cross on p");
    let (_, _, _, e3) = compose(&a, &b);
    assert_eq!(e3, BTreeSet::new());
}

#[test]
fn k_e2_the_6_cycle_emerges_33_atoms_not_one() {
    let a = (
        program(&[("base", "path(?x, ?y) :- e(?x, ?y)")]),
        facts(&["e(0, 1)", "e(1, 2)", "e(2, 3)"]),
    );
    let b = (
        program(&[("trans", "path(?x, ?z) :- path(?x, ?y), path(?y, ?z)")]),
        facts(&["e(3, 4)", "e(4, 5)", "e(5, 0)"]),
    );
    let (l1, l2, l, e3) = compose(&a, &b);
    let all_paths: Vec<String> = (0..6)
        .flat_map(|i| (0..6).map(move |j| format!("path({i}, {j})")))
        .collect();
    let all_paths = addrs(&all_paths.iter().map(String::as_str).collect::<Vec<_>>());
    assert_eq!(l.intersection(&all_paths).count(), 36);
    assert_eq!(l1.intersection(&all_paths).count(), 3);
    assert_eq!(l2.intersection(&all_paths).count(), 0);
    let alone = addrs(&["path(0, 1)", "path(1, 2)", "path(2, 3)"]);
    assert_eq!(e3, all_paths.difference(&alone).copied().collect());
    assert_eq!(
        e3.len(),
        33,
        "refutes \"exactly one emergent third\" at this layer"
    );
}

// ── K-E3 crossing is necessary, not sufficient ───────────────────────────

#[test]
fn k_e3_an_emergent_third_implies_a_crossing_edge() {
    let (mut emergent, mut crossing_without) = (0, 0);
    for (i, (a, b)) in pairs().iter().enumerate() {
        let (_, _, _, e3) = compose(a, b);
        if !e3.is_empty() {
            emergent += 1;
            assert!(crosses(a, b), "pair {i}: E₃ ≠ ∅ with no crossing edge");
        } else if crosses(a, b) {
            crossing_without += 1;
        }
    }
    assert!(emergent >= 50, "only {emergent} emergent pairs");
    // Not sufficient: the sample also holds crossing pairs with E₃ = ∅.
    assert!(
        crossing_without >= 20,
        "only {crossing_without} crossing pairs with E₃ = ∅"
    );
}

// ── K-E4 fusion is idempotence ───────────────────────────────────────────

#[test]
fn k_e4_composing_a_program_with_itself_changes_nothing() {
    let fixtures = pairs().into_iter().map(|(a, _)| a).chain([(
        program(&[
            ("base", "path(?x, ?y) :- e(?x, ?y)"),
            ("step", "path(?x, ?z) :- path(?x, ?y), e(?y, ?z)"),
        ]),
        facts(&["e(a, b)", "e(b, c)", "e(c, b)", "e(c, d)"]),
    )]);
    for (i, a) in fixtures.enumerate() {
        let once = lfp(&a.0, &a.1);
        let (p, f) = union(&a, &a);
        let twice = lfp(&p, &f);
        assert_eq!(twice.model_addr, once.model_addr, "fixture {i}");
        let (_, _, _, e3) = compose(&a, &a);
        assert!(e3.is_empty(), "fixture {i}");
    }
}

// ── K-E6 retraction under W: a part disagrees with the glue ──────────────

/// Atoms true in the part and not true in the glue (math §2.6, read as
/// Prop. 2.6 states it: "a true atom of a part is false in the glue").
fn retracted_by_glue(
    part: &(NormalProgram, Facts),
    glue: &(NormalProgram, Facts),
) -> BTreeSet<Fact> {
    let wp = wf(&part.0, &part.1);
    let wg = wf(&glue.0, &glue.1);
    wp.true_atoms
        .values()
        .filter(|a| wg.status(a) != Status::True)
        .cloned()
        .collect()
}

fn normal_union(a: &(NormalProgram, Facts), b: &(NormalProgram, Facts)) -> (NormalProgram, Facts) {
    let mut rules = a.0.rules().to_vec();
    rules.extend(b.0.rules().iter().cloned());
    let mut f = a.1.clone();
    f.atoms.extend(b.1.atoms.iter().cloned());
    (
        NormalProgram::new(rules, vec![]).expect("shared signature"),
        f,
    )
}

/// A positive side, read as a normal program for `W`.
fn as_normal(s: &Side) -> (NormalProgram, Facts) {
    let rules =
        s.0.rules()
            .iter()
            .map(|r| NormalRule {
                id: r.id.clone(),
                head: r.head.clone(),
                body: r.body.iter().cloned().map(Literal::Pos).collect(),
            })
            .collect();
    (NormalProgram::new(rules, vec![]).unwrap(), s.1.clone())
}

#[test]
fn k_e6_a_true_atom_of_a_part_is_false_in_the_glue() {
    let p1 = (normal(&[("p", "p :- not q")]), facts(&[]));
    let p2 = (normal(&[("q", "q")]), facts(&[]));
    let glue = normal_union(&p1, &p2);
    let w1 = wf(&p1.0, &p1.1);
    let wg = wf(&glue.0, &glue.1);
    assert_eq!(w1.status(&fact("p")), Status::True);
    assert_eq!(wg.status(&fact("p")), Status::False);
    assert_eq!(wg.status(&fact("q")), Status::True);
    assert_eq!(retracted_by_glue(&p1, &glue), BTreeSet::from([fact("p")]));
}

#[test]
fn k_e6_on_positive_pairs_no_part_is_retracted() {
    for (i, (a, b)) in pairs().iter().enumerate().take(200) {
        let (a, b) = (as_normal(a), as_normal(b));
        let glue = normal_union(&a, &b);
        assert!(retracted_by_glue(&a, &glue).is_empty(), "pair {i}");
        assert!(retracted_by_glue(&b, &glue).is_empty(), "pair {i}");
    }
}

// ── K-E7 the tick's shape ────────────────────────────────────────────────

/// Every grounding of `body[i..]` inside `model`, as variable bindings.
fn groundings(
    body: &[&Atom],
    model: &BTreeMap<Fact, u64>,
    env: &mut BTreeMap<Var, Const>,
    out: &mut Vec<BTreeMap<Var, Const>>,
) {
    let Some((a, rest)) = body.split_first() else {
        out.push(env.clone());
        return;
    };
    for f in model
        .keys()
        .filter(|f| f.pred == a.pred && f.args.len() == a.args.len())
    {
        let mut added = Vec::new();
        let ok = a.args.iter().zip(&f.args).all(|(t, v)| match t {
            Term::Const(c) => c == v,
            Term::Var(x) => match env.get(x) {
                Some(w) => w == v,
                None => {
                    env.insert(x.clone(), v.clone());
                    added.push(x.clone());
                    true
                }
            },
        });
        if ok {
            groundings(rest, model, env, out);
        }
        for x in added {
            env.remove(&x);
        }
    }
}

fn ground(a: &Atom, env: &BTreeMap<Var, Const>) -> Fact {
    Fact {
        pred: a.pred.clone(),
        args: a
            .args
            .iter()
            .map(|t| match t {
                Term::Const(c) => c.clone(),
                Term::Var(x) => env[x].clone(),
            })
            .collect(),
    }
}

/// Prop. 2.1: each atom at tick t ≥ 1 has a rule instance whose premises
/// all sit at ticks ≤ t − 1, at least one at exactly t − 1.
fn tick_shape(p: &PositiveProgram, f: &Facts, label: &str) {
    let run = lfp(p, f);
    let ticks = common::ticks(&run);
    for (atom, &t) in ticks.iter().filter(|(_, &t)| t >= 1) {
        let found = p.rules().iter().any(|r| {
            let body: Vec<&Atom> = r.body.iter().collect();
            let mut subs = Vec::new();
            groundings(&body, &ticks, &mut BTreeMap::new(), &mut subs);
            subs.iter().any(|s| {
                if ground(&r.head, s) != *atom {
                    return false;
                }
                let premise: Vec<u64> = body.iter().map(|b| ticks[&ground(b, s)]).collect();
                if premise.is_empty() {
                    return t == 1;
                }
                premise.iter().all(|&u| u + 1 <= t) && premise.iter().any(|&u| u + 1 == t)
            })
        });
        assert!(
            found,
            "{label}: {atom:?} at tick {t} has no instance shaped by tick {}",
            t - 1
        );
    }
}

#[test]
fn k_e7_every_tick_leans_on_the_last_one() {
    let tc = program(&[
        ("base", "path(?x, ?y) :- edge(?x, ?y)"),
        ("step", "path(?x, ?z) :- path(?x, ?y), edge(?y, ?z)"),
    ]);
    tick_shape(
        &tc,
        &facts(&["edge(a, b)", "edge(b, c)", "edge(c, b)", "edge(c, d)"]),
        "vector 01",
    );
    let chain: Vec<String> = (0..10).map(|i| format!("edge({i}, {})", i + 1)).collect();
    tick_shape(
        &tc,
        &facts(&chain.iter().map(String::as_str).collect::<Vec<_>>()),
        "chain 10",
    );
    for (i, (a, b)) in pairs().iter().enumerate() {
        tick_shape(&a.0, &a.1, &format!("pair {i} side 1"));
        let (p, f) = union(a, b);
        tick_shape(&p, &f, &format!("pair {i} union"));
    }
}
