use sha2::{Digest, Sha256};

use super::{Addressable, Setting};
use crate::narrative::ContentAddressed;
use crate::tree::{self, Tree};

story::domain_oid!(/// Content address for filesystem nodes.
pub FolderOid);

/// The filesystem context.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Filesystem;

impl fragmentation::encoding::Encode for Folder {
    fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"folder:");
        bytes.extend_from_slice(self.name.as_bytes());
        if let Some(content) = &self.content {
            bytes.extend_from_slice(b":");
            bytes.extend_from_slice(content.as_bytes());
        }
        bytes
    }
}

impl ContentAddressed for Folder {
    type Oid = FolderOid;
    fn content_oid(&self) -> FolderOid {
        let mut hasher = Sha256::new();
        hasher.update(b"folder:");
        hasher.update(self.name.as_bytes());
        if let Some(content) = &self.content {
            hasher.update(b":");
            hasher.update(content.as_bytes());
        }
        FolderOid::new(hex::encode(hasher.finalize()))
    }
}

impl Addressable for Folder {
    fn node_name(&self) -> &str {
        &self.name
    }
    fn node_content(&self) -> Option<&str> {
        self.content.as_deref()
    }
}

impl Setting for Filesystem {
    type Token = Folder;

    fn id() -> &'static str {
        "filesystem"
    }
}

/// A filesystem entry as tree data.
///
/// Branch = directory (has children, no content).
/// Leaf = file (has content, no children).
#[derive(Clone, Debug, PartialEq, Eq)]
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
    fn folder_content_addressed() {
        let a = Folder {
            name: "test".into(),
            content: Some("hello".into()),
        };
        let b = Folder {
            name: "test".into(),
            content: Some("hello".into()),
        };
        assert_eq!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn folder_different_content_different_oid() {
        let a = Folder {
            name: "test".into(),
            content: Some("hello".into()),
        };
        let b = Folder {
            name: "test".into(),
            content: Some("world".into()),
        };
        assert_ne!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn folder_dir_vs_file_different_oid() {
        let dir = Folder {
            name: "test".into(),
            content: None,
        };
        let file = Folder {
            name: "test".into(),
            content: Some("".into()),
        };
        assert_ne!(dir.content_oid(), file.content_oid());
    }

    #[test]
    fn filesystem_id() {
        assert_eq!(Filesystem::id(), "filesystem");
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

    // -- Encode --

    #[test]
    fn folder_encode_with_content() {
        use fragmentation::encoding::Encode;
        let f = Folder {
            name: "test".into(),
            content: Some("hello".into()),
        };
        let bytes = f.encode();
        assert_eq!(bytes, b"folder:test:hello");
    }

    #[test]
    fn folder_encode_without_content() {
        use fragmentation::encoding::Encode;
        let f = Folder {
            name: "dir".into(),
            content: None,
        };
        let bytes = f.encode();
        assert_eq!(bytes, b"folder:dir");
    }
}
