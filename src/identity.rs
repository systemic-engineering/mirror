use crate::domain::Context;

/// Identity: who you are in the system.
///
/// The trait for anything that can identify an actor.
/// Maps to git's author concept. An identity has a name,
/// an address, and keys for signing/encryption.
///
/// Identity is a first-class concept in Conversation.
/// Every gradient application is attributable to an identity.
pub trait Identity<C: Context> {
    fn name(&self) -> &str;
    fn email(&self) -> &str;
    fn keys(&self) -> &C::Keys;
}

/// An actor in the system. Carries identity, applies gradients.
///
/// Context-parameterized: `Actor<Filesystem>` carries `PlainKeys`,
/// a future `Actor<Encrypted>` could carry SSH/GPG keys.
/// Default context is Filesystem (PlainKeys, infallible).
///
/// Every observation in a session is attributable to an actor.
#[derive(Debug, Clone)]
pub struct Actor<C: Context = crate::domain::filesystem::Filesystem> {
    name: String,
    email: String,
    keys: C::Keys,
}

impl<C: Context> Actor<C> {
    pub fn new(name: impl Into<String>, email: impl Into<String>, keys: C::Keys) -> Self {
        Actor {
            name: name.into(),
            email: email.into(),
            keys,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn keys(&self) -> &C::Keys {
        &self.keys
    }
}

impl<C: Context> Identity<C> for Actor<C> {
    fn name(&self) -> &str {
        &self.name
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn keys(&self) -> &C::Keys {
        &self.keys
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::filesystem::Filesystem;
    use fragmentation::keys::PlainKeys;

    // -- New API: Actor<C> with keys --

    #[test]
    fn actor_filesystem_has_plain_keys() {
        let actor: Actor<Filesystem> = Actor::new("Reed", "reed@systemic.engineer", PlainKeys);
        assert_eq!(actor.name(), "Reed");
        assert_eq!(actor.email(), "reed@systemic.engineer");
        assert_eq!(*actor.keys(), PlainKeys);
    }

    #[test]
    fn actor_default_is_filesystem() {
        let actor: Actor = Actor::new("Reed", "reed@systemic.engineer", PlainKeys);
        assert_eq!(actor.name(), "Reed");
        assert_eq!(*actor.keys(), PlainKeys);
    }

    #[test]
    fn actor_conversation_domain() {
        use crate::domain::conversation::Conversation;
        let actor: Actor<Conversation> = Actor::new("Reed", "reed@systemic.engineer", PlainKeys);
        assert_eq!(actor.name(), "Reed");
        assert_eq!(*actor.keys(), PlainKeys);
    }

    #[test]
    fn identity_is_trait() {
        fn requires_identity<C: Context>(_: &impl Identity<C>) {}
        let actor: Actor<Filesystem> = Actor::new("Reed", "reed@systemic.engineer", PlainKeys);
        requires_identity(&actor);
    }

    // -- Old API: must break to prove Key is gone --

    #[test]
    fn actor_has_identity() {
        let actor = Actor::new("Reed", "reed@systemic.engineer");
        assert_eq!(actor.name(), "Reed");
        assert_eq!(actor.email(), "reed@systemic.engineer");
        assert_eq!(actor.key(), None);
    }

    #[test]
    fn actor_with_key() {
        let actor =
            Actor::new("Reed", "reed@systemic.engineer").with_key(Key::new("99060D23EBFAA0D4"));
        assert_eq!(actor.key().unwrap().fingerprint, "99060D23EBFAA0D4");
    }
}
