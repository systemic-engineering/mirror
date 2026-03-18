use conversation::prism;
use conversation::{Conversation, Filesystem, Folder, OutputNode, Prism, Repo, Store, Vector};
use fragmentation::commit::{Commit, Draft};
use fragmentation::encoding;
use fragmentation::fragment::{content_oid, Fractal};
use fragmentation::ref_::Ref;
use fragmentation::sha;
use fragmentation::witnessed::Committer;

fn test_ref(label: &str) -> Ref {
    Ref::new(sha::hash(label), label)
}

fn leaf_folder(name: &str, content: &str) -> Prism<Folder> {
    prism::shard(
        test_ref(name),
        Folder {
            name: name.into(),
            content: Some(content.into()),
        },
    )
}

fn dir_folder(name: &str, children: Vec<Prism<Folder>>) -> Prism<Folder> {
    prism::fractal(
        test_ref(name),
        Folder {
            name: name.into(),
            content: None,
        },
        children,
    )
}

fn test_conv_source() -> &'static str {
    "in @filesystem\ntemplate $t {\n\tslug\n}\nout root {\n\titems: sub { $t }\n}\n"
}

fn test_domain_tree() -> Prism<Folder> {
    dir_folder(
        "root",
        vec![dir_folder(
            "sub",
            vec![leaf_folder(
                "post.md",
                "---\nslug: hello-world\n---\nContent here",
            )],
        )],
    )
}

fn test_committer() -> Committer {
    Committer::new("Test", "test@test.com")
}

const TEST_TIMESTAMP: &str = "1234567890 +0000";

/// Parse+resolve+execute a .conv file, commit the output tree to a Store.
#[test]
fn conversation_output_committed() {
    let resolved = Conversation::<Filesystem>::from_source(test_conv_source()).unwrap();
    let result = resolved.trace(test_domain_tree()).unwrap();
    let json = serde_json::to_string_pretty(&result).unwrap();

    let fractal = encoding::encode(&json);
    let mut store = Store::<Fractal<String>>::new();
    let commit = Draft::root("conversation output", fractal).commit(
        &mut store,
        test_committer(),
        TEST_TIMESTAMP,
    );

    assert!(matches!(commit, Commit::Root { .. }));
    assert!(!commit.sha().0.is_empty());
    assert!(store.read_commit(commit.sha()).is_some());
}

/// Same input -> same tree OID in Store.
#[test]
fn committed_output_content_addressed() {
    let resolved = Conversation::<Filesystem>::from_source(test_conv_source()).unwrap();
    let result1 = resolved.trace(test_domain_tree()).unwrap();
    let result2 = resolved.trace(test_domain_tree()).unwrap();

    let json1 = serde_json::to_string_pretty(&result1).unwrap();
    let json2 = serde_json::to_string_pretty(&result2).unwrap();

    let fractal1 = encoding::encode(&json1);
    let fractal2 = encoding::encode(&json2);

    let mut store = Store::<Fractal<String>>::new();
    let oid1 = store.write_tree(&fractal1);
    let oid2 = store.write_tree(&fractal2);

    assert_eq!(oid1, oid2);
}

/// Multiple .conv executions form a commit chain.
#[test]
fn commit_chain_from_pipeline() {
    let resolved = Conversation::<Filesystem>::from_source(test_conv_source()).unwrap();
    let mut store = Store::<Fractal<String>>::new();
    let committer = test_committer();

    // First execution
    let result1 = resolved.trace(test_domain_tree()).unwrap();
    let json1 = serde_json::to_string_pretty(&result1).unwrap();
    let c1 = Draft::root("first pipeline run", encoding::encode(&json1)).commit(
        &mut store,
        committer.clone(),
        "1000000000 +0000",
    );

    // Second execution — different content, child commit
    let tree2 = dir_folder(
        "root",
        vec![dir_folder(
            "sub",
            vec![leaf_folder(
                "post.md",
                "---\nslug: updated-post\n---\nNew content",
            )],
        )],
    );
    let result2 = resolved.trace(tree2).unwrap();
    let json2 = serde_json::to_string_pretty(&result2).unwrap();
    let c2 = c1
        .child("second pipeline run", encoding::encode(&json2))
        .commit(&mut store, committer, "1000000001 +0000");

    assert!(matches!(
        store.read_commit(c1.sha()),
        Some(Commit::Root { .. })
    ));
    assert!(matches!(
        store.read_commit(c2.sha()),
        Some(Commit::Child { .. })
    ));
    assert_ne!(c1.sha(), c2.sha());
}

// ---------------------------------------------------------------------------
// Phase 4: Transformation trees as the bridge format
// ---------------------------------------------------------------------------

/// The resolved transformation tree is content-addressed and committable.
#[test]
fn transformation_tree_committed() {
    let resolved = Conversation::<Filesystem>::from_source(test_conv_source()).unwrap();

    // The transformation IS the content tree — Groups and Selects, not JSON output
    let transformation: &Prism<OutputNode> = &resolved.content;
    let oid = content_oid(transformation);

    let mut store = Store::<Prism<OutputNode>>::new();
    let commit = Draft::root(
        "transformation: root { items: sub { $t } }",
        transformation.clone(),
    )
    .commit(&mut store, test_committer(), TEST_TIMESTAMP);

    assert!(matches!(commit, Commit::Root { .. }));
    assert!(!oid.is_empty());
    assert!(store.read_commit(commit.sha()).is_some());
}

/// Same .conv source -> same transformation OID. The transformation is deterministic.
#[test]
fn transformation_tree_deterministic() {
    let a = Conversation::<Filesystem>::from_source(test_conv_source()).unwrap();
    let b = Conversation::<Filesystem>::from_source(test_conv_source()).unwrap();

    assert_eq!(content_oid(&a.content), content_oid(&b.content));
}

/// Different .conv sources -> different transformation OIDs.
#[test]
fn transformation_tree_different_sources_different_oid() {
    let source_a = test_conv_source();
    let source_b =
        "in @filesystem\ntemplate $t {\n\ttitle\n}\nout root {\n\tposts: data { $t }\n}\n";

    let a = Conversation::<Filesystem>::from_source(source_a).unwrap();
    let b = Conversation::<Filesystem>::from_source(source_b).unwrap();

    assert_ne!(content_oid(&a.content), content_oid(&b.content));
}
