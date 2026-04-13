//! Parse — the bridge from `.mirror` source text to content-addressed AST.
//!
//! `Parse` implements `Vector<String, Prism<AstNode>>`. Spectral and other
//! consumers use `Parse.trace(source)` to get a content-addressed parse tree.

use crate::declaration::DeclKind;
use crate::domain::conversation::Kind;
use crate::kernel::{ContentAddressed, Oid, Trace, TraceOid, Vector};
use crate::mirror_runtime::{parse_form, Form};
use crate::prism::{self, Prism};
use fragmentation::encoding::Encode;
use fragmentation::ref_::Ref;
use fragmentation::sha;

// ---------------------------------------------------------------------------
// AstNode — the parsed node type for external consumers
// ---------------------------------------------------------------------------

/// A node in the content-addressed parse tree.
///
/// Carries structural kind, semantic name, and raw value.
/// Used by spectral and other tools that consume `.mirror` grammars.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AstNode {
    /// Structural role: Decl, Atom, Ref, or Form.
    pub kind: Kind,
    /// Semantic name (e.g., "grammar", "action-def", "type", "in").
    pub name: String,
    /// The value/identifier (e.g., "@reed", "observe", "string").
    pub value: String,
}

impl AstNode {
    /// Canonical identity string for content addressing.
    fn label(&self) -> String {
        format!("{:?}:{}:{}", self.kind, self.name, self.value)
    }

    /// Check if this node is a declaration with the given name.
    pub fn is_decl(&self, name: &str) -> bool {
        self.kind == Kind::Decl && self.name == name
    }
}

impl Encode for AstNode {
    fn encode(&self) -> Vec<u8> {
        self.label().into_bytes()
    }
}

impl ContentAddressed for AstNode {
    type Oid = Oid;
    fn content_oid(&self) -> Oid {
        Oid::hash(self.label().as_bytes())
    }
}

domain_oid!(
    /// Content address for AST nodes.
    pub AstOid
);

// ---------------------------------------------------------------------------
// Form → Prism<AstNode> conversion
// ---------------------------------------------------------------------------

/// Map a DeclKind to the (Kind, name) pair for AstNode.
fn decl_to_ast(kind: &DeclKind) -> (Kind, &'static str) {
    match kind {
        DeclKind::Form => (Kind::Form, "form"),
        DeclKind::Type => (Kind::Decl, "type"),
        DeclKind::Prism => (Kind::Decl, "prism"),
        DeclKind::In => (Kind::Decl, "in"),
        DeclKind::Out => (Kind::Decl, "out"),
        DeclKind::Property => (Kind::Decl, "property"),
        DeclKind::Fold => (Kind::Decl, "fold"),
        DeclKind::Requires => (Kind::Decl, "requires"),
        DeclKind::Invariant => (Kind::Decl, "invariant"),
        DeclKind::Ensures => (Kind::Decl, "ensures"),
        DeclKind::Focus => (Kind::Decl, "focus"),
        DeclKind::Project => (Kind::Decl, "project"),
        DeclKind::Split => (Kind::Decl, "split"),
        DeclKind::Zoom => (Kind::Decl, "zoom"),
        DeclKind::Refract => (Kind::Decl, "refract"),
        DeclKind::Traversal => (Kind::Decl, "traversal"),
        DeclKind::Lens => (Kind::Decl, "lens"),
        DeclKind::Action => (Kind::Decl, "action-def"),
        DeclKind::Grammar => (Kind::Decl, "grammar"),
    }
}

/// Convert a Form tree into a Prism<AstNode> tree.
fn form_to_prism(form: &Form) -> Prism<AstNode> {
    let (kind, name) = decl_to_ast(&form.kind);
    let node = AstNode {
        kind,
        name: name.to_string(),
        value: form.name.clone(),
    };
    let ref_ = {
        let label = node.label();
        Ref::new(sha::hash(&label), &label)
    };
    let children: Vec<Prism<AstNode>> = form.children.iter().map(form_to_prism).collect();
    if children.is_empty() {
        prism::shard(ref_, node)
    } else {
        prism::fractal(ref_, node, children)
    }
}

// ---------------------------------------------------------------------------
// Parse — the Vector implementation
// ---------------------------------------------------------------------------

/// Unit struct for parsing `.mirror` source into content-addressed AST.
///
/// ```ignore
/// use mirror::parse::Parse;
/// use mirror::Vector;
///
/// let ast = Parse.trace(source).into_result().unwrap();
/// for child in ast.children() {
///     println!("{}:{}", child.data().name, child.data().value);
/// }
/// ```
pub struct Parse;

impl Vector<String, Prism<AstNode>> for Parse {
    type Error = String;

    fn trace(&self, source: String) -> Trace<Prism<AstNode>, String> {
        match parse_form(&source) {
            Ok(form) => {
                let tree = form_to_prism(&form);
                let oid = tree.content_oid();
                Trace::success(tree, oid.into(), None)
            }
            Err(e) => Trace::failure(e.to_string(), TraceOid::new("parse-error"), None),
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_grammar_produces_decl_node() {
        let source = "grammar @test {\n  type id\n}".to_string();
        let result = Parse.trace(source);
        assert!(result.is_ok());
        let tree = result.unwrap();
        // Top-level form should be a grammar declaration
        assert!(tree.data().is_decl("grammar"));
        assert_eq!(tree.data().value, "@test");
    }

    #[test]
    fn parse_empty_source_produces_error() {
        let source = "".to_string();
        let result = Parse.trace(source);
        // Empty source should either parse to something or error —
        // either way, the Vector contract is satisfied.
        // parse_form on empty returns an error.
        assert!(result.is_err());
    }

    #[test]
    fn ast_node_content_addressed_deterministic() {
        let a = AstNode {
            kind: Kind::Decl,
            name: "grammar".into(),
            value: "@test".into(),
        };
        let b = AstNode {
            kind: Kind::Decl,
            name: "grammar".into(),
            value: "@test".into(),
        };
        assert_eq!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn ast_node_different_value_different_oid() {
        let a = AstNode {
            kind: Kind::Decl,
            name: "grammar".into(),
            value: "@test".into(),
        };
        let b = AstNode {
            kind: Kind::Decl,
            name: "grammar".into(),
            value: "@other".into(),
        };
        assert_ne!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn ast_node_is_decl() {
        let node = AstNode {
            kind: Kind::Decl,
            name: "grammar".into(),
            value: "@test".into(),
        };
        assert!(node.is_decl("grammar"));
        assert!(!node.is_decl("type"));
    }

    #[test]
    fn ast_node_non_decl_is_not_decl() {
        let node = AstNode {
            kind: Kind::Form,
            name: "grammar".into(),
            value: "@test".into(),
        };
        assert!(!node.is_decl("grammar"));
    }

    #[test]
    fn parse_grammar_children_include_types() {
        let source = "grammar @test {\n  type id\n  type name\n}".to_string();
        let result = Parse.trace(source);
        assert!(result.is_ok());
        let tree = result.unwrap();
        assert!(tree.children().len() >= 2);
        // Children should be type declarations
        for child in tree.children() {
            assert!(child.data().is_decl("type"));
        }
    }

    #[test]
    fn parse_action_maps_to_action_def() {
        let source = "grammar @test {\n  type id\n  action greet(name)\n}".to_string();
        let result = Parse.trace(source);
        assert!(result.is_ok());
        let tree = result.unwrap();
        let actions: Vec<_> = tree
            .children()
            .iter()
            .filter(|c| c.data().name == "action-def")
            .collect();
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].data().value, "greet");
    }

    #[test]
    fn ast_oid_from_oid() {
        let base = Oid::new("abc");
        let ast = AstOid::from(base);
        assert_eq!(ast.as_ref(), "abc");
        assert_eq!(ast.to_string(), "abc");
        let direct = AstOid::new("def");
        assert_eq!(direct.as_ref(), "def");
    }
}
