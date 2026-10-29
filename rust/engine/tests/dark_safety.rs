//! Safety (spec §4.1 "Program"; audit §3.4, §4 item 8). An unsafe rule, an
//! arity mismatch, or a negative literal handed to the positive core makes
//! the program `dark` (not run), naming the culprit. Never a panic.
//! (Function terms are unrepresentable: `Term` has no third case.)

mod common;

use common::{facts, program};
use engine::Dark;

#[test]
fn unsafe_head_variable_is_dark_naming_it() {
    let p = program(&[("bad", "p(?x, ?y) :- e(?x)")]);
    assert_eq!(
        engine::run(&p, &facts(&["e(a)"]), 8),
        Err(Dark::Unsafe { rule: "bad".into(), var: "y".into() })
    );
}

#[test]
fn unsafe_empty_body_with_variable_is_dark() {
    let p = program(&[("bad", "p(?x)")]);
    assert_eq!(engine::run(&p, &facts(&[]), 8), Err(Dark::Unsafe { rule: "bad".into(), var: "x".into() }));
}

#[test]
fn arity_mismatch_is_dark() {
    let p = program(&[("r", "p(?x) :- e(?x, ?y)")]);
    assert_eq!(
        engine::run(&p, &facts(&["e(a)"]), 8),
        Err(Dark::Arity { pred: "e".into(), expected: 1, found: 2 })
    );
}

#[test]
fn negation_is_dark_in_the_core() {
    let p = program(&[("r", "p(?x) :- e(?x), not q(?x)")]);
    assert_eq!(engine::run(&p, &facts(&["e(a)"]), 8), Err(Dark::Negation { rule: "r".into() }));
}

#[test]
fn dark_verdict_does_not_depend_on_rule_order() {
    let a = program(&[("ok", "p(?x) :- e(?x)"), ("bad1", "q(?y) :- e(?x)"), ("bad2", "s(?z) :- e(?x)")]);
    let b = program(&[("bad2", "s(?z) :- e(?x)"), ("ok", "p(?x) :- e(?x)"), ("bad1", "q(?y) :- e(?x)")]);
    let f = facts(&["e(a)"]);
    assert_eq!(engine::run(&a, &f, 8), engine::run(&b, &f, 8));
    assert!(engine::run(&a, &f, 8).is_err());
}
