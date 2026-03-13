pub mod beam;
pub mod conversation;
pub mod filesystem;
pub mod git;

pub use story::scene::{Addressable, Scene};

#[cfg(test)]
mod tests {
    use super::*;
    use filesystem::Filesystem;

    #[test]
    fn scene_is_trait() {
        fn requires_scene<C: Scene>() -> &'static str {
            C::id()
        }
        requires_scene::<Filesystem>();
    }

    #[test]
    fn filesystem_token_is_folder() {
        fn assert_token<C: Scene<Token = filesystem::Folder>>() {}
        assert_token::<Filesystem>();
    }

    #[test]
    fn conversation_token_is_ast_node() {
        fn assert_token<C: Scene<Token = crate::ast::AstNode>>() {}
        assert_token::<conversation::Conversation>();
    }

    // -- Git domain --

    #[test]
    fn git_token_is_git_node() {
        fn assert_token<C: Scene<Token = git::GitNode>>() {}
        assert_token::<git::Git>();
    }

    // -- BEAM domain --

    #[test]
    fn beam_token_is_beam_node() {
        fn assert_token<C: Scene<Token = beam::BeamNode>>() {}
        assert_token::<beam::Beam>();
    }
}
