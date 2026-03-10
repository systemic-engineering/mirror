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
        assert_eq!(Filesystem::local_name(&Language::Directory), "Directory");
        assert_eq!(Filesystem::local_name(&Language::File), "File");
    }
}
