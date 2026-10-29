//! Ford's functionality (spec §4.1 "functional p(1, 2) → 3"; audit §3.3,
//! §4 item 7). The declaration asserts at most one value of argument 3 per
//! value of arguments (1, 2) on the model. A violation is a reported finding
//! `functional_violation(p, key, v, v′)`, never a panic and never silently
//! first-wins.
//!
//! The PEG clauses below are math §5.1's table, transcribed by hand for
//! each sub-expression of tiny grammars (not compiled: the PEG → Horn
//! translation waits, audit §3.5). Terminals use §4.1's EDB: `lit(t, i, j)`
//! and `nolit(t, i)`. Expression names are `Sym` constants.

mod common;

use common::{bounded, fact, facts, nat, pred, rule, shuffled, sym, ticks, Lcg};
use engine::{
    Bounded, Const, Dark, Facts, Functional, FunctionalViolation, Instance, Position,
    PositiveProgram, PositiveRule,
};
use std::collections::BTreeSet;

fn pos(p: u32) -> Position {
    Position::new(p).unwrap()
}

fn m_functional() -> Functional {
    Functional {
        pred: pred("m"),
        key: vec![pos(1), pos(2)],
        value: pos(3),
    }
}

type Run = engine::Run<Bounded>;

// ── math §5.1, one helper per row, applied by hand ─────────────────────

/// `"c"`: m("c",i,i+1) :- input(i,"c").  f("c",i) :- ¬input(i,"c").
fn terminal(e: &str, c: &str) -> Vec<PositiveRule> {
    vec![
        rule(
            &format!("m-{e}"),
            &format!("m({e}, ?i, ?j) :- lit({c}, ?i, ?j)"),
        ),
        rule(&format!("f-{e}"), &format!("f({e}, ?i) :- nolit({c}, ?i)")),
    ]
}

/// `A`: m(A,i,j) :- m(R(A),i,j).  f(A,i) :- f(R(A),i).
fn nonterminal(a: &str, body: &str) -> Vec<PositiveRule> {
    vec![
        rule(
            &format!("m-{a}"),
            &format!("m({a}, ?i, ?j) :- m({body}, ?i, ?j)"),
        ),
        rule(&format!("f-{a}"), &format!("f({a}, ?i) :- f({body}, ?i)")),
    ]
}

/// `e₁ e₂`.
fn seq(e: &str, e1: &str, e2: &str) -> Vec<PositiveRule> {
    vec![
        rule(
            &format!("m-{e}"),
            &format!("m({e}, ?i, ?k) :- m({e1}, ?i, ?j), m({e2}, ?j, ?k)"),
        ),
        rule(&format!("f1-{e}"), &format!("f({e}, ?i) :- f({e1}, ?i)")),
        rule(
            &format!("f2-{e}"),
            &format!("f({e}, ?i) :- m({e1}, ?i, ?j), f({e2}, ?j)"),
        ),
    ]
}

/// `e₁ / e₂`.
fn choice(e: &str, e1: &str, e2: &str) -> Vec<PositiveRule> {
    vec![
        rule(
            &format!("m1-{e}"),
            &format!("m({e}, ?i, ?j) :- m({e1}, ?i, ?j)"),
        ),
        rule(
            &format!("m2-{e}"),
            &format!("m({e}, ?i, ?j) :- f({e1}, ?i), m({e2}, ?i, ?j)"),
        ),
        rule(
            &format!("f-{e}"),
            &format!("f({e}, ?i) :- f({e1}, ?i), f({e2}, ?i)"),
        ),
    ]
}

/// The EDB for `input` over terminals `ts`: lit for each matching byte,
/// nolit for every other (terminal, position) in 0..=n.
fn edb(input: &str, ts: &[&str]) -> Facts {
    let mut atoms = Vec::new();
    let bytes: Vec<char> = input.chars().collect();
    for i in 0..=bytes.len() {
        for t in ts {
            if i < bytes.len() && bytes[i].to_string() == *t {
                atoms.push(format!("lit({t}, {i}, {})", i + 1));
            } else {
                atoms.push(format!("nolit({t}, {i})"));
            }
        }
    }
    facts(&atoms.iter().map(String::as_str).collect::<Vec<_>>())
}

fn grammar(parts: Vec<Vec<PositiveRule>>) -> PositiveProgram {
    PositiveProgram::new(parts.into_iter().flatten().collect(), vec![m_functional()]).unwrap()
}

fn without_decls(p: &PositiveProgram) -> PositiveProgram {
    PositiveProgram::new(p.rules().to_vec(), vec![]).unwrap()
}

fn has(run: &Run, s: &str) -> bool {
    run.model.values().any(|f| *f == fact(s))
}

/// Has `(e, i)` neither an `m` nor an `f`?
fn undefined(run: &Run, e: &str, i: u64) -> bool {
    let e = sym(e);
    !run.model.values().any(|f| {
        (f.pred.as_str() == "m" || f.pred.as_str() == "f")
            && f.args.len() >= 2
            && f.args[0] == e
            && f.args[1] == nat(i)
    })
}

// ── Functional grammars: no findings ─────────────────────────────────────

#[test]
fn seams_example_s_choice_a_l_on_a_parses_and_l_is_undefined() {
    // S <- 'a' / L ; L <- L 'b'   on "a"
    let p = grammar(vec![
        terminal("ta", "a"),
        terminal("tb", "b"),
        choice("Sbody", "ta", "L"),
        nonterminal("S", "Sbody"),
        seq("Lb", "L", "tb"),
        nonterminal("L", "Lb"),
    ]);
    let run = bounded(&p, &edb("a", &["a", "b"]), 1 << 16);
    assert!(matches!(run.stop, Bounded::Fixed(_)));
    assert!(has(&run, "m(S, 0, 1)"));
    assert!(undefined(&run, "L", 0));
    assert!(undefined(&run, "L", 1));
    assert_eq!(run.findings, vec![], "Ford: functional");
}

#[test]
fn right_recursion_s_a_s_or_a_on_aaa() {
    // S <- 'a' S / 'a'   on "aaa"
    let p = grammar(vec![
        terminal("ta", "a"),
        seq("aS", "ta", "S"),
        choice("Sbody", "aS", "ta"),
        nonterminal("S", "Sbody"),
    ]);
    let run = bounded(&p, &edb("aaa", &["a"]), 1 << 16);
    assert!(has(&run, "m(S, 0, 3)"));
    assert!(
        !has(&run, "m(S, 0, 1)"),
        "ordered choice: the first alternative wins"
    );
    assert!(has(&run, "f(S, 3)"));
    assert_eq!(run.findings, vec![]);
}

#[test]
fn left_recursion_a_a_a_or_a_on_aa_is_undefined_not_a_hang() {
    // A <- A 'a' / 'a'   on "aa"
    let p = grammar(vec![
        terminal("ta", "a"),
        seq("Aa", "A", "ta"),
        choice("Abody", "Aa", "ta"),
        nonterminal("A", "Abody"),
    ]);
    let run = bounded(&p, &edb("aa", &["a"]), 1 << 16);
    assert!(matches!(run.stop, Bounded::Fixed(_)));
    assert!(undefined(&run, "A", 0));
    assert_eq!(run.findings, vec![]);
}

// ── A deliberately non-functional clause set: a finding, not a panic ─────

/// Longest-match-and-shortest-match together: two `j`s for one `(e, i)`.
fn non_functional_rules() -> Vec<PositiveRule> {
    vec![
        rule("one", "m(x, ?i, ?j) :- lit(a, ?i, ?j)"),
        rule("two", "m(x, ?i, ?k) :- lit(a, ?i, ?j), lit(a, ?j, ?k)"),
        rule(
            "three",
            "m(x, ?i, ?l) :- lit(a, ?i, ?j), lit(a, ?j, ?k), lit(a, ?k, ?l)",
        ),
    ]
}

fn non_functional() -> (PositiveProgram, Facts) {
    let p = PositiveProgram::new(non_functional_rules(), vec![m_functional()]).unwrap();
    (p, facts(&["lit(a, 0, 1)", "lit(a, 1, 2)", "lit(a, 2, 3)"]))
}

fn violation(key: &[Const], least: u64, other: u64) -> FunctionalViolation {
    FunctionalViolation::new(pred("m"), key.to_vec(), nat(least), nat(other)).unwrap()
}

#[test]
fn non_functional_clauses_are_reported_never_a_panic() {
    let (p, f) = non_functional();
    let run = bounded(&p, &f, 1 << 16);
    let x = sym("x");
    // (x, 0) → {1, 2, 3}; (x, 1) → {2, 3}; (x, 2) → {3}. Every extra value
    // is paired with the least one, in key then value order.
    assert_eq!(
        run.findings,
        vec![
            violation(&[x.clone(), nat(0)], 1, 2),
            violation(&[x.clone(), nat(0)], 1, 3),
            violation(&[x.clone(), nat(1)], 2, 3),
        ]
    );
    // Not first-wins: every value stays in M.
    for s in [
        "m(x, 0, 1)",
        "m(x, 0, 2)",
        "m(x, 0, 3)",
        "m(x, 1, 2)",
        "m(x, 1, 3)",
        "m(x, 2, 3)",
    ] {
        assert!(has(&run, s), "{s} missing: the finding must not drop atoms");
    }
}

#[test]
fn findings_do_not_depend_on_order() {
    let (p, f) = non_functional();
    let base = bounded(&p, &f, 1 << 16);
    let mut rng = Lcg(0x666f_7264);
    for _ in 0..32 {
        let p2 =
            PositiveProgram::new(shuffled(p.rules(), &mut rng), p.functional().to_vec()).unwrap();
        let f2 = Facts {
            altitude: f.altitude.clone(),
            atoms: shuffled(&f.atoms, &mut rng),
        };
        assert_eq!(bounded(&p2, &f2, 1 << 16).findings, base.findings);
    }
}

#[test]
fn a_declaration_never_changes_m_or_the_ticks() {
    // The index serves the join; it must not change what is derived.
    let (p, f) = non_functional();
    let with = bounded(&p, &f, 1 << 16);
    let without = bounded(&without_decls(&p), &f, 1 << 16);
    assert_eq!(ticks(&with), ticks(&without));
    assert_eq!(without.findings, vec![]);

    let g = grammar(vec![
        terminal("ta", "a"),
        seq("aS", "ta", "S"),
        choice("Sbody", "aS", "ta"),
        nonterminal("S", "Sbody"),
    ]);
    let input = edb("aaaa", &["a"]);
    let with = bounded(&g, &input, 1 << 16);
    let without = bounded(&without_decls(&g), &input, 1 << 16);
    assert_eq!(ticks(&with), ticks(&without));
}

#[test]
fn a_declaration_naming_missing_positions_is_dark() {
    // §4.2: the first offending position in declaration order, or the
    // value position when it sits inside the key. Was `Functional { pred }`.
    for (bad, position) in [
        (
            Functional {
                pred: pred("m"),
                key: vec![pos(1), pos(2)],
                value: pos(4),
            },
            4,
        ),
        (
            Functional {
                pred: pred("m"),
                key: vec![pos(5), pos(2)],
                value: pos(4),
            },
            5,
        ),
        (
            Functional {
                pred: pred("m"),
                key: vec![pos(1), pos(3)],
                value: pos(3),
            },
            3,
        ),
    ] {
        assert_eq!(
            PositiveProgram::new(non_functional_rules(), vec![bad]),
            Err(Dark::Functional {
                pred: pred("m"),
                position: pos(position)
            })
        );
    }
}

#[test]
fn position_zero_cannot_be_built() {
    // Was the `key: vec![0]` case above: a 0-based slip now can't be written.
    assert_eq!(Position::new(0), None);
    assert_eq!(Position::new(1).map(Position::get), Some(1));
}

#[test]
fn a_violation_record_needs_least_below_other() {
    assert!(FunctionalViolation::new(pred("m"), vec![], nat(2), nat(1)).is_none());
    assert!(FunctionalViolation::new(pred("m"), vec![], nat(1), nat(1)).is_none());
    let v = violation(&[], 1, 2);
    assert_eq!((v.least(), v.other()), (&nat(1), &nat(2)));
}

#[test]
fn a_declaration_on_a_fact_only_predicate_is_checked_against_the_facts() {
    let p = PositiveProgram::new(
        vec![],
        vec![Functional {
            pred: pred("m"),
            key: vec![pos(1)],
            value: pos(4),
        }],
    )
    .unwrap();
    assert_eq!(
        Instance::new(&p, facts(&["m(x, 0, 1)"])).err(),
        Some(Dark::Functional {
            pred: pred("m"),
            position: pos(4)
        })
    );
}

#[test]
fn findings_are_a_set_of_distinct_records() {
    let (p, f) = non_functional();
    let run = bounded(&p, &f, 1 << 16);
    let set: BTreeSet<_> = run.findings.iter().collect();
    assert_eq!(set.len(), run.findings.len());
}
