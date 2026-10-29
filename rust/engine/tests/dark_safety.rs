//! Safety (spec §4.1 "Program", §4.2; audit §3.4, §4 item 8). An unsafe
//! rule or an arity mismatch makes the program `dark` (not run), naming the
//! culprit. Never a panic. Since §4.2 these are found at construction:
//! `PositiveProgram::new` / `NormalProgram::new` for the rules,
//! `Instance::new` for facts against the program's signature.
//!
//! Negation in the positive core is no longer a runtime verdict: a
//! `PositiveRule` body is `Vec<Atom>`, so it can't be written. That is a
//! `compile_fail` doctest on `PositiveRule` in `src/lib.rs` (it replaces
//! `negation_is_dark_in_the_core`). Function terms are unrepresentable too.

mod common;

use common::{facts, pred, program, rule_id, try_program, var};
use engine::{Arity, Dark, Instance};

#[test]
fn unsafe_head_variable_is_dark_naming_it() {
    assert_eq!(
        try_program(&[("bad", "p(?x, ?y) :- e(?x)")]),
        Err(Dark::Unsafe { rule: rule_id("bad"), var: var("y") })
    );
}

#[test]
fn unsafe_empty_body_with_variable_is_dark() {
    assert_eq!(try_program(&[("bad", "p(?x)")]), Err(Dark::Unsafe { rule: rule_id("bad"), var: var("x") }));
}

#[test]
fn arity_mismatch_is_dark() {
    // Facts against the program's signature: `expected` is the program's
    // arity (e/2), `found` the fact's (e/1). Was `expected 1, found 2`, the
    // least two arities, an artefact of checking rules and facts together.
    let p = program(&[("r", "p(?x) :- e(?x, ?y)")]);
    assert_eq!(
        Instance::new(&p, facts(&["e(a)"])).err(),
        Some(Dark::Arity { pred: pred("e"), expected: Arity(2), found: Arity(1) })
    );
}

#[test]
fn arity_mismatch_inside_the_rules_is_dark_at_construction() {
    assert_eq!(
        try_program(&[("r", "p(?x) :- e(?x), e(?x, ?y)")]),
        Err(Dark::Arity { pred: pred("e"), expected: Arity(1), found: Arity(2) })
    );
}

#[test]
fn dark_verdict_does_not_depend_on_rule_order() {
    let a = try_program(&[("ok", "p(?x) :- e(?x)"), ("bad1", "q(?y) :- e(?x)"), ("bad2", "s(?z) :- e(?x)")]);
    let b = try_program(&[("bad2", "s(?z) :- e(?x)"), ("ok", "p(?x) :- e(?x)"), ("bad1", "q(?y) :- e(?x)")]);
    assert_eq!(a, b);
    assert!(a.is_err());
}
