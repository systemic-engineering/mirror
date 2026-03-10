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

    // -- System<C>: composable node --

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

    #[test]
    fn identity_trait_methods() {
        fn check<C: Context>(id: &impl Identity<C>) -> &str {
            id.name()
        }
        let system: System<Filesystem> = System::new("Reed", PlainKeys);
        let name = check(&system);
        assert_eq!(name, "Reed");
    }

    #[test]
    fn identity_trait_keys() {
        fn get_keys<C: Context>(id: &impl Identity<C>) -> &C::Keys {
            id.keys()
        }
        let system: System<Filesystem> = System::new("Reed", PlainKeys);
        assert_eq!(*get_keys(&system), PlainKeys);
    }
}
