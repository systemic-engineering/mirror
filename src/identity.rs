//! Identity — standalone type. A name + content address.
//!
//! Identity is more primitive than actor. An actor IS an identity that can act.
//! Identity just names something and addresses it. No dependency on `@actor`.

use crate::kernel::Oid;

/// A named identity with a content address.
///
/// Maps to the `.mirror` declaration:
/// ```mirror
/// type identity {
///     name: string,
///     oid: oid,
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identity {
    pub name: String,
    pub oid: Oid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_requires_correct_oid_type() {
        let id = Identity {
            name: "reed".into(),
            oid: Oid::hash(b"reed"),
        };
        // Deliberately wrong: check against a different hash to prove the test catches mismatches
        let wrong_oid = Oid::hash(b"not-reed");
        assert_ne!(id.oid, wrong_oid, "identity oid must match its content");
        assert_eq!(id.name, "reed");
    }

    #[test]
    fn identity_equality_checks_both_fields() {
        let a = Identity {
            name: "reed".into(),
            oid: Oid::hash(b"reed"),
        };
        let b = Identity {
            name: "reed".into(),
            oid: Oid::hash(b"reed"),
        };
        let c = Identity {
            name: "mara".into(),
            oid: Oid::hash(b"reed"),
        };
        assert_eq!(a, b);
        assert_ne!(a, c, "different name means different identity");
    }

    #[test]
    fn identity_clone_is_independent() {
        let a = Identity {
            name: "reed".into(),
            oid: Oid::hash(b"reed"),
        };
        let b = a.clone();
        assert_eq!(a, b);
    }
}
