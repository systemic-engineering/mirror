pub mod conversation;
pub mod filesystem;

use std::borrow::Cow;

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

    fn id() -> &'static str;

    /// Human-readable name for a token variant.
    /// Default: Debug name of the variant.
    fn local_name(kind: &Self::Token) -> Cow<'static, str> {
        format!("{:?}", kind).into()
    }
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
    fn default_local_name_uses_debug() {
        assert_eq!(
            Filesystem::local_name(&filesystem::Token::Directory),
            "Directory"
        );
        assert_eq!(
            conversation::Conversation::local_name(&conversation::Token::TemplateRef),
            "TemplateRef"
        );
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
}
