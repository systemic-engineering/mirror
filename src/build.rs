//! Build: compile .conv source → git object storage.
//!
//! Parses and resolves a `.conv` source, emits EAF (ETF-encoded Erlang
//! Abstract Format), stores the result as git objects, and updates
//! `refs/fragmentation/build/<branch>`.

use crate::compile;
use crate::domain::filesystem::Filesystem;
use crate::resolve::Conversation;

/// Compile a `.conv` source and store the result in git.
///
/// Returns the git tree OID as a hex string on success.
///
/// The compiled EAF blob is stored under `<module_name>.eaf` in a git tree,
/// and `refs/fragmentation/build/<branch>` is updated to point at that tree.
pub fn build(source: &str, repo_path: &str) -> Result<String, String> {
    todo!("build not yet implemented")
}

fn current_branch(repo: &git2::Repository) -> String {
    repo.head()
        .ok()
        .and_then(|h| h.shorthand().map(String::from))
        .unwrap_or_else(|| "HEAD".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_compiles_conv_to_git_objects() {
        let dir = tempfile::tempdir().unwrap();
        let repo = git2::Repository::init(dir.path()).unwrap();

        // Create an initial commit so HEAD exists
        {
            let sig = git2::Signature::now("test", "test@test").unwrap();
            let tree_oid = repo.treebuilder(None).unwrap().write().unwrap();
            let tree = repo.find_tree(tree_oid).unwrap();
            repo.commit(Some("HEAD"), &sig, &sig, "init", &tree, &[])
                .unwrap();
        }

        let source = "in @filesystem\ntemplate $t {\n\tslug\n}\nout blog {\n\titems: sub { $t }\n}\n";
        let oid = build(source, dir.path().to_str().unwrap()).expect("build should succeed");

        // OID should be a valid hex SHA
        assert_eq!(oid.len(), 40, "OID should be 40 hex chars");
        assert!(oid.chars().all(|c| c.is_ascii_hexdigit()));

        // The ref should exist
        let ref_name = format!(
            "refs/fragmentation/build/{}",
            current_branch(&repo)
        );
        let reference = repo.find_reference(&ref_name).expect("ref should exist");
        let target = reference.target().expect("ref should have target");
        assert_eq!(target.to_string(), oid);

        // The tree should contain a .eaf entry
        let tree = repo.find_tree(target).expect("should be a tree");
        let entry = tree.get_name("blog.eaf").expect("should have blog.eaf entry");
        assert_eq!(entry.filemode(), 0o100644);
    }

    #[test]
    fn build_bad_source_returns_error() {
        let dir = tempfile::tempdir().unwrap();
        git2::Repository::init(dir.path()).unwrap();

        let result = build("garbage\n", dir.path().to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn current_branch_returns_main_for_init() {
        let dir = tempfile::tempdir().unwrap();
        let repo = git2::Repository::init(dir.path()).unwrap();

        // No HEAD yet → falls back to "HEAD"
        assert_eq!(current_branch(&repo), "HEAD");

        // Create initial commit on main
        let sig = git2::Signature::now("test", "test@test").unwrap();
        let tree_oid = repo.treebuilder(None).unwrap().write().unwrap();
        let tree = repo.find_tree(tree_oid).unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "init", &tree, &[])
            .unwrap();
        // Now HEAD points to a branch
        let branch = current_branch(&repo);
        assert!(!branch.is_empty());
    }
}
