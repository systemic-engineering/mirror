//! The conversation domain. The AST's own vocabulary.
//!
//! A .conv file parsed into a tree is a tree in this domain.
//! The crate describes itself.

use super::Domain;

/// The conversation vocabulary.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Conversation;

/// What a conversation node can be.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Language {
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

impl Domain for Conversation {
    type Language = Language;

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
        assert_eq!(Conversation::local_name(&Language::In), "In");
        assert_eq!(Conversation::local_name(&Language::Out), "Out");
        assert_eq!(Conversation::local_name(&Language::Template), "Template");
        assert_eq!(Conversation::local_name(&Language::Field), "Field");
        assert_eq!(Conversation::local_name(&Language::Qualifier), "Qualifier");
        assert_eq!(Conversation::local_name(&Language::Pipe), "Pipe");
        assert_eq!(Conversation::local_name(&Language::Group), "Group");
        assert_eq!(Conversation::local_name(&Language::Select), "Select");
        assert_eq!(
            Conversation::local_name(&Language::TemplateRef),
            "TemplateRef"
        );
        assert_eq!(Conversation::local_name(&Language::DomainRef), "DomainRef");
    }

    #[test]
    fn conversation_is_domain() {
        fn requires_domain<D: Domain>() -> &'static str {
            D::id()
        }
        assert_eq!(requires_domain::<Conversation>(), "conversation");
    }
}
