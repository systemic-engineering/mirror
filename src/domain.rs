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

/// The filesystem vocabulary.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Filesystem;

/// What a filesystem node can be.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FsLanguage {
    Directory,
    File,
}

impl Domain for Filesystem {
    type Language = FsLanguage;

    fn id() -> &'static str {
        "filesystem"
    }

    fn local_name(kind: &FsLanguage) -> Cow<'static, str> {
        match kind {
            FsLanguage::Directory => "dir".into(),
            FsLanguage::File => "file".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- Domain trait --

    #[test]
    fn domain_is_trait() {
        fn requires_domain<D: Domain>() -> &'static str {
            D::id()
        }
        requires_domain::<Filesystem>();
    }

    // -- Filesystem domain --

    #[test]
    fn filesystem_id() {
        assert_eq!(Filesystem::id(), "filesystem");
    }

    #[test]
    fn filesystem_local_names() {
        assert_eq!(Filesystem::local_name(&FsLanguage::Directory), "dir");
        assert_eq!(Filesystem::local_name(&FsLanguage::File), "file");
    }
}
