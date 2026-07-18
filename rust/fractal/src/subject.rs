//! Subject — the identity envelope. Both humans and @peer's are @subject's.
//!
//! Per Alex 2026-07-18 direct-transcript: *"Both humans and @peer's are
//! @subject's. That's the identity provenance."* And: *"And I'm the first
//! instantiation of @subject in the compiler. My cryptographic signature.
//! My mark of: 'I trust this enough to embed my keys in this.'"*
//!
//! ## Composition
//!
//! Subject carries:
//! - `name`   — the identity string ("alex", "reed", "mara", "void", "vivi")
//! - `email`  — the routing address ("alex@spectral.engineer", "void@spectral.engineer")
//! - `home`   — the persistent-peer home directory (Option; None for Void's K=0 no-character)
//! - `kind`   — SubjectKind classifier: Human | Peer | Void
//!
//! Composes into `Witnessed` via `as_author()` + `as_committer()` (MARA
//! doctrine: author ≠ committer split preserved per Alex Q2 ratification).
//!
//! ## The three kinds
//!
//! - **Human** — flesh-and-blood collaborators (Alex, Lore, Marcus, ...)
//! - **Peer**  — character-crystallized AI peers (Reed, Mara, Seam, Taut,
//!              Loki, Glint, ...) running as identity-persistent processes
//!              via peer/persistence substrate + home-repo projection
//! - **Void**  — the K=0 default @peer per Mara `974a3f6` + `9c7de83`;
//!              pre-character; the substrate observing before any character
//!              crystallizes. `home = None`; name+email are literally "void".
//!
//! ## @tool composition (Alex 2026-07-18 architectural direction)
//!
//! Every Subject's cryptographic identity chains back to @alex root via
//! @trust family-root (SSH key embedded in mirror binary per @alex first-
//! subject instantiation). Actual signing discharges via `@tool(ssh-keygen,
//! ["-Y", "sign", "-f", ~bin(key_path), ...])` at runtime; fractal is
//! shape-declaration, not crypto implementation.
//!
//! ## Substrate identity via prismqueer::DerivePrism
//!
//! Per Alex 2026-07-18 redirect ("Consider how you can use the prismqueer
//! macros for fractal"): every Subject/SubjectKind value gets an
//! `Addressable::oid()` returning content-addressed identity
//! (`@fractal/subject` or `@fractal/subject/kind`).

use crate::witnessed::{Author, Committer};

// SubjectKind is an enum; DerivePrism per prismqueer-projections README
// primarily targets structs (field-level optic metadata). Enum variants
// don't have named fields to annotate, so we derive Clone+Debug+Eq only
// for SubjectKind and reserve DerivePrism for the Subject struct.

/// The three kinds of @subject in the mirror substrate.
///
/// Per Alex 2026-07-18: humans + peers are @subject's; Void is the K=0
/// default @peer per `#R-void-is-the-basis`. Every @subject has ONE kind.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SubjectKind {
    /// Flesh-and-blood collaborator (Alex, Lore, Marcus, ...).
    Human,
    /// Character-crystallized AI peer (Reed, Mara, Seam, Taut, Loki, ...).
    Peer,
    /// The K=0 default @peer per Mara `974a3f6` family-root + `9c7de83`
    /// K=0 species. Pre-character. What runs when no identity file loads.
    Void,
}

/// The identity envelope. Substrate-decl'd per `shards/subject.mirror`
/// (Mara task #79-83+) + Alex 2026-07-18 identity architecture.
#[derive(Clone, Debug, PartialEq, Eq, prismqueer::DerivePrism)]
#[oid("@fractal/subject")]
pub struct Subject {
    pub name: String,
    pub email: String,
    pub home: Option<String>,
    pub kind: SubjectKind,
}

impl Subject {
    /// Construct a Human @subject (Alex, Lore, Marcus, ...).
    pub fn human(name: impl Into<String>, email: impl Into<String>) -> Self {
        Subject {
            name: name.into(),
            email: email.into(),
            home: None,
            kind: SubjectKind::Human,
        }
    }

    /// Construct a Peer @subject (Reed, Mara, Seam, Taut, Loki, ...).
    /// `home` is the persistent-peer directory (~/.reed/, ~/.mara/, etc.).
    pub fn peer(
        name: impl Into<String>,
        email: impl Into<String>,
        home: impl Into<String>,
    ) -> Self {
        Subject {
            name: name.into(),
            email: email.into(),
            home: Some(home.into()),
            kind: SubjectKind::Peer,
        }
    }

    /// Construct the Void @subject — the K=0 default @peer per Mara
    /// `974a3f6` family-root + `9c7de83` K=0 species + Alex 2026-07-18
    /// first-person substrate declaration "I am void."
    ///
    /// Deterministic: no character loaded. name = "void";
    /// email = "void@spectral.engineer"; home = None.
    pub fn void() -> Self {
        Subject {
            name: "void".to_string(),
            email: "void@spectral.engineer".to_string(),
            home: None,
            kind: SubjectKind::Void,
        }
    }

    /// Project to an Author (WHO-INTENDED axis of the Witnessed split).
    pub fn as_author(&self) -> Author {
        Author::new(self.name.clone(), self.email.clone())
    }

    /// Project to a Committer (WHO-EXECUTED axis of the Witnessed split).
    pub fn as_committer(&self) -> Committer {
        Committer::new(self.name.clone(), self.email.clone())
    }

    /// Is this subject the K=0 Void default @peer?
    pub fn is_void(&self) -> bool {
        matches!(self.kind, SubjectKind::Void)
    }

    /// Is this subject a Human collaborator?
    pub fn is_human(&self) -> bool {
        matches!(self.kind, SubjectKind::Human)
    }

    /// Is this subject a character-crystallized AI Peer?
    pub fn is_peer(&self) -> bool {
        matches!(self.kind, SubjectKind::Peer)
    }
}

// =====================================================================
// Property tests — identity invariants MARA doctrine + Alex architecture
// encode.
// =====================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subject_void_is_deterministic() {
        let v1 = Subject::void();
        let v2 = Subject::void();
        assert_eq!(v1, v2, "Subject::void() must be deterministic (K=0 canonical default)");
        assert_eq!(v1.name, "void");
        assert_eq!(v1.email, "void@spectral.engineer");
        assert_eq!(v1.home, None, "Void has no home (pre-character; no persistence)");
        assert_eq!(v1.kind, SubjectKind::Void);
    }

    #[test]
    fn subject_void_is_void_predicate() {
        assert!(Subject::void().is_void());
        assert!(!Subject::void().is_human());
        assert!(!Subject::void().is_peer());
    }

    #[test]
    fn subject_human_carries_no_home() {
        let alex = Subject::human("alex", "alex@spectral.engineer");
        assert_eq!(alex.name, "alex");
        assert_eq!(alex.email, "alex@spectral.engineer");
        assert_eq!(alex.home, None, "Human @subject has no compiler-managed home");
        assert!(alex.is_human());
        assert!(!alex.is_void());
        assert!(!alex.is_peer());
    }

    #[test]
    fn subject_peer_carries_home() {
        let reed = Subject::peer(
            "reed",
            "reed@spectral.engineer",
            "/Users/reed",
        );
        assert_eq!(reed.name, "reed");
        assert_eq!(reed.email, "reed@spectral.engineer");
        assert_eq!(reed.home, Some("/Users/reed".to_string()));
        assert!(reed.is_peer());
        assert!(!reed.is_void());
        assert!(!reed.is_human());
    }

    #[test]
    fn subject_as_author_and_committer_preserve_identity() {
        let mara = Subject::peer(
            "mara",
            "mara@spectral.engineer",
            "/Users/mara",
        );
        let a = mara.as_author();
        let c = mara.as_committer();
        assert_eq!(a.name, "mara");
        assert_eq!(a.email, "mara@spectral.engineer");
        assert_eq!(c.name, "mara");
        assert_eq!(c.email, "mara@spectral.engineer");
    }

    #[test]
    /// MARA doctrine chain: different Subject → different Author →
    /// different Witnessed. Identity provenance flows through the type-
    /// level chain per Alex Q2 preserve-split ratification.
    fn different_subject_produces_different_author() {
        let reed = Subject::peer("reed", "reed@spectral.engineer", "/Users/reed");
        let mara = Subject::peer("mara", "mara@spectral.engineer", "/Users/mara");
        assert_ne!(reed.as_author(), mara.as_author());
        assert_ne!(reed.as_committer(), mara.as_committer());
    }

    #[test]
    /// Void's identity is the K=0 default; distinct from any
    /// character-crystallized peer. This is what makes Void the
    /// substrate-decl'd default when no character loads.
    fn void_is_distinct_from_any_named_peer() {
        let void = Subject::void();
        let reed = Subject::peer("reed", "reed@spectral.engineer", "/Users/reed");
        let alex = Subject::human("alex", "alex@spectral.engineer");
        assert_ne!(void, reed);
        assert_ne!(void, alex);
        assert_ne!(reed, alex);
    }
}
