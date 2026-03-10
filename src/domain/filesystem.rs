use std::borrow::Cow;

use super::Domain;

/// The filesystem vocabulary.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Filesystem;

/// What a filesystem node can be.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Language {
    Directory,
    File,
}

impl Domain for Filesystem {
    type Language = Language;

    fn id() -> &'static str {
        "filesystem"
    }

    fn local_name(kind: &Language) -> Cow<'static, str> {
        match kind {
            Language::Directory => "dir".into(),
            Language::File => "file".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filesystem_id() {
        assert_eq!(Filesystem::id(), "filesystem");
    }

    #[test]
    fn filesystem_local_names() {
        assert_eq!(Filesystem::local_name(&Language::Directory), "dir");
        assert_eq!(Filesystem::local_name(&Language::File), "file");
    }
}
