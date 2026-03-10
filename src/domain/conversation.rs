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
pub enum Token {
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
}

impl Context for Conversation {
    type Token = Token;
    type Data = crate::ast::AstNode;
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
    fn conversation_local_names_from_debug() {
        assert_eq!(Conversation::local_name(&Token::In), "In");
        assert_eq!(Conversation::local_name(&Token::Out), "Out");
        assert_eq!(Conversation::local_name(&Token::Template), "Template");
        assert_eq!(Conversation::local_name(&Token::Field), "Field");
        assert_eq!(Conversation::local_name(&Token::Qualifier), "Qualifier");
        assert_eq!(Conversation::local_name(&Token::Pipe), "Pipe");
        assert_eq!(Conversation::local_name(&Token::Group), "Group");
        assert_eq!(Conversation::local_name(&Token::Select), "Select");
        assert_eq!(Conversation::local_name(&Token::TemplateRef), "TemplateRef");
        assert_eq!(Conversation::local_name(&Token::DomainRef), "DomainRef");
    }

    #[test]
    fn conversation_is_context() {
        fn requires_context<C: Context>() -> &'static str {
            C::id()
        }
        assert_eq!(requires_context::<Conversation>(), "conversation");
    }
}
