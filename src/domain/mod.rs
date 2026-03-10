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
}
