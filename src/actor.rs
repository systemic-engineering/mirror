//! Actor: the edge, the glue, the gradient between two contexts.
//!
//! An actor always exists in two contexts. Native is where they come from.
//! Mask is the context they perform in. The gradient goes from native to mask.

use crate::domain::Context;
use crate::gradient::Gradient;
use crate::identity::{Identity, Signal, System};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::filesystem::Filesystem;
    use crate::domain::git::Git;
    use crate::identity::tests::TestIdentity;
    use crate::identity::Node;
    use crate::vector::Vector;

    fn test_id(name: &str) -> TestIdentity {
        TestIdentity::new(name, None)
    }

    // -- Construction --

    #[test]
    fn actor_carries_identity() {
        let actor = Actor::new(test_id("Reed"), Filesystem, Git);
        assert_eq!(actor.identity().name().as_ref(), "Reed");
    }

    #[test]
    fn actor_accessors() {
        let actor = Actor::new(test_id("Reed"), Filesystem, Git);
        assert_eq!(*actor.native(), Filesystem);
        assert_eq!(*actor.mask(), Git);
    }

    // -- System<Native> --

    #[test]
    fn actor_is_system_in_native_context() {
        fn assert_system<C: Context>(_: &impl System<C>) {}
        let actor = Actor::new(test_id("Reed"), Filesystem, Git);
        assert_system::<Filesystem>(&actor);
    }

    // -- Gradient --

    #[test]
    fn actor_implements_gradient() {
        fn assert_gradient<S, T>(_: &impl Gradient<S, T>) {}
        let actor = Actor::new(test_id("Reed"), Filesystem, Git);
        assert_gradient::<Signal<Filesystem>, Signal<Git>>(&actor);
    }

    // -- Actor in Vector --

    #[test]
    fn actor_as_vector_gradient() {
        let actor = Actor::new(test_id("gradient"), Filesystem, Git);
        let _v = Vector {
            left: Node::new(test_id("fs"), Filesystem),
            right: Node::new(test_id("git"), Git),
            gradient: actor,
        };
    }
}
