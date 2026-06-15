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
//! Tick A of `docs/specs/kintsugi-minimum-runnable.md`, amended by the
//! cascade landing in `docs/specs/store-vs-db-and-the-cascade.md`. This
//! module IS the floor primitive that binds substrate action declarations
//! (the parked `\` bodies of `@kintsugi/fracture/*` and `@cli.*` actions)
//! to Rust implementations. The capability stays in the substrate; the
//! dispatcher carries only the binding. Per AGENTS.md §"Boundary Rust is
//! not frozen capability" — `[substrate-pull:realize]`.
//!
//! ## What lives here
//!
//! - [`MerkleHash`] — the trait that abstracts the hash backend. The
//!   cascade pivots through this: anything content-addressed in the
//!   floor parameterises over `H: MerkleHash`. Per the landing-page
//!   spec §2.1.
//! - [`Blake3`] — the default backend. `@mirror/store` (the open
//!   content-addressed storage gate) is `Merkle<BLAKE3>` — standard,
//!   fast, Merkle-native, sidesteps Attack 1 from
//!   `spectral-hash-design.md` §3.1.
//! - [`Splinter<H>`] (default `H = Blake3`) — content-addressed,
//!   OID-proving, self-similar value. The currency the dispatcher
//!   passes across the substrate boundary. Merkle-style OID (each
//!   level hashes from its children's OIDs, not the recursive
//!   content). Default `H` ONLY on Splinter; the other generic types
//!   require explicit `H` so the type system prevents accidental
//!   single-world coupling (per landing-page spec §2.1).
//! - [`Body<H>`] — a parked substrate action body realized as a Rust
//!   closure. Prism-shaped: takes a seed Beam carrying a `Splinter<H>`,
//!   returns an [`Imperfect`] verdict (Success / Partial / Failure).
//! - [`Crystallization<H>`] — a (path, body) pair, the realisation of
//!   one substrate action.
//! - [`Crystallizations<H>`] — the table from [`Ref`] to [`Body<H>`].
//!   The only place "this substrate ref means this Rust function" lives.
//!   Renamed from `Registry` per the cascade — the plural of
//!   `Crystallization` names the discipline; `Registry` is generic
//!   language.
//! - [`Ref`] — substrate reference (`@`-prefixed nav-ref). Renamed
//!   from `ActionPath` per the cascade — matches mirror's nav-ref
//!   vocabulary (the `.`, `..`, `...`, `~`, `@`, `^`, `HEAD` set).
//!   Hash-blind; stays concrete.
//! - [`CrystallizeError`] — three shapes: [`Uncrystallized`] (the floor
//!   doesn't realise this substrate claim yet), [`Boundary`] (an `@io`
//!   call failed), [`Mismatch`] (substrate passed an unexpected shape).
//!   Hash-blind; stays concrete.
//! - [`kintsugi_tick`] — free function. Generic dispatcher consumer
//!   that routes a `Ref` through a `Crystallizations<H>` and returns
//!   the verdict.
//!
//! ## What does NOT live here
//!
//! - Concrete [`Crystallization`]s for `@kintsugi/fracture/rename` or
//!   `@cli/new` — those are Ticks B/C/F.
//! - Edge / graph structure on [`Splinter`] — that's `@spectral/db`'s
//!   domain, building on this content-addressed foundation. The
//!   `VoidPointer` reclaim (spectral coordinate, NOT a hash function)
//!   lives outside the `H` generic; see the landing-page spec §3.
//! - CLI dispatch — `main.rs` carries the integration; this module is
//!   the harness.
//!
//! ## Adaptation note — Body's exact shape
//!
//! The spec says `Body: Fn(Beam<Splinter>) -> Imperfect<Beam<Splinter>,
//! CrystallizeError, Transparency<Ref>>`. In `prismqueer` [`Beam`] is a
//! *trait*, not a type constructor; the concrete seed-beam shape used
//! everywhere (`apply_h`, `seed(...)`) is `Optic<(), S>`. We use:
//!
//! ```ignore
//! Fn(Optic<(), Splinter<H>>)
//!     -> Imperfect<Splinter<H>, CrystallizeError, Transparency<Ref>>
//! ```
//!
//! — the input is the seed-beam-carrying-Splinter, the output is the
//! verdict carrying the next Splinter. This matches `apply_h`'s shape
//! exactly and is the only honest concrete instantiation of "Beam<T>"
//! given how the trait surfaces in prism-core. The shape carries
//! through the cascade verbatim — `Splinter` becomes `Splinter<H>`,
//! and the loss carrier is [`Transparency<Ref>`] (structured opacities
//! located at `Ref`s) rather than a single scalar.
//!
//! [`Transparency<Ref>`]: prismqueer::Transparency
//!
//! ## Adaptation note — the hash backend
//!
//! The pre-cascade Tick A used plain `Sha256`. The cascade swaps the
//! concrete hash for a trait-bound parameter. Default `H = Blake3` —
//! BLAKE3 is Merkle-native by construction, fast, no float dependency,
//! and sidesteps Attack 1 from `spectral-hash-design.md` §3.1. Other
//! `H`-worlds (e.g. `MockHash` in tests, future engine-internal hash
//! choices) coexist in the same binary because the type parameter
//! prevents accidental mixing.
//!
//! The bootstrap's existing `canonical_hash` / `CoincidenceHash<5,5>`
//! in `hash.rs` is unaffected: that is the *spectral-triple Dirac
//! action* on AST nodes (eigenvalue-projected vectors in a 5-d basis),
//! the wrong altitude for self-similar content addressing. Splinter
//! retains its own Merkle backend.

use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::marker::PhantomData;
use std::sync::Arc;

use prismqueer::{Optic, Transparency};
use terni::Imperfect;

// `Ref` is re-exported from prism-core so existing call sites continue to
// read `crystallize::Ref` while the canonical definition lives one altitude
// down (substrate-shared with the bundled `Transparency<Ref>` loss carrier).
pub use prismqueer::Ref;

// ---------------------------------------------------------------------------
// MerkleHash — the trait the cascade pivots through.
// ---------------------------------------------------------------------------

/// Abstract hash backend for [`Splinter`]'s Merkle OID. Implementations
/// supply an opaque `Oid` type (fixed-width content address) and a
/// single byte-folding primitive.
///
/// The minimal surface is intentional: the cascade only needs
/// `hash_bytes` because [`Splinter`] composes the bytes-to-hash itself
/// (the Merkle encoding of `Content` — see [`compute_oid`]) and hashes
/// the assembled buffer in one shot. Streaming `update` was considered
/// and rejected as premature abstraction at this altitude; it can be
/// added without breaking changes (default-method on `MerkleHash`)
/// when a consumer demands it.
///
/// The `Oid` type carries an [`OidBytes`] supertrait bound so the
/// Merkle recursion in [`compute_oid`] can fold a child OID into its
/// parent's pre-hash byte buffer without per-call where-clauses.
/// `OidBytes` is also `pub` — any out-of-crate `MerkleHash` impl must
/// supply both — but consumers don't interact with it directly.
///
/// Per `docs/specs/store-vs-db-and-the-cascade.md` §2.1.
///
/// `Clone + Debug` are required on the backend itself (not just `Oid`)
/// so derived traits on the generic types tower (`Splinter<H>`,
/// `Content<H>`, `Crystallizations<H>`) compose without per-type
/// manual impls. Concrete backends are zero-sized marker structs;
/// satisfying both is `#[derive(Clone, Debug)]`.
pub trait MerkleHash: Clone + fmt::Debug {
    /// Fixed-width content address. Newtype to keep raw byte arrays
    /// out of the floor surface (`feedback-no-bare-types`).
    type Oid: Clone + fmt::Debug + Eq + std::hash::Hash + Ord + OidBytes;

    /// Hash a byte buffer to an [`Self::Oid`]. Deterministic.
    fn hash_bytes(bytes: &[u8]) -> Self::Oid;
}

// ---------------------------------------------------------------------------
// Blake3 — the default backend for @mirror/store.
// ---------------------------------------------------------------------------

/// 32-byte BLAKE3 content address.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Blake3Oid([u8; 32]);

impl Blake3Oid {
    /// Raw bytes view.
    pub fn bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// BLAKE3 backend. The default `H` for [`Splinter`]; the hash
/// `@mirror/store` uses for its open content-addressed storage gate.
/// Per the landing-page spec §1.1, §2.1. Zero-sized marker — the
/// `Clone + Debug` derives satisfy [`MerkleHash`]'s supertrait bounds
/// at no runtime cost.
#[derive(Clone, Debug)]
pub struct Blake3;

impl MerkleHash for Blake3 {
    type Oid = Blake3Oid;

    fn hash_bytes(bytes: &[u8]) -> Self::Oid {
        let digest = blake3::hash(bytes);
        Blake3Oid(*digest.as_bytes())
    }
}

// ---------------------------------------------------------------------------
// Hash-blind newtypes — no-bare-types discipline (feedback-no-bare-types).
// ---------------------------------------------------------------------------

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

// `Ref` (substrate reference, `@`-prefixed nav-ref) is no longer defined
// locally — it is re-exported from `prismqueer` at the top of the module
// (`pub use prismqueer::Ref`). Renamed from `ActionPath` in an earlier
// cascade; hoisted to prism in the Transparency cascade so the
// `Transparency<Ref>` loss carrier shares the same `Ref` substrate
// vocabulary across every prism consumer. Existing call sites that read
// `crystallize::Ref` continue to work because of the re-export.

// ---------------------------------------------------------------------------
// Splinter — content-addressed self-similar value, generic over H.
// ---------------------------------------------------------------------------

/// The shape of a [`Splinter`]'s payload. Three forms — leaf text, named
/// record, ordered list — sufficient to carry substrate actions' inputs
/// and outputs without inventing more.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Content<H: MerkleHash> {
    Text(Text),
    Record(BTreeMap<FieldName, Splinter<H>>),
    List(Vec<Splinter<H>>),
}

/// Content-addressed, OID-proving, self-similar.
///
/// The OID is **Merkle-style**: each level computes from its children's
/// OIDs, not the full recursive content. Three shapes, hash-tag-prefixed
/// for domain separation:
///
/// - `Text(t)` — `H::hash_bytes(b"T" || u64_le(t.len()) || t)`
/// - `Record(m)` — `H::hash_bytes(b"R" || u64_le(m.len())
///                       || for each (key, sub) in sorted key order:
///                            u64_le(key.len()) || key || sub.oid().bytes())`
/// - `List(items)` — `H::hash_bytes(b"L" || u64_le(items.len())
///                       || for each sub: sub.oid().bytes())`
///
/// The encoding stays Tick-A's; only the underlying hash function
/// changes with `H`. `H` defaults to [`Blake3`] — naive callers get the
/// `@mirror/store` hash without ceremony. Per the landing-page spec §2.1.
///
/// [`Splinter::new`] computes the OID at construction.
/// [`Splinter::verify`] recomputes from content and compares — the
/// symmetric N−1 integrity check.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Splinter<H: MerkleHash = Blake3> {
    content: Content<H>,
    oid: H::Oid,
}

impl<H: MerkleHash> Splinter<H> {
    /// Construct, computing the Merkle OID under `H`.
    pub fn new(content: Content<H>) -> Self {
        let oid = compute_oid::<H>(&content);
        Splinter { content, oid }
    }

    pub fn content(&self) -> &Content<H> {
        &self.content
    }

    pub fn oid(&self) -> &H::Oid {
        &self.oid
    }

    /// Recompute the OID from content and compare to the stored one.
    /// True iff they agree. The floor's symmetric N−1 verification.
    pub fn verify(&self) -> bool {
        compute_oid::<H>(&self.content) == self.oid
    }
}

/// Render `Content<H>` into the canonical Merkle bytes and hash them
/// with `H::hash_bytes`. Encoding rules are documented on [`Splinter`].
///
/// Children's OIDs must be byte-accessible to be included in the
/// parent hash. [`MerkleHash`]'s associated `Oid` type is opaque, so
/// we route the bytes through [`OidBytes`] — an internal blanket
/// trait every `Oid` implementation satisfies via [`BackendBytes`].
fn compute_oid<H: MerkleHash>(content: &Content<H>) -> H::Oid {
    let mut buf: Vec<u8> = Vec::new();
    match content {
        Content::Text(t) => {
            buf.push(b'T');
            let bytes = t.as_str().as_bytes();
            buf.extend_from_slice(&u64_le(bytes.len() as u64));
            buf.extend_from_slice(bytes);
        }
        Content::Record(map) => {
            buf.push(b'R');
            buf.extend_from_slice(&u64_le(map.len() as u64));
            // BTreeMap iteration is sorted-by-key; sort order is part
            // of the OID definition.
            for (key, sub) in map.iter() {
                let kbytes = key.as_str().as_bytes();
                buf.extend_from_slice(&u64_le(kbytes.len() as u64));
                buf.extend_from_slice(kbytes);
                buf.extend_from_slice(sub.oid().oid_bytes());
            }
        }
        Content::List(items) => {
            buf.push(b'L');
            buf.extend_from_slice(&u64_le(items.len() as u64));
            for sub in items {
                buf.extend_from_slice(sub.oid().oid_bytes());
            }
        }
    }
    H::hash_bytes(&buf)
}

/// Surfaces an `Oid`'s raw bytes for inclusion in a parent Merkle
/// hash. Carried as a [`MerkleHash::Oid`] supertrait bound so the
/// Merkle recursion in [`compute_oid`] can fold a child OID without
/// per-call where-clauses. Implemented for every concrete `Oid`
/// defined in this module; out-of-crate `MerkleHash` impls must
/// supply this too.
pub trait OidBytes {
    fn oid_bytes(&self) -> &[u8];
}

impl OidBytes for Blake3Oid {
    fn oid_bytes(&self) -> &[u8] {
        &self.0
    }
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
// Body, Crystallization, Crystallizations, CrystallizeError.
// ---------------------------------------------------------------------------

/// A realised substrate action body. Prism-shaped: takes a seed beam
/// (input state in [`Optic<(), Splinter<H>>`]), returns the verdict
/// ([`Imperfect`]) carrying the next [`Splinter<H>`].
///
/// See the module-level "Adaptation note — Body's exact shape" for why
/// this is the literal type and not the spec's nominal `Beam<Splinter>
/// -> Imperfect<Beam<Splinter>, ...>`.
///
/// No default on `H` — the binding is per-consumer; the registry's `H`
/// determines the world its bodies inhabit, and the type system
/// prevents cross-world mixing (landing-page spec §2.1, §2.4).
///
/// The loss carrier is [`Transparency<Ref>`] — structured opacities
/// located at substrate refs (`@kintsugi/fracture/validate`,
/// `@quantize`, `@positive`, …) rather than a single scalar. A `Body`
/// that succeeds returns `Imperfect::Success(...)` (the `Clear`
/// identity is the absence-of-loss); a `Body` that partially succeeds
/// returns `Imperfect::Partial(splinter, Transparency::Opaque({path →
/// verdict, ...}))`. Composition of bodies unions the opacity maps via
/// [`PropertyVerdict::merge_with`] at colliding paths.
///
/// [`PropertyVerdict::merge_with`]: prismqueer::PropertyVerdict::merge_with
pub type Body<H> = Arc<
    dyn Fn(Optic<(), Splinter<H>>) -> Imperfect<Splinter<H>, CrystallizeError, Transparency<Ref>>
        + Send
        + Sync,
>;

/// (path, body) — one substrate action realized.
///
/// Per the cascade, `Crystallization` is the singular event; the table
/// holding many such events is [`Crystallizations<H>`].
pub struct Crystallization<H: MerkleHash> {
    pub path: Ref,
    pub body: Body<H>,
}

/// Errors the dispatcher can surface. Three shapes — no more.
///
/// Hash-blind; stays concrete (no `H` parameter). Per landing-page
/// spec §2.2.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CrystallizeError {
    /// The floor has no [`Body`] for this path. The substrate claim is
    /// not yet realized at this altitude. Returned by
    /// [`Crystallizations::crystallize`] for unknown paths.
    Uncrystallized(Ref),
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

/// The dispatcher table. [`Ref`] → [`Body<H>`], with
/// [`Crystallizations::crystallize`] as the dispatch primitive. Empty
/// by default; ticks B/F populate it.
///
/// Renamed from `Registry` per the cascade — the plural of
/// [`Crystallization`] names the discipline; `Registry` is generic
/// language. Per landing-page spec §2.3 and
/// `kintsugi-minimum-runnable.md` §11.1.
///
/// `H` has no default (deliberately — landing-page spec §2.1): each
/// consumer (store, db, future engines) declares which `H`-world its
/// crystallizations inhabit at construction. The type system then
/// prevents accidentally registering a `Body<Blake3>` into a
/// `Crystallizations<MockHash>` or dispatching a `Splinter<Blake3>`
/// against a `Crystallizations<OtherHash>`.
pub struct Crystallizations<H: MerkleHash> {
    table: HashMap<Ref, Body<H>>,
    // Mirrors `H` into the struct so the type parameter is genuinely
    // load-bearing even when `table` is empty. `HashMap<Ref, Body<H>>`
    // already references `H`, but the marker keeps the relationship
    // explicit and resilient to future field changes.
    _h: PhantomData<fn(H) -> H>,
}

impl<H: MerkleHash> Crystallizations<H> {
    /// Empty crystallizations. Used at startup by [`floor_crystallizations`].
    pub fn new() -> Self {
        Crystallizations {
            table: HashMap::new(),
            _h: PhantomData,
        }
    }

    /// Register one [`Crystallization`]. The dispatcher learns one
    /// substrate-ref → Rust-body binding.
    pub fn register(&mut self, c: Crystallization<H>) {
        self.table.insert(c.path, c.body);
    }

    /// True iff a [`Body`] is registered for `path`.
    pub fn knows(&self, path: &Ref) -> bool {
        self.table.contains_key(path)
    }

    /// Dispatch: look up the [`Body`] for `path` and invoke it on
    /// `input`. Returns [`Imperfect::Failure`] with
    /// [`CrystallizeError::Uncrystallized`] when `path` has no body —
    /// the substrate claim is not yet realised at this altitude.
    pub fn crystallize(
        &self,
        path: &Ref,
        input: Optic<(), Splinter<H>>,
    ) -> Imperfect<Splinter<H>, CrystallizeError, Transparency<Ref>> {
        match self.table.get(path) {
            Some(body) => body(input),
            None => Imperfect::failure(CrystallizeError::Uncrystallized(path.clone())),
        }
    }
}

impl<H: MerkleHash> Default for Crystallizations<H> {
    fn default() -> Self {
        Self::new()
    }
}

/// The bootstrap's startup crystallizations, generic over the consumer's
/// `H`. Empty in Tick A; ticks B and F populate concrete instantiations
/// as their crystallizations land.
///
/// Per the landing-page spec §2.4, the bootstrap binary may host
/// multiple `H`-worlds. The startup picks one explicitly at the call
/// site (`floor_crystallizations::<Blake3>()` is the default for the
/// `@mirror/store` world).
pub fn floor_crystallizations<H: MerkleHash>() -> Crystallizations<H> {
    Crystallizations::new()
}

/// Free dispatcher consumer: route a [`Ref`] through a
/// [`Crystallizations<H>`] and return the verdict. Equivalent to
/// `crystallizations.crystallize(fracture, input)` — exists as a free
/// function so the bootstrap's `kintsugi_tick` integration point can
/// stay shallow.
///
/// Per `kintsugi-minimum-runnable.md` §11.2: the dispatcher takes the
/// consumer's `Crystallizations<H>` rather than a single global table.
pub fn kintsugi_tick<H: MerkleHash>(
    crystallizations: &Crystallizations<H>,
    fracture: &Ref,
    input: Optic<(), Splinter<H>>,
) -> Imperfect<Splinter<H>, CrystallizeError, Transparency<Ref>> {
    crystallizations.crystallize(fracture, input)
}

// ---------------------------------------------------------------------------
// Tests — Tick A red-first set, cascade-aware.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use prismqueer::Beam;

    // --- Ref validation ---

    #[test]
    fn ref_no_bare_string() {
        // Valid.
        let p = Ref::new("@kintsugi/fracture/rename").unwrap();
        assert_eq!(p.as_str(), "@kintsugi/fracture/rename");
        // Empty rejected.
        assert!(Ref::new("").is_err());
        // Missing '@' prefix rejected.
        assert!(Ref::new("kintsugi/fracture").is_err());
        // Whitespace rejected.
        assert!(Ref::new("@kintsugi fracture").is_err());
    }

    // --- Splinter OID — Text leaf, default Blake3 ---

    #[test]
    fn splinter_oid_roundtrip() {
        let s: Splinter = Splinter::new(Content::Text(Text::new("hello")));
        assert!(s.verify(), "freshly constructed Splinter must verify");
    }

    #[test]
    fn splinter_oid_deterministic() {
        let a: Splinter = Splinter::new(Content::Text(Text::new("hello")));
        let b: Splinter = Splinter::new(Content::Text(Text::new("hello")));
        assert_eq!(a.oid(), b.oid());
    }

    #[test]
    fn splinter_text_different_content_different_oid() {
        let a: Splinter = Splinter::new(Content::Text(Text::new("hello")));
        let b: Splinter = Splinter::new(Content::Text(Text::new("world")));
        assert_ne!(a.oid(), b.oid());
    }

    // --- Splinter OID — Record Merkle ---

    #[test]
    fn splinter_record_merkle() {
        // A Record's OID changes if any sub-Splinter changes.
        let k = FieldName::new("name").unwrap();
        let mut m1: BTreeMap<FieldName, Splinter> = BTreeMap::new();
        m1.insert(k.clone(), Splinter::new(Content::Text(Text::new("alex"))));
        let r1 = Splinter::new(Content::Record(m1));

        let mut m2: BTreeMap<FieldName, Splinter> = BTreeMap::new();
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
        let mut m1: BTreeMap<FieldName, Splinter> = BTreeMap::new();
        m1.insert(
            FieldName::new("a").unwrap(),
            Splinter::new(Content::Text(Text::new("x"))),
        );
        let r1 = Splinter::new(Content::Record(m1));

        let mut m2: BTreeMap<FieldName, Splinter> = BTreeMap::new();
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
        let l1: Splinter = Splinter::new(Content::List(vec![
            Splinter::new(Content::Text(Text::new("a"))),
            Splinter::new(Content::Text(Text::new("b"))),
        ]));
        let l2: Splinter = Splinter::new(Content::List(vec![
            Splinter::new(Content::Text(Text::new("b"))),
            Splinter::new(Content::Text(Text::new("a"))),
        ]));
        assert_ne!(l1.oid(), l2.oid(), "List OID respects order");
        assert!(l1.verify());
    }

    // --- Crystallizations — empty state ---

    #[test]
    fn crystallizations_empty_knows_nothing() {
        let r: Crystallizations<Blake3> = Crystallizations::new();
        let p = Ref::new("@kintsugi/fracture/rename").unwrap();
        assert!(!r.knows(&p));
    }

    #[test]
    fn crystallizations_empty_returns_uncrystallized() {
        let r: Crystallizations<Blake3> = Crystallizations::new();
        let p = Ref::new("@kintsugi/fracture/rename").unwrap();
        let input = Optic::ok((), Splinter::new(Content::Text(Text::new("seed"))));
        let verdict = r.crystallize(&p, input);
        match verdict {
            Imperfect::Failure(CrystallizeError::Uncrystallized(got), _) => {
                assert_eq!(got, p);
            }
            other => panic!("expected Uncrystallized failure, got {:?}", other),
        }
    }

    // --- Crystallizations — register and dispatch ---

    /// Echo body: return the input's value Splinter unchanged as Success.
    fn echo_body() -> Body<Blake3> {
        Arc::new(|input: Optic<(), Splinter<Blake3>>| {
            let splinter = input
                .result()
                .ok()
                .cloned()
                .expect("echo body: input must carry a value");
            Imperfect::success(splinter)
        })
    }

    #[test]
    fn crystallizations_register_and_crystallize() {
        let mut r: Crystallizations<Blake3> = Crystallizations::new();
        let p = Ref::new("@test/echo").unwrap();
        r.register(Crystallization {
            path: p.clone(),
            body: echo_body(),
        });
        assert!(r.knows(&p));

        let seed_splinter: Splinter = Splinter::new(Content::Text(Text::new("hi")));
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
    fn crystallizations_unregistered_after_register() {
        // Register one path; a DIFFERENT path still returns Uncrystallized.
        let mut r: Crystallizations<Blake3> = Crystallizations::new();
        let known = Ref::new("@test/echo").unwrap();
        let unknown = Ref::new("@test/unknown").unwrap();
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

    // --- floor_crystallizations — empty in Tick A ---

    #[test]
    fn floor_crystallizations_is_empty_in_tick_a() {
        let r: Crystallizations<Blake3> = floor_crystallizations();
        let kintsugi = Ref::new("@kintsugi/fracture/rename").unwrap();
        let cli = Ref::new("@cli/new").unwrap();
        assert!(!r.knows(&kintsugi), "Tick A: no kintsugi crystallization");
        assert!(!r.knows(&cli), "Tick A: no @cli crystallization");
    }
}

// ---------------------------------------------------------------------------
// Cascade tests — generic-over-hash surface (the +4 from the cascade).
//
// These exercise the genericity that the existing tests' default-Blake3
// path leaves implicit: a non-Blake3 backend (MockHash), recursive
// Merkle structure under it, multiple H-worlds coexisting in one
// binary, and the free `kintsugi_tick<H>` consumer routing across both.
//
// Per `docs/specs/store-vs-db-and-the-cascade.md` §2 and
// `docs/specs/kintsugi-minimum-runnable.md` §11.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod cascade_tests {
    use super::*;
    use prismqueer::Beam;
    use std::collections::BTreeMap;

    // --- MockHash: a trivial, test-only `MerkleHash` impl ---
    //
    // 4-byte Oid; `hash_bytes` returns the little-endian byte length of
    // the input (folded into 4 bytes). Trivial by construction; the point
    // is that it is *not* Blake3, so the generic-ness is testable rather
    // than asserted.
    #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct MockOid([u8; 4]);

    impl OidBytes for MockOid {
        fn oid_bytes(&self) -> &[u8] {
            &self.0
        }
    }

    #[derive(Clone, Debug)]
    pub struct MockHash;

    impl MerkleHash for MockHash {
        type Oid = MockOid;
        fn hash_bytes(bytes: &[u8]) -> Self::Oid {
            let n = bytes.len() as u32;
            MockOid(n.to_le_bytes())
        }
    }

    // --- Test 1: Splinter works with a non-Blake3 backend ---

    #[test]
    fn splinter_with_mock_hash() {
        let s = Splinter::<MockHash>::new(Content::Text(Text::new("hello")));
        // MockHash::hash_bytes returns LE u32 of input length.
        // Text canonical bytes: "T" || u64_le(5) || "hello" = 1+8+5 = 14 bytes.
        let expected = MockHash::hash_bytes(&[0u8; 14]);
        assert_eq!(s.oid(), &expected);
    }

    // --- Test 2: Recursive Merkle structure under non-Blake3 backend ---

    #[test]
    fn content_record_recursive_with_mock_hash() {
        let leaf = Splinter::<MockHash>::new(Content::Text(Text::new("leaf")));
        let mut map: BTreeMap<FieldName, Splinter<MockHash>> = BTreeMap::new();
        map.insert(FieldName::new("k").unwrap(), leaf);
        let record = Splinter::<MockHash>::new(Content::Record(map));
        assert!(
            record.verify(),
            "Record OID must be Merkle-correct under MockHash"
        );
    }

    // --- Test 3: Multiple H-worlds coexist in one binary ---

    #[test]
    fn crystallizations_blake3_and_mock_coexist() {
        let mut blake3_crys: Crystallizations<Blake3> = Crystallizations::new();
        let mut mock_crys: Crystallizations<MockHash> = Crystallizations::new();

        let p = Ref::new("@test/echo").unwrap();

        blake3_crys.register(Crystallization {
            path: p.clone(),
            body: echo_body_blake3(),
        });
        mock_crys.register(Crystallization {
            path: p.clone(),
            body: echo_body_mock(),
        });

        // Both registries know the path; types prevent cross-mixing.
        assert!(blake3_crys.knows(&p));
        assert!(mock_crys.knows(&p));

        // Dispatching through each returns a Splinter under its own H.
        let blake3_seed = Splinter::<Blake3>::new(Content::Text(Text::new("hi")));
        let blake3_oid = blake3_seed.oid().clone();
        let blake3_input = Optic::ok((), blake3_seed);
        match blake3_crys.crystallize(&p, blake3_input) {
            Imperfect::Success(out) => assert_eq!(out.oid(), &blake3_oid),
            other => panic!("expected Success<Blake3>, got {:?}", other),
        }

        let mock_seed = Splinter::<MockHash>::new(Content::Text(Text::new("hi")));
        let mock_oid = mock_seed.oid().clone();
        let mock_input = Optic::ok((), mock_seed);
        match mock_crys.crystallize(&p, mock_input) {
            Imperfect::Success(out) => assert_eq!(out.oid(), &mock_oid),
            other => panic!("expected Success<MockHash>, got {:?}", other),
        }
    }

    // --- Test 4: kintsugi_tick is generic over H ---

    #[test]
    fn kintsugi_tick_generic_over_h() {
        let p = Ref::new("@test/echo").unwrap();

        let mut blake3_crys: Crystallizations<Blake3> = Crystallizations::new();
        blake3_crys.register(Crystallization {
            path: p.clone(),
            body: echo_body_blake3(),
        });

        let mut mock_crys: Crystallizations<MockHash> = Crystallizations::new();
        mock_crys.register(Crystallization {
            path: p.clone(),
            body: echo_body_mock(),
        });

        // Free `kintsugi_tick<H>` function routes the dispatch generically.
        let blake3_seed = Splinter::<Blake3>::new(Content::Text(Text::new("hi")));
        let blake3_oid = blake3_seed.oid().clone();
        let blake3_input = Optic::ok((), blake3_seed);
        match kintsugi_tick(&blake3_crys, &p, blake3_input) {
            Imperfect::Success(out) => assert_eq!(out.oid(), &blake3_oid),
            other => panic!("expected Success<Blake3>, got {:?}", other),
        }

        let mock_seed = Splinter::<MockHash>::new(Content::Text(Text::new("hi")));
        let mock_oid = mock_seed.oid().clone();
        let mock_input = Optic::ok((), mock_seed);
        match kintsugi_tick(&mock_crys, &p, mock_input) {
            Imperfect::Success(out) => assert_eq!(out.oid(), &mock_oid),
            other => panic!("expected Success<MockHash>, got {:?}", other),
        }
    }

    // --- Helpers ---

    fn echo_body_blake3() -> Body<Blake3> {
        std::sync::Arc::new(|input: Optic<(), Splinter<Blake3>>| {
            let splinter = input
                .result()
                .ok()
                .cloned()
                .expect("echo body: input must carry a value");
            Imperfect::success(splinter)
        })
    }

    fn echo_body_mock() -> Body<MockHash> {
        std::sync::Arc::new(|input: Optic<(), Splinter<MockHash>>| {
            let splinter = input
                .result()
                .ok()
                .cloned()
                .expect("echo body: input must carry a value");
            Imperfect::success(splinter)
        })
    }
}

// ---------------------------------------------------------------------------
// Transparency cascade tests — the 🟢 of the TDD pair opened in `a291b69`.
//
// Post-cascade, `Body<H>` returns
// `Imperfect<Splinter<H>, CrystallizeError, Transparency<Ref>>` (was:
// `... ScalarLoss>`). The local `Ref` definition has been removed in
// favour of `pub use prismqueer::Ref` so the substrate-shared `Ref`
// vocabulary is the single source of truth across mirror, cosmos-mirror,
// spectral-db, and any future prism consumer.
//
// The `BodyT` alias defined here is now structurally identical to
// `Body` (Option α from the 🟢 brief): keeping the alias preserves the
// 🔴 commit's executable-spec verbatim while unifying the implementation.
// The `fracture_validate_body` helper inspects the input shape and opens
// an opacity at `@kintsugi/fracture/validate` on non-Record payloads,
// realising the cascade's structural-verdict semantics.
// ---------------------------------------------------------------------------
#[cfg(test)]
mod transparency_cascade_tests {
    use super::*;
    use prismqueer::{Beam, Diagnostic, PropertyVerdict, Transparency};

    /// Post-cascade alias for [`Body`]. Identical shape; preserved so the
    /// 🔴 commit's test bodies type-check verbatim and the unifying step
    /// is a one-line alias rather than a rewrite of the 🔴 spec.
    pub type BodyT<H> = Body<H>;

    /// A body that opens an opacity at `@kintsugi/fracture/validate` on
    /// any non-Record input. Real implementation (no longer a stub) — the
    /// 🟢 of the cascade: inspect the content shape and surface a located
    /// `PropertyVerdict::Fail` when the payload is not a Record.
    fn fracture_validate_body() -> BodyT<Blake3> {
        Arc::new(|input: Optic<(), Splinter<Blake3>>| {
            let splinter = input
                .result()
                .ok()
                .cloned()
                .expect("fracture_validate_body: input must carry a value");
            match splinter.content() {
                Content::Record(_) => Imperfect::success(splinter),
                other => {
                    let validate_path = Ref::new("@kintsugi/fracture/validate")
                        .expect("@kintsugi/fracture/validate is a valid Ref");
                    let shape_name = match other {
                        Content::Text(_) => "Text",
                        Content::List(_) => "List",
                        Content::Record(_) => unreachable!(),
                    };
                    let verdict = PropertyVerdict::Fail(Diagnostic::new(format!(
                        "fracture body requires Record, got {shape_name}"
                    )));
                    let transparency = Transparency::single(validate_path, verdict);
                    Imperfect::partial(splinter, transparency)
                }
            }
        })
    }

    #[test]
    fn body_t_opens_opacity_on_invalid_shape() {
        // Pass a Text splinter to a body that requires a Record. The body
        // should return Partial with Opaque at @kintsugi/fracture/validate.
        let body = fracture_validate_body();
        let text_splinter: Splinter<Blake3> =
            Splinter::new(Content::Text(Text::new("not-a-record")));
        let input = Optic::ok((), text_splinter);
        let verdict = body(input);
        match verdict {
            Imperfect::Partial(_out, transparency) => {
                let validate_path = Ref::new("@kintsugi/fracture/validate").expect("valid ref");
                assert!(
                    transparency.is_opaque_at(&validate_path),
                    "expected Opaque at @kintsugi/fracture/validate, got {:?}",
                    transparency
                );
                let opacities = transparency.opacities().unwrap();
                match &opacities[&validate_path] {
                    PropertyVerdict::Fail(_) | PropertyVerdict::Partial { .. } => {}
                    other => panic!("expected Fail or Partial verdict, got {:?}", other),
                }
            }
            other => panic!(
                "expected Partial verdict with Transparency loss, got {:?}",
                other
            ),
        }
    }

    #[test]
    fn body_t_clear_on_valid_shape() {
        // Pass a Record splinter — the body should succeed with no loss
        // (Imperfect::Success), Transparency-typed.
        let body = fracture_validate_body();
        let mut m: BTreeMap<FieldName, Splinter<Blake3>> = BTreeMap::new();
        m.insert(
            FieldName::new("name").unwrap(),
            Splinter::new(Content::Text(Text::new("alex"))),
        );
        let record_splinter: Splinter<Blake3> = Splinter::new(Content::Record(m));
        let input = Optic::ok((), record_splinter);
        let verdict = body(input);
        match verdict {
            Imperfect::Success(_) => {}
            other => panic!("expected Success, got {:?}", other),
        }
    }
}
