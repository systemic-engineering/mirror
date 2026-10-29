//! The atom-address formula, pinned (spec §4.1 "Fact store", §5.0; audit
//! §3.6: once §4.1 pins content bytes, address tests may pin them).
//!
//! `addr = H("splinter\0" ‖ τ ‖ enc(content))`,
//! `content = enc(p) ‖ u64(n) ‖ arg₁ ‖ … ‖ arg_n`, `arg = tag ‖ enc(bytes)`,
//! tags sym 0x00 / nat 0x01 / addr 0x02, nat bytes = u64 big-endian,
//! `τ = UTF-8(altitude) ‖ 0x00 ‖ 0x00` (root-grammar flag).
//! `M`'s address: `H("fragment\0" ‖ τ ‖ enc("model") ‖ u64(|M|) ‖ addr…)`,
//! children sorted. Status splinter: an atom at `@pact/status` with content
//! `enc("status") ‖ u64(3) ‖ (addr, atom) ‖ (sym, "undefined") ‖ (sym, "false")`.
//!
//! Every expected address here is assembled byte by byte in this file and
//! hashed with `blake3` directly (the crate's own dependency, linked into
//! the test; no dev-dependency added), never by calling `engine`'s encoder.
//! One hex literal is pinned as well, so a change in `blake3` itself shows.
//!
//! Also the off-by-one bound (§4.1 A3): `T ≤ |B| − |F| + 1`; `a.` stops
//! fixed at 2.

mod common;

use common::{alt, bounded, fact, facts, fixpoint, nat, normal, pred, program, sym, wf, TEST_ALTITUDE};
use engine::{Addr, Bounded, Budget, Const, Fact, Fixed, Instance, Tick};

fn u64be(n: u64) -> Vec<u8> {
    n.to_be_bytes().to_vec()
}

fn enc(b: &[u8]) -> Vec<u8> {
    let mut v = u64be(b.len() as u64);
    v.extend_from_slice(b);
    v
}

fn tau(alt: &str) -> Vec<u8> {
    let mut v = alt.as_bytes().to_vec();
    v.extend_from_slice(&[0x00, 0x00]);
    v
}

enum Arg<'a> {
    Sym(&'a str),
    Nat(u64),
    Addr([u8; 32]),
}

fn content(p: &str, args: &[Arg]) -> Vec<u8> {
    let mut c = enc(p.as_bytes());
    c.extend(u64be(args.len() as u64));
    for a in args {
        match a {
            Arg::Sym(s) => {
                c.push(0x00);
                c.extend(enc(s.as_bytes()));
            }
            Arg::Nat(n) => {
                c.push(0x01);
                c.extend(enc(&u64be(*n)));
            }
            Arg::Addr(x) => {
                c.push(0x02);
                c.extend(enc(x));
            }
        }
    }
    c
}

fn splinter(alt: &str, p: &str, args: &[Arg]) -> [u8; 32] {
    let mut b = b"splinter\0".to_vec();
    b.extend(tau(alt));
    b.extend(enc(&content(p, args)));
    *blake3::hash(&b).as_bytes()
}

fn hex(a: &[u8; 32]) -> String {
    a.iter().map(|b| format!("{b:02x}")).collect()
}

#[test]
fn atom_address_is_the_outer_enc_formula() {
    let f = Fact { pred: pred("m"), args: vec![sym("S"), nat(0), nat(3)] };
    let want = splinter("@parse/item", "m", &[Arg::Sym("S"), Arg::Nat(0), Arg::Nat(3)]);
    assert_eq!(engine::addr(&alt("@parse/item"), &f).as_bytes(), &want);

    let a = [7u8; 32];
    let g = Fact { pred: pred("send"), args: vec![Const::Addr(Addr::from_hex(&"07".repeat(32)).unwrap())] };
    assert_eq!(engine::addr(&alt("@pact/fact"), &g).as_bytes(), &splinter("@pact/fact", "send", &[Arg::Addr(a)]));

    let z = Fact { pred: pred("q"), args: vec![] };
    assert_eq!(engine::addr(&alt(TEST_ALTITUDE), &z).as_bytes(), &splinter(TEST_ALTITUDE, "q", &[]));
}

#[test]
fn one_hex_literal_pins_the_formula_and_blake3() {
    let want = splinter("@datalog/fact", "edge", &[Arg::Sym("a"), Arg::Sym("b")]);
    assert_eq!(hex(&want), PINNED_EDGE_A_B);
    assert_eq!(engine::addr(&alt("@datalog/fact"), &fact("edge(a, b)")).as_bytes(), &want);
    // The checked parse round-trips the pinned hex.
    assert_eq!(Addr::from_hex(PINNED_EDGE_A_B).unwrap().as_bytes(), &want);
}

#[test]
fn addr_parse_and_altitude_are_checked() {
    assert!(Addr::from_hex("00").is_none());
    assert!(Addr::from_hex(&"zz".repeat(32)).is_none());
    for bad in ["", "datalog/fact", "@", "@a//b", "@a/", "@a\0b"] {
        assert!(engine::Altitude::new(bad).is_none(), "{bad:?}");
    }
    assert!(engine::Altitude::new("@pact/status").is_some());
    assert!(engine::Sym::new("a\0").is_none());
    assert!(Budget::new(0).is_none());
}

/// `edge(a, b)` at `@datalog/fact`. A third assembly: the 72 message bytes
/// were built in Python (`struct.pack('>Q', …)`) and hashed with `b3sum`
/// 1.8.7, sharing no code with this file or with `engine`.
const PINNED_EDGE_A_B: &str = "946bfa436b2cee0a30b991372b4a812f91586a8de81da2278487e0bd2610cde8";

#[test]
fn model_address_is_the_sorted_model_fragment() {
    let run = fixpoint(&program(&[("p", "p(?x) :- e(?x)")]), &facts(&["e(a)", "e(b)"]));
    let mut kids: Vec<[u8; 32]> = ["e", "p"]
        .iter()
        .flat_map(|p| ["a", "b"].map(|c| splinter(TEST_ALTITUDE, p, &[Arg::Sym(c)])))
        .collect();
    kids.sort();
    let mut b = b"fragment\0".to_vec();
    b.extend(tau(TEST_ALTITUDE));
    b.extend(enc(b"model"));
    b.extend(u64be(kids.len() as u64));
    for k in &kids {
        b.extend_from_slice(k);
    }
    assert_eq!(run.model_addr.as_bytes(), blake3::hash(&b).as_bytes());
}

#[test]
fn a_retraction_is_deposited_as_a_status_splinter() {
    let p = normal(&[("b", "b :- not a"), ("c", "c :- not b"), ("d", "d :- not c")]);
    let f = engine::Facts { altitude: alt("@pact/fact"), atoms: vec![fact("a")] };
    let w = wf(&p, &f);
    let d = splinter("@pact/fact", "d", &[]);
    let status = splinter("@pact/status", "status", &[Arg::Addr(d), Arg::Sym("undefined"), Arg::Sym("false")]);
    assert_eq!(w.retractions.len(), 1);
    assert_eq!(w.retractions[0].addr.as_bytes(), &d);
    assert_eq!(w.retractions[0].status_addr.as_bytes(), &status);
}

// ── The bound: T ≤ |B| − |F| + 1 ─────────────────────────────────────────

#[test]
fn a_rule_with_no_body_and_no_facts_stops_fixed_at_2() {
    // |B| = 1, F = ∅: the old `k_max ≥ |B|` was one short.
    let p = program(&[("a", "a")]);
    assert_eq!(fixpoint(&p, &facts(&[])).stop, Fixed(Tick(2)));
    assert_eq!(bounded(&p, &facts(&[]), 1).stop, Bounded::Budget(Budget::new(1).unwrap()));
    assert_eq!(bounded(&p, &facts(&[]), 2).stop, Bounded::Fixed(Tick(2)));
    assert_eq!(Instance::new(&p, facts(&[])).unwrap().sufficient_budget().get(), 2);
}

#[test]
fn the_chain_meets_the_bound_with_the_sufficient_budget() {
    // e(0,1)…e(n−1,n), closure: |B| = 2·(n+1)², |F| = n, T = n + 1.
    for n in 1..=8u64 {
        let atoms: Vec<String> = (0..n).map(|i| format!("e({i}, {})", i + 1)).collect();
        let f = facts(&atoms.iter().map(String::as_str).collect::<Vec<_>>());
        let p = program(&[("b", "p(?x, ?y) :- e(?x, ?y)"), ("s", "p(?x, ?z) :- p(?x, ?y), e(?y, ?z)")]);
        let b = 2 * (n + 1) * (n + 1);
        let sufficient = b - n + 1;
        assert_eq!(Instance::new(&p, f.clone()).unwrap().sufficient_budget().get(), sufficient, "n = {n}");
        let run = bounded(&p, &f, sufficient);
        let Bounded::Fixed(Tick(t)) = run.stop else { panic!("n = {n}: budget at the sufficient bound") };
        assert!(t <= sufficient, "n = {n}");
    }
}
