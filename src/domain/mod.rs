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
    fn local_name(kind: &Self::Language) -> Cow<'static, str>;
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
}
