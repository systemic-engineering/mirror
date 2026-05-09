//! Mirror declaration types — content-addressed fragments.
//!
//! DeclKind is dead. MirrorData is dead. The five operations are the type system.
//! This file contains OpticOp, fragment types, and fragment helpers.

use fragmentation::encoding::Encode;
use fragmentation::fragment::Fractal;
use fragmentation::ref_::Ref;
use fragmentation::sha::{HashAlg, Sha};

use crate::mirror_ast::MirrorAST;

// ---------------------------------------------------------------------------
// OpticOp — the five prism operations as operator tokens
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OpticOp {
    Iso,
    Fold,
    Split,
    Focus,
    Zoom,
    Refract,
    Subset,
    Superset,
    NotIso,
    Unfold,
}

impl OpticOp {
    pub fn from_token(token: &str) -> Option<OpticOp> {
        match token {
            "=" => Some(OpticOp::Iso),
            "<=" => Some(OpticOp::Fold),
            "|" => Some(OpticOp::Split),
            "->" | "|>" | "<|" | "/" => Some(OpticOp::Zoom),
            "+" => Some(OpticOp::Zoom),
            ".." => Some(OpticOp::Refract),
            "<" => Some(OpticOp::Subset),
            ">" => Some(OpticOp::Superset),
            "!=" => Some(OpticOp::NotIso),
            "=>" => Some(OpticOp::Unfold),
            "<-" => Some(OpticOp::Zoom),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            OpticOp::Iso => "=",
            OpticOp::Fold => "<=",
            OpticOp::Split => "|",
            OpticOp::Focus => "()",
            OpticOp::Zoom => "->",
            OpticOp::Refract => "..",
            OpticOp::Subset => "<",
            OpticOp::Superset => ">",
            OpticOp::NotIso => "!=",
            OpticOp::Unfold => "=>",
        }
    }
}

impl std::fmt::Display for OpticOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

// ---------------------------------------------------------------------------
// MirrorFragment — content-addressed AST tree
// ---------------------------------------------------------------------------

pub type MirrorFragment = Fractal<MirrorAST>;
pub type MirrorHash = Sha;

pub trait MirrorFragmentExt {
    fn mirror_ast(&self) -> &MirrorAST;
    fn mirror_children(&self) -> &[MirrorFragment];
    fn content_hash(&self) -> &MirrorHash;
}

impl MirrorFragmentExt for MirrorFragment {
    fn mirror_ast(&self) -> &MirrorAST {
        use fragmentation::fragment::Fragmentable;
        self.data()
    }

    fn mirror_children(&self) -> &[MirrorFragment] {
        use fragmentation::fragment::Fragmentable;
        self.children()
    }

    fn content_hash(&self) -> &MirrorHash {
        use fragmentation::fragment::Fragmentable;
        &self.self_ref().sha
    }
}

pub fn fragment(ast: MirrorAST, children: Vec<MirrorFragment>) -> MirrorFragment {
    let encoded = ast.encode();
    let hash = Sha::hash(&encoded);
    let ref_ = Ref::new(hash, ast.kind_name());
    if children.is_empty() {
        Fractal::shard_typed(ref_, ast)
    } else {
        Fractal::new_typed(ref_, ast, children)
    }
}
