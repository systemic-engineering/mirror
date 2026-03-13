pub mod beam;
pub mod conversation;
pub mod filesystem;
pub mod git;

/// The tree's context. Defines the domain's token vocabulary.
///
/// A Context names what nodes in a tree can mean.
/// `@filesystem`: directories and files.
/// `@html`: articles, sections, headings.
/// `@document`: sections, paragraphs, code blocks.
///
/// The context makes a tree interpretable.
/// Crossing between domains is a Gradient.
pub trait Context: Clone + std::fmt::Debug + PartialEq + Eq {
    type Token: Clone + std::fmt::Debug + PartialEq + Eq + crate::witness::ContentAddressed;

    fn id() -> &'static str;
}

/// What a tree node must provide for conversation execution.
///
/// The conversation program navigates domain trees by name
/// and extracts content from leaf nodes. This is the interface
/// between the resolved program and the domain's tree structure.
pub trait Addressable {
    fn node_name(&self) -> &str;
    fn node_content(&self) -> Option<&str>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use filesystem::Filesystem;

    #[test]
    fn context_is_trait() {
        fn requires_context<C: Context>() -> &'static str {
            C::id()
        }
        requires_context::<Filesystem>();
    }

    #[test]
    fn filesystem_token_is_folder() {
        fn assert_token<C: Context<Token = filesystem::Folder>>() {}
        assert_token::<Filesystem>();
    }

    #[test]
    fn conversation_token_is_ast_node() {
        fn assert_token<C: Context<Token = crate::ast::AstNode>>() {}
        assert_token::<conversation::Conversation>();
    }

    // -- Git domain --

    #[test]
    fn git_token_is_git_node() {
        fn assert_token<C: Context<Token = git::GitNode>>() {}
        assert_token::<git::Git>();
    }

    // -- BEAM domain --

    #[test]
    fn beam_token_is_beam_node() {
        fn assert_token<C: Context<Token = beam::BeamNode>>() {}
        assert_token::<beam::Beam>();
    }
}
