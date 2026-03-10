//! The conversation domain. The AST's own vocabulary.
//!
//! A .conv file parsed into a tree is a tree in this domain.
//! The crate describes itself.

use std::borrow::Cow;

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

    fn local_name(kind: &Language) -> Cow<'static, str> {
        match kind {
            Language::In => "in".into(),
            Language::Out => "out".into(),
            Language::Template => "template".into(),
            Language::Field => "field".into(),
            Language::Qualifier => "qualifier".into(),
            Language::Pipe => "pipe".into(),
            Language::Group => "group".into(),
            Language::Select => "select".into(),
            Language::TemplateRef => "template_ref".into(),
            Language::DomainRef => "domain_ref".into(),
        }
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
    fn conversation_local_names() {
        assert_eq!(Conversation::local_name(&Language::In), "in");
        assert_eq!(Conversation::local_name(&Language::Out), "out");
        assert_eq!(Conversation::local_name(&Language::Template), "template");
        assert_eq!(Conversation::local_name(&Language::Field), "field");
        assert_eq!(Conversation::local_name(&Language::Qualifier), "qualifier");
        assert_eq!(Conversation::local_name(&Language::Pipe), "pipe");
        assert_eq!(Conversation::local_name(&Language::Group), "group");
        assert_eq!(Conversation::local_name(&Language::Select), "select");
        assert_eq!(
            Conversation::local_name(&Language::TemplateRef),
            "template_ref"
        );
        assert_eq!(Conversation::local_name(&Language::DomainRef), "domain_ref");
    }

    #[test]
    fn conversation_is_domain() {
        fn requires_domain<D: Domain>() -> &'static str {
            D::id()
        }
        assert_eq!(requires_domain::<Conversation>(), "conversation");
    }
}
