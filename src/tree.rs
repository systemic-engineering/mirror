use fragmentation::encoding::Encode;
use fragmentation::fragment::Fragmentable;
use fragmentation::ref_::Ref;

/// The primitive. Content-addressed, self-similar, arbitrary-depth.
/// Turtles all the way down: your children are yourself.
///
/// A tree is either:
/// - a **leaf**: terminal, carries data, stops
/// - a **branch**: carries data, contains child trees
///
/// Every tree is content-addressed. Same content = same ref.
/// Git-compatible: leaves produce blob OIDs, branches produce tree OIDs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tree<E = Vec<u8>> {
    /// Terminal: carries data, no children.
    Leaf { ref_: Ref, data: E },
    /// Self-similar: carries data, contains child trees.
    Branch {
        ref_: Ref,
        data: E,
        children: Vec<Tree<E>>,
    },
}

impl<E> Tree<E> {
    /// The node's ref.
    pub fn self_ref(&self) -> &Ref {
        match self {
            Tree::Leaf { ref_, .. } => ref_,
            Tree::Branch { ref_, .. } => ref_,
        }
    }

    /// The node's data.
    pub fn data(&self) -> &E {
        match self {
            Tree::Leaf { data, .. } => data,
            Tree::Branch { data, .. } => data,
        }
    }

    /// The node's children. Empty for leaves.
    pub fn children(&self) -> &[Tree<E>] {
        match self {
            Tree::Leaf { .. } => &[],
            Tree::Branch { children, .. } => children,
        }
    }

    /// True if this is a leaf (terminal, no children).
    pub fn is_shard(&self) -> bool {
        matches!(self, Tree::Leaf { .. })
    }

    /// True if this is a branch (has children).
    pub fn is_fractal(&self) -> bool {
        matches!(self, Tree::Branch { .. })
    }
}

impl<E: Encode> Fragmentable for Tree<E> {
    type Data = E;

    fn self_ref(&self) -> &Ref {
        self.self_ref()
    }

    fn data(&self) -> &E {
        self.data()
    }

    fn children(&self) -> &[Tree<E>] {
        self.children()
    }

    fn is_shard(&self) -> bool {
        self.is_shard()
    }

    fn is_fractal(&self) -> bool {
        self.is_fractal()
    }
}

/// A leaf. Terminal node, carries data, no children.
pub fn leaf<E>(ref_: Ref, data: E) -> Tree<E> {
    Tree::Leaf { ref_, data }
}

/// A branch. Carries data, contains child trees.
pub fn branch<E>(ref_: Ref, data: E, children: Vec<Tree<E>>) -> Tree<E> {
    Tree::Branch {
        ref_,
        data,
        children,
    }
}

/// Re-export the tree interface.
pub use fragmentation::fragment::Fragmentable as Treelike;

/// Re-export content addressing.
pub use fragmentation::fragment::content_oid;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ContentAddressed;
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

    /// Tree is its own enum (Leaf/Branch), not a Fractal alias.
    /// Pattern matching on Tree::Leaf/Tree::Branch must work.
    #[test]
    fn tree_is_own_enum() {
        let l: Tree<String> = leaf(test_ref("a"), "data".into());
        assert!(matches!(l, Tree::Leaf { .. }));

        let b: Tree<String> = branch(test_ref("b"), "root".into(), vec![l]);
        assert!(matches!(b, Tree::Branch { .. }));
    }

    /// Tree works with Store — it's a first-class Fragmentable.
    #[test]
    fn tree_works_with_store() {
        use fragmentation::repo::Repo;
        use fragmentation::store::Store;

        let t: Tree<String> = branch(
            test_ref("root"),
            "parent".into(),
            vec![leaf(test_ref("a"), "child".into())],
        );
        let mut store = Store::<Tree<String>>::new();
        let oid = store.write_tree(&t);
        assert_eq!(store.read_tree(&oid), Some(t));
    }

    /// Exercise all Fragmentable trait methods on both leaf and branch.
    fn assert_fragmentable<F: Treelike>(f: &F, expect_shard: bool, child_count: usize) {
        assert_eq!(Treelike::is_shard(f), expect_shard);
        assert_eq!(Treelike::is_fractal(f), !expect_shard);
        let _ = Treelike::self_ref(f);
        let _ = Treelike::data(f);
        assert_eq!(Treelike::children(f).len(), child_count);
    }

    #[test]
    fn fragmentable_trait_coverage() {
        let child: Tree<String> = leaf(test_ref("c"), "child".into());
        let parent: Tree<String> = branch(test_ref("p"), "parent".into(), vec![child.clone()]);

        assert_fragmentable(&child, true, 0);
        assert_fragmentable(&parent, false, 1);

        // content_oid through Fragmentable (exercises branch path)
        assert_eq!(content_oid(&parent), content_oid(&parent));
    }
}
