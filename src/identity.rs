use crate::domain::Context;
use std::marker::PhantomData;

/// A name. Value type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name(String);

/// An email address. Value type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Email(String);

/// Who you are, regardless of context.
pub trait Identity: Clone + std::fmt::Debug {
    type Keys: fragmentation::keys::Keys;

    fn name(&self) -> &Name;
    fn email(&self) -> Option<&Email>;
    fn keys(&self) -> &Self::Keys;
}

/// Typed communication within a context.
pub struct Signal<C: Context> {
    pub token: C::Token,
}

/// A signal with identity provenance.
pub struct Signed<S> {
    pub inner: S,
    pub signer: Name,
}

/// A composable node in the system. Carries name and keys.
///
/// Context-parameterized: `System<Filesystem>` carries `PlainKeys`,
/// a future `System<Encrypted>` could carry SSH/GPG keys.
/// Default context is Filesystem (PlainKeys, infallible).
///
/// Every observation in a session is attributable to a system.
#[derive(Debug, Clone)]
pub struct System<C: Context = crate::domain::filesystem::Filesystem> {
    name: String,
    keys: C::Keys,
}

impl<C: Context> System<C> {
    pub fn new(name: impl Into<String>, keys: C::Keys) -> Self {
        System {
            name: name.into(),
            keys,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn keys(&self) -> &C::Keys {
        &self.keys
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::filesystem::Filesystem;
    use fragmentation::keys::PlainKeys;

    // -- System (existing) --

    #[test]
    fn system_filesystem_has_plain_keys() {
        let system: System<Filesystem> = System::new("Reed", PlainKeys);
        assert_eq!(system.name(), "Reed");
        assert_eq!(*system.keys(), PlainKeys);
    }

    #[test]
    fn system_default_is_filesystem() {
        let system: System = System::new("Reed", PlainKeys);
        assert_eq!(system.name(), "Reed");
        assert_eq!(*system.keys(), PlainKeys);
    }

    #[test]
    fn system_conversation_domain() {
        use crate::domain::conversation::Conversation;
        let system: System<Conversation> = System::new("Reed", PlainKeys);
        assert_eq!(system.name(), "Reed");
        assert_eq!(*system.keys(), PlainKeys);
    }

    // -- Name --

    #[test]
    fn name_wraps_string() {
        let name = Name::new("Reed");
        assert_eq!(name.as_ref(), "Reed");
    }

    #[test]
    fn name_is_clone_eq_debug() {
        let a = Name::new("Reed");
        let b = a.clone();
        assert_eq!(a, b);
        assert_eq!(format!("{:?}", a), "Name(\"Reed\")");
    }

    // -- Email --

    #[test]
    fn email_wraps_string() {
        let email = Email::new("reed@systemic.engineer");
        assert_eq!(email.as_ref(), "reed@systemic.engineer");
    }

    #[test]
    fn email_is_clone_eq_debug() {
        let a = Email::new("reed@systemic.engineer");
        let b = a.clone();
        assert_eq!(a, b);
        assert_eq!(format!("{:?}", a), "Email(\"reed@systemic.engineer\")");
    }

    // -- Identity trait --

    #[test]
    fn identity_name_and_keys() {
        let id = TestIdentity::new("Reed", None);
        assert_eq!(id.name().as_ref(), "Reed");
        assert!(id.email().is_none());
        assert_eq!(*id.keys(), PlainKeys);
    }

    #[test]
    fn identity_with_email() {
        let id = TestIdentity::new("Reed", Some("reed@systemic.engineer"));
        let email = id.email().unwrap();
        assert_eq!(email.as_ref(), "reed@systemic.engineer");
    }

    // -- Signal --

    #[test]
    fn signal_wraps_token() {
        use crate::domain::filesystem::Folder;
        let signal: Signal<Filesystem> = Signal {
            token: Folder {
                name: "test".into(),
                content: None,
            },
        };
        assert_eq!(signal.token.name, "test");
    }

    // -- Signed --

    #[test]
    fn signed_wraps_with_signer() {
        let signed = Signed {
            inner: "hello",
            signer: Name::new("Reed"),
        };
        assert_eq!(signed.inner, "hello");
        assert_eq!(signed.signer.as_ref(), "Reed");
    }
}
