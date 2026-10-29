//! E3 (spec §4.1; math Prop. 1.7; audit §4 items 3 and 5) and E6 (order
//! never matters). The tick is not in the address: one atom derived at
//! tick 1 in one run and at tick 3 in another has one address; re-deriving
//! an atom deposits nothing new. Every permutation of `P` and `F` gives the
//! same `M` address, the same deposit ticks, and a byte-identical addressed
//! sorted model.
//!
//! No test here pins address hex (audit §3.2): addresses are compared with
//! each other, never with a literal.

mod common;

use common::{
    alt, fact, facts, fixpoint, nat, pred, program, reprogram, shuffled, sym, ticks, Lcg,
    TEST_ALTITUDE,
};
use engine::{Addr, Const, Fact, Facts, Fixed, PositiveProgram, Tick};
use std::collections::BTreeSet;

#[test]
fn e3_one_atom_at_tick_1_and_at_tick_3_has_one_address() {
    let f = facts(&["e(a, b)"]);
    let short = program(&[("p", "p(?x, ?y) :- e(?x, ?y)")]);
    let long = program(&[
        ("q", "q(?x, ?y) :- e(?x, ?y)"),
        ("r", "r(?x, ?y) :- q(?x, ?y)"),
        ("p", "p(?x, ?y) :- r(?x, ?y)"),
    ]);
    let pab = fact("p(a, b)");
    let a = fixpoint(&short, &f);
    let b = fixpoint(&long, &f);
    assert_eq!(ticks(&a)[&pab], 1);
    assert_eq!(ticks(&b)[&pab], 3);
    let addr_a = a.model().iter().find(|(_, x)| **x == pab).unwrap().0;
    let addr_b = b.model().iter().find(|(_, x)| **x == pab).unwrap().0;
    assert_eq!(addr_a, addr_b);
    assert_eq!(*addr_a, engine::addr(&alt(TEST_ALTITUDE), &pab));
}

#[test]
fn e3_rederivation_deposits_nothing_new() {
    // p(a, b) falls at tick 1, and is derived again at tick 3 by the long way.
    let p = program(&[
        ("p-short", "p(?x, ?y) :- e(?x, ?y)"),
        ("q", "q(?x, ?y) :- e(?x, ?y)"),
        ("s", "s(?x, ?y) :- q(?x, ?y)"),
        ("p-long", "p(?x, ?y) :- s(?x, ?y)"),
    ]);
    let run = fixpoint(&p, &facts(&["e(a, b)"]));
    let pab = engine::addr(&alt(TEST_ALTITUDE), &fact("p(a, b)"));
    let deps: Vec<_> = run.deposits().iter().filter(|d| d.addr == pab).collect();
    assert_eq!(deps.len(), 1);
    assert_eq!(deps[0].tick, Tick(1));
    assert_eq!(run.stop(), Fixed(Tick(3)));
    let distinct: BTreeSet<_> = run.deposits().iter().map(|d| d.addr).collect();
    assert_eq!(distinct.len(), run.deposits().len(), "one deposit per atom");
}

#[test]
fn e3_addr_is_injective_on_distinct_atoms() {
    let ones = Addr::from_hex(&"01".repeat(32)).unwrap();
    let atoms = vec![
        Fact {
            pred: pred("p"),
            args: vec![sym("ab")],
        },
        Fact {
            pred: pred("p"),
            args: vec![sym("a"), sym("b")],
        },
        Fact {
            pred: pred("pa"),
            args: vec![sym("b")],
        },
        Fact {
            pred: pred("p"),
            args: vec![sym("1")],
        },
        Fact {
            pred: pred("p"),
            args: vec![nat(1)],
        },
        Fact {
            pred: pred("p"),
            args: vec![Const::Addr(ones)],
        },
        Fact {
            pred: pred("p"),
            args: vec![sym("")],
        },
        Fact {
            pred: pred("p"),
            args: vec![],
        },
        Fact {
            pred: pred("p"),
            args: vec![nat(0)],
        },
        Fact {
            pred: pred("p"),
            args: vec![nat(0), nat(0)],
        },
    ];
    let mut seen = BTreeSet::new();
    for tau in ["@datalog/fact", "@pact/fact", "@parse/item"] {
        for a in &atoms {
            assert!(
                seen.insert(engine::addr(&alt(tau), a)),
                "collision at {tau}: {a:?}"
            );
        }
    }
    // Equality under re-derivation: the function is a function.
    assert_eq!(
        engine::addr(&alt("@pact/fact"), &atoms[0]),
        engine::addr(&alt("@pact/fact"), &atoms[0])
    );
}

/// Fixtures with recursion, mutual recursion and repeated variables.
fn fixtures() -> Vec<(PositiveProgram, Facts)> {
    vec![
        (
            program(&[
                ("path-base", "path(?x, ?y) :- edge(?x, ?y)"),
                ("path-step", "path(?x, ?z) :- path(?x, ?y), edge(?y, ?z)"),
            ]),
            facts(&["edge(a, b)", "edge(b, c)", "edge(c, b)", "edge(c, d)"]),
        ),
        (
            program(&[
                ("even-0", "even(0)"),
                ("odd", "odd(?y) :- even(?x), next(?x, ?y)"),
                ("even", "even(?y) :- odd(?x), next(?x, ?y)"),
                ("self", "loop(?x) :- edge(?x, ?x)"),
                ("sym", "sym(?x, ?y) :- edge(?x, ?y), edge(?y, ?x)"),
            ]),
            facts(&[
                "next(0, 1)",
                "next(1, 2)",
                "next(2, 3)",
                "next(3, 4)",
                "edge(c, c)",
                "edge(a, b)",
                "edge(b, a)",
            ]),
        ),
    ]
}

/// The byte record E6 compares: model address, then every (addr, fact
/// rendering) in address order, then every (tick, addr) deposit.
fn record(run: &engine::Run<Fixed>) -> Vec<u8> {
    let mut out = run.model_addr().as_bytes().to_vec();
    for (a, f) in run.model() {
        out.extend_from_slice(a.as_bytes());
        out.extend_from_slice(format!("{f:?}").as_bytes());
    }
    for d in run.deposits() {
        out.extend_from_slice(&d.tick.0.to_be_bytes());
        out.extend_from_slice(d.addr.as_bytes());
    }
    out
}

#[test]
fn e3_e6_permutations_of_rules_and_facts_change_nothing() {
    let mut rng = Lcg(0x7065_726d_7574_6521);
    for (i, (p, f)) in fixtures().into_iter().enumerate() {
        let base = fixpoint(&p, &f);
        for k in 0..64 {
            let p2 = reprogram(&p, shuffled(p.rules(), &mut rng));
            let f2 = Facts {
                altitude: f.altitude.clone(),
                atoms: shuffled(&f.atoms, &mut rng),
            };
            let r = fixpoint(&p2, &f2);
            assert_eq!(r.model_addr(), base.model_addr(), "fixture {i} permutation {k}");
            assert_eq!(ticks(&r), ticks(&base), "fixture {i} permutation {k}");
            assert_eq!(
                record(&r),
                record(&base),
                "fixture {i} permutation {k}: E6 bytes"
            );
        }
    }
}

#[test]
fn e3_duplicate_facts_are_one_atom() {
    let run = fixpoint(&program(&[]), &facts(&["e(a, b)", "e(a, b)"]));
    assert_eq!(run.model().len(), 1);
    assert_eq!(run.deposits().len(), 1);
}

#[test]
fn e3_model_address_ignores_ticks() {
    // Same M, reached at different ticks: same model address.
    let f = facts(&["e(a, b)"]);
    let a = fixpoint(&program(&[("p", "p(?x, ?y) :- e(?x, ?y)")]), &f);
    let b = fixpoint(
        &program(&[
            ("q", "q(?x, ?y) :- e(?x, ?y)"),
            ("p", "p(?x, ?y) :- q(?x, ?y)"),
            ("q2", "q(?x, ?y) :- p(?x, ?y)"),
        ]),
        &facts(&["e(a, b)", "q(a, b)"]),
    );
    // a: {e, p}; b: {e, q, p}. Different models, different addresses...
    assert_ne!(a.model_addr(), b.model_addr());
    // ...and the same model reached at tick 0 instead of tick 1 has one address.
    let c = fixpoint(&program(&[]), &facts(&["e(a, b)", "p(a, b)"]));
    assert_eq!(a.model_addr(), c.model_addr());
}
