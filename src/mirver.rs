//! Mirver — structural semantic version of the mirror compiler + active grammar set.
//!
//! `mirver = (compiler_oid, grammars_oid)` where:
//! - `compiler_oid` fingerprints the AST shape (variant set, layout) the compiler produces.
//!   It is bumped by maintainers when [`crate::ast::Ast`] gains/loses variants or fields.
//! - `grammars_oid` is `Oid::hash(beta_normal(active_grammars))`. It tracks the user-defined
//!   grammars (the .mirror files in their project). Identical grammar sources on different
//!   machines produce the same `grammars_oid` regardless of compiler patch version.
//!
//! Two mirvers are [`Compat::Patch`] iff both components are byte-equal.
//! [`Compat::Minor`] / [`Compat::Major`] detection over the AST is stubbed
//! pending a structural AST diff; until then any mismatch is conservatively [`Compat::Major`].
//!
//! The mirver is the "spectral semver" — a release cannot claim Minor compatibility
//! unless the compiler's normalized AST is a strict superset. The compiler proves
//! its own version structurally; no human declares it.
//!
//! Used by spectral cross-repo OIDs: every spectral content OID is prefixed
//! `<mirver_short>:<spectral_sha>`. `memory_cherrypick` requires
//! `compatibility(local, foreign) ∈ {Patch, Minor}` unless overridden.

use crate::ast::{Ast, Atom, Body, Ref};
use crate::kernel::Oid;

// ---------------------------------------------------------------------------
// COMPILER_VERSION_TAG — bump when [`Ast`] shape changes.
// ---------------------------------------------------------------------------

/// Maintainer-bumped tag identifying the [`Ast`] shape this compiler produces.
///
/// Bump rules (proves the version structurally):
/// - **Patch:** internal changes, no `Ast` variant or field affected.
/// - **Minor:** new `Ast` variant added; existing variants unchanged.
/// - **Major:** existing `Ast` variant removed, renamed, or its shape changed.
///
/// The tag string is hashed into [`compiler_oid`]; the literal value is
/// irrelevant beyond uniqueness, so future automated AST-shape derivation
/// can replace this constant without breaking the public API.
pub const COMPILER_VERSION_TAG: &str =
    "mirror/ast=v1: Atom Ref Body Call Prism";

// ---------------------------------------------------------------------------
// Mirver
// ---------------------------------------------------------------------------

/// Layered structural version: compiler shape ⊕ active grammars.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Mirver {
    pub compiler: Oid,
    pub grammars: Oid,
}

impl Mirver {
    /// Compose a mirver from its two components.
    pub fn new(compiler: Oid, grammars: Oid) -> Self {
        Mirver { compiler, grammars }
    }

    /// The mirver of the running compiler with no active grammars.
    /// Equivalent to `Mirver { compiler: compiler_oid(), grammars: empty_grammars_oid() }`.
    pub fn current() -> Self {
        Mirver {
            compiler: compiler_oid(),
            grammars: empty_grammars_oid(),
        }
    }

    /// The mirver of the running compiler with the given parsed grammars.
    pub fn with_grammars(grammars: &[Ast]) -> Self {
        Mirver {
            compiler: compiler_oid(),
            grammars: grammars_oid(grammars),
        }
    }

    /// Short hex form: 8 hex chars of compiler ⊕ 8 hex chars of grammars,
    /// suitable for prefixing spectral content OIDs.
    pub fn short(&self) -> String {
        let c: &str = self.compiler.as_ref();
        let g: &str = self.grammars.as_ref();
        let cn = c.len().min(8);
        let gn = g.len().min(8);
        format!("{}{}", &c[..cn], &g[..gn])
    }
}

// ---------------------------------------------------------------------------
// Compat — structural compatibility classification.
// ---------------------------------------------------------------------------

/// Compatibility relation between two mirvers.
///
/// Reads "from `a` to `b`" — i.e. `compatibility(local, foreign)` asks
/// whether content produced under `foreign` can be safely accepted under `local`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Compat {
    /// Both components byte-equal. Cross-repo OID stable; cherry-pick free.
    Patch,
    /// Compiler equal; grammars are strict superset (b's grammar set ⊇ a's).
    /// All a's content parses under b. **Currently never returned**: stub for the
    /// AST-diff implementation. Treated optimistically as a `Patch` neighbor.
    Minor,
    /// Compiler differs, or grammars differ in non-additive way.
    /// Cross-repo cherry-pick must be opt-in; OID re-derivation may be required.
    Major,
    /// Mirvers share neither compiler nor grammar lineage.
    /// Treated as fully foreign content; cherry-pick refused unless explicitly forced.
    Unknown,
}

/// Compare two mirvers and return the compatibility class.
///
/// Phase 1 implementation:
/// - byte-equal → [`Compat::Patch`]
/// - compiler equal, grammars differ → [`Compat::Major`] (TODO: structural superset → Minor)
/// - compilers differ → [`Compat::Major`] or [`Compat::Unknown`] depending on tag presence
pub fn compatibility(a: &Mirver, b: &Mirver) -> Compat {
    if a == b {
        return Compat::Patch;
    }
    if a.compiler == b.compiler {
        // Same compiler; only grammars differ.
        // TODO: structural AST diff to detect superset/subset for Minor.
        // For now, conservatively classify any grammar mismatch as Major.
        return Compat::Major;
    }
    // Compilers differ. If both came from a tagged mirror compiler,
    // call it Major; otherwise Unknown.
    if a.compiler == empty_grammars_oid() || b.compiler == empty_grammars_oid() {
        Compat::Unknown
    } else {
        Compat::Major
    }
}

// ---------------------------------------------------------------------------
// beta_normal — canonical form (stub: identity).
// ---------------------------------------------------------------------------

/// Canonicalize an AST so that semantically-equivalent inputs hash identically.
///
/// Stub for now: returns the input unchanged. The intended canonicalization is:
/// - **alpha**: rename bound symbols to a deterministic order (currently mirror's
///   AST has no binders that introduce alpha-equivalence classes).
/// - **beta**: substitute resolved references where possible (`@x` where `@x = atom`).
/// - **eta**: drop redundant single-element `Body` wrappers.
/// - **structural**: sort declaration-order-independent siblings (top-level grammar
///   declarations, prism action lists). Order-dependent contexts (Call args)
///   remain untouched.
///
/// Until the full normalizer lands, two grammars that differ only by ordering of
/// top-level declarations will hash differently. This is conservative: it produces
/// false `Major` verdicts but never false `Patch`.
pub fn beta_normal(ast: &Ast) -> Ast {
    // Stub: identity. Replace with structural canonicalization in mirver Phase 2.
    ast.clone()
}

/// [`beta_normal`] applied to a slice of grammars, producing a stable byte
/// serialization suitable for hashing.
fn beta_normal_serialized(asts: &[Ast]) -> Vec<u8> {
    let mut out = Vec::with_capacity(asts.len() * 64);
    for ast in asts {
        let normal = beta_normal(ast);
        emit_canonical(&normal, &mut out);
        out.push(b'\n');
    }
    out
}

/// Emit a canonical byte representation of an AST node.
///
/// Format is deliberately minimal and stable:
/// - `Atom(s)` → `A:<s>`
/// - `Ref(r)`  → `R:<r>`
/// - `Body([..])` → `B(<child>;<child>;...)`
/// - `Call{name, args}` → `C:<name>(<arg>;<arg>;...)`
/// - `Prism{name, body}` → `P:<name>{<body-children>;...}`
fn emit_canonical(ast: &Ast, out: &mut Vec<u8>) {
    match ast {
        Ast::Atom(Atom(s)) => {
            out.extend_from_slice(b"A:");
            out.extend_from_slice(s.as_bytes());
        }
        Ast::Ref(Ref(Atom(s))) => {
            out.extend_from_slice(b"R:");
            out.extend_from_slice(s.as_bytes());
        }
        Ast::Body(Body(children)) => {
            out.extend_from_slice(b"B(");
            for (i, c) in children.iter().enumerate() {
                if i > 0 {
                    out.push(b';');
                }
                emit_canonical(c, out);
            }
            out.push(b')');
        }
        Ast::Call { name, args } => {
            out.extend_from_slice(b"C:");
            out.extend_from_slice(name.as_str().as_bytes());
            out.push(b'(');
            for (i, a) in args.iter().enumerate() {
                if i > 0 {
                    out.push(b';');
                }
                emit_canonical(a, out);
            }
            out.push(b')');
        }
        Ast::Prism { name, body } => {
            out.extend_from_slice(b"P:");
            out.extend_from_slice(name.as_str().as_bytes());
            out.push(b'{');
            for (i, c) in body.children().iter().enumerate() {
                if i > 0 {
                    out.push(b';');
                }
                emit_canonical(c, out);
            }
            out.push(b'}');
        }
    }
}

// ---------------------------------------------------------------------------
// compiler_oid / grammars_oid — the two components of a mirver.
// ---------------------------------------------------------------------------

/// Hash of [`COMPILER_VERSION_TAG`].
///
/// This is the OID of the running compiler's AST shape. Bumping
/// [`COMPILER_VERSION_TAG`] forces all spectral OIDs to gain a new prefix.
pub fn compiler_oid() -> Oid {
    Oid::hash(COMPILER_VERSION_TAG.as_bytes())
}

/// Hash of an empty grammar set. Used by [`Mirver::current`] when no project
/// context is available (e.g., the bare compiler with no `.mirror` files loaded).
pub fn empty_grammars_oid() -> Oid {
    Oid::hash(b"mirror/grammars=empty")
}

/// Hash of the beta-normalized active grammar set.
///
/// `grammars` is the slice of top-level [`Ast`] nodes produced by parsing all
/// active `.mirror` / `.conv` files (typically every grammar declaration the
/// project has registered).
pub fn grammars_oid(grammars: &[Ast]) -> Oid {
    if grammars.is_empty() {
        return empty_grammars_oid();
    }
    let serialized = beta_normal_serialized(grammars);
    Oid::hash(&serialized)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn atom(s: &str) -> Ast {
        Ast::Atom(Atom::new(s))
    }

    fn refn(s: &str) -> Ast {
        Ast::Ref(Ref::new(s))
    }

    #[test]
    fn current_is_deterministic() {
        let a = Mirver::current();
        let b = Mirver::current();
        assert_eq!(a, b);
    }

    #[test]
    fn empty_grammars_oid_is_stable() {
        let a = empty_grammars_oid();
        let b = empty_grammars_oid();
        assert_eq!(a, b);
    }

    #[test]
    fn compiler_oid_matches_tag() {
        let direct = Oid::hash(COMPILER_VERSION_TAG.as_bytes());
        assert_eq!(compiler_oid(), direct);
    }

    #[test]
    fn equal_mirvers_are_patch_compatible() {
        let a = Mirver::current();
        let b = Mirver::current();
        assert_eq!(compatibility(&a, &b), Compat::Patch);
    }

    #[test]
    fn different_grammars_are_major() {
        let a = Mirver::with_grammars(&[atom("foo")]);
        let b = Mirver::with_grammars(&[atom("bar")]);
        assert_eq!(compatibility(&a, &b), Compat::Major);
    }

    #[test]
    fn same_grammars_yield_same_oid() {
        let g = vec![atom("foo"), refn("bar")];
        let a = Mirver::with_grammars(&g);
        let b = Mirver::with_grammars(&g);
        assert_eq!(a, b);
        assert_eq!(compatibility(&a, &b), Compat::Patch);
    }

    #[test]
    fn empty_grammars_match_default() {
        let a = Mirver::current();
        let b = Mirver::with_grammars(&[]);
        assert_eq!(a, b);
    }

    #[test]
    fn short_form_has_expected_length() {
        let m = Mirver::current();
        let s = m.short();
        // Up to 16 hex chars; allow shorter if Oid is shorter.
        assert!(s.len() <= 16);
        assert!(!s.is_empty());
    }

    #[test]
    fn beta_normal_is_identity_for_now() {
        let a = atom("x");
        assert_eq!(beta_normal(&a), a);
    }

    #[test]
    fn canonical_emit_is_deterministic() {
        let a = Ast::Call {
            name: Atom::new("f"),
            args: vec![atom("x"), atom("y")],
        };
        let mut out1 = Vec::new();
        let mut out2 = Vec::new();
        emit_canonical(&a, &mut out1);
        emit_canonical(&a, &mut out2);
        assert_eq!(out1, out2);
        assert_eq!(out1, b"C:f(A:x;A:y)");
    }

    #[test]
    fn distinct_asts_yield_distinct_grammars_oid() {
        let g1 = vec![atom("foo")];
        let g2 = vec![atom("bar")];
        assert_ne!(grammars_oid(&g1), grammars_oid(&g2));
    }
}
