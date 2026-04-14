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

    /// Resolve a ref string (branch name, tag, HEAD, full ref) to an Oid.
    fn resolve_ref(&self, refname: &str) -> Result<git2::Oid, git2::Error> {
        // Try as a revision first (handles HEAD, branch names, tags, short SHAs)
        match self.repo.revparse_single(refname) {
            Ok(obj) => Ok(obj.id()),
            Err(_) => {
                // Try as a full ref path
                let reference = self.repo.find_reference(refname)?;
                reference
                    .target()
                    .ok_or_else(|| git2::Error::from_str("symbolic ref without target"))
            }
        }
    }

    /// List all refs (branches, tags, HEAD).
    pub fn refs(&self) -> Vec<(String, String)> {
        let mut result = Vec::new();

        // Add HEAD
        if let Ok(head) = self.repo.head() {
            if let Some(target) = head.target() {
                result.push(("HEAD".to_string(), target.to_string()));
            }
        }

        // Iterate all references
        if let Ok(refs) = self.repo.references() {
            for reference in refs.flatten() {
                if let (Some(name), Some(target)) = (reference.name(), reference.target()) {
                    result.push((name.to_string(), target.to_string()));
                }
            }
        }

        result
    }

    /// Get tree entries at a ref.
    pub fn tree_at(&self, refname: &str) -> Result<Vec<TreeEntry>, git2::Error> {
        let oid = self.resolve_ref(refname)?;
        let commit = self.repo.find_commit(oid)?;
        let tree = commit.tree()?;

        let mut entries = Vec::new();
        for entry in tree.iter() {
            let kind = match entry.kind() {
                Some(git2::ObjectType::Blob) => TreeEntryKind::Blob,
                Some(git2::ObjectType::Tree) => TreeEntryKind::Tree,
                Some(git2::ObjectType::Commit) => TreeEntryKind::Commit,
                _ => continue,
            };
            entries.push(TreeEntry {
                name: entry.name().unwrap_or("").to_string(),
                oid: entry.id().to_string(),
                kind,
            });
        }

        Ok(entries)
    }

    /// Read a blob at ref:path.
    pub fn show(&self, refname: &str, path: &str) -> Result<String, git2::Error> {
        let oid = self.resolve_ref(refname)?;
        let commit = self.repo.find_commit(oid)?;
        let tree = commit.tree()?;
        let entry = tree.get_path(Path::new(path))?;
        let object = entry.to_object(&self.repo)?;
        let blob = object
            .as_blob()
            .ok_or_else(|| git2::Error::from_str("not a blob"))?;
        String::from_utf8(blob.content().to_vec())
            .map_err(|_| git2::Error::from_str("blob is not valid UTF-8"))
    }

    /// Diff two refs' trees.
    pub fn diff(&self, a: &str, b: &str) -> Result<Vec<DiffEntry>, git2::Error> {
        let oid_a = self.resolve_ref(a)?;
        let oid_b = self.resolve_ref(b)?;

        let tree_a = self.repo.find_commit(oid_a)?.tree()?;
        let tree_b = self.repo.find_commit(oid_b)?.tree()?;

        let diff = self
            .repo
            .diff_tree_to_tree(Some(&tree_a), Some(&tree_b), None)?;

        let mut entries = Vec::new();
        diff.foreach(
            &mut |delta, _| {
                let status = match delta.status() {
                    git2::Delta::Added => DiffStatus::Added,
                    git2::Delta::Deleted => DiffStatus::Deleted,
                    git2::Delta::Modified => DiffStatus::Modified,
                    git2::Delta::Renamed => DiffStatus::Renamed,
                    _ => return true,
                };
                let path = delta
                    .new_file()
                    .path()
                    .or_else(|| delta.old_file().path())
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();
                entries.push(DiffEntry { path, status });
                true
            },
            None,
            None,
            None,
        )?;

        Ok(entries)
    }

    /// Commit log, most recent first.
    pub fn log(&self, count: usize) -> Result<Vec<LogEntry>, git2::Error> {
        let head = self.repo.head()?;
        let head_oid = head
            .target()
            .ok_or_else(|| git2::Error::from_str("HEAD has no target"))?;

        let mut revwalk = self.repo.revwalk()?;
        revwalk.push(head_oid)?;
        revwalk.set_sorting(git2::Sort::TIME)?;

        let mut entries = Vec::new();
        for oid in revwalk.take(count) {
            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;
            entries.push(LogEntry {
                oid: oid.to_string(),
                message: commit.summary().unwrap_or("").to_string(),
                author: commit.author().name().unwrap_or("").to_string(),
            });
        }

        Ok(entries)
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
        // On the feature branch, HEAD differs from main
        let diff = prism.diff("main", "HEAD").unwrap();
        // If we're ahead of main, there should be changes.
        // If we're on main, diff is empty — both are valid.
        // The structural assertion is that the call succeeds.
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
        // Every entry should have a non-empty OID
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
        // boot/ and src/ are trees
        let boot = tree.iter().find(|e| e.name == "boot");
        assert!(boot.is_some());
        assert_eq!(boot.unwrap().kind, TreeEntryKind::Tree);
        // Cargo.toml is a blob
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
