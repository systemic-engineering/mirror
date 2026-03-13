pub mod beam;
pub mod conversation;
pub mod filesystem;
pub mod git;

pub use story::setting::{Addressable, Setting};

#[cfg(test)]
mod tests {
    use super::*;
    use filesystem::Filesystem;

    #[test]
    fn scene_is_trait() {
        fn requires_scene<C: Setting>() -> &'static str {
            C::id()
        }
        requires_scene::<Filesystem>();
    }

    #[test]
    fn filesystem_token_is_folder() {
        fn assert_token<C: Setting<Token = filesystem::Folder>>() {}
        assert_token::<Filesystem>();
    }

    #[test]
    fn conversation_token_is_ast_node() {
        fn assert_token<C: Setting<Token = crate::ast::AstNode>>() {}
        assert_token::<conversation::Script>();
    }

    // -- Git domain --

    #[test]
    fn git_token_is_git_node() {
        fn assert_token<C: Setting<Token = git::GitNode>>() {}
        assert_token::<git::Git>();
    }

    // -- BEAM domain --

    #[test]
    fn beam_token_is_beam_node() {
        fn assert_token<C: Setting<Token = beam::BeamNode>>() {}
        assert_token::<beam::Beam>();
    }
}
