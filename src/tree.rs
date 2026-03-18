//! Tree<E> is a type alias for Prism<E>.
//!
//! This module preserves backwards compatibility during the Prism migration.
//! `leaf()` → `shard()`, `branch()` → `fractal()`, `Tree::Leaf` → `Prism::Shard`,
//! `Tree::Branch` → `Prism::Fractal`.

use fragmentation::ref_::Ref;

use crate::prism::Prism;

/// Tree<E> is now a type alias for Prism<E>.
pub type Tree<E = Vec<u8>> = Prism<E>;

/// Re-export the tree interface.
pub use fragmentation::fragment::Fragmentable as Treelike;

/// Re-export content addressing.
pub use fragmentation::fragment::content_oid;

/// A leaf. Terminal node — delegates to prism::shard().
pub fn leaf<E>(ref_: Ref, data: E) -> Tree<E> {
    crate::prism::shard(ref_, data)
}

/// A branch. Carries data, contains children — delegates to prism::fractal().
pub fn branch<E>(ref_: Ref, data: E, children: Vec<Tree<E>>) -> Tree<E> {
    crate::prism::fractal(ref_, data, children)
}

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

    /// Leaf maps to Prism::Shard, Branch maps to Prism::Fractal.
    #[test]
    fn tree_alias_maps_to_prism_variants() {
        let l: Tree<String> = leaf(test_ref("a"), "data".into());
        assert!(matches!(l, Prism::Shard { .. }));

        let b: Tree<String> = branch(test_ref("b"), "root".into(), vec![l]);
        assert!(matches!(b, Prism::Fractal { .. }));
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
