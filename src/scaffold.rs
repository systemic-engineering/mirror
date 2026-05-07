//! scaffold — codegen for the @new grammar.
//!
//! This module is the Rust implementation of the `io` actions
//! declared in `boot/std/new.mirror`. When mirror compiles io
//! actions natively, this entire file is generated, not written.
//!
//! Layer 3: codegen. NOT business logic. The grammar is the truth.
//! See: boot/std/new.mirror, boot/std/new.template.mirror

use std::path::Path;

use crate::git_store::MirrorGitStore;

#[derive(Debug)]
pub struct ScaffoldError(pub String);

impl std::fmt::Display for ScaffoldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ScaffoldError {}

/// Codegen target for `@new.new(name, root)`.
///
/// Each step maps to an `io` action in boot/std/new.mirror.
/// When mirror compiles io → native, this function is generated.
pub fn scaffold_project(root: &Path, name: &str) -> Result<(), ScaffoldError> {
    // Guard: @new grammar's implicit precondition
    let spec_path = root.join("mirror.spec");
    if spec_path.exists() {
        return Err(ScaffoldError(format!(
            "{} already contains a mirror.spec",
            root.display()
        )));
    }

    // io mkdir(root)
    std::fs::create_dir_all(root)
        .map_err(|e| ScaffoldError(format!("cannot create {}: {}", root.display(), e)))?;

    // io write(root, "mirror.spec", template(name))
    std::fs::write(&spec_path, render_template(name))
        .map_err(|e| ScaffoldError(format!("cannot write mirror.spec: {}", e)))?;

    // io mkdir(root, "mirror")
    let mirror_dir = root.join("mirror");
    std::fs::create_dir_all(&mirror_dir)
        .map_err(|e| ScaffoldError(format!("cannot create mirror/: {}", e)))?;

    // io write(root, "mirror/.gitkeep", "")
    std::fs::write(mirror_dir.join(".gitkeep"), "")
        .map_err(|e| ScaffoldError(format!("cannot write .gitkeep: {}", e)))?;

    // io write(root, ".gitignore", gitignore)
    std::fs::write(root.join(".gitignore"), GITIGNORE)
        .map_err(|e| ScaffoldError(format!("cannot write .gitignore: {}", e)))?;

    // zoom git_init(root)
    git2::Repository::init(root)
        .map_err(|e| ScaffoldError(format!("git init failed: {}", e)))?;

    // refract store_init(root)
    MirrorGitStore::open(root)
        .map_err(|e| ScaffoldError(format!("mirror store init failed: {}", e)))?;

    Ok(())
}

/// Codegen for `@new_template.spec(name)`.
/// Source: boot/std/new.template.mirror
fn render_template(name: &str) -> String {
    format!(
        "\
@{name}

store {{
  path = .git/mirror
}}

craft {{
  default mirror/*.mirror
}}

run {{
  default mirror/*.mirror
}}

kintsugi {{
  --hoist
  --sort-deps
  --normalize
  --align
  naming = snake_case
  indent = 2
}}

properties {{
  requires {{
    types_lowercase
    unique_variants
    every_type_reachable
    no_dead_variants
  }}
  invariant {{
    deterministic
    pure
    no_cycles
  }}
  ensures {{
    always_halts
  }}
}}
"
    )
}

/// Codegen for `@new.gitignore` template.
/// Source: boot/std/new.mirror
const GITIGNORE: &str = "\
# mirror artifacts
*.shatter
*.shatter.sig

# build
target/
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_creates_spec_file() {
        let dir = tempfile::tempdir().unwrap();
        let project_dir = dir.path().join("my-project");
        scaffold_project(&project_dir, "my-project").unwrap();
        assert!(project_dir.join("mirror.spec").exists());
    }

    #[test]
    fn scaffold_spec_starts_with_identity() {
        let dir = tempfile::tempdir().unwrap();
        let project_dir = dir.path().join("my-project");
        scaffold_project(&project_dir, "my-project").unwrap();
        let content = std::fs::read_to_string(project_dir.join("mirror.spec")).unwrap();
        assert!(content.starts_with("@my-project\n"));
    }

    #[test]
    fn scaffold_spec_has_run_block() {
        let dir = tempfile::tempdir().unwrap();
        let project_dir = dir.path().join("my-project");
        scaffold_project(&project_dir, "my-project").unwrap();
        let content = std::fs::read_to_string(project_dir.join("mirror.spec")).unwrap();
        assert!(content.contains("run {"));
        assert!(content.contains("default mirror/*.mirror"));
    }

    #[test]
    fn scaffold_creates_mirror_dir() {
        let dir = tempfile::tempdir().unwrap();
        let project_dir = dir.path().join("my-project");
        scaffold_project(&project_dir, "my-project").unwrap();
        assert!(project_dir.join("mirror").is_dir());
        assert!(project_dir.join("mirror/.gitkeep").exists());
    }

    #[test]
    fn scaffold_creates_gitignore() {
        let dir = tempfile::tempdir().unwrap();
        let project_dir = dir.path().join("my-project");
        scaffold_project(&project_dir, "my-project").unwrap();
        let content = std::fs::read_to_string(project_dir.join(".gitignore")).unwrap();
        assert!(content.contains(".shatter"));
    }

    #[test]
    fn scaffold_inits_git_repo() {
        let dir = tempfile::tempdir().unwrap();
        let project_dir = dir.path().join("my-project");
        scaffold_project(&project_dir, "my-project").unwrap();
        assert!(project_dir.join(".git").exists());
    }

    #[test]
    fn scaffold_inits_mirror_store() {
        let dir = tempfile::tempdir().unwrap();
        let project_dir = dir.path().join("my-project");
        scaffold_project(&project_dir, "my-project").unwrap();
        assert!(project_dir.join(".git/mirror").exists());
    }

    #[test]
    fn scaffold_spec_roundtrips() {
        let dir = tempfile::tempdir().unwrap();
        let project_dir = dir.path().join("roundtrip");
        scaffold_project(&project_dir, "roundtrip").unwrap();
        let content = std::fs::read_to_string(project_dir.join("mirror.spec")).unwrap();
        let spec = crate::spec::parse_spec_source(&content).unwrap();
        assert_eq!(spec.oid, "@roundtrip");
        assert!(!spec.craft.default.is_empty());
        assert!(!spec.run.default.is_empty());
        assert!(!spec.properties.requires.is_empty());
    }

    #[test]
    fn scaffold_refuses_existing_spec() {
        let dir = tempfile::tempdir().unwrap();
        let project_dir = dir.path().join("existing");
        std::fs::create_dir_all(&project_dir).unwrap();
        std::fs::write(project_dir.join("mirror.spec"), "existing").unwrap();
        assert!(scaffold_project(&project_dir, "existing").is_err());
    }
}
