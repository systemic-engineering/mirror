//! Integration tests for cross-domain grammar resolution.
//!
//! Verifies that grammar packages (@actor, @compiler, @beam, @mail)
//! parse, compile through Mirror, and integrate via the package
//! discovery system.

use std::fs;

use mirror::packages::PackageRegistry;
use mirror::resolve::Resolve;
use mirror::{Conversation, Filesystem, Parse, Vector};
use tempfile::TempDir;

// ---------------------------------------------------------------------------
// Grammar file contents (matching conv/ directory)
// ---------------------------------------------------------------------------

const ACTOR_GRAMMAR: &str = "\
grammar @actor {
  type = identity | session | signal

  type signal = message | question | insight | work | init | exit

  type visibility = public | protected | private
}
";

const COMPILER_GRAMMAR: &str = "\
in @actor

grammar @compiler {
  type = target | artifact | request | result

  type target = gleam | elixir | rust | fortran | eaf

  type artifact = source | bytecode | oid | blob

  type status = ok | error | pending

  action compile(source: artifact, target: target)
}
";

const BEAM_GRAMMAR: &str = "\
grammar @beam {
  type = process | supervision | module
}
";

const MAIL_GRAMMAR: &str = "\
grammar @mail {
  type = message | thread | attachment | address | server

  type header = from | to | cc | bcc | subject
              | date | reply-to | message-id
              | in-reply-to | references

  type flag = seen | flagged | draft | answered | deleted

  type protocol = smtp | imap | jmap

  type server = stalwart | maddy | mailbox

  type dns = spf | dkim | dmarc | mta-sts | dane

  action send(from: address, to: address, subject, body: article)
  action reply(in-reply-to: message-id, body: article)
  action forward(message: message-id, to: address)
}

template $message(@imap) {
  message-id
  from: address
  to: address
  subject
  date
  body: article
}
";

// ---------------------------------------------------------------------------
// Mirror compilation
// ---------------------------------------------------------------------------

#[test]
fn actor_grammar_compiles() {
    let ast = Parse.trace(ACTOR_GRAMMAR.to_string()).unwrap();
    let grammar = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("grammar"))
        .unwrap();
    let reg = mirror::model::Mirror::from_grammar(grammar).unwrap();
    assert_eq!(reg.domain_name(), "actor");
    assert!(reg.has_variant("", "identity"));
    assert!(reg.has_variant("", "session"));
    assert!(reg.has_variant("", "signal"));
    assert!(reg.has_variant("signal", "message"));
    assert!(reg.has_variant("signal", "exit"));
    assert!(reg.has_variant("visibility", "public"));
    assert!(reg.has_variant("visibility", "private"));
}

#[test]
fn compiler_grammar_compiles() {
    let ast = Parse.trace(COMPILER_GRAMMAR.to_string()).unwrap();
    let grammar = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("grammar"))
        .unwrap();
    let reg = mirror::model::Mirror::from_grammar(grammar).unwrap();
    assert_eq!(reg.domain_name(), "compiler");
    assert!(reg.has_variant("", "target"));
    assert!(reg.has_variant("", "artifact"));
    assert!(reg.has_variant("target", "gleam"));
    assert!(reg.has_variant("target", "fortran"));
    assert!(reg.has_variant("target", "eaf"));
    assert!(reg.has_variant("artifact", "source"));
    assert!(reg.has_variant("artifact", "bytecode"));
    assert!(reg.has_variant("status", "ok"));
    assert!(reg.has_variant("status", "error"));
}

#[test]
fn compiler_grammar_has_action_compile() {
    let ast = Parse.trace(COMPILER_GRAMMAR.to_string()).unwrap();
    let grammar = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("grammar"))
        .unwrap();
    let reg = mirror::model::Mirror::from_grammar(grammar).unwrap();
    assert!(reg.has_action("compile"));
    let fields = reg.act_fields("compile").unwrap();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].0, "source");
    assert_eq!(fields[0].1, Some("artifact"));
    assert_eq!(fields[1].0, "target");
    assert_eq!(fields[1].1, Some("target"));
}

#[test]
fn beam_grammar_compiles() {
    let ast = Parse.trace(BEAM_GRAMMAR.to_string()).unwrap();
    let grammar = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("grammar"))
        .unwrap();
    let reg = mirror::model::Mirror::from_grammar(grammar).unwrap();
    assert_eq!(reg.domain_name(), "beam");
    assert!(reg.has_variant("", "process"));
    assert!(reg.has_variant("", "supervision"));
    assert!(reg.has_variant("", "module"));
}

#[test]
fn mail_grammar_compiles_full_type_hierarchy() {
    let ast = Parse.trace(MAIL_GRAMMAR.to_string()).unwrap();
    let grammar = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("grammar"))
        .unwrap();
    let reg = mirror::model::Mirror::from_grammar(grammar).unwrap();
    assert_eq!(reg.domain_name(), "mail");
    assert!(reg.has_variant("", "message"));
    assert!(reg.has_variant("", "thread"));
    assert!(reg.has_variant("", "attachment"));
    assert!(reg.has_variant("header", "from"));
    assert!(reg.has_variant("header", "subject"));
    assert!(reg.has_variant("header", "message-id"));
    assert!(reg.has_variant("header", "in-reply-to"));
    assert!(reg.has_variant("flag", "seen"));
    assert!(reg.has_variant("flag", "deleted"));
    assert!(reg.has_variant("protocol", "smtp"));
    assert!(reg.has_variant("protocol", "jmap"));
    assert!(reg.has_variant("server", "stalwart"));
    assert!(reg.has_variant("dns", "spf"));
    assert!(reg.has_variant("dns", "dmarc"));
}

#[test]
fn mail_grammar_has_three_actions() {
    let ast = Parse.trace(MAIL_GRAMMAR.to_string()).unwrap();
    let grammar = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("grammar"))
        .unwrap();
    let reg = mirror::model::Mirror::from_grammar(grammar).unwrap();

    // action send
    assert!(reg.has_action("send"));
    let send = reg.act_fields("send").unwrap();
    assert_eq!(send.len(), 4);
    assert_eq!(send[0].0, "from");
    assert_eq!(send[0].1, Some("address"));
    assert_eq!(send[1].0, "to");
    assert_eq!(send[2].0, "subject");
    assert_eq!(send[2].1, Some("subject")); // param sugar: subject → subject:subject
    assert_eq!(send[3].0, "body");
    assert_eq!(send[3].1, Some("article"));

    // action reply
    assert!(reg.has_action("reply"));
    let reply = reg.act_fields("reply").unwrap();
    assert_eq!(reply.len(), 2);
    assert_eq!(reply[0].0, "in-reply-to");
    assert_eq!(reply[0].1, Some("message-id"));

    // action forward
    assert!(reg.has_action("forward"));
    let forward = reg.act_fields("forward").unwrap();
    assert_eq!(forward.len(), 2);
    assert_eq!(forward[0].0, "message");
    assert_eq!(forward[1].0, "to");
}

// ---------------------------------------------------------------------------
// Action visibility
// ---------------------------------------------------------------------------

const VISIBILITY_GRAMMAR: &str = "\
grammar @filesystem {
  type = file | path

  public action read(path: path)

  protected action write(path: path, content: file)

  private action validate_path(path: path)

  action default_action(path: path)
}
";

#[test]
fn public_action_parsed_with_visibility() {
    let ast = Parse.trace(VISIBILITY_GRAMMAR.to_string()).unwrap();
    let grammar = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("grammar"))
        .unwrap();
    let action = grammar
        .children()
        .iter()
        .find(|c| c.data().is_decl("action-def") && c.data().value == "read")
        .expect("should have action 'read'");
    let vis = action
        .children()
        .iter()
        .find(|c| c.data().is_atom("visibility"))
        .expect("should have visibility child");
    assert_eq!(vis.data().value, "public");
}

#[test]
fn protected_action_parsed_with_visibility() {
    let ast = Parse.trace(VISIBILITY_GRAMMAR.to_string()).unwrap();
    let grammar = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("grammar"))
        .unwrap();
    let action = grammar
        .children()
        .iter()
        .find(|c| c.data().is_decl("action-def") && c.data().value == "write")
        .expect("should have action 'write'");
    let vis = action
        .children()
        .iter()
        .find(|c| c.data().is_atom("visibility"))
        .expect("should have visibility child");
    assert_eq!(vis.data().value, "protected");
}

#[test]
fn private_action_parsed_with_visibility() {
    let ast = Parse.trace(VISIBILITY_GRAMMAR.to_string()).unwrap();
    let grammar = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("grammar"))
        .unwrap();
    let action = grammar
        .children()
        .iter()
        .find(|c| c.data().is_decl("action-def") && c.data().value == "validate_path")
        .expect("should have action 'validate_path'");
    let vis = action
        .children()
        .iter()
        .find(|c| c.data().is_atom("visibility"))
        .expect("should have visibility child");
    assert_eq!(vis.data().value, "private");
}

#[test]
fn bare_action_defaults_to_protected() {
    let ast = Parse.trace(VISIBILITY_GRAMMAR.to_string()).unwrap();
    let grammar = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("grammar"))
        .unwrap();
    let action = grammar
        .children()
        .iter()
        .find(|c| c.data().is_decl("action-def") && c.data().value == "default_action")
        .expect("should have action 'default_action'");
    let vis = action
        .children()
        .iter()
        .find(|c| c.data().is_atom("visibility"))
        .expect("bare action should have visibility child defaulting to protected");
    assert_eq!(vis.data().value, "protected");
}

#[test]
fn visibility_stored_in_registry() {
    let ast = Parse.trace(VISIBILITY_GRAMMAR.to_string()).unwrap();
    let grammar = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("grammar"))
        .unwrap();
    let reg = mirror::model::Mirror::from_grammar(grammar).unwrap();
    assert_eq!(
        reg.action_visibility("read"),
        mirror::resolve::Visibility::Public,
    );
    assert_eq!(
        reg.action_visibility("write"),
        mirror::resolve::Visibility::Protected,
    );
    assert_eq!(
        reg.action_visibility("validate_path"),
        mirror::resolve::Visibility::Private,
    );
    assert_eq!(
        reg.action_visibility("default_action"),
        mirror::resolve::Visibility::Protected,
    );
}

#[test]
fn mail_template_extracted_by_namespace() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("@mail"), MAIL_GRAMMAR).unwrap();

    let registry = PackageRegistry::discover(dir.path()).unwrap();
    let namespace = registry.to_namespace().unwrap();

    let templates = namespace.get_templates("mail").unwrap();
    assert!(templates.contains_key("$message"));
}

// ---------------------------------------------------------------------------
// Package discovery + namespace
// ---------------------------------------------------------------------------

#[test]
fn discover_grammars_as_packages() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("@actor"), ACTOR_GRAMMAR).unwrap();
    fs::write(dir.path().join("@compiler"), COMPILER_GRAMMAR).unwrap();
    fs::write(dir.path().join("@beam"), BEAM_GRAMMAR).unwrap();
    fs::write(dir.path().join("@mail"), MAIL_GRAMMAR).unwrap();

    let registry = PackageRegistry::discover(dir.path()).unwrap();
    assert_eq!(registry.len(), 4);

    let namespace = registry.to_namespace().unwrap();
    assert!(namespace.contains("actor"));
    assert!(namespace.contains("compiler"));
    assert!(namespace.contains("beam"));
    assert!(namespace.contains("mail"));

    assert!(namespace.has_grammar("actor"));
    assert!(namespace.has_grammar("compiler"));
    assert!(namespace.has_grammar("beam"));
    assert!(namespace.has_grammar("mail"));
}

#[test]
fn compiler_grammar_types_available_via_namespace() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("@actor"), ACTOR_GRAMMAR).unwrap();
    fs::write(dir.path().join("@compiler"), COMPILER_GRAMMAR).unwrap();

    let registry = PackageRegistry::discover(dir.path()).unwrap();
    let namespace = registry.to_namespace().unwrap();
    let compiler_dom = namespace
        .domain("compiler")
        .expect("compiler domain registered");
    assert!(compiler_dom.has_variant("target", "eaf"));
    assert!(compiler_dom.has_action("compile"));
}

// ---------------------------------------------------------------------------
// Cross-domain resolution: in @compiler resolves when namespace has it
// ---------------------------------------------------------------------------

#[test]
fn in_compiler_resolves_with_namespace() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("@actor"), ACTOR_GRAMMAR).unwrap();
    fs::write(dir.path().join("@compiler"), COMPILER_GRAMMAR).unwrap();

    let registry = PackageRegistry::discover(dir.path()).unwrap();
    let namespace = registry.to_namespace().unwrap();
    let resolve = Resolve::new().with_namespace(namespace);

    // A .conv file that declares `in @compiler` should resolve
    // when @compiler is in the namespace.
    let source = "in @compiler\ntemplate $t {\n\tname\n}\nout targets {\n\tall: sub { $t }\n}\n";
    let result = Conversation::<Filesystem>::from_source_with(source, resolve);
    assert!(result.is_ok(), "expected Ok, got: {:?}", result.err());
}

#[test]
fn in_actor_resolves_with_namespace() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("@actor"), ACTOR_GRAMMAR).unwrap();

    let registry = PackageRegistry::discover(dir.path()).unwrap();
    let namespace = registry.to_namespace().unwrap();
    let resolve = Resolve::new().with_namespace(namespace);

    let source = "in @actor\ntemplate $t {\n\tname\n}\nout signals {\n\tall: sub { $t }\n}\n";
    let result = Conversation::<Filesystem>::from_source_with(source, resolve);
    assert!(result.is_ok(), "expected Ok, got: {:?}", result.err());
}

#[test]
fn in_unknown_domain_fails() {
    let resolve = Resolve::new();
    let source = "in @nonexistent\ntemplate $t {\n\tname\n}\nout x {\n\ty: sub { $t }\n}\n";
    let result = Conversation::<Filesystem>::from_source_with(source, resolve);
    assert!(result.is_err());
    let err = format!("{}", result.unwrap_err());
    assert!(
        err.contains("unknown domain"),
        "expected 'unknown domain', got: {}",
        err
    );
}
