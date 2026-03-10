use super::Context;
use crate::tree::{self, Tree};

/// The filesystem context.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Filesystem;

/// What a filesystem node can be.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Directory,
    File,
}

impl Context for Filesystem {
    type Token = Token;
    type Data = Folder;
    type Keys = fragmentation::keys::PlainKeys;

    fn id() -> &'static str {
        "filesystem"
    }
}

/// A filesystem entry as tree data.
///
/// Branch = directory (has children, no content).
/// Leaf = file (has content, no children).
#[derive(Clone, Debug)]
pub struct Folder {
    pub name: String,
    pub content: Option<String>,
}

impl Folder {
    /// Build a `Tree<Folder>` from a filesystem path.
    ///
    /// Directories become branches, files become leaves.
    pub fn read_tree(path: &str) -> Tree<Folder> {
        use fragmentation::ref_::Ref;
        use fragmentation::sha;

        let p = std::path::Path::new(path);
        let name = p
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        if p.is_dir() {
            let mut children: Vec<Tree<Folder>> = Vec::new();
            if let Ok(entries) = std::fs::read_dir(p) {
                let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
                entries.sort_by_key(|e| e.file_name());
                for entry in entries {
                    children.push(Folder::read_tree(entry.path().to_str().unwrap()));
                }
            }
            let ref_ = Ref::new(sha::hash(&name), &name);
            tree::branch(
                ref_,
                Folder {
                    name,
                    content: None,
                },
                children,
            )
        } else {
            let content = std::fs::read_to_string(p).ok();
            let ref_ = Ref::new(sha::hash(&name), &name);
            tree::leaf(ref_, Folder { name, content })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::Treelike;

    #[test]
    fn filesystem_id() {
        assert_eq!(Filesystem::id(), "filesystem");
    }

    #[test]
    fn filesystem_local_names() {
        assert_eq!(Filesystem::local_name(&Token::Directory), "Directory");
        assert_eq!(Filesystem::local_name(&Token::File), "File");
    }

    #[test]
    fn folder_read_tree_builds_structure() {
        let dir = tempfile::tempdir().unwrap();
        let sub = dir.path().join("child");
        std::fs::create_dir(&sub).unwrap();
        std::fs::write(sub.join("file.txt"), "hello").unwrap();

        let tree = Folder::read_tree(dir.path().to_str().unwrap());
        assert!(tree.is_fractal());
        assert_eq!(tree.children().len(), 1);
        let child = &tree.children()[0];
        assert_eq!(child.data().name, "child");
        assert!(child.is_fractal());
        assert_eq!(child.children().len(), 1);
        let file = &child.children()[0];
        assert_eq!(file.data().name, "file.txt");
        assert_eq!(file.data().content.as_deref(), Some("hello"));
    }

    #[test]
    fn folder_read_tree_nonexistent_path_produces_leaf() {
        let tree = Folder::read_tree("/nonexistent/path/that/does/not/exist");
        assert!(tree.is_shard());
        assert!(tree.data().content.is_none());
    }

    #[test]
    fn folder_read_tree_unreadable_dir_produces_empty_branch() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let restricted = dir.path().join("noperm");
        std::fs::create_dir(&restricted).unwrap();
        std::fs::write(restricted.join("file.txt"), "hidden").unwrap();
        std::fs::set_permissions(&restricted, std::fs::Permissions::from_mode(0o000)).unwrap();

        let tree = Folder::read_tree(restricted.to_str().unwrap());
        assert!(tree.is_fractal());
        assert_eq!(tree.children().len(), 0);

        // Restore permissions for cleanup
        std::fs::set_permissions(&restricted, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
}
