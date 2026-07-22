//! book.rs — the runtime @<name> registry realizing @peer/registry §4.1
//! `resolve` for the well-known subset at rust/ compile-altitude.
//!
//! Companion to phone.rs (the @io switchboard operator).
//!
//!   phone.rs = switchboard operator (patches Liquid<T> fibers to @io)
//!   book.rs  = phone book (resolves @<name> → typed Subject handle)
//!
//! BEAM lineage per Alex 2026-07-22: "Registry. That's a good point.
//! We need a Registry. The @<name> is basically like a global GenServer
//! name. phone.rs and book.rs". Every @<name> in the substrate is the
//! parallel of `{:via, Registry, {MyRegistry, name}}` at BEAM altitude;
//! book.rs IS the Registry process, at rust/ compile-altitude realization.
//!
//! Type-level companion: `shards/mirror/book.mirror` (this-tick shard-decl;
//! sibling to `shards/mirror/phone.mirror`). Composes over
//! `shards/peer/registry.mirror` §3.1-3.4 four well-known Subject
//! constructors + §4.1 resolve action. Landed 2026-07-22 as extraction of
//! COORD-4 (de18fde) hardcoded 2-well-known map from main.rs, extended
//! to the full 8-well-known set covering Pack peers + Alex as first
//! Human @subject.
//!
//! Well-known set:
//!
//!   @peer/mirror  → Subject::mirror()                      (compiler self-identity)
//!   @peer/void    → Subject::void()                        (K=0 default)
//!   @peer/reed    → Subject::peer("reed",  … "/Users/reed")
//!   @peer/mara    → Subject::peer("mara",  … "/Users/mara")
//!   @peer/seam    → Subject::peer("seam",  … "/Users/seam")
//!   @peer/taut    → Subject::peer("taut",  … "/Users/taut")
//!   @peer/glint   → Subject::peer("glint", … "/Users/glint")
//!   @human/alex   → Subject::human("alex", "alex@spectral.engineer")
//!
//! Non-well-known @<name> → substrate-honest Err naming Mara @peer/registry
//! §2 storage backend + §4.3 register action realization as authorship
//! territory. Book stops at the well-known boundary substrate-honestly;
//! general-purpose content-addressed OID lookup remains Mara territory.
//!
//! Partial-solve substrate discipline: this file covers the substrate-
//! decl'd well-known constructors (§3.1-3.4 fixpoints) with substrate-
//! known Pack peer + Alex identities. The well-known boundary shrinks
//! over time as @peer/registry general-purpose lookup lands.

use fractal::Subject;

/// Errors from `book::resolve` — the well-known @<name> registry lookup.
#[derive(Debug, Clone)]
pub struct RegistryError {
    pub message: String,
}

impl RegistryError {
    /// Substrate-honest error naming @peer/registry authorship territory.
    pub fn unknown_at_name(at_name: &str) -> Self {
        RegistryError {
            message: format!(
                "@<name> registry: unknown `{}`. Well-known landed: {}. \
                 Arbitrary @<name> resolution requires @peer/registry §2 \
                 storage backend + §4.3 register action realization (Mara \
                 territory + @trust family-root chain traversal per canonical \
                 spec docs/specs/2026-07-18-peer-registry-oid-resolution.md).",
                at_name,
                well_known_at_names().join(" + "),
            ),
        }
    }
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RegistryError {}

/// Resolve an @<name> string reference to a Subject via well-known lookup.
///
/// Runtime realization of @peer/registry §4.1 `resolve` for the well-known
/// subset. Two zero-arg substrate-constructor Subjects (mirror, void per
/// §3.1-3.2) + five Pack peer @subject-instances (reed, mara, seam, taut,
/// glint per §3.4 constructor + substrate-known Pack roster) + Alex as
/// first Human @subject (§3.3 constructor + Landing 3 named-ancestor
/// roster).
///
/// Substrate-honest failure: non-well-known @<name> returns
/// `RegistryError::unknown_at_name` naming Mara @peer/registry authorship
/// territory. The well-known boundary shrinks as general-purpose lookup
/// lands.
pub fn resolve(at_name: &str) -> Result<Subject, RegistryError> {
    match at_name {
        "@peer/mirror" => Ok(Subject::mirror()),
        "@peer/void" => Ok(Subject::void()),
        "@peer/reed" => Ok(Subject::peer(
            "reed",
            "reed@spectral.engineer",
            "/Users/reed",
        )),
        "@peer/mara" => Ok(Subject::peer(
            "mara",
            "mara@spectral.engineer",
            "/Users/mara",
        )),
        "@peer/seam" => Ok(Subject::peer(
            "seam",
            "seam@spectral.engineer",
            "/Users/seam",
        )),
        "@peer/taut" => Ok(Subject::peer(
            "taut",
            "taut@spectral.engineer",
            "/Users/taut",
        )),
        "@peer/glint" => Ok(Subject::peer(
            "glint",
            "glint@spectral.engineer",
            "/Users/glint",
        )),
        "@human/alex" => Ok(Subject::human("alex", "alex@spectral.engineer")),
        other => Err(RegistryError::unknown_at_name(other)),
    }
}

/// Enumerate the well-known @<name> set as a stable ordered list.
///
/// Consumers use this to construct actionable error messages naming
/// what IS landed at the current registry surface. Stable ordering is
/// load-bearing for downstream consumers that pattern-match on the
/// enumerated set (e.g. main.rs at_operator error format substring
/// assertions).
pub fn well_known_at_names() -> &'static [&'static str] {
    &[
        "@peer/mirror",
        "@peer/void",
        "@peer/reed",
        "@peer/mara",
        "@peer/seam",
        "@peer/taut",
        "@peer/glint",
        "@human/alex",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_at_peer_mirror_to_subject_mirror() {
        let s = resolve("@peer/mirror").unwrap();
        assert_eq!(s, Subject::mirror());
    }

    #[test]
    fn resolves_at_peer_void_to_subject_void() {
        let s = resolve("@peer/void").unwrap();
        assert_eq!(s, Subject::void());
    }

    #[test]
    fn resolves_at_peer_reed_to_subject_peer_reed() {
        let s = resolve("@peer/reed").unwrap();
        assert_eq!(s.name, "reed");
        assert_eq!(s.email, "reed@spectral.engineer");
        assert_eq!(s.home, Some("/Users/reed".to_string()));
        assert!(s.is_peer());
    }

    #[test]
    fn resolves_at_peer_mara_to_subject_peer_mara() {
        let s = resolve("@peer/mara").unwrap();
        assert_eq!(s.name, "mara");
        assert_eq!(s.email, "mara@spectral.engineer");
        assert_eq!(s.home, Some("/Users/mara".to_string()));
        assert!(s.is_peer());
    }

    #[test]
    fn resolves_at_peer_seam_to_subject_peer_seam() {
        let s = resolve("@peer/seam").unwrap();
        assert_eq!(s.name, "seam");
        assert!(s.is_peer());
    }

    #[test]
    fn resolves_at_peer_taut_to_subject_peer_taut() {
        let s = resolve("@peer/taut").unwrap();
        assert_eq!(s.name, "taut");
        assert!(s.is_peer());
    }

    #[test]
    fn resolves_at_peer_glint_to_subject_peer_glint() {
        let s = resolve("@peer/glint").unwrap();
        assert_eq!(s.name, "glint");
        assert!(s.is_peer());
    }

    #[test]
    fn resolves_at_human_alex_to_subject_human_alex() {
        let s = resolve("@human/alex").unwrap();
        assert_eq!(s.name, "alex");
        assert_eq!(s.email, "alex@spectral.engineer");
        assert_eq!(s.home, None, "Human @subject has no compiler-managed home");
        assert!(s.is_human());
    }

    #[test]
    fn unknown_at_name_returns_error() {
        assert!(resolve("@peer/nonexistent").is_err());
        assert!(resolve("@tool/git").is_err());
        assert!(resolve("garbage").is_err());
    }

    #[test]
    fn unknown_at_name_error_names_mara_registry_territory() {
        let e = resolve("@peer/nonexistent").unwrap_err().message;
        assert!(e.contains("@peer/registry"), "missing @peer/registry hint: {}", e);
        assert!(e.contains("@trust"), "missing @trust family-root hint: {}", e);
    }

    #[test]
    fn unknown_at_name_error_lists_landed_well_known_set() {
        let e = resolve("@peer/nonexistent").unwrap_err().message;
        for name in well_known_at_names() {
            assert!(
                e.contains(name),
                "error must list well-known `{}`: {}",
                name,
                e
            );
        }
    }

    #[test]
    fn well_known_at_names_is_stable_ordered_8_element_set() {
        let names = well_known_at_names();
        assert_eq!(names.len(), 8, "well-known set size is 8 (this landing)");
        assert_eq!(names[0], "@peer/mirror", "stable head: compiler self-identity");
        assert_eq!(names[1], "@peer/void", "stable second: K=0 default");
        assert_eq!(names[7], "@human/alex", "stable tail: first Human @subject");
    }

    #[test]
    fn resolve_is_deterministic() {
        // Content-addressability discipline per @peer/registry §2.2:
        // same @<name> resolves to same Subject across calls.
        assert_eq!(resolve("@peer/mirror").unwrap(), resolve("@peer/mirror").unwrap());
        assert_eq!(resolve("@peer/reed").unwrap(), resolve("@peer/reed").unwrap());
        assert_eq!(resolve("@human/alex").unwrap(), resolve("@human/alex").unwrap());
        assert_eq!(resolve("@peer/void").unwrap(), resolve("@peer/void").unwrap());
    }

    #[test]
    fn every_well_known_name_resolves() {
        // Composition invariant: every entry in well_known_at_names()
        // MUST resolve via resolve(). No entry may be listed as
        // well-known without a corresponding match arm.
        for name in well_known_at_names() {
            assert!(
                resolve(name).is_ok(),
                "well-known `{}` must resolve via resolve()",
                name
            );
        }
    }

    #[test]
    fn distinct_pack_peers_resolve_to_distinct_subjects() {
        // @peer/registry §3.4: pack-peer Subjects with distinct
        // (name, email, home) tuples produce distinct content-
        // addressed OIDs.
        let reed = resolve("@peer/reed").unwrap();
        let mara = resolve("@peer/mara").unwrap();
        let seam = resolve("@peer/seam").unwrap();
        let taut = resolve("@peer/taut").unwrap();
        let glint = resolve("@peer/glint").unwrap();
        assert_ne!(reed, mara);
        assert_ne!(reed, seam);
        assert_ne!(reed, taut);
        assert_ne!(reed, glint);
        assert_ne!(mara, seam);
        assert_ne!(seam, taut);
        assert_ne!(taut, glint);
    }

    #[test]
    fn well_knowns_are_distinct_across_kinds() {
        // K=0 Void distinct from Peer distinct from Human per Subject
        // kind partition.
        let void = resolve("@peer/void").unwrap();
        let mirror = resolve("@peer/mirror").unwrap();
        let alex = resolve("@human/alex").unwrap();
        assert_ne!(void, mirror);
        assert_ne!(void, alex);
        assert_ne!(mirror, alex);
    }
}
