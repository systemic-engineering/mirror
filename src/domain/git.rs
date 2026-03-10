//! The git domain. Repository structure as tree vocabulary.
//!
//! A git repository is a heterogeneous tree: refs point to commits,
//! commits contain trees, trees contain entries and blobs.
//! The discriminant IS the data — each level carries different information.

use super::Context;

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

impl Context for Git {
    type Token = GitNode;
    type Keys = fragmentation::keys::PlainKeys;

    fn id() -> &'static str {
        "git"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Context;
    use crate::tree::{self, Treelike};
    use fragmentation::ref_::Ref;
    use fragmentation::sha;

    fn test_ref(label: &str) -> Ref {
        Ref::new(sha::hash(label), label)
    }

    #[test]
    fn git_id() {
        assert_eq!(Git::id(), "git");
    }

    #[test]
    fn git_is_context() {
        fn requires_context<C: Context>() -> &'static str {
            C::id()
        }
        assert_eq!(requires_context::<Git>(), "git");
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
