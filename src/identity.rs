/// Identity: who you are in the system.
///
/// The trait for anything that can identify an actor.
/// Maps to git's author concept. An identity has a name,
/// an address, and optionally a key for signing.
///
/// Identity is a first-class concept in Conversation.
/// Every gradient application is attributable to an identity.
pub trait Identity {
    fn name(&self) -> &str;
    fn email(&self) -> &str;
    fn key(&self) -> Option<&Key>;
}

/// A cryptographic key. First-class concept.
///
/// Used for signing events, proving provenance.
/// Maps to GPG keys in git commits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Key {
    /// Key fingerprint (e.g. GPG fingerprint)
    pub fingerprint: String,
}

impl Key {
    pub fn new(fingerprint: impl Into<String>) -> Self {
        Key {
            fingerprint: fingerprint.into(),
        }
    }
}

/// An actor in the system. Carries identity, applies gradients.
///
/// In Witness (Elixir), Actor is a GenServer per module.
/// In Conversation, Actor is a value that identifies who is
/// applying transformations. Actor is a first-class concept.
///
/// Every observation in a session is attributable to an actor.
#[derive(Debug, Clone)]
pub struct Actor {
    name: String,
    email: String,
    key: Option<Key>,
}

impl Actor {
    pub fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
        Actor {
            name: name.into(),
            email: email.into(),
            key: None,
        }
    }

    pub fn with_key(mut self, key: Key) -> Self {
        self.key = Some(key);
        self
    }
}

impl Identity for Actor {
    fn name(&self) -> &str {
        &self.name
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn key(&self) -> Option<&Key> {
        self.key.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn identity_is_trait() {
        fn requires_identity(_: &impl Identity) {}
        let actor = Actor::new("Reed", "reed@systemic.engineer");
        requires_identity(&actor);
    }

    #[test]
    fn same_actor_different_instance() {
        let a = Actor::new("Reed", "reed@systemic.engineer");
        let b = Actor::new("Reed", "reed@systemic.engineer");
        assert_eq!(a.name(), b.name());
        assert_eq!(a.email(), b.email());
    }
}
