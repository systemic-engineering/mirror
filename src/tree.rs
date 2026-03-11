use fragmentation::fragment::Fractal;

/// The primitive. Content-addressed, self-similar, arbitrary-depth.
/// Turtles all the way down: your children are yourself.
///
/// `Tree<E>` is `Fractal<E>` from fragmentation, re-exported as the
/// language primitive. A tree is either:
/// - a **leaf**: terminal, carries data, stops
/// - a **branch**: carries data, contains child trees
///
/// Every tree is content-addressed. Same content = same ref.
/// Git-compatible: leaves produce blob OIDs, branches produce tree OIDs.
pub type Tree<E = Vec<u8>> = Fractal<E>;

/// A leaf. Terminal node, carries data, no children.
pub fn leaf<E>(ref_: fragmentation::ref_::Ref, data: E) -> Tree<E> {
    Fractal::shard_typed(ref_, data)
}

/// A branch. Carries data, contains child trees.
pub fn branch<E>(ref_: fragmentation::ref_::Ref, data: E, children: Vec<Tree<E>>) -> Tree<E> {
    Fractal::new_typed(ref_, data, children)
}

/// Re-export the tree interface.
pub use fragmentation::fragment::Fragmentable as Treelike;

/// Re-export content addressing.
pub use fragmentation::fragment::content_oid;

use crate::witness::{ContentAddressed, Oid};

impl<E: fragmentation::encoding::Encode> ContentAddressed for Tree<E> {
    fn content_oid(&self) -> Oid {
        Oid::new(content_oid(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fragmentation::ref_::Ref;
    use fragmentation::sha;

    fn test_ref(label: &str) -> Ref {
        Ref::new(sha::hash(label), label)
    }

    #[test]
    fn leaf_is_terminal() {
        let t: Tree<String> = leaf(test_ref("a"), "hello".into());
        assert!(t.is_shard());
        assert!(!t.is_fractal());
        assert_eq!(t.data(), "hello");
    }

    #[test]
    fn branch_has_children() {
        let t: Tree<String> = branch(
            test_ref("root"),
            "parent".into(),
            vec![
                leaf(test_ref("a"), "left".into()),
                leaf(test_ref("b"), "right".into()),
            ],
        );
        assert!(t.is_fractal());
        assert_eq!(t.children().len(), 2);
    }

    #[test]
    fn tree_is_self_similar() {
        let child: Tree<String> = branch(
            test_ref("inner"),
            "subtree".into(),
            vec![leaf(test_ref("leaf"), "data".into())],
        );
        let root: Tree<String> = branch(test_ref("root"), "top".into(), vec![child]);
        assert!(root.is_fractal());
        assert!(root.children()[0].is_fractal());
        assert!(root.children()[0].children()[0].is_shard());
    }

    #[test]
    fn tree_content_addressed() {
        let a: Tree<String> = leaf(test_ref("x"), "same".into());
        let b: Tree<String> = leaf(test_ref("y"), "same".into());
        assert_eq!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn tree_different_content_different_oid() {
        let a: Tree<String> = leaf(test_ref("x"), "hello".into());
        let b: Tree<String> = leaf(test_ref("x"), "world".into());
        assert_ne!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn same_content_same_oid() {
        let a: Tree<String> = leaf(test_ref("x"), "same".into());
        let b: Tree<String> = leaf(test_ref("y"), "same".into());
        assert_eq!(content_oid(&a), content_oid(&b));
    }
}
