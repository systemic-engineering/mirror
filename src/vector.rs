//! Vector: directed edge between two endpoints. IS a Gradient.

use crate::gradient::Gradient;
use crate::witness::{ContentAddressed, Trace};

/// A directed edge between two endpoints. Carries a gradient.
///
/// L and R are the endpoints (typically System impls).
/// G is the gradient that transforms between them.
pub struct Vector<L, R, G> {
    pub left: L,
    pub right: R,
    pub gradient: G,
}

impl<L, R, S, T: ContentAddressed, G: Gradient<S, T>> Gradient<S, T> for Vector<L, R, G> {
    type Error = G::Error;

    fn trace(&self, source: S) -> Trace<T, Self::Error> {
        self.gradient.trace(source)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::filesystem::Filesystem;
    use crate::domain::git::Git;
    use crate::gradient::{self, Gradient};
    use crate::identity::tests::TestIdentity;
    use crate::identity::{Identity, Node};

    use super::Vector;

    fn test_id(name: &str) -> TestIdentity {
        TestIdentity::new(name, None)
    }

    #[test]
    fn vector_holds_two_nodes() {
        let v = Vector {
            left: Node::new(test_id("fs-node"), Filesystem),
            right: Node::new(test_id("git-node"), Git),
            gradient: gradient::Identity::<String>::new(),
        };
        assert_eq!(v.left.id.name().as_ref(), "fs-node");
        assert_eq!(v.right.id.name().as_ref(), "git-node");
    }

    #[test]
    fn vector_is_gradient_trace() {
        let v = Vector {
            left: Node::new(test_id("a"), Filesystem),
            right: Node::new(test_id("b"), Git),
            gradient: gradient::Identity::<String>::new(),
        };
        let result = v.trace("hello".to_string()).unwrap();
        assert_eq!(result, "hello");
    }
}
