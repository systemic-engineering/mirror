use crate::domain::Context;

/// A name. Value type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name(String);

impl Name {
    pub fn new(name: impl Into<String>) -> Self {
        Name(name.into())
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// An email address. Value type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn new(email: impl Into<String>) -> Self {
        Email(email.into())
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Who you are, regardless of context.
pub trait Identity: Clone + std::fmt::Debug {
    type Keys: fragmentation::keys::Keys;

    fn name(&self) -> &Name;
    fn email(&self) -> Option<&Email>;
    fn keys(&self) -> &Self::Keys;
}

/// Typed communication within a context.
#[derive(Debug)]
pub struct Signal<C: Context> {
    pub token: C::Token,
}

impl<C: Context> crate::witness::ContentAddressed for Signal<C> {
    fn content_oid(&self) -> crate::witness::Oid {
        self.token.content_oid()
    }
}

/// Cryptographic proof of identity.
pub struct Signature<I: Identity> {
    pub signer: I,
    pub signature: Vec<u8>,
}

/// A signal with identity provenance.
pub struct Signed<I: Identity, C: Context> {
    pub signature: Signature<I>,
    pub signal: Signal<C>,
}

/// Anything with identity in a context.
pub trait System<C: Context> {
    type Identity: Identity;

    fn identity(&self) -> &Self::Identity;
}

/// Leaf node: a named identity in one context.
#[derive(Debug, Clone)]
pub struct Node<I: Identity, C: Context> {
    pub id: I,
    context: C,
}

impl<I: Identity, C: Context> Node<I, C> {
    pub fn new(id: I, context: C) -> Self {
        Node { id, context }
    }

    pub fn context(&self) -> &C {
        &self.context
    }
}

impl<I: Identity, C: Context> System<C> for Node<I, C> {
    type Identity = I;

    fn identity(&self) -> &I {
        &self.id
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::domain::filesystem::Filesystem;
    use fragmentation::keys::PlainKeys;

    #[derive(Clone, Debug)]
    pub(crate) struct TestIdentity {
        name: Name,
        email: Option<Email>,
        keys: PlainKeys,
    }

    impl TestIdentity {
        pub(crate) fn new(name: &str, email: Option<&str>) -> Self {
            TestIdentity {
                name: Name::new(name),
                email: email.map(Email::new),
                keys: PlainKeys,
            }
        }
    }

    impl Identity for TestIdentity {
        type Keys = PlainKeys;

        fn name(&self) -> &Name {
            &self.name
        }

        fn email(&self) -> Option<&Email> {
            self.email.as_ref()
        }

        fn keys(&self) -> &PlainKeys {
            &self.keys
        }
    }

    // -- Node + System trait --

    #[test]
    fn node_carries_identity() {
        let node = Node::new(TestIdentity::new("Reed", None), Filesystem);
        assert_eq!(node.identity().name().as_ref(), "Reed");
    }

    #[test]
    fn node_implements_system() {
        fn assert_is_system<C: Context>(_: &impl System<C>) {}
        let node = Node::new(TestIdentity::new("Reed", None), Filesystem);
        assert_is_system::<Filesystem>(&node);
        assert_eq!(node.identity().name().as_ref(), "Reed");
    }

    #[test]
    fn node_in_different_contexts() {
        use crate::domain::git::Git;
        let fs_node = Node::new(TestIdentity::new("fs", None), Filesystem);
        let git_node = Node::new(TestIdentity::new("git", None), Git);
        assert_eq!(fs_node.identity().name().as_ref(), "fs");
        assert_eq!(git_node.identity().name().as_ref(), "git");
        assert_eq!(*fs_node.context(), Filesystem);
        assert_eq!(*git_node.context(), Git);
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

    // -- Signal ContentAddressed --

    #[test]
    fn signal_content_addressed() {
        use crate::domain::filesystem::Folder;
        use crate::witness::ContentAddressed;
        let signal: Signal<Filesystem> = Signal {
            token: Folder {
                name: "test".into(),
                content: Some("hello".into()),
            },
        };
        let folder = Folder {
            name: "test".into(),
            content: Some("hello".into()),
        };
        assert_eq!(signal.content_oid(), folder.content_oid());
    }

    // -- Signature --

    #[test]
    fn signature_carries_identity_and_bytes() {
        let sig = Signature {
            signer: TestIdentity::new("Reed", None),
            signature: vec![0xDE, 0xAD],
        };
        assert_eq!(sig.signer.name().as_ref(), "Reed");
        assert_eq!(sig.signature, vec![0xDE, 0xAD]);
    }

    // -- Signed --

    #[test]
    fn signed_carries_signature_and_signal() {
        use crate::domain::filesystem::Folder;
        let signed: Signed<TestIdentity, Filesystem> = Signed {
            signature: Signature {
                signer: TestIdentity::new("Reed", None),
                signature: vec![0xCA, 0xFE],
            },
            signal: Signal {
                token: Folder {
                    name: "test".into(),
                    content: None,
                },
            },
        };
        assert_eq!(signed.signature.signer.name().as_ref(), "Reed");
        assert_eq!(signed.signature.signature, vec![0xCA, 0xFE]);
        assert_eq!(signed.signal.token.name, "test");
    }
}
