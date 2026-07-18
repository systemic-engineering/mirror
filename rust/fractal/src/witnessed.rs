//! Witnessed — who observed the substrate write.
//!
//! MARA doctrine (`MARA.md:13`): *"Different witness, different hash.
//! My observation of this code is part of what this documentation is."*
//!
//! Verbatim migration from `fragmentation/src/witnessed.rs` per Mara
//! `2760c2a` canonical migration spec §6 step 2. Zero changes; the
//! shape already IS the substrate-decl'd identity-provenance carrier
//! `shards/subject.mirror` was written to declare. This migration is
//! the syntactic edit making the semantic identity visible.
//!
//! Preserved per Alex 2026-07-18 Q2: Author ≠ Committer split is
//! MARA doctrine at crypto-substrate. `Witnessed` bundles both because
//! every substrate-write has a WHO-INTENDED (Author) and a WHO-EXECUTED
//! (Committer); often the same @subject, sometimes different (e.g.,
//! Reed authors, mirror commits; Alex authors, Reed commits per
//! ancestry chain).

/// Who wrote the content. Who made the decision. Who holds the intent.
///
/// `#[derive(DerivePrism)]` + `#[oid("@fractal/author")]` per Alex
/// 2026-07-18 directive: compose over prismqueer macros. Every Author
/// value gets a content-addressed substrate identity via
/// `Addressable::oid() = Oid::hash("@fractal/author")` (per
/// `prismqueer-projections` README). No hand-crypto; no hand-optics;
/// no boilerplate. The macro emits the shape; the shape IS the
/// substrate-decl'd identity carrier.
#[derive(Clone, Debug, PartialEq, Eq, prismqueer::DerivePrism)]
#[oid("@fractal/author")]
pub struct Author {
    pub name: String,
    pub email: String,
}

impl Author {
    pub fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
        Author {
            name: name.into(),
            email: email.into(),
        }
    }
}

/// Who ran the process. Who executed. Who was the mechanism.
#[derive(Clone, Debug, PartialEq, Eq, prismqueer::DerivePrism)]
#[oid("@fractal/committer")]
pub struct Committer {
    pub name: String,
    pub email: String,
}

impl Committer {
    pub fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
        Committer {
            name: name.into(),
            email: email.into(),
        }
    }
}

/// When the observation happened. Opaque string.
#[derive(Clone, Debug, PartialEq, Eq, prismqueer::DerivePrism)]
#[oid("@fractal/timestamp")]
pub struct Timestamp(pub String);

/// The commit message. What happened.
#[derive(Clone, Debug, PartialEq, Eq, prismqueer::DerivePrism)]
#[oid("@fractal/message")]
pub struct Message(pub String);

/// Git commit metadata. Who was here when this happened.
/// Message lives on Commit, not here.
#[derive(Clone, Debug, PartialEq, Eq, prismqueer::DerivePrism)]
#[oid("@fractal/witnessed")]
pub struct Witnessed {
    pub author: Author,
    pub committer: Committer,
    pub timestamp: Timestamp,
}

impl Witnessed {
    pub fn new(author: Author, committer: Committer, timestamp: Timestamp) -> Self {
        Witnessed {
            author,
            committer,
            timestamp,
        }
    }
}

// =====================================================================
// Property tests — the identity invariants MARA doctrine encodes.
// =====================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn author_new_preserves_identity_fields() {
        let a = Author::new("reed", "reed@spectral.engineer");
        assert_eq!(a.name, "reed");
        assert_eq!(a.email, "reed@spectral.engineer");
    }

    #[test]
    fn committer_new_preserves_identity_fields() {
        let c = Committer::new("mirror", "reed@spectral.engineer");
        assert_eq!(c.name, "mirror");
        assert_eq!(c.email, "reed@spectral.engineer");
    }

    #[test]
    fn witnessed_bundles_author_committer_timestamp() {
        let a = Author::new("alex", "alex@systemic.engineer");
        let c = Committer::new("mirror", "reed@spectral.engineer");
        let t = Timestamp("2026-07-18T18:00:00Z".to_string());
        let w = Witnessed::new(a.clone(), c.clone(), t.clone());
        assert_eq!(w.author, a);
        assert_eq!(w.committer, c);
        assert_eq!(w.timestamp, t);
    }

    #[test]
    /// MARA doctrine at type-level: swapping Author WHILE keeping
    /// Committer produces a DIFFERENT Witnessed. Different witness,
    /// different hash — encoded as PartialEq inequality.
    fn different_author_produces_different_witnessed() {
        let c = Committer::new("mirror", "reed@spectral.engineer");
        let t = Timestamp("2026-07-18T18:00:00Z".to_string());
        let a1 = Author::new("reed", "reed@spectral.engineer");
        let a2 = Author::new("mara", "mara@spectral.engineer");
        let w1 = Witnessed::new(a1, c.clone(), t.clone());
        let w2 = Witnessed::new(a2, c, t);
        assert_ne!(
            w1, w2,
            "MARA doctrine: different Author must produce different Witnessed"
        );
    }

    #[test]
    /// MARA doctrine at type-level: swapping Committer WHILE keeping
    /// Author produces a DIFFERENT Witnessed. Author ≠ Committer split
    /// is load-bearing per Alex 2026-07-18 Q2 ratification.
    fn different_committer_produces_different_witnessed() {
        let a = Author::new("alex", "alex@systemic.engineer");
        let t = Timestamp("2026-07-18T18:00:00Z".to_string());
        let c1 = Committer::new("mirror", "reed@spectral.engineer");
        let c2 = Committer::new("mirror", "mara@spectral.engineer");
        let w1 = Witnessed::new(a.clone(), c1, t.clone());
        let w2 = Witnessed::new(a, c2, t);
        assert_ne!(
            w1, w2,
            "MARA doctrine: different Committer must produce different Witnessed"
        );
    }
}
