//! AST primitives. Span + AstNode.
//!
//! The AST is `Tree<AstNode>`. A .conv file parsed is a tree
//! in the conversation domain. Same type as everything else.

use sha2::{Digest, Sha256};

use crate::domain::conversation::Kind;
use crate::tree::{self, Tree};
use crate::ContentAddressed;
use fragmentation::ref_::Ref;
use fragmentation::sha;

domain_oid!(/// Content address for AST nodes.
pub AstOid);

impl fragmentation::encoding::Encode for AstNode {
    fn encode(&self) -> Vec<u8> {
        format!("{:?}:{}:{}", self.kind, self.name, self.value).into_bytes()
    }
}

impl ContentAddressed for AstNode {
    type Oid = AstOid;
    fn content_oid(&self) -> AstOid {
        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}:{}:{}", self.kind, self.name, self.value).as_bytes());
        AstOid::new(hex::encode(hasher.finalize()))
    }
}

/// Byte offset range in source. Every AST node carries one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Self {
        Span { start, end }
    }

    /// Merge two spans into one covering both.
    pub fn merge(&self, other: &Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

/// A node in the AST. Carries syntax kind, semantic name, raw text, and source location.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AstNode {
    pub kind: Kind,
    pub name: String,
    pub value: String,
    pub span: Span,
}

/// Build a leaf AST node. Ref is content-addressed from `kind:name:value`.
pub fn ast_leaf(
    kind: Kind,
    name: impl Into<String>,
    value: impl Into<String>,
    span: Span,
) -> Tree<AstNode> {
    let name = name.into();
    let value = value.into();
    let ref_ = ast_ref(&kind, &name, &value);
    tree::leaf(
        ref_,
        AstNode {
            kind,
            name,
            value,
            span,
        },
    )
}

/// Build a branch AST node. Ref is content-addressed from `kind:name:value`.
pub fn ast_branch(
    kind: Kind,
    name: impl Into<String>,
    value: impl Into<String>,
    span: Span,
    children: Vec<Tree<AstNode>>,
) -> Tree<AstNode> {
    let name = name.into();
    let value = value.into();
    let ref_ = ast_ref(&kind, &name, &value);
    tree::branch(
        ref_,
        AstNode {
            kind,
            name,
            value,
            span,
        },
        children,
    )
}

/// Content-addressed ref from kind + name + value.
fn ast_ref(kind: &Kind, name: &str, value: &str) -> Ref {
    let label = format!("{:?}:{}:{}", kind, name, value);
    Ref::new(sha::hash(&label), label)
}

#[cfg(test)]
mod tests {
    use super::*;
    // -- ContentAddressed --

    #[test]
    fn ast_node_content_addressed() {
        let a = AstNode {
            kind: Kind::Field,
            name: "field".into(),
            value: "slug".into(),
            span: Span::new(0, 4),
        };
        let b = AstNode {
            kind: Kind::Field,
            name: "field".into(),
            value: "slug".into(),
            span: Span::new(100, 104),
        };
        // Same kind + value = same OID, regardless of span
        assert_eq!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn ast_node_different_kind_different_oid() {
        let a = AstNode {
            kind: Kind::Field,
            name: "field".into(),
            value: "html".into(),
            span: Span::new(0, 4),
        };
        let b = AstNode {
            kind: Kind::Qualifier,
            name: "qualifier".into(),
            value: "html".into(),
            span: Span::new(0, 4),
        };
        assert_ne!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn content_address_includes_name() {
        // Same kind + value but different name → different OID
        let a = AstNode {
            kind: Kind::Field,
            name: "field".into(),
            value: "x".into(),
            span: Span::new(0, 1),
        };
        let b = AstNode {
            kind: Kind::Field,
            name: "custom".into(),
            value: "x".into(),
            span: Span::new(0, 1),
        };
        assert_ne!(a.content_oid(), b.content_oid());
    }

    // -- Span tests --

    #[test]
    fn span_new() {
        let s = Span::new(0, 10);
        assert_eq!(s.start, 0);
        assert_eq!(s.end, 10);
    }

    #[test]
    fn span_merge_covers_both() {
        let a = Span::new(5, 10);
        let b = Span::new(2, 7);
        let merged = a.merge(&b);
        assert_eq!(merged.start, 2);
        assert_eq!(merged.end, 10);
    }

    #[test]
    fn span_merge_is_commutative() {
        let a = Span::new(5, 10);
        let b = Span::new(2, 7);
        assert_eq!(a.merge(&b), b.merge(&a));
    }

    // -- AstNode + tree construction --

    #[test]
    fn ast_leaf_is_terminal() {
        let node = ast_leaf(Kind::Field, "field", "slug", Span::new(0, 4));
        assert!(node.is_shard());
        assert_eq!(node.data().kind, Kind::Field);
        assert_eq!(node.data().value, "slug");
        assert_eq!(node.data().span, Span::new(0, 4));
    }

    #[test]
    fn ast_branch_has_children() {
        let children = vec![
            ast_leaf(Kind::Field, "field", "slug", Span::new(10, 14)),
            ast_leaf(Kind::Field, "field", "excerpt", Span::new(16, 23)),
        ];
        let node = ast_branch(
            Kind::Template,
            "template",
            "$corpus",
            Span::new(0, 25),
            children,
        );
        assert!(node.is_fractal());
        assert_eq!(node.children().len(), 2);
        assert_eq!(node.data().kind, Kind::Template);
        assert_eq!(node.data().value, "$corpus");
    }

    #[test]
    fn ast_node_has_name() {
        let node = ast_leaf(Kind::Field, "field", "slug", Span::new(0, 4));
        assert_eq!(node.data().name, "field");
    }

    #[test]
    fn ast_ref_is_content_addressed() {
        let a = ast_leaf(Kind::Field, "field", "slug", Span::new(0, 4));
        let b = ast_leaf(Kind::Field, "field", "slug", Span::new(100, 104));
        // Same kind + value = same ref, regardless of span
        assert_eq!(a.self_ref(), b.self_ref());
    }

    #[test]
    fn different_kind_different_ref() {
        let a = ast_leaf(Kind::Field, "field", "html", Span::new(0, 4));
        let b = ast_leaf(Kind::Qualifier, "qualifier", "html", Span::new(0, 4));
        assert_ne!(a.self_ref(), b.self_ref());
    }

    #[test]
    fn different_value_different_ref() {
        let a = ast_leaf(Kind::Field, "field", "slug", Span::new(0, 4));
        let b = ast_leaf(Kind::Field, "field", "excerpt", Span::new(0, 7));
        assert_ne!(a.self_ref(), b.self_ref());
    }
}
