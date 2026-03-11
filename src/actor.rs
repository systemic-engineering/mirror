//! Actor: the edge, the glue, the gradient between two contexts.
//!
//! An actor always exists in two contexts. Native is where they come from.
//! Mask is the context they perform in. The gradient goes from native to mask.

use crate::domain::Context;
use crate::gradient::Gradient;
use crate::identity::{Identity, Signal, System};
use crate::witness::{Oid, Trace};

/// An actor always exists in two contexts.
///
/// Native is where they come from. Mask is the context they perform in.
/// The mask isn't deception — it's the interface. You can't participate
/// without one.
///
/// Actor implements `System<Native>` — you are a system where you come from.
/// Actor implements `Gradient<Signal<Native>, Signal<Mask>>` — the gradient
/// goes from native to mask. `trace` is code-switching.
pub struct Actor<I: Identity, Native: Context, Mask: Context> {
    id: I,
    native: Native,
    mask: Mask,
}

impl<I: Identity, N: Context, M: Context> Actor<I, N, M> {
    pub fn new(id: I, native: N, mask: M) -> Self {
        Actor { id, native, mask }
    }

    pub fn identity(&self) -> &I {
        &self.id
    }

    pub fn native(&self) -> &N {
        &self.native
    }

    pub fn mask(&self) -> &M {
        &self.mask
    }
}

impl<I: Identity, N: Context, M: Context> System<N> for Actor<I, N, M> {
    type Identity = I;

    fn identity(&self) -> &I {
        &self.id
    }
}

/// Placeholder error for Actor gradient operations.
/// Real ODA transformation logic comes later.
#[derive(Debug)]
pub struct ActorError;

impl std::fmt::Display for ActorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "actor gradient not yet implemented")
    }
}

impl<I, N, M> Gradient<Signal<N>, Signal<M>> for Actor<I, N, M>
where
    I: Identity,
    N: Context,
    M: Context,
{
    type Error = ActorError;

    fn trace(&self, _source: Signal<N>) -> Trace<Signal<M>, Self::Error> {
        Trace::leaf(Err(ActorError), Oid::new("error"))
    }
}

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
        let actor = Actor::new(test_id("Reed"), Filesystem, Git);
        let sys: &dyn System<Filesystem, Identity = TestIdentity> = &actor;
        assert_eq!(sys.identity().name().as_ref(), "Reed");
    }

    // -- Gradient --

    #[test]
    fn actor_gradient_trace_returns_placeholder_error() {
        use crate::domain::filesystem::Folder;
        let actor = Actor::new(test_id("Reed"), Filesystem, Git);
        let signal = Signal {
            token: Folder {
                name: "test".into(),
                content: None,
            },
        };
        let result = actor.trace(signal);
        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.into_result().unwrap_err()),
            "actor gradient not yet implemented"
        );
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
