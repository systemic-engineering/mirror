//! E5 (spec §4.1; audit §3.4, §4 items 1 and 6): pacts through the
//! well-founded wrapper `W`.
//!
//! The vectors are transcribed as Rust literals from commit `5babfbad`
//! (branch `reed/mirror-006-pact`; also in `1324aae8`), blobs:
//!   tests/pact/01-determinism.json          44243526c42ffcaa7f97d3359d26a7af400ae0cf
//!   tests/pact/03-false-with-provenance.json 3d9a97d091215dba2686b828b88ad1b486c1f69f
//!   tests/pact/04-negation-cycle.json        3c9df80a6777ea50bb41ad7062c219727407d467
//!   tests/pact/oracles.md                    9f96bfff0d493e05f0f855186bb506bade6066e4
//! Check with `git show <blob>`. The oracle is `oracles.md`: if this
//! transcription and the oracle disagree, the oracle is right.
//!
//! Per the oracle, `true` and `undefined` list every derived (intensional)
//! atom, so facts are subtracted before comparing. 01 and 04 are in full;
//! 03 is statuses only (its `because` tree is a why-not query, not floor:
//! audit §3.4).

mod common;

use common::{alt, fact, fixpoint, normal, program, rule_id, shuffled, try_normal, var, wf, Lcg};
use engine::{Dark, Facts, NormalProgram, Outcome, Round, Status, WellFounded};
use std::collections::BTreeSet;

const PACT: &str = "@pact/fact";

fn pact_facts(atoms: &[&str]) -> Facts {
    Facts {
        altitude: alt(PACT),
        atoms: atoms.iter().map(|s| fact(s)).collect(),
    }
}

/// The derived atoms of a status set: the set minus the input facts.
fn derived(
    set: &std::collections::BTreeMap<engine::Addr, engine::Fact>,
    f: &Facts,
) -> BTreeSet<engine::Fact> {
    let given: BTreeSet<_> = f.atoms.iter().cloned().collect();
    set.values()
        .filter(|a| !given.contains(a))
        .cloned()
        .collect()
}

fn set(atoms: &[&str]) -> BTreeSet<engine::Fact> {
    atoms.iter().map(|s| fact(s)).collect()
}

fn w(p: &NormalProgram, f: &Facts) -> WellFounded {
    wf(p, f)
}

// ── Vector 01: determinism ───────────────────────────────────────────────

fn v01() -> (NormalProgram, Facts) {
    (
        normal(&[
            ("path-base", "path(?x, ?y) :- edge(?x, ?y)"),
            ("path-step", "path(?x, ?z) :- path(?x, ?y), edge(?y, ?z)"),
        ]),
        pact_facts(&["edge(a, b)", "edge(b, c)", "edge(c, b)", "edge(c, d)"]),
    )
}

#[test]
fn e5_vector_01_nine_paths_true_clear() {
    let (p, f) = v01();
    let r = w(&p, &f);
    assert_eq!(r.outcome, Outcome::Clear);
    assert_eq!(
        derived(&r.true_atoms, &f),
        set(&[
            "path(a, b)",
            "path(a, c)",
            "path(a, d)",
            "path(b, b)",
            "path(b, c)",
            "path(b, d)",
            "path(c, b)",
            "path(c, c)",
            "path(c, d)",
        ])
    );
    assert!(r.undefined.is_empty());
    assert_eq!(r.retractions, vec![]);
}

// ── Vector 03: false (statuses only) ─────────────────────────────────────

#[test]
fn e5_vector_03_send_false_clear() {
    let p = normal(&[
        (
            "ready",
            "ready(?a) :- application(?a), answered(?a, alex, yes)",
        ),
        (
            "send",
            "send(?a) :- ready(?a), pdf_stands_alone(?a), dates_only(?a), not reader_tracking(?a)",
        ),
    ]);
    let f = pact_facts(&["application(a1)", "pdf_stands_alone(a1)", "dates_only(a1)"]);
    let r = w(&p, &f);
    assert_eq!(r.outcome, Outcome::Clear);
    assert_eq!(r.status(&fact("send(a1)")), Status::False);
    assert_eq!(r.status(&fact("ready(a1)")), Status::False);
    // `absent` in the oracle: extensional, not a fact, so false.
    assert_eq!(r.status(&fact("answered(a1, alex, yes)")), Status::False);
    assert!(derived(&r.true_atoms, &f).is_empty());
    assert!(r.undefined.is_empty());
    // The facts themselves are true.
    assert_eq!(r.status(&fact("application(a1)")), Status::True);
}

// ── Vector 04: negation cycle ────────────────────────────────────────────

fn v04() -> (NormalProgram, Facts) {
    (
        normal(&[
            ("p", "p :- not q"),
            ("q", "q :- not p"),
            ("r", "r :- not s"),
            ("t", "t :- p"),
        ]),
        pact_facts(&[]),
    )
}

#[test]
fn e5_vector_04_r_true_p_q_t_undefined_opaque() {
    // Also Prop. 1.9 / audit §4 item 6: γ alternates {p,q,r,t}, {r},
    // {p,q,r,t}, … A cumulative stop would read {p,q,r,t} as true; the
    // consecutive-equality stop on `under` gives true {r}.
    let (p, f) = v04();
    let r = w(&p, &f);
    assert_eq!(r.outcome, Outcome::Opaque);
    assert_eq!(derived(&r.true_atoms, &f), set(&["r"]));
    assert_eq!(derived(&r.undefined, &f), set(&["p", "q", "t"]));
    assert_eq!(r.status(&fact("s")), Status::False);
    assert_eq!(r.status(&fact("p")), Status::Undefined);
}

// ── E6: permute: true vectors are order-independent ──────────────────────

#[test]
fn e6_vectors_01_and_04_identical_under_permutation() {
    let mut rng = Lcg(0x7061_6374);
    for (name, (p, f)) in [("01", v01()), ("04", v04())] {
        let base = w(&p, &f);
        for k in 0..64 {
            let p2 =
                NormalProgram::new(shuffled(p.rules(), &mut rng), p.functional().to_vec()).unwrap();
            let f2 = Facts {
                altitude: f.altitude.clone(),
                atoms: shuffled(&f.atoms, &mut rng),
            };
            assert_eq!(w(&p2, &f2), base, "vector {name} permutation {k}");
        }
    }
}

// ── Prop. 5.5: retraction as a deposited status splinter ─────────────────

#[test]
fn prop_5_5_d_is_retracted_at_round_1_and_nothing_is_deleted() {
    // b :- not a.  c :- not b.  d :- not c.   fact a.
    // over₀ = {a, c, d}; under₁ = {a, c}; over₁ = {a, c}: d retracted.
    let p = normal(&[
        ("b", "b :- not a"),
        ("c", "c :- not b"),
        ("d", "d :- not c"),
    ]);
    let f = pact_facts(&["a"]);
    let r = w(&p, &f);
    assert_eq!(r.retractions.len(), 1, "{:?}", r.retractions);
    assert_eq!(r.retractions[0].fact, fact("d"));
    assert_eq!(r.retractions[0].round, Round(1));
    assert_eq!(r.retractions[0].addr, engine::addr(&alt(PACT), &fact("d")));
    assert_eq!(derived(&r.true_atoms, &f), set(&["c"]));
    assert_eq!(r.status(&fact("d")), Status::False);
    assert_eq!(r.status(&fact("b")), Status::False);
    assert_eq!(r.outcome, Outcome::Clear);
}

#[test]
fn a_longer_alternating_chain_settles_with_one_retraction_per_round() {
    // a. b :- not a. c :- not b. d :- not c. e :- not d. g :- not e.
    let p = normal(&[
        ("b", "b :- not a"),
        ("c", "c :- not b"),
        ("d", "d :- not c"),
        ("e", "e :- not d"),
        ("g", "g :- not e"),
    ]);
    let f = pact_facts(&["a"]);
    let r = w(&p, &f);
    assert_eq!(derived(&r.true_atoms, &f), set(&["c", "e"]));
    assert!(r.undefined.is_empty());
    let retracted: Vec<_> = r
        .retractions
        .iter()
        .map(|x| (x.fact.clone(), x.round))
        .collect();
    assert_eq!(
        retracted,
        vec![(fact("d"), Round(1)), (fact("g"), Round(2))]
    );
}

// ── Safety in W ──────────────────────────────────────────────────────────

#[test]
fn an_unsafe_negative_literal_is_dark_naming_the_variable() {
    // Now found when the program is built (§4.2), not when W runs.
    assert_eq!(
        try_normal(&[("bad", "p(?x) :- e(?x), not q(?y)")]),
        Err(Dark::Unsafe {
            rule: rule_id("bad"),
            var: var("y")
        })
    );
}

#[test]
fn a_positive_program_through_w_agrees_with_the_core() {
    let (p, f) = v01();
    let positive = program(&[
        ("path-base", "path(?x, ?y) :- edge(?x, ?y)"),
        ("path-step", "path(?x, ?z) :- path(?x, ?y), edge(?y, ?z)"),
    ]);
    let core = fixpoint(&positive, &f);
    assert_eq!(w(&p, &f).true_atoms, core.model);
}
