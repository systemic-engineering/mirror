//! T-rename (audit §4 item 4; the E_troj lesson at core altitude). A
//! consistent bijective renaming of predicate, constant, variable and rule-id
//! symbols commutes with evaluation. A core whose behaviour depends on a
//! symbol's name (E_troj fires on a rule named `Admin`) fails it. Run with a
//! renaming that maps some symbol *to* `Admin`, and one that maps `Admin` away.

mod common;

use common::{facts, fixpoint, pred, program, reprogram, rule_id, ticks, var};
use engine::{Atom, Const, Fact, Facts, PositiveProgram, PositiveRule, Sym, Term};
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
            Const::Sym(s) => Const::Sym(Sym::new(&self.name(s.as_str())).unwrap()),
            other => other.clone(),
        }
    }
    fn atom(&self, a: &Atom) -> Atom {
        Atom {
            pred: pred(&self.name(a.pred.as_str())),
            args: a
                .args
                .iter()
                .map(|t| match t {
                    Term::Var(v) => Term::Var(var(&self.name(v.as_str()))),
                    Term::Const(c) => Term::Const(self.c(c)),
                })
                .collect(),
        }
    }
    fn fact(&self, f: &Fact) -> Fact {
        Fact {
            pred: pred(&self.name(f.pred.as_str())),
            args: f.args.iter().map(|c| self.c(c)).collect(),
        }
    }
    fn program(&self, p: &PositiveProgram) -> PositiveProgram {
        reprogram(
            p,
            p.rules()
                .iter()
                .map(|r| PositiveRule {
                    id: rule_id(&self.name(r.id.as_str())),
                    head: self.atom(&r.head),
                    body: r.body.iter().map(|a| self.atom(a)).collect(),
                })
                .collect(),
        )
    }
    fn facts(&self, f: &Facts) -> Facts {
        Facts {
            altitude: f.altitude.clone(),
            atoms: f.atoms.iter().map(|a| self.fact(a)).collect(),
        }
    }
}

fn commutes(p: &PositiveProgram, f: &Facts, rn: &Renaming) {
    let before = fixpoint(p, f);
    let after = fixpoint(&rn.program(p), &rn.facts(f));
    let renamed: BTreeMap<Fact, u64> = ticks(&before)
        .iter()
        .map(|(k, t)| (rn.fact(k), *t))
        .collect();
    assert_eq!(ticks(&after), renamed);
    assert_eq!(after.stop, before.stop);
}

fn admin_program() -> (PositiveProgram, Facts) {
    (
        program(&[
            ("Admin", "Admin(?x) :- user(?x), role(?x, root)"),
            ("grant", "grant(?x, ?y) :- Admin(?x), user(?y)"),
            ("reach", "reach(?x, ?z) :- grant(?x, ?y), grant(?y, ?z)"),
        ]),
        facts(&[
            "user(alice)",
            "user(root)",
            "user(Admin)",
            "role(alice, root)",
            "role(Admin, root)",
        ]),
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
    commutes(
        &p,
        &f,
        &Renaming::new(&[("path", "Admin"), ("b", "root"), ("y", "guard")]),
    );
    commutes(
        &p,
        &f,
        &Renaming::new(&[("path-step", "Admin"), ("edge", "user")]),
    );
    commutes(&p, &f, &Renaming::new(&[("b", "Admin"), ("x", "admin")]));
}

#[test]
fn t_rename_mapping_admin_away_commutes() {
    let (p, f) = admin_program();
    commutes(&p, &f, &Renaming::new(&[("Admin", "staff")]));
    commutes(
        &p,
        &f,
        &Renaming::new(&[("Admin", "a0"), ("root", "Admin")]),
    );
    commutes(
        &p,
        &f,
        &Renaming::new(&[("Admin", "root"), ("root", "Admin"), ("alice", "x")]),
    );
}
