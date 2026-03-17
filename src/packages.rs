//! Package discovery. Find `.conv` packages from a directory.
//!
//! Two file patterns:
//!   `@name` (no extension) — content IS .conv source
//!   `name.conv` in a directory — directory package

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::resolve::{resolve_template, Namespace, TemplateProvider, TypeRegistry};

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
    ///
    /// Walks `root` recursively (following symlinks). Two patterns match:
    ///   - `@name` files (no extension): strip `@`, domain = "name"
    ///   - `name.conv` inside `name/` or `@name/`: domain = "name"
    ///
    /// First-found wins on duplicates.
    pub fn discover(root: &Path) -> Result<Self, String> {
        let mut packages = HashMap::new();
        walk_dir(root, &mut packages)?;
        Ok(PackageRegistry { packages })
    }

    /// Convert to a Namespace for the resolver.
    ///
    /// Parses each package's source, compiles grammars, extracts templates,
    /// and registers everything so `use @name` and `in @name` resolve.
    pub fn to_namespace(&self) -> Result<Namespace, String> {
        use crate::parse::Parse;
        use crate::Vector;

        let mut namespace = Namespace::new();

        for (name, package) in &self.packages {
            // Strip test section (below ---) before parsing.
            let spec = strip_tests(&package.source);

            let ast = Parse
                .trace(spec.to_string())
                .into_result()
                .map_err(|e| format!("@{}: {}", name, e.message))?;

            // Extract grammars
            for child in ast.children() {
                if child.data().is_decl("grammar") {
                    let registry = TypeRegistry::compile(child)
                        .map_err(|e| format!("@{}: {}", name, e.message))?;
                    let domain = registry.domain.clone();
                    namespace.register_grammar(&domain, registry);
                }
            }

            // Extract templates
            let mut templates = HashMap::new();
            for child in ast.children() {
                if child.data().is_decl("template") {
                    let tmpl_name = child.data().value.clone();
                    templates.insert(tmpl_name, resolve_template(child));
                }
            }

            // Register module (even if no templates, so is_known_domain works)
            namespace.register(name, TemplateProvider::Inline(templates));
        }

        Ok(namespace)
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

/// Walk a directory recursively, following symlinks.
fn walk_dir(dir: &Path, packages: &mut HashMap<String, Package>) -> Result<(), String> {
    let entries = std::fs::read_dir(dir).map_err(|e| format!("{}: {}", dir.display(), e))?;

    for entry in entries.flatten() {
        let path = entry.path();

        // Follow symlinks for type detection
        let metadata = match std::fs::metadata(&path) {
            Ok(m) => m,
            Err(_) => continue, // broken symlink or unreadable — skip
        };

        if metadata.is_dir() {
            walk_dir(&path, packages)?;
        } else if let Some(pkg) = try_file_package(&path) {
            packages.entry(pkg.name.clone()).or_insert(pkg);
        } else if let Some(pkg) = try_dir_package(&path) {
            packages.entry(pkg.name.clone()).or_insert(pkg);
        }
    }

    Ok(())
}

/// Try to match a file as `@name` (no extension) package.
fn try_file_package(path: &Path) -> Option<Package> {
    let file_name = path.file_name()?.to_str()?;
    if file_name.starts_with('@') && path.extension().is_none() {
        let name = file_name.strip_prefix('@')?.to_string();
        let source = std::fs::read_to_string(path).ok()?;
        Some(Package {
            name,
            source,
            path: path.to_path_buf(),
        })
    } else {
        None
    }
}

/// Try to match `name.conv` inside `name/` or `@name/` directory.
fn try_dir_package(path: &Path) -> Option<Package> {
    let ext = path.extension()?.to_str()?;
    if ext != "conv" {
        return None;
    }
    let stem = path.file_stem()?.to_str()?;
    let parent = path.parent()?;
    let parent_name = parent.file_name()?.to_str()?;
    let dir_name = parent_name.strip_prefix('@').unwrap_or(parent_name);
    if stem == dir_name {
        let source = std::fs::read_to_string(path).ok()?;
        Some(Package {
            name: stem.to_string(),
            source,
            path: path.to_path_buf(),
        })
    } else {
        None
    }
}

/// Strip test section (everything after `\n---\n`) from source.
fn strip_tests(source: &str) -> &str {
    if let Some(pos) = source.find("\n---\n") {
        &source[..pos]
    } else if let Some(pos) = source.find("\n---") {
        let rest = &source[pos + 4..];
        if rest.is_empty() || rest.chars().all(char::is_whitespace) {
            &source[..pos]
        } else {
            source
        }
    } else {
        source
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
        assert!(registry.is_empty());
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
    fn to_namespace_strips_test_section() {
        let dir = TempDir::new().unwrap();
        fs::write(
            dir.path().join("@beam"),
            "grammar @beam {\n  type = process\n}\n\n---\n\ntest \"t\" {\n  @beam has process\n}\n",
        )
        .unwrap();
        let registry = PackageRegistry::discover(dir.path()).unwrap();
        // Should not error — test section is stripped before parsing
        let namespace = registry.to_namespace().unwrap();
        assert!(namespace.contains("beam"));
    }

    #[test]
    fn strip_tests_preserves_source_without_separator() {
        assert_eq!(
            strip_tests("grammar @x { type = a }"),
            "grammar @x { type = a }"
        );
    }

    #[test]
    fn strip_tests_removes_test_section() {
        let source = "grammar @x { type = a }\n---\ntest { }";
        assert_eq!(strip_tests(source), "grammar @x { type = a }");
    }

    #[test]
    fn strip_tests_handles_trailing_separator() {
        let source = "grammar @x { type = a }\n---";
        assert_eq!(strip_tests(source), "grammar @x { type = a }");
    }

    #[test]
    fn strip_tests_handles_trailing_separator_with_whitespace() {
        let source = "grammar @x { type = a }\n---  \n";
        assert_eq!(strip_tests(source), "grammar @x { type = a }");
    }

    #[test]
    fn strip_tests_preserves_dashes_followed_by_text() {
        // \n--- followed by non-whitespace — not a separator
        let source = "grammar @x { type = a }\n---not a separator";
        assert_eq!(strip_tests(source), source);
    }

    #[test]
    fn discover_skips_broken_symlinks() {
        let dir = TempDir::new().unwrap();
        // Also add a valid package to prove discovery continues past the broken link
        fs::write(
            dir.path().join("@beam"),
            "grammar @beam {\n  type = process\n}\n",
        )
        .unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink("/nonexistent/target", dir.path().join("broken")).unwrap();
        #[cfg(not(unix))]
        return;
        let registry = PackageRegistry::discover(dir.path()).unwrap();
        assert_eq!(registry.len(), 1);
        assert!(registry.packages.contains_key("beam"));
    }

    #[test]
    fn to_namespace_parse_error() {
        let dir = TempDir::new().unwrap();
        // "unexpected:" line triggers ParseError in parse_source
        fs::write(dir.path().join("@bad"), ">>> not valid conv\n").unwrap();
        let registry = PackageRegistry::discover(dir.path()).unwrap();
        let result = registry.to_namespace();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("@bad"));
    }

    #[test]
    fn to_namespace_grammar_compile_error() {
        let dir = TempDir::new().unwrap();
        // Parameterized variant referencing a non-existent type → TypeRegistry error
        fs::write(
            dir.path().join("@bad"),
            "grammar @bad {\n  type = thing(missing)\n}\n",
        )
        .unwrap();
        let registry = PackageRegistry::discover(dir.path()).unwrap();
        let result = registry.to_namespace();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("@bad"));
    }

    #[test]
    fn discover_nonexistent_dir() {
        let result = PackageRegistry::discover(Path::new("/nonexistent/path/12345"));
        assert!(result.is_err());
    }

    #[test]
    fn packages_dir_default_and_env_override() {
        // Combined to avoid race condition — both tests mutate CONVERSATION_PACKAGES.
        std::env::set_var("CONVERSATION_PACKAGES", "/custom/path");
        let dir = PackageRegistry::packages_dir();
        assert_eq!(dir, PathBuf::from("/custom/path"));

        std::env::remove_var("CONVERSATION_PACKAGES");
        let dir = PackageRegistry::packages_dir();
        assert!(dir.ends_with(".conversation"));
    }
}
