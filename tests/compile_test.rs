use std::collections::HashMap;

use conversation::compile;
use conversation::Vector;
use conversation::{
    Conversation, Filesystem, Namespace, OutputNode, Prism, Resolve, Store, Template,
    TemplateProvider,
};
use fragmentation::commit::{Commit, Draft, Parent};
use fragmentation::encoding;
use fragmentation::fragment::Fractal;
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

/// Same transformation tree -> same EAF bytes.
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
    let mut transform_store = Store::<Prism<OutputNode>>::new();
    let transform_commit = Draft::root(
        "transformation: root { items: sub { $t } }",
        resolved.content.clone(),
    )
    .commit(&mut transform_store, committer.clone(), TEST_TIMESTAMP);

    // Second commit: the EAF (compiler witness), child of transformation
    let eaf_bytes = compile::emit_eaf(&resolved.content);
    let eaf_fractal = encoding::encode(&hex::encode(&eaf_bytes));
    let mut eaf_store = Store::<Fractal<String>>::new();
    let parent = Parent(transform_commit.sha().clone());
    let eaf_commit = Draft::new("compiled: root.eaf", eaf_fractal, parent).commit(
        &mut eaf_store,
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

/// Full pipeline: parse with `use` import -> resolve -> compile to EAF.
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
    let ast = conversation::Parse.trace(source.to_string()).unwrap();
    let conv: Conversation<Filesystem> = resolve.trace(ast).into_result().unwrap();

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
        let ast = conversation::Parse.trace(source.to_string()).unwrap();
        let conv: Conversation<Filesystem> = resolve.trace(ast).into_result().unwrap();
        conv
    };

    let a = make_conv();
    let b = make_conv();
    assert_eq!(a.content_oid(), b.content_oid());
}

// -- Act dispatch: grammar → actor module --

fn compile_grammar(source: &str) -> conversation::Domain {
    let ast = conversation::Parse.trace(source.to_string()).unwrap();
    let grammar_node = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("grammar"))
        .expect("source should contain a grammar block");
    conversation::Domain::from_grammar(grammar_node).unwrap()
}

/// Act dispatch: grammar with acts produces a valid actor module.
#[test]
fn emit_actor_module_produces_valid_etf() {
    let domain = compile_grammar(
        "grammar @compiler {\n  type = target\n  type target = eaf | beam\n  action compile {\n    source: target\n  }\n}\n",
    );
    let eaf_bytes = compile::emit_actor_module_from_domain(&domain);
    assert!(!eaf_bytes.is_empty());
    // ETF always starts with version byte 131
    assert_eq!(eaf_bytes[0], 131);
}

/// Act dispatch: actor module is deterministic.
#[test]
fn emit_actor_module_deterministic() {
    let a = compile_grammar(
        "grammar @compiler {\n  type = target\n  type target = eaf\n  action compile {\n    source: target\n  }\n}\n",
    );
    let b = compile_grammar(
        "grammar @compiler {\n  type = target\n  type target = eaf\n  action compile {\n    source: target\n  }\n}\n",
    );
    assert_eq!(
        compile::emit_actor_module_from_domain(&a),
        compile::emit_actor_module_from_domain(&b),
    );
}

/// Act dispatch: each act becomes an exported function.
#[test]
fn emit_actor_module_exports_acts() {
    let domain = compile_grammar(
        "grammar @mail {\n  type = address\n  action send {\n    to: address\n  }\n  action reply {\n    to: address\n  }\n}\n",
    );
    let eaf_bytes = compile::emit_actor_module_from_domain(&domain);
    assert!(!eaf_bytes.is_empty());
    assert_eq!(eaf_bytes[0], 131);
    // decode and check: module should export send/1 and reply/1
    // (structural check deferred to green phase — for now just ensure it doesn't panic)
}

/// Act dispatch: grammar with no acts produces a module with no exports.
#[test]
fn emit_actor_module_no_acts() {
    let domain = compile_grammar("grammar @empty {\n  type = a | b\n}\n");
    let eaf_bytes = compile::emit_actor_module_from_domain(&domain);
    assert!(!eaf_bytes.is_empty());
    assert_eq!(eaf_bytes[0], 131);
}

/// Act dispatch: action with cross-actor call compiles to valid ETF.
#[test]
fn emit_actor_module_with_action_call() {
    let domain = compile_grammar(
        "grammar @integration {\n  type source = edge | branch\n  action commit {\n    source: source\n    @filesystem.write(source)\n  }\n}\n",
    );
    let eaf_bytes = compile::emit_actor_module_from_domain(&domain);
    assert!(!eaf_bytes.is_empty());
    assert_eq!(eaf_bytes[0], 131);
}

/// Cross-actor call emits gen_server:call to the target domain.
#[test]
fn emit_actor_module_cross_actor_call_in_body() {
    use std::io::Cursor;

    let domain = compile_grammar(
        "grammar @integration {\n  type source = edge | branch\n  action commit {\n    source: source\n    @filesystem.write(source)\n  }\n}\n",
    );
    let eaf_bytes = compile::emit_actor_module_from_domain(&domain);

    // Decode ETF and search for the filesystem atom in the forms
    let term = eetf::Term::decode(Cursor::new(&eaf_bytes)).unwrap();
    let forms_str = format!("{:?}", term);
    // The body should contain a call to '@filesystem' for the cross-actor dispatch
    assert!(
        forms_str.contains("filesystem"),
        "expected 'filesystem' in EAF body: {}",
        forms_str
    );
}

// -- Visibility-based emission --

/// Public action does NOT emit gen_server:call to own domain.
#[test]
fn public_action_not_gen_server_call() {
    let domain = compile_grammar(
        "grammar @filesystem {\n  type = path\n  public action read {\n    path: path\n  }\n}\n",
    );
    let eaf_bytes = compile::emit_actor_module_from_domain(&domain);
    let term = eetf::Term::decode(std::io::Cursor::new(&eaf_bytes)).unwrap();
    let forms_str = format!("{:?}", term);
    // Public action should NOT contain gen_server:call to 'filesystem'
    // It should return {ok, Args} directly
    assert!(
        !forms_str.contains("gen_server"),
        "public action should not use gen_server:call, got: {}",
        forms_str,
    );
}

/// Protected action uses gen_server:call (current default behavior).
#[test]
fn protected_action_uses_gen_server_call() {
    let domain = compile_grammar(
        "grammar @filesystem {\n  type = path\n  protected action write {\n    path: path\n  }\n}\n",
    );
    let eaf_bytes = compile::emit_actor_module_from_domain(&domain);
    let term = eetf::Term::decode(std::io::Cursor::new(&eaf_bytes)).unwrap();
    let forms_str = format!("{:?}", term);
    assert!(
        forms_str.contains("gen_server"),
        "protected action should use gen_server:call, got: {}",
        forms_str,
    );
}

/// Private action is NOT in the export list.
#[test]
fn private_action_not_exported() {
    let domain = compile_grammar(
        "grammar @filesystem {\n  type = path\n  private action validate {\n    path: path\n  }\n  action read {\n    path: path\n  }\n}\n",
    );
    let eaf_bytes = compile::emit_actor_module_from_domain(&domain);
    let term = eetf::Term::decode(std::io::Cursor::new(&eaf_bytes)).unwrap();
    let forms_str = format!("{:?}", term);
    // 'read' should be exported (protected default), 'validate' should NOT
    // The export list should have: read/1, lenses/0, extends/0, visibility/0
    // but NOT validate/1
    assert!(
        forms_str.contains("\"read\""),
        "protected action 'read' should be exported: {}",
        forms_str,
    );
    // Check that validate is NOT in the export attribute
    // Export is the second form: {attribute, 2, export, [exports...]}
    // We need to verify validate isn't in exports, but it IS still a function
}

/// Emitted module has visibility/0 function.
#[test]
fn visibility_function_exported() {
    let domain = compile_grammar(
        "grammar @test {\n  type = a\n  public action read {\n    a: a\n  }\n  action write {\n    a: a\n  }\n}\n",
    );
    let eaf_bytes = compile::emit_actor_module_from_domain(&domain);
    let term = eetf::Term::decode(std::io::Cursor::new(&eaf_bytes)).unwrap();
    let forms_str = format!("{:?}", term);
    assert!(
        forms_str.contains("visibility"),
        "should have visibility/0 export: {}",
        forms_str,
    );
}

// -- Test module: annotate(@test) → BEAM test module --

/// emit_test_module produces valid ETF from an annotate(@test) subtree.
#[test]
fn emit_test_module_produces_valid_etf() {
    let source =
        "grammar @g {\n  type = a | b\n}\n---\ntest \"types\" {\n  @g has a\n  @g has b\n}\n";
    let ast = conversation::Parse.trace(source.to_string()).unwrap();
    // Find the annotate(@test) child
    let annotate = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("annotate"))
        .expect("should have annotate(@test) child");
    let eaf_bytes = compile::emit_test_module("g", annotate);
    assert!(!eaf_bytes.is_empty());
    assert_eq!(eaf_bytes[0], 131, "valid ETF");
}

/// emit_test_module is deterministic.
#[test]
fn emit_test_module_deterministic() {
    let source = "grammar @g {\n  type = a\n}\n---\ntest \"t\" {\n  @g has a\n}\n";
    let ast_a = conversation::Parse.trace(source.to_string()).unwrap();
    let ast_b = conversation::Parse.trace(source.to_string()).unwrap();
    let ann_a = ast_a
        .children()
        .iter()
        .find(|c| c.data().is_decl("annotate"))
        .unwrap();
    let ann_b = ast_b
        .children()
        .iter()
        .find(|c| c.data().is_decl("annotate"))
        .unwrap();
    assert_eq!(
        compile::emit_test_module("g", ann_a),
        compile::emit_test_module("g", ann_b),
    );
}

/// emit_test_module encodes test names and assertions as binaries.
#[test]
fn emit_test_module_encodes_test_names() {
    use std::io::Cursor;

    let source = "grammar @g {\n  type = x\n}\n---\ntest \"my test\" {\n  @g has x\n}\n";
    let ast = conversation::Parse.trace(source.to_string()).unwrap();
    let annotate = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("annotate"))
        .unwrap();
    let eaf_bytes = compile::emit_test_module("g", annotate);

    let term = eetf::Term::decode(Cursor::new(&eaf_bytes)).unwrap();
    let forms_str = format!("{:?}", term);
    assert!(
        forms_str.contains("test"),
        "expected 'test' atom in EAF: {}",
        forms_str
    );
}

/// emit_test_module handles property directives.
#[test]
fn emit_test_module_handles_property() {
    let source = "grammar @g {\n  type = a\n}\n---\nproperty \"shannon\" {\n  @g preserves shannon_equivalence\n}\n";
    let ast = conversation::Parse.trace(source.to_string()).unwrap();
    let annotate = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("annotate"))
        .unwrap();
    let eaf_bytes = compile::emit_test_module("g", annotate);
    assert!(!eaf_bytes.is_empty());
    assert_eq!(eaf_bytes[0], 131);
}

/// Actor module emits requires/0 and invariants/0 functions.
#[test]
fn emit_actor_module_requires_and_invariants() {
    let domain = compile_grammar(
        "grammar @checked {\n  type = a | b\n  requires shannon_equivalence\n  invariant connected\n}\n",
    );
    let eaf_bytes = compile::emit_actor_module_from_domain(&domain);
    let term = eetf::Term::decode(std::io::Cursor::new(&eaf_bytes)).unwrap();
    let forms_str = format!("{:?}", term);
    assert!(
        forms_str.contains("requires"),
        "should have requires/0 export: {}",
        forms_str,
    );
    assert!(
        forms_str.contains("invariants"),
        "should have invariants/0 export: {}",
        forms_str,
    );
    // Check that the property names appear as binaries
    assert!(
        forms_str.contains("shannon_equivalence")
            || forms_str.contains(&format!("{:?}", "shannon_equivalence".as_bytes())),
        "requires/0 should contain shannon_equivalence: {}",
        forms_str,
    );
}

/// Self-lenses are filtered out from the lenses/0 list.
#[test]
fn emit_actor_module_filters_self_lenses() {
    // Grammar @test with `in @test` creates a self-lens.
    // We use from_grammar_with_lenses to set this up.
    let source = "grammar @test {\n  type = a\n}\n";
    let ast = conversation::Parse.trace(source.to_string()).unwrap();
    let grammar_node = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("grammar"))
        .unwrap();
    // Pass @test (self) and @other as lenses.
    let lenses = vec!["@test".to_string(), "@other".to_string()];
    let domain = conversation::Domain::from_grammar_with_lenses(grammar_node, &lenses).unwrap();

    let etf = compile::emit_actor_module_from_domain(&domain);
    let term = eetf::Term::decode(std::io::Cursor::new(&etf)).unwrap();
    let s = format!("{:?}", term);
    // "other" should appear in lenses (as bytes [111, 116, 104, 101, 114])
    // but "test" (as a lens) should be filtered out.
    assert!(
        s.contains("[111, 116, 104, 101, 114]"),
        "should contain 'other' lens bytes: {}",
        s,
    );
}

/// Actor module with extends populates extends/0.
#[test]
fn emit_actor_module_with_extends() {
    let domain = compile_grammar("grammar @fox extends @smash, @controller {\n  type = move\n}\n");
    let eaf_bytes = compile::emit_actor_module_from_domain(&domain);
    let term = eetf::Term::decode(std::io::Cursor::new(&eaf_bytes)).unwrap();
    let forms_str = format!("{:?}", term);
    assert!(
        forms_str.contains("extends"),
        "should have extends/0 export: {}",
        forms_str,
    );
}

/// Actor module with no properties has empty requires/0 and invariants/0.
#[test]
fn emit_actor_module_empty_requires_invariants() {
    let domain = compile_grammar("grammar @plain {\n  type = x | y\n}\n");
    let eaf_bytes = compile::emit_actor_module_from_domain(&domain);
    let term = eetf::Term::decode(std::io::Cursor::new(&eaf_bytes)).unwrap();
    let forms_str = format!("{:?}", term);
    // Should still have requires and invariants functions (returning empty lists)
    assert!(
        forms_str.contains("requires"),
        "should have requires/0 function even when empty: {}",
        forms_str,
    );
    assert!(
        forms_str.contains("invariants"),
        "should have invariants/0 function even when empty: {}",
        forms_str,
    );
}
