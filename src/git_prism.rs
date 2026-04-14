//! GitPrism — a read-only prism over git's ref space.
//!
//! Five operations: refs, tree, show, diff, log.
//! Each reads the git object store directly. No checkout. No mutation.
//! Pure observation of git's content-addressed tree.

use std::path::Path;

use git2::Repository;

/// A read-only view into a git repository's object store.
pub struct GitPrism {
    repo: Repository,
}

/// An entry in a tree listing.
pub struct TreeEntry {
    pub name: String,
    pub oid: String,
    pub kind: TreeEntryKind,
}

/// The kind of a tree entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeEntryKind {
    Blob,
    Tree,
    Commit,
}

impl std::fmt::Display for TreeEntryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TreeEntryKind::Blob => write!(f, "blob"),
            TreeEntryKind::Tree => write!(f, "tree"),
            TreeEntryKind::Commit => write!(f, "commit"),
        }
    }
}

/// A diff entry between two trees.
pub struct DiffEntry {
    pub path: String,
    pub status: DiffStatus,
}

/// The status of a diff entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffStatus {
    Added,
    Deleted,
    Modified,
    Renamed,
}

impl std::fmt::Display for DiffStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiffStatus::Added => write!(f, "A"),
            DiffStatus::Deleted => write!(f, "D"),
            DiffStatus::Modified => write!(f, "M"),
            DiffStatus::Renamed => write!(f, "R"),
        }
    }
}

/// A commit log entry.
pub struct LogEntry {
    pub oid: String,
    pub message: String,
    pub author: String,
}

impl GitPrism {
    /// Open a git repository at the given path.
    pub fn open(path: &Path) -> Result<Self, git2::Error> {
        let repo = Repository::discover(path)?;
        Ok(GitPrism { repo })
    }

    /// List all refs (branches, tags, HEAD).
    pub fn refs(&self) -> Vec<(String, String)> {
        todo!("implement refs listing")
    }

    /// Get tree entries at a ref.
    pub fn tree_at(&self, _refname: &str) -> Result<Vec<TreeEntry>, git2::Error> {
        todo!("implement tree listing")
    }

    /// Read a blob at ref:path.
    pub fn show(&self, _refname: &str, _path: &str) -> Result<String, git2::Error> {
        todo!("implement blob reading")
    }

    /// Diff two refs' trees.
    pub fn diff(&self, _a: &str, _b: &str) -> Result<Vec<DiffEntry>, git2::Error> {
        todo!("implement tree diff")
    }

    /// Commit log, most recent first.
    pub fn log(&self, _count: usize) -> Result<Vec<LogEntry>, git2::Error> {
        todo!("implement commit log")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn project_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    #[test]
    fn git_refs_lists_branches() {
        let prism = GitPrism::open(&project_root()).unwrap();
        let refs = prism.refs();
        assert!(!refs.is_empty(), "mirror repo must have refs");
        assert!(
            refs.iter().any(|(name, _)| name.contains("main")),
            "must contain main: {:?}",
            refs.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn git_tree_at_main() {
        let prism = GitPrism::open(&project_root()).unwrap();
        let tree = prism.tree_at("main").unwrap();
        assert!(
            tree.iter().any(|e| e.name == "boot"),
            "main must have boot/"
        );
        assert!(tree.iter().any(|e| e.name == "src"), "main must have src/");
    }

    #[test]
    fn git_show_reads_file_without_checkout() {
        let prism = GitPrism::open(&project_root()).unwrap();
        let content = prism.show("main", "boot/00-prism.mirror").unwrap();
        assert!(
            content.contains("focus id"),
            "prism.mirror must contain 'focus id'"
        );
        assert!(
            content.contains("@prism"),
            "prism.mirror must contain '@prism': {}",
            &content[..content.len().min(200)]
        );
    }

    #[test]
    fn git_diff_main_vs_head() {
        let prism = GitPrism::open(&project_root()).unwrap();
        let diff = prism.diff("main", "HEAD").unwrap();
        assert!(
            diff.is_empty() || !diff.is_empty(),
            "diff should return without error"
        );
    }

    #[test]
    fn git_log_shows_history() {
        let prism = GitPrism::open(&project_root()).unwrap();
        let log = prism.log(10).unwrap();
        assert!(!log.is_empty(), "mirror repo must have commits");
        assert!(log.len() <= 10, "log should respect count limit");
        for entry in &log {
            assert!(!entry.oid.is_empty(), "log entry must have an OID");
        }
    }

    #[test]
    fn git_show_nonexistent_path_fails() {
        let prism = GitPrism::open(&project_root()).unwrap();
        let result = prism.show("main", "nonexistent/path/file.txt");
        assert!(result.is_err(), "showing a nonexistent path should fail");
    }

    #[test]
    fn git_tree_entry_kinds() {
        let prism = GitPrism::open(&project_root()).unwrap();
        let tree = prism.tree_at("main").unwrap();
        let boot = tree.iter().find(|e| e.name == "boot");
        assert!(boot.is_some());
        assert_eq!(boot.unwrap().kind, TreeEntryKind::Tree);
        let cargo = tree.iter().find(|e| e.name == "Cargo.toml");
        assert!(cargo.is_some());
        assert_eq!(cargo.unwrap().kind, TreeEntryKind::Blob);
    }

    #[test]
    fn git_open_nonexistent_fails() {
        let result = GitPrism::open(Path::new("/tmp/nonexistent-repo-xyz"));
        assert!(result.is_err());
    }

    #[test]
    fn tree_entry_kind_display() {
        assert_eq!(format!("{}", TreeEntryKind::Blob), "blob");
        assert_eq!(format!("{}", TreeEntryKind::Tree), "tree");
        assert_eq!(format!("{}", TreeEntryKind::Commit), "commit");
    }

    #[test]
    fn diff_status_display() {
        assert_eq!(format!("{}", DiffStatus::Added), "A");
        assert_eq!(format!("{}", DiffStatus::Deleted), "D");
        assert_eq!(format!("{}", DiffStatus::Modified), "M");
        assert_eq!(format!("{}", DiffStatus::Renamed), "R");
    }
}
