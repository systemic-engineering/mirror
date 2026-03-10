use crate::domain::Context;

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
}
