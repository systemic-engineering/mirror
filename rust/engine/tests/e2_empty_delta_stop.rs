//! E2 (spec §4.1): the empty-delta stop. Facts `e(0,1) … e(n−1,n)` with
//! `p(X,Y) :- e(X,Y). p(X,Z) :- p(X,Y), e(Y,Z).` give `stop fixed at n + 1`,
//! the last deposit at tick `n`, and `|M| = n + n(n+1)/2`. A program with
//! no rules gives `M = F` and `stop fixed at 1`.

mod common;

use common::{facts, program};
use engine::Stop;

fn chain(n: u64) -> engine::Facts {
    let atoms: Vec<String> = (0..n).map(|i| format!("e({i}, {})", i + 1)).collect();
    facts(&atoms.iter().map(String::as_str).collect::<Vec<_>>())
}

fn closure() -> engine::Program {
    program(&[("p-base", "p(?x, ?y) :- e(?x, ?y)"), ("p-step", "p(?x, ?z) :- p(?x, ?y), e(?y, ?z)")])
}

#[test]
fn e2_chain_stops_fixed_at_n_plus_1() {
    for n in 1..=12u64 {
        let run = engine::run(&closure(), &chain(n), 1 << 20).unwrap();
        assert_eq!(run.stop, Stop::Fixed(n + 1), "n = {n}");
        assert_eq!(run.deposits.iter().map(|d| d.tick).max(), Some(n), "n = {n}: last deposit tick");
        assert_eq!(run.model.len() as u64, n + n * (n + 1) / 2, "n = {n}: |M|");
    }
}

#[test]
fn e2_no_rules_gives_m_equals_f_fixed_at_1() {
    let f = facts(&["e(a, b)", "e(b, c)", "q"]);
    let run = engine::run(&program(&[]), &f, 1 << 20).unwrap();
    assert_eq!(run.stop, Stop::Fixed(1));
    let m: std::collections::BTreeSet<_> = run.model.values().cloned().collect();
    let want: std::collections::BTreeSet<_> = f.atoms.iter().cloned().collect();
    assert_eq!(m, want);
    assert!(run.deposits.iter().all(|d| d.tick == 0));
}

#[test]
fn e2_budget_is_reported_when_round_k_max_still_derives() {
    // n = 5 needs round 6 to see the empty delta.
    let run = engine::run(&closure(), &chain(5), 5).unwrap();
    assert_eq!(run.stop, Stop::Budget(5));
    let run = engine::run(&closure(), &chain(5), 6).unwrap();
    assert_eq!(run.stop, Stop::Fixed(6));
}
