//! The conversation domain. The AST's own vocabulary.
//!
//! A .conv file parsed into a tree is a tree in this domain.
//! The crate describes itself.

use super::Setting;

/// The conversation context.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Script;

/// Structural role of an AST node. Not vocabulary — the grammar owns vocabulary.
///
/// Semantic identity lives in `AstNode.name` (e.g., "in", "field", "domain-ref").
/// Kind carries only the structural tier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Top-level: in, out, template, grammar, use, branch, case, when/*
    Decl,
    /// Content: field, param, qualifier, pipe, expr, literal, path, wild, cmp/*
    Atom,
    /// Reference: domain-ref, template-ref, type-ref, ref, alias, home, self, domain-param
    Ref,
    /// Structure: group, select, pipeline, arm, type-def, variant
    Form,
}

impl Setting for Script {
    type Token = crate::ast::AstNode;

    fn id() -> &'static str {
        "conversation"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversation_id() {
        assert_eq!(Script::id(), "conversation");
    }

    #[test]
    fn conversation_is_scene() {
        fn requires_scene<C: Setting>() -> &'static str {
            C::id()
        }
        assert_eq!(requires_scene::<Script>(), "conversation");
    }

    #[test]
    fn kind_debug_decl() {
        assert_eq!(format!("{:?}", Kind::Decl), "Decl");
    }

    #[test]
    fn kind_debug_atom() {
        assert_eq!(format!("{:?}", Kind::Atom), "Atom");
    }

    #[test]
    fn kind_debug_ref() {
        assert_eq!(format!("{:?}", Kind::Ref), "Ref");
    }

    #[test]
    fn kind_debug_form() {
        assert_eq!(format!("{:?}", Kind::Form), "Form");
    }

    #[test]
    fn kind_eq() {
        assert_eq!(Kind::Decl, Kind::Decl);
        assert_ne!(Kind::Decl, Kind::Atom);
    }

    #[test]
    fn kind_clone() {
        let k = Kind::Decl;
        assert_eq!(k.clone(), Kind::Decl);
    }
}
