use std::collections::HashMap;

use conversation::compile;
use conversation::Story;
use conversation::{
    Conversation, Filesystem, Namespace, OutputNode, Repo, Resolve, Template, TemplateProvider,
};
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
    assert_eq!(compile::emit_eaf(&a.content), compile::emit_eaf(&b.content),);
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
    let eaf_commit = Draft::new("compiled: root.eaf", eaf_fractal, parent).commit(
        &mut eaf_repo,
        committer,
        "1234567891 +0000",
    );

    assert!(matches!(transform_commit, Commit::Root { .. }));
    assert!(matches!(eaf_commit, Commit::Child { .. }));
    assert_ne!(transform_commit.sha(), eaf_commit.sha());
}

fn branch_conv_source() -> &'static str {
    "in @filesystem\ntemplate $t {\n\tslug\n}\nout root {\n\titems: sub { $t }\n}\nbranch(.action) {\n  \"hold\" => ..\n  \"exit\" => exit\n}\n"
}

/// EAF emission handles branch nodes.
#[test]
fn emit_eaf_with_branch() {
    let resolved = Conversation::<Filesystem>::from_source(branch_conv_source()).unwrap();
    let eaf_bytes = compile::emit_eaf(&resolved.content);
    assert!(!eaf_bytes.is_empty());
    assert_eq!(eaf_bytes[0], 131);
}

/// Branch EAF is deterministic.
#[test]
fn emit_eaf_branch_deterministic() {
    let a = Conversation::<Filesystem>::from_source(branch_conv_source()).unwrap();
    let b = Conversation::<Filesystem>::from_source(branch_conv_source()).unwrap();
    assert_eq!(compile::emit_eaf(&a.content), compile::emit_eaf(&b.content));
}

/// EAF handles branch with wildcard and expr actions.
#[test]
fn emit_eaf_branch_wild_and_expr() {
    let source = "in @filesystem\ntemplate $t {\n\tslug\n}\nout root {\n\titems: sub { $t }\n}\nbranch(.status) {\n  \"ok\" => ..\n  \"custom\" => handle\n  _ => exit\n}\n";
    let resolved = Conversation::<Filesystem>::from_source(source).unwrap();
    let eaf_bytes = compile::emit_eaf(&resolved.content);
    assert!(!eaf_bytes.is_empty());
    assert_eq!(eaf_bytes[0], 131);
}

// -- Integration: imported templates compile to EAF --

/// Full pipeline: parse with `use` import → resolve → compile to EAF.
#[test]
fn compile_with_imported_template() {
    // Set up namespace with a shared template
    let mut templates = HashMap::new();
    templates.insert("$shared".to_string(), Template::with_fields(&["slug"]));
    let mut ns = Namespace::new();
    ns.register("shared", TemplateProvider::Inline(templates));
    let resolve = Resolve::new().with_namespace(ns);

    // .conv source that imports from @shared and uses both local + imported
    let source = "use $shared from @shared\ntemplate $local {\n\ttitle\n}\nout articles {\n\tdrafts: blog { $shared }\n\tpages: static { $local }\n}\n";
    let ast = conversation::Parse.record(source.to_string()).unwrap();
    let conv: Conversation<Filesystem> = resolve.record(ast).into_result().unwrap();

    // Imported template flows through to Select nodes
    let eaf_bytes = compile::emit_eaf(&conv.content);
    assert!(!eaf_bytes.is_empty());
    assert_eq!(eaf_bytes[0], 131, "valid ETF");
}

/// Same import produces the same OID (content-addressed).
#[test]
fn compile_imported_template_content_addressed() {
    use conversation::ContentAddressed;

    let mut templates = HashMap::new();
    templates.insert("$t".to_string(), Template::with_fields(&["slug"]));

    let make_conv = || {
        let mut ns = Namespace::new();
        ns.register("shared", TemplateProvider::Inline(templates.clone()));
        let resolve = Resolve::new().with_namespace(ns);
        let source = "use $t from @shared\nout r {\n\tx: f { $t }\n}\n";
        let ast = conversation::Parse.record(source.to_string()).unwrap();
        let conv: Conversation<Filesystem> = resolve.record(ast).into_result().unwrap();
        conv
    };

    let a = make_conv();
    let b = make_conv();
    assert_eq!(a.content_oid(), b.content_oid());
}
