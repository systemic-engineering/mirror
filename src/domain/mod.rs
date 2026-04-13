pub mod conversation;
pub mod filesystem;

pub use crate::kernel::{Addressable, Setting};

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
    fn conversation_token_is_ast() {
        fn assert_token<C: Setting<Token = crate::ast::Ast>>() {}
        assert_token::<conversation::Script>();
    }
}
