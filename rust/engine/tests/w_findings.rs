//! Functionality findings under `W` (spec §4.1 "The well-founded wrapper",
//! review A5): computed on `true` only, i.e. definite violations. Undefined
//! atoms can't violate definitively, and the `γ` calls' intermediate
//! findings (on `over`, which is an over-estimate) are not reported.

mod common;

use common::{alt, fact, nat, normal_rule, pred, sym, wf};
use engine::{Facts, Functional, FunctionalViolation, NormalProgram, Position, Status};

fn m_key1_value2() -> Functional {
    Functional { pred: pred("m"), key: vec![Position::new(1).unwrap()], value: Position::new(2).unwrap() }
}

fn program(rules: &[(&str, &str)]) -> NormalProgram {
    NormalProgram::new(rules.iter().map(|(id, s)| normal_rule(id, s)).collect(), vec![m_key1_value2()]).unwrap()
}

fn pact(atoms: &[&str]) -> Facts {
    Facts { altitude: alt("@pact/fact"), atoms: atoms.iter().map(|s| fact(s)).collect() }
}

#[test]
fn a_definite_violation_on_true_is_reported() {
    let p = program(&[("one", "m(x, 1) :- a"), ("two", "m(x, 2) :- a")]);
    let w = wf(&p, &pact(&["a"]));
    assert_eq!(w.findings, vec![FunctionalViolation::new(pred("m"), vec![sym("x")], nat(1), nat(2)).unwrap()]);
}

#[test]
fn an_undefined_value_does_not_violate() {
    // m(x, 1) hangs on the p/q negation cycle, so it is undefined;
    // m(x, 2) is true. `over` holds both, `true` only one: no finding.
    let p = program(&[
        ("p", "p :- not q"),
        ("q", "q :- not p"),
        ("one", "m(x, 1) :- not q"),
        ("two", "m(x, 2) :- a"),
    ]);
    let w = wf(&p, &pact(&["a"]));
    assert_eq!(w.status(&fact("m(x, 1)")), Status::Undefined);
    assert_eq!(w.status(&fact("m(x, 2)")), Status::True);
    assert_eq!(w.findings, vec![]);
}

#[test]
fn a_retracted_value_does_not_violate() {
    // m(x, 1) is in over₀ (b is not yet known) and retracted at round 1
    // once b is under; m(x, 2) is true. Only `true` counts: no finding.
    let p = program(&[("b", "b :- a"), ("one", "m(x, 1) :- not b"), ("two", "m(x, 2) :- a")]);
    let w = wf(&p, &pact(&["a"]));
    assert_eq!(w.status(&fact("m(x, 1)")), Status::False);
    assert_eq!(w.findings, vec![]);
}
