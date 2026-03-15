use conversation::compile;
use conversation::{Conversation, Filesystem, OutputNode, Repo};
use fragmentation::commit::{Commit, Draft, Parent};
use fragmentation::encoding;
use fragmentation::witnessed::Committer;

fn test_conv_source() -> &'static str {
    "in @filesystem\ntemplate $t {\n\tslug\n}\nout root {\n\titems: sub { $t }\n}\n"
}

fn test_committer() -> Committer {
    Committer::new("Test", "test@test.com")
}

const TEST_TIMESTAMP: &str = "1234567890 +0000";

/// EAF emission produces valid ETF bytes.
#[test]
fn emit_eaf_produces_valid_etf() {
    let resolved = Conversation::<Filesystem>::from_source(test_conv_source()).unwrap();
    let eaf_bytes = compile::emit_eaf(&resolved.content);
    assert!(!eaf_bytes.is_empty());
    // ETF always starts with version byte 131
    assert_eq!(eaf_bytes[0], 131);
}

/// Same transformation tree → same EAF bytes.
#[test]
fn emit_eaf_deterministic() {
    let a = Conversation::<Filesystem>::from_source(test_conv_source()).unwrap();
    let b = Conversation::<Filesystem>::from_source(test_conv_source()).unwrap();
    assert_eq!(
        compile::emit_eaf(&a.content),
        compile::emit_eaf(&b.content),
    );
}

/// EAF bytes committed as child of the transformation commit.
#[test]
fn eaf_committed_as_child_of_transformation() {
    let resolved = Conversation::<Filesystem>::from_source(test_conv_source()).unwrap();
    let committer = test_committer();

    // First commit: the transformation tree (author witness)
    let mut transform_repo = Repo::<OutputNode>::new();
    let transform_commit = Draft::root(
        "transformation: root { items: sub { $t } }",
        resolved.content.clone(),
    )
    .commit(&mut transform_repo, committer.clone(), TEST_TIMESTAMP);

    // Second commit: the EAF (compiler witness), child of transformation
    let eaf_bytes = compile::emit_eaf(&resolved.content);
    let eaf_fractal = encoding::encode(&hex::encode(&eaf_bytes));
    let mut eaf_repo = Repo::<String>::new();
    let parent = Parent(transform_commit.sha().clone());
    let eaf_commit = Draft::new("compiled: root.eaf", eaf_fractal, parent)
        .commit(&mut eaf_repo, committer, "1234567891 +0000");

    assert!(matches!(transform_commit, Commit::Root { .. }));
    assert!(matches!(eaf_commit, Commit::Child { .. }));
    assert_ne!(transform_commit.sha(), eaf_commit.sha());
}
