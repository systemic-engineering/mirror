// Tick A's surface (Splinter constructors, FieldName::new, Body, the
// Crystallization tuple, the Boundary/Mismatch error variants, the
// Merkle helpers) is exercised only by the in-module tests until
// Ticks B/C/F populate the floor's first Crystallizations. `dead_code`
// would fire on every export otherwise. Once the first concrete
// Crystallization lands (Tick B for @kintsugi/fracture/rename or
// Tick F for @cli/new), this attribute drops.
#![allow(dead_code)]
//! Crystallize — the substrate-execution dispatcher harness.
//!
//! Tick A of `docs/specs/kintsugi-minimum-runnable.md`. This module IS the
//! floor primitive that binds substrate action declarations (the parked `\`
//! bodies of `@kintsugi/fracture/*` and `@cli.*` actions) to Rust
//! implementations. The capability stays in the substrate; the dispatcher
//! carries only the binding. Per AGENTS.md §"Boundary Rust is not frozen
//! capability" — `[substrate-pull:realize]`.
//!
//! ## What lives here
//!
//! - [`Splinter`] — content-addressed, OID-proving, self-similar value.
//!   The currency the dispatcher passes across the substrate boundary.
//!   Merkle-style OID (each level hashes from its children's OIDs, not
//!   the recursive content).
//! - [`Body`] — a parked substrate action body realized as a Rust closure.
//!   Prism-shaped: takes a seed Beam carrying a Splinter, returns an
//!   [`Imperfect`] verdict (Success / Partial / Failure).
//! - [`Crystallization`] — a (path, body) pair, the realisation of one
//!   substrate action.
//! - [`Registry`] — the static map from [`ActionPath`] to [`Body`]. The
//!   only place "this substrate ref means this Rust function" lives.
//! - [`CrystallizeError`] — three shapes: [`Uncrystallized`] (the floor
//!   doesn't realise this substrate claim yet), [`Boundary`] (an `@io`
//!   call failed), [`Mismatch`] (substrate passed an unexpected shape).
//!
//! ## What does NOT live here
//!
//! - Concrete [`Crystallization`]s for `@kintsugi/fracture/rename` or
//!   `@cli/new` — those are Ticks B/C/F.
//! - Edge / graph structure on [`Splinter`] — that's @spectral/db's
//!   domain, building on this content-addressed foundation.
//! - CLI dispatch — `main.rs` is unchanged.
//!
//! ## Adaptation note — Body's exact shape
//!
//! The spec says `Body: Fn(Beam<Splinter>) -> Imperfect<Beam<Splinter>,
//! CrystallizeError, ScalarLoss>`. In `prism_core` [`Beam`] is a *trait*,
//! not a type constructor; the concrete seed-beam shape used everywhere
//! (`apply_h`, `seed(...)`) is `Optic<(), S>`. We use:
//!
//! ```ignore
//! Fn(Optic<(), Splinter>) -> Imperfect<Splinter, CrystallizeError, ScalarLoss>
//! ```
//!
//! — the input is the seed-beam-carrying-Splinter, the output is the
//! verdict carrying the next Splinter. This matches `apply_h`'s shape
//! exactly and is the only honest concrete instantiation of "Beam<T>"
//! given how the trait surfaces in prism-core.
//!
//! ## Adaptation note — Splinter OID source
//!
//! The spec assumes plain `Sha256` for Splinter's Merkle OID computation
//! (a standard content hash, distinct from the bootstrap's existing
//! `canonical_hash` / `CoincidenceHash<5,5>` in `hash.rs`). The
//! coincidence hash is the *spectral-triple Dirac action* on AST nodes
//! — its inputs are eigenvalue-projected vectors in a 5-d basis, which
//! is the wrong altitude for self-similar content addressing. Splinter
//! gets its own Merkle SHA-256 — minimal floor, no overlap.

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use prism_core::{Optic, ScalarLoss};
use sha2::{Digest, Sha256};
use terni::Imperfect;

// ---------------------------------------------------------------------------
// Newtypes — no-bare-types discipline (feedback-no-bare-types).
// ---------------------------------------------------------------------------

/// 32-byte content address. Distinct from `prism_core::Oid` (which is a
/// 64-char hex string carrying CoincidenceHash<3>): this `Oid` is the
/// raw SHA-256 digest of a Merkle node, the bootstrap's own
/// content-addressing for [`Splinter`]. Splinter is the dispatcher's
/// currency; the coincidence hash is the spectral-triple Dirac action.
/// Different altitudes; different machinery.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Oid([u8; 32]);

impl Oid {
    /// Raw bytes view, for inclusion in a parent Merkle hash.
    pub fn bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// A literal text leaf in a [`Splinter`]. Newtype to keep bare `String`
/// out of the floor surface.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Text(String);

impl Text {
    pub fn new(s: impl Into<String>) -> Self {
        Text(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A record-field name in a [`Splinter`]. Newtype with whitespace
/// rejected at construction.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FieldName(String);

impl FieldName {
    /// Construct. Returns `Err` if `name` is empty or contains whitespace.
    pub fn new(name: impl Into<String>) -> Result<Self, &'static str> {
        let s = name.into();
        if s.is_empty() {
            return Err("FieldName must be non-empty");
        }
        if s.chars().any(|c| c.is_whitespace()) {
            return Err("FieldName must not contain whitespace");
        }
        Ok(FieldName(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A substrate action path, like `"@kintsugi/fracture/rename"`. Newtype
/// with `@`-prefix and non-empty validation at construction.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ActionPath(String);

impl ActionPath {
    /// Construct. Returns `Err` if `path` is empty, lacks a `@` prefix,
    /// or contains whitespace.
    pub fn new(path: impl Into<String>) -> Result<Self, &'static str> {
        let s = path.into();
        if s.is_empty() {
            return Err("ActionPath must be non-empty");
        }
        if !s.starts_with('@') {
            return Err("ActionPath must start with '@'");
        }
        if s.chars().any(|c| c.is_whitespace()) {
            return Err("ActionPath must not contain whitespace");
        }
        Ok(ActionPath(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ---------------------------------------------------------------------------
// Splinter — content-addressed self-similar value.
// ---------------------------------------------------------------------------

/// The shape of a [`Splinter`]'s payload. Three forms — leaf text, named
/// record, ordered list — sufficient to carry substrate actions' inputs
/// and outputs without inventing more.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Content {
    Text(Text),
    Record(BTreeMap<FieldName, Splinter>),
    List(Vec<Splinter>),
}

/// Content-addressed, OID-proving, self-similar.
///
/// The OID is **Merkle-style**: each level computes from its children's
/// OIDs, not the full recursive content. Three shapes:
///
/// - `Text(t)` — `sha256(b"T" || u64_le(t.len()) || t)`
/// - `Record(m)` — `sha256(b"R" || u64_le(m.len())
///                       || for each (key, sub) in sorted key order:
///                            u64_le(key.len()) || key || sub.oid().bytes())`
/// - `List(items)` — `sha256(b"L" || u64_le(items.len())
///                       || for each sub: sub.oid().bytes())`
///
/// [`Splinter::new`] computes the OID at construction.
/// [`Splinter::verify`] recomputes from content and compares — the
/// symmetric N−1 integrity check.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Splinter {
    content: Content,
    oid: Oid,
}

impl Splinter {
    /// Construct, computing the Merkle OID.
    pub fn new(content: Content) -> Self {
        let oid = compute_oid(&content);
        Splinter { content, oid }
    }

    pub fn content(&self) -> &Content {
        &self.content
    }

    pub fn oid(&self) -> &Oid {
        &self.oid
    }

    /// Recompute the OID from content and compare to the stored one.
    /// True iff they agree. The floor's symmetric N−1 verification.
    pub fn verify(&self) -> bool {
        compute_oid(&self.content) == self.oid
    }
}

fn compute_oid(content: &Content) -> Oid {
    let mut h = Sha256::new();
    match content {
        Content::Text(t) => {
            h.update(b"T");
            let bytes = t.as_str().as_bytes();
            h.update(u64_le(bytes.len() as u64));
            h.update(bytes);
        }
        Content::Record(map) => {
            h.update(b"R");
            h.update(u64_le(map.len() as u64));
            // BTreeMap iteration is sorted-by-key; sort order is part of
            // the OID definition.
            for (key, sub) in map.iter() {
                let kbytes = key.as_str().as_bytes();
                h.update(u64_le(kbytes.len() as u64));
                h.update(kbytes);
                h.update(sub.oid().bytes());
            }
        }
        Content::List(items) => {
            h.update(b"L");
            h.update(u64_le(items.len() as u64));
            for sub in items {
                h.update(sub.oid().bytes());
            }
        }
    }
    let digest = h.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    Oid(out)
}

fn u64_le(v: u64) -> [u8; 8] {
    // Matches `bootstrap/src/hash.rs::u64_le` byte-for-byte (index-by-shift
    // is the canonical form the hash module uses; preserving that idiom
    // keeps the two modules' little-endian encoding obviously identical).
    #[allow(clippy::needless_range_loop)]
    {
        let mut out = [0u8; 8];
        for k in 0..8 {
            out[k] = ((v >> (k * 8)) & 0xff) as u8;
        }
        out
    }
}

// ---------------------------------------------------------------------------
// Body, Crystallization, Registry, CrystallizeError.
// ---------------------------------------------------------------------------

/// A realised substrate action body. Prism-shaped: takes a seed beam
/// (input state in [`Optic<(), Splinter>`]), returns the verdict
/// ([`Imperfect`]) carrying the next [`Splinter`].
///
/// See the module-level "Adaptation note — Body's exact shape" for why
/// this is the literal type and not the spec's nominal `Beam<Splinter>
/// -> Imperfect<Beam<Splinter>, ...>`.
pub type Body = Arc<
    dyn Fn(Optic<(), Splinter>) -> Imperfect<Splinter, CrystallizeError, ScalarLoss>
        + Send
        + Sync,
>;

/// (path, body) — one substrate action realized.
pub struct Crystallization {
    pub path: ActionPath,
    pub body: Body,
}

/// Errors the dispatcher can surface. Three shapes — no more.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CrystallizeError {
    /// The floor has no [`Body`] for this path. The substrate claim is
    /// not yet realized at this altitude. Returned by
    /// [`Registry::crystallize`] for unknown paths.
    Uncrystallized(ActionPath),
    /// An `@io` call failed at the boundary. The payload is a free-form
    /// string today (the existing bootstrap `@io` errors are
    /// `io::Error`-derived strings); when the substrate names its own
    /// boundary-error type, this variant can take that type instead.
    Boundary(String),
    /// The substrate passed a [`Splinter`] shape the [`Body`] cannot
    /// accept (e.g. a Text where a Record was expected). Carries
    /// static names for diagnostic clarity, not values, to keep the
    /// error small and `Clone`able.
    Mismatch {
        expected: &'static str,
        got: &'static str,
    },
}

/// The dispatcher. Path → Body, with [`Registry::crystallize`] as the
/// dispatch primitive. Empty by default; ticks B/F populate it.
pub struct Registry {
    table: HashMap<ActionPath, Body>,
}

impl Registry {
    /// Empty registry. Used at startup by [`floor_registry`].
    pub fn new() -> Self {
        Registry {
            table: HashMap::new(),
        }
    }

    /// Register one [`Crystallization`]. The dispatcher learns one
    /// substrate-action → Rust-body binding.
    pub fn register(&mut self, c: Crystallization) {
        self.table.insert(c.path, c.body);
    }

    /// True iff a [`Body`] is registered for `path`.
    pub fn knows(&self, path: &ActionPath) -> bool {
        self.table.contains_key(path)
    }

    /// Dispatch: look up the [`Body`] for `path` and invoke it on
    /// `input`. Returns [`Imperfect::Failure`] with
    /// [`CrystallizeError::Uncrystallized`] when `path` has no body —
    /// the substrate claim is not yet realised at this altitude.
    pub fn crystallize(
        &self,
        path: &ActionPath,
        input: Optic<(), Splinter>,
    ) -> Imperfect<Splinter, CrystallizeError, ScalarLoss> {
        match self.table.get(path) {
            Some(body) => body(input),
            None => Imperfect::failure(CrystallizeError::Uncrystallized(path.clone())),
        }
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

/// The bootstrap's startup registry. Tick A leaves it empty; Ticks B
/// and F populate it as their crystallizations land.
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

    // --- ActionPath validation ---

    #[test]
    fn action_path_no_bare_string() {
        // Valid.
        let p = ActionPath::new("@kintsugi/fracture/rename").unwrap();
        assert_eq!(p.as_str(), "@kintsugi/fracture/rename");
        // Empty rejected.
        assert!(ActionPath::new("").is_err());
        // Missing '@' prefix rejected.
        assert!(ActionPath::new("kintsugi/fracture").is_err());
        // Whitespace rejected.
        assert!(ActionPath::new("@kintsugi fracture").is_err());
    }

    // --- Splinter OID — Text leaf ---

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

    // --- Splinter OID — Record Merkle ---

    #[test]
    fn splinter_record_merkle() {
        // A Record's OID changes if any sub-Splinter changes.
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

        assert_ne!(
            r1.oid(),
            r2.oid(),
            "Record OID must change when a sub-Splinter changes"
        );
        assert!(r1.verify());
        assert!(r2.verify());
    }

    #[test]
    fn splinter_record_key_change_changes_oid() {
        // Same value under a different field name → different OID.
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

    // --- Splinter OID — List ---

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
        assert_ne!(l1.oid(), l2.oid(), "List OID respects order");
        assert!(l1.verify());
    }

    // --- Registry — empty state ---

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

    // --- Registry — register and dispatch ---

    /// Echo body: return the input's value Splinter unchanged as Success.
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
        // Register one path; a DIFFERENT path still returns Uncrystallized.
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

    // --- floor_registry — empty in Tick A ---

    #[test]
    fn floor_registry_is_empty_in_tick_a() {
        let r = floor_registry();
        let kintsugi = ActionPath::new("@kintsugi/fracture/rename").unwrap();
        let cli = ActionPath::new("@cli/new").unwrap();
        assert!(!r.knows(&kintsugi), "Tick A: no kintsugi crystallization");
        assert!(!r.knows(&cli), "Tick A: no @cli crystallization");
    }
}
