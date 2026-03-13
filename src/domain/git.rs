//! The git domain. Repository structure as tree vocabulary.
//!
//! A git repository is a heterogeneous tree: refs point to commits,
//! commits contain trees, trees contain entries and blobs.
//! The discriminant IS the data — each level carries different information.

use sha2::{Digest, Sha256};

use super::{Addressable, Setting};
use crate::ContentAddressed;

story::domain_oid!(/// Content address for git nodes.
pub GitOid);

/// The git context.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Git;

/// A node in a git tree. Heterogeneous — each variant carries
/// different data for its level in the repository structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GitNode {
    /// A named reference pointing to a commit.
    Ref { name: String, target: String },
    /// A commit with message and author.
    Commit {
        message: String,
        author: String,
        email: String,
    },
    /// A tree entry (directory-like).
    Entry { name: String },
    /// Raw content.
    Blob { content: Vec<u8> },
}

impl fragmentation::encoding::Encode for GitNode {
    fn encode(&self) -> Vec<u8> {
        match self {
            GitNode::Ref { name, target } => format!("ref:{}:{}", name, target).into_bytes(),
            GitNode::Commit {
                message,
                author,
                email,
            } => format!("commit:{}:{}:{}", message, author, email).into_bytes(),
            GitNode::Entry { name } => format!("entry:{}", name).into_bytes(),
            GitNode::Blob { content } => {
                let mut bytes = b"blob:".to_vec();
                bytes.extend_from_slice(content);
                bytes
            }
        }
    }
}

impl ContentAddressed for GitNode {
    type Oid = GitOid;
    fn content_oid(&self) -> GitOid {
        let mut hasher = Sha256::new();
        match self {
            GitNode::Ref { name, target } => {
                hasher.update(b"ref:");
                hasher.update(name.as_bytes());
                hasher.update(b":");
                hasher.update(target.as_bytes());
            }
            GitNode::Commit {
                message,
                author,
                email,
            } => {
                hasher.update(b"commit:");
                hasher.update(message.as_bytes());
                hasher.update(b":");
                hasher.update(author.as_bytes());
                hasher.update(b":");
                hasher.update(email.as_bytes());
            }
            GitNode::Entry { name } => {
                hasher.update(b"entry:");
                hasher.update(name.as_bytes());
            }
            GitNode::Blob { content } => {
                hasher.update(b"blob:");
                hasher.update(content);
            }
        }
        GitOid::new(hex::encode(hasher.finalize()))
    }
}

impl Addressable for GitNode {
    fn node_name(&self) -> &str {
        match self {
            GitNode::Ref { name, .. } => name,
            GitNode::Commit { message, .. } => message,
            GitNode::Entry { name } => name,
            GitNode::Blob { .. } => "",
        }
    }
    fn node_content(&self) -> Option<&str> {
        match self {
            GitNode::Blob { content } => std::str::from_utf8(content).ok(),
            _ => None,
        }
    }
}

impl Setting for Git {
    type Token = GitNode;

    fn id() -> &'static str {
        "git"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Addressable, Setting};
    use crate::tree::{self, Treelike};
    use fragmentation::ref_::Ref;
    use fragmentation::sha;

    fn test_ref(label: &str) -> Ref {
        Ref::new(sha::hash(label), label)
    }

    // -- ContentAddressed --

    #[test]
    fn git_node_content_addressed() {
        let a = GitNode::Entry { name: "src".into() };
        let b = GitNode::Entry { name: "src".into() };
        assert_eq!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn git_node_ref_content_addressed() {
        let a = GitNode::Ref {
            name: "main".into(),
            target: "abc".into(),
        };
        let b = GitNode::Ref {
            name: "main".into(),
            target: "abc".into(),
        };
        assert_eq!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn git_node_commit_content_addressed() {
        let a = GitNode::Commit {
            message: "init".into(),
            author: "Reed".into(),
            email: "reed@systemic.engineer".into(),
        };
        let b = GitNode::Commit {
            message: "init".into(),
            author: "Reed".into(),
            email: "reed@systemic.engineer".into(),
        };
        assert_eq!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn git_node_blob_content_addressed() {
        let a = GitNode::Blob {
            content: b"hello".to_vec(),
        };
        let b = GitNode::Blob {
            content: b"hello".to_vec(),
        };
        assert_eq!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn git_node_different_variant_different_oid() {
        let a = GitNode::Entry { name: "src".into() };
        let b = GitNode::Blob {
            content: b"src".to_vec(),
        };
        assert_ne!(a.content_oid(), b.content_oid());
    }

    // -- Addressable --

    #[test]
    fn addressable_node_name() {
        assert_eq!(
            GitNode::Ref {
                name: "main".into(),
                target: "abc".into()
            }
            .node_name(),
            "main"
        );
        assert_eq!(
            GitNode::Commit {
                message: "init".into(),
                author: "a".into(),
                email: "e".into()
            }
            .node_name(),
            "init"
        );
        assert_eq!(GitNode::Entry { name: "src".into() }.node_name(), "src");
        assert_eq!(
            GitNode::Blob {
                content: b"x".to_vec()
            }
            .node_name(),
            ""
        );
    }

    #[test]
    fn addressable_node_content() {
        assert_eq!(
            GitNode::Blob {
                content: b"hello".to_vec()
            }
            .node_content(),
            Some("hello")
        );
        assert_eq!(GitNode::Entry { name: "src".into() }.node_content(), None);
        // Invalid UTF-8 returns None
        assert_eq!(
            GitNode::Blob {
                content: vec![0xFF, 0xFE]
            }
            .node_content(),
            None
        );
    }

    #[test]
    fn git_id() {
        assert_eq!(Git::id(), "git");
    }

    #[test]
    fn git_is_scene() {
        fn requires_scene<C: Setting>() -> &'static str {
            C::id()
        }
        assert_eq!(requires_scene::<Git>(), "git");
    }

    #[test]
    fn git_node_ref_variant() {
        let node = GitNode::Ref {
            name: "main".into(),
            target: "abc123".into(),
        };
        assert_eq!(
            node,
            GitNode::Ref {
                name: "main".into(),
                target: "abc123".into()
            }
        );
        assert_eq!(node.clone(), node);
    }

    #[test]
    fn git_node_commit_variant() {
        let node = GitNode::Commit {
            message: "initial".into(),
            author: "Reed".into(),
            email: "reed@systemic.engineer".into(),
        };
        assert_eq!(
            node,
            GitNode::Commit {
                message: "initial".into(),
                author: "Reed".into(),
                email: "reed@systemic.engineer".into()
            }
        );
    }

    #[test]
    fn git_node_entry_variant() {
        let node = GitNode::Entry { name: "src".into() };
        assert_eq!(node, GitNode::Entry { name: "src".into() });
    }

    #[test]
    fn git_node_blob_variant() {
        let node = GitNode::Blob {
            content: b"hello".to_vec(),
        };
        assert_eq!(
            node,
            GitNode::Blob {
                content: b"hello".to_vec()
            }
        );
    }

    // -- Encode --

    #[test]
    fn git_node_encode_ref() {
        use fragmentation::encoding::Encode;
        let node = GitNode::Ref {
            name: "main".into(),
            target: "abc".into(),
        };
        assert_eq!(node.encode(), b"ref:main:abc");
    }

    #[test]
    fn git_node_encode_commit() {
        use fragmentation::encoding::Encode;
        let node = GitNode::Commit {
            message: "init".into(),
            author: "Reed".into(),
            email: "reed@systemic.engineer".into(),
        };
        assert_eq!(node.encode(), b"commit:init:Reed:reed@systemic.engineer");
    }

    #[test]
    fn git_node_encode_entry() {
        use fragmentation::encoding::Encode;
        let node = GitNode::Entry { name: "src".into() };
        assert_eq!(node.encode(), b"entry:src");
    }

    #[test]
    fn git_node_encode_blob() {
        use fragmentation::encoding::Encode;
        let node = GitNode::Blob {
            content: b"hello".to_vec(),
        };
        assert_eq!(node.encode(), b"blob:hello");
    }

    #[test]
    fn git_tree_heterogeneous() {
        // Build a ref → commit → entry → blob tree.
        // Proves heterogeneous nodes work in Tree<GitNode>.
        let blob = tree::leaf(
            test_ref("file.rs"),
            GitNode::Blob {
                content: b"fn main() {}".to_vec(),
            },
        );
        let entry = tree::branch(
            test_ref("src"),
            GitNode::Entry { name: "src".into() },
            vec![blob],
        );
        let commit = tree::branch(
            test_ref("abc123"),
            GitNode::Commit {
                message: "initial".into(),
                author: "Reed".into(),
                email: "reed@systemic.engineer".into(),
            },
            vec![entry],
        );
        let ref_node = tree::branch(
            test_ref("refs/heads/main"),
            GitNode::Ref {
                name: "main".into(),
                target: "abc123".into(),
            },
            vec![commit],
        );

        // Verify structure
        assert!(ref_node.is_fractal());
        let commit_node = &ref_node.children()[0];
        assert!(commit_node.is_fractal());
        let entry_node = &commit_node.children()[0];
        assert!(entry_node.is_fractal());
        let blob_node = &entry_node.children()[0];
        assert!(blob_node.is_shard());

        // Verify data at each level
        assert!(matches!(ref_node.data(), GitNode::Ref { .. }));
        assert!(matches!(commit_node.data(), GitNode::Commit { .. }));
        assert!(matches!(entry_node.data(), GitNode::Entry { .. }));
        assert!(matches!(blob_node.data(), GitNode::Blob { .. }));
    }
}
