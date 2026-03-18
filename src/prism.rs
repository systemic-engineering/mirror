use fragmentation::encoding::Encode;
use fragmentation::fragment::Fragmentable;
use fragmentation::ref_::Ref;
use fragmentation::sha::Sha;

/// The primitive. Content-addressed, self-similar, arbitrary-depth.
/// Extends fragmentation's 3-variant Fractal with a 4th: Optics.
///
/// - **Shard**: terminal, carries data, stops (= Tree::Leaf)
/// - **Fractal**: carries data, contains children (= Tree::Branch)
/// - **Lens**: carries data, references external trees by OID
/// - **Optics**: carries data, has both children and external references
///
/// Every prism is content-addressed. Same content = same ref.
/// Git-compatible: shards produce blob OIDs, fractals produce tree OIDs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Prism<V> {
    /// Terminal: carries data, no children, no targets.
    Shard { ref_: Ref, data: V },
    /// Self-similar: carries data, contains child prisms.
    Fractal {
        ref_: Ref,
        data: V,
        children: Vec<Prism<V>>,
    },
    /// Lens: carries data, references external trees by OID. Edges, not containment.
    Lens {
        ref_: Ref,
        data: V,
        targets: Vec<Sha>,
    },
    /// Optics: carries data, has both children and external references.
    Optics {
        ref_: Ref,
        data: V,
        targets: Vec<Sha>,
        children: Vec<Prism<V>>,
    },
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

    // -- Constructors --

    #[test]
    fn shard_constructor() {
        let p: Prism<String> = shard(test_ref("a"), "hello".into());
        assert!(matches!(p, Prism::Shard { .. }));
        assert_eq!(p.data(), "hello");
    }

    #[test]
    fn fractal_constructor() {
        let child = shard(test_ref("c"), "child".into());
        let p: Prism<String> = fractal(test_ref("root"), "parent".into(), vec![child]);
        assert!(matches!(p, Prism::Fractal { .. }));
        assert_eq!(p.children().len(), 1);
    }

    #[test]
    fn lens_constructor() {
        let target = Sha("abc123".into());
        let p: Prism<String> = lens(test_ref("l"), "data".into(), vec![target.clone()]);
        assert!(matches!(p, Prism::Lens { .. }));
        assert_eq!(p.targets(), &[target]);
    }

    #[test]
    fn optics_constructor() {
        let child = shard(test_ref("c"), "child".into());
        let target = Sha("abc123".into());
        let p: Prism<String> =
            optics(test_ref("o"), "data".into(), vec![target.clone()], vec![child]);
        assert!(matches!(p, Prism::Optics { .. }));
        assert_eq!(p.children().len(), 1);
        assert_eq!(p.targets(), &[target]);
    }

    // -- Accessors --

    #[test]
    fn shard_accessors() {
        let r = test_ref("a");
        let p: Prism<String> = shard(r.clone(), "hello".into());
        assert_eq!(p.self_ref(), &r);
        assert_eq!(p.data(), "hello");
        assert!(p.children().is_empty());
        assert!(p.targets().is_empty());
    }

    #[test]
    fn fractal_accessors() {
        let r = test_ref("root");
        let child = shard(test_ref("c"), "child".into());
        let p: Prism<String> = fractal(r.clone(), "parent".into(), vec![child]);
        assert_eq!(p.self_ref(), &r);
        assert_eq!(p.data(), "parent");
        assert_eq!(p.children().len(), 1);
        assert!(p.targets().is_empty());
    }

    #[test]
    fn lens_accessors() {
        let r = test_ref("l");
        let target = Sha("abc".into());
        let p: Prism<String> = lens(r.clone(), "data".into(), vec![target.clone()]);
        assert_eq!(p.self_ref(), &r);
        assert_eq!(p.data(), "data");
        assert!(p.children().is_empty());
        assert_eq!(p.targets(), &[target]);
    }

    #[test]
    fn optics_accessors() {
        let r = test_ref("o");
        let child = shard(test_ref("c"), "child".into());
        let target = Sha("abc".into());
        let p: Prism<String> =
            optics(r.clone(), "data".into(), vec![target.clone()], vec![child]);
        assert_eq!(p.self_ref(), &r);
        assert_eq!(p.data(), "data");
        assert_eq!(p.children().len(), 1);
        assert_eq!(p.targets(), &[target]);
    }

    // -- Fragmentable predicates --

    #[test]
    fn shard_is_shard() {
        let p: Prism<String> = shard(test_ref("a"), "data".into());
        assert!(p.is_shard());
        assert!(!p.is_fractal());
        assert!(!p.is_lens());
    }

    #[test]
    fn fractal_is_fractal() {
        let p: Prism<String> =
            fractal(test_ref("r"), "data".into(), vec![shard(test_ref("c"), "c".into())]);
        assert!(!p.is_shard());
        assert!(p.is_fractal());
        assert!(!p.is_lens());
    }

    #[test]
    fn lens_is_lens() {
        let p: Prism<String> = lens(test_ref("l"), "data".into(), vec![Sha("t".into())]);
        assert!(!p.is_shard());
        assert!(!p.is_fractal());
        assert!(p.is_lens());
    }

    #[test]
    fn optics_predicates() {
        let p: Prism<String> = optics(
            test_ref("o"),
            "data".into(),
            vec![Sha("t".into())],
            vec![shard(test_ref("c"), "c".into())],
        );
        // Optics is none of the three — it's its own thing
        assert!(!p.is_shard());
        assert!(!p.is_fractal());
        assert!(!p.is_lens());
    }

    // -- Self-similarity --

    #[test]
    fn prism_is_self_similar() {
        let child: Prism<String> = fractal(
            test_ref("inner"),
            "subtree".into(),
            vec![shard(test_ref("leaf"), "data".into())],
        );
        let root: Prism<String> = fractal(test_ref("root"), "top".into(), vec![child]);
        assert!(root.is_fractal());
        assert!(root.children()[0].is_fractal());
        assert!(root.children()[0].children()[0].is_shard());
    }

    // -- ContentAddressed --

    #[test]
    fn prism_content_addressed() {
        let a: Prism<String> = shard(test_ref("x"), "same".into());
        let b: Prism<String> = shard(test_ref("y"), "same".into());
        assert_eq!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn prism_different_content_different_oid() {
        let a: Prism<String> = shard(test_ref("x"), "hello".into());
        let b: Prism<String> = shard(test_ref("x"), "world".into());
        assert_ne!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn prism_content_oid_via_function() {
        let a: Prism<String> = shard(test_ref("x"), "same".into());
        let b: Prism<String> = shard(test_ref("y"), "same".into());
        assert_eq!(content_oid(&a), content_oid(&b));
    }

    // -- OID stability sentinels: Prism ↔ Tree --

    #[test]
    fn oid_stability_shard_equals_leaf() {
        use crate::tree;
        let data = "sentinel-data".to_string();
        let prism_node: Prism<String> = shard(test_ref("x"), data.clone());
        let tree_node: crate::tree::Tree<String> = tree::leaf(test_ref("y"), data);
        assert_eq!(prism_node.content_oid(), tree_node.content_oid());
    }

    #[test]
    fn oid_stability_fractal_equals_branch() {
        use crate::tree;
        let child_data = "child-data".to_string();
        let parent_data = "parent-data".to_string();

        let prism_child: Prism<String> = shard(test_ref("c1"), child_data.clone());
        let prism_parent: Prism<String> =
            fractal(test_ref("p1"), parent_data.clone(), vec![prism_child]);

        let tree_child = tree::leaf(test_ref("c2"), child_data);
        let tree_parent = tree::branch(test_ref("p2"), parent_data, vec![tree_child]);

        assert_eq!(prism_parent.content_oid(), tree_parent.content_oid());
    }

    // -- Store integration --

    #[test]
    fn prism_works_with_store() {
        use fragmentation::repo::Repo;
        use fragmentation::store::Store;

        let p: Prism<String> = fractal(
            test_ref("root"),
            "parent".into(),
            vec![shard(test_ref("a"), "child".into())],
        );
        let mut store = Store::<Prism<String>>::new();
        let oid = store.write_tree(&p);
        assert_eq!(store.read_tree(&oid), Some(p));
    }

    // -- Fragmentable trait coverage --

    fn assert_fragmentable<F: Treelike>(f: &F, expect_shard: bool, child_count: usize) {
        assert_eq!(Treelike::is_shard(f), expect_shard);
        let _ = Treelike::self_ref(f);
        let _ = Treelike::data(f);
        assert_eq!(Treelike::children(f).len(), child_count);
    }

    #[test]
    fn fragmentable_trait_coverage() {
        let child: Prism<String> = shard(test_ref("c"), "child".into());
        let parent: Prism<String> = fractal(test_ref("p"), "parent".into(), vec![child.clone()]);
        let l: Prism<String> = lens(test_ref("l"), "lens".into(), vec![Sha("t".into())]);
        let o: Prism<String> = optics(
            test_ref("o"),
            "optics".into(),
            vec![Sha("t".into())],
            vec![shard(test_ref("oc"), "oc".into())],
        );

        assert_fragmentable(&child, true, 0);
        assert_fragmentable(&parent, false, 1);
        assert_fragmentable(&l, false, 0);
        assert_fragmentable(&o, false, 1);

        // content_oid through Fragmentable
        assert_eq!(content_oid(&parent), content_oid(&parent));
        assert_eq!(content_oid(&l), content_oid(&l));
        assert_eq!(content_oid(&o), content_oid(&o));
    }

    // -- Lens and Optics OID isolation --

    #[test]
    fn lens_oid_includes_targets() {
        let a: Prism<String> =
            lens(test_ref("l"), "data".into(), vec![Sha("target1".into())]);
        let b: Prism<String> =
            lens(test_ref("l"), "data".into(), vec![Sha("target2".into())]);
        // Different targets → different OID
        assert_ne!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn optics_oid_includes_children() {
        let a: Prism<String> = optics(
            test_ref("o"),
            "data".into(),
            vec![Sha("t".into())],
            vec![shard(test_ref("c1"), "child1".into())],
        );
        let b: Prism<String> = optics(
            test_ref("o"),
            "data".into(),
            vec![Sha("t".into())],
            vec![shard(test_ref("c2"), "child2".into())],
        );
        // Different children → different OID
        assert_ne!(a.content_oid(), b.content_oid());
    }
}
