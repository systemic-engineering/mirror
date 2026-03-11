//! Vector: directed edge between two Systems. IS a Gradient.

use crate::domain::Context;
use crate::gradient::Gradient;
use crate::identity::System;

/// A directed edge between two systems. Carries a gradient.
///
/// `Vector<Filesystem, Git, G>` is a gradient from the filesystem
/// system to the git system, delegating to `G` for the actual
/// transformation.
pub struct Vector<A: Context, B: Context, G> {
    pub left: System<A>,
    pub right: System<B>,
    pub gradient: G,
}

impl<A: Context, B: Context, S, T, G: Gradient<S, T>> Gradient<S, T> for Vector<A, B, G> {
    type Error = G::Error;

    fn emit(&self, source: S) -> Result<T, Self::Error> {
        self.gradient.emit(source)
    }

    fn absorb(&self, source: T) -> Result<S, Self::Error> {
        self.gradient.absorb(source)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::filesystem::Filesystem;
    use crate::domain::git::Git;
    use crate::gradient::{self, Gradient};
    use crate::identity::System;

    use super::Vector;

    #[test]
    fn vector_holds_two_systems() {
        let left: System<Filesystem> = System::new("fs-node");
        let right: System<Git> = System::new("git-node");
        let g = gradient::Identity::<String>::new();
        let v = Vector {
            left,
            right,
            gradient: g,
        };
        assert_eq!(v.left.name(), "fs-node");
        assert_eq!(v.right.name(), "git-node");
    }

    #[test]
    fn vector_is_gradient_emit() {
        let v = Vector {
            left: System::<Filesystem>::new("a"),
            right: System::<Git>::new("b"),
            gradient: gradient::Identity::<String>::new(),
        };
        let result = v.emit("hello".to_string()).unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn vector_is_gradient_absorb() {
        let v = Vector {
            left: System::<Filesystem>::new("a"),
            right: System::<Git>::new("b"),
            gradient: gradient::Identity::<String>::new(),
        };
        let result = v.absorb("world".to_string()).unwrap();
        assert_eq!(result, "world");
    }
}
