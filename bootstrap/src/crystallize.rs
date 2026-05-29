//! Crystallize — the substrate-execution dispatcher harness.
//!
//! Tick A of `docs/specs/kintsugi-minimum-runnable.md`. This module IS the
//! floor primitive that binds substrate action declarations (the parked `\`
//! bodies of `@kintsugi/fracture/*` and `@cli.*` actions) to Rust
//! implementations. The capability stays in the substrate; the dispatcher
//! carries only the binding. Per AGENTS.md §"Boundary Rust is not frozen
//! capability" — `[substrate-pull:realize]`.
//!
//! RED commit: the types compile; the bodies are stubbed (`todo!()`) so
//! the tests below COMPILE-then-PANIC. The green commit replaces each
//! `todo!()` with the Merkle / dispatch realization.

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use prism_core::{Optic, ScalarLoss};
use terni::Imperfect;

// ---------------------------------------------------------------------------
// Newtypes — no-bare-types discipline (feedback-no-bare-types).
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Oid([u8; 32]);

impl Oid {
    pub fn bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Text(String);

impl Text {
    pub fn new(_s: impl Into<String>) -> Self {
        todo!("Text::new — green commit lands the body")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FieldName(String);

impl FieldName {
    pub fn new(_name: impl Into<String>) -> Result<Self, &'static str> {
        todo!("FieldName::new — green commit lands the body")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ActionPath(String);

impl ActionPath {
    pub fn new(_path: impl Into<String>) -> Result<Self, &'static str> {
        todo!("ActionPath::new — green commit lands the body")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ---------------------------------------------------------------------------
// Splinter — content-addressed self-similar value.
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Content {
    Text(Text),
    Record(BTreeMap<FieldName, Splinter>),
    List(Vec<Splinter>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Splinter {
    content: Content,
    oid: Oid,
}

impl Splinter {
    pub fn new(_content: Content) -> Self {
        todo!("Splinter::new — green commit lands the Merkle OID computation")
    }

    pub fn content(&self) -> &Content {
        &self.content
    }

    pub fn oid(&self) -> &Oid {
        &self.oid
    }

    pub fn verify(&self) -> bool {
        todo!("Splinter::verify — green commit lands the recompute-and-compare")
    }
}

// ---------------------------------------------------------------------------
// Body, Crystallization, Registry, CrystallizeError.
// ---------------------------------------------------------------------------

pub type Body = Arc<
    dyn Fn(Optic<(), Splinter>) -> Imperfect<Splinter, CrystallizeError, ScalarLoss>
        + Send
        + Sync,
>;

pub struct Crystallization {
    pub path: ActionPath,
    pub body: Body,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CrystallizeError {
    Uncrystallized(ActionPath),
    Boundary(String),
    Mismatch {
        expected: &'static str,
        got: &'static str,
    },
}

pub struct Registry {
    table: HashMap<ActionPath, Body>,
}

impl Registry {
    pub fn new() -> Self {
        todo!("Registry::new — green commit lands the empty-table init")
    }

    pub fn register(&mut self, _c: Crystallization) {
        todo!("Registry::register — green commit lands the table insert")
    }

    pub fn knows(&self, _path: &ActionPath) -> bool {
        todo!("Registry::knows — green commit lands the lookup")
    }

    pub fn crystallize(
        &self,
        _path: &ActionPath,
        _input: Optic<(), Splinter>,
    ) -> Imperfect<Splinter, CrystallizeError, ScalarLoss> {
        todo!("Registry::crystallize — green commit lands the dispatch")
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

pub fn floor_registry() -> Registry {
    Registry::new()
}

// ---------------------------------------------------------------------------
// Tests — Tick A red-first set.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use prism_core::Beam;

    #[test]
    fn action_path_no_bare_string() {
        let p = ActionPath::new("@kintsugi/fracture/rename").unwrap();
        assert_eq!(p.as_str(), "@kintsugi/fracture/rename");
        assert!(ActionPath::new("").is_err());
        assert!(ActionPath::new("kintsugi/fracture").is_err());
        assert!(ActionPath::new("@kintsugi fracture").is_err());
    }

    #[test]
    fn splinter_oid_roundtrip() {
        let s = Splinter::new(Content::Text(Text::new("hello")));
        assert!(s.verify(), "freshly constructed Splinter must verify");
    }

    #[test]
    fn splinter_oid_deterministic() {
        let a = Splinter::new(Content::Text(Text::new("hello")));
        let b = Splinter::new(Content::Text(Text::new("hello")));
        assert_eq!(a.oid(), b.oid());
    }

    #[test]
    fn splinter_text_different_content_different_oid() {
        let a = Splinter::new(Content::Text(Text::new("hello")));
        let b = Splinter::new(Content::Text(Text::new("world")));
        assert_ne!(a.oid(), b.oid());
    }

    #[test]
    fn splinter_record_merkle() {
        let k = FieldName::new("name").unwrap();
        let mut m1 = BTreeMap::new();
        m1.insert(
            k.clone(),
            Splinter::new(Content::Text(Text::new("alex"))),
        );
        let r1 = Splinter::new(Content::Record(m1));

        let mut m2 = BTreeMap::new();
        m2.insert(k, Splinter::new(Content::Text(Text::new("reed"))));
        let r2 = Splinter::new(Content::Record(m2));

        assert_ne!(r1.oid(), r2.oid());
        assert!(r1.verify());
        assert!(r2.verify());
    }

    #[test]
    fn splinter_record_key_change_changes_oid() {
        let mut m1 = BTreeMap::new();
        m1.insert(
            FieldName::new("a").unwrap(),
            Splinter::new(Content::Text(Text::new("x"))),
        );
        let r1 = Splinter::new(Content::Record(m1));

        let mut m2 = BTreeMap::new();
        m2.insert(
            FieldName::new("b").unwrap(),
            Splinter::new(Content::Text(Text::new("x"))),
        );
        let r2 = Splinter::new(Content::Record(m2));

        assert_ne!(r1.oid(), r2.oid());
    }

    #[test]
    fn splinter_list_merkle() {
        let l1 = Splinter::new(Content::List(vec![
            Splinter::new(Content::Text(Text::new("a"))),
            Splinter::new(Content::Text(Text::new("b"))),
        ]));
        let l2 = Splinter::new(Content::List(vec![
            Splinter::new(Content::Text(Text::new("b"))),
            Splinter::new(Content::Text(Text::new("a"))),
        ]));
        assert_ne!(l1.oid(), l2.oid());
        assert!(l1.verify());
    }

    #[test]
    fn registry_empty_knows_nothing() {
        let r = Registry::new();
        let p = ActionPath::new("@kintsugi/fracture/rename").unwrap();
        assert!(!r.knows(&p));
    }

    #[test]
    fn registry_empty_returns_uncrystallized() {
        let r = Registry::new();
        let p = ActionPath::new("@kintsugi/fracture/rename").unwrap();
        let input = Optic::ok((), Splinter::new(Content::Text(Text::new("seed"))));
        let verdict = r.crystallize(&p, input);
        match verdict {
            Imperfect::Failure(CrystallizeError::Uncrystallized(got), _) => {
                assert_eq!(got, p);
            }
            other => panic!("expected Uncrystallized failure, got {:?}", other),
        }
    }

    fn echo_body() -> Body {
        Arc::new(|input: Optic<(), Splinter>| {
            let splinter = input
                .result()
                .ok()
                .cloned()
                .expect("echo body: input must carry a value");
            Imperfect::success(splinter)
        })
    }

    #[test]
    fn registry_register_and_crystallize() {
        let mut r = Registry::new();
        let p = ActionPath::new("@test/echo").unwrap();
        r.register(Crystallization {
            path: p.clone(),
            body: echo_body(),
        });
        assert!(r.knows(&p));

        let seed_splinter = Splinter::new(Content::Text(Text::new("hi")));
        let expected_oid = seed_splinter.oid().clone();
        let input = Optic::ok((), seed_splinter);
        let verdict = r.crystallize(&p, input);
        match verdict {
            Imperfect::Success(out) => {
                assert_eq!(out.oid(), &expected_oid);
                assert!(out.verify());
            }
            other => panic!("expected Success, got {:?}", other),
        }
    }

    #[test]
    fn registry_unregistered_after_register() {
        let mut r = Registry::new();
        let known = ActionPath::new("@test/echo").unwrap();
        let unknown = ActionPath::new("@test/unknown").unwrap();
        r.register(Crystallization {
            path: known.clone(),
            body: echo_body(),
        });
        assert!(r.knows(&known));
        assert!(!r.knows(&unknown));

        let input = Optic::ok((), Splinter::new(Content::Text(Text::new("hi"))));
        let verdict = r.crystallize(&unknown, input);
        match verdict {
            Imperfect::Failure(CrystallizeError::Uncrystallized(got), _) => {
                assert_eq!(got, unknown);
            }
            other => panic!("expected Uncrystallized, got {:?}", other),
        }
    }

    #[test]
    fn floor_registry_is_empty_in_tick_a() {
        let r = floor_registry();
        let kintsugi = ActionPath::new("@kintsugi/fracture/rename").unwrap();
        let cli = ActionPath::new("@cli/new").unwrap();
        assert!(!r.knows(&kintsugi));
        assert!(!r.knows(&cli));
    }
}
