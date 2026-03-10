//! Vector: directed edge between two Systems. IS a Gradient.

#[cfg(test)]
mod tests {
    use crate::domain::filesystem::Filesystem;
    use crate::domain::git::Git;
    use crate::gradient::{self, Gradient};
    use crate::identity::System;
    use fragmentation::keys::PlainKeys;

    use super::Vector;

    #[test]
    fn vector_holds_two_systems() {
        let left: System<Filesystem> = System::new("fs-node", PlainKeys);
        let right: System<Git> = System::new("git-node", PlainKeys);
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
            left: System::<Filesystem>::new("a", PlainKeys),
            right: System::<Git>::new("b", PlainKeys),
            gradient: gradient::Identity::<String>::new(),
        };
        let result = v.emit("hello".to_string()).unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn vector_is_gradient_absorb() {
        let v = Vector {
            left: System::<Filesystem>::new("a", PlainKeys),
            right: System::<Git>::new("b", PlainKeys),
            gradient: gradient::Identity::<String>::new(),
        };
        let result = v.absorb("world".to_string()).unwrap();
        assert_eq!(result, "world");
    }
}
