//! The ruler envelope (docs/audits/2026-10-29-seam-pact-eval-and-ruler-envelope.md
//! §2.2, §2.3; docs/specs/mirror-ruler.md §1.1).
//!
//! The address bytes are assembled in this file from spec §1.1 and hashed
//! with `blake3` directly (the crate's own dependency, linked into the
//! test), never by calling the ruler's encoder. The digest literal comes
//! from a third assembly: the 41 preimage bytes built in Python and hashed
//! with `b3sum` 1.8.7.

use engine::{Bounded, Facts, Instance, PositiveProgram, Tick};
use ruler::{Approval, Count, ReaderDependence, ReaderRefused, Ruler, Truncated, Unit, UnitMismatch};
use std::num::NonZeroU64;

fn count(n: u64, unit: Unit) -> Count {
    Count { n: NonZeroU64::new(n).unwrap(), unit }
}

fn ruler(n: u64, unit: Unit) -> Ruler<Count> {
    Ruler::new(count(n, unit), ReaderDependence::Independent).expect("Count admits Independent")
}

/// Spec §1.1's preimage for `Count { n, unit }`, Independent.
fn preimage(n: u64, unit_tag: u8) -> Vec<u8> {
    let mut body = vec![0x01, 0x01, 0x00]; // family Budget, body Count, Independent
    body.extend_from_slice(&n.to_be_bytes());
    body.push(unit_tag);
    let mut p = b"ruler\0".to_vec();
    p.extend_from_slice(b"@mirror/ruler");
    p.extend_from_slice(&[0x00, 0x00]);
    p.extend_from_slice(&(body.len() as u64).to_be_bytes());
    p.extend_from_slice(&body);
    p
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

// ── The address (spec §1.1) ──────────────────────────────────────────────

#[test]
fn the_worked_example_preimage_is_the_spec_bytes() {
    let p = preimage(100, 0x01);
    assert_eq!(p.len(), 6 + 15 + 8 + 12);
    assert_eq!(hex(&p[29..]), "010100000000000000006401", "the body bytes of spec §1.1");
}

#[test]
fn count_100_ticks_has_the_pinned_digest() {
    let want = blake3::hash(&preimage(100, 0x01));
    assert_eq!(ruler(100, Unit::Ticks).addr().as_bytes(), want.as_bytes());
    assert_eq!(hex(want.as_bytes()), "e7b519a4ec908835dbaa132760da8871a5f4651dd9d7a819cedb58e1740d4b5e");
}

#[test]
fn the_address_is_a_function_of_the_body() {
    assert_eq!(ruler(7, Unit::Ticks).addr(), ruler(7, Unit::Ticks).addr());
    assert_ne!(ruler(7, Unit::Ticks).addr(), ruler(8, Unit::Ticks).addr());
    assert_ne!(ruler(7, Unit::Ticks).addr(), ruler(7, Unit::Rounds).addr());
    assert_eq!(ruler(7, Unit::Rounds).addr().as_bytes(), blake3::hash(&preimage(7, 0x02)).as_bytes());
    assert_eq!(ruler(u64::MAX, Unit::Ticks).addr().as_bytes(), blake3::hash(&preimage(u64::MAX, 0x01)).as_bytes());
}

#[test]
fn count_refuses_a_reader_dependent_declaration() {
    let reader = ruler(1, Unit::Ticks).addr();
    let dependent = ReaderDependence::Dependent(reader);
    assert_eq!(
        Ruler::new(count(100, Unit::Ticks), dependent),
        Err(ReaderRefused { reader: dependent })
    );
}

#[test]
fn the_ruler_keeps_its_body_and_reader() {
    let r = ruler(100, Unit::Ticks);
    assert_eq!(*r.body(), count(100, Unit::Ticks));
    assert_eq!(*r.reader(), ReaderDependence::Independent);
}

// ── run_bounded: the shaped wrapper (audit §2.3) ─────────────────────────

fn chain(n: u64) -> (PositiveProgram, Facts) {
    use engine::{Atom, Const, Fact, Predicate, PositiveRule, RuleId, Sym, Term, Var};
    let v = |s: &str| Term::Var(Var::new(s).unwrap());
    let atom = |p: &str, args: Vec<Term>| Atom { pred: Predicate::new(p).unwrap(), args };
    let rules = vec![
        PositiveRule { id: RuleId::new("base").unwrap(), head: atom("p", vec![v("x"), v("y")]), body: vec![atom("e", vec![v("x"), v("y")])] },
        PositiveRule {
            id: RuleId::new("step").unwrap(),
            head: atom("p", vec![v("x"), v("z")]),
            body: vec![atom("p", vec![v("x"), v("y")]), atom("e", vec![v("y"), v("z")])],
        },
    ];
    let sym = |i: u64| Const::Sym(Sym::new(&format!("c{i}")).unwrap());
    let atoms = (0..n).map(|i| Fact { pred: Predicate::new("e").unwrap(), args: vec![sym(i), sym(i + 1)] }).collect();
    (PositiveProgram::new(rules, vec![]).unwrap(), Facts { altitude: engine::Altitude::new("@datalog/fact").unwrap(), atoms })
}

#[test]
fn a_budget_that_cuts_the_run_records_truncated_at_its_tick() {
    // A chain of 5 needs round 6 to see the empty delta.
    let (p, f) = chain(5);
    let inst = Instance::new(&p, f).unwrap();
    let budget = ruler(5, Unit::Ticks);
    let shaped = ruler::run_bounded(&inst, &budget).expect("Ticks");
    assert!(matches!(shaped.value().stop, Bounded::Budget(_)));
    assert_eq!(shaped.erasure(), Some(&Truncated { at: Tick(5) }));
    assert_eq!(shaped.shaped_by().len(), 1);
    assert_eq!(*shaped.shaped_by().first(), budget.addr());
    assert!(shaped.observed_by().is_empty());
    assert_eq!(shaped.approval(), Approval::Unsigned);
}

#[test]
fn a_budget_that_suffices_erases_nothing() {
    let (p, f) = chain(5);
    let inst = Instance::new(&p, f).unwrap();
    let budget = ruler(6, Unit::Ticks);
    let shaped = ruler::run_bounded(&inst, &budget).expect("Ticks");
    assert_eq!(shaped.value().stop, Bounded::Fixed(Tick(6)));
    assert_eq!(shaped.erasure(), None);
    assert_eq!(*shaped.shaped_by().first(), budget.addr());
    assert_eq!(shaped.approval(), Approval::Unsigned);
}

#[test]
fn the_shaped_run_is_the_engines_bounded_run() {
    let (p, f) = chain(5);
    let inst = Instance::new(&p, f).unwrap();
    for k in 1..=8 {
        let shaped = ruler::run_bounded(&inst, &ruler(k, Unit::Ticks)).unwrap();
        let bare = engine::run_bounded(&inst, engine::Budget::new(k).unwrap());
        assert_eq!(shaped.into_value(), bare, "k = {k}");
    }
}

#[test]
fn a_count_in_rounds_is_refused_for_a_core_run() {
    let (p, f) = chain(2);
    let inst = Instance::new(&p, f).unwrap();
    assert_eq!(
        ruler::run_bounded(&inst, &ruler(10, Unit::Rounds)).err(),
        Some(UnitMismatch { expected: Unit::Ticks, found: Unit::Rounds })
    );
}
