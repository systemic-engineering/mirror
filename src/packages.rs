//! Package discovery. Find `.conv` packages from a directory.
//!
//! Two file patterns:
//!   `@name` (no extension) — content IS .conv source
//!   `name.conv` in a directory — directory package

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::resolve::{Namespace, TypeRegistry};

/// A discovered package.
#[derive(Clone, Debug)]
pub struct Package {
    pub name: String,
    pub source: String,
    pub path: PathBuf,
}

/// Registry of discovered packages.
#[derive(Clone, Debug)]
pub struct PackageRegistry {
    packages: HashMap<String, Package>,
}

impl PackageRegistry {
    /// Discover packages from a directory.
    pub fn discover(_root: &Path) -> Result<Self, String> {
        Ok(PackageRegistry {
            packages: HashMap::new(),
        })
    }

    /// Convert to a Namespace for the resolver.
    pub fn to_namespace(&self) -> Result<Namespace, String> {
        Ok(Namespace::new())
    }

    /// Number of discovered packages.
    pub fn len(&self) -> usize {
        self.packages.len()
    }

    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.packages.is_empty()
    }

    /// Package directory from env or default.
    pub fn packages_dir() -> PathBuf {
        if let Ok(dir) = std::env::var("CONVERSATION_PACKAGES") {
            PathBuf::from(dir)
        } else {
            dirs_home().join(".conversation")
        }
    }
}

fn dirs_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn discover_empty_dir() {
        let dir = TempDir::new().unwrap();
        let registry = PackageRegistry::discover(dir.path()).unwrap();
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn discover_file_package() {
        let dir = TempDir::new().unwrap();
        fs::write(
            dir.path().join("@beam"),
            "grammar @beam {\n  type = process | supervision | module\n}\n",
        )
        .unwrap();
        let registry = PackageRegistry::discover(dir.path()).unwrap();
        assert_eq!(registry.len(), 1);
        assert!(registry.packages.contains_key("beam"));
        assert_eq!(registry.packages["beam"].name, "beam");
    }

    #[test]
    fn discover_dir_package() {
        let dir = TempDir::new().unwrap();
        let git_dir = dir.path().join("@git");
        fs::create_dir(&git_dir).unwrap();
        fs::write(
            git_dir.join("git.conv"),
            "grammar @git {\n  type = ref | commit | entry | blob\n}\n",
        )
        .unwrap();
        let registry = PackageRegistry::discover(dir.path()).unwrap();
        assert_eq!(registry.len(), 1);
        assert!(registry.packages.contains_key("git"));
    }

    #[test]
    fn discover_dir_package_without_at_prefix() {
        let dir = TempDir::new().unwrap();
        let glue_dir = dir.path().join("glue");
        fs::create_dir(&glue_dir).unwrap();
        fs::write(
            glue_dir.join("glue.conv"),
            "grammar @glue {\n  type = session\n}\n",
        )
        .unwrap();
        let registry = PackageRegistry::discover(dir.path()).unwrap();
        assert_eq!(registry.len(), 1);
        assert!(registry.packages.contains_key("glue"));
    }

    #[test]
    fn discover_follows_symlinks() {
        let dir = TempDir::new().unwrap();
        let actual = TempDir::new().unwrap();
        fs::write(
            actual.path().join("glue.conv"),
            "grammar @glue {\n  type = session\n}\n",
        )
        .unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(actual.path(), dir.path().join("glue")).unwrap();
        #[cfg(not(unix))]
        return; // symlinks need unix

        let registry = PackageRegistry::discover(dir.path()).unwrap();
        assert_eq!(registry.len(), 1);
        assert!(registry.packages.contains_key("glue"));
    }

    #[test]
    fn discover_skips_non_conv() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("README.md"), "# not a package").unwrap();
        fs::write(dir.path().join("main.conv"), "grammar @x { type = a }").unwrap();
        fs::write(
            dir.path().join("@beam"),
            "grammar @beam {\n  type = process\n}\n",
        )
        .unwrap();
        let registry = PackageRegistry::discover(dir.path()).unwrap();
        // Should find @beam but not README.md or main.conv (not @-prefixed file)
        assert_eq!(registry.len(), 1);
        assert!(registry.packages.contains_key("beam"));
    }

    #[test]
    fn discover_nested_directory() {
        let dir = TempDir::new().unwrap();
        let public = dir.path().join("public");
        fs::create_dir(&public).unwrap();
        fs::write(
            public.join("@beam"),
            "grammar @beam {\n  type = process\n}\n",
        )
        .unwrap();
        let registry = PackageRegistry::discover(dir.path()).unwrap();
        assert_eq!(registry.len(), 1);
        assert!(registry.packages.contains_key("beam"));
    }

    #[test]
    fn discover_first_wins_on_duplicate() {
        let dir = TempDir::new().unwrap();
        // Two locations for @beam — first found wins
        fs::write(
            dir.path().join("@beam"),
            "grammar @beam {\n  type = process\n}\n",
        )
        .unwrap();
        let sub = dir.path().join("sub");
        fs::create_dir(&sub).unwrap();
        fs::write(sub.join("@beam"), "grammar @beam {\n  type = module\n}\n").unwrap();
        let registry = PackageRegistry::discover(dir.path()).unwrap();
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn to_namespace_registers_grammars() {
        let dir = TempDir::new().unwrap();
        fs::write(
            dir.path().join("@beam"),
            "grammar @beam {\n  type = process | supervision | module\n}\n",
        )
        .unwrap();
        let registry = PackageRegistry::discover(dir.path()).unwrap();
        let namespace = registry.to_namespace().unwrap();
        assert!(namespace.contains("beam"));
        let grammars = namespace.grammars();
        assert!(grammars.contains_key("beam"));
        assert!(grammars["beam"].has_variant("", "process"));
    }

    #[test]
    fn to_namespace_extracts_templates() {
        let dir = TempDir::new().unwrap();
        fs::write(
            dir.path().join("@mail"),
            "grammar @mail {\n  type = message\n}\n\ntemplate $message(@imap) {\n  subject\n  body: article\n}\n",
        )
        .unwrap();
        let registry = PackageRegistry::discover(dir.path()).unwrap();
        let namespace = registry.to_namespace().unwrap();
        let templates = namespace.get_templates("mail").unwrap();
        assert!(templates.contains_key("$message"));
    }

    #[test]
    fn packages_dir_default() {
        // Clear env to test default
        std::env::remove_var("CONVERSATION_PACKAGES");
        let dir = PackageRegistry::packages_dir();
        assert!(dir.ends_with(".conversation"));
    }

    #[test]
    fn packages_dir_env_override() {
        std::env::set_var("CONVERSATION_PACKAGES", "/custom/path");
        let dir = PackageRegistry::packages_dir();
        assert_eq!(dir, PathBuf::from("/custom/path"));
        std::env::remove_var("CONVERSATION_PACKAGES");
    }
}
