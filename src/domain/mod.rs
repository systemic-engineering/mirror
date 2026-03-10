pub mod conversation;
pub mod filesystem;
pub mod git;

/// What `@` addresses. The domain a `.conv` file operates in.
///
/// Known domains resolve without registration.
/// External domains require explicit registration via `Resolve::with_domain`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Domain {
    Filesystem,
    Json,
    Git,
    External(String),
}

impl Domain {
    /// The string name this domain is addressed by.
    pub fn id(&self) -> &str {
        match self {
            Domain::Filesystem => "filesystem",
            Domain::Json => "json",
            Domain::Git => "git",
            Domain::External(name) => name,
        }
    }

    /// Resolve a name to a known domain variant. Returns None for unknown names.
    pub fn from_name(name: &str) -> Option<Domain> {
        match name {
            "filesystem" => Some(Domain::Filesystem),
            "json" => Some(Domain::Json),
            "git" => Some(Domain::Git),
            _ => None,
        }
    }

    /// The names of all known (built-in) domains.
    pub fn known_names() -> &'static [&'static str] {
        &["filesystem", "json", "git"]
    }
}

/// The tree's context. Defines the domain's token vocabulary.
///
/// A Context names what nodes in a tree can mean.
/// `@filesystem`: directories and files.
/// `@html`: articles, sections, headings.
/// `@document`: sections, paragraphs, code blocks.
///
/// The context makes a tree interpretable.
/// Crossing between domains is a Gradient.
pub trait Context: Clone + std::fmt::Debug + PartialEq + Eq {
    type Token: Clone + std::fmt::Debug + PartialEq + Eq;
    type Keys: fragmentation::keys::Keys;

    fn id() -> &'static str;
}

/// What a tree node must provide for conversation execution.
///
/// The conversation program navigates domain trees by name
/// and extracts content from leaf nodes. This is the interface
/// between the resolved program and the domain's tree structure.
pub trait Addressable {
    fn node_name(&self) -> &str;
    fn node_content(&self) -> Option<&str>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use filesystem::Filesystem;

    #[test]
    fn context_is_trait() {
        fn requires_context<C: Context>() -> &'static str {
            C::id()
        }
        requires_context::<Filesystem>();
    }

    #[test]
    fn filesystem_token_is_folder() {
        fn assert_token<C: Context<Token = filesystem::Folder>>() {}
        assert_token::<Filesystem>();
    }

    #[test]
    fn filesystem_keys_is_plain() {
        fn assert_keys_type<C: Context<Keys = fragmentation::keys::PlainKeys>>() {}
        assert_keys_type::<Filesystem>();
    }

    #[test]
    fn conversation_token_is_ast_node() {
        fn assert_token<C: Context<Token = crate::ast::AstNode>>() {}
        assert_token::<conversation::Conversation>();
    }

    #[test]
    fn conversation_keys_is_plain() {
        fn assert_keys_type<C: Context<Keys = fragmentation::keys::PlainKeys>>() {}
        assert_keys_type::<conversation::Conversation>();
    }

    // -- Domain enum --

    #[test]
    fn domain_filesystem_id() {
        assert_eq!(Domain::Filesystem.id(), "filesystem");
    }

    #[test]
    fn domain_json_id() {
        assert_eq!(Domain::Json.id(), "json");
    }

    #[test]
    fn domain_external_id() {
        assert_eq!(Domain::External("html".into()).id(), "html");
    }

    #[test]
    fn domain_from_str_known() {
        assert_eq!(Domain::from_name("filesystem"), Some(Domain::Filesystem));
        assert_eq!(Domain::from_name("json"), Some(Domain::Json));
    }

    #[test]
    fn domain_from_str_unknown() {
        assert_eq!(Domain::from_name("html"), None);
    }

    #[test]
    fn domain_eq() {
        assert_eq!(Domain::Filesystem, Domain::Filesystem);
        assert_eq!(Domain::External("x".into()), Domain::External("x".into()));
        assert_ne!(Domain::Filesystem, Domain::Json);
        assert_ne!(Domain::External("a".into()), Domain::External("b".into()));
    }

    #[test]
    fn domain_known_names() {
        let names = Domain::known_names();
        assert!(names.contains(&"filesystem"));
        assert!(names.contains(&"json"));
    }

    // -- Git domain --

    #[test]
    fn git_token_is_git_node() {
        fn assert_token<C: Context<Token = git::GitNode>>() {}
        assert_token::<git::Git>();
    }

    #[test]
    fn git_keys_is_plain() {
        fn assert_keys<C: Context<Keys = fragmentation::keys::PlainKeys>>() {}
        assert_keys::<git::Git>();
    }

    #[test]
    fn domain_git_id() {
        assert_eq!(Domain::Git.id(), "git");
    }

    #[test]
    fn domain_from_name_git() {
        assert_eq!(Domain::from_name("git"), Some(Domain::Git));
    }

    #[test]
    fn domain_known_names_includes_git() {
        let names = Domain::known_names();
        assert!(names.contains(&"git"));
    }
}
