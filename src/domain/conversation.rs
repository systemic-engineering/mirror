//! The conversation domain. The AST's own vocabulary.
//!
//! A .conv file parsed into a tree is a tree in this domain.
//! The crate describes itself.

use super::Context;

/// The conversation context.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Conversation;

/// What a conversation node can be.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    /// `in @domain`
    In,
    /// `out name { ... }`
    Out,
    /// `template $name { ... }`
    Template,
    /// `slug`, `excerpt`
    Field,
    /// `h2`, `article`
    Qualifier,
    /// `| @html`
    Pipe,
    /// `pieces { ... }`
    Group,
    /// `draft: 1draft { $corpus }`
    Select,
    /// `$corpus`
    TemplateRef,
    /// `@filesystem`
    DomainRef,
    /// `@git(branch: "master") | HEAD | @git(branch: "test")`
    Pipeline,
    /// `branch: "master"` — key-value parameter on a domain ref
    DomainParam,
    /// `HEAD` — a bare reference in a pipeline
    Ref,
    /// `$a` in `in @number as $a` — binding alias
    Alias,
    /// `$a + $b` — expression in output block
    Expr,
}

impl Context for Conversation {
    type Token = crate::ast::AstNode;
    type Keys = fragmentation::keys::PlainKeys;

    fn id() -> &'static str {
        "conversation"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversation_id() {
        assert_eq!(Conversation::id(), "conversation");
    }

    #[test]
    fn conversation_is_context() {
        fn requires_context<C: Context>() -> &'static str {
            C::id()
        }
        assert_eq!(requires_context::<Conversation>(), "conversation");
    }
}
