pub mod conversation;
pub mod filesystem;

use std::borrow::Cow;

/// The tree's vocabulary. Defines the domain's language.
///
/// A Domain names what nodes in a tree can mean.
/// `@filesystem`: directories and files.
/// `@html`: articles, sections, headings.
/// `@document`: sections, paragraphs, code blocks.
///
/// The domain makes a tree interpretable.
/// Crossing between domains is a Gradient.
pub trait Domain: Clone + std::fmt::Debug + PartialEq + Eq {
    type Language: Clone + std::fmt::Debug + PartialEq + Eq;

    fn id() -> &'static str;

    /// Human-readable name for a language variant.
    /// Default: Debug name of the variant.
    fn local_name(kind: &Self::Language) -> Cow<'static, str> {
        format!("{:?}", kind).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use filesystem::Filesystem;

    #[test]
    fn domain_is_trait() {
        fn requires_domain<D: Domain>() -> &'static str {
            D::id()
        }
        requires_domain::<Filesystem>();
    }

    #[test]
    fn default_local_name_uses_debug() {
        // Neither domain overrides local_name — both use the default
        assert_eq!(
            Filesystem::local_name(&filesystem::Language::Directory),
            "Directory"
        );
        assert_eq!(
            conversation::Conversation::local_name(&conversation::Language::TemplateRef),
            "TemplateRef"
        );
    }
}
