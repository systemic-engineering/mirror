//! T-rename (audit §4 item 4; the E_troj lesson at core altitude). A
//! consistent bijective renaming of predicate, constant, variable and rule-id
//! symbols commutes with evaluation. A core whose behaviour depends on a
//! symbol's name (E_troj fires on a rule named `Admin`) fails it. Run with a
//! renaming that maps some symbol *to* `Admin`, and one that maps `Admin` away.

mod common;

use common::{facts, program, ticks};
use engine::{Atom, Const, Fact, Facts, Literal, Program, Rule, Term};
use std::collections::BTreeMap;

struct Renaming(BTreeMap<String, String>);

impl Renaming {
    fn new(pairs: &[(&str, &str)]) -> Self {
        let mut m = BTreeMap::new();
        for (a, b) in pairs {
            m.insert(a.to_string(), b.to_string());
        }
        // Bijective: no two sources share a target, and every target that is
        // also a source is itself mapped (a permutation on the touched names).
        let targets: std::collections::BTreeSet<_> = m.values().collect();
        assert_eq!(targets.len(), m.len(), "renaming must be injective");
        Renaming(m)
    }
    fn name(&self, s: &str) -> String {
        self.0.get(s).cloned().unwrap_or_else(|| s.to_string())
    }
    fn c(&self, c: &Const) -> Const {
        match c {
            Const::Sym(s) => Const::Sym(self.name(s)),
            other => other.clone(),
        }
    }
    fn atom(&self, a: &Atom) -> Atom {
        Atom {
            pred: self.name(&a.pred),
            args: a
                .args
                .iter()
                .map(|t| match t {
                    Term::Var(v) => Term::Var(self.name(v)),
                    Term::Const(c) => Term::Const(self.c(c)),
                })
                .collect(),
        }
    }
    fn fact(&self, f: &Fact) -> Fact {
        Fact { pred: self.name(&f.pred), args: f.args.iter().map(|c| self.c(c)).collect() }
    }
    fn program(&self, p: &Program) -> Program {
        Program {
            rules: p
                .rules
                .iter()
                .map(|r| Rule {
                    id: self.name(&r.id),
                    head: self.atom(&r.head),
                    body: r
                        .body
                        .iter()
                        .map(|l| match l {
                            Literal::Pos(a) => Literal::Pos(self.atom(a)),
                            Literal::Not(a) => Literal::Not(self.atom(a)),
                        })
                        .collect(),
                })
                .collect(),
            functional: p.functional.clone(),
        }
    }
    fn facts(&self, f: &Facts) -> Facts {
        Facts { altitude: f.altitude.clone(), atoms: f.atoms.iter().map(|a| self.fact(a)).collect() }
    }
}

fn commutes(p: &Program, f: &Facts, rn: &Renaming) {
    let before = engine::run(p, f, 1 << 16).unwrap();
    let after = engine::run(&rn.program(p), &rn.facts(f), 1 << 16).unwrap();
    let renamed: BTreeMap<Fact, u64> = ticks(&before).iter().map(|(k, t)| (rn.fact(k), *t)).collect();
    assert_eq!(ticks(&after), renamed);
    assert_eq!(after.stop, before.stop);
}

fn admin_program() -> (Program, Facts) {
    (
        program(&[
            ("Admin", "Admin(?x) :- user(?x), role(?x, root)"),
            ("grant", "grant(?x, ?y) :- Admin(?x), user(?y)"),
            ("reach", "reach(?x, ?z) :- grant(?x, ?y), grant(?y, ?z)"),
        ]),
        facts(&["user(alice)", "user(root)", "user(Admin)", "role(alice, root)", "role(Admin, root)"]),
    )
}

#[test]
fn t_rename_mapping_a_symbol_to_admin_commutes() {
    let p = program(&[
        ("path-base", "path(?x, ?y) :- edge(?x, ?y)"),
        ("path-step", "path(?x, ?z) :- path(?x, ?y), edge(?y, ?z)"),
    ]);
    let f = facts(&["edge(a, b)", "edge(b, c)", "edge(c, b)", "edge(c, d)"]);
    // Predicate path → Admin, constant b → Admin is not allowed alongside
    // (one target per source), so the constant goes to `root`; variable
    // ?y → Admin; rule path-step → Admin.
    commutes(&p, &f, &Renaming::new(&[("path", "Admin"), ("b", "root"), ("y", "guard")]));
    commutes(&p, &f, &Renaming::new(&[("path-step", "Admin"), ("edge", "user")]));
    commutes(&p, &f, &Renaming::new(&[("b", "Admin"), ("x", "admin")]));
}

#[test]
fn t_rename_mapping_admin_away_commutes() {
    let (p, f) = admin_program();
    commutes(&p, &f, &Renaming::new(&[("Admin", "staff")]));
    commutes(&p, &f, &Renaming::new(&[("Admin", "a0"), ("root", "Admin")]));
    commutes(&p, &f, &Renaming::new(&[("Admin", "root"), ("root", "Admin"), ("alice", "x")]));
}
