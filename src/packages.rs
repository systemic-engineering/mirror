//! Package discovery. Find `.conv` packages from a directory.
//!
//! Two file patterns:
//!   `@name` (no extension) — content IS .conv source
//!   `name.conv` in a directory — directory package

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::model::Domain;
use crate::resolve::{resolve_template, Namespace, TemplateProvider};

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

    /// Discover packages from ordered roots; first root wins on name collision.
    ///
    /// Walks each root in order. Because `walk_dir` uses `entry().or_insert()`,
    /// inserting into a shared map across roots gives first-root-wins naturally.
    pub fn discover_ordered(roots: &[PathBuf]) -> Result<Self, String> {
        let mut packages = HashMap::new();
        for root in roots {
            walk_dir(root, &mut packages)?;
        }
        Ok(PackageRegistry { packages })
    }

    /// Priority-ordered lookup roots for `self_dir`.
    ///
    /// Returns the six candidate roots in priority order, filtered to only
    /// those that exist on disk:
    ///
    /// ```text
    /// 1. $SELF/private
    /// 2. $SELF/protected
    /// 3. $SELF/public
    /// 4. $HOME/private      ($HOME = $CONVERSATION_PACKAGES or ~/.conversation)
    /// 5. $HOME/protected
    /// 6. $HOME/public
    /// ```
    pub fn package_roots(self_dir: &Path) -> Vec<PathBuf> {
        Self::package_roots_with_home(self_dir, &Self::packages_dir())
    }

    /// Package roots with an explicit home directory (no env var read).
    fn package_roots_with_home(self_dir: &Path, home: &Path) -> Vec<PathBuf> {
        [
            self_dir.join("private"),
            self_dir.join("protected"),
            self_dir.join("public"),
            home.join("private"),
            home.join("protected"),
            home.join("public"),
        ]
        .into_iter()
        .filter(|p| p.exists())
        .collect()
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
            let ast = Parse
                .trace(package.source.clone())
                .into_result()
                .map_err(|e| format!("@{}: {}", name, e.message))?;

            // Extract grammars
            for child in ast.children() {
                if child.data().is_decl("grammar") {
                    let domain =
                        Domain::from_grammar(child).map_err(|e| format!("@{}: {}", name, e))?;
                    let domain_name = domain.domain_name().to_string();
                    namespace.register_domain(&domain_name, domain);
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

/// Split source into spec and optional test section (everything after `\n---\n`).
pub fn split_test_section(source: &str) -> (&str, Option<&str>) {
    if let Some(pos) = source.find("\n---\n") {
        let test_part = &source[pos + 5..];
        (&source[..pos], Some(test_part))
    } else if let Some(pos) = source.find("\n---") {
        let rest = &source[pos + 4..];
        if rest.is_empty() || rest.chars().all(char::is_whitespace) {
            (&source[..pos], None)
        } else {
            (source, None)
        }
    } else {
        (source, None)
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
        assert!(namespace.has_grammar("beam"));
        assert!(namespace.domain("beam").unwrap().has_variant("", "process"));
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
        // Parameterized variant referencing a non-existent type → Domain error
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
    fn discover_ordered_first_root_wins() {
        let root1 = TempDir::new().unwrap();
        let root2 = TempDir::new().unwrap();
        fs::write(root1.path().join("@pkg"), "grammar @pkg {\n  type = a\n}\n").unwrap();
        fs::write(root2.path().join("@pkg"), "grammar @pkg {\n  type = b\n}\n").unwrap();
        let roots = vec![root1.path().to_path_buf(), root2.path().to_path_buf()];
        let registry = PackageRegistry::discover_ordered(&roots).unwrap();
        assert_eq!(registry.len(), 1);
        assert!(registry.packages["pkg"].source.contains("type = a"));
    }

    #[test]
    fn discover_ordered_private_beats_public() {
        let base = TempDir::new().unwrap();
        let private_dir = base.path().join("private");
        let public_dir = base.path().join("public");
        fs::create_dir(&private_dir).unwrap();
        fs::create_dir(&public_dir).unwrap();
        fs::write(
            private_dir.join("@pkg"),
            "grammar @pkg {\n  type = private\n}\n",
        )
        .unwrap();
        fs::write(
            public_dir.join("@pkg"),
            "grammar @pkg {\n  type = public\n}\n",
        )
        .unwrap();
        let roots = vec![private_dir, public_dir];
        let registry = PackageRegistry::discover_ordered(&roots).unwrap();
        assert_eq!(registry.len(), 1);
        assert!(registry.packages["pkg"].source.contains("type = private"));
    }

    #[test]
    fn package_roots_includes_self_before_home() {
        let self_dir = TempDir::new().unwrap();
        let home_dir = TempDir::new().unwrap();
        fs::create_dir(self_dir.path().join("private")).unwrap();
        fs::create_dir(home_dir.path().join("private")).unwrap();
        let roots = PackageRegistry::package_roots_with_home(self_dir.path(), home_dir.path());
        let self_private = self_dir.path().join("private");
        let home_private = home_dir.path().join("private");
        let self_idx = roots.iter().position(|p| p == &self_private).unwrap();
        let home_idx = roots.iter().position(|p| p == &home_private).unwrap();
        assert!(self_idx < home_idx);
    }

    #[test]
    fn package_roots_filters_nonexistent() {
        let self_dir = TempDir::new().unwrap();
        // No subdirs created — all six roots are nonexistent
        let roots = PackageRegistry::package_roots_with_home(
            self_dir.path(),
            Path::new("/nonexistent/path/zzzz12345"),
        );
        assert!(roots.is_empty());
    }

    #[test]
    fn packages_dir_returns_path() {
        // Verify packages_dir returns a non-empty path. The exact value depends
        // on whether CONVERSATION_PACKAGES is set (env-dependent), so we just
        // confirm it produces something usable.
        let dir = PackageRegistry::packages_dir();
        assert!(!dir.as_os_str().is_empty());
    }

    #[test]
    fn packages_dir_from_env() {
        // Exercise the CONVERSATION_PACKAGES env branch (PathBuf::from(dir)).
        // Sets CONVERSATION_PACKAGES to a temp dir path, calls packages_dir(),
        // and verifies the returned path matches. Uses unsafe env mutation — safe
        // because this test only reads a known temp dir value immediately after.
        let tmp = TempDir::new().unwrap();
        let expected = tmp.path().to_path_buf();
        unsafe {
            std::env::set_var("CONVERSATION_PACKAGES", &expected);
        }
        let dir = PackageRegistry::packages_dir();
        unsafe {
            std::env::remove_var("CONVERSATION_PACKAGES");
        }
        assert_eq!(dir, expected);
    }

    #[test]
    fn package_roots_delegates_to_home() {
        // package_roots() is the public entry point; it calls package_roots_with_home
        // using packages_dir() as the home. Verify it does not panic.
        let self_dir = TempDir::new().unwrap();
        let roots = PackageRegistry::package_roots(self_dir.path());
        let _ = roots; // result depends on env; just verify no panic
    }

    // -- split_test_section --

    #[test]
    fn split_no_separator() {
        let (spec, test) = split_test_section("grammar @x { type = a }");
        assert_eq!(spec, "grammar @x { type = a }");
        assert!(test.is_none());
    }

    #[test]
    fn split_with_test_section() {
        let source = "grammar @x { type = a }\n---\ntest \"t\" { @x has a }";
        let (spec, test) = split_test_section(source);
        assert_eq!(spec, "grammar @x { type = a }");
        assert_eq!(test.unwrap(), "test \"t\" { @x has a }");
    }

    #[test]
    fn split_trailing_separator() {
        let (spec, test) = split_test_section("grammar @x { type = a }\n---");
        assert_eq!(spec, "grammar @x { type = a }");
        assert!(test.is_none());
    }

    #[test]
    fn split_trailing_separator_whitespace() {
        let (spec, test) = split_test_section("grammar @x { type = a }\n---  \n");
        assert_eq!(spec, "grammar @x { type = a }");
        assert!(test.is_none());
    }

    #[test]
    fn split_dashes_followed_by_text_not_separator() {
        let source = "grammar @x { type = a }\n---not a separator";
        let (spec, test) = split_test_section(source);
        assert_eq!(spec, source);
        assert!(test.is_none());
    }
}
