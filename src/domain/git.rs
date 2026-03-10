//! The git domain. Repository structure as tree vocabulary.
//!
//! A git repository is a heterogeneous tree: refs point to commits,
//! commits contain trees, trees contain entries and blobs.
//! The discriminant IS the data — each level carries different information.

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
        if let GitNode::Ref { name, target } = &node {
            assert_eq!(name, "main");
            assert_eq!(target, "abc123");
        } else {
            panic!("expected Ref");
        }
    }

    #[test]
    fn git_node_commit_variant() {
        let node = GitNode::Commit {
            message: "initial".into(),
            author: "Reed".into(),
            email: "reed@systemic.engineer".into(),
        };
        if let GitNode::Commit {
            message,
            author,
            email,
        } = &node
        {
            assert_eq!(message, "initial");
            assert_eq!(author, "Reed");
            assert_eq!(email, "reed@systemic.engineer");
        } else {
            panic!("expected Commit");
        }
    }

    #[test]
    fn git_node_entry_variant() {
        let node = GitNode::Entry {
            name: "src".into(),
        };
        if let GitNode::Entry { name } = &node {
            assert_eq!(name, "src");
        } else {
            panic!("expected Entry");
        }
    }

    #[test]
    fn git_node_blob_variant() {
        let node = GitNode::Blob {
            content: b"hello".to_vec(),
        };
        if let GitNode::Blob { content } = &node {
            assert_eq!(content, b"hello");
        } else {
            panic!("expected Blob");
        }
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
            GitNode::Entry {
                name: "src".into(),
            },
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
